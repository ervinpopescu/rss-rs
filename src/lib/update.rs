use crate::app::App;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};

/// update program on key input events
pub fn update_on_key(app: &mut App, key_event: KeyEvent, height: u16) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }
        }
        KeyCode::Right | KeyCode::Char('j') => {
            app.scroll_offset = 0;
            match app.index.cmp(&{ app.pseudo_hashmap.len() - 1 }) {
                std::cmp::Ordering::Less => app.increment_index(),
                std::cmp::Ordering::Equal => app.index = 0,
                std::cmp::Ordering::Greater => app.index = 0,
            }
        }
        KeyCode::Left | KeyCode::Char('k') => {
            app.scroll_offset = 0;
            match app.index.cmp(&1) {
                std::cmp::Ordering::Less => app.index = app.pseudo_hashmap.len() - 1,
                std::cmp::Ordering::Equal => app.decrement_index(),
                std::cmp::Ordering::Greater => app.decrement_index(),
            }
        }
        KeyCode::Home => {
            app.scroll_offset = 0;
            app.index = 0;
        }
        KeyCode::End => {
            app.scroll_offset = 0;
            app.index = app.pseudo_hashmap.len() - 1;
        }
        KeyCode::Up => {
            if app.scroll_offset > 0 {
                app.scroll_offset -= 1;
            }
        }
        KeyCode::Down => {
            if app.scroll_offset < height {
                app.scroll_offset += 1;
            }
        }
        _ => {}
    }
}

/// update program on mouse events
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
