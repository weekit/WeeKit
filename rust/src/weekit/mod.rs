#![allow(dead_code)]

mod openvg;

pub mod input;
pub mod font;

extern crate libc;

use self::openvg::*;
use self::font::*;

use std::sync::Arc;
use std::sync::Mutex;

#[derive(Copy, Clone, Debug)]
pub struct Event {
    pub slot: usize, // A unique integer for tracking a touch.
    pub kind: i32,   // 1 = TouchDown, 2 = TouchMoved, 3 = TouchEnded
    pub x: i32,      // X position on the touchpad.
    pub y: i32,      // Y position on the touchpad.
    pub sec: i32,
    pub usec: i32,
}

impl Event {
    pub fn new(slot: usize, kind: i32, x: i32, y: i32, sec: i32, usec: i32) -> Event {
        Event {
            slot: slot,
            kind: kind,
            x: x,
            y: y,
            sec: sec,
            usec: usec,
        }
    }
}

#[link(name = "wee")]
extern "C" {
    fn WKMain(f: extern "C" fn(u32, u32) -> (), e: extern "C" fn(u16, u16, i32) -> ()) -> i64;
}

pub trait Application {
    fn draw(&mut self, width: u32, height: u32) -> ();
    fn event(&mut self, event: &Event) -> ();
}

static mut APPLICATION: Option<Arc<Mutex<Application>>> = None;

static mut TOUCH_PAD: Option<input::TouchPad> = None;

fn draw(x: Arc<Mutex<Application>>, width: u32, height: u32) {
    let d = x.clone();
    let mut app = d.lock().unwrap();
    app.draw(width, height);
}

extern "C" fn draw_handler_wrapper(width: u32, height: u32) -> () {
    unsafe {
        match APPLICATION {
            Some(ref app) => draw(app.clone(), width, height),
            None => {}
        }
    }
}

extern "C" fn event_handler_wrapper(t: u16, c: u16, v: i32) -> () {
    unsafe {
        match TOUCH_PAD {
            Some(ref mut touchpad) => touchpad.handle(t, c, v, APPLICATION.clone()),
            None => {}
        }
    }
}

// main should be called from client applications to run the main event loop.
pub fn main<T: Application + 'static>(application: T) -> i64 {
    unsafe {
        APPLICATION = Some(Arc::new(Mutex::new(application)));
        TOUCH_PAD = Some(input::TouchPad::new());
        return WKMain(draw_handler_wrapper, event_handler_wrapper);
    }
}

pub struct Screen {
    w: u32,
    h: u32,
}

impl Screen {
    // new creates a new Screen.
    pub fn new(w: u32, h: u32) -> Screen {
        let screen = Screen { w: w, h: h };
        screen.background(255, 255, 255);
        fill(0, 0, 0, 1.0);
        stroke(0, 0, 0, 1.0);
        stroke_width(0.0);
        unsafe {
            vgLoadIdentity();
        }
        screen
    }

    // backgroud clears the screen to a solid background color.
    pub fn background(&self, r: u32, g: u32, b: u32) {
        let color = rgb(r, g, b);
        unsafe {
            vgSetfv(VGParamType::VG_CLEAR_COLOR, 4, color.as_ptr());
            vgClear(0, 0, self.w, self.h);
        }
    }

    // background_rgb clears the screen to a background color with alpha.
    pub fn background_rgb(&self, r: u32, g: u32, b: u32, a: f32) {
        let color = rgba(r, g, b, a);
        unsafe {
            vgSetfv(VGParamType::VG_CLEAR_COLOR, 4, color.as_ptr());
            vgClear(0, 0, self.w, self.h);
        }
    }
    // window_clear clears the window to previously set background colour.
    pub fn window_clear(&self) {
        unsafe {
            vgClear(0, 0, self.w, self.h);
        }
    }

    // area_clear clears a given rectangle in window coordinates.
    // (not affected by transformations)
    pub fn area_clear(x: u32, y: u32, w: u32, h: u32) {
        unsafe {
            vgClear(x, y, w, h);
        }
    }
}

