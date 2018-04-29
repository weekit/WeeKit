#![allow(dead_code)]

pub mod display;
pub mod draw;
pub mod event;
pub mod font;

mod input;
mod openvg;

use std::sync::{Arc, Mutex};

// applications should implement the Application trait.
pub trait Application {
    fn draw(&mut self, width: u32, height: u32) -> ();
    fn input(&mut self, event: &event::Event) -> ();
}

// main should be called from client applications to run the main event loop.
pub fn main<T: Application + 'static>(application: T) -> i64 {
    unsafe {
        APPLICATION = Some(Arc::new(Mutex::new(application)));
        INPUT_LISTENER = Some(input::Listener::new());
        return WKMain(draw_handler, input_handler);
    }
}

extern "C" {
    fn WKMain(f: extern "C" fn(u32, u32) -> (), e: extern "C" fn(u16, u16, i32) -> ()) -> i64;
}

static mut APPLICATION: Option<Arc<Mutex<Application>>> = None;

extern "C" fn draw_handler(width: u32, height: u32) -> () {
    unsafe {
        match APPLICATION {
            Some(ref arc) => {
                let arc = arc.clone();
                arc.lock().unwrap().draw(width, height);
            }
            None => {}
        }
    }
}

static mut INPUT_LISTENER: Option<input::Listener> = None;

extern "C" fn input_handler(t: u16, c: u16, v: i32) -> () {
    unsafe {
        match INPUT_LISTENER {
            Some(ref mut listener) => match APPLICATION {
                Some(ref arc) => listener.handle(t, c, v, arc.clone()),
                None => {}
            },
            None => {}
        }
    }
}
