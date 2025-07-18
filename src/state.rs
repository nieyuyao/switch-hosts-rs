use crate::data::{read_config, ConfigItem, ConfigItemType};

fn create_sys_item() -> ConfigItem {
    ConfigItem::new(
        String::from("system"),
        true,
        String::from("system"),
        ConfigItemType::System,
    )
}

pub struct State {
    pub selected_host_item_id: Option<String>,
    pub all_hosts_item_list: Vec<ConfigItem>,
    pub filter_input: String,
    pub is_filter: bool,
}

impl State {
    pub fn default() -> Self {
        State {
            selected_host_item_id: None,
            all_hosts_item_list: Vec::new(),
            filter_input: String::new(),
            is_filter: false,
        }
    }

    pub fn init(&mut self) {
        self.all_hosts_item_list.push(create_sys_item());
        if let Ok(mut config_item_list) = read_config() {
            self.all_hosts_item_list.append(&mut config_item_list);
        }
        self.selected_host_item_id = Some(self.all_hosts_item_list[0].id().to_owned());
    }

    pub fn filter() {
        // TODO: 更新filter_input
    }

    pub fn clear_filter() {}
}
