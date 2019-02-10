// Copyright 2018 The WeeKit Authors. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate rand;
extern crate weekit;

use weekit::*;

use std::collections::HashMap;
use rand::Rng;

const S: usize = 10;
const W: usize = 5 * S;
const H: usize = 3 * S;

struct Life {
    grid: [[[bool; H]; W]; 2],
    page: usize,
    paused: bool,
    history: HashMap<u64, i8>,
}

impl Life {
    fn new() -> Life {
        let mut life = Life {
            grid: [[[false; H]; W]; 2],
            page: 0,
            paused: false,
            history: HashMap::new(),
        };
        life.reset();
        life
    }

    fn signature(&self) -> u64 {
        let mut s : u64 = 0;
        for j in 0..H {
            for i in 0..W {
                if self.grid[self.page][i][j] {
                    s ^= 1 << ((j * W + i) % 64)
                }
            }
        }
        s
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
        self.history.clear();
    }

    fn update(&mut self) -> () {
        if self.paused {
            return;
        }
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
        if ev.key == 57 {
            if ev.kind == 1 {
                self.paused = true;
            } else if ev.kind == 0 {
                self.paused = false;
                self.reset();
            }
        }
    }

    fn needs_reset(&mut self) -> bool {
        let s = self.signature();
        let c = self.history.entry(s).or_insert(0);
        *c += 1;
        //println!("{:016X} {}", s, *c);
        *c >= 64
    }

    fn handle_tick(&mut self) -> () {
        self.update();
        if self.needs_reset() {
            self.reset();
        }
    }
}

impl Application for Life {
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
        draw::fill(255, 255, 128, 1.0);

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
    }

    fn handle(&mut self, ev: &event::Event) {
        match ev {
            &event::Event::Touch(t, _) => self.handle_touch(t),
            &event::Event::Key(k, _) => self.handle_key(k),
            &event::Event::Tick(_) => self.handle_tick(),
        }
    }
}

fn main() {
    weekit::main(Life::new());
}
