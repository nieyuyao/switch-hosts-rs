use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Padding, Widget},
};

#[derive(Debug, Default)]
pub struct Popup();

impl Popup {
    pub fn draw(&mut self, text: String, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().padding(Padding::ZERO);
        block.render(area, buf);
        let line = Line::from(text)
            .style(Style::default().fg(Color::White));
        let width = line.width();
        let x = area.x + area.width / 2 - (width as u16) / 2;
        let y = area.y + area.height / 2;
        let line_rect = Rect::new(
            x,
            y,
            width as u16,
            2,
        );
        line.render(line_rect, buf);
    }
}
