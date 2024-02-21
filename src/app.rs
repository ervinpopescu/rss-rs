#[derive(Debug, Default)]
pub struct App {
    pub titles: Vec<String>,
    pub descriptions: Vec<String>,
    pub index: usize,
    pub scroll_offset: u16,
    pub should_quit: bool,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(titles: Vec<String>, descriptions: Vec<String>) -> Self {
        Self {
            titles,
            descriptions,
            scroll_offset: 0,
            index: 0,
            should_quit: false,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn increment_index(&mut self) {
        if let Some(res) = self.index.checked_add(1) {
            self.index = res;
        }
    }
    pub fn decrement_index(&mut self) {
        if let Some(res) = self.index.checked_sub(1) {
            self.index = res;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_app_increment_counter() {
        let mut app = App::default();
        app.increment_index();
        assert_eq!(app.index, 1);
    }

    #[test]
    fn test_app_decrement_counter() {
        let mut app = App::default();
        app.decrement_index();
        assert_eq!(app.index, 0);
    }
}
