

#![allow(dead_code)]
#![allow(non_camel_case_types)]

#[link(name = "app")]
extern {
  pub fn WKMain(f: extern fn(i32,i32) -> ()) -> i64;
}

type VGfloat = f32;
type VGbyte = i8;
type VGubyte = u8;
type VGshort = i16;
type VGint = i32;
type VGuint = u32;
type VGbitfield = u32;

const VG_MAX_ENUM : isize = 0x7FFFFFFF;

const VG_PATH_FORMAT_STANDARD : VGint = 0;

#[repr(C)] 
pub enum VGPathDatatype {
    VG_PATH_DATATYPE_S_8                        =  0,
    VG_PATH_DATATYPE_S_16                       =  1,
    VG_PATH_DATATYPE_S_32                       =  2,
    VG_PATH_DATATYPE_F                          =  3,

    VG_PATH_DATATYPE_FORCE_SIZE                 = VG_MAX_ENUM
}


type VGHandle = u32;
type VGPath = VGHandle;
type VGImage = VGHandle;
type VGMaskLayer = VGHandle;
type VGFont = VGHandle;
type VGPaint = VGHandle;


#[repr(C)] 
pub enum VGPaintMode {
    VG_STROKE_PATH                              = (1 << 0),
    VG_FILL_PATH                                = (1 << 1),

    VG_PAINT_MODE_FORCE_SIZE                    = VG_MAX_ENUM
} 

#[repr(C)] 
pub enum VGPathCapabilities {
    VG_PATH_CAPABILITY_APPEND_FROM              = (1 <<  0),
    VG_PATH_CAPABILITY_APPEND_TO                = (1 <<  1),
    VG_PATH_CAPABILITY_MODIFY                   = (1 <<  2),
    VG_PATH_CAPABILITY_TRANSFORM_FROM           = (1 <<  3),
    VG_PATH_CAPABILITY_TRANSFORM_TO             = (1 <<  4),
    VG_PATH_CAPABILITY_INTERPOLATE_FROM         = (1 <<  5),
    VG_PATH_CAPABILITY_INTERPOLATE_TO           = (1 <<  6),
    VG_PATH_CAPABILITY_PATH_LENGTH              = (1 <<  7),
    VG_PATH_CAPABILITY_POINT_ALONG_PATH         = (1 <<  8),
    VG_PATH_CAPABILITY_TANGENT_ALONG_PATH       = (1 <<  9),
    VG_PATH_CAPABILITY_PATH_BOUNDS              = (1 << 10),
    VG_PATH_CAPABILITY_PATH_TRANSFORMED_BOUNDS  = (1 << 11),
    VG_PATH_CAPABILITY_ALL                      = ((1 << 12) - 1),

    VG_PATH_CAPABILITIES_FORCE_SIZE             = VG_MAX_ENUM
}

#[link(name = "AmanithVG.4")]
extern {
  pub fn vgCreatePath(pathFormat : VGint, 
                      datatype : VGPathDatatype,
                      scale: VGfloat,
                      bias: VGfloat,
                      segmentCapacityHint: VGint,
                      coordCapacityHint: VGint,
                      capabilities: VGbitfield) -> VGPath;
  pub fn vguRect(path : VGPath, x : VGfloat, y : VGfloat, w : VGfloat, h : VGfloat);
  pub fn vgDrawPath(path : VGPath, paintModes : VGbitfield);
  pub fn vgDestroyPath(path : VGPath);
}


// setfill sets the fill color
fn setfill(color : [VGfloat; 4]) {
  unsafe {
    //VGPaint fillPaint = vgCreatePaint();
    //vgSetParameteri(fillPaint, VG_PAINT_TYPE, VG_PAINT_TYPE_COLOR);
    //vgSetParameterfv(fillPaint, VG_PAINT_COLOR, 4, color);
    //vgSetPaint(fillPaint, VG_FILL_PATH);
    //vgDestroyPaint(fillPaint);
  }
}

extern fn draw(_x:i32, _y:i32) {
  //println!("draw: {} {}", x, y);
  setfill([1.0, 0.0, 0.0, 1.0]);
  unsafe {
    let path = vgCreatePath(VG_PATH_FORMAT_STANDARD, VGPathDatatype::VG_PATH_DATATYPE_F, 1.0, 0.0, 0, 0, VGPathCapabilities::VG_PATH_CAPABILITY_APPEND_TO as u32);
    vguRect(path, 0.0, 0.0, 10.0, 10.0);
    vgDrawPath(path, VGPaintMode::VG_FILL_PATH as u32 + VGPaintMode::VG_STROKE_PATH as u32);
    vgDestroyPath(path);
  }
}

fn main() {
    println!("Hello, world!");
    unsafe {
	WKMain(draw);
    }
}
