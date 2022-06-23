#![deny(unsafe_code)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
//! Globally unique sortable id generator. A Rust port of <https://github.com/rs/xid>.
//!
//! The binary representation is compatible with the Mongo DB 12-byte
//! [`ObjectId`][object-id]. The value consists of:
//!
//! - a 4-byte timestamp value in seconds since the Unix epoch
//! - a 3-byte value based on the machine identifier
//! - a 2-byte value based on the process id
//! - a 3-byte incrementing counter, initialized to a random value
//!
//! The string representation is 20 bytes, using a base32 hex variant with
//! characters `[0-9a-v]` to retain the sortable property of the id.
//!
//! See the original [`xid`] project for more details.
//!
//! ## Usage
//!
//! ```
//! println!("{}", xid::new()); //=> bva9lbqn1bt68k8mj62g
//! ```
//!
//! [`xid`]:  https://github.com/rs/xid
//! [object-id]: https://docs.mongodb.org/manual/reference/object-id/
mod generator;
mod id;
mod machine_id;
mod pid;

pub use id::{Id, ParseIdError};

/// Generate a new globally unique id.
#[must_use]
pub fn new() -> Id {
    generator::get().new_id()
}

#[cfg(test)]
mod tests {
    use super::*;

    // https://github.com/rs/xid/blob/efa678f304ab65d6d57eedcb086798381ae22206/id_test.go#L64
    #[test]
    fn test_new() {
        let mut ids = Vec::new();
        for _ in 0..10 {
            ids.push(new());
        }

        for i in 1..10 {
            // Test for uniqueness among all other 9 generated ids
            for j in 0..10 {
                if i != j {
                    assert_ne!(ids[i], ids[j]);
                }
            }

            let id = &ids[i];
            let prev_id = &ids[i - 1];
            // Check that timestamp was incremented and is within 5 seconds of the previous one
            // Panics if it went backwards.
            let secs = id.time().duration_since(prev_id.time()).unwrap().as_secs();
            assert!(secs <= 5);
            // Check that machine ids are the same
            assert_eq!(id.machine(), prev_id.machine());
            // Check that pids are the same
            assert_eq!(id.pid(), prev_id.pid());
            // Test for proper increment
            assert_eq!(id.counter() - prev_id.counter(), 1);
        }
    }
}
