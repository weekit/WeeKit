//! Representation of OpenVG fonts.

mod deja_vu_serif;
mod deja_vu_sans;
mod deja_vu_sans_mono;

use super::openvg::*;

/// Represents an OpenVG font.
pub struct Font<'a> {
    pub character_map: &'a [i16],
    pub glyph_advances: &'a [i32],
    pub glyph_count: i32,
    pub descender_height: i32,
    pub font_height: i32,
    pub glyphs: [VGPath; 500],
}

impl<'a> Drop for Font<'a> {
    fn drop(&mut self) {
        for i in 0..self.glyph_count {
            unsafe { vgDestroyPath(self.glyphs[i as usize]) }
        }
    }
}

impl<'a> Font<'a> {
    /// Creates a new Font.
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
    ) -> Font<'a> {
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

        Font {
            character_map: character_map,
            glyph_advances: glyph_advances,
            glyph_count: glyph_count,
            descender_height: descender_height,
            font_height: font_height,
            glyphs: glyphs,
        }
    }

    /// Creates a deja_vu_serif font.
    pub fn serif() -> Font<'a> {
        Font::new(
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

    /// Creates a deja_vu_sans font.
    pub fn sans() -> Font<'a> {
        Font::new(
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

    /// Creates a deja_vu_sans_mono font.
    pub fn sans_mono() -> Font<'a> {
        Font::new(
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
