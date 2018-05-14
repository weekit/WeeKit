#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

pub mod draw;
pub mod event;
pub mod font;

mod input;
mod openvg;

use std::sync::{Arc, Mutex};

/// Specifies required application capabilities.
pub trait Application {
    /// Draws the current application screen.
    fn draw(&mut self, width: u32, height: u32) -> ();

    /// Handles application events.
    fn handle(&mut self, _event: &event::Event) -> () {}
}

/// Starts the application and runs the main event loop.
pub fn main<T: Application + 'static>(application: T) -> i64 {
    unsafe {
        APPLICATION = Some(Arc::new(Mutex::new(application)));
        INPUT_LISTENER = Some(input::Listener::new());
        return WKMain(draw_handler, input_handler, tick_handler);
    }
}

extern "C" {
    fn WKMain(
        f: extern "C" fn(u32, u32) -> (),
        e: extern "C" fn(u16, u16, i32) -> (),
        t: extern "C" fn() -> (),
    ) -> i64;
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

extern "C" fn tick_handler() -> () {
    unsafe {
        match APPLICATION {
            Some(ref arc) => {
                let arc = arc.clone();
                arc.lock().unwrap().handle(&event::Event::new_tick());
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
