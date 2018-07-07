// Copyright 2018 The WeeKit Authors. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License. 
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(dead_code)]
#![allow(non_camel_case_types)]

// vgplatform.h
pub type VGfloat = f32;
pub type VGbyte = i8;
pub type VGubyte = u8;
pub type VGshort = i16;
pub type VGint = i32;
pub type VGuint = u32;
pub type VGbitfield = u32;

// openvg.h
pub const VG_MAXSHORT: isize = 0x7FFF;
pub const VG_MAXINT: isize = 0x7FFFFFFF;
pub const VG_MAX_ENUM: isize = 0x7FFFFFFF;

pub type VGHandle = u32;
pub type VGPath = VGHandle;
pub type VGImage = VGHandle;
pub type VGMaskLayer = VGHandle;
pub type VGFont = VGHandle;
pub type VGPaint = VGHandle;

pub const VG_INVALID_HANDLE: VGHandle = 0;

#[repr(C)]
pub enum VGboolean {
    VG_FALSE = 0,
    VG_TRUE = 1,

    VG_BOOLEAN_FORCE_SIZE = VG_MAX_ENUM,
}

#[repr(C)]
pub enum VGErrorCode {
    VG_NO_ERROR = 0,
    VG_BAD_HANDLE_ERROR = 0x1000,
    VG_ILLEGAL_ARGUMENT_ERROR = 0x1001,
    VG_OUT_OF_MEMORY_ERROR = 0x1002,
    VG_PATH_CAPABILITY_ERROR = 0x1003,
    VG_UNSUPPORTED_IMAGE_FORMAT_ERROR = 0x1004,
    VG_UNSUPPORTED_PATH_FORMAT_ERROR = 0x1005,
    VG_IMAGE_IN_USE_ERROR = 0x1006,
    VG_NO_CONTEXT_ERROR = 0x1007,

    VG_ERROR_CODE_FORCE_SIZE = VG_MAX_ENUM,
}

#[repr(C)]
pub enum VGParamType {
    /* Mode settings */
    VG_MATRIX_MODE = 0x1100,
    VG_FILL_RULE = 0x1101,
    VG_IMAGE_QUALITY = 0x1102,
    VG_RENDERING_QUALITY = 0x1103,
    VG_BLEND_MODE = 0x1104,
    VG_IMAGE_MODE = 0x1105,

    /* Scissoring rectangles */
    VG_SCISSOR_RECTS = 0x1106,

    /* Color Transformation */
    VG_COLOR_TRANSFORM = 0x1170,
    VG_COLOR_TRANSFORM_VALUES = 0x1171,

    /* Stroke parameters */
    VG_STROKE_LINE_WIDTH = 0x1110,
    VG_STROKE_CAP_STYLE = 0x1111,
    VG_STROKE_JOIN_STYLE = 0x1112,
    VG_STROKE_MITER_LIMIT = 0x1113,
    VG_STROKE_DASH_PATTERN = 0x1114,
    VG_STROKE_DASH_PHASE = 0x1115,
    VG_STROKE_DASH_PHASE_RESET = 0x1116,

    /* Edge fill color for VG_TILE_FILL tiling mode */
    VG_TILE_FILL_COLOR = 0x1120,

    /* Color for vgClear */
    VG_CLEAR_COLOR = 0x1121,

    /* Glyph origin */
    VG_GLYPH_ORIGIN = 0x1122,

    /* Enable/disable alpha masking and scissoring */
    VG_MASKING = 0x1130,
    VG_SCISSORING = 0x1131,

    /* Pixel layout hint information */
    VG_PIXEL_LAYOUT = 0x1140,
    VG_SCREEN_LAYOUT = 0x1141,

    /* Source format selection for image filters */
    VG_FILTER_FORMAT_LINEAR = 0x1150,
    VG_FILTER_FORMAT_PREMULTIPLIED = 0x1151,

    /* Destination write enable mask for image filters */
    VG_FILTER_CHANNEL_MASK = 0x1152,

    /* Implementation limits (read-only) */
    VG_MAX_SCISSOR_RECTS = 0x1160,
    VG_MAX_DASH_COUNT = 0x1161,
    VG_MAX_KERNEL_SIZE = 0x1162,
    VG_MAX_SEPARABLE_KERNEL_SIZE = 0x1163,
    VG_MAX_COLOR_RAMP_STOPS = 0x1164,
    VG_MAX_IMAGE_WIDTH = 0x1165,
    VG_MAX_IMAGE_HEIGHT = 0x1166,
    VG_MAX_IMAGE_PIXELS = 0x1167,
    VG_MAX_IMAGE_BYTES = 0x1168,
    VG_MAX_FLOAT = 0x1169,
    VG_MAX_GAUSSIAN_STD_DEVIATION = 0x116A,

