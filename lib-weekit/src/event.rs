#[derive(Copy, Clone, Debug)]
pub struct Event {
    pub slot: usize, // A unique integer for tracking a touch.
    pub kind: i32,   // 1 = TouchDown, 2 = TouchMoved, 3 = TouchEnded
    pub x: i32,      // X position on the touchpad.
    pub y: i32,      // Y position on the touchpad.
    pub sec: i32,
    pub usec: i32,
}

impl Event {
    pub fn new(slot: usize, kind: i32, x: i32, y: i32, sec: i32, usec: i32) -> Event {
        Event {
            slot: slot,
            kind: kind,
            x: x,
            y: y,
            sec: sec,
            usec: usec,
        }
    }
}
