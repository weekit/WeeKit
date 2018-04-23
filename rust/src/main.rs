mod weekit;

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

struct Application {
    event_count: i32,
    circles: [Circle; 10],
}

impl Application {
    fn new() -> Application {
        Application {
            event_count: 0,
            circles: [Circle::new(); 10],
        }
    }
}

impl weekit::Application for Application {
    fn draw(&self, width: u32, height: u32) -> () {
        let screen = Screen::new(width, height);
        screen.background(64, 0, 64);

        fill(44, 77, 232, 1.0); // Big blue marble
        circle(width as f32 / 2.0, 0 as f32, width as f32); // The "world"

        fill(255, 255, 255, 1.0); // White text

        let str_0 = "hello, world";
        let str_1 = "Héj, världen";
        let str_2 = "Helló Világ";
        let str_3 = "Ahoj světe";

        let serif_typeface = Font::serif();
        let sans_typeface = Font::sans();
        let sans_mono_typeface = Font::sans_mono();

        text_mid(
            width as f32 / 2.0,
            height as f32 * 0.7,
            str_0,
            &serif_typeface,
            width / 15,
        );
        text_mid(
            width as f32 / 2.0,
            height as f32 * 0.5,
            &str_1,
            &sans_typeface,
            width / 15,
        );
        text_mid(
            width as f32 / 2.0,
            height as f32 * 0.3,
            str_2,
            &sans_mono_typeface,
            width / 15,
        );
        text_mid(
            width as f32 / 2.0,
            height as f32 * 0.1,
            str_3,
            &serif_typeface,
            width / 15,
        );
        stroke_width(1.0);
        fill(255, 0, 0, 1.0);
        stroke(255, 255, 255, 1.0);

        let s = width as f32 * 0.05;
        let m = 2.0;

        rect(m, m, s, s);
        fill(0, 0, 255, 1.0);
        stroke(255, 0, 255, 1.0);
        rect(width as f32 - s - m, height as f32 - s - m, s, s);

        fill(255, 255, 255, 0.5);
        stroke(255, 255, 255, 1.0);
        for i in 0..10 as usize {
            if self.circles[i].visible {
                circle(self.circles[i].x as f32, self.circles[i].y as f32, 100.0);
            }
        }
    }

    fn event(&mut self, ev: &weekit::Event) -> () {
        self.event_count += 1;
        println!("RECEIVED EVENT {} {:?}", self.event_count, ev);
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
    weekit::main(Application::new());
}