    VG_PARAM_TYPE_FORCE_SIZE = VG_MAX_ENUM,
}

#[repr(C)]
pub enum VGRenderingQuality {
    VG_RENDERING_QUALITY_NONANTIALIASED = 0x1200,
    VG_RENDERING_QUALITY_FASTER = 0x1201,
    VG_RENDERING_QUALITY_BETTER = 0x1202,

    VG_RENDERING_QUALITY_FORCE_SIZE = VG_MAX_ENUM,
}

#[repr(C)]
pub enum VGPixelLayout {
    VG_PIXEL_LAYOUT_UNKNOWN = 0x1300,
    VG_PIXEL_LAYOUT_RGB_VERTICAL = 0x1301,
    VG_PIXEL_LAYOUT_BGR_VERTICAL = 0x1302,
    VG_PIXEL_LAYOUT_RGB_HORIZONTAL = 0x1303,
    VG_PIXEL_LAYOUT_BGR_HORIZONTAL = 0x1304,

    VG_PIXEL_LAYOUT_FORCE_SIZE = VG_MAX_ENUM,
}

#[repr(C)]
pub enum VGMatrixMode {
    VG_MATRIX_PATH_USER_TO_SURFACE = 0x1400,
    VG_MATRIX_IMAGE_USER_TO_SURFACE = 0x1401,
    VG_MATRIX_FILL_PAINT_TO_USER = 0x1402,
    VG_MATRIX_STROKE_PAINT_TO_USER = 0x1403,
    VG_MATRIX_GLYPH_USER_TO_SURFACE = 0x1404,

    VG_MATRIX_MODE_FORCE_SIZE = VG_MAX_ENUM,
}

#[repr(C)]
pub enum VGMaskOperation {
    VG_CLEAR_MASK = 0x1500,
    VG_FILL_MASK = 0x1501,
    VG_SET_MASK = 0x1502,
    VG_UNION_MASK = 0x1503,
    VG_INTERSECT_MASK = 0x1504,
    VG_SUBTRACT_MASK = 0x1505,

    VG_MASK_OPERATION_FORCE_SIZE = VG_MAX_ENUM,
}

pub const VG_PATH_FORMAT_STANDARD: VGint = 0;

#[repr(C)]
pub enum VGPathDatatype {
    VG_PATH_DATATYPE_S_8 = 0,
    VG_PATH_DATATYPE_S_16 = 1,
    VG_PATH_DATATYPE_S_32 = 2,
    VG_PATH_DATATYPE_F = 3,

    VG_PATH_DATATYPE_FORCE_SIZE = VG_MAX_ENUM,
}

#[repr(C)]
pub enum VGPathAbsRel {
    VG_ABSOLUTE = 0,
    VG_RELATIVE = 1,

    VG_PATH_ABS_REL_FORCE_SIZE = VG_MAX_ENUM,
}

#[repr(C)]
pub enum VGPathSegment {
    VG_CLOSE_PATH = (0 << 1),
    VG_MOVE_TO = (1 << 1),
    VG_LINE_TO = (2 << 1),
    VG_HLINE_TO = (3 << 1),
    VG_VLINE_TO = (4 << 1),
    VG_QUAD_TO = (5 << 1),
    VG_CUBIC_TO = (6 << 1),
    VG_SQUAD_TO = (7 << 1),
    VG_SCUBIC_TO = (8 << 1),
    VG_SCCWARC_TO = (9 << 1),
    VG_SCWARC_TO = (10 << 1),
    VG_LCCWARC_TO = (11 << 1),
    VG_LCWARC_TO = (12 << 1),

    VG_PATH_SEGMENT_FORCE_SIZE = VG_MAX_ENUM,
}

