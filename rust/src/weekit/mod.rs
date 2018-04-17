#![allow(dead_code)]

mod openvg;
mod deja_vu_serif;
mod deja_vu_sans;
mod deja_vu_sans_mono;

extern crate libc;

use self::openvg::*;

pub struct Screen {
    w: u32,
    h: u32,
}

impl Screen {
    pub fn new(w: u32, h: u32) -> Screen {
        let screen = Screen { w: w, h: h };
        screen.clear(255, 255, 255);
        fill(0, 0, 0, 1.0);
        stroke(0, 0, 0, 1.0);
        stroke_width(0.0);
        unsafe {
            vgLoadIdentity();
        }
        screen
    }

    pub fn clear(&self, r: u32, g: u32, b: u32) {
        let color = rgb(r, g, b);
        unsafe {
            vgSetfv(VGParamType::VG_CLEAR_COLOR, 4, color.as_ptr());
            vgClear(0, 0, self.w, self.h);
        }
    }
}

pub struct Fontinfo<'a> {
    character_map: &'a [i16],
    glyph_advances: &'a [i32],
    glyph_count: i32,
    descender_height: i32,
    font_height: i32,
    glyphs: [VGPath; 500],
}

impl<'a> Drop for Fontinfo<'a> {
    fn drop(&mut self) {
        for i in 0..self.glyph_count {
            unsafe { vgDestroyPath(self.glyphs[i as usize]) }
        }
    }
}

impl<'a> Fontinfo<'a> {
    pub fn new(
        glyph_points: &'a [VGfloat],
        glyph_point_indices: &'a [i32],
        glyph_instructions: &'a [i8],
        glyph_instruction_indices: &'a [i32],
        glyph_instruction_counts: &'a [i32],
        glyph_advances: &'a [i32],
        character_map: &'a [i16],
        glyph_count: i32,
        descender_height: i32,
        font_height: i32,
    ) -> Fontinfo<'a> {
        let mut glyphs: [VGPath; 500] = [0; 500];

        for i in 0..glyph_count {
            unsafe {
                let path = vgCreatePath(
                    VG_PATH_FORMAT_STANDARD,
                    VGPathDatatype::VG_PATH_DATATYPE_F,
                    1.0 / 65536.0,
                    0.0,
                    0,
                    0,
                    VGPathCapabilities::VG_PATH_CAPABILITY_ALL as u32,
                );
                let ic = glyph_instruction_counts[i as usize];
                if ic > 0 {
                    let instructions = glyph_instructions
                        [glyph_instruction_indices[i as usize] as usize..]
                        .as_ptr() as *const u8;
                    let points = glyph_points[glyph_point_indices[i as usize] as usize * 2..]
                        .as_ptr() as *const i8;
                    vgAppendPathData(path, ic, instructions, points);
                }
                glyphs[i as usize] = path;
            }
        }

