// Event instances are sent by the TouchPad.
#[derive(Copy, Clone)]
pub struct Event {
    slot: i32, // A unique integer for tracking a touch.
    kind: i32, // 1 = TouchDown, 2 = TouchMoved, 3 = TouchEnded
    x: i32,    // X position on the touchpad.
    y: i32,    // Y position on the touchpad.
    sec: i32,  // Time of the touch (seconds).
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
        Touch {
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

const EV_SYN: u16 = 0x00;
const EV_KEY: u16 = 0x01;
const EV_ABS: u16 = 0x03;

const ABS_X: u16 = 0x00;
const ABS_Y: u16 = 0x01;
const ABS_MT_SLOT: u16 = 0x2f; /* MT slot being modified */
const ABS_MT_POSITION_X: u16 = 0x35; /* Center X ellipse position */
const ABS_MT_POSITION_Y: u16 = 0x36; /* Center Y ellipse position */
const ABS_MT_TRACKING_ID: u16 = 0x39; /* Unique ID of initiated contact */

impl TouchPad {
    pub fn new() -> TouchPad {
        TouchPad {
            touches: [Touch::new(); 10],
            active_slot: 0,
        }
    }
    pub fn handle(&mut self, t: u16, c: u16, v: i32) {
        match t {
            EV_SYN => self.handle_syn(c, v),
            EV_KEY => self.handle_key(c, v),
            EV_ABS => self.handle_abs(c, v),
            _ => {}
        }
    }
    pub fn handle_syn(&mut self, _c: u16, _v: i32) {
        println!("SYN");
    }
    pub fn handle_key(&mut self, c: u16, v: i32) {
        if c == 330 {
            if v == 0 {
                println!("TOUCH UP");
            } else if v == 1 {
                println!("TOUCH DOWN");
            }
        }
    }
    pub fn handle_abs(&mut self, c: u16, v: i32) {
        match c {
            ABS_X => self.handle_abs_x(v),
            ABS_Y => self.handle_abs_y(v),
            ABS_MT_SLOT => self.handle_mt_slot(v),
            ABS_MT_POSITION_X => self.handle_mt_position_x(v),
            ABS_MT_POSITION_Y => self.handle_mt_position_y(v),
            ABS_MT_TRACKING_ID => self.handle_mt_tracking_id(v),
            _ => {}
        }
    }
    pub fn handle_abs_x(&mut self, v: i32) {
        self.touches[self.active_slot as usize].position_x = v;
    }
    pub fn handle_abs_y(&mut self, v: i32) {
        self.touches[self.active_slot as usize].position_y = v;
    }
    pub fn handle_mt_slot(&mut self, v: i32) {
        self.active_slot = v;
    }
    pub fn handle_mt_position_x(&mut self, v: i32) {
        self.touches[self.active_slot as usize].position_x = v;
    }
    pub fn handle_mt_position_y(&mut self, v: i32) {
        self.touches[self.active_slot as usize].position_y = v;
    }
    pub fn handle_mt_tracking_id(&mut self, v: i32) {
        self.touches[self.active_slot as usize].tracking_id = v;
    }
}
