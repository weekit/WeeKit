mod openvg;
mod weekit;
mod deja_vu_serif;

extern "C" fn draw_handler(x: i32, y: i32) {
    weekit::demo(x as u32, y as u32);
    weekit::set_fill(&[1.0, 0.0, 0.0, 1.0]);
    weekit::set_stroke(&[1.0, 1.0, 0.0, 1.0]);
    weekit::rect(0.0, 0.0, 10.0, 10.0);
    weekit::set_fill(&[0.0, 0.0, 1.0, 1.0]);
    weekit::set_stroke(&[1.0, 0.0, 1.0, 1.0]);
    weekit::rect(x as f32 - 20., y as f32 - 20., 20., 20.);
}

fn main() {
    weekit::main(draw_handler);
}