        Fontinfo {
            character_map: character_map,
            glyph_advances: glyph_advances,
            glyph_count: glyph_count,
            descender_height: descender_height,
            font_height: font_height,
            glyphs: glyphs,
        }
    }

    pub fn serif() -> Fontinfo<'static> {
        Fontinfo::new(
            &deja_vu_serif::GLYPH_POINTS,
            &deja_vu_serif::GLYPH_POINT_INDICES,
            &deja_vu_serif::GLYPH_INSTRUCTIONS,
            &deja_vu_serif::GLYPH_INSTRUCTION_INDICES,
            &deja_vu_serif::GLYPH_INSTRUCTION_COUNTS,
            &deja_vu_serif::GLYPH_ADVANCES,
            &deja_vu_serif::CHARACTER_MAP,
            deja_vu_serif::GLYPH_COUNT,
            deja_vu_serif::DESCENDER_HEIGHT,
            deja_vu_serif::FONT_HEIGHT,
        )
    }

    pub fn sans() -> Fontinfo<'static> {
        Fontinfo::new(
            &deja_vu_sans::GLYPH_POINTS,
            &deja_vu_sans::GLYPH_POINT_INDICES,
            &deja_vu_sans::GLYPH_INSTRUCTIONS,
            &deja_vu_sans::GLYPH_INSTRUCTION_INDICES,
            &deja_vu_sans::GLYPH_INSTRUCTION_COUNTS,
            &deja_vu_sans::GLYPH_ADVANCES,
            &deja_vu_sans::CHARACTER_MAP,
            deja_vu_sans::GLYPH_COUNT,
            deja_vu_sans::DESCENDER_HEIGHT,
            deja_vu_sans::FONT_HEIGHT,
        )
    }

    pub fn sans_mono() -> Fontinfo<'static> {
        Fontinfo::new(
            &deja_vu_sans_mono::GLYPH_POINTS,
            &deja_vu_sans_mono::GLYPH_POINT_INDICES,
            &deja_vu_sans_mono::GLYPH_INSTRUCTIONS,
            &deja_vu_sans_mono::GLYPH_INSTRUCTION_INDICES,
            &deja_vu_sans_mono::GLYPH_INSTRUCTION_COUNTS,
            &deja_vu_sans_mono::GLYPH_ADVANCES,
            &deja_vu_sans_mono::CHARACTER_MAP,
            deja_vu_sans_mono::GLYPH_COUNT,
            deja_vu_sans_mono::DESCENDER_HEIGHT,
            deja_vu_sans_mono::FONT_HEIGHT,
        )
    }
}

// text_width returns the width of a text string at the specified font and size.
pub fn text_width(s: &str, f: &Fontinfo, pointsize: u32) -> f32 {
    let mut tw: VGfloat = 0.0;
    let size = pointsize as VGfloat;
    for c in s.chars() {
        let glyph_index = f.character_map[c as usize];
        if glyph_index != -1 {
            tw += size * f.glyph_advances[glyph_index as usize] as f32 / 65536.0;
        }
    }
    return tw as f32;
}

// text renders a string of text at a specified location, size, using the specified font glyphs
pub fn text(x: VGfloat, y: VGfloat, s: &str, f: &Fontinfo, pointsize: u32) {
    let size = pointsize as VGfloat;
    let mut xx = x;
    let mm: [VGfloat; 9] = [0.0; 9];
    unsafe {
        vgGetMatrix(&mm as *const VGfloat);
        for c in s.chars() {
            let glyph_index = f.character_map[c as usize];
            if glyph_index == -1 {
                continue;
            }
            let mat: [VGfloat; 9] = [size, 0.0, 0.0, 0.0, size, 0.0, xx, y, 1.0];
            vgLoadMatrix(&mm as *const VGfloat);
            vgMultMatrix(&mat as *const VGfloat);
            let path = f.glyphs[glyph_index as usize];
            vgDrawPath(
                path,
                VGPaintMode::VG_FILL_PATH as u32 | VGPaintMode::VG_STROKE_PATH as u32,
            );
            xx += size * f.glyph_advances[glyph_index as usize] as f32 / 65536.0;
        }
        vgLoadMatrix(&mm as *const VGfloat);
    }
}

// text_mid draws text, centered on (x,y)
pub fn text_mid(x: VGfloat, y: VGfloat, s: &str, f: &Fontinfo, pointsize: u32) {
    let tw = text_width(s, f, pointsize);
    text(x - (tw / 2.0), y, s, f, pointsize);
}

#[link(name = "wee")]
extern "C" {
    fn WKMain(f: extern "C" fn(u32, u32) -> ()) -> i64;
}

pub fn main(f: extern "C" fn(u32, u32) -> ()) -> i64 {
    unsafe {
        return WKMain(f);
    }
}

