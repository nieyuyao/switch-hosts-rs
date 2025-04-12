use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
};
use crate::single_line_textarea::SingleLineTextarea;

fn create_new_textarea<'a>() -> SingleLineTextarea<'a> {
    SingleLineTextarea::new("输入标题".to_owned(), "Hosts标题".to_owned())
}

#[derive(Debug, Default)]
pub struct TitleInput<'a> {
    textarea: SingleLineTextarea<'a>,
}

impl<'a> TitleInput<'a> {
    pub fn new() -> Self {
        TitleInput { textarea: create_new_textarea() }
    }

    pub fn handle_event<F: FnMut((bool, Option<String>))>(
        &mut self,
        event: KeyEvent,
        mut callback: F,
    ) -> () {
        match event.code {
            KeyCode::Esc => {
                callback((true, None));
                self.textarea = create_new_textarea()
            }
            KeyCode::Enter => {
                let text = self.textarea.get_text();
                if text.is_empty() {
                    callback((false, Some(String::from("不能输入空标题"))));
                } else {
                    callback((true, Some(String::from(text))));
                    self.textarea = create_new_textarea()
                }
            }
            _ => {
                self.textarea.input(event);
                callback((false, None));
            }
        }
    }

    pub fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        self.textarea.draw(area, buf);
    }
}
