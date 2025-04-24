use ratatui::{style::{palette::material::{GREEN, WHITE}, Modifier, Style}, text::Line, widgets::ListItem};
use serde::Serialize;
use serde_json::Value;
use std::{env, fs, path::PathBuf, vec::Vec};

use crate::util::{find_config_by_id, find_mut_config_by_id};
use crate::util::Result;

const SWITCH_HOSTS_RS_DIR: &str = ".SwitchHostsRs";

#[derive(Clone, Debug, Default, Serialize)]
pub struct ConfigItem {
    id: String,
    on: bool,
    title: String,
}

impl ConfigItem {
    pub fn new(id: String, on: bool, title: String) -> Self {
        ConfigItem { id, on, title }
    }

    pub fn is_on(&self) -> bool {
        self.on
    }

    pub fn id(&self) -> &String {
        return &self.id;
    }

    pub fn on_off(&mut self, is_on: bool) {
        self.on = is_on;
    }

    pub fn title(&self) -> &String {
        &self.title
    }
}

impl From<&ConfigItem> for ListItem<'_> {
    fn from(value: &ConfigItem) -> Self {
        let line = if value.on {
            Line::styled(
                format!("✓ {}", value.title),
                Style::new().fg(GREEN.c100).add_modifier(Modifier::BOLD),
            )
        } else {
            Line::styled(format!("{}", value.title), WHITE)
        };

        ListItem::new(line)
    }
}


pub fn get_home_dir() -> Option<PathBuf> {
    env::var_os("HOME").map(Into::into)
}

pub fn get_switch_hosts_rs_dir() -> Option<PathBuf> {
    get_home_dir().map(|buf| buf.join(SWITCH_HOSTS_RS_DIR))
}

pub fn get_config_path() -> Option<PathBuf> {
    get_switch_hosts_rs_dir().map(|buf| buf.join("config.json"))
}

pub fn get_data_dir() -> Option<PathBuf> {
    get_switch_hosts_rs_dir().map(|buf| buf.join("data"))
}

pub fn check_switch_host_rs_dir_exist() -> Result<()> {
    let dir = get_switch_hosts_rs_dir().unwrap();
    if !fs::exists(&dir)? {
        fs::create_dir(&dir)?;
    }
    Ok(())
}

pub fn check_data_dir_exist() -> Result<()> {
    let dir = get_data_dir().unwrap();
    if !fs::exists(&dir)? {
        fs::create_dir(&dir)?;
    }
    Ok(())
}

pub fn read_item_data(id: &String) -> Result<String> {
    check_switch_host_rs_dir_exist()?;
    check_data_dir_exist()?;
    let data_dir = get_data_dir().unwrap();
    let file_name = &data_dir.join(format!("{}.txt", id));
    if !fs::exists(file_name)? {
        Ok("".to_owned())
    } else {
        Ok(fs::read_to_string(file_name)?)
    }
}

pub fn write_item_data(id: &String, content: String) -> Result<()> {
    check_switch_host_rs_dir_exist()?;
    check_data_dir_exist()?;
    let data_dir = get_data_dir().unwrap();
    let file_name = &data_dir.join(format!("{}.txt", id));
    fs::write(file_name, content)?;
    Ok(())
}

pub fn delete_item(id: &String) -> Result<()> {
    check_switch_host_rs_dir_exist()?;
    check_data_dir_exist()?;
    let data_dir = get_data_dir().unwrap();
    let file_name = &data_dir.join(format!("{}.txt", id));
    if fs::exists(file_name)? {
        match fs::remove_file(&file_name) {
            Ok(_) => delete_config_item(id),
            Err(err) => Err(err.into())
        }
    } else {
        Ok(())
    }
}

pub fn add_item(id: String, title: String, content: String) -> Result<()> {
    check_switch_host_rs_dir_exist()?;
    check_data_dir_exist()?;
    let data_dir = get_data_dir().unwrap();
    let file_name = &data_dir.join(format!("{}.txt", id.clone()));
    match fs::write(file_name, content) {
        Ok(_) => add_config_item(id, title),
        Err(err) => Err(err.into())
    }
}

pub fn read_config() -> Result<Vec<ConfigItem>> {
    check_switch_host_rs_dir_exist()?;
    let path = get_config_path().unwrap();
    let empty = Vec::new();
    if !fs::exists(&path)? {
        fs::write(&path, "")?;
        Ok(empty)
    } else {
        let content = String::from_utf8(fs::read(&path)?).unwrap();
        serde_json::from_str(&content).map_or(Ok(empty.clone()), |value: Value| match value {
            Value::Array(val) => Ok(val
                .iter()
                .map(|item| ConfigItem {
                    id: item["id"].as_str().unwrap().to_owned(),
                    on: item["on"].as_bool().unwrap_or(false),
                    title: item["title"].as_str().unwrap().to_owned(),
                })
                .collect()),
            _ => Ok(empty),
        })
    }
}

pub fn write_config(content: impl Into<String> + AsRef<[u8]>) -> Result<()> {
    check_switch_host_rs_dir_exist()?;
    let path = get_config_path().unwrap();
    fs::write(&path, &content)?;
    Ok(())
}

pub fn delete_config_item(id: &String) -> Result<()> {
    let mut config = read_config()?;
    if let Some(idx) = config.iter().position(|item| item.id() == id) {
        config.remove(idx);
        deserialize_and_write_config(&config)?;
    }
    Ok(())
}

pub fn deserialize_and_write_config(config: &Vec<ConfigItem>) ->  Result<()>  {
    let json= serde_json::to_string_pretty(&config)?;
    write_config(json)?;
    Ok(())
}

pub fn add_config_item(id: String, title: String) -> Result<()> {
    let mut config = read_config()?;
    match config.iter().position(|item| item.id == id) {
        Some(index) => {
            let item = config.get_mut(index).unwrap();
            item.title = title;
            deserialize_and_write_config(&config)?;
        },
        _ => {
            config.push(ConfigItem {
                id,
                title,
                on: false,
            });
            deserialize_and_write_config(&config)?;
        }
    }
    Ok(())
}

pub fn update_config_item(id: String, new_config: &ConfigItem) -> Result<()> {
    let mut config = read_config()?;
    if let Some(target)  = find_mut_config_by_id(&mut config, &id) {
        target.on = new_config.is_on();
        target.title = new_config.title().to_owned();
        let new_config_json = serde_json::to_string_pretty(&config)?;
        write_config(new_config_json)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use color_eyre::eyre::Ok;

    use super::*;

    #[test]
    fn test_read_writer_config() -> Result<()> {
        write_config("")?;
        let data = r#"
            [
                { "id": "a", "on": true, "title": "A" },
                { "id": "b", "on": false, "title": "B" }
            ]
        "#;
        write_config(data)?;
        let config = read_config()?;
        assert_eq!(config.len(), 2);
        assert_eq!(config[0].id, "a".to_owned());
        assert_eq!(config[1].id, "b".to_owned());
        Ok(())
    }
}
