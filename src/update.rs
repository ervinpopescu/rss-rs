use crate::app::App;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};
pub fn update_on_key(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }
        }
        KeyCode::Right | KeyCode::Char('j') => match app.index.cmp(&{ app.titles.len() - 1 }) {
            std::cmp::Ordering::Less => app.increment_index(),
            std::cmp::Ordering::Equal => app.index = 0,
            std::cmp::Ordering::Greater => app.index = 0,
        },
        KeyCode::Left | KeyCode::Char('k') => match app.index.cmp(&1) {
            std::cmp::Ordering::Less => app.index = app.titles.len() - 1,
            std::cmp::Ordering::Equal => app.decrement_index(),
            std::cmp::Ordering::Greater => app.decrement_index(),
        },
        _ => {}
    }
}

pub fn update_on_mouse(app: &mut App, mouse_event: MouseEvent, height: u16) {
    match mouse_event.kind {
        MouseEventKind::ScrollUp => {
            if app.scroll_offset > 0 {
                app.scroll_offset -= 1;
            }
        }
        MouseEventKind::ScrollDown => {
            if app.scroll_offset < height {
                app.scroll_offset += 1;
            }
        }
        _ => {}
    }
}
