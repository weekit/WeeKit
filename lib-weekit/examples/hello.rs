extern crate weekit;

use weekit::*;

#[derive(Copy, Clone)]
struct Circle {
    x: i32,
    y: i32,
    visible: bool,
}

impl Circle {
    fn new() -> Circle {
        Circle {
            x: 0,
            y: 0,
            visible: false,
        }
    }
}

struct Demo<'a> {
    event_count: i32,
    circles: [Circle; 10],
    serif_typeface: Option<font::Font<'a>>,
    sans_typeface: Option<font::Font<'a>>,
    sans_mono_typeface: Option<font::Font<'a>>,
}

impl<'a> Demo<'a> {
    fn new() -> Demo<'a> {
        Demo {
            event_count: 0,
            circles: [Circle::new(); 10],
            serif_typeface: None,
            sans_typeface: None,
            sans_mono_typeface: None,
        }
    }
    fn load_fonts(&mut self) -> () {
        self.serif_typeface = Some(font::Font::serif());
        self.sans_typeface = Some(font::Font::sans());
        self.sans_mono_typeface = Some(font::Font::sans_mono());
    }
}

impl<'a> Application for Demo<'a> {
    fn draw(&mut self, width: u32, height: u32) -> () {
        match self.serif_typeface {
            Some(_) => (),
            None => self.load_fonts(),
        }

        let canvas = draw::Canvas::new(width, height);
        canvas.background(64, 0, 64);

        draw::fill(44, 77, 232, 1.0); // Big blue marble
        draw::circle(width as f32 / 2.0, 0 as f32, width as f32); // The "world"

        draw::fill(255, 255, 255, 1.0); // White text

        let str_0 = "hello, world";
        let str_1 = "Héj, världen";
        let str_2 = "Helló Világ";
        let str_3 = "Ahoj světe";

        match self.serif_typeface {
            Some(ref font) => draw::text_mid(
                width as f32 / 2.0,
                height as f32 * 0.7,
                str_0,
                font,
                width / 15,
            ),
            None => {}
        }

        match self.serif_typeface {
            Some(ref font) => draw::text_mid(
                width as f32 / 2.0,
                height as f32 * 0.5,
                &str_1,
                font,
                width / 15,
            ),
            None => {}
        }

        match self.serif_typeface {
            Some(ref font) => draw::text_mid(
                width as f32 / 2.0,
                height as f32 * 0.3,
                str_2,
                font,
                width / 15,
            ),
            None => {}
        }

        match self.serif_typeface {
            Some(ref font) => draw::text_mid(
                width as f32 / 2.0,
                height as f32 * 0.1,
                str_3,
                font,
                width / 15,
            ),
            None => {}
        }

        draw::stroke_width(1.0);
        draw::fill(255, 0, 0, 1.0);
        draw::stroke(255, 255, 255, 1.0);

        let s = width as f32 * 0.05;
        let m = 2.0;

        draw::rect(m, m, s, s);
        draw::fill(0, 0, 255, 1.0);
        draw::stroke(255, 0, 255, 1.0);
        draw::rect(width as f32 - s - m, height as f32 - s - m, s, s);

        draw::fill(255, 255, 255, 0.5);
        draw::stroke(255, 255, 255, 1.0);
        for i in 0..10 as usize {
            if self.circles[i].visible {
                draw::circle(
                    self.circles[i].x as f32,
                    (height as i32 - self.circles[i].y) as f32,
                    100.0,
                );
            }
        }
    }

    fn handle_touch(&mut self, ev: &event::TouchEvent) -> () {
        println!("input");
        self.event_count += 1;
        if ev.kind == 1 {
            self.circles[ev.slot as usize].visible = true;
        } else if ev.kind == 3 {
            self.circles[ev.slot as usize].visible = false;
        }
        self.circles[ev.slot as usize].x = ev.x;
        self.circles[ev.slot as usize].y = ev.y;
    }
}

fn main() {
    weekit::main(Demo::new());
}
