use openvg::*;

#[link(name = "app")]
extern "C" {
    fn WKMain(f: extern "C" fn(i32, i32) -> ()) -> i64;
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
