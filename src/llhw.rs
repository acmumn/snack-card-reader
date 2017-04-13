use ioctls::{eviocgid, input_id};
use std::io::Result as IoResult;
use std::fs::{File, read_dir};
use std::mem::uninitialized;
use std::os::unix::io::{AsRawFd, RawFd};

pub fn get_devices() -> Vec<input_id> {
    let iter = if let Ok(iter) = read_dir("/dev/input") {
        iter
    } else {
        return Vec::new();
    };
    iter.filter_map(|r| {
        r.and_then(|entry| File::open(entry.path()))
            .ok()
            .and_then(get_input_id)
    }).collect()
}

pub fn get_input_id(f: File) -> Option<input_id> {
    let fd = f.as_raw_fd();
    unsafe {
        let id: input_id = uninitialized();
        if eviocgid(fd, &mut id as *mut input_id) == 0 {
            Some(id)
        } else {
            None
        }
    }
}
