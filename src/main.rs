#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

use std::{io, panic};

use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::DefaultTerminal;

mod app;

mod list;

mod editor;

mod hosts;

mod data;

mod tip;

mod hosts_title_input;

mod popup;

mod util;

mod password_input;

mod single_line_textarea;

mod observer;

mod logger;

mod filter;

mod state;

mod filter_result;

use app::App;

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
    crossterm::execute!(stdout, EnterAlternateScreen)?;
    terminal::enable_raw_mode();
    let terminal = ratatui::init();
    Ok(terminal)
}

fn restore_terminal() -> color_eyre::Result<()> {
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, LeaveAlternateScreen)?;
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
