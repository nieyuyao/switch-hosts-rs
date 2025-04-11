use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Line,
    widgets::Widget,
};

#[derive(Debug, Default)]
pub struct Tip<'a> {
    line: Option<Line<'a>>,
}
impl<'a> Tip<'a> {
    pub fn new() -> Self {
        return Tip::default()
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
