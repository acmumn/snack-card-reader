use ioctls::{eviocgid, input_event, input_id};
use std::fs::{File, read_dir};
use std::io::{Error, ErrorKind, Read, Result};
use std::mem;
use std::os::unix::io::AsRawFd;

pub fn get_device() -> Result<File> {
    read_dir("/dev/input")?.filter_map(|r| {
        r.and_then(|entry| {
            let path = entry.path();
            File::open(&path)
        }).ok().and_then(|mut f| {
            get_input_id(&mut f).map(|i| (f, i))
        })
    }).filter(|&(_, id)| {
        id.vendor == 1204 && id.product == 48289
    }).map(|(f, _)| f).next().ok_or(Error::new(ErrorKind::Other, "no such device"))
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

pub fn read_input_event<R: Read>(mut r: R) -> Result<input_event> {
    let mut buf: Vec<u8> = vec![0; mem::size_of::<input_event>()];
    r.read_exact(&mut buf)?;
    let mut ev: input_event = unsafe { mem::uninitialized() };
    ev.clone_from(unsafe {
        mem::transmute(buf.as_mut_ptr())
    });
    Ok(ev)
}
