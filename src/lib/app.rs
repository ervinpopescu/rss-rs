use chrono::NaiveDateTime;
use ratatui::text::Line;
use regex::Regex;

/// Global program state
#[derive(Debug, Default)]
pub struct App<'a> {
    /// Job titles and descriptions from RSS channel
    pub pseudo_hashmap: Vec<(String, Vec<Line<'a>>)>,
    /// Current job index
    pub index: usize,
    /// Used for scrolling in [`ui::render`]
    ///
    /// [`ui::render`]: crate::ui::render
    pub scroll_offset: u16,
    /// Set when user quits the app, terminates the main event loop
    pub should_quit: bool,
}

impl<'a> App<'a> {
    /// Constructs a new instance of [`App`].
    pub fn new(titles: Vec<String>, descriptions: Vec<String>) -> Self {
        let mut pseudo_hashmap: Vec<(String, String)> = titles
            .into_iter()
            .map(|title| title.to_string())
            .zip(descriptions.into_iter().map(|desc| desc.to_string()))
            .collect();

        pseudo_hashmap.sort_by(|(_, desc_a), (_, desc_b)| {
            let date_a = App::extract_date(desc_a);
            let date_b = App::extract_date(desc_b);

            let date_cmp = date_b.cmp(&date_a);
            match date_cmp {
                std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                std::cmp::Ordering::Equal => {
                    let budget_a = App::extract_budget(desc_a);
                    let budget_b = App::extract_budget(desc_b);

                    // Compare budgets in descending order if both are Some
                    if let (Some(ba), Some(bb)) = (budget_a, budget_b) {
                        return bb.partial_cmp(&ba).unwrap_or(std::cmp::Ordering::Equal);
                    }

                    // Place descriptions without budget after those with budget
                    match (budget_a, budget_b) {
                        (Some(_), None) => std::cmp::Ordering::Less,
                        (None, Some(_)) => std::cmp::Ordering::Greater,
                        _ => std::cmp::Ordering::Equal,
                    }
                }
            }
        });

        let pseudo_hashmap: Vec<(String, Vec<Line>)> = pseudo_hashmap
            .iter()
            .map(|(t, d)| {
                (
                    t.clone(),
                    d.lines()
                        .map(|line| Line::raw(String::from(line)))
                        .collect::<Vec<Line>>(),
                )
            })
            .collect();
        Self {
            pseudo_hashmap,
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
    /// increment [`App`].index
    pub fn increment_index(&mut self) {
        if let Some(res) = self.index.checked_add(1) {
            self.index = res;
        }
    }
    /// decrement [`App`].index
    pub fn decrement_index(&mut self) {
        if let Some(res) = self.index.checked_sub(1) {
            self.index = res;
        }
    }
    #[doc(hidden)]
    // pub fn extract_date(description: &str) -> String {
    pub fn extract_date(description: &str) -> NaiveDateTime {
        let re =
            Regex::new(r"Posted On: ([A-Za-z]+) (\d{2,3}), (\d{4}) (\d{1,2}):(\d{2}) UTC").unwrap();
        match re.captures(description) {
            Some(captures) => {
                let month = captures.get(1).unwrap().as_str();
                let day = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
                let year = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
                let hour = captures.get(4).unwrap().as_str().parse::<u32>().unwrap();
                let minute = captures.get(5).unwrap().as_str().parse::<u32>().unwrap();
                match NaiveDateTime::parse_from_str(
                    &format!("{} {} {} {}:{}", month, day, year, hour, minute),
                    "%B %e %Y %H:%M",
                ) {
                    Ok(date) => date,
                    Err(_) => NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
                }
            }
            None => NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
        }
    }

    #[doc(hidden)]
    pub fn extract_budget(description: &str) -> Option<f64> {
        description
            .lines()
            .find(|line| line.starts_with("Budget: $"))
            .and_then(|line| line.trim_start_matches("Budget: $").parse().ok())
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
