extern crate byteorder;
extern crate ioctl;
extern crate void;

use byteorder::{ByteOrder, NativeEndian};
use ioctl::input_event;
use ioctl::libc::timeval;
use std::fs::File;
use std::io::prelude::*;
use std::mem::size_of;
use void::{Void, ResultVoidErrExt};

fn main() {
    loop {
        let event_num = 1;
        println!("{:?}", run(event_num).void_unwrap_err());
    }
}

fn run(event_num: usize) -> Result<Void, std::io::Error> {
    let mut f = File::open(format!("/dev/input/event{}", event_num))?;
    loop {
        let ev = read_input_event(&mut f)?;
        println!("Got event: {:?}", ev);
    }
}

fn read_input_event<R: Read>(mut r: R) -> Result<input_event, std::io::Error> {
    let mut buf: Vec<u8> = vec![0; size_of::<input_event>()];
    assert_eq!(buf.len(), 16);
    r.read_exact(&mut buf)?;
    let tv_sec = NativeEndian::read_i32(&buf[0..4]);
    let tv_usec = NativeEndian::read_i32(&buf[4..8]);
    let _type = NativeEndian::read_u16(&buf[8..10]);
    let code = NativeEndian::read_u16(&buf[10..12]);
    let value = NativeEndian::read_i32(&buf[12..16]);
    Ok(input_event {
        time: timeval {
            tv_sec,
            tv_usec,
        },
        _type,
        code,
        value,
    })
}
