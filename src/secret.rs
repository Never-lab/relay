//! Keeps opted-in secrets in the system credential store: Windows Credential Manager, the
//! macOS Keychain, or the Secret Service on Linux. Server library connection strings and
//! remote passwords for saved connections both live here; the connection library itself never
//! holds them.
const SERVICE: &str = "Relay";

/// The credential store entry for each kind of server library.
#[derive(Clone, Copy)]
pub enum Account {
    Mssql,
    Mysql,
}

impl Account {
    fn name(self) -> &'static str {
        match self {
            Self::Mssql => "MSSQL connection string",
            Self::Mysql => "MySQL connection URL",
        }
    }
}

fn entry(account: Account) -> Result<keyring::Entry, String> {
    keyring::Entry::new(SERVICE, account.name())
        .map_err(|e| format!("The system credential store is unavailable: {e}"))
}

pub fn save(account: Account, connection: &str) -> Result<(), String> {
    entry(account)?
        .set_password(connection)
        .map_err(|e| e.to_string())
}

pub fn load(account: Account) -> Option<String> {
    entry(account).ok()?.get_password().ok()
}

pub fn forget(account: Account) -> Result<(), String> {
    match entry(account)?.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

fn connection_entry(id: u64) -> Result<keyring::Entry, String> {
    keyring::Entry::new(SERVICE, &format!("connection:{id}"))
        .map_err(|e| format!("The system credential store is unavailable: {e}"))
}

/// Stores a remote password for library connection `id`.
pub fn save_connection_password(id: u64, password: &str) -> Result<(), String> {
    connection_entry(id)?
        .set_password(password)
        .map_err(|e| e.to_string())
}

/// The password saved for library connection `id`, if any.
pub fn load_connection_password(id: u64) -> Option<String> {
    connection_entry(id).ok()?.get_password().ok()
}

/// Removes any password saved for library connection `id`.
pub fn forget_connection_password(id: u64) -> Result<(), String> {
    match connection_entry(id)?.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

/// Fills `password` from the credential store when `id` is a saved connection that has one,
/// and sets `remember` so the sign-in form keeps the box ticked.
pub fn fill_connection_password(id: u64, password: &mut String, remember: &mut bool) {
    if id == 0 {
        return;
    }
    if let Some(saved) = load_connection_password(id) {
        *password = saved;
        *remember = true;
    }
}

/// Saves or forgets the password for library connection `id` according to `remember`.
/// Ad-hoc connections (`id == 0`) are left alone.
pub fn persist_connection_password(id: u64, password: &str, remember: bool) -> Result<(), String> {
    if id == 0 {
        return Ok(());
    }
    if remember {
        if password.is_empty() {
            forget_connection_password(id)
        } else {
            save_connection_password(id, password)
        }
    } else {
        forget_connection_password(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forgetting_a_missing_connection_password_is_ok() {
        // A high id is extremely unlikely to collide with a real saved connection.
        assert!(forget_connection_password(u64::MAX - 7).is_ok());
        assert!(load_connection_password(u64::MAX - 7).is_none());
    }

    #[test]
    fn persist_skips_ad_hoc_connections() {
        assert!(persist_connection_password(0, "secret", true).is_ok());
        assert!(load_connection_password(0).is_none());
    }

    #[test]
    fn connection_password_round_trips_in_the_credential_store() {
        let id = u64::MAX - 11;
        let _ = forget_connection_password(id);
        save_connection_password(id, "relay-test-password")
            .expect("credential store should accept a test password");
        assert_eq!(
            load_connection_password(id).as_deref(),
            Some("relay-test-password")
        );
        forget_connection_password(id).unwrap();
        assert!(load_connection_password(id).is_none());
    }

    #[test]
    fn fill_marks_remember_when_a_password_is_saved() {
        let id = u64::MAX - 13;
        let _ = forget_connection_password(id);
        save_connection_password(id, "saved").unwrap();
        let mut password = String::new();
        let mut remember = false;
        fill_connection_password(id, &mut password, &mut remember);
        assert_eq!(password, "saved");
        assert!(remember);
        let _ = forget_connection_password(id);
    }
}
