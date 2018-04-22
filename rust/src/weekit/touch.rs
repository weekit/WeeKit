

// Event instances are sent by the TouchPad.
#[derive(Copy, Clone)]
pub struct Event {
	slot: i32, // A unique integer for tracking a touch.
	kind: i32, // 1 = TouchDown, 2 = TouchMoved, 3 = TouchEnded
	x: i32,	// X position on the touchpad.
	y: i32,	// Y position on the touchpad.
	sec: i32, // Time of the touch (seconds).
	usec: i32, // Time of the touch (sub-second milliseconds).
}

#[derive(Copy, Clone)]
pub struct Touch {
	tracking_id: i32,
	position_x: i32,
	position_y: i32,
	began: bool,
	ended: bool,
	moved: bool,
}

impl Touch {
  pub fn new() -> Touch {
	Touch{
	tracking_id: 0,
	position_x: 0,
	position_y: 0,
	began: false,
	ended: false,
	moved: false,
	}
  }
}

// A TouchPad monitors events on a touchscreen.
pub struct TouchPad {
	touches: [Touch; 10],
	active_slot: i32,
}

impl TouchPad {
  pub fn new() -> TouchPad {
	TouchPad {
	touches: [Touch::new(); 10],
	active_slot: 0}
  }
  pub fn handle(&self, t: u16, c: u16, v: i32) {
    println!("TOUCHPAD {} {} {}", t, c, v);
  }
}


