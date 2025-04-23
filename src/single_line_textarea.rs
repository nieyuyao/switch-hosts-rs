use crate::util::create_new_textarea;
use crossterm::event::KeyEvent;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize},
    widgets::{Block, Borders, Widget},
};
use tui_textarea::TextArea;

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
            r#type,
        };
        t
    }

    pub fn get_text(&self) -> String {
        let lines = self.textarea.lines();
        let text = lines.join("");
        text
    }

    pub fn input(&mut self, event: KeyEvent) {
        self.textarea.input(event);
    }

    pub fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .bg(Color::Blue)
            .title(self.title.clone());
        self.textarea.set_block(block);
        self.textarea.render(area, buf);
    }
}
