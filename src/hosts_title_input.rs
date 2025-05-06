use crate::single_line_textarea::{SingleLineTextarea, SinglelineTextareaType};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{buffer::Buffer, layout::Rect};

fn create_new_textarea<'a>() -> SingleLineTextarea<'a> {
    SingleLineTextarea::new(
        "输入标题".to_owned(),
        "Hosts标题".to_owned(),
        SinglelineTextareaType::Text,
    )
}

#[derive(Debug, Default)]
pub struct TitleInput<'a> {
    textarea: SingleLineTextarea<'a>,
    title: String,
    is_new: bool
}

impl<'a> TitleInput<'a> {
    pub fn new() -> Self {
        TitleInput {
            textarea: create_new_textarea(),
            title: String::new(),
            is_new: true,
        }
    }

    pub fn set_text(&mut self, text: String, is_new: bool) {
        self.textarea.set_text(text);
        self.textarea.move_cursor_to_end();
        self.is_new = is_new;
    }

    pub fn handle_event<F: FnMut((bool, Option<String>, bool))>(
        &mut self,
        event: KeyEvent,
        mut callback: F,
    ) -> () {
        match event.code {
            KeyCode::Esc => {
                callback((true, None, self.is_new));
                self.is_new = true;
                self.textarea = create_new_textarea()
            }
            KeyCode::Enter => {
                let text = self.textarea.get_text();
                if text.is_empty() {
                    callback((false, Some(String::from("不能输入空标题")), self.is_new));
                } else {
                    callback((true, Some(String::from(text)), self.is_new));
                    self.is_new = true;
                    self.textarea = create_new_textarea();
                }
            }
            _ => {
                self.textarea.input(event);
                callback((false, None, self.is_new));
            }
        }
    }

    pub fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        self.textarea.draw(area, buf);
    }
}
