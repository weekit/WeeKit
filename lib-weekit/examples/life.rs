extern crate weekit;

use weekit::*;

use std::cmp;

const S : usize = 20;

struct Life {
    touch_count: u32,
    grid: [[bool; S]; S],
}

impl Life {
    fn new() -> Life {
        Life {
            touch_count: 0,
	    grid: [[false; S]; S],
        }
    }
}

impl Application for Life {
    fn draw(&mut self, width: u32, height: u32) -> () {

        let screen = display::Screen::new(width, height);
        screen.background(64, 0, 0);

	// define a square in the middle of the screen
	let s = cmp::min(width, height) as f32 * 0.8;
  	let x0 = 0.5*(width as f32 - s);
	let y0 = 0.5*(height as f32 - s);	
	let w = s;
	let h = s;

	// draw the square 
        draw::fill(32, 32, 32, 1.0);
        draw::rect(x0, y0, w, h);

	// draw a grid of inset squares
        draw::fill(255, 255, 255, 1.0);

	let ww = w / S as f32;	

	for j in 0..S {
		let yj = y0 + j as f32 * ww;
		for i in 0..S {
			let xi = x0 + i as f32 * ww;
			let inset = ww * 0.2;	
				
			draw::rect(xi+inset, yj+inset, ww-inset*2.0, ww-inset*2.0);
		}
	}
	
        draw::stroke_width(1.0);
        draw::fill(255, 0, 0, 1.0);
        draw::stroke(255, 255, 255, 1.0);
    }

    fn input(&mut self, ev: &event::Event) -> () {
        self.touch_count += 1;
    }
}

fn main() {
    weekit::main(Life::new());
}
