use std::{
    fmt::Display,
    fs,
    io::Write,
    os::unix::fs::PermissionsExt,
    process::{Command, Stdio},
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

pub fn write_sys_hosts(content: impl Into<String> + AsRef<[u8]>) -> Result<()> {
    let hosts_path = get_sys_hosts_path();
    fs::write(&hosts_path, &content)?;
    Ok(())
}

pub fn set_sudo_permissions<'a>(password: impl Into<String> + Display) -> Result<()> {
    let sys_hosts_path = get_sys_hosts_path();
    let mut command = Command::new("sudo");
    let args = ["-S", "chmod", "777", sys_hosts_path.as_str()];
    command.args(&args).stdin(Stdio::piped());
    let mut child = command.spawn()?;
    let mut stdin = child.stdin.take().unwrap();
    writeln!(stdin, "{}", password).expect("failed to write to stdin");
    child.wait_with_output()?;

    Ok(())
}

pub fn resume_permissions(
    password: impl Into<String> + Display,
    old_permission_mode: &str,
) -> Result<()> {
    let sys_hosts_path = get_sys_hosts_path();
    let mut command = Command::new("sudo");
    let args = ["-S", "chmod", old_permission_mode, sys_hosts_path.as_str()];
    command.args(&args).stdin(Stdio::piped());
    let mut child = command.spawn()?;
    let mut stdin = child.stdin.take().unwrap();
    writeln!(stdin, "{}", password).expect("failed to write to stdin");
    child.wait_with_output()?;
    Ok(())
}

pub fn write_sys_hosts_with_sudo(password: String, appended: String) -> Result<()> {
    let sys_hosts_path = get_sys_hosts_path();
    let metadata = fs::metadata(&sys_hosts_path)?;
    let old_mode = metadata.permissions().mode();
    let mask = 0o000777;
    let old_permission_mode = old_mode & mask;
    set_sudo_permissions(&password)?;

    let mut hosts_content = String::from_utf8(fs::read(&sys_hosts_path)?).unwrap();
    let start_index = hosts_content.find(CONTENT_START);
    let end_index = hosts_content.find(CONTENT_END);
    let a = start_index..end_index;
    match [start_index, end_index] {
        [Some(start), Some(end)] => {
            let mut new_appended = String::from("\n");
            new_appended.push_str(&appended);
            new_appended.push_str("\n");
            hosts_content.replace_range((start + CONTENT_START.len())..end, new_appended.as_str());
        }
        _ => {
            hosts_content.push_str(&format!("\n\n{}\n{}\n{}\n", CONTENT_START, appended, CONTENT_END));
        }
    }
    fs::write(&sys_hosts_path, &hosts_content)?;
    resume_permissions(&password, format!("{:o}", old_permission_mode).as_str())?;
    Ok(())
}

pub fn read_sys_hosts() -> Result<String> {
    let hosts_path = get_sys_hosts_path();
    let content = fs::read(&hosts_path).map(|buf| String::from_utf8(buf).unwrap_or(String::new())).unwrap();
    Ok(content)
}