#[repr(C)]
pub enum VGPathCommand {
    VG_MOVE_TO_ABS = VGPathSegment::VG_MOVE_TO as isize | VGPathAbsRel::VG_ABSOLUTE as isize,
    VG_MOVE_TO_REL = VGPathSegment::VG_MOVE_TO as isize | VGPathAbsRel::VG_RELATIVE as isize,
    VG_LINE_TO_ABS = VGPathSegment::VG_LINE_TO as isize | VGPathAbsRel::VG_ABSOLUTE as isize,
    VG_LINE_TO_REL = VGPathSegment::VG_LINE_TO as isize | VGPathAbsRel::VG_RELATIVE as isize,
    VG_HLINE_TO_ABS = VGPathSegment::VG_HLINE_TO as isize | VGPathAbsRel::VG_ABSOLUTE as isize,
    VG_HLINE_TO_REL = VGPathSegment::VG_HLINE_TO as isize | VGPathAbsRel::VG_RELATIVE as isize,
    VG_VLINE_TO_ABS = VGPathSegment::VG_VLINE_TO as isize | VGPathAbsRel::VG_ABSOLUTE as isize,
    VG_VLINE_TO_REL = VGPathSegment::VG_VLINE_TO as isize | VGPathAbsRel::VG_RELATIVE as isize,
    VG_QUAD_TO_ABS = VGPathSegment::VG_QUAD_TO as isize | VGPathAbsRel::VG_ABSOLUTE as isize,
    VG_QUAD_TO_REL = VGPathSegment::VG_QUAD_TO as isize | VGPathAbsRel::VG_RELATIVE as isize,
    VG_CUBIC_TO_ABS = VGPathSegment::VG_CUBIC_TO as isize | VGPathAbsRel::VG_ABSOLUTE as isize,
    VG_CUBIC_TO_REL = VGPathSegment::VG_CUBIC_TO as isize | VGPathAbsRel::VG_RELATIVE as isize,
    VG_SQUAD_TO_ABS = VGPathSegment::VG_SQUAD_TO as isize | VGPathAbsRel::VG_ABSOLUTE as isize,
    VG_SQUAD_TO_REL = VGPathSegment::VG_SQUAD_TO as isize | VGPathAbsRel::VG_RELATIVE as isize,
    VG_SCUBIC_TO_ABS = VGPathSegment::VG_SCUBIC_TO as isize | VGPathAbsRel::VG_ABSOLUTE as isize,
    VG_SCUBIC_TO_REL = VGPathSegment::VG_SCUBIC_TO as isize | VGPathAbsRel::VG_RELATIVE as isize,
    VG_SCCWARC_TO_ABS = VGPathSegment::VG_SCCWARC_TO as isize | VGPathAbsRel::VG_ABSOLUTE as isize,
    VG_SCCWARC_TO_REL = VGPathSegment::VG_SCCWARC_TO as isize | VGPathAbsRel::VG_RELATIVE as isize,
    VG_SCWARC_TO_ABS = VGPathSegment::VG_SCWARC_TO as isize | VGPathAbsRel::VG_ABSOLUTE as isize,
    VG_SCWARC_TO_REL = VGPathSegment::VG_SCWARC_TO as isize | VGPathAbsRel::VG_RELATIVE as isize,
    VG_LCCWARC_TO_ABS = VGPathSegment::VG_LCCWARC_TO as isize | VGPathAbsRel::VG_ABSOLUTE as isize,
    VG_LCCWARC_TO_REL = VGPathSegment::VG_LCCWARC_TO as isize | VGPathAbsRel::VG_RELATIVE as isize,
    VG_LCWARC_TO_ABS = VGPathSegment::VG_LCWARC_TO as isize | VGPathAbsRel::VG_ABSOLUTE as isize,
    VG_LCWARC_TO_REL = VGPathSegment::VG_LCWARC_TO as isize | VGPathAbsRel::VG_RELATIVE as isize,

    VG_PATH_COMMAND_FORCE_SIZE = VG_MAX_ENUM,
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

#[repr(C)]
pub enum VGPathParamType {
    VG_PATH_FORMAT = 0x1600,
    VG_PATH_DATATYPE = 0x1601,
    VG_PATH_SCALE = 0x1602,
    VG_PATH_BIAS = 0x1603,
    VG_PATH_NUM_SEGMENTS = 0x1604,
    VG_PATH_NUM_COORDS = 0x1605,

    VG_PATH_PARAM_TYPE_FORCE_SIZE = VG_MAX_ENUM,
}

#[repr(C)]
pub enum VGCapStyle {
    VG_CAP_BUTT = 0x1700,
    VG_CAP_ROUND = 0x1701,
    VG_CAP_SQUARE = 0x1702,

    VG_CAP_STYLE_FORCE_SIZE = VG_MAX_ENUM,
}

#[repr(C)]
pub enum VGJoinStyle {
    VG_JOIN_MITER = 0x1800,
    VG_JOIN_ROUND = 0x1801,
    VG_JOIN_BEVEL = 0x1802,

    VG_JOIN_STYLE_FORCE_SIZE = VG_MAX_ENUM,
}

#[repr(C)]
pub enum VGFillRule {
    VG_EVEN_ODD = 0x1900,
    VG_NON_ZERO = 0x1901,

    VG_FILL_RULE_FORCE_SIZE = VG_MAX_ENUM,
}

#[repr(C)]
pub enum VGPaintMode {
    VG_STROKE_PATH = (1 << 0),
    VG_FILL_PATH = (1 << 1),

