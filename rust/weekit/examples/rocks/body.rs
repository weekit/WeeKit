const ACCELERATION: f32 = 1.0;

pub enum Turn {
    Right,
    Left,
}

pub enum Direction {
    Forward,
    Backward,
}

#[derive(Debug, Copy, Clone)]
pub struct Pair {
    pub x: f32,
    pub y: f32,
}

/// An object that can move in space.
#[derive(Debug, Copy, Clone)]
pub struct Body {
    pub position: Pair,
    pub velocity: Pair,
    pub radius: f32,
}

impl Body {
    pub fn new(radius: f32) -> Body {
        Body {
            position: Pair { x: 0.0, y: 0.0 },
            velocity: Pair { x: 0.0, y: 0.0 },
            radius: radius,
        }
    }

    pub fn accelerate(&mut self, rate: f32, heading: f32) {
        let rad = heading / 180.0 * 3.141529;
        let dx = rate * ACCELERATION * rad.sin();
        let dy = rate * ACCELERATION * rad.cos();
        self.velocity.x += dx;
        self.velocity.y += dy;
    }

    pub fn move_with_bounds(&mut self, x0: f32, y0: f32, x1: f32, y1: f32) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        if self.position.x <= x0 {
            self.position.x += x1 - x0;
        } else if self.position.x >= x1 {
            self.position.x -= x1 - x0;
        }
        if self.position.y <= y0 {
            self.position.y += y1 - y0;
        } else if self.position.y >= y1 {
            self.position.y -= y1 - y0;
        }
    }

    pub fn intersects(&self, other: &Body) -> bool {
        let dx = self.position.x - other.position.x;
        let dy = self.position.y - other.position.y;
        let dr = self.radius + other.radius;
        dx * dx + dy * dy < dr * dr
    }
}
