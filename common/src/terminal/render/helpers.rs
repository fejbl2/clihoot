use ratatui::{
    layout::Alignment,
    prelude::*,
    style::{self},
    widgets::{block::Title, Block, BorderType, Borders, Padding, Paragraph, Wrap},
};

#[must_use]
pub fn get_outer_block(name: &str) -> Block<'_> {
    let title = Title::from(" Clihoot: ".to_owned() + name + " ");
    let block = Block::default()
        .title(title)
        .title_style(style::Style::default().bold())
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .padding(Padding::new(2, 2, 1, 1));
    block
}

#[must_use]
pub fn get_inner_block(title: &str) -> Block<'_> {
    let block = Block::new()
        .borders(Borders::TOP)
        .title(title)
        .title_style(style::Style::default().bold())
        .title_alignment(Alignment::Center)
        .padding(Padding::new(1, 1, 1, 1));
    block
}

#[must_use]
pub fn get_bordered_block() -> Block<'static> {
    let block = Block::default().borders(Borders::ALL);
    block
}

#[must_use]
pub fn get_highlighted_style() -> style::Style {
    style::Style::default().add_modifier(style::Modifier::ITALIC)
}

#[must_use]
pub fn get_player_style() -> style::Style {
    style::Style::default().underlined().bold()
}

#[must_use]
pub fn get_centered_paragraph<'a>(text: &'a str, block: Block<'a>) -> Paragraph<'a> {
    let paragraph = Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .block(block)
        .alignment(Alignment::Center);
    paragraph
}
