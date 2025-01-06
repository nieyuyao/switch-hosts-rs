use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Layout, Constraint, Flex}
};
use crate::{editor::Editor, list::HostsList};

#[derive(Debug, Default)]
pub struct App {
    running: bool,
    hosts_list: HostsList,
    editor: Editor<'static>
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
                self.draw(frame)
            })?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    ///
    /// ┌               ┐  ┌             ┐
    /// |  List ~ 240 px|  |    Editor   |
    /// └               ┘  └             ┘
    fn draw(&mut self, frame: &mut Frame) {
        let [list_area, editor_area] = Layout::horizontal(vec![
            Constraint::Length(240),
            Constraint::Min(240)
        ])
        .flex(Flex::Start)
        .areas(frame.area());
        self.hosts_list.draw(list_area, frame.buffer_mut());
        self.editor.draw(editor_area, frame.buffer_mut());
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(event) if event.kind == KeyEventKind::Press => self.on_key_event(event),
            _ => {}
        }
        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // Add other key handlers here.
            _ => {
                self.hosts_list.handle_event(key);
            }
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
