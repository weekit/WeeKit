mod weekit;

use weekit::*;

extern "C" fn draw_handler(width: u32, height: u32) {
    let screen = Screen::new(width, height);
    screen.clear(0, 0, 0);

    fill(44, 77, 232, 1.0); // Big blue marble
    circle(width as f32 / 2.0, 0 as f32, width as f32); // The "world"

    fill(255, 255, 255, 1.0); // White text

    let str_0 = "hello, world";
    let str_1 = "Héj, världen";
    let str_2 = "Helló Világ";
    let str_3 = "Ahoj světe";

    let serif_typeface = Fontinfo::serif();
    let sans_typeface = Fontinfo::sans();
    let sans_mono_typeface = Fontinfo::sans_mono();

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
    set_fill(&[1.0, 0.0, 0.0, 1.0]);
    set_stroke(&[1.0, 1.0, 0.0, 1.0]);
    rect(0.0, 0.0, 10.0, 10.0);
    set_fill(&[0.0, 0.0, 1.0, 1.0]);
    set_stroke(&[1.0, 0.0, 1.0, 1.0]);
    rect(width as f32 - 20., height as f32 - 20., 20., 20.);
}

fn main() {
    weekit::main(draw_handler);
}