fn new_path() -> VGPath {
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

// rect makes a rectangle at the specified location and dimensions
pub fn rect(x: VGfloat, y: VGfloat, w: VGfloat, h: VGfloat) {
    let path = new_path();
    unsafe {
        vguRect(path, x, y, w, h);
        vgDrawPath(
            path,
            VGPaintMode::VG_FILL_PATH as u32 | VGPaintMode::VG_STROKE_PATH as u32,
        );
        vgDestroyPath(path);
    }
}

// line makes a line from (x1,y1) to (x2,y2)
pub fn line(x1: VGfloat, y1: VGfloat, x2: VGfloat, y2: VGfloat) {
    let path = new_path();
    unsafe {
        vguLine(path, x1, y1, x2, y2);
        vgDrawPath(path, VGPaintMode::VG_STROKE_PATH as u32);
        vgDestroyPath(path);
    }
}

// round_rect makes an rounded rectangle at the specified location and dimensions
pub fn round_rect(x: VGfloat, y: VGfloat, w: VGfloat, h: VGfloat, rw: VGfloat, rh: VGfloat) {
    let path = new_path();
    unsafe {
        vguRoundRect(path, x, y, w, h, rw, rh);
        vgDrawPath(
            path,
            VGPaintMode::VG_FILL_PATH as u32 | VGPaintMode::VG_STROKE_PATH as u32,
        );
        vgDestroyPath(path);
    }
}

// Ellipse makes an ellipse at the specified location and dimensions
pub fn ellipse(x: VGfloat, y: VGfloat, w: VGfloat, h: VGfloat) {
    let path = new_path();
    unsafe {
        vguEllipse(path, x, y, w, h);
        vgDrawPath(
            path,
            VGPaintMode::VG_FILL_PATH as u32 | VGPaintMode::VG_STROKE_PATH as u32,
        );
        vgDestroyPath(path);
    }
}

// Circle makes a circle at the specified location and dimensions
pub fn circle(x: VGfloat, y: VGfloat, r: VGfloat) {
    ellipse(x, y, r, r);
}

// RGBA fills a color vectors from a RGBA quad.
pub fn rgba(r: u32, g: u32, b: u32, a: VGfloat) -> [VGfloat; 4] {
    let mut color: [VGfloat; 4] = [0.0, 0.0, 0.0, 1.0];
    if r <= 255 {
        color[0] = r as VGfloat / 255.0;
    }
    if g <= 255 {
        color[1] = g as VGfloat / 255.0;
    }
    if b <= 255 {
        color[2] = b as VGfloat / 255.0;
    }
    if a >= 0.0 && a <= 1.0 {
        color[3] = a;
    }
    return color;
}

// RGB returns a solid color from a RGB triple
pub fn rgb(r: u32, g: u32, b: u32) -> [VGfloat; 4] {
    return rgba(r, g, b, 1.0);
}

// Stroke sets the stroke color, defined as a RGB triple.
pub fn stroke(r: u32, g: u32, b: u32, a: VGfloat) {
    let color = rgba(r, g, b, a);
    set_stroke(&color);
}

// Fill sets the fillcolor, defined as a RGBA quad.
pub fn fill(r: u32, g: u32, b: u32, a: VGfloat) {
    let color = rgba(r, g, b, a);
    set_fill(&color);
}

// set_fill sets the fill color
pub fn set_fill(color: &[VGfloat]) {
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
        vgSetPaint(fill_paint, VGPaintMode::VG_FILL_PATH as u32);
        vgDestroyPaint(fill_paint);
    }
}

// set_stroke sets the stroke color
pub fn set_stroke(color: &[VGfloat]) {
    unsafe {
        let stroke_paint = vgCreatePaint();
        vgSetParameteri(
            stroke_paint,
            VGPaintParamType::VG_PAINT_TYPE as i32,
            VGPaintType::VG_PAINT_TYPE_COLOR as i32,
        );
        vgSetParameterfv(
            stroke_paint,
            VGPaintParamType::VG_PAINT_COLOR as i32,
            4,
            color.as_ptr(),
        );
        vgSetPaint(stroke_paint, VGPaintMode::VG_STROKE_PATH as u32);
        vgDestroyPaint(stroke_paint);
    }
}

// StrokeWidth sets the stroke width
pub fn stroke_width(width: VGfloat) {
    unsafe {
        vgSetf(VGParamType::VG_STROKE_LINE_WIDTH, width);
        vgSeti(
            VGParamType::VG_STROKE_CAP_STYLE,
            VGCapStyle::VG_CAP_BUTT as i32,
        );
        vgSeti(
            VGParamType::VG_STROKE_JOIN_STYLE,
            VGJoinStyle::VG_JOIN_MITER as i32,
        );
    }
}
