#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

use std::{io, panic};

pub use app::App;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::DefaultTerminal;

pub mod app;

pub mod list;

pub mod editor;

pub mod hosts;

pub mod data;

pub mod tip;

pub mod hosts_title_input;

pub mod popup;

pub mod util;

pub mod password_input;

pub mod single_line_textarea;

pub mod observer;

pub mod logger;

fn init_hooks() -> color_eyre::Result<()> {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let _ = restore_terminal();
        original_hook(panic_info);
    }));
    Ok(())
}

fn init_terminal() -> color_eyre::Result<DefaultTerminal> {
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    terminal::enable_raw_mode();
    let terminal = ratatui::init();
    Ok(terminal)
}

fn restore_terminal() -> color_eyre::Result<()> {
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    terminal::disable_raw_mode();
    Ok(())
}

fn main() -> color_eyre::Result<()> {
    init_hooks()?;
    logger::init_logger();
    let mut terminal = init_terminal()?;
    let result = App::new().run(&mut terminal);
    restore_terminal()?;
    result
}
