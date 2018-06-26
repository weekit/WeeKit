//
// libshapes: high-level OpenVG API
// Anthony Starks (ajstarks@gmail.com)
//
// Additional outline / windowing functions
// Paeryn (github.com/paeryn)
//
#include <stdio.h>
#include <stdlib.h>
#include <termios.h>
#include <assert.h>
#include <jpeglib.h>
#include <string.h>
#include "VG/openvg.h"
#include "VG/vgu.h"

//
// Terminal settings
//

// terminal settings structures
struct termios new_term_attr;
struct termios orig_term_attr;

// saveterm saves the current terminal settings
void saveterm() {
  tcgetattr(fileno(stdin), &orig_term_attr);
}

// rawterm sets the terminal to raw mode
void rawterm() {
  memcpy(&new_term_attr, &orig_term_attr, sizeof(struct termios));
  new_term_attr.c_lflag &= ~(ICANON | ECHO | ECHOE | ECHOK | ECHONL | ECHOPRT | ECHOKE | ICRNL);
  new_term_attr.c_cc[VTIME] = 0;
  new_term_attr.c_cc[VMIN] = 0;
  tcsetattr(fileno(stdin), TCSANOW, &new_term_attr);
}

// restore resets the terminal to the previously saved setting
void restoreterm() {
  tcsetattr(fileno(stdin), TCSANOW, &orig_term_attr);
}

//
// Image services
//

VGImage createImageFromJpegDecompressStruct(struct jpeg_decompress_struct *jdc);

// createImageFromJpegData decompresses a JPEG image to the standard image format
// source: https://github.com/ileben/ShivaVG/blob/master/examples/test_image.c
VGImage createImageFromJpegData(VGubyte *image_data, unsigned int length) {
  // Setup default error handling
  struct jpeg_decompress_struct jdc;
  struct jpeg_error_mgr jerr;
  jdc.err = jpeg_std_error(&jerr);
  jpeg_create_decompress(&jdc);
  
  // Set input file
  jpeg_mem_src(&jdc, image_data, length);
  
  return createImageFromJpegDecompressStruct(&jdc);
}

// createImageFromJpeg decompresses a JPEG image to the standard image format
// source: https://github.com/ileben/ShivaVG/blob/master/examples/test_image.c
VGImage createImageFromJpegFile(const char *filename) {
  // Try to open image file
  FILE *infile = fopen(filename, "rb");
  if (infile == NULL) {
    printf("Failed opening '%s' for reading!\n", filename);
    return VG_INVALID_HANDLE;
  }
  
  // Setup default error handling
  struct jpeg_decompress_struct jdc;
  struct jpeg_error_mgr jerr;  
  jdc.err = jpeg_std_error(&jerr);
  jpeg_create_decompress(&jdc);
  
  // Set input file
  jpeg_stdio_src(&jdc, infile);
  
  return createImageFromJpegDecompressStruct(&jdc);
}
   
VGImage createImageFromJpegDecompressStruct(struct jpeg_decompress_struct *jdc) {
  // Check for endianness
  unsigned int lilEndianTest = 1;
  VGImageFormat rgbaFormat;
  if (((unsigned char *)&lilEndianTest)[0] == 1)
    rgbaFormat = VG_sABGR_8888;
  else
    rgbaFormat = VG_sRGBA_8888;
	
  // Read header and start
  jpeg_read_header(jdc, TRUE);
  jpeg_start_decompress(jdc);
  unsigned int width = jdc->output_width;
  unsigned int height = jdc->output_height;
  
  // Allocate buffer using jpeg allocator
  unsigned int bbpp = jdc->output_components;
  unsigned int bstride = width * bbpp;
  JSAMPARRAY buffer = (*jdc->mem->alloc_sarray)
  ((j_common_ptr) jdc, JPOOL_IMAGE, bstride, 1);
  
  // Allocate image data buffer
  unsigned int dbpp = 4;
  unsigned int dstride = width * dbpp;
  VGubyte *data = (VGubyte *) malloc(dstride * height);
  
  // Iterate until all scanlines processed
  while (jdc->output_scanline < height) {
    
    // Read scanline into buffer
    jpeg_read_scanlines(jdc, buffer, 1);
    VGubyte *drow = data + (height - jdc->output_scanline) * dstride;
    VGubyte *brow = buffer[0];
    // Expand to RGBA
    for (unsigned int x = 0; x < width; ++x, drow += dbpp, brow += bbpp) {
      switch (bbpp) {
        case 4:
          drow[0] = brow[0];
          drow[1] = brow[1];
          drow[2] = brow[2];
          drow[3] = brow[3];
          break;
        case 3:
          drow[0] = brow[0];
          drow[1] = brow[1];
          drow[2] = brow[2];
          drow[3] = 255;
          break;
      }
    }
  }
  
  // Create VG image
  VGImage img = vgCreateImage(rgbaFormat, width, height, VG_IMAGE_QUALITY_BETTER);
  vgImageSubData(img, data, dstride, rgbaFormat, 0, 0, width, height);
  
  // Cleanup
  jpeg_destroy_decompress(jdc);
  
  return img;
}

// makeimage makes an image from a raw raster of red, green, blue, alpha values
void makeimage(VGfloat x, VGfloat y, int w, int h, VGubyte * data) {
  unsigned int dstride = w * 4;
  VGImageFormat rgbaFormat = VG_sABGR_8888;
  VGImage img = vgCreateImage(rgbaFormat, w, h, VG_IMAGE_QUALITY_BETTER);
  vgImageSubData(img, (void *)data, dstride, rgbaFormat, 0, 0, w, h);
  vgSetPixels(x, y, img, 0, 0, w, h);
  vgDestroyImage(img);
}