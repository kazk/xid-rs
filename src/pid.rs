use std::fs;
use std::process;

use crc32fast::Hasher;

// 2 bytes of PID
// https://github.com/rs/xid/blob/efa678f304ab65d6d57eedcb086798381ae22206/id.go#L159
pub fn get() -> u16 {
    // https://github.com/rs/xid/blob/efa678f304ab65d6d57eedcb086798381ae22206/id.go#L105
    // > If /proc/self/cpuset exists and is not /, we can assume that we are in a
    // > form of container and use the content of cpuset xor-ed with the PID in
    // > order get a reasonable machine global unique PID.
    let pid = match fs::read("/proc/self/cpuset") {
        Ok(buff) if buff.len() > 1 => process::id() ^ crc32(&buff),
        _ => process::id(),
    };

    pid as u16
}

fn crc32(buff: &[u8]) -> u32 {
    let mut hasher = Hasher::new();
    hasher.update(buff);
    hasher.finalize()
}
