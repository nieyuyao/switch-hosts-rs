use crate::editor::Editor;
use crate::list::HostsList;
use crate::password_input::PasswordInput;
use crate::popup::Popup;
use crate::tip::Tip;
use crate::hosts_title_input::TitleInput;
use crate::util::Result;
use crate::{observer::Subject};
use crossterm::event::KeyEventKind;
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
        MouseEventKind,
    },
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::error;
use ratatui::{
    layout::{Constraint, Flex, Layout},
    prelude::Rect,
    widgets::Clear,
    DefaultTerminal, Frame,
};
use std::time::{Duration, Instant};
use std::{cell::RefCell, io, rc::Rc};

const MOUSE_SCROLL_THROTTLE_INTERVAL: u128 = 100;

const POPUP_VISIBLE_INTERVAL: u128 = 600;

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
    mode: Mode,
    hosts_title_input: TitleInput<'a>,
    hosts_hosts_title_input: bool,
    show_password_input: bool,
    password_input: PasswordInput<'a>,
    instant: Instant,
    popup_instant: Instant,
    cached_password: Option<String>,
    popup: Popup,
    show_popup: bool,
    popup_text: String
}

fn title_input_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(3)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = horizontal.areas(vertical.areas::<1>(area)[0]);
    area
}


fn popup_area(area: Rect, length: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(3)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Length(length)]).flex(Flex::Center);
    let [area] = horizontal.areas(vertical.areas::<1>(area)[0]);
    area
}


