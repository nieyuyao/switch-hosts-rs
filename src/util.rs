use ratatui::style::{Modifier, Style};
use tui_textarea::TextArea;

use crate::data::ConfigItem;

pub fn find_mut_config_by_id<'a>(
    list: &'a mut Vec<ConfigItem>,
    id: &String,
) -> Option<&'a mut ConfigItem> {
    list.iter_mut().find(|item| {
        return item.id().to_owned() == id.to_owned();
    })
}
pub fn find_config_by_id<'a>(
    list: &'a Vec<ConfigItem>,
    id: &String,
) -> Option<&'a ConfigItem> {
    list.iter().find(|item| {
        return item.id().to_owned() == id.to_owned();
    })
}

pub fn find_selected_index(list: &Vec<ConfigItem>, id: &String) -> Option<usize> {
    list.iter().position(|item| item.id() == id)
}

pub type Result<T> = std::result::Result<T, color_eyre::eyre::Error>;

pub fn create_new_textarea<'a>(place_holder: impl Into<String>) -> TextArea<'a> {
    let mut textarea = TextArea::default();
    textarea.set_cursor_line_style(Style::default());
    textarea.set_placeholder_text(place_holder);
    textarea.set_placeholder_style(Style::default().add_modifier(Modifier::ITALIC));
    textarea
}
