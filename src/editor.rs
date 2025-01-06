use crossterm::event::KeyEvent;
use ratatui::{
    prelude::{Buffer, Rect},
    style::Style
}; 



use tui_textarea::{TextArea, Input};

#[derive(Debug, Default)]
enum InputMode {
    #[default]
    Normal,
    Editing,    
}

#[derive(Debug, Default)]
pub struct Editor<'a> {
    name: String,
    content: String,
    input_mode: InputMode,
    name_textarea: TextArea<'a>,
    content_textarea: TextArea<'a>,
}

impl Editor<'_> {
    fn new() -> Self {
        let mut name_textarea = TextArea::default();
        name_textarea.set_cursor_line_style(Style::default());
        name_textarea.set_placeholder_text("Enter name");
        let mut content_textarea = TextArea::default();
        content_textarea.set_cursor_line_style(Style::default());
        content_textarea.set_placeholder_text("Enter hosts");
        Editor {
            name: String::new(),
            content: String::new(),
            input_mode: InputMode::Normal,
            name_textarea,
            content_textarea,
        }
    }

    pub fn handle_event(&mut self, event: KeyEvent) {
    }

    pub fn draw(&self, area: Rect, buf: &mut Buffer) {
    }
}
