mod openvg;
mod weekit;

use weekit::*;

extern "C" fn draw_handler(_x: i32, _y: i32) {
    demo(_x as u32, _y as u32);
    set_color(PaintMode::Fill, &[1.0, 0.0, 0.0, 1.0]);
    set_color(PaintMode::Stroke, &[1.0, 1.0, 0.0, 1.0]);
    draw_rect(0.0, 0.0, 10.0, 10.0);
    set_color(PaintMode::Fill, &[0.0, 0.0, 1.0, 1.0]);
    set_color(PaintMode::Stroke, &[1.0, 0.0, 1.0, 1.0]);
    draw_rect(20., 20., 20., 20.);
}

fn main() {
    weekit::main(draw_handler);
}
