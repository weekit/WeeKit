use body::{Body, Direction, Turn};
use shot::Shot;

const SHIP: f32 = 20.0;
const TURN: f32 = 10.0;

#[derive(Debug)]
pub struct Ship {
    pub body: Body,
    pub heading: f32,
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            body: Body::new(SHIP),
            heading: 0.0,
        }
    }
    pub fn rotate(&mut self, turn: Turn) {
        match turn {
            Turn::Right => self.heading += TURN,
            Turn::Left => self.heading -= TURN,
        }
        if self.heading > 360.0 {
            self.heading -= 360.0;
        } else if self.heading < 0.0 {
            self.heading += 360.0;
        }
    }
    pub fn accelerate(&mut self, direction: Direction) {
        match direction {
            Direction::Forward => self.body.accelerate(1.0, self.heading),
            Direction::Backward => self.body.accelerate(1.0, (self.heading + 180.0) % 360.0),
        }
    }
    pub fn shoot(&self) -> Shot {
        let mut s = Shot::new();
        let r = s.body.radius;
        s.body = self.body.clone();
        s.body.radius = r;
        s.body.accelerate(10.0, self.heading);
        s
    }
}
