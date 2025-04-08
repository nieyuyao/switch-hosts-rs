#![allow(dead_code)] 
#![allow(unused_variables)]

pub use app::App;

pub mod app;

pub mod list;

pub mod editor;

pub mod hosts;

pub mod data;

pub mod message;

pub mod title_dialog;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}
