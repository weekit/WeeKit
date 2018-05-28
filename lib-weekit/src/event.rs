//! Representation of user input events.

use std::time::{Duration, Instant};

/// Represents applicatoin events.
pub enum Event {
    Touch(Touch, Duration),
    Key(Key, Duration),
    Tick(Duration),
}

fn elapsed() -> Duration {
    lazy_static! {
        static ref START: Instant = { Instant::now() };
    };
    START.elapsed()
}

impl Event {
    /// Creates a new Touch event.
    pub fn new_touch(t: Touch) -> Event {
        Event::Touch(t, elapsed())
    }
    /// Creates a new Key event.
    pub fn new_key(k: Key) -> Event {
        Event::Key(k, elapsed())
    }
    /// Creates a new Tick event.
    pub fn new_tick() -> Event {
        Event::Tick(elapsed())
    }
}

/// Represents a user input event.
#[derive(Copy, Clone, Debug)]
pub struct Touch {
    /// A unique integer for tracking a touch.
    pub slot: usize,
    /// 1 = TouchDown, 2 = TouchMoved, 3 = TouchEnded
    pub kind: i32,
    /// X position on the touchpad.
    pub x: i32,
    /// Y position on the touchpad.
    pub y: i32,
}

impl Touch {
    /// Creates an event.
    pub fn new(slot: usize, kind: i32, x: i32, y: i32) -> Touch {
        Touch {
            slot: slot,
            kind: kind,
            x: x,
            y: y,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Key {
    /// An integer code for the key.
    pub key: u16,
    /// 0=up, 1=down, 2=repeat.
    pub kind: u8,
}

impl Key {
    /// Creates an event.
    pub fn new(key: u16, kind: u8) -> Key {
        Key {
            key: key,
            kind: kind,
        }
    }
}