    VG_PAINT_MODE_FORCE_SIZE = VG_MAX_ENUM,
}

#[repr(C)]
pub enum VGPaintParamType {
    /* Color paint parameters */
    VG_PAINT_TYPE = 0x1A00,
    VG_PAINT_COLOR = 0x1A01,
    VG_PAINT_COLOR_RAMP_SPREAD_MODE = 0x1A02,
    VG_PAINT_COLOR_RAMP_PREMULTIPLIED = 0x1A07,
    VG_PAINT_COLOR_RAMP_STOPS = 0x1A03,

    /* Linear gradient paint parameters */
    VG_PAINT_LINEAR_GRADIENT = 0x1A04,

    /* Radial gradient paint parameters */
    VG_PAINT_RADIAL_GRADIENT = 0x1A05,

    /* Pattern paint parameters */
    VG_PAINT_PATTERN_TILING_MODE = 0x1A06,

    VG_PAINT_PARAM_TYPE_FORCE_SIZE = VG_MAX_ENUM,
}

#[repr(C)]
pub enum VGPaintType {
    VG_PAINT_TYPE_COLOR = 0x1B00,
    VG_PAINT_TYPE_LINEAR_GRADIENT = 0x1B01,
    VG_PAINT_TYPE_RADIAL_GRADIENT = 0x1B02,
    VG_PAINT_TYPE_PATTERN = 0x1B03,

    VG_PAINT_TYPE_FORCE_SIZE = VG_MAX_ENUM,
}

#[repr(C)]
pub enum VGColorRampSpreadMode {
    VG_COLOR_RAMP_SPREAD_PAD = 0x1C00,
    VG_COLOR_RAMP_SPREAD_REPEAT = 0x1C01,
    VG_COLOR_RAMP_SPREAD_REFLECT = 0x1C02,

    VG_COLOR_RAMP_SPREAD_MODE_FORCE_SIZE = VG_MAX_ENUM,
}

#[repr(C)]
pub enum VGTilingMode {
    VG_TILE_FILL = 0x1D00,
    VG_TILE_PAD = 0x1D01,
    VG_TILE_REPEAT = 0x1D02,
    VG_TILE_REFLECT = 0x1D03,

    VG_TILING_MODE_FORCE_SIZE = VG_MAX_ENUM,
}

#[repr(C)]
pub enum VGImageFormat {
    VG_sRGBX_8888 = 0,
    VG_sRGBA_8888 = 1,
    VG_sRGBA_8888_PRE = 2,
    VG_sRGB_565 = 3,
    VG_sRGBA_5551 = 4,
    VG_sRGBA_4444 = 5,
    VG_sL_8 = 6,
    VG_lRGBX_8888 = 7,
    VG_lRGBA_8888 = 8,
    VG_lRGBA_8888_PRE = 9,
    VG_lL_8 = 10,
    VG_A_8 = 11,
    VG_BW_1 = 12,
    VG_A_1 = 13,
    VG_A_4 = 14,

    /* {A,X}RGB channel ordering */
    VG_sXRGB_8888 = 0 | (1 << 6),
    VG_sARGB_8888 = 1 | (1 << 6),
    VG_sARGB_8888_PRE = 2 | (1 << 6),
    VG_sARGB_1555 = 4 | (1 << 6),
    VG_sARGB_4444 = 5 | (1 << 6),
    VG_lXRGB_8888 = 7 | (1 << 6),
    VG_lARGB_8888 = 8 | (1 << 6),
    VG_lARGB_8888_PRE = 9 | (1 << 6),

    /* BGR{A,X} channel ordering */
    VG_sBGRX_8888 = 0 | (1 << 7),
    VG_sBGRA_8888 = 1 | (1 << 7),
    VG_sBGRA_8888_PRE = 2 | (1 << 7),
    VG_sBGR_565 = 3 | (1 << 7),
    VG_sBGRA_5551 = 4 | (1 << 7),
    VG_sBGRA_4444 = 5 | (1 << 7),
    VG_lBGRX_8888 = 7 | (1 << 7),
    VG_lBGRA_8888 = 8 | (1 << 7),
    VG_lBGRA_8888_PRE = 9 | (1 << 7),

    /* {A,X}BGR channel ordering */
    VG_sXBGR_8888 = 0 | (1 << 6) | (1 << 7),
    VG_sABGR_8888 = 1 | (1 << 6) | (1 << 7),
    VG_sABGR_8888_PRE = 2 | (1 << 6) | (1 << 7),
    VG_sABGR_1555 = 4 | (1 << 6) | (1 << 7),
    VG_sABGR_4444 = 5 | (1 << 6) | (1 << 7),
    VG_lXBGR_8888 = 7 | (1 << 6) | (1 << 7),
    VG_lABGR_8888 = 8 | (1 << 6) | (1 << 7),
    VG_lABGR_8888_PRE = 9 | (1 << 6) | (1 << 7),

