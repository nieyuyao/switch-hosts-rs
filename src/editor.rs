use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::{Buffer, Rect},
    style::Style,
    widgets::Widget,
};
use tui_textarea::TextArea;

#[derive(Debug, Default)]
pub struct Editor<'a> {
    name: String,
    textarea: TextArea<'a>,
}

impl Editor<'_> {
    pub fn new() -> Self {
        let mut textarea = TextArea::default();
        textarea.set_cursor_line_style(Style::default());
        textarea.set_placeholder_text("Enter hosts");
        let editor = Editor {
            name: String::new(),
            textarea,
        };
        editor
    }

    pub fn handle_event(&mut self, event: KeyEvent, mut on_close: impl FnMut() -> ()) -> () {
        match event.code {
            KeyCode::Esc => {
                on_close();
            },
            KeyCode::Enter => {
                // TODO:
            },
            _ => {
                self.textarea.input(event);
            }
        }
    }

    pub fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        self.textarea.render(area, buf);
    }
}
