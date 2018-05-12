//! Support for OpenVG-based displays.

use openvg::*;
use draw;

/// Represents a device screen.
pub struct Screen {
    w: u32,
    h: u32,
}

impl Screen {
    /// Creates a new Screen.
    pub fn new(w: u32, h: u32) -> Screen {
        let screen = Screen { w: w, h: h };
        screen.background(255, 255, 255);
        draw::reset();
        unsafe {
            vgLoadIdentity();
        }
        screen
    }

    /// Clears the screen to a solid background color.
    pub fn background(&self, r: u32, g: u32, b: u32) {
        let color = draw::rgb(r, g, b);
        unsafe {
            vgSetfv(VGParamType::VG_CLEAR_COLOR, 4, color.as_ptr());
            vgClear(0, 0, self.w, self.h);
        }
    }

    /// Clears the screen to a background color with alpha.
    pub fn background_rgb(&self, r: u32, g: u32, b: u32, a: f32) {
        let color = draw::rgba(r, g, b, a);
        unsafe {
            vgSetfv(VGParamType::VG_CLEAR_COLOR, 4, color.as_ptr());
            vgClear(0, 0, self.w, self.h);
        }
    }

    /// Clears the window to previously set background colour.
    pub fn window_clear(&self) {
        unsafe {
            vgClear(0, 0, self.w, self.h);
        }
    }

    /// Clears a given rectangle in window coordinates (unaffected by transformations).
    pub fn area_clear(x: u32, y: u32, w: u32, h: u32) {
        unsafe {
            vgClear(x, y, w, h);
        }
    }
}
