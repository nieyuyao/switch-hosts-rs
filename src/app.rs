use crate::{common::EventHandler, editor::Editor, list::HostsList, tip::Tip, title_dialog::TitleDialog};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Flex, Layout},
    prelude::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Clear,
    DefaultTerminal, Frame,
};

#[derive(Debug, Default, PartialEq)]

enum Mode {
    #[default]
    Normal,
    EditingTitle,
    EditingHosts,
}

#[derive(Debug, Default)]
pub struct App<'a> {
    running: bool,
    hosts_list: HostsList,
    editor: Editor<'a>,
    tip: Message<'a>,
    edit_list_message_line: Line<'a>,
    edit_hosts_message_line: Line<'a>,
    mode: Mode,
    title_dialog: TitleDialog<'a>,
    show_title_dialog: bool,
}

fn title_dialog_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

impl App<'static> {
    pub fn new() -> Self {
        let hosts_list = HostsList::new();
        let editor = Editor::new();
        let tip = Tip::new();
        let edit_list_message_line = Line::from(vec![
            Span::raw("Press "),
            Span::styled("^N", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to add new hosts, "),
            Span::styled("^D", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to delete one, "),
            Span::styled("^C to quit", Style::default().add_modifier(Modifier::BOLD)),
        ]);
        let edit_hosts_message_line = Line::from(vec![
            Span::raw("Press "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to inert new line "),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to exit edit"),
        ]);
        let edit_title_message_line = Line::from(vec![
            Span::raw("Press "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to add new hosts,  "),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to exit dialog"),
        ]);
        let title_dialog = TitleDialog::new();
        App {
            running: false,
            hosts_list,
            editor,
            tip,
            edit_list_message_line,
            edit_hosts_message_line,
            mode: Mode::Normal,
            title_dialog,
            show_title_dialog: false,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            if self.mode == Mode::Normal {
                self.tip.set_line(self.edit_list_message_line.clone());
            }
            terminal.draw(|frame| {
                self.draw(frame);
            })?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let frame_area = frame.area();
        let buf = frame.buffer_mut();
        let [main_area, message_area] =
            Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).areas(frame_area);
        let [left, right] =
            Layout::horizontal(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                .flex(Flex::Start)
                .areas(main_area);
        self.hosts_list.draw(left, buf);
        self.editor.draw(right, buf);
        self.tip.draw(message_area, buf);
        if self.show_title_dialog {
            self.draw_title_dialog(frame_area, frame);
        }
    }

    fn draw_title_dialog(&mut self, frame_area: Rect, frame: &mut Frame) {
        let buf = frame.buffer_mut();
        let area = title_dialog_area(frame_area, 60, 20);
        frame.render_widget(Clear, area);
        let buf = frame.buffer_mut();
        self.title_dialog.draw(area, buf);
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(event) if event.kind == KeyEventKind::Press => self.on_key_event(event),
            _ => {}
        }
        Ok(())
    }

    fn on_key_event(&mut self, event: KeyEvent) {
        if self.mode == Mode::EditingTitle {
            self.title_dialog.handle_event(event, || {
                self.show_title_dialog = false;
                self.mode = Mode::Normal;
            });
            return;
        }
        else if self.mode == Mode::EditingHosts {
            self.editor.handle_event(event, || {});
            return;
        }
        match (event.modifiers, event.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            (KeyModifiers::SHIFT, KeyCode::Char('n') | KeyCode::Char('N')) => {
                if self.mode == Mode::Normal {
                    self.show_title_dialog = true;
                    self.mode = Mode::EditingTitle;
                }
            },
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
