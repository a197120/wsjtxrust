use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::appstate::AppState;

pub fn update(appstate: &mut AppState, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Up => {
            appstate.list_state.previous();
        }
        KeyCode::Down => {
            appstate.list_state.next();
        }
        KeyCode::Esc | KeyCode::Char('q') => appstate.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                appstate.quit()
            }
        }
        _ => {}
    };
}