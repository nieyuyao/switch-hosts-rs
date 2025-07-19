use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::single_line_textarea::{
    create_new_single_line_textarea, SingleLineTextarea, SinglelineTextareaType,
};

pub struct Search {
    filter_area: SingleLineTextarea<'static>,
}

impl Search {
    pub fn new() -> Self {
        Search {
            filter_area: create_new_single_line_textarea(
                "",
                "Filter",
                SinglelineTextareaType::Text,
            ),
        }
    }

    pub fn handle_event(&mut self, event: KeyEvent) {
        self.filter_area.input(event);
    }

    pub fn get_text(&self) -> String {
        self.filter_area.get_text()
    }

    pub fn clear(&mut self) {
        self.filter_area =
            create_new_single_line_textarea("", "Filter", SinglelineTextareaType::Text)
    }

    pub fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        self.filter_area.draw(area, buf);
    }
}
