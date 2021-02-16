use std::{
    sync::atomic::{AtomicU32, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

use once_cell::sync::OnceCell;
use rand::RngCore;

use crate::{
    id::{Id, RAW_LEN},
    machine_id, pid,
};

#[derive(Debug)]
pub struct Generator {
    counter: AtomicU32,
    machine_id: [u8; 3],
    pid: [u8; 2],
}

pub fn get() -> &'static Generator {
    static INSTANCE: OnceCell<Generator> = OnceCell::new();

    INSTANCE.get_or_init(|| Generator {
        counter: AtomicU32::new(init_random()),
        machine_id: machine_id::get(),
        pid: pid::get().to_be_bytes(),
    })
}

impl Generator {
    pub fn new_id(&self) -> Id {
        self.with_time(&SystemTime::now())
    }

    fn with_time(&self, time: &SystemTime) -> Id {
        // Panic if the time is before the epoch.
        let unix_ts = time
            .duration_since(UNIX_EPOCH)
            .expect("Clock may have gone backwards");
        #[allow(clippy::clippy::cast_possible_truncation)]
        self.generate(unix_ts.as_secs() as u32)
    }

    fn generate(&self, unix_ts: u32) -> Id {
        let counter = self.counter.fetch_add(1, Ordering::SeqCst);

        let mut raw = [0_u8; RAW_LEN];
        // 4 bytes of Timestamp (big endian)
        raw[0..=3].copy_from_slice(&unix_ts.to_be_bytes());
        // 3 bytes of Machine ID
        raw[4..=6].copy_from_slice(&self.machine_id);
        // 2 bytes of PID
        raw[7..=8].copy_from_slice(&self.pid);
        // 3 bytes of increment counter (big endian)
        raw[9..].copy_from_slice(&counter.to_be_bytes()[1..]);

        Id(raw)
    }
}

// https://github.com/rs/xid/blob/efa678f304ab65d6d57eedcb086798381ae22206/id.go#L136
fn init_random() -> u32 {
    let mut bs = [0_u8; 3];
    rand::thread_rng().fill_bytes(&mut bs);
    u32::from_be_bytes([0, bs[0], bs[1], bs[2]])
}
