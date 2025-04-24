use crate::observer::Observer;
use crate::util::Result;
use crate::{
    data::{read_item_data, write_item_data},
    hosts::read_sys_hosts,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    prelude::{Buffer, Rect},
    style::{Modifier, Style, Stylize},
    widgets::{block::Block, Widget},
};
use tui_textarea::{CursorMove, TextArea};

pub struct Editor<'a> {
    name: String,
    textarea: TextArea<'a>,
    id: String,
    activated: bool,
}

impl Editor<'_> {
    pub fn new() -> Self {
        let textarea = TextArea::default();
        let mut editor = Editor {
            name: String::new(),
            textarea,
            id: "".to_owned(),
            activated: false,
        };
        editor.inactivate();
        editor
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn get_text(&self) -> String {
        let lines = self.textarea.lines();
        let text = lines.join("\n");
        text
    }

    pub fn save_item_content(&self, content: String) -> Result<()> {
        write_item_data(&self.id, content)?;
        Ok(())
    }

    pub fn inactivate(&mut self) {
        self.activated = false;
        self.textarea.set_cursor_line_style(Style::default());
        self.textarea.set_cursor_style(Style::default());
        let style = Style::new().white().on_dark_gray().bold();
        let block = Block::bordered().style(style).title("Hosts Content");
        self.textarea.set_block(block);
    }

    pub fn activate(&mut self) {
        self.activated = true;
        self.textarea
            .set_cursor_line_style(Style::default().add_modifier(Modifier::UNDERLINED));
        self.textarea
            .set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
        let style = Style::new().white().on_black().bold();
        let block = Block::bordered().style(style).title("Hosts Content");
        self.textarea.set_block(block);
    }

    pub fn handle_event(
        &mut self,
        event: KeyEvent,
        mut callback: impl FnMut(bool, Option<String>) -> (),
    ) -> () {
        let is_system = self.id == String::from("system");
        match (event.modifiers, event.code) {
            (_, KeyCode::Esc) => {
                self.inactivate();
                self.save_item_content(self.get_text());
                callback(true, None);
            }
            (KeyModifiers::CONTROL, KeyCode::Char('s') | KeyCode::Char('S')) => {
                if is_system {
                    return;
                }
                self.save_item_content(self.get_text());
                callback(false, None);
            }
            (KeyModifiers::CONTROL, KeyCode::Char('z') | KeyCode::Char('Z')) => {
                if is_system {
                    return;
                }
                self.textarea.undo();
            }
            (KeyModifiers::SHIFT, KeyCode::Left) => {
                self.textarea.move_cursor(CursorMove::Head);
            }
            (KeyModifiers::SHIFT, KeyCode::Right) => {
                self.textarea.move_cursor(CursorMove::End);
            }
            (KeyModifiers::SHIFT, KeyCode::Up) => {
                self.textarea.move_cursor(CursorMove::Top);
            }
            (KeyModifiers::SHIFT, KeyCode::Down) => {
                self.textarea.move_cursor(CursorMove::Bottom);
            }
            other => {
                if is_system
                    && other.1 != KeyCode::Up
                    && other.1 != KeyCode::Down
                    && other.1 != KeyCode::Left
                    && other.1 != KeyCode::Right
                {
                    return;
                }
                self.textarea.input(event);
            }
        }
    }

    pub fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        self.textarea.render(area, buf);
    }
}

impl Observer for Editor<'_> {
    fn update(&mut self, current_id: &str) {
        let id = current_id.to_owned();
        let res: Result<String> = {
            if id == String::from("system") {
                read_sys_hosts()
            } else {
                read_item_data(&id)
            }
        };
        res.and_then(|content| {
            let textarea = TextArea::from(content.split("\n"));
            self.textarea = textarea;
            if self.activated {
                self.activate();
            } else {
                self.inactivate();
            }
            Ok(())
        });
    }
}
