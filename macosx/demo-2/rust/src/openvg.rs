#![allow(dead_code)]
#![allow(non_camel_case_types)]

// OpenVG types

pub type VGfloat = f32;
pub type VGbyte = i8;
pub type VGubyte = u8;
pub type VGshort = i16;
pub type VGint = i32;
pub type VGuint = u32;
pub type VGbitfield = u32;

pub const VG_MAX_ENUM: isize = 0x7FFFFFFF;

pub const VG_PATH_FORMAT_STANDARD: VGint = 0;

#[repr(C)]
pub enum VGPathDatatype {
    VG_PATH_DATATYPE_S_8 = 0,
    VG_PATH_DATATYPE_S_16 = 1,
    VG_PATH_DATATYPE_S_32 = 2,
    VG_PATH_DATATYPE_F = 3,

    VG_PATH_DATATYPE_FORCE_SIZE = VG_MAX_ENUM,
}

pub type VGHandle = u32;
pub type VGPath = VGHandle;
pub type VGImage = VGHandle;
pub type VGMaskLayer = VGHandle;
pub type VGFont = VGHandle;
pub type VGPaint = VGHandle;

#[repr(C)]
pub enum VGPaintMode {
    VG_STROKE_PATH = (1 << 0),
    VG_FILL_PATH = (1 << 1),

    VG_PAINT_MODE_FORCE_SIZE = VG_MAX_ENUM,
}

#[repr(C)]
pub enum VGPathCapabilities {
    VG_PATH_CAPABILITY_APPEND_FROM = (1 << 0),
    VG_PATH_CAPABILITY_APPEND_TO = (1 << 1),
    VG_PATH_CAPABILITY_MODIFY = (1 << 2),
    VG_PATH_CAPABILITY_TRANSFORM_FROM = (1 << 3),
    VG_PATH_CAPABILITY_TRANSFORM_TO = (1 << 4),
    VG_PATH_CAPABILITY_INTERPOLATE_FROM = (1 << 5),
    VG_PATH_CAPABILITY_INTERPOLATE_TO = (1 << 6),
    VG_PATH_CAPABILITY_PATH_LENGTH = (1 << 7),
    VG_PATH_CAPABILITY_POINT_ALONG_PATH = (1 << 8),
    VG_PATH_CAPABILITY_TANGENT_ALONG_PATH = (1 << 9),
    VG_PATH_CAPABILITY_PATH_BOUNDS = (1 << 10),
    VG_PATH_CAPABILITY_PATH_TRANSFORMED_BOUNDS = (1 << 11),
    VG_PATH_CAPABILITY_ALL = ((1 << 12) - 1),
	
    VG_PATH_CAPABILITIES_FORCE_SIZE = VG_MAX_ENUM,
}

pub enum VGPaintParamType {
    // Color paint parameters
    VG_PAINT_TYPE = 0x1A00,
    VG_PAINT_COLOR = 0x1A01,
    VG_PAINT_COLOR_RAMP_SPREAD_MODE = 0x1A02,
    VG_PAINT_COLOR_RAMP_PREMULTIPLIED = 0x1A07,
    VG_PAINT_COLOR_RAMP_STOPS = 0x1A03,
    // Linear gradient paint parameters
    VG_PAINT_LINEAR_GRADIENT = 0x1A04,
    // Radial gradient paint parameters
    VG_PAINT_RADIAL_GRADIENT = 0x1A05,
    // Pattern paint parameters
    VG_PAINT_PATTERN_TILING_MODE = 0x1A06,
    
	VG_PAINT_PARAM_TYPE_FORCE_SIZE = VG_MAX_ENUM,
}

pub enum VGPaintType {
    VG_PAINT_TYPE_COLOR = 0x1B00,
    VG_PAINT_TYPE_LINEAR_GRADIENT = 0x1B01,
    VG_PAINT_TYPE_RADIAL_GRADIENT = 0x1B02,
    VG_PAINT_TYPE_PATTERN = 0x1B03,

    VG_PAINT_TYPE_FORCE_SIZE = VG_MAX_ENUM,
}

#[link(name = "AmanithVG.4")]
extern "C" {
    pub fn vgCreatePath(
        pathFormat: VGint,
        datatype: VGPathDatatype,
        scale: VGfloat,
        bias: VGfloat,
        segmentCapacityHint: VGint,
        coordCapacityHint: VGint,
        capabilities: VGbitfield,
    ) -> VGPath;
    pub fn vguRect(path: VGPath, x: VGfloat, y: VGfloat, w: VGfloat, h: VGfloat);
    pub fn vgDrawPath(path: VGPath, paintModes: VGbitfield);
    pub fn vgDestroyPath(path: VGPath);

    pub fn vgCreatePaint() -> VGPaint;
    pub fn vgSetPaint(paint: VGPaint, paintModes: VGbitfield);
    pub fn vgDestroyPaint(paint: VGPaint);
    pub fn vgSetParameteri(object: VGHandle, paramType: VGint, value: VGint);
    pub fn vgSetParameterfv(
        object: VGHandle,
        paramType: VGint,
        count: VGint,
        values: &[VGfloat; 4],
    );
}
