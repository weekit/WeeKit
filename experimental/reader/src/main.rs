use libc::timeval;
use std::fs::File;
use std::io::Read;
use std::mem;
use std::slice;
use std::thread;

extern crate libc;

#[repr(C, packed)]
struct InputEvent {
    time: timeval,
    kind: u16,
    code: u16,
    value: i32,
}

fn main() {
    println!("Hello, world!");
    handle_inputs("/dev/input/touchscreen");
    handle_inputs("/dev/input/keyboard");
    loop {}
}

fn handle_inputs(filename: &'static str) {
    thread::spawn(move || {
        let mut f = File::open(filename).expect(&("unable to open ".to_owned() + filename));

        let mut input_event: InputEvent = unsafe { mem::zeroed() };
        let input_event_size = mem::size_of::<InputEvent>();
        loop {
            unsafe {
                let input_event_slice = slice::from_raw_parts_mut(
                    &mut input_event as *mut _ as *mut u8,
                    input_event_size,
                );
                f.read_exact(input_event_slice).unwrap();
                println!(
                    "{} {} {}",
                    input_event.kind, input_event.code, input_event.value
                );
            }
        }
    });
}
