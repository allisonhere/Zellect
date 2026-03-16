use std::collections::BTreeMap;
use zellij_tile::prelude::*;

mod state;
mod ui;

use state::State;

register_plugin!(State);

// wasmtime requires `_start` to initialize the WASI context (stdin/stdout).
// cdylib crates don't generate it automatically, so we export it explicitly.
#[no_mangle]
pub fn _start() {
    std::panic::set_hook(Box::new(|info| {
        report_panic(info);
    }));
}

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
        ]);
        subscribe(&[
            EventType::Key,
            EventType::PermissionRequestResult,
            EventType::SessionUpdate,
        ]);
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::PermissionRequestResult(PermissionStatus::Granted) => {
                self.permissions_granted = true;
                true
            }
            Event::PermissionRequestResult(PermissionStatus::Denied) => {
                self.error = Some("Permission denied".to_string());
                true
            }
            Event::SessionUpdate(session_infos, _) => {
                self.handle_session_update(session_infos);
                true
            }
            Event::Key(key) => self.handle_key(key),
            _ => false,
        }
    }

    fn render(&mut self, rows: usize, cols: usize) {
        ui::render(self, rows, cols);
    }
}