    VG_IMAGE_FORMAT_FORCE_SIZE = VG_MAX_ENUM,
}

#[repr(C)]
pub enum VGImageQuality {
    VG_IMAGE_QUALITY_NONANTIALIASED = (1 << 0),
    VG_IMAGE_QUALITY_FASTER = (1 << 1),
    VG_IMAGE_QUALITY_BETTER = (1 << 2),

    VG_IMAGE_QUALITY_FORCE_SIZE = VG_MAX_ENUM,
}

#[repr(C)]
pub enum VGImageParamType {
    VG_IMAGE_FORMAT = 0x1E00,
    VG_IMAGE_WIDTH = 0x1E01,
    VG_IMAGE_HEIGHT = 0x1E02,

    VG_IMAGE_PARAM_TYPE_FORCE_SIZE = VG_MAX_ENUM,
}

#[repr(C)]
pub enum VGImageMode {
    VG_DRAW_IMAGE_NORMAL = 0x1F00,
    VG_DRAW_IMAGE_MULTIPLY = 0x1F01,
    VG_DRAW_IMAGE_STENCIL = 0x1F02,

    VG_IMAGE_MODE_FORCE_SIZE = VG_MAX_ENUM,
}

#[repr(C)]
pub enum VGImageChannel {
    VG_RED = (1 << 3),
    VG_GREEN = (1 << 2),
    VG_BLUE = (1 << 1),
    VG_ALPHA = (1 << 0),

    VG_IMAGE_CHANNEL_FORCE_SIZE = VG_MAX_ENUM,
}

#[repr(C)]
pub enum VGBlendMode {
    VG_BLEND_SRC = 0x2000,
    VG_BLEND_SRC_OVER = 0x2001,
    VG_BLEND_DST_OVER = 0x2002,
    VG_BLEND_SRC_IN = 0x2003,
    VG_BLEND_DST_IN = 0x2004,
    VG_BLEND_MULTIPLY = 0x2005,
    VG_BLEND_SCREEN = 0x2006,
    VG_BLEND_DARKEN = 0x2007,
    VG_BLEND_LIGHTEN = 0x2008,
    VG_BLEND_ADDITIVE = 0x2009,

    VG_BLEND_MODE_FORCE_SIZE = VG_MAX_ENUM,
}

#[repr(C)]
pub enum VGFontParamType {
    VG_FONT_NUM_GLYPHS = 0x2F00,

    VG_FONT_PARAM_TYPE_FORCE_SIZE = VG_MAX_ENUM,
}

#[repr(C)]
pub enum VGHardwareQueryType {
    VG_IMAGE_FORMAT_QUERY = 0x2100,
    VG_PATH_DATATYPE_QUERY = 0x2101,

    VG_HARDWARE_QUERY_TYPE_FORCE_SIZE = VG_MAX_ENUM,
}

#[repr(C)]
pub enum VGHardwareQueryResult {
    VG_HARDWARE_ACCELERATED = 0x2200,
    VG_HARDWARE_UNACCELERATED = 0x2201,

    VG_HARDWARE_QUERY_RESULT_FORCE_SIZE = VG_MAX_ENUM,
}

#[repr(C)]
pub enum VGStringID {
    VG_VENDOR = 0x2300,
    VG_RENDERER = 0x2301,
    VG_VERSION = 0x2302,
    VG_EXTENSIONS = 0x2303,

