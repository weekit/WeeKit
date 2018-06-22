extern crate rand;
use rand::Rng;

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

extern crate weekit;
use weekit::*;

mod body;
mod rock;
mod ship;
mod shot;

use body::{Direction, Turn};
use rock::Rock;
use ship::Ship;
use shot::Shot;

const S: usize = 10;
const W: usize = 5 * S;
const H: usize = 3 * S;

const ROCKS: usize = 4;
const SHOT_LIFETIME: i32 = 40;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RocksEvent {
    StartShooting,
    StopShooting,
    StartTurningLeft,
    StopTurningLeft,
    StartTurningRight,
    StopTurningRight,
    StartThrustingForward,
    StopThrustingForward,
    StartThrustingBackward,
    StopThrustingBackward,
    Pause,
    Resume,
    Tick,
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect {
            x: x,
            y: y,
            w: w,
            h: h,
        }
    }
    pub fn contains(&self, p: &Point) -> bool {
        p.x >= self.x && p.y >= self.y && p.x <= self.x + self.w && p.y <= self.y + self.h
    }
}

#[derive(Clone)]
pub struct Button {
    pub frame: Rect,
    pub sender: Sender<RocksEvent>,
    pub down_event: RocksEvent,
    pub up_event: RocksEvent,
}

impl Button {
    /// Creates a new Button.
    pub fn new(
        frame: Rect,
        sender: Sender<RocksEvent>,
        down_event: RocksEvent,
        up_event: RocksEvent,
    ) -> Button {
        Button {
            frame: frame,
            sender: sender,
            down_event: down_event,
            up_event: up_event,
        }
    }
    pub fn contains(&self, p: &Point) -> bool {
        return self.frame.contains(p);
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

    shot_counter: u32,

    is_shooting: bool,
    is_turning_left: bool,
    is_turning_right: bool,
    is_thrusting_forward: bool,
    is_thrusting_backward: bool,

    buttons: Vec<Button>,

    rng: rand::ThreadRng, // thread_rng is often the most convenient source of randomness

    tx: Sender<RocksEvent>,
    rx: Receiver<RocksEvent>,
}

impl Rocks {
    fn new() -> Rocks {
        let (tx, rx): (Sender<RocksEvent>, Receiver<RocksEvent>) = mpsc::channel();

        let mut world = Rocks {
            ship: Ship::new(),
            shots: Vec::new(),
            rocks: Vec::new(),
            width: 0.0,
            height: 0.0,
            grid: [[[false; H]; W]; 2],
            page: 0,
            paused: false,

            shot_counter: 0,

            is_shooting: false,
            is_turning_left: false,
            is_turning_right: false,
            is_thrusting_forward: false,
            is_thrusting_backward: false,

            buttons: Vec::new(),
            rng: rand::thread_rng(),
            tx: tx,
            rx: rx,
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
        if self.is_turning_left {
            self.ship.rotate(Turn::Left);
        }
        if self.is_turning_right {
            self.ship.rotate(Turn::Right);
        }
        if self.is_thrusting_forward {
            self.ship.accelerate(Direction::Forward);
        }
        if self.is_thrusting_backward {
            self.ship.accelerate(Direction::Backward);
        }
        if self.is_shooting {
            if self.shot_counter == 0 {
                self.shots.push(self.ship.shoot());
                self.shot_counter = 5;
            } else {
                self.shot_counter -= 1;
            }
        } else {
            self.shot_counter = 0;
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
        let mut collision = false;
        let mut split_rocks = Vec::new();
        let mut j: usize = 0;
        for rock in &mut self.rocks {
            if self.ship.body.intersects(&rock.body) {
                collision = true;
                split_rocks.push(j);
            }
            j = j + 1;
        }
        for j in split_rocks.iter().rev() {
            for r in self.rocks[*j].explode(self.ship.body.velocity) {
                self.rocks.push(r);
            }
            self.rocks.remove(*j);
        }
        if collision {
            self.center_ship();
        }

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
                for r in self.rocks[*j].explode(shot.body.velocity) {
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

        if self.rocks.len() == 0 {
            self.spawn_rocks();
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
        for button in &mut self.buttons {
            if button.contains(&Point::new(ev.x, ev.y)) {
                if ev.kind == 3 {
                    button.sender.send(button.up_event).unwrap();;
                } else if ev.kind == 1 {
                    button.sender.send(button.down_event).unwrap();;
                }
            }
        }
    }

    fn handle_key(&mut self, ev: event::Key) -> () {
        match ev.key as u16 {
            key::A => match ev.kind {
                1 => self.tx.send(RocksEvent::Pause).unwrap(),
                0 => self.tx.send(RocksEvent::Resume).unwrap(),
                _ => {}
            },
            key::UP => match ev.kind {
                1 => self.tx.send(RocksEvent::StartThrustingForward).unwrap(),
                0 => self.tx.send(RocksEvent::StopThrustingForward).unwrap(),
                _ => {}
            },
            key::DOWN => match ev.kind {
                1 => self.tx.send(RocksEvent::StartThrustingBackward).unwrap(),
                0 => self.tx.send(RocksEvent::StopThrustingBackward).unwrap(),
                _ => {}
            },
            key::LEFT => match ev.kind {
                1 => self.tx.send(RocksEvent::StartTurningLeft).unwrap(),
                0 => self.tx.send(RocksEvent::StopTurningLeft).unwrap(),
                _ => {}
            },
            key::RIGHT => match ev.kind {
                1 => self.tx.send(RocksEvent::StartTurningRight).unwrap(),
                0 => self.tx.send(RocksEvent::StopTurningRight).unwrap(),
                _ => {}
            },
            key::SPACE => match ev.kind {
                1 => self.tx.send(RocksEvent::StartShooting).unwrap(),
                0 => self.tx.send(RocksEvent::StopShooting).unwrap(),
                _ => {}
            },
            _ => {}
        }
    }

    fn handle_tick(&mut self) -> () {
        self.update();
    }

    fn center_ship(&mut self) -> () {
        self.ship.body.position.x = self.width * 0.5;
        self.ship.body.position.y = self.height * 0.5;
        self.ship.body.velocity.x = 0.0;
        self.ship.body.velocity.y = 0.0;
        self.ship.heading = 0.0;
    }

    fn spawn_rocks(&mut self) -> () {
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
    }

    fn respond(&mut self, ev: RocksEvent) {
        match ev {
            RocksEvent::StartShooting => {
                self.is_shooting = true;
            }
            RocksEvent::StopShooting => {
                self.is_shooting = false;
            }
            RocksEvent::StartTurningLeft => {
                self.is_turning_left = true;
            }
            RocksEvent::StopTurningLeft => {
                self.is_turning_left = false;
            }
            RocksEvent::StartTurningRight => {
                self.is_turning_right = true;
            }
            RocksEvent::StopTurningRight => {
                self.is_turning_right = false;
            }
            RocksEvent::StartThrustingForward => {
                self.is_thrusting_forward = true;
            }
            RocksEvent::StopThrustingForward => {
                self.is_thrusting_forward = false;
            }
            RocksEvent::StartThrustingBackward => {
                self.is_thrusting_backward = true;
            }
            RocksEvent::StopThrustingBackward => {
                self.is_thrusting_backward = false;
            }
            RocksEvent::Pause => {
                self.paused = true;
            }
            RocksEvent::Resume => {
                self.paused = false;
                self.reset();
            }
            RocksEvent::Tick => {}
        }
    }
}

impl Application for Rocks {
    /// Set the size of the game world.
    fn size(&mut self, width: u32, height: u32) -> () {
        self.width = width as f32;
        self.height = height as f32;
        // center the ship
        self.center_ship();
        // create some rocks at random locations
        self.spawn_rocks();
        // clear all shots
        self.shots.clear();
        // recreate all buttons
        self.buttons.clear();
        let s = height as i32 / 3;
        self.buttons.push(Button::new(
            Rect::new(0, 2 * s, s, s),
            self.tx.clone(),
            RocksEvent::StartShooting,
            RocksEvent::StopShooting,
        ));
        self.buttons.push(Button::new(
            Rect::new(width as i32 - s, 2 * s, s, s),
            self.tx.clone(),
            RocksEvent::StartThrustingBackward,
            RocksEvent::StopThrustingBackward,
        ));
        self.buttons.push(Button::new(
            Rect::new(0, s, s, s),
            self.tx.clone(),
            RocksEvent::StartTurningLeft,
            RocksEvent::StopTurningLeft,
        ));
        self.buttons.push(Button::new(
            Rect::new(width as i32 - s, s, s, s),
            self.tx.clone(),
            RocksEvent::StartTurningRight,
            RocksEvent::StopTurningRight,
        ));
        self.buttons.push(Button::new(
            Rect::new(0, 0, s, s),
            self.tx.clone(),
            RocksEvent::StartShooting,
            RocksEvent::StopShooting,
        ));
        self.buttons.push(Button::new(
            Rect::new(width as i32 - s, 0, s, s),
            self.tx.clone(),
            RocksEvent::StartThrustingForward,
            RocksEvent::StopThrustingForward,
        ));
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
        match self.rx.try_recv() {
            Err(std::sync::mpsc::TryRecvError::Empty) => {}
            Ok(ev) => self.respond(ev),
            _ => println!("ok"),
        }
    }
}

fn main() {
    weekit::main(Rocks::new());
}