impl App<'static> {
    pub fn new() -> Self {
        let mut hosts_list = HostsList::new();
        let editor = Rc::new(RefCell::new(Editor::new()));
        let tip = Tip::new();
        let popup = Popup();
        let hosts_title_input = TitleInput::new();
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
            mode: Mode::Normal,
            hosts_title_input,
            hosts_hosts_title_input: false,
            show_password_input: false,
            password_input,
            instant: Instant::now(),
            popup_instant: Instant::now(),
            cached_password: None,
            popup,
            show_popup: false,
            popup_text: String::from(""),
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
                self.tip.show_line(0);
            } else if self.mode == Mode::EditingTitle {
                self.tip.show_line(1);
            } else if self.mode == Mode::EditingHosts {
                self.tip.show_line(2);
            }
            if self.show_popup && self.popup_instant.elapsed().as_millis() > POPUP_VISIBLE_INTERVAL {
                self.show_popup = false
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
        if self.hosts_hosts_title_input {
            self.draw_title_input(frame_area, frame);
        }
        if self.show_password_input {
            self.draw_password_input(frame_area, frame);
        }
        if self.show_popup {
            self.draw_popup(frame_area, frame);
        }
    }

    fn draw_title_input(&mut self, frame_area: Rect, frame: &mut Frame) {
        let buf = frame.buffer_mut();
        let area = title_input_area(frame_area, 60, 20);
        frame.render_widget(Clear, area);
        let buf = frame.buffer_mut();
        self.hosts_title_input.draw(area, buf);
    }

    fn draw_password_input(&mut self, frame_area: Rect, frame: &mut Frame) {
        let buf = frame.buffer_mut();
        let area = title_input_area(frame_area, 60, 20);
        frame.render_widget(Clear, area);
        let buf = frame.buffer_mut();
        self.password_input.draw(area, buf);
    }


    fn draw_popup(&mut self, frame_area: Rect, frame: &mut Frame) {
        let buf = frame.buffer_mut();
        let area = popup_area(frame_area,self.popup_text.len() as u16 + 4);
        frame.render_widget(Clear, area);
        let buf = frame.buffer_mut();
        self.popup.draw(self.popup_text.clone(), area, buf);
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(20))? {
            match event::read() {
                Ok(Event::Key(event)) => {
                    if event.kind == KeyEventKind::Press {
                        self.on_key_event(event)?;
                    }
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
        }
        Ok(())
    }

    fn update_show_password_input(&mut self, res: Result<()>) {
        match res {
            Ok(_) => {
                self.mode = Mode::Normal;
                self.show_password_input = false;
            }
            Err(e) => {
                if e.to_string() == String::from("no permission") {
                    if !self.show_popup {
                        self.show_popup = true;
                        self.popup_instant = Instant::now();
                        self.popup_text = String::from("没有写入 Hosts 文件的权限");
                    }
                    if cfg!(target_os = "windows") {
                        return;
                    }
                    self.mode = Mode::InputPassword;
                    self.show_password_input = true;
                } else {
                    error!("{e}");
                }
            }
        }
    }

    fn handle_event(&mut self, event: KeyEvent) -> Result<()> {
        match (event.modifiers, event.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            (KeyModifiers::SHIFT, KeyCode::Char('n') | KeyCode::Char('N')) => {
                if self.mode == Mode::Normal {
                    self.hosts_hosts_title_input = true;
                    self.mode = Mode::EditingTitle;
                }
            }
            (KeyModifiers::SHIFT, KeyCode::Char('d') | KeyCode::Char('D')) => {
                self.hosts_list.delete_current_item();
            }
            (KeyModifiers::SHIFT, KeyCode::Char('t') | KeyCode::Char('T')) => {
                self.hosts_list.move_to_top();
            }
            (KeyModifiers::SHIFT, KeyCode::Char('b') | KeyCode::Char('B'))  => {
                 self.hosts_list.move_to_bottom();
            }
            (KeyModifiers::SHIFT, KeyCode::Up) => {
               self.hosts_list.move_to_previous();
            }
            (KeyModifiers::SHIFT, KeyCode::Down) => {
                self.hosts_list.move_to_next();
            }
            (KeyModifiers::SHIFT, KeyCode::Char('m') | KeyCode::Char('M')) => {
                if self.mode == Mode::Normal {
                    let selected = self.hosts_list.get_selected_item().unwrap();
                    if selected.id() != "system" {
                        self.hosts_title_input.set_text(selected.title().clone(), false);
                        self.hosts_hosts_title_input = true;
                        self.mode = Mode::EditingTitle;
                    }
                 
                }
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
                self.update_show_password_input(res);
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

    fn on_key_event(&mut self, event: KeyEvent) -> Result<()> {
        if self.mode == Mode::EditingTitle {
            self.hosts_title_input.handle_event(event, |res| {
                match res {
                    (true, None, _) => {
                        self.mode = Mode::Normal;
                        self.hosts_hosts_title_input = false;
                    }
                    (true, Some(title), is_new) => {
                        self.mode = Mode::Normal;
                        self.hosts_hosts_title_input = false;
                        if is_new {
                            self.hosts_list.add_item(title, "".to_owned());
                        } else {
                            self.hosts_list.update_item_title(title);
                        }
                    }
                    _ => {}
                }
            });
            return Ok(());
        } else if self.mode == Mode::EditingHosts {
            let res = self.editor.borrow_mut().handle_event(event);

            match res {
                None => {
                    return Ok(());
                }
                Some(quit) => {
                    let mut old_mode = Mode::EditingHosts;
                    if quit {
                        self.mode = Mode::Normal;
                        old_mode = Mode::Normal;
                    }
                    let toggled_res = self
                        .hosts_list
                        .toggle_on_off(self.cached_password.clone(), true);
                    self.update_show_password_input(toggled_res);
                    self.mode = old_mode;
                }
            };
        } else if self.mode == Mode::InputPassword {
            let res = self.password_input.handle_event(event);
            match res {
                (true, None) => {
                    self.mode = Mode::Normal;
                    self.show_password_input = false;
                }
                (true, password) => {
                    self.cached_password = password.clone();
                    let res = self.hosts_list.toggle_on_off(password, false);
                    self.update_show_password_input(res);
                }
                _ => {}
            };
            return Ok(());
        }
        return self.handle_event(event);
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
