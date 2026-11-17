# Changelog

## 0.6.2 — 2026-11-17

### Changed

- Refined Settings with clearer sections, consistent controls, keyboard-friendly theme choices, and layouts that adapt to smaller windows and enlarged text.
- Improved the connection library with resizable columns, saved column widths, and more compact layouts.
- Kept remote addresses visible in existing session headers and tab-bar space, preserving room for terminals, desktops, and file browsers.
- CSV, JSON, SSH config, and RDP imports now load in the background with a cancellable loading state. Import previews render only visible rows to keep large libraries responsive.

### Fixed

- Improved keyboard navigation, focus indicators, accessible names, selection states, and form labels throughout navigation, connection rows, tabs, palettes, and file-browser controls.
- Exposed terminal output, cursor positions, and selections to assistive technology, with output announcements limited to the focused terminal's live view.
- Kept connection-dialog actions and validation messages visible in smaller windows, and made connection failures easier to recover from with reconnect and edit actions.
- Kept transfer actions visible in compact split panes, and improved placeholder contrast in light themes.
- Kept import errors inside the review dialog and ensured cancelled imports cannot change the library when their background work finishes.

[Full changelog](https://github.com/nurlennart/relay/compare/v0.6.1...v0.6.2)

## 0.6.1 — 2026-11-16

### Changed

- The commit ID in Settings now opens its GitHub commit page. Clicking the version still copies the full build information.

### Fixed

- RDP on macOS and Linux now refreshes cursor hiding when you click the remote desktop or return to its window, addressing the local pointer remaining visible over the remote pointer.
- Deleted connections are removed from the library's recent history after startup, saves, refreshes, and background syncs. Other libraries' history is preserved, and failed loads do not clear recents.

[Full changelog](https://github.com/nurlennart/relay/compare/v0.6.0...v0.6.1)
