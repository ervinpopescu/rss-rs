use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    text::Line,
    widgets::{block::Title, Block, BorderType, Borders, Padding, Paragraph, Wrap},
};

use crate::app::App;

pub fn render(app: &App, frame: &mut Frame<'_>) {
    let description_block = Block::default()
        .title(Title::from(app.titles[app.index].to_string()).alignment(Alignment::Left))
        .title(
            Title::from(format!("{}/{}", app.index + 1, app.titles.len()))
                .alignment(Alignment::Right),
        )
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::proportional(5));
    let description_text = app.descriptions[app.index]
        .lines()
        .map(Line::raw)
        .collect::<Vec<Line>>();
    let description = Paragraph::new(description_text)
        .block(description_block)
        .wrap(Wrap { trim: false })
        .style(Style::default().fg(Color::LightBlue))
        .alignment(Alignment::Left)
        .scroll((app.scroll_offset, 0));
    frame.render_widget(description, frame.size());
}
