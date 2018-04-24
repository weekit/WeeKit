#![allow(dead_code)]

mod openvg;

pub mod input;
pub mod font;
pub mod display;
pub mod draw;

extern crate libc;

use std::sync::Arc;
use std::sync::Mutex;

pub trait Application {
    fn draw(&mut self, width: u32, height: u32) -> ();
    fn event(&mut self, event: &input::Event) -> ();
}

// main should be called from client applications to run the main event loop.
pub fn main<T: Application + 'static>(application: T) -> i64 {
    unsafe {
        APPLICATION = Some(Arc::new(Mutex::new(application)));
        TOUCH_PAD = Some(input::TouchPad::new());
        return WKMain(draw_handler_wrapper, event_handler_wrapper);
    }
}

#[link(name = "wee")]
extern "C" {
    fn WKMain(f: extern "C" fn(u32, u32) -> (), e: extern "C" fn(u16, u16, i32) -> ()) -> i64;
}

static mut APPLICATION: Option<Arc<Mutex<Application>>> = None;

static mut TOUCH_PAD: Option<input::TouchPad> = None;

fn draw(x: Arc<Mutex<Application>>, width: u32, height: u32) {
    let d = x.clone();
    let mut app = d.lock().unwrap();
    app.draw(width, height);
}

extern "C" fn draw_handler_wrapper(width: u32, height: u32) -> () {
    unsafe {
        match APPLICATION {
            Some(ref app) => draw(app.clone(), width, height),
            None => {}
        }
    }
}

extern "C" fn event_handler_wrapper(t: u16, c: u16, v: i32) -> () {
    unsafe {
        match TOUCH_PAD {
            Some(ref mut touchpad) => touchpad.handle(t, c, v, APPLICATION.clone()),
            None => {}
        }
    }
}