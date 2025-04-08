
use uuid::Uuid;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::{Buffer, Rect},
    style::{
        palette::tailwind::{GREEN, WHITE},
        Modifier, Style, Stylize,
    },
    text::Line,
    widgets::{Block, List, ListItem, ListState, StatefulWidget, Widget},
};
use crate::data::{delete_item, write_item_data};

#[derive(Debug, Default)]
pub struct HostsListItem {
    // title of list item
    name: String,
    // is enable
    on: bool,
    // hosts content
    hosts: String,
}

impl HostsListItem {
    fn create(name: String, on: bool, hosts: String) -> Self {
        HostsListItem {
            name: String::new(),
            on: false,
            hosts: String::new(),
        }
    }
}

impl From<&HostsListItem> for ListItem<'_> {
    fn from(value: &HostsListItem) -> Self {
        let line = if value.on {
            Line::styled(
                format!("✓ {}", value.name),
                Style::new().fg(GREEN.c100).add_modifier(Modifier::BOLD),
            )
        } else {
            Line::styled(format!("{}", value.name), WHITE)
        };

        ListItem::new(line)
    }
}

#[derive(Debug, Default)]
pub struct HostsList {
    data_list: Vec<HostsListItem>,
    state: ListState,
    selected_item: Option<HostsListItem>,
}

impl HostsList {
    pub fn new() -> Self {
        HostsList {
            data_list: Vec::<HostsListItem>::new(),
            selected_item: None,
            state: ListState::default(),
        }
    }

    pub fn handle_event(&mut self, event: KeyEvent) {
    }

    fn read_from_local(&self) {}

    // 添加hosts
    fn add_item(&self, content: String) {
        let id = Uuid::new_v4();
        write_item_data(id.to_string(), content).unwrap_or_else(|err| {
            todo!();
        })
    }

    // 删除hosts
    fn del_item(&self, id: String) {
        delete_item(id).unwrap_or_else(|err| {
            todo!();
        })
    }

    // 选中hosts
    fn select_item(&self) {}

    // 反选hosts
    fn select_none(&self) {}

    // toggle next hosts
    fn toggle_next(&self) {}

    // toggle prev hosts
    fn toggle_previous(&self) {}

    pub fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new();
        block.render(area, buf);

        let block = Block::bordered()
            .style(Style::new().white().on_black().bold())
            .title("Your hosts list ↓↓↓");
        let items: Vec<ListItem> = self
            .data_list
            .iter()
            .map(|hosts_item| ListItem::from(hosts_item))
            .collect();
        let list = List::new(items).block(block).highlight_symbol(">");
        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}


#[cfg(test)]
mod tests {
    use uuid::Uuid;

    #[test]
    pub fn test_uuid() {
        let id = Uuid::new_v4();
        let id2 = Uuid::new_v4();
        assert_ne!(id, id2);
    }
}