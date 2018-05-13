//! Representation of user input events.

/// Represents a user input event.
#[derive(Copy, Clone, Debug)]
pub struct TouchEvent {
    /// A unique integer for tracking a touch.
    pub slot: usize, 
    /// 1 = TouchDown, 2 = TouchMoved, 3 = TouchEnded
    pub kind: i32,   
    /// X position on the touchpad.
    pub x: i32,      
    /// Y position on the touchpad.
    pub y: i32,      
    /// Time of the event (seconds component).
    pub sec: i32,    
    /// Time of the event (microseconds component).
    pub usec: i32,   
}

impl TouchEvent {
    /// Creates an event.
    pub fn new(slot: usize, kind: i32, x: i32, y: i32) -> TouchEvent {
        TouchEvent {
            slot: slot,
            kind: kind,
            x: x,
            y: y,
            sec: 0,
            usec: 0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct KeyEvent {
    /// An integer code for the key.
    pub key: u16,
    /// 0=up, 1=down, 2=repeat.
    pub kind: u8,
    /// Time of the event (seconds component).
    pub sec: i32,    
    /// Time of the event (microseconds component).
    pub usec: i32,   
}

impl KeyEvent {
    /// Creates an event.
    pub fn new(key: u16, kind: u8) -> KeyEvent {
        KeyEvent {
            key: key,
            kind: kind,
            sec: 0,
            usec: 0,
        }
    }
}
