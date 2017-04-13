use ioctls::{eviocgid, eviocgname, input_event, input_id};
use std::ffi::CString;
use std::fs::{File, read_dir};
use std::io::Read;
use std::io::Result as IoResult;
use std::mem;
use std::os::unix::io::AsRawFd;

pub fn get_device() -> Option<File> {
    let iter = if let Ok(iter) = read_dir("/dev/input") {
        iter
    } else {
        return None;
    };
    iter.filter_map(|r| {
        r.and_then(|entry| {
            let path = entry.path();
            File::open(&path)
        }).ok().and_then(|mut f| {
            get_input_id(&mut f).map(|i| (f, i))
        })
    }).filter(|&(_, id)| {
        id.vendor == 1204 && id.product == 48289
    }).map(|(f, _)| f).next()
}

fn get_input_id(f: &mut File) -> Option<input_id> {
    let fd = f.as_raw_fd();
    unsafe {
        let mut id: input_id = mem::uninitialized();
        if eviocgid(fd, &mut id as *mut input_id) == 0 {
            Some(id)
        } else {
            None
        }
    }
}

pub fn get_device_name(f: &mut File) -> String {
    let fd = f.as_raw_fd();
    const BUFLEN: usize = 255;
    let mut buf: Vec<u8> = vec![0; BUFLEN];
    let ret_code = unsafe {
        eviocgname(fd, buf.as_mut_ptr(), BUFLEN - 1)
    };
    if ret_code < 0 {
        panic!("Invalid eviocgname ioctl")
    }
    while let Some(&0) = buf.last() {
        buf.pop();
    }
    CString::new(buf)
        .unwrap()
        .into_string()
        .unwrap()
}

pub fn read_input_event<R: Read>(mut r: R) -> IoResult<input_event> {
    let mut buf: Vec<u8> = vec![0; mem::size_of::<input_event>()];
    r.read_exact(&mut buf)?;
    let mut ev: input_event = unsafe { mem::uninitialized() };
    ev.clone_from(unsafe {
        mem::transmute(buf.as_mut_ptr())
    });
    Ok(ev)
}
