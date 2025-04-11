use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, Borders, Widget},
};
use tui_textarea::TextArea;

#[derive(Debug, Default)]
pub struct TitleDialog<'a> {
    textarea: TextArea<'a>,
}

impl<'a> TitleDialog<'a> {
    pub fn new() -> Self {
        let mut textarea = TextArea::default();
        textarea.set_cursor_line_style(Style::default());
        textarea.set_placeholder_text("Input hosts title");
        textarea.set_placeholder_style(Style::default().add_modifier(Modifier::ITALIC));
        TitleDialog { textarea }
    }

    pub fn get_text(&self) -> String {
        let lines = self.textarea.lines();
        let text = lines.join("");
        text
    }

    pub fn handle_event<F: FnMut((bool, Option<String>))>(
        &mut self,
        event: KeyEvent,
        mut callback: F,
    ) -> () {
        match event.code {
            KeyCode::Esc => {
                callback((true, None));
            }
            KeyCode::Enter => {
                let text = self.get_text();
                if text.is_empty() {
                    callback((false, Some(String::from("不能输入空标题"))));
                } else {
                    callback((true, Some(String::from(text))));
                }
            }
            _ => {
                self.textarea.input(event);
                callback((false, None));
            }
        }
    }

    pub fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .bg(Color::Blue)
            .title(String::from("Hosts title"));
        self.textarea.set_block(block);
        self.textarea.render(area, buf);
    }
}
