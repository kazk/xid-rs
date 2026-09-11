#![cfg(unix)]

use std::{
    fs::File,
    io::{Read, Write},
    os::unix::io::FromRawFd,
};

fn new_in_child() -> xid::Id {
    let mut fds = [0; 2];
    assert_eq!(unsafe { libc::pipe(fds.as_mut_ptr()) }, 0);
    match unsafe { libc::fork() } {
        -1 => panic!("fork failed"),
        0 => {
            let mut w = unsafe { File::from_raw_fd(fds[1]) };
            w.write_all(xid::new().as_bytes()).unwrap();
            unsafe { libc::_exit(0) }
        }
        pid => {
            unsafe { libc::close(fds[1]) };
            let mut buf = Vec::new();
            unsafe { File::from_raw_fd(fds[0]) }
                .read_to_end(&mut buf)
                .unwrap();
            unsafe { libc::waitpid(pid, std::ptr::null_mut(), 0) };
            xid::Id::from_bytes(&buf).unwrap()
        }
    }
}

#[test]
fn test_fork() {
    let parent = xid::new();
    let child1 = new_in_child();
    let child2 = new_in_child();

    assert_eq!(parent.machine(), child1.machine());
    assert_ne!(parent.pid(), child1.pid());
    assert_ne!(parent, child1);
    assert_ne!(child1, child2);
}
