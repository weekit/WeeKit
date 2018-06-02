use body::{Body, Pair};

const ROCK: f32 = 50.0;
const ROCK_MIN: f32 = 13.0;

#[derive(Debug, Copy, Clone)]
pub struct Rock {
    pub body: Body,
}

impl Rock {
    pub fn new() -> Rock {
        Rock {
            body: Body::new(ROCK),
        }
    }

    pub fn explode(&self, vector: Pair) -> Vec<Rock> {
        let mut v = Vec::new();
        if self.body.radius > ROCK_MIN {
            let mut incoming_heading = (vector.x / vector.y).atan() * 180.0 / 3.1415;
            if vector.y < 0.0 {
                incoming_heading += 180.0;
            } else if vector.x < 0.0 {
                incoming_heading += 360.0;
            }
            let mut h1 = incoming_heading - 90.0;
            if h1 > 360.0 {
                h1 -= 360.0;
            }
            let mut h2 = incoming_heading + 90.0;
            if h2 < 0.0 {
                h2 += 360.0;
            }
            let mag = (vector.x * vector.x + vector.y * vector.y).sqrt();
            let split_x = -vector.y / mag * self.body.radius * 0.5;
            let split_y = vector.x / mag * self.body.radius * 0.5;
            let mut r1 = self.clone();
            r1.body.radius *= 0.5;
            r1.body.position.x += split_x;
            r1.body.position.y += split_y;
            r1.body.accelerate(1.0, h1);
            v.push(r1);
            let mut r2 = self.clone();
            r2.body.radius *= 0.5;
            r2.body.position.x -= split_x;
            r2.body.position.y -= split_y;
            r2.body.accelerate(1.0, h2);
            v.push(r2);
        }
        v
    }
}
