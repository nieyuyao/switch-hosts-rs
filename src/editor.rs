use crossterm::event::KeyEvent;
use ratatui::{
    prelude::{Buffer, Rect},
    style::{Style, Stylize},
    widgets::{Block, Borders, Widget},
};
use tui_textarea::TextArea;

#[derive(Debug, Default)]
pub struct Editor<'a> {
    name: String,
    textarea: TextArea<'a>,
}

impl Editor<'_> {
    pub fn new() -> Self {
        let mut textarea = TextArea::default();
        textarea.set_placeholder_text("Enter hosts");
        let mut editor = Editor {
            name: String::new(),
            textarea,
        };
        editor.inactivate();
        editor
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
                .title("Hosts content"),
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
            .title(String::from("Hosts content"));
        self.textarea.set_block(block);
    }
    pub fn handle_event(&mut self, event: KeyEvent) {
        self.textarea.input(event);
    }

    pub fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        self.textarea.render(area, buf);
    }
}
