use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::{Buffer, Rect},
    widgets::{List, ListItem, ListState, StatefulWidget},
    style::{
        Style,
        Modifier,
        palette::tailwind::{GREEN, WHITE}
    },
    text::Line
};

use crate::hosts;

#[derive(Debug, Default)]
pub struct HostsItem {
    // title of list item
    name: String,
    // is enable
    on: bool,
    // hosts content
    hosts: String,
}


impl HostsItem {
    fn create(name: String, on: bool, hosts: String) -> Self {
        HostsItem {
            name: String::new(),
            on: false,
            hosts: String::new(),
        }
    }
}

impl From<&HostsItem> for ListItem<'_> {
    fn from(value: &HostsItem) -> Self {
        let line = if value.on {
            Line::styled(format!("✓ {}", value.name), Style::new().fg(GREEN.c100).add_modifier(Modifier::BOLD))
        } else {
            Line::styled(format!("{}", value.name), WHITE)
        };

        ListItem::new(line)
    }
}

#[derive(Debug, Default)]
pub struct HostsList {
    data_list: Vec<HostsItem>,
    state: ListState,
    selected_item: Option<HostsItem>,
}


impl HostsList {
    pub fn new() -> Self {
        HostsList {
            data_list: Vec::<HostsItem>::new(),
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
            KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => {
                todo!();
            }
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
        let items: Vec<ListItem> = self
            .data_list
            .iter()
            .map(|hosts_item| {
                ListItem::from(hosts_item)
            })
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let list = List::new(items)
            .highlight_symbol(">");

        // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
        // same method name `render`.
        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}
