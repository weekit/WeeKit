extern crate weekit;

use weekit::*;

struct Demo<'a> {
    touch_count: u32,
    sans_typeface: Option<font::Font<'a>>,
}

impl<'a> Demo<'a> {
    fn new() -> Demo<'a> {
        Demo {
            touch_count: 0,
            sans_typeface: None,
        }
    }
    fn load_fonts(&mut self) -> () {
        self.sans_typeface = Some(font::Font::sans());
    }
}

impl<'a> Application for Demo<'a> {
    fn draw(&mut self, width: u32, height: u32) -> () {
        match self.sans_typeface {
            Some(_) => (),
            None => self.load_fonts(),
        }

        let screen = display::Screen::new(width, height);
        screen.background((self.touch_count * 4) % 256, 0, 0);

        draw::fill(44, 77, (128 + 4*self.touch_count) % 256, 1.0); // Big blue marble
        draw::circle(width as f32 / 2.0, 0 as f32, width as f32); // The "world"

        draw::fill(255, 255, 255, 1.0); // White text

        match self.sans_typeface {
            Some(ref font) => draw::text_mid(
                width as f32 / 2.0,
                height as f32 * 0.5,
                "hello, world",
                font,
                width / 15,
            ),
            None => {}
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
    weekit::main(Demo::new());
}
