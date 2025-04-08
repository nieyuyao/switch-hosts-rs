use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize},
    widgets::{Block, Borders, Widget},
};
use tui_textarea::TextArea;

#[derive(Debug, Default)]
pub struct TitleDialog<'a> {
    text_area: TextArea<'a>,
}

impl<'a> TitleDialog<'a> {
    pub fn new() -> Self {
        let text_area = TextArea::default();
        return TitleDialog { text_area };
    }

    pub fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .bg(Color::Blue)
            .title(String::from("Hosts title"));
        self.text_area.set_block(block);
        self.text_area.render(area, buf);
    }
}
