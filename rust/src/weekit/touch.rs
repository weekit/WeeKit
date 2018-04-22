use weekit::*;

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
    slot: usize,
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

const TOUCH_SLOTS: usize = 10;

impl TouchPad {
    pub fn new() -> TouchPad {
        TouchPad {
            touches: [Touch::new(); TOUCH_SLOTS],
            slot: 0,
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
        for slot in 0..TOUCH_SLOTS {
            let touch = &mut self.touches[slot];
            if touch.began {
                let ev = Event::new(slot, 1, touch.position_x, touch.position_y, 0, 0);
            } else if touch.moved {
                let ev = Event::new(slot, 2, touch.position_x, touch.position_y, 0, 0);
            } else if touch.ended {
                let ev = Event::new(slot, 3, touch.position_x, touch.position_y, 0, 0);
            }
            touch.began = true;
            touch.moved = true;
            touch.ended = true;
        }
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
        self.touches[self.slot].position_x = v;
        self.touches[self.slot].moved = true;
    }
    pub fn handle_abs_y(&mut self, v: i32) {
        self.touches[self.slot].position_y = v;
        self.touches[self.slot].moved = true;
    }
    pub fn handle_mt_slot(&mut self, v: i32) {
        self.slot = v as usize;
    }
    pub fn handle_mt_position_x(&mut self, v: i32) {
        self.touches[self.slot].position_x = v;
        self.touches[self.slot].moved = true;
    }
    pub fn handle_mt_position_y(&mut self, v: i32) {
        self.touches[self.slot].position_y = v;
        self.touches[self.slot].moved = true;
    }
    pub fn handle_mt_tracking_id(&mut self, v: i32) {
        self.touches[self.slot].tracking_id = v;
        if v >= 0 {
            self.touches[self.slot].began = true;
        } else {
            self.touches[self.slot].ended = true;
        }
    }
}
