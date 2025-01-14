use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    prelude::{Rect, Buffer},
    layout::{Constraint, Flex, Layout}, widgets::Widget, DefaultTerminal, Frame
};
use crate::{editor::Editor, list::HostsList};

#[derive(Debug, Default)]
pub struct App {
    running: bool,
    hosts_list: HostsList,
    editor: Editor<'static>,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        self.hosts_list.init();
        while self.running {
            terminal.draw(|frame| {
                frame.render_widget(&mut self, frame.area());
            })?;
            self.handle_crossterm_events()?;
        }
        terminal.show_cursor()?;
        Ok(())
    }

    /// ┌       ┐  ┌         ┐
    /// |  50%  |  |   50%   |
    /// └       ┘  └         ┘
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        let [left, right] = Layout::horizontal(vec![
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .flex(Flex::Start)
        .areas(area);
        self.hosts_list.draw(left, buf);
        self.editor.draw(right, buf);
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(event) if event.kind == KeyEventKind::Press => self.on_key_event(event),
            _ => {}
        }
        Ok(())
    }

    fn on_key_event(&mut self, event: KeyEvent) {
        match (event.modifiers, event.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            _ => {
                self.hosts_list.handle_event(event);
                self.editor.handle_event(event);
            }
        }
    }
    

    fn quit(&mut self) {
        self.running = false;
    }
}


impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.draw(area, buf);
    }
}