#[cfg(target_os = "windows")]
fn get_sys_hosts_path() -> String {
    use std::env;   
    let windir = match env::var("windir") {
        Ok(val) => val,
        Err(_) => String::from(r"C:\WINDOWS\system32\drivers\etc\hosts")
    };
    return windir
}

#[cfg(not(target_os = "windows"))]
fn get_sys_hosts_path() -> String {
    String::from("/etc/hosts")
}

// read saved hosts from disk
pub fn write_hosts() {}


// write saved hosts from disk
pub fn read_hosts() {}