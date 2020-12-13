use hostname;
use md5;
use rand::RngCore;
#[cfg(any(target_os = "macos"))]
use sysctl::{Sysctl, SysctlError};

// https://github.com/rs/xid/blob/efa678f304ab65d6d57eedcb086798381ae22206/id.go#L117
pub fn get() -> [u8; 3] {
    let id = match machine_id().unwrap_or_default() {
        x if !x.is_empty() => x,
        _ => hostname::get()
            .map(|s| s.into_string().unwrap_or_default())
            .unwrap_or_default(),
    };

    let mut bytes = [0u8; 3];
    if id.is_empty() {
        // Fallback to random bytes
        rand::thread_rng().fill_bytes(&mut bytes);
    } else {
        bytes.copy_from_slice(&md5::compute(id)[0..3]);
    }
    bytes
}

// https://github.com/rs/xid/blob/efa678f304ab65d6d57eedcb086798381ae22206/hostid_linux.go
// Not checking "/sys/class/dmi/id/product_uuid" because normal users can't read it.
#[cfg(target_os = "linux")]
fn machine_id() -> std::io::Result<String> {
    use std::fs;
    // Get machine-id and remove the trailing new line.
    fs::read_to_string("/var/lib/dbus/machine-id")
        .or_else(|_| fs::read_to_string("/etc/machine-id"))
        .map(|s| s.trim_end().to_string())
}

// https://github.com/rs/xid/blob/efa678f304ab65d6d57eedcb086798381ae22206/hostid_darwin.go
#[cfg(target_os = "macos")]
fn machine_id() -> Result<String, SysctlError> {
    sysctl::Ctl::new("kern.uuid")?
        .value()
        .map(|v| v.to_string())
}

// https://github.com/rs/xid/blob/efa678f304ab65d6d57eedcb086798381ae22206/hostid_windows.go
#[cfg(target_os = "windows")]
fn machine_id() -> std::io::Result<String> {
    let hklm = winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE);
    let guid: String = hklm
        .open_subkey("SOFTWARE\\Microsoft\\Cryptography")?
        .get_value("MachineGuid")?;
    Ok(guid)
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
fn machine_id() -> std::io::Result<String> {
    // Fallback to hostname or a random value
    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "linux")]
    #[test]
    fn test_linux() {
        // machine-id has length 32
        assert_eq!(machine_id().unwrap().len(), 32);
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn test_macos() {
        // Ensure non empty string
        assert!(machine_id().unwrap().len() > 0);
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_windows() {
        // MachineGuid has length 36
        // xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
        assert_eq!(machine_id().unwrap().len(), 36);
    }
}
