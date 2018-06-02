use body::Body;

const SHOT: f32 = 3.0;

#[derive(Debug)]
pub struct Shot {
    pub body: Body,
    pub age: i32,
}

impl Shot {
    pub fn new() -> Shot {
        Shot {
            body: Body::new(SHOT),
            age: 0,
        }
    }
}
