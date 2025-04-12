use std::{cell::RefCell, rc::Rc};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Flex, Layout},
    prelude::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Clear,
    DefaultTerminal, Frame,
};
use crate::{hosts::write_sys_hosts, message::Message, observer::UpdateHostsContentSubject};
use crate::{editor::Editor, list::HostsList, tip::Tip, title_input::TitleInput};
use crate::util::Result;


#[derive(Debug, Default, PartialEq)]

enum Mode {
    #[default]
    Normal,
    EditingTitle,
    EditingHosts,
}

pub struct App<'a> {
    running: bool,
    hosts_list: HostsList,
    editor: Rc<RefCell<Editor<'a>>>,
    tip: Tip<'a>,
    edit_list_message_line: Line<'a>,
    edit_hosts_message_line: Line<'a>,
    mode: Mode,
    title_input: TitleInput<'a>,
    show_title_input: bool,
    message: Message,
    show_message: bool,
    message_text: String,
}

fn title_input_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(3)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = horizontal.areas(vertical.areas::<1>(area)[0]);
    area
}

fn message_area(area: Rect) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(1)]).flex(Flex::End);
    let [area] = vertical.areas(area);
    area
}

impl App<'static> {
    pub fn new() -> Self {
        let mut hosts_list = HostsList::new();
        let editor = Rc::new(RefCell::new(Editor::new()));
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
        let title_input = TitleInput::new();
        let message = Message();
        let subject = Rc::new(RefCell::new(UpdateHostsContentSubject::new()));
        subject.borrow_mut().register(editor.clone());
        hosts_list.inject_subject(subject.clone());
        hosts_list.init();
        App {
            running: false,
            hosts_list,
            editor,
            tip,
            edit_list_message_line,
            edit_hosts_message_line,
            mode: Mode::Normal,
            title_input,
            show_title_input: false,
            message,
            show_message: false,
            message_text: String::from(""),
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
        let [main_area, tip_area] =
            Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).areas(frame_area);
        let [left, right] =
            Layout::horizontal(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                .flex(Flex::Start)
                .areas(main_area);
        self.hosts_list.draw(left, buf);
        self.editor.borrow_mut().draw(right, buf);
        self.tip.draw(tip_area, buf);
        if self.show_title_input {
            self.draw_title_input(frame_area, frame);
        }
        if self.show_message {
            self.draw_message(frame_area, frame);
        }
    }

    fn draw_title_input(&mut self, frame_area: Rect, frame: &mut Frame) {
        let buf = frame.buffer_mut();
        let area = title_input_area(frame_area, 60, 20);
        frame.render_widget(Clear, area);
        let buf = frame.buffer_mut();
        self.title_input.draw(area, buf);
    }

    fn draw_message(&mut self, frame_area: Rect, frame: &mut Frame) {
        let buf = frame.buffer_mut();
        let area = message_area(frame_area);
        frame.render_widget(Clear, area);
        let buf = frame.buffer_mut();
        self.message.draw(self.message_text.clone(), area, buf);
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        if let Ok(Event::Key(event)) = event::read() {
            if event.kind == KeyEventKind::Press {
                self.on_key_event(event)?;
            }
        }
        Ok(())
    }

    fn on_key_event(&mut self, event: KeyEvent) -> Result<()> {
        if self.mode == Mode::EditingTitle {
            self.title_input.handle_event(event, |res| {
                match res {
                    (true, None) => {
                        // 关闭
                        self.mode = Mode::Normal;
                        self.show_message = false;
                        self.show_title_input = false;
                    }
                    (true, Some(title)) => {
                        self.mode = Mode::Normal;
                        self.show_message = false;
                        self.show_title_input = false;
                        self.hosts_list
                            .add_item(title, "".to_owned())
                            .unwrap_or_else(|e| {
                                e.to_string();
                            })
                    }
                    (false, None) => {
                        self.show_message = false;
                    }
                    (false, Some(msg)) => {
                        self.show_message = true;
                        self.message_text = msg;
                    }
                }
            });
            return Ok(());
        } else if self.mode == Mode::EditingHosts {
            self.editor.borrow_mut().handle_event(event, |quit, payload| {
                match (quit, payload) {
                    (true, None) => {
                        self.mode = Mode::Normal;
                        self.show_message = false
                    },
                    (false, None) => {
                        self.show_message = true;
                        self.message_text = String::from("保存成功");
                    }
                    _ => {}
                }
            });
            return Ok(());
        }
        match (event.modifiers, event.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            (KeyModifiers::SHIFT, KeyCode::Char('n') | KeyCode::Char('N')) => {
                if self.mode == Mode::Normal {
                    self.show_title_input = true;
                    self.mode = Mode::EditingTitle;
                }
            }
            (_, KeyCode::Delete | KeyCode::Backspace) => {
                let _ = self.hosts_list.delete_current_item();
            }
            (_, KeyCode::Up) => {
                self.hosts_list.toggle_previous();
            }
            (_, KeyCode::Down) => {
                self.hosts_list.toggle_next();
            }
            (_, KeyCode::Enter) => {
                if let Ok(_) = self.hosts_list.toggle_on_off() {
                    let hosts_content = self.hosts_list.generate_hosts_content()?;
                    write_sys_hosts(hosts_content).unwrap_or_else(|e| {
                        panic!("{}", e.to_string());
                    });
                }
            }
            (_, KeyCode::Tab) => {
                if let Some(id) = self.hosts_list.get_selected_id() {
                    self.mode = Mode::EditingHosts;
                    self.editor.borrow_mut().set_id(id.to_owned());
                    self.editor.borrow_mut().activate();
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
