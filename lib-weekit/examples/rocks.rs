extern crate rand;
extern crate weekit;

use weekit::*;

use rand::Rng;

const S: usize = 10;
const W: usize = 5 * S;
const H: usize = 3 * S;

const TURN: f32 = 10.0;
const ACCELERATION: f32 = 1.0;

#[derive(Debug)]
struct Coordinate {
    x: f32,
    y: f32,
}

/// An object that can move in space.
#[derive(Debug)]
struct Body {
    position: Coordinate,
    velocity: Coordinate,
}

impl Body {
    fn new() -> Body {
        Body {
            position: Coordinate { x: 0.0, y: 0.0 },
            velocity: Coordinate { x: 0.0, y: 0.0 },
        }
    }
    fn accelerate(&mut self, heading: f32) {
        let rad = heading / 180.0 * 3.141529;
        let dx = ACCELERATION * -rad.sin();
        let dy = ACCELERATION * rad.cos();
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
}

#[derive(Debug)]
struct Rock {
    body: Body,
    radius: f32,
}

impl Rock {
    fn new() -> Rock {
        Rock {
            body: Body::new(),
            radius: 100.0,
        }
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

struct Shot {
    body: Body,
    radius: f32,
}

impl Shot {
    fn new() -> Shot {
        Shot {
            body: Body::new(),
            radius: 10.0,
        }
    }
}

struct Ship {
    body: Body,
    heading: f32,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            body: Body::new(),
            heading: 0.0,
        }
    }
    fn rotate(&mut self, turn: Turn) {
        match turn {
            Turn::Right => self.heading -= TURN,
            Turn::Left => self.heading += TURN,
        }
        if self.heading > 360.0 {
            self.heading -= 360.0;
        } else if self.heading < 0.0 {
            self.heading += 360.0;
        }
    }
    fn accelerate(&mut self, direction: Direction) {
        match direction {
            Direction::Forward => self.body.accelerate(self.heading),
            Direction::Backward => self.body.accelerate((self.heading + 180.0) % 360.0),
        }
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
        };
        world.reset();
        world
    }

    fn reset(&mut self) {
        // thread_rng is often the most convenient source of randomness:
        let mut rng = rand::thread_rng();
        for j in 0..H {
            for i in 0..W {
                let x: f64 = rng.gen(); // random number in range (0, 1)
                self.grid[0][i][j] = x < 0.5;
            }
        }
        self.page = 0;
    }

    fn update(&mut self) -> () {
        if self.paused {
            return;
        }

        // move the ship
        self.ship
            .body
            .move_with_bounds(0.0, 0.0, self.width, self.height);

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
        println!("{:?}", ev);
        if ev.key == key::A {
            if ev.kind == 1 {
                self.paused = true;
            } else if ev.kind == 0 {
                self.paused = false;
                self.reset();
            }
        }
        if ev.key == key::LEFT {
            println!("key left");
            self.ship.rotate(Turn::Left);
        }
        if ev.key == key::RIGHT {
            println!("key right");
            self.ship.rotate(Turn::Right);
        }
        if ev.key == key::UP {
            self.ship.accelerate(Direction::Forward);
        }
        if ev.key == key::DOWN {
            self.ship.accelerate(Direction::Backward);
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
        let mut rng: rand::ThreadRng = rand::thread_rng();
        for i in 0..4 {
            let mut rock = Rock::new();
            let x: f64 = rng.gen(); // random number in range (0, 1)
            let y: f64 = rng.gen(); // random number in range (0, 1)
            rock.body.position.x = ((self.width as f64) * x) as f32;
            rock.body.position.y = ((self.height as f64) * y) as f32;
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
        draw::fill(255, 255, 255, 0.2);
        draw::stroke(255, 255, 255, 0.5);
        for rock in &self.rocks {
            draw::circle(rock.body.position.x, rock.body.position.y, rock.radius);
        }
        // draw the ship
        let sh = 50.0;
        let sw = 50.0;
        draw::fill(255, 255, 128, 1.0);
        draw::translate(self.ship.body.position.x, self.ship.body.position.y);
        draw::rotate(self.ship.heading);
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
