use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{buffer::Buffer, layout::Rect};
use crate::single_line_textarea::{SingleLineTextarea, SinglelineTextareaType};

fn create_new_textarea<'a>() -> SingleLineTextarea<'a> {
    SingleLineTextarea::new(
        "".to_owned(),
        "请输入你的登录密码（sudo 密码）".to_owned(),
        SinglelineTextareaType::Password,
    )
}

#[derive(Debug, Default)]
pub struct PasswordInput<'a> {
    textarea: SingleLineTextarea<'a>,
}

impl<'a> PasswordInput<'a> {
    pub fn new() -> Self {
        PasswordInput {
            textarea: create_new_textarea(),
        }
    }

    pub fn handle_event<F: FnMut(bool, Option<String>)>(
        &mut self,
        event: KeyEvent,
        mut callback: F,
    ) -> () {
        match event.code {
            KeyCode::Esc => {
                callback(true, None);
                self.textarea = create_new_textarea()
            }
            KeyCode::Enter => {
                let text = self.textarea.get_text();
                callback(true, Some(String::from(text)));
                self.textarea = create_new_textarea();
            }
            _ => {
                self.textarea.input(event);
                callback(false, None);
            }
        }
    }

    pub fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        self.textarea.draw(area, buf);
    }
}
