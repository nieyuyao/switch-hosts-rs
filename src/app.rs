use crate::editor::Editor;
use crate::list::HostsList;
use crate::password_input::PasswordInput;
use crate::tip::Tip;
use crate::title_input::TitleInput;
use crate::util::Result;
use crate::{message::Message, observer::Subject};
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
        MouseEventKind,
    },
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::{Constraint, Flex, Layout},
    prelude::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Clear,
    DefaultTerminal, Frame,
};
use std::time::Instant;
use std::{cell::RefCell, io, rc::Rc};

const MOUSE_SCROLL_THROTTLE_INTERVAL: u128 = 100;

#[derive(Debug, Default, PartialEq)]
enum Mode {
    #[default]
    Normal,
    EditingTitle,
    EditingHosts,
    InputPassword,
}

pub struct App<'a> {
    running: bool,
    hosts_list: HostsList,
    editor: Rc<RefCell<Editor<'a>>>,
    tip: Tip<'a>,
    edit_list_message_line: Line<'a>,
    edit_hosts_message_line: Line<'a>,
    edit_title_message_line: Line<'a>,
    mode: Mode,
    title_input: TitleInput<'a>,
    show_title_input: bool,
    message: Message,
    show_message: bool,
    message_text: String,
    show_password_input: bool,
    password_input: PasswordInput<'a>,
    instant: Instant,
    cached_password: Option<String>,
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
            Span::styled("Ctrl+N", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" 添加hosts "),
            Span::styled("Ctrl+D", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" 删除hosts "),
            Span::styled("Ctrl+C", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" 退出 "),
            Span::styled("Tab", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" 编辑hosts "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" 启用/禁用hosts "),
        ]);
        let edit_hosts_message_line = Line::from(vec![
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" 退出编辑模式 "),
            Span::styled("Shit+←/→", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" 移动光标至行首/行尾 "),
            Span::styled("Shit+o/g", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" 移动光标至顶部/底部 "),
            Span::styled("Ctrl+Z", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" 撤销编辑 "),
        ]);
        let edit_title_message_line = Line::from(vec![
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" 确定添加 "),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" 退出 "),
        ]);
        let title_input = TitleInput::new();
        let message = Message();
        let hosts_list_subject = Rc::new(RefCell::new(Subject::new()));
        hosts_list_subject.borrow_mut().register(editor.clone());
        hosts_list.inject_subject(hosts_list_subject.clone());
        hosts_list.init();
        let password_input = PasswordInput::new();
        App {
            running: false,
            hosts_list,
            editor,
            tip,
            edit_list_message_line,
            edit_hosts_message_line,
            edit_title_message_line,
            mode: Mode::Normal,
            title_input,
            show_title_input: false,
            message,
            show_message: false,
            message_text: String::from(""),
            show_password_input: false,
            password_input,
            instant: Instant::now(),
            cached_password: None,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        terminal::enable_raw_mode();
        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        self.running = true;
        while self.running {
            if self.mode == Mode::Normal {
                self.tip.set_line(self.edit_list_message_line.clone());
            } else if self.mode == Mode::EditingTitle {
                self.tip.set_line(self.edit_title_message_line.clone());
            } else if self.mode == Mode::EditingHosts {
                self.tip.set_line(self.edit_hosts_message_line.clone());
            }
            terminal.draw(|frame| {
                self.draw(frame);
            })?;
            self.handle_crossterm_events()?;
        }
        terminal::disable_raw_mode();
        crossterm::execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
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
        if self.show_password_input {
            self.draw_password_input(frame_area, frame);
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

    fn draw_password_input(&mut self, frame_area: Rect, frame: &mut Frame) {
        let buf = frame.buffer_mut();
        let area = title_input_area(frame_area, 60, 20);
        frame.render_widget(Clear, area);
        let buf = frame.buffer_mut();
        self.password_input.draw(area, buf);
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read() {
            Ok(Event::Key(event)) => {
                self.on_key_event(event)?;
            }
            Ok(Event::Mouse(e)) => {
                if self.instant.elapsed().as_millis() < MOUSE_SCROLL_THROTTLE_INTERVAL {
                    return Ok(());
                }
                self.instant = Instant::now();
                if e.kind == MouseEventKind::ScrollUp {
                    if self.mode == Mode::Normal {
                        self.hosts_list.toggle_previous();
                    } else if self.mode == Mode::EditingHosts {
                        self.editor.borrow_mut().cursor_move_up();
                    }
                } else if e.kind == MouseEventKind::ScrollDown {
                    if self.mode == Mode::Normal {
                        self.hosts_list.toggle_next();
                    } else if self.mode == Mode::EditingHosts {
                        self.editor.borrow_mut().cursor_move_down();
                    }
                }
            }
            _ => {}
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
            self.editor
                .borrow_mut()
                .handle_event(event, |quit, payload| match (quit, payload) {
                    (quit, None) => {
                        if quit {
                            self.mode = Mode::Normal;
                            self.show_message = false
                        } else {
                            self.show_message = true;
                            self.message_text = String::from("保存成功");
                        }
                        let res = self
                            .hosts_list
                            .toggle_on_off(self.cached_password.clone(), true);
                        // TODO: 这段逻辑有多处，考虑下怎么抽出来
                        // 直接抽成方法，会报错 closure requires unique access to `*self` but it is already borrowed
                        match res {
                            Ok(_) => {
                                self.mode = Mode::Normal;
                                self.show_password_input = false;
                            }
                            Err(e) => {
                                if e.to_string() == String::from("no permission") {
                                    self.mode = Mode::InputPassword;
                                    self.show_password_input = true;
                                }
                            }
                        }
                    },
                    _ => {}
                });
            return Ok(());
        } else if self.mode == Mode::InputPassword {
            self.password_input
                .handle_event(event, |quit, payload| match (quit, payload) {
                    (true, None) => {
                        self.mode = Mode::Normal;
                        self.show_password_input = false;
                    }
                    (true, password) => {
                        self.cached_password = password.clone();
                        let res = self.hosts_list.toggle_on_off(password, false);
                        match res {
                            Ok(_) => {
                                self.mode = Mode::Normal;
                                self.show_password_input = false;
                            }
                            Err(e) => {
                                if e.to_string() == String::from("no permission") {
                                    self.mode = Mode::InputPassword;
                                    self.show_password_input = true;
                                }
                            }
                        }
                    }
                    _ => {
                        self.mode = Mode::Normal;
                        self.show_password_input = false;
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
            (KeyModifiers::SHIFT, KeyCode::Char('d') | KeyCode::Char('D')) => {
                let _ = self.hosts_list.delete_current_item();
            }
            (_, KeyCode::Up) => {
                self.hosts_list.toggle_previous();
            }
            (_, KeyCode::Down) => {
                self.hosts_list.toggle_next();
            }
            (_, KeyCode::Enter) => {
                let res = self
                    .hosts_list
                    .toggle_on_off(self.cached_password.clone(), false);
                match res {
                    Ok(_) => {
                        self.mode = Mode::Normal;
                        self.show_password_input = false;
                    }
                    Err(e) => {
                        if e.to_string() == String::from("no permission") {
                            self.mode = Mode::InputPassword;
                            self.show_password_input = true;
                        }
                    }
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
