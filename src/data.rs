use std::{env, fmt::Display, fs, path::PathBuf};
use color_eyre::eyre::{Error, Ok};
use serde_json::{Map, Value};

const SWITCH_HOSTS_RS_DIR: &str = ".SwitchHostsRs";

pub fn get_home_dir() -> Option<PathBuf> {
    env::var_os("HOME").map(Into::into)
}

pub fn get_switch_hosts_rs_dir() -> Option<PathBuf> {
    get_home_dir().map(|buf| {
        buf.join(SWITCH_HOSTS_RS_DIR)
    })
}

pub fn get_config_path() -> Option<PathBuf> {
    get_switch_hosts_rs_dir().map(|buf| {
        buf.join("config.json")
    })
}

pub fn get_data_dir() -> Option<PathBuf> {
    get_switch_hosts_rs_dir().map(|buf| {
        buf.join("data")
    })
}

pub  fn check_switch_host_rs_dir_exist() -> Result<(), Error> {
    let dir = get_switch_hosts_rs_dir().unwrap();
    if !fs::exists(&dir)? {
        fs::create_dir(&dir)?;
    }
    Ok(())
}

pub  fn check_data_dir_exist() -> Result<(), Error> {
    let dir = get_data_dir().unwrap();
    if !fs::exists(&dir)? {
        fs::create_dir(&dir)?;
    }
    Ok(())
}


pub fn read_item_data(id: impl Display) -> Result<String, Error> {
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

pub fn write_item_data(id: impl Display, content: String) -> Result<(), Error> {
    check_switch_host_rs_dir_exist()?;
    check_data_dir_exist()?;
    let data_dir = get_data_dir().unwrap();
    let file_name = &data_dir.join(format!("{}.txt", id));
    fs::write(file_name, content)?;
    Ok(())
}

pub fn read_config() -> Result<Map<String, Value>, Error> {
    check_switch_host_rs_dir_exist()?;
    let path = get_config_path().unwrap();
    let empty =  Map::new();
    if !fs::exists(&path)? {
        fs::write(&path, "")?;
        Ok(empty)
    } else {
        let content = String::from_utf8(fs::read(&path)?).unwrap();
        serde_json::from_str(&content).map_or(Ok(empty.clone()), |value: Value| {
            let parsed = match value {
                Value::Object(map) => map,
                _ => empty
            };
            Ok(parsed)
        })
    }
}

pub fn write_config(content: impl Into<String> + AsRef<[u8]>) -> Result<(), Error>  {
    check_switch_host_rs_dir_exist()?;
    let path = get_config_path().unwrap();
    fs::write(& path, &content)?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use color_eyre::eyre::Ok;

    use super::*;

    #[test]
    fn test_read_config() -> Result<(), Error> {
        write_config("")?;
        let data = r#"
            {
                "a":"abc",
                "b":"bbc"
            }
        "#;
        write_config(data)?;
        let config = read_config()?;
        let val_a = match config.get(&"a".to_owned()).unwrap()  {
            Value::String(s) => s.as_str(),
            _ => ""
        };
        let val_b = match config.get(&"b".to_owned()).unwrap()  {
            Value::String(s) => s.as_str(),
            _ => ""
        };
        assert_eq!(val_a, "abc");
        assert_eq!(val_b, "bbc");
        Ok(())
    }

    #[test]
    fn test_write_config() -> Result<(), Error> {
        write_config("")?;
        let data = r#"
            {
                "a":"abc",
                "b":"bbc"
            }
        "#;
        let content = serde_json::to_string(data)?;
        write_config(content)?;
        Ok(())
    }
}