use crate::data::{ConfigItem};


pub struct State {
    pub selected_host_item_id: String,
    pub all_hosts_item_list: Vec<ConfigItem>,
    pub filter_hosts_item_list: Vec<ConfigItem>,
    pub filter_input: String,
    pub is_filter: bool
}

impl State {
    pub fn default() -> Self {
        State {
            selected_host_item_id: String::new(),
            all_hosts_item_list: Vec::new(),
            filter_hosts_item_list: Vec::new(),
            filter_input: String::new(),
            is_filter: false,
        }
    }

    pub fn filter() {}

    pub fn clear_filter() {}
}