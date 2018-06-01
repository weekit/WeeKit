extern crate rand;
extern crate weekit;

use weekit::*;

use rand::Rng;
use std::collections::HashMap;

const S: usize = 10;
const W: usize = 5 * S;
const H: usize = 3 * S;

const SHIP: f32 = 20.0;
const ROCK: f32 = 50.0;
const SHOT: f32 = 3.0;
const ROCK_MIN: f32 = 13.0;

const ROCKS: usize = 4;
const TURN: f32 = 10.0;
const ACCELERATION: f32 = 1.0;
const SHOT_LIFETIME: i32 = 40;

#[derive(Debug, Copy, Clone)]
struct Coordinate {
    x: f32,
    y: f32,
}

/// An object that can move in space.
#[derive(Debug, Copy, Clone)]
struct Body {
    position: Coordinate,
    velocity: Coordinate,
    radius: f32,
}

impl Body {
    fn new(radius: f32) -> Body {
        Body {
            position: Coordinate { x: 0.0, y: 0.0 },
            velocity: Coordinate { x: 0.0, y: 0.0 },
            radius: radius,
        }
    }
    fn accelerate(&mut self, rate: f32, heading: f32) {
        let rad = heading / 180.0 * 3.141529;
        let dx = rate * ACCELERATION * rad.sin();
        let dy = rate * ACCELERATION * rad.cos();
        self.velocity.x += dx;
        self.velocity.y += dy;
    }
    fn move_with_bounds(&mut self, x0: f32, y0: f32, x1: f32, y1: f32) {
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
    fn intersects(&self, other: &Body) -> bool {
        let dx = self.position.x - other.position.x;
        let dy = self.position.y - other.position.y;
        let dr = self.radius + other.radius;
        dx * dx + dy * dy < dr * dr
    }
}

#[derive(Debug, Copy, Clone)]
struct Rock {
    body: Body,
}

impl Rock {
    fn new() -> Rock {
        Rock {
            body: Body::new(ROCK),
        }
    }

    fn split(&self, vector: Coordinate) -> Vec<Rock> {
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

enum Turn {
    Right,
    Left,
}

enum Direction {
    Forward,
    Backward,
}

#[derive(Debug)]
struct Shot {
    body: Body,
    age: i32,
}

impl Shot {
    fn new() -> Shot {
        Shot {
            body: Body::new(SHOT),
            age: 0,
        }
    }
}

#[derive(Debug)]
struct Ship {
    body: Body,
    heading: f32,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            body: Body::new(SHIP),
            heading: 0.0,
        }
    }
    fn rotate(&mut self, turn: Turn) {
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
    fn accelerate(&mut self, direction: Direction) {
        match direction {
            Direction::Forward => self.body.accelerate(1.0, self.heading),
            Direction::Backward => self.body.accelerate(1.0, (self.heading + 180.0) % 360.0),
        }
    }
    fn shoot(&self) -> Shot {
        let mut s = Shot::new();
        let r = s.body.radius;
        s.body = self.body.clone();
        s.body.radius = r;
        s.body.accelerate(10.0, self.heading);
        s
    }
}

struct Rocks {
    ship: Ship,
    shots: Vec<Shot>,
    rocks: Vec<Rock>,
    width: f32,
    height: f32,

    grid: [[[bool; H]; W]; 2],
    page: usize,
    paused: bool,

    keys: HashMap<u16, u16>,

    rng: rand::ThreadRng, // thread_rng is often the most convenient source of randomness
}

impl Rocks {
    fn new() -> Rocks {
        let mut world = Rocks {
            ship: Ship::new(),
            shots: Vec::new(),
            rocks: Vec::new(),
            width: 0.0,
            height: 0.0,
            grid: [[[false; H]; W]; 2],
            page: 0,
            paused: false,

            keys: HashMap::new(),
            rng: rand::thread_rng(),
        };
        world.reset();
        world
    }

    fn random(&mut self, max: f32) -> f32 {
        let x: f32 = self.rng.gen(); // random number in range (0, 1)
        x * max
    }

    fn reset(&mut self) {
        for j in 0..H {
            for i in 0..W {
                let x: f32 = self.rng.gen(); // random number in range (0, 1)
                self.grid[0][i][j] = x < 0.5;
            }
        }
        self.page = 0;
    }

    fn update(&mut self) -> () {
        if self.paused {
            return;
        }

        // handle user inputs
        if self.keys.contains_key(&key::LEFT) {
            self.ship.rotate(Turn::Left);
        }
        if self.keys.contains_key(&key::RIGHT) {
            self.ship.rotate(Turn::Right);
        }
        if self.keys.contains_key(&key::UP) {
            self.ship.accelerate(Direction::Forward);
        }
        if self.keys.contains_key(&key::DOWN) {
            self.ship.accelerate(Direction::Backward);
        }
        if self.keys.contains_key(&key::SPACE) {
            let c = self.keys[&key::SPACE];
            if c % 10 == 0 {
                self.shots.push(self.ship.shoot());
            }
            self.keys.insert(key::SPACE, c + 1);
        }

        // move the ship
        self.ship
            .body
            .move_with_bounds(0.0, 0.0, self.width, self.height);

        // move the rocks
        for rock in &mut self.rocks {
            rock.body
                .move_with_bounds(0.0, 0.0, self.width, self.height);
        }

        // move the shots
        let mut expired = Vec::new();
        let mut i: usize = 0;
        for shot in &mut self.shots {
            shot.age += 1;
            shot.body
                .move_with_bounds(0.0, 0.0, self.width, self.height);
            if shot.age >= SHOT_LIFETIME {
                expired.push(i);
            }
            i = i + 1;
        }
        for i in expired.iter().rev() {
            self.shots.remove(*i);
        }

        // handle collisions
        let mut exploded_shots = Vec::new();
        let mut i: usize = 0;
        for shot in &mut self.shots {
            let mut collision = false;
            let mut split_rocks = Vec::new();
            let mut j: usize = 0;
            for rock in &mut self.rocks {
                if shot.body.intersects(&rock.body) {
                    collision = true;
                    split_rocks.push(j)
                }
                j = j + 1;
            }
            for j in split_rocks.iter().rev() {
                for r in self.rocks[*j].split(shot.body.velocity) {
                    self.rocks.push(r);
                }
                self.rocks.remove(*j);
            }
            if collision {
                exploded_shots.push(i);
            }
            i = i + 1;
        }
        for i in exploded_shots.iter().rev() {
            self.shots.remove(*i);
        }

        // update life
        let next = 1 - self.page;
        for j in 0..H {
            for i in 0..W {
                let is_live = self.grid[self.page][i][j];
                let mut live_neighbors = 0;
                for di in 0..3 {
                    for dj in 0..3 {
                        if di != 1 || dj != 1 {
                            let mut ii: i32 = i as i32 + di - 1;
                            if ii < 0 {
                                ii += W as i32;
                            } else if ii >= W as i32 {
                                ii -= W as i32;
                            }
                            let mut jj: i32 = j as i32 + dj - 1;
                            if jj < 0 {
                                jj += H as i32;
                            } else if jj >= H as i32 {
                                jj -= H as i32;
                            }
                            if self.grid[self.page][ii as usize][jj as usize] {
                                live_neighbors += 1;
                            }
                        }
                    }
                }
                // Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
                // Any live cell with two or three live neighbours lives on to the next generation.
                // Any live cell with more than three live neighbours dies, as if by overpopulation.
                // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
                if is_live {
                    self.grid[next][i][j] = (live_neighbors == 2) || (live_neighbors == 3);
                } else {
                    self.grid[next][i][j] = live_neighbors == 3;
                }
            }
        }
        self.page = next;
    }

    fn handle_touch(&mut self, ev: event::Touch) -> () {
        println!("{:?}", ev);
    }

    fn handle_key(&mut self, ev: event::Key) -> () {
        if ev.kind == 1 {
            self.keys.insert(ev.key, 0);
        } else if ev.kind == 0 {
            self.keys.remove(&ev.key);
        }
        if ev.key == key::A {
            if ev.kind == 1 {
                self.paused = true;
            } else if ev.kind == 0 {
                self.paused = false;
                self.reset();
            }
        }
    }

    fn handle_tick(&mut self) -> () {
        self.update();
    }
}

impl Application for Rocks {
    /// Set the size of the game world.
    fn size(&mut self, width: u32, height: u32) -> () {
        self.width = width as f32;
        self.height = height as f32;
        // center the ship
        self.ship.body.position.x = self.width * 0.5;
        self.ship.body.position.y = self.height * 0.5;
        // create some rocks at random locations
        self.rocks.clear();
        for _ in 0..ROCKS {
            let mut rock = Rock::new();
            let w = self.width;
            let h = self.height;
            rock.body.position.x = self.random(w);
            rock.body.position.y = self.random(h);
            rock.body.accelerate(1.0, self.random(360.0));
            self.rocks.push(rock);
        }
        // clear all shots
        self.shots.clear();
    }
    /// Draw the game world.
    fn draw(&mut self, width: u32, height: u32) -> () {
        let canvas = draw::Canvas::new(width, height);
        canvas.background(64, 0, 0);
        // define a rectangle in the middle of the canvas
        let cw = width as f32 / W as f32;
        let ch = height as f32 / H as f32;
        let x0 = 0.5 * (width as f32 - cw * W as f32);
        let y0 = 0.5 * (height as f32 - ch * H as f32);
        // draw the square
        draw::fill(8, 8, 8, 1.0);
        draw::rect(x0, y0, cw * W as f32, ch * H as f32);
        // draw a grid of inset squares
        draw::fill(64, 64, 64, 1.0);
        let inset = cw * 0.1;
        for j in 0..H {
            let yj = y0 + j as f32 * ch;
            for i in 0..W {
                if self.grid[self.page][i][j] {
                    let xi = x0 + i as f32 * cw;
                    draw::rect(xi + inset, yj + inset, cw - inset * 2.0, ch - inset * 2.0);
                }
            }
        }
        // draw the rocks
        draw::fill(128, 128, 255, 0.7);
        draw::stroke(255, 255, 255, 0.9);
        for rock in &self.rocks {
            draw::circle(
                rock.body.position.x,
                rock.body.position.y,
                2.0 * rock.body.radius,
            );
        }
        // draw the shots
        draw::fill(255, 0, 0, 0.7);
        draw::stroke(255, 0, 0, 1.0);
        for shot in &self.shots {
            draw::circle(
                shot.body.position.x,
                shot.body.position.y,
                2.0 * shot.body.radius,
            );
        }
        // draw the ship
        let sh = self.ship.body.radius;
        let sw = self.ship.body.radius;
        draw::fill(255, 255, 128, 1.0);
        draw::translate(self.ship.body.position.x, self.ship.body.position.y);
        draw::rotate(-self.ship.heading);
        let x: [f32; 4] = [0.0, 0.5 * sw, 0.0, -0.5 * sw];
        let y: [f32; 4] = [0.5 * sh, -0.5 * sh, 0.0, -0.5 * sh];
        draw::polygon(&x, &y, 4);
    }
    /// Handle an event in the game world.
    fn handle(&mut self, ev: &event::Event) {
        match ev {
            &event::Event::Touch(t, _) => self.handle_touch(t),
            &event::Event::Key(k, _) => self.handle_key(k),
            &event::Event::Tick(_) => self.handle_tick(),
        }
    }
}

fn main() {
    weekit::main(Rocks::new());
}
