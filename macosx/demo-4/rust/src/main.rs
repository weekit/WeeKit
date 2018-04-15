mod openvg;
mod weekit;

use weekit::*;

extern "C" fn draw_handler(x: i32, y: i32) {
    demo(x as u32, y as u32);
    set_color(PaintMode::Fill, &[1.0, 0.0, 0.0, 1.0]);
    set_color(PaintMode::Stroke, &[1.0, 1.0, 0.0, 1.0]);
    draw_rect(0.0, 0.0, 10.0, 10.0);
    set_color(PaintMode::Fill, &[0.0, 0.0, 1.0, 1.0]);
    set_color(PaintMode::Stroke, &[1.0, 0.0, 1.0, 1.0]);
    draw_rect(x as f32 - 20., y as f32 - 20., 20., 20.);
}

fn main() {
    weekit::main(draw_handler);
}
