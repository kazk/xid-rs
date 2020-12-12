use std::fs;
use std::io;

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

// OS dependent machine ids. Only linux was confirmed.

// https://github.com/rs/xid/blob/efa678f304ab65d6d57eedcb086798381ae22206/hostid_linux.go
// Not checking "/sys/class/dmi/id/product_uuid" because normal users can't read it.
#[cfg(target_os = "linux")]
fn machine_id() -> io::Result<String> {
    fs::read_to_string("/var/lib/dbus/machine-id")
        .or_else(|_| fs::read_to_string("/etc/machine-id"))
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
fn machine_id() -> io::Result<String> {
    winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE)
        .open_subkey("SOFTWARE\\Microsoft\\Cryptography")?
        .get_value("MachineGuid")?
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
fn machine_id() -> io::Result<String> {
    // Fallback to hostname or a random value
    Ok("".to_string())
}
