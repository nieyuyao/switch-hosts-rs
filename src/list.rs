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

    pub fn init(&self) {}

    pub fn handle_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('h') | KeyCode::Left => self.select_none(),
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
            KeyCode::Char('g') | KeyCode::Home => self.select_first(),
            KeyCode::Char('G') | KeyCode::End => self.select_last(),
            _ => {}
        }
    }

    fn read_from_local(&self) {}

    fn add_item(&self) {}

    fn del_item(&self) {}

    fn select_item(&self) {}

    fn select_none(&self) {}

    fn select_next(&self) {}

    fn select_previous(&self) {}

    fn select_first(&self) {}

    fn select_last(&self) {}

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
