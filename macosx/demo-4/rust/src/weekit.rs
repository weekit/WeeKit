#![allow(dead_code)]

extern crate libc;
use std;
use std::iter::FromIterator;



use openvg::*;

type Fontinfo = [u8; 2032];

#[link(name = "wee")]
extern "C" {
    // platform-dependent
    fn WKMain(f: extern "C" fn(i32, i32) -> ()) -> i64;

    // libshapes
    static SansTypeface: *const Fontinfo;
    static SerifTypeface: *const Fontinfo;
    static MonoTypeface: *const Fontinfo;
    fn init(w: u32, h: u32);
    fn finish();
    fn Start(x: u32, y: u32);
    fn Background(r:u32, g:u32, b:u32);
    fn Fill(r:u32, g:u32, b:u32, a:f32);
    fn Stroke(r:u32, g:u32, b:u32, a:f32);
    fn Circle(r:f32, g:f32, b:f32);
    fn TextMid(x:f32, y:f32, s:*const libc::c_char, f:*const Fontinfo, size:u32);
    fn puts(s:*const libc::c_char);
}

pub enum PaintMode {
    Stroke = (1 << 0),
    Fill = (1 << 1),
}

pub fn set_color(mode: PaintMode, color: &[VGfloat; 4]) {
    unsafe {
        let fill_paint = vgCreatePaint();
        vgSetParameteri(
            fill_paint,
            VGPaintParamType::VG_PAINT_TYPE as i32,
            VGPaintType::VG_PAINT_TYPE_COLOR as i32,
        );
        vgSetParameterfv(
            fill_paint,
            VGPaintParamType::VG_PAINT_COLOR as i32,
            4,
            color.as_ptr(),
        );
        match mode {
            PaintMode::Stroke => vgSetPaint(fill_paint, VGPaintMode::VG_STROKE_PATH as u32),
            PaintMode::Fill => vgSetPaint(fill_paint, VGPaintMode::VG_FILL_PATH as u32),
        }
        vgDestroyPaint(fill_paint);
    }
}

pub fn new_path() -> VGPath {
    unsafe {
        return vgCreatePath(
            VG_PATH_FORMAT_STANDARD,
            VGPathDatatype::VG_PATH_DATATYPE_F,
            1.0,
            0.0,
            0,
            0,
            VGPathCapabilities::VG_PATH_CAPABILITY_APPEND_TO as u32,
        );
    }
}

pub fn draw_rect(x: f32, y: f32, w: f32, h: f32) {
    unsafe {
        let path = new_path();
        vguRect(path, x, y, w, h);
        vgDrawPath(
            path,
            VGPaintMode::VG_FILL_PATH as u32 + VGPaintMode::VG_STROKE_PATH as u32,
        );
        vgDestroyPath(path);
    }
}

pub fn main(f: extern "C" fn(i32, i32) -> ()) -> i64 {
    unsafe {
        return WKMain(f);
    }
}

pub fn demo(width : u32, height : u32) {
    println!("{} {}", width, height);
    let str_0 = "hello, world";
    let c_str_0 = std::ffi::CString::new(str_0).unwrap();
    let c_ptr_0 = c_str_0.as_ptr();

    let vec_1 = vec!['H','é','j',',',' ','v', 'ä', 'r' , 'l','d' ,'e','n'];
    let str_1 = String::from_iter(vec_1);
    let c_str_1 = std::ffi::CString::new(str_1).unwrap();
    let c_ptr_1 = c_str_1.as_ptr();

    let str_2 = "Helló Világ";
    let c_str_2 = std::ffi::CString::new(str_2).unwrap();
    let c_ptr_2 = c_str_2.as_ptr();

    let str_3 = "Ahoj světe";
    let c_str_3 = std::ffi::CString::new(str_3).unwrap();
    let c_ptr_3 = c_str_3.as_ptr();

    unsafe {
        init(width, height);                              // Start the picture
        Start(width, height);                              // Start the picture
        Background(0, 0, 0);                               // Black background
        Fill(44, 77, 232, 1.0);                              // Big blue marble
        Circle(width as f32 / 2.0, 0 as f32, width as f32);                       // The "world"
        Fill(255, 255, 255, 1.0);                            // White text

        TextMid(width as f32 / 2.0, height as f32 * 0.7, c_ptr_0, SerifTypeface, width/15);
        TextMid(width as f32 / 2.0, height as f32 * 0.5, c_ptr_1, SerifTypeface, width/15);
        TextMid(width as f32 / 2.0, height as f32 * 0.3, c_ptr_2, SerifTypeface, width/15);
        TextMid(width as f32 / 2.0, height as f32 * 0.1, c_ptr_3, SerifTypeface, width/15);

	finish();
    }
}
