//! Representation of user input events.

use std::time::Duration;

pub enum Event {
    Touch(Touch, Duration),
    Key(Key, Duration),
    Tick(Duration),
}

impl Event {
    /// Creates a new Touch event.
    pub fn new_touch(t: Touch) -> Event {
        let now = Duration::new(0, 0);
        Event::Touch(t, now)
    }
    /// Creates a new Key event.
    pub fn new_key(k: Key) -> Event {
        let now = Duration::new(0, 0);
        Event::Key(k, now)
    }
    /// Creates a new Tick event.
    pub fn new_tick() -> Event {
        let now = Duration::new(0, 0);
        Event::Tick(now)
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
