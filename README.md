# Zellect

Zellect is a small Zellij plugin that shows the layouts available to the current session and lets you add the selected layout as new tab(s).

It is intentionally scoped to the current session. It does not create or switch to a different Zellij session.

## What It Does

- Lists the layouts Zellij reports for the current session
- Opens the selected layout as new tab(s) in the current session
- Runs as a floating plugin pane

## Requirements

- Rust toolchain with `cargo`
- The `wasm32-wasip1` Rust target
- Zellij `0.43.x`

## Build

Install the WASI target if you do not already have it:

```bash
rustup target add wasm32-wasip1
```

Build the plugin:

```bash
cargo build --release --target wasm32-wasip1
```

The compiled plugin will be written to:

```text
target/wasm32-wasip1/release/zellect.wasm
```

## Install In Zellij

Choose one of these approaches.

### Option 1: Use The Built Plugin From Your Checkout

This is the simplest option while developing locally. Use the absolute path to the built `.wasm` file from your own clone of the repo.

Example:

```kdl
bind "Alt l" {
    LaunchOrFocusPlugin "file:/absolute/path/to/Zellect/target/wasm32-wasip1/release/zellect.wasm" {
        floating true
        move_to_focused_tab true
    }
}
```

Replace `/absolute/path/to/Zellect` with the location of your checkout.

### Option 2: Copy The Plugin Into Your Zellij Plugins Directory

Create a plugins directory if needed:

```bash
mkdir -p ~/.config/zellij/plugins
```

Copy the built plugin:

```bash
cp target/wasm32-wasip1/release/zellect.wasm ~/.config/zellij/plugins/zellect.wasm
```

Then reference that stable path from your Zellij config:

```kdl
bind "Alt l" {
    LaunchOrFocusPlugin "file:~/.config/zellij/plugins/zellect.wasm" {
        floating true
        move_to_focused_tab true
    }
}
```

Add the bind inside the `shared_among "normal" "locked"` section of your Zellij config.

If `Alt l` is already in use in your config, either remove the conflicting bind or choose another shortcut.

The example bind snippet is also included in [zellect.kdl](./zellect.kdl).

## Reload And Test

After changing the config, start a fresh Zellij session:

```bash
zellij
```

Then press:

```text
Alt+l
```

Inside the plugin:

- `j` / `k` or arrow keys move the selection
- `Enter` adds the selected layout as new tab(s)
- `q` or `Esc` closes the plugin

## Direct Test Command

If you want to test the plugin without relying on a keybinding, run this from inside an active Zellij session:

```bash
zellij action launch-or-focus-plugin -f -m "file:/absolute/path/to/Zellect/target/wasm32-wasip1/release/zellect.wasm"
```

Or, if you copied it into the Zellij plugins directory:

```bash
zellij action launch-or-focus-plugin -f -m "file:~/.config/zellij/plugins/zellect.wasm"
```

## Development Notes

- The plugin requests:
  - `ReadApplicationState`
  - `ChangeApplicationState`
- Layout selection is populated from the current session's `available_layouts`
- Applying a layout uses `new_tabs_with_layout_info(...)`

## Project Layout

```text
src/lib.rs     Plugin registration, permissions, subscriptions
src/state.rs   Session state and key handling
src/ui.rs      Terminal UI rendering
zellect.kdl    Example Zellij keybinding snippet
```

## Troubleshooting

### `Alt+l` does nothing

Common causes:

- The bind was added to the wrong key table
- Another `Alt l` bind appears earlier in the active section
- Zellij was not restarted after the config change

Use the direct launch command above to confirm the plugin itself works.

### The plugin opens but shows no layouts

Make sure your Zellij installation can see layouts in its configured layouts directory.

### I changed the code but Zellij still loads the old behavior

Rebuild the plugin:

```bash
cargo build --release --target wasm32-wasip1
```

Then launch the plugin again in a fresh Zellij session.
