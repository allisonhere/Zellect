use zellij_tile::prelude::*;

pub struct State {
    pub layouts: Vec<(String, LayoutInfo)>,
    pub selected: usize,
    pub permissions_granted: bool,
    pub session_update_received: bool,
    pub error: Option<String>,
}

impl Default for State {
    fn default() -> Self {
        State {
            layouts: Vec::new(),
            selected: 0,
            permissions_granted: false,
            session_update_received: false,
            error: None,
        }
    }
}

impl State {
    pub fn handle_session_update(&mut self, session_infos: Vec<SessionInfo>) {
        self.session_update_received = true;
        self.layouts = session_infos
            .into_iter()
            .find(|session_info| session_info.is_current_session)
            .map(|session_info| {
                session_info
                    .available_layouts
                    .into_iter()
                    .map(|layout_info| (layout_info.name().to_string(), layout_info))
                    .collect()
            })
            .unwrap_or_default();
        if !self.layouts.is_empty() && self.selected >= self.layouts.len() {
            self.selected = self.layouts.len() - 1;
        }
    }

    pub fn handle_key(&mut self, key: KeyWithModifier) -> bool {
        match key.bare_key {
            BareKey::Up | BareKey::Char('k') => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
                true
            }
            BareKey::Down | BareKey::Char('j') => {
                if self.selected + 1 < self.layouts.len() {
                    self.selected += 1;
                }
                true
            }
            BareKey::Enter => {
                self.apply_layout_new_tab();
                true
            }
            BareKey::Char('s') => {
                self.apply_layout_new_session();
                true
            }
            BareKey::Esc | BareKey::Char('q') => {
                hide_self();
                true
            }
            _ => false,
        }
    }

    fn apply_layout_new_tab(&self) {
        if let Some((_, layout_info)) = self.layouts.get(self.selected) {
            new_tabs_with_layout_info(layout_info.clone());
            hide_self();
        }
    }

    fn apply_layout_new_session(&self) {
        if let Some((_, layout_info)) = self.layouts.get(self.selected) {
            switch_session_with_layout(None, layout_info.clone(), None);
            hide_self();
        }
    }
}
