#![allow(dead_code)] 
#![allow(unused_variables)]
#![allow(unused_must_use)]

pub use app::App;

pub mod app;

pub mod list;

pub mod editor;

pub mod hosts;

pub mod data;

pub mod tip;

pub mod title_input;

pub mod message;

pub mod util;

pub mod password_input;

pub mod single_line_textarea;

pub mod observer;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}
