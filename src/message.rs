use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::Widget,
};

#[derive(Debug, Default)]
pub struct Message();

impl Message {
    pub fn draw(&mut self, text: String, area: Rect, buf: &mut Buffer) {
        let line = Line::from(text).style(Style::default().fg(Color::Black).bg(Color::Yellow));
        line.render(area, buf);
    }
}
