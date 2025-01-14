use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Flex, Layout},
    prelude::{Buffer, Rect},
    style::{Modifier, Style, Stylize},
    widgets::{Block, Borders, Widget},
};

use tui_textarea::TextArea;

#[derive(Debug, Default)]
enum InputMode {
    #[default]
    Normal,
    Editing,
}

#[derive(Debug, Default)]
pub struct Editor<'a> {
    name: String,
    content: String,
    input_mode: InputMode,
    name_textarea: TextArea<'a>,
    content_textarea: TextArea<'a>,
    which: u8,
}

impl Editor<'_> {
    fn new() -> Self {
        let mut name_textarea = TextArea::default();
        name_textarea.set_cursor_line_style(Style::new());
        name_textarea.set_placeholder_text("Enter name");

        let mut content_textarea = TextArea::default();
        content_textarea.set_cursor_line_style(Style::default());
        content_textarea.set_placeholder_text("Enter hosts");

        Editor {
            name: String::new(),
            content: String::new(),
            input_mode: InputMode::Normal,
            name_textarea,
            content_textarea,
            which: 0,
        }
    }

    fn inactivate<'a>(textarea: &mut TextArea<'a>, title: &'a str) {
        textarea.set_cursor_line_style(Style::default());
        textarea.set_cursor_style(Style::default());
        textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .style(
                    Style::new().white().on_black().bold().not_underlined()
                )
                .title(title),
        );
    }

    fn activate(textarea: &mut TextArea<'_>, title: &str) {
        textarea.set_cursor_line_style(Style::default().add_modifier(Modifier::UNDERLINED));
        textarea.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
        textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .style(
                    Style::new().light_green().on_black().bold().not_underlined()
                )
                .title(String::from(title)),
        );
    }

    pub fn handle_event(&mut self, event: KeyEvent) {
        if event.code == KeyCode::Enter {
            return;
        }
        match event.code {
            KeyCode::Enter => {}
            KeyCode::Tab => {
                self.which = (self.which + 1) % 2;
            }
            _ => {
                self.name_textarea.input(event);
            }
        }
    }

    pub fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        let [input_rect, textarea_rect] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Fill(1)])
            .flex(Flex::Start)
            .areas(area);

    
        if self.which == 0 {
            Editor::activate(&mut self.name_textarea, "Hosts name");
            Editor::inactivate(&mut self.content_textarea, "Hosts content");
        } else {
            Editor::inactivate(&mut self.name_textarea, "Hosts name");
            Editor::activate(&mut self.content_textarea, "Hosts content");
        }

        self.name_textarea.render(input_rect, buf);

        self.content_textarea.render(textarea_rect, buf);
    }
}
