use crate::util::create_new_textarea;
use crossterm::event::KeyEvent;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize},
    widgets::{Block, Borders, Widget},
};
use tui_textarea::{CursorMove, TextArea};

#[derive(Debug, Default, PartialEq)]
pub enum SinglelineTextareaType {
    #[default]
    Text,
    Password,
}

#[derive(Debug, Default)]
pub struct SingleLineTextarea<'a> {
    textarea: TextArea<'a>,
    title: String,
    error_title: String,
    r#type: SinglelineTextareaType,
}

impl<'a> SingleLineTextarea<'a> {
    pub fn new(place_holder: String, title: String, r#type: SinglelineTextareaType) -> Self {
        let mut textarea = create_new_textarea(place_holder);
        if r#type == SinglelineTextareaType::Password {
            textarea.set_mask_char('*');
        }
        let t = SingleLineTextarea {
            textarea,
            title,
            error_title: String::new(),
            r#type,
        };
        t
    }

    pub fn get_text(&self) -> String {
        let lines = self.textarea.lines();
        let text = lines.join("");
        text
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        let textarea = TextArea::from(text.into().split("\n"));
        self.textarea = textarea;
    }

    pub fn input(&mut self, event: KeyEvent) {
        self.textarea.input(event);
    }

    pub fn move_cursor_to_end(&mut self) {
        self.textarea.move_cursor(CursorMove::End);
    }

    pub fn set_error(&mut self, error_title: impl Into<String>) {
        self.error_title = error_title.into();
    }

    pub fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        let block = if self.error_title.is_empty() {
            Block::default()
                .borders(Borders::ALL)
                .fg(Color::White)
                .bg(Color::Black)
                .title(self.title.clone())
        } else {
            Block::default()
                .borders(Borders::ALL)
                .fg(Color::White)
                .bg(Color::Black)
                .border_style(Color::Red)
                .title(self.error_title.clone())
        };
        self.textarea.set_block(block);
        self.textarea.render(area, buf);
    }
}

pub fn create_new_single_line_textarea<'a>(
    place_holder: impl Into<String>,
    title: impl Into<String>,
    r#type: SinglelineTextareaType
) -> SingleLineTextarea<'a> {
    SingleLineTextarea::new(
        place_holder.into(),
        title.into(),
        r#type,
    )
}



pub fn create_new_single_line_textarea2 (
    place_holder: impl Into<String>,
) {
    let b = place_holder.into();
   
}