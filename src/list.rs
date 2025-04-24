use std::cell::RefCell;
use std::rc::Rc;
use thiserror::Error;
use crate::data::{
    add_item, delete_item, read_config, read_item_data, update_config_item, write_item_data,
    ConfigItem,
};
use crate::hosts::{write_sys_hosts, write_sys_hosts_with_sudo};
use crate::observer::UpdateHostsContentSubject;
use crate::util::Result;
use crate::util::{find_mut_config_by_id, find_config_by_id, find_selected_index};
use ratatui::{
    prelude::{Buffer, Rect},
    style::{Style, Stylize},
    widgets::{Block, List, ListItem, ListState, StatefulWidget, Widget},
};
use uuid::Uuid;


#[derive(Error, Debug)]
pub enum SwitchHostsError {
    #[error("未找到配置文件")]
    NotFoundConfig,
    #[error("未知错误")]
    Unknown,
}

pub struct HostsList {
    item_list: Vec<ConfigItem>,
    state: ListState,
    enabled_ids: Vec<String>,
    selected: Option<String>,
    subject: Option<Rc<RefCell<UpdateHostsContentSubject>>>,
}

impl HostsList {
    pub fn new() -> Self {
        HostsList {
            item_list: Vec::<ConfigItem>::new(),
            enabled_ids: Vec::<String>::new(),
            selected: None,
            state: ListState::default(),
            subject: None,
        }
    }

    pub fn init(&mut self) {
        if let Ok(config_item_list) = read_config() {
            self.item_list = config_item_list;
        }
        if !self.item_list.is_empty() {
            self.selected = Some(self.item_list[0].id().to_owned());
            self.dispatch_update_hosts_content_subject();
        }
    }

    pub fn add_item(&mut self, title: String, content: String) -> Result<()> {
        let id = Uuid::new_v4().to_string();
        add_item(id.clone(), title.clone(), content).and_then(|_| {
            let item = ConfigItem::new(id.clone(), false, title);
            self.item_list.push(item);
            if self.item_list.len() == 1 {
                self.selected = Some(id);
            }
            Ok(())
        })
    }

    pub fn delete_current_item(&mut self) -> Result<()> {
        if let Some(id) = &self.selected {
            self.delete_item(id.to_owned())?
        }
        Ok(())
    }

    pub fn delete_item(&mut self, id: String) -> Result<()> {
        delete_item(&id).and_then(|_| {
            if let Some(idx) = self
                .item_list
                .iter()
                .position(|item| item.id().to_owned() == id)
            {
                self.item_list.remove(idx);
                if id == self.get_selected_id().clone().unwrap_or("".to_owned()) {
                    if self.item_list.len() > 0 {
                        self.selected = Some(self.item_list[0].id().to_owned());
                    } else {
                        self.selected = None;
                    }
                }
            }
            Ok(())
        })?;
        Ok(())
    }

    pub fn toggle_on_off(&mut self, password: Option<String>, mut callback: impl FnMut(bool) -> ()) -> Result<()> {
        let id = self.selected.clone().unwrap_or("".to_owned());
        let config = find_config_by_id(&self.item_list, &id).ok_or(SwitchHostsError::NotFoundConfig)?;
        let config_title = config.title().to_owned();
        let on = !config.is_on();
        let hosts_content = self.generate_hosts_content(&id, on)?;
        if password.is_none() && write_sys_hosts(hosts_content.clone()).is_err() {
            callback(true);
            return Ok(());
        } else if write_sys_hosts_with_sudo(password.clone().unwrap_or("".to_owned()), hosts_content).is_err() {
            callback(true);
            return Ok(());
        }
        update_config_item(
            id.clone(),
            &ConfigItem::new(id.clone(), on, config_title),
        )?;
        callback(false);
        let config = find_mut_config_by_id(&mut self.item_list, &id).unwrap();
        config.on_off(on);
        Ok(())
    }

    pub fn get_selected_id(&self) -> &Option<String> {
        &self.selected
    }

    pub fn toggle_previous(&mut self) {
        if self.selected.is_none() {
            return;
        }
        if let Some(idx) = self
            .item_list
            .iter()
            .position(|item| return item.id() == self.selected.clone().unwrap().as_str())
        {
            if idx >= 1 {
                self.selected = Some(self.item_list[idx - 1].id().to_owned());
                self.dispatch_update_hosts_content_subject();
            }
        }
    }

    pub fn toggle_next(&mut self) {
        if self.selected.is_none() {
            return;
        }
        if let Some(idx) = self
            .item_list
            .iter()
            .position(|item| return item.id() == self.selected.clone().unwrap().as_str())
        {
            if idx + 1 < self.item_list.len() {
                self.selected = Some(self.item_list[idx + 1].id().to_owned());
                self.dispatch_update_hosts_content_subject();
            }
        }
    }

    pub fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new();
        block.render(area, buf);
        let block = Block::bordered()
            .style(Style::new().white().on_black().bold())
            .title("Hosts List");
        let items: Vec<ListItem> = self
            .item_list
            .iter()
            .map(|hosts_item| ListItem::from(hosts_item))
            .collect();
        let list = List::new(items).block(block).highlight_symbol(">");
        self.state.select(find_selected_index(
            &self.item_list,
            &self.selected.clone().unwrap_or("".to_owned()),
        ));
        StatefulWidget::render(list, area, buf, &mut self.state);
    }

    pub fn generate_hosts_content(&self, toggled_id: &String, toggled: bool) -> Result<String> {
        let enabled = self
            .item_list
            .iter()
            .filter(|item| {
                if item.id() == toggled_id {
                    return toggled;
                }
                item.is_on()
            })
            .collect::<Vec<_>>();
        let mut hosts_content = String::new();
        for item in enabled {
            let id = item.id();
            let item_content = read_item_data(id)?;
            hosts_content.push_str(&item_content);
        }
        Ok(hosts_content)
    }

    pub fn inject_subject(&mut self, subject: Rc<RefCell<UpdateHostsContentSubject>>) {
        self.subject.get_or_insert(subject);
    }

    pub fn dispatch_update_hosts_content_subject(&self) {
        if let Some(s) = self.subject.clone() {
            s.borrow()
            .notify(self.selected.clone().unwrap_or("".to_owned()).as_str());
        }
    }
}