// text_width returns the width of a text string at the specified font and size.
pub fn text_width(s: &str, f: &Font, pointsize: u32) -> f32 {
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

// text renders a string of text at a specified location, size, using the specified font glyphs.
pub fn text(x: VGfloat, y: VGfloat, s: &str, f: &Font, pointsize: u32) {
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

// text_mid draws text centered on (x,y).
pub fn text_mid(x: VGfloat, y: VGfloat, s: &str, f: &Font, pointsize: u32) {
    let tw = text_width(s, f, pointsize);
    text(x - (tw / 2.0), y, s, f, pointsize);
}

// text_end draws text with its end aligned to (x,y).
pub fn text_end(x: VGfloat, y: VGfloat, s: &str, f: &Font, pointsize: u32) {
    let tw = text_width(s, f, pointsize);
    text(x - tw, y, s, f, pointsize);
}

// text_height reports a font's height.
pub fn text_height(f: &Font, pointsize: u32) -> VGfloat {
    return (f.font_height * pointsize as i32) as VGfloat / 65536.0;
}

// text_depth reports a font's depth (how far under the baseline it goes).
pub fn text_depth(f: &Font, pointsize: u32) -> VGfloat {
    return (-f.descender_height * pointsize as i32) as VGfloat / 65536.0;
}

//
// Transformations
//

// translate translates the coordinate system to x,y.
pub fn translate(x: VGfloat, y: VGfloat) {
    unsafe {
        vgTranslate(x, y);
    }
}

// rotate rotates the coordinate system around angle r.
pub fn rotate(r: VGfloat) {
    unsafe {
        vgRotate(r);
    }
}

// shear shears the x coordinate by x degrees, the y coordinate by y degrees.
pub fn shear(x: VGfloat, y: VGfloat) {
    unsafe {
        vgShear(x, y);
    }
}

// scale scales by x, y.
pub fn scale(x: VGfloat, y: VGfloat) {
    unsafe {
        vgScale(x, y);
    }
}

//
// Style functions
//

// set_fill sets the fill color.
fn set_fill(color: &[VGfloat]) {
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

// set_stroke sets the stroke color.
fn set_stroke(color: &[VGfloat]) {
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

// stroke_width sets the stroke width.
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

//
// Color functions
//

// rgba fills a color vectors from a RGBA quad.
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

// rgb returns a solid color from a RGB triple.
pub fn rgb(r: u32, g: u32, b: u32) -> [VGfloat; 4] {
    return rgba(r, g, b, 1.0);
}

// stroke sets the stroke color, defined as a RGB triple.
pub fn stroke(r: u32, g: u32, b: u32, a: VGfloat) {
    let color = rgba(r, g, b, a);
    set_stroke(&color);
}

// fill sets the fillcolor, defined as a RGBA quad.
pub fn fill(r: u32, g: u32, b: u32, a: VGfloat) {
    let color = rgba(r, g, b, a);
    set_fill(&color);
}

// set_stops sets color stops for gradients.
pub fn set_stop(paint: VGPaint, stops: &[VGfloat], n: i32) {
    unsafe {
        let multmode = VGboolean::VG_FALSE;
        let spreadmode = VGColorRampSpreadMode::VG_COLOR_RAMP_SPREAD_REPEAT;
        vgSetParameteri(
            paint,
            VGPaintParamType::VG_PAINT_COLOR_RAMP_SPREAD_MODE as i32,
            spreadmode as i32,
        );
        vgSetParameteri(
            paint,
            VGPaintParamType::VG_PAINT_COLOR_RAMP_PREMULTIPLIED as i32,
            multmode as i32,
        );
        vgSetParameterfv(
            paint,
            VGPaintParamType::VG_PAINT_COLOR_RAMP_STOPS as i32,
            5 * n,
            stops.as_ptr(),
        );
        vgSetPaint(paint, VGPaintMode::VG_FILL_PATH as u32);
    }
}

// fill_linear_gradient fills with a linear gradient.
pub fn fill_linear_gradient(
    x1: VGfloat,
    y1: VGfloat,
    x2: VGfloat,
    y2: VGfloat,
    stops: &[VGfloat],
    ns: i32,
) {
    unsafe {
        let lgcoord: [VGfloat; 4] = [x1, y1, x2, y2];
        let paint = vgCreatePaint();
        vgSetParameteri(
            paint,
            VGPaintParamType::VG_PAINT_TYPE as i32,
            VGPaintType::VG_PAINT_TYPE_LINEAR_GRADIENT as i32,
        );
        vgSetParameterfv(
            paint,
            VGPaintParamType::VG_PAINT_LINEAR_GRADIENT as i32,
            4,
            lgcoord.as_ptr(),
        );
        set_stop(paint, stops, ns);
        vgDestroyPaint(paint);
    }
}

// fill_radial_gradient fills with a radial gradient.
pub fn fill_radial_gradient(
    cx: VGfloat,
    cy: VGfloat,
    fx: VGfloat,
    fy: VGfloat,
    radius: VGfloat,
    stops: &[VGfloat],
    ns: i32,
) {
    unsafe {
        let radialcoord: [VGfloat; 5] = [cx, cy, fx, fy, radius];
        let paint = vgCreatePaint();
        vgSetParameteri(
            paint,
            VGPaintParamType::VG_PAINT_TYPE as i32,
            VGPaintType::VG_PAINT_TYPE_RADIAL_GRADIENT as i32,
        );
        vgSetParameterfv(
            paint,
            VGPaintParamType::VG_PAINT_RADIAL_GRADIENT as i32,
            5,
            radialcoord.as_ptr(),
        );
        set_stop(paint, stops, ns);
        vgDestroyPaint(paint);
    }
}

// clip_rect limits the drawing area to specified rectangle.
pub fn clip_rect(x: VGint, y: VGint, w: VGint, h: VGint) {
    unsafe {
        vgSeti(VGParamType::VG_SCISSORING, VGboolean::VG_TRUE as i32);
        let coords: [VGint; 4] = [x, y, w, h];
        vgSetiv(VGParamType::VG_SCISSOR_RECTS, 4, coords.as_ptr());
    }
}

// clip_end stops limiting drawing area to specified rectangle.
pub fn clip_end() {
    unsafe {
        vgSeti(VGParamType::VG_SCISSORING, VGboolean::VG_FALSE as i32);
    }
}

//
// Shape functions
//

// new_path creates a path for internal use.
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

// make_curve makes path data using specified segments and coordinates.
pub fn make_curve(segments: &[VGubyte], coords: &[VGfloat], flags: VGbitfield) {
    let path = new_path();
    unsafe {
        vgAppendPathData(path, 2, segments.as_ptr(), coords.as_ptr() as *const i8);
        vgDrawPath(path, flags);
        vgDestroyPath(path);
    }
}

// cbezier makes a cubic bezier curve.
pub fn cbezier(
    sx: VGfloat,
    sy: VGfloat,
    cx: VGfloat,
    cy: VGfloat,
    px: VGfloat,
    py: VGfloat,
    ex: VGfloat,
    ey: VGfloat,
) {
    let segments: [VGubyte; 2] = [
        VGPathCommand::VG_MOVE_TO_ABS as VGubyte,
        VGPathSegment::VG_CUBIC_TO as VGubyte,
    ];
    let coords: [VGfloat; 8] = [sx, sy, cx, cy, px, py, ex, ey];
    make_curve(
        &segments,
        &coords,
        VGPaintMode::VG_FILL_PATH as u32 | VGPaintMode::VG_STROKE_PATH as u32,
    );
}

// qbezier makes a quadratic bezier curve.
pub fn qbezier(sx: VGfloat, sy: VGfloat, cx: VGfloat, cy: VGfloat, ex: VGfloat, ey: VGfloat) {
    let segments: [VGubyte; 2] = [
        VGPathCommand::VG_MOVE_TO_ABS as VGubyte,
        VGPathSegment::VG_QUAD_TO as VGubyte,
    ];
    let coords: [VGfloat; 6] = [sx, sy, cx, cy, ex, ey];
    make_curve(
        &segments,
        &coords,
        VGPaintMode::VG_FILL_PATH as u32 | VGPaintMode::VG_STROKE_PATH as u32,
    );
}

// interleave interleaves arrays of x, y into a single array.
pub fn interleave(x: &[VGfloat], y: &[VGfloat], n: i32, points: &mut [VGfloat]) {
    for i in 0..(n as usize) {
        points[2 * i] = x[i];
        points[2 * i + 1] = y[i];
    }
}

// poly makes either a polygon or polyline.
pub fn poly(x: &[VGfloat], y: &[VGfloat], n: VGint, flag: VGbitfield) {
    let mut points = vec![0.0f32; (n as usize) * 2];
    let path = new_path();
    interleave(x, y, n, points.as_mut_slice());
    unsafe {
        vguPolygon(path, points.as_ptr(), n, VGboolean::VG_FALSE);
        vgDrawPath(path, flag);
        vgDestroyPath(path);
    }
}

// polygon makes a filled polygon with vertices in x, y arrays.
pub fn polygon(x: &[VGfloat], y: &[VGfloat], n: i32) {
    poly(x, y, n, VGPaintMode::VG_FILL_PATH as u32);
}

// polyline makes a polyline with vertices at x, y arrays.
pub fn polyline(x: &[VGfloat], y: &[VGfloat], n: i32) {
    poly(x, y, n, VGPaintMode::VG_STROKE_PATH as u32);
}

// rect makes a rectangle at the specified location and dimensions.
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

// line makes a line from (x1,y1) to (x2,y2).
pub fn line(x1: VGfloat, y1: VGfloat, x2: VGfloat, y2: VGfloat) {
    let path = new_path();
    unsafe {
        vguLine(path, x1, y1, x2, y2);
        vgDrawPath(path, VGPaintMode::VG_STROKE_PATH as u32);
        vgDestroyPath(path);
    }
}

// round_rect makes a rounded rectangle at the specified location and dimensions.
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

// ellipse makes an ellipse at the specified location and dimensions.
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

// circle makes a circle at the specified location and dimensions.
pub fn circle(x: VGfloat, y: VGfloat, r: VGfloat) {
    ellipse(x, y, r, r);
}

// arc makes an elliptical arc at the specified location and dimensions.
pub fn arc(x: VGfloat, y: VGfloat, w: VGfloat, h: VGfloat, sa: VGfloat, aext: VGfloat) {
    let path = new_path();
    unsafe {
        vguArc(path, x, y, w, h, sa, aext, VGUArcType::VGU_ARC_OPEN);
        vgDrawPath(
            path,
            VGPaintMode::VG_FILL_PATH as u32 | VGPaintMode::VG_STROKE_PATH as u32,
        );
        vgDestroyPath(path);
    }
}

// Outlined shapes
// Hollow shapes -because filling still happens even with a fill of 0,0,0,0
// unlike where using a strokewidth of 0 disables the stroke.
// Either this or change the original functions to require the VG_x_PATH flags

// cbezier_outline makes a cubic bezier curve, stroked.
pub fn cbezier_outline(
    sx: VGfloat,
    sy: VGfloat,
    cx: VGfloat,
    cy: VGfloat,
    px: VGfloat,
    py: VGfloat,
    ex: VGfloat,
    ey: VGfloat,
) {
    let segments: [VGubyte; 2] = [
        VGPathCommand::VG_MOVE_TO_ABS as VGubyte,
        VGPathSegment::VG_CUBIC_TO as VGubyte,
    ];
    let coords: [VGfloat; 8] = [sx, sy, cx, cy, px, py, ex, ey];
    make_curve(&segments, &coords, VGPaintMode::VG_STROKE_PATH as u32);
}

// qbezier_outline makes a quadratic bezier curve, outlined.
pub fn qbezier_outline(
    sx: VGfloat,
    sy: VGfloat,
    cx: VGfloat,
    cy: VGfloat,
    ex: VGfloat,
    ey: VGfloat,
) {
    let segments: [VGubyte; 2] = [
        VGPathCommand::VG_MOVE_TO_ABS as VGubyte,
        VGPathSegment::VG_QUAD_TO as VGubyte,
    ];
    let coords: [VGfloat; 6] = [sx, sy, cx, cy, ex, ey];
    make_curve(&segments, &coords, VGPaintMode::VG_STROKE_PATH as u32);
}

// rect_outline makes a rectangle at the specified location and dimensions, outlined.
pub fn rect_outline(x: VGfloat, y: VGfloat, w: VGfloat, h: VGfloat) {
    let path = new_path();
    unsafe {
        vguRect(path, x, y, w, h);
        vgDrawPath(path, VGPaintMode::VG_STROKE_PATH as u32);
        vgDestroyPath(path);
    }
}

// roundrect_outline makes a rounded rectangle at the specified location and dimensions, outlined.
pub fn roundrect_outline(x: VGfloat, y: VGfloat, w: VGfloat, h: VGfloat, rw: VGfloat, rh: VGfloat) {
    let path = new_path();
    unsafe {
        vguRoundRect(path, x, y, w, h, rw, rh);
        vgDrawPath(path, VGPaintMode::VG_STROKE_PATH as u32);
        vgDestroyPath(path);
    }
}

// ellipse_outline makes an ellipse at the specified location and dimensions, outlined.
pub fn ellipse_outline(x: VGfloat, y: VGfloat, w: VGfloat, h: VGfloat) {
    let path = new_path();
    unsafe {
        vguEllipse(path, x, y, w, h);
        vgDrawPath(path, VGPaintMode::VG_STROKE_PATH as u32);
        vgDestroyPath(path);
    }
}

// circle_outline makes a circle at the specified location and dimensions, outlined.
pub fn circle_outline(x: VGfloat, y: VGfloat, r: VGfloat) {
    ellipse_outline(x, y, r, r);
}

// arc_outline makes an elliptical arc at the specified location and dimensions, outlined.
pub fn arc_outline(x: VGfloat, y: VGfloat, w: VGfloat, h: VGfloat, sa: VGfloat, aext: VGfloat) {
    let path = new_path();
    unsafe {
        vguArc(path, x, y, w, h, sa, aext, VGUArcType::VGU_ARC_OPEN);
        vgDrawPath(path, VGPaintMode::VG_STROKE_PATH as u32);
        vgDestroyPath(path);
    }
}
