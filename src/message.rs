use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Line,
    widgets::Widget,
};

#[derive(Debug, Default)]
pub struct Message<'a> {
    line: Option<Line<'a>>
}
impl<'a> Message<'a> {
    pub fn new() -> Self {
        return Message::default()
    }

    pub fn set_line(&mut self, line: Line<'a>) {
        self.line = Some(line);
    }

    pub fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        if let Some(line) = &self.line {
            line.render(area, buf);
        }
    }
}
