extern crate rand;
extern crate weekit;

use weekit::*;

use rand::Rng;

const S: usize = 10;
const W: usize = 5*S;
const H: usize = 3*S;

struct Life {
    grid: [[[bool; H]; W]; 2],
    page: usize,
    paused: bool,
}

impl Life {
    fn new() -> Life {
        let mut life = Life {
            grid: [[[false; H]; W]; 2],
            page: 0,
            paused: false,
        };
        life.reset();
        life
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
	    return
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
}

impl Application for Life {
    fn draw(&mut self, width: u32, height: u32) -> () {
        let screen = display::Screen::new(width, height);
        screen.background(64, 0, 0);

        // define a rectangle in the middle of the screen
        let cw = width as f32 / W as f32;
        let ch = height as f32 / H as f32;
        let x0 = 0.5 * (width as f32 - cw * W as f32);
        let y0 = 0.5 * (height as f32 - ch * H as f32);

        // draw the square
        draw::fill(32, 32, 32, 1.0);
        draw::rect(x0, y0, cw * W as f32, ch * H as f32);

        // draw a grid of inset squares
        draw::fill(255, 255, 255, 1.0);

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

    fn handle_touch(&mut self, ev: &event::TouchEvent) -> () {
	println!("{:?}", ev);
    }

    fn handle_key(&mut self, ev: &event::KeyEvent) -> () {
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

    fn tick(&mut self, _time: std::time::Duration) -> () {
	self.update();
    }
}

fn main() {
    weekit::main(Life::new());
}
