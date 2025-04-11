use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, Borders, Widget},
};
use tui_textarea::TextArea;
use crate::common::EventHandler;

#[derive(Debug, Default)]
pub struct TitleDialog<'a> {
    textarea: TextArea<'a>,
}

impl<'a> TitleDialog<'a> {
    pub fn new() -> Self {
        let mut textarea = TextArea::default();
        textarea.set_placeholder_text("Input hosts title");
        textarea.set_placeholder_style(Style::default().add_modifier(Modifier::ITALIC));
        TitleDialog { textarea }
    }

    fn get_text(&self) -> String {
        let lines = self.textarea.lines();
        let text = lines.join("");
        text
    }
    
    pub fn inactivate(&mut self) {
        self.textarea.set_cursor_line_style(Style::default());
        self.textarea.set_cursor_style(Style::default());
        self.textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .style(
                    Style::new().white().on_black().bold().not_underlined()
                )
                .title("Hosts title"),
        );
    }

    pub fn activate(&mut self) {
        self.textarea.set_cursor_line_style(Style::default());
        self.textarea.set_cursor_style(Style::default());
        let block = Block::default()
            .borders(Borders::ALL)
            .style(
                Style::new().light_green().on_black().bold().not_underlined()
            )
            .title(String::from("Hosts title"));
        self.textarea.set_block(block);
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


impl<'a> EventHandler for TitleDialog<'a> {
    fn handle_event(&mut self, event: KeyEvent, mut on_close: impl FnMut() -> ()) -> () {
        match event.code {
            KeyCode::Esc => {
                on_close();
            }
            KeyCode::Enter => {
                // TODO:
            },
            _ => {
                self.textarea.input(event);
            }
        }
    }
}