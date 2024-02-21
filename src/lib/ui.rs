use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    text::Line,
    widgets::{block::Title, Block, BorderType, Borders, Padding, Paragraph, Wrap},
};

use crate::app::App;

/// select title and description from global [`App`] struct based on current [`index`] and `render` them
///
/// [`App`]: crate::app::App
/// [`index`]: crate::app::App#structfield.index
pub fn render(app: &App, frame: &mut Frame<'_>) {
    let description_block = Block::default()
        .title(Title::from(app.pseudo_hashmap[app.index].0.clone()).alignment(Alignment::Left))
        .title(
            Title::from(format!("{}/{}", app.index + 1, app.pseudo_hashmap.len()))
                .alignment(Alignment::Right),
        )
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::proportional(5));
    let description_text = app.pseudo_hashmap[app.index].1.clone();
    let description = Paragraph::new(description_text)
        .block(description_block)
        .wrap(Wrap { trim: false })
        .style(Style::default().fg(Color::LightBlue))
        .alignment(Alignment::Left)
        .scroll((app.scroll_offset, 0));
    frame.render_widget(description, frame.size());
}