    VG_STRING_ID_FORCE_SIZE = VG_MAX_ENUM,
}

// vgu.h
#[repr(C)]
pub enum VGUErrorCode {
    VGU_NO_ERROR = 0,
    VGU_BAD_HANDLE_ERROR = 0xF000,
    VGU_ILLEGAL_ARGUMENT_ERROR = 0xF001,
    VGU_OUT_OF_MEMORY_ERROR = 0xF002,
    VGU_PATH_CAPABILITY_ERROR = 0xF003,
    VGU_BAD_WARP_ERROR = 0xF004,
}

#[repr(C)]
pub enum VGUArcType {
    VGU_ARC_OPEN = 0xF100,
    VGU_ARC_CHORD = 0xF101,
    VGU_ARC_PIE = 0xF102,
}

extern "C" {
    pub fn vgGetError() -> VGErrorCode;
    pub fn vgFlush();
    pub fn vgFinish();
    /* Getters and Setters */
    pub fn vgSetf(param_type: VGParamType, value: VGfloat);
    pub fn vgSeti(param_type: VGParamType, value: VGint);
    pub fn vgSetfv(param_type: VGParamType, count: VGint, values: *const VGfloat);
    pub fn vgSetiv(param_type: VGParamType, count: VGint, values: *const VGint);
    pub fn vgGetf(param_type: VGParamType) -> VGfloat;
    pub fn vgGeti(param_type: VGParamType) -> VGint;
    pub fn vgGetVectorSize(param_type: VGParamType) -> VGint;
    pub fn vgGetfv(param_type: VGParamType, count: VGint, values: *const VGfloat);
    pub fn vgGetiv(param_type: VGParamType, count: VGint, values: *const VGint);
    pub fn vgSetParameterf(object: VGHandle, paramType: VGint, value: VGfloat);
    pub fn vgSetParameteri(object: VGHandle, paramType: VGint, value: VGint);
    pub fn vgSetParameterfv(
        object: VGHandle,
        paramType: VGint,
        count: VGint,
        values: *const VGfloat,
    );
    pub fn vgSetParameteriv(object: VGHandle, paramType: VGint, count: VGint, values: *const VGint);
    pub fn vgGetParameterf(object: VGHandle, paramType: VGint) -> VGfloat;
    pub fn vgGetParameteri(object: VGHandle, paramType: VGint) -> VGint;
    pub fn vgGetParameterVectorSize(object: VGHandle, paramType: VGint) -> VGint;
    pub fn vgGetParameterfv(
        object: VGHandle,
        paramType: VGint,
        count: VGint,
        values: *const VGfloat,
    );
    pub fn vgGetParameteriv(object: VGHandle, paramType: VGint, count: VGint, values: *const VGint);
    /* Matrix Manipulation */
    pub fn vgLoadIdentity();
    pub fn vgLoadMatrix(m: *const VGfloat);
    pub fn vgGetMatrix(m: *const VGfloat);
    pub fn vgMultMatrix(m: *const VGfloat);
    pub fn vgTranslate(tx: VGfloat, ty: VGfloat);
    pub fn vgScale(sx: VGfloat, sy: VGfloat);
    pub fn vgShear(shx: VGfloat, shy: VGfloat);
    pub fn vgRotate(angle: VGfloat);
    /* Masking and Clearing */
    pub fn vgMask(
        mask: VGHandle,
        operation: VGMaskOperation,
        x: VGint,
        y: VGint,
        width: VGint,
        height: VGint,
    );
    pub fn vgRenderToMask(path: VGPath, paintModes: VGbitfield, operation: VGMaskOperation);
    pub fn vgCreateMaskLayer(path: VGPath, height: VGint) -> VGMaskLayer;
    pub fn vgDestroyMaskLayer(maskLayer: VGMaskLayer);
    pub fn vgFillMaskLayer(
        maskLayer: VGMaskLayer,
        x: VGint,
        y: VGint,
        width: VGint,
        height: VGint,
        value: VGfloat,
    );
    pub fn vgCopyMask(
        maskLayer: VGMaskLayer,
        dx: VGint,
        dy: VGint,
        sx: VGint,
        sy: VGint,
        width: VGint,
        height: VGint,
    );
    pub fn vgClear(x: VGuint, y: VGuint, width: VGuint, height: VGuint);
    /* Paths */
    pub fn vgCreatePath(
        pathFormat: VGint,
        datatype: VGPathDatatype,
        scale: VGfloat,
        bias: VGfloat,
        segmentCapacityHint: VGint,
        coordCapacityHint: VGint,
        capabilities: VGbitfield,
    ) -> VGPath;
    pub fn vgClearPath(path: VGPath, capabilities: VGbitfield);
    pub fn vgDestroyPath(path: VGPath);
    pub fn vgRemovePathCapabilities(path: VGPath, capabilities: VGbitfield);
    pub fn vgGetPathCapabilities(path: VGPath) -> VGbitfield;
    pub fn vgAppendPath(dstPath: VGPath, srcPath: VGPath);
    pub fn vgAppendPathData(
        dstPath: VGPath,
        numSegments: VGint,
        pathSegmemts: *const VGubyte,
        pathData: *const VGbyte,
    );
    pub fn vgModifyPathCoords(
        dstPath: VGPath,
        startIndex: VGint,
        numSegments: VGint,
        pathData: *const VGbyte,
    );
    pub fn vgTransformPath(dstPath: VGPath, srcPath: VGPath);
    pub fn vgInterpolatePath(
        dstPath: VGPath,
        startPath: VGPath,
        endPath: VGPath,
        amount: VGfloat,
    ) -> VGboolean;
    pub fn vgPathLength(path: VGPath, startSegment: VGint, numSegments: VGint) -> VGfloat;
    pub fn vgPointAlongPath(
        path: VGPath,
        startSegment: VGint,
        numSegments: VGint,
        distance: VGint,
        x: *const VGfloat,
        y: *const VGfloat,
        tangentX: *const VGfloat,
        tangentY: *const VGfloat,
    );
    pub fn vgPathBounds(
        path: VGPath,
        minX: *const VGfloat,
        minY: *const VGfloat,
        width: *const VGfloat,
        height: *const VGfloat,
    );
    pub fn vgPathTransformedBounds(
        path: VGPath,
        minX: *const VGfloat,
        minY: *const VGfloat,
        width: *const VGfloat,
        height: *const VGfloat,
    );
    pub fn vgDrawPath(path: VGPath, paintModes: VGbitfield);
    /* Paint */
    pub fn vgCreatePaint() -> VGPaint;
    pub fn vgDestroyPaint(paint: VGPaint);
    pub fn vgSetPaint(paint: VGPaint, paintModes: VGbitfield);
    pub fn vgGetPaint(paintMode: VGPaintMode) -> VGPaint;
    pub fn vgSetColor(paint: VGPaint, rgba: VGuint);
    pub fn vgGetColor(paint: VGPaint) -> VGuint;
    pub fn vgPaintPattern(paint: VGPaint, pattern: VGImage);
    /* Images */
    pub fn vgCreateImage(
        format: VGImageFormat,
        width: VGint,
        height: VGint,
        allowedQuality: VGbitfield,
    ) -> VGImage;
    pub fn vgDestroyImage(image: VGImage);
    pub fn vgClearImage(image: VGImage, x: VGint, y: VGint, width: VGint, height: VGint);
    pub fn vgImageSubData(
        image: VGImage,
        data: *const VGbyte,
        dataStride: VGint,
        dataFormat: VGImageFormat,
        x: VGint,
        y: VGint,
        width: VGint,
        height: VGint,
    );
    pub fn vgGetImageSubData(
        image: VGImage,
        data: *const VGbyte,
        dataStride: VGint,
        dataFormat: VGImageFormat,
        x: VGint,
        y: VGint,
        width: VGint,
        height: VGint,
    );
    pub fn vgChildImage(
        parent: VGImage,
        x: VGint,
        y: VGint,
        width: VGint,
        height: VGint,
    ) -> VGImage;
    pub fn vgGetParent(image: VGImage) -> VGImage;
    pub fn vgCopyImage(
        dst: VGImage,
        dx: VGint,
        dy: VGint,
        src: VGImage,
        sx: VGint,
        sy: VGint,
        width: VGint,
        height: VGint,
        dither: VGboolean,
    );
    pub fn vgDrawImage(image: VGImage);
    pub fn vgSetPixels(
        dx: VGint,
        dy: VGint,
        src: VGImage,
        sx: VGint,
        sy: VGint,
        width: VGint,
        height: VGint,
    );
    pub fn vgWritePixels(
        data: *const VGbyte,
        dataStride: VGint,
        dataFormat: VGImageFormat,
        dx: VGint,
        dy: VGint,
        width: VGint,
        height: VGint,
    );
    pub fn vgGetPixels(
        dst: VGImage,
        dx: VGint,
        dy: VGint,
        sx: VGint,
        sy: VGint,
        width: VGint,
        height: VGint,
    );
    pub fn vgReadPixels(
        data: *const VGbyte,
        dataStride: VGint,
        dataFormat: VGImageFormat,
        sx: VGint,
        sy: VGint,
        width: VGint,
        height: VGint,
    );
    pub fn vgCopyPixels(dx: VGint, dy: VGint, sx: VGint, sy: VGint, width: VGint, height: VGint);
    /* Text */
    pub fn vgCreateFont(glyphCapacityHint: VGint) -> VGFont;
    pub fn vgDestroyFont(font: VGFont);
    pub fn vgSetGlyphToPath(
        font: VGFont,
        glyphIndex: VGuint,
        path: VGPath,
        isHinted: VGboolean,
        glyphOrigin: &[VGfloat; 2],
        escapement: &[VGfloat; 2],
    );
    pub fn vgSetGlyphToImage(
        font: VGFont,
        glyphIndex: VGuint,
        image: VGImage,
        glyphOrigin: &[VGfloat; 2],
        escapement: &[VGfloat; 2],
    );
    pub fn vgClearGlyph(font: VGFont, glyphIndex: VGuint);
    pub fn vgDrawGlyph(
        font: VGFont,
        glyphIndex: VGuint,
        paintModes: VGbitfield,
        allowAutoHinting: VGboolean,
    );
    pub fn vgDrawGlyphs(
        font: VGFont,
        glyphCount: VGint,
        glyphIndices: *const VGfloat,
        adjustments_x: *const VGfloat,
        adjustments_y: *const VGfloat,
        paintModes: VGbitfield,
        allowAutoHinting: VGboolean,
    );
    /* Image Filters */
    pub fn vgColorMatrix(dst: VGImage, src: VGImage, matrix: *const VGfloat);
    pub fn vgConvolve(
        dst: VGImage,
        src: VGImage,
        kernelWidth: VGint,
        kernelHeight: VGint,
        shiftX: VGint,
        shiftY: VGint,
        kernel: *const VGshort,
        scale: VGfloat,
        bias: VGfloat,
        tilingMode: VGTilingMode,
    );
    pub fn vgSeparableConvolve(
        dst: VGImage,
        src: VGImage,
        kernelWidth: VGint,
        kernelHeight: VGint,
        shiftX: VGint,
        shiftY: VGint,
        kernelX: *const VGshort,
        kernelY: *const VGshort,
        scale: VGfloat,
        bias: VGfloat,
        tilingMode: VGfloat,
    );
    pub fn vgGaussianBlur(
        dst: VGImage,
        src: VGImage,
        stdDeviationX: VGfloat,
        stdDeviationY: VGfloat,
        tilingMode: VGTilingMode,
    );
    pub fn vgLookup(
        dst: VGImage,
        src: VGImage,
        redLUT: *const VGuint,
        greenLUT: *const VGuint,
        blueLUT: *const VGuint,
        alphaLUT: *const VGuint,
        outputLinear: VGboolean,
        outputPremultiplied: VGboolean,
    );
    pub fn vgLookupSingle(
        dst: VGImage,
        src: VGImage,
        lookupTable: *const VGuint,
        sourceChannel: VGImageChannel,
        outputLinear: VGboolean,
        outputPremultiplied: VGboolean,
    );
    /* Hardware Queries */
    pub fn vgHardwareQuery(key: VGHardwareQueryType, setting: VGint) -> VGHardwareQueryResult;
    /* Renderer and Extension Information */
    pub fn vgGetString(name: VGStringID) -> *const VGubyte;
    // vgu.h
    pub fn vguLine(
        path: VGPath,
        x0: VGfloat,
        y0: VGfloat,
        x1: VGfloat,
        y1: VGfloat,
    ) -> VGUErrorCode;
    pub fn vguPolygon(
        path: VGPath,
        points: *const VGfloat,
        count: VGint,
        closed: VGboolean,
    ) -> VGUErrorCode;
    pub fn vguRect(
        path: VGPath,
        x: VGfloat,
        y: VGfloat,
        width: VGfloat,
        height: VGfloat,
    ) -> VGUErrorCode;
    pub fn vguRoundRect(
        path: VGPath,
        x: VGfloat,
        y: VGfloat,
        width: VGfloat,
        height: VGfloat,
        arcWidth: VGfloat,
        arcHeight: VGfloat,
    ) -> VGUErrorCode;
    pub fn vguEllipse(
        path: VGPath,
        cx: VGfloat,
        cy: VGfloat,
        width: VGfloat,
        height: VGfloat,
    ) -> VGUErrorCode;
    pub fn vguArc(
        path: VGPath,
        x: VGfloat,
        y: VGfloat,
        width: VGfloat,
        height: VGfloat,
        startAngle: VGfloat,
        angleExtent: VGfloat,
        arcType: VGUArcType,
    ) -> VGUErrorCode;
    pub fn vguComputeWarpQuadToSquare(
        sx0: VGfloat,
        sy0: VGfloat,
        sx1: VGfloat,
        sy1: VGfloat,
        sx2: VGfloat,
        sy2: VGfloat,
        sx3: VGfloat,
        sy3: VGfloat,
        matrix: *const VGfloat,
    ) -> VGUErrorCode;
    pub fn vguComputeWarpSquareToQuad(
        dx0: VGfloat,
        dy0: VGfloat,
        dx1: VGfloat,
        dy1: VGfloat,
        dx2: VGfloat,
        dy2: VGfloat,
        dx3: VGfloat,
        dy3: VGfloat,
        matrix: *const VGfloat,
    ) -> VGUErrorCode;
    pub fn vguComputeWarpQuadToQuad(
        dx0: VGfloat,
        dy0: VGfloat,
        dx1: VGfloat,
        dy1: VGfloat,
        dx2: VGfloat,
        dy2: VGfloat,
        dx3: VGfloat,
        dy3: VGfloat,
        sx0: VGfloat,
        sy0: VGfloat,
        sx1: VGfloat,
        sy1: VGfloat,
        sx2: VGfloat,
        sy2: VGfloat,
        sx3: VGfloat,
        sy3: VGfloat,
        matrix: *const VGfloat,
    ) -> VGUErrorCode;
}
