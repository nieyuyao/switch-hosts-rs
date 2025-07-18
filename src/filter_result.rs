use ratatui::{layout::Rect, buffer::Buffer};

pub struct FilterResult {}

impl FilterResult {
    pub fn new() -> Self {
        FilterResult {}
    }

    pub fn draw(&mut self, area: Rect, buf: &mut Buffer) {}
}