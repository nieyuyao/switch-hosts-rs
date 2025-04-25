use std::{
    fmt::Display,
    fs,
    os::unix::fs::PermissionsExt,
    process::Command,
};

use crate::util::Result;

const CONTENT_START: &str = "# --- SWITCHHOSTS_RS_CONTENT_START ---";

const CONTENT_END: &str = "# --- SWITCHHOSTS_RS_CONTENT_END ---";

fn get_sys_hosts_path() -> String {
    String::from("/etc/hosts")
}

fn check_access() -> bool {
    let hosts_path = get_sys_hosts_path();
    match fs::metadata(&hosts_path) {
        Ok(meta) => meta.permissions().readonly(),
        Err(_) => false,
    }
}

pub fn write_sys_hosts(appended: impl Into<String> + AsRef<[u8]>) -> Result<()> {
    let hosts_path = get_sys_hosts_path();
    let hosts_content = generate_sys_hosts_content(appended.into());
    fs::write(&hosts_path, &hosts_content)?;
    Ok(())
}

pub fn set_sudo_permissions<'a>(password: impl Into<String> + Display) -> Result<()> {
    let sys_hosts_path = get_sys_hosts_path();
    let arg_str: String = format!(r#"echo "{}" | sudo -S chmod 777 {}"#, password, sys_hosts_path);
    let output = Command::new("sh")
        .arg("-c")
        .arg(arg_str)
        .output()?;

    if !output.status.success() {
        return  Err(color_eyre::eyre::Error::msg("Failed to execute sudo command"));
    }
    Ok(())
}

pub fn resume_permissions(
    password: impl Into<String> + Display,
    old_permission_mode: &str,
) -> Result<()> {
    let sys_hosts_path = get_sys_hosts_path();
    let arg_str: String = format!(r#"echo "{}" | sudo -S chmod {} {}"#, password, old_permission_mode, sys_hosts_path);
    let output = Command::new("sh")
        .arg("-c")
        .arg(arg_str)
        .output()?;

    if !output.status.success() {
        return  Err(color_eyre::eyre::Error::msg("Failed to execute sudo command"));
    }
    Ok(())
}

pub fn generate_sys_hosts_content(appended: String) -> String {
    let mut content = read_sys_hosts().unwrap();
    let start_index = content.find(CONTENT_START);
    let end_index = content.find(CONTENT_END);
    match [start_index, end_index] {
        [Some(start), Some(end)] => {
            let mut new_appended = String::from("\n");
            new_appended.push_str(&appended);
            new_appended.push_str("\n");
            content.replace_range((start + CONTENT_START.len())..end, new_appended.as_str());
        }
        _ => {
            content.push_str(&format!(
                "\n\n{}\n{}\n{}\n",
                CONTENT_START, appended, CONTENT_END
            ));
        }
    }
    content
}

pub fn write_sys_hosts_with_sudo(password: String, appended: String) -> Result<()> {
    let sys_hosts_path = get_sys_hosts_path();
    let metadata = fs::metadata(&sys_hosts_path)?;
    let old_mode = metadata.permissions().mode();
    let mask = 0o000777;
    let old_permission_mode = old_mode & mask;
    set_sudo_permissions(&password)?;
    let hosts_content = generate_sys_hosts_content(appended);
    fs::write(&sys_hosts_path, &hosts_content)?;
    resume_permissions(&password, format!("{:o}", old_permission_mode).as_str())?;
    Ok(())
}

pub fn read_sys_hosts() -> Result<String> {
    let hosts_path = get_sys_hosts_path();
    let content = fs::read(&hosts_path)
        .map(|buf| String::from_utf8(buf).unwrap_or(String::new()))
        .unwrap();
    Ok(content)
}
