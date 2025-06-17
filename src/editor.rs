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

    pub fn cursor_move_up(&mut self) {
        self.textarea.move_cursor(CursorMove::Down);
    }

    pub fn cursor_move_down(&mut self) {
        self.textarea.move_cursor(CursorMove::Up);
    }

    pub fn handle_event(
        &mut self,
        event: KeyEvent,
    ) -> Option<bool> {
        let is_system = self.id == String::from("system");
        match (event.modifiers, event.code) {
            (_, KeyCode::Esc) => {
                self.inactivate();
                self.save_item_content(self.get_text());
                Some(true)
            }
            (KeyModifiers::CONTROL, KeyCode::Char('s') | KeyCode::Char('S')) => {
                if is_system {
                    return None
                }
                self.save_item_content(self.get_text());

                Some(false)
            }
            (KeyModifiers::CONTROL, KeyCode::Char('z') | KeyCode::Char('Z')) => {
                if is_system {
                    return None;
                }
                self.textarea.undo();
                None
            }
            (KeyModifiers::ALT, KeyCode::Left) => {
                self.textarea.move_cursor(CursorMove::Head);
                None
            }
            (KeyModifiers::ALT, KeyCode::Right) => {
                self.textarea.move_cursor(CursorMove::End);
                None
            }
            (KeyModifiers::SHIFT,  KeyCode::Char('d') | KeyCode::Char('D')) => {
                self.textarea.delete_line_by_head();
                None
            }
            (KeyModifiers::SHIFT, KeyCode::Char('o') | KeyCode::Char('O')) => {
                self.textarea.move_cursor(CursorMove::Top);
                None
            }
            (KeyModifiers::SHIFT, KeyCode::Char('g') | KeyCode::Char('G')) => {
                self.textarea.move_cursor(CursorMove::Bottom);
                None
            }
            other => {
                if is_system
                    && other.1 != KeyCode::Up
                    && other.1 != KeyCode::Down
                    && other.1 != KeyCode::Left
                    && other.1 != KeyCode::Right
                {
                    return None;
                }
                self.textarea.input(event);
                None
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
