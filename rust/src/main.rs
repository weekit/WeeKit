mod weekit;

use weekit::*;

struct Application {}

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
    }

    fn event(&self, ev: &weekit::Event) -> () {
	println!("RECEIVED EVENT {:?}", ev);
    }
}

static APP: Application = Application {};

fn main() {
    weekit::main(&APP);
}
