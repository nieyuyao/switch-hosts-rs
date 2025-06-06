use crate::{
    hosts::check_password_correct,
    single_line_textarea::{
        create_new_single_line_textarea, SingleLineTextarea, SinglelineTextareaType,
    },
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{buffer::Buffer, layout::Rect};

#[derive(Debug, Default)]
pub struct PasswordInput<'a> {
    textarea: SingleLineTextarea<'a>,
}

impl<'a> PasswordInput<'a> {
    pub fn new() -> Self {
        PasswordInput {
            textarea: create_new_single_line_textarea(
                "",
                "请输入你的登录密码（sudo 密码）",
                SinglelineTextareaType::Password,
            ),
        }
    }

    pub fn handle_event(&mut self, event: KeyEvent) -> (bool, Option<String>) {
        match event.code {
            KeyCode::Esc => {
                self.textarea.set_text("");
                return (true, None);
            }
            KeyCode::Enter => {
                let text: String = self.textarea.get_text();
                let is_correct = check_password_correct(text.clone(), || {});
                if is_correct.is_ok() {
                    self.textarea.set_text("text");
                    return (true, Some(String::from(text)));
                } else {
                    self.textarea.set_error("密码错误，请重新输入");
                    return (false, None);
                }
            }
            _ => {
                self.textarea.set_error("");
                self.textarea.input(event);
                return (false, None);
            }
        }
    }

    pub fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        self.textarea.draw(area, buf);
    }
}
