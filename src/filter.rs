use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::single_line_textarea::{
    create_new_single_line_textarea, SingleLineTextarea, SinglelineTextareaType,
};

pub struct Filter {
    filter_area: SingleLineTextarea<'static>,
}

impl Filter {
    pub fn new() -> Self {
        Filter {
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

    pub fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        self.filter_area.draw(area, buf);
    }

}
