
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
#include "VG/openvg.h"
#include "VG/vgu.h"
#include "EGL/egl.h"
#include "bcm_host.h"
#include "DejaVuSans.inc"				   // font data
#include "DejaVuSerif.inc"
#include "DejaVuSansMono.inc"
#include "eglstate.h"					   // data structures for graphics state
#include "fontinfo.h"					   // font data structure

static STATE_T _state, *state = &_state;	// global graphics state
static const int MAXFONTPATH = 500;
static int init_x = 0;		// Initial window position and size
static int init_y = 0;
static unsigned int init_w = 0;
static unsigned int init_h = 0;

// dumpscreen writes the raster
void dumpscreen(int w, int h, FILE * fp) {
	void *ScreenBuffer = malloc(w * h * 4);
	vgReadPixels(ScreenBuffer, (w * 4), VG_sABGR_8888, 0, 0, w, h);
	fwrite(ScreenBuffer, 1, w * h * 4, fp);
	free(ScreenBuffer);
}

// initWindowSize requests a specific window size & position, if not called
// then init() will open a full screen window.
// Done this way to preserve the original init() behaviour.
void egl_initWindowSize(int x, int y, unsigned int w, unsigned int h) {
	init_x = x;
	init_y = y;
	init_w = w;
	init_h = h;
}

// init sets the system to its initial state
void egl_init(int *w, int *h) {
	bcm_host_init();
	memset(state, 0, sizeof(*state));
	state->window_x = init_x;
	state->window_y = init_y;
	state->window_width = init_w;
	state->window_height = init_h;
	oglinit(state);
	*w = state->window_width;
	*h = state->window_height;
}

// finish cleans up
void egl_finish() {
	eglSwapBuffers(state->display, state->surface);
	eglMakeCurrent(state->display, EGL_NO_SURFACE, EGL_NO_SURFACE, EGL_NO_CONTEXT);
	eglDestroySurface(state->display, state->surface);
	eglDestroyContext(state->display, state->context);
	eglTerminate(state->display);
}

// End checks for errors, and renders to the display
void End() {
	assert(vgGetError() == VG_NO_ERROR);
	eglSwapBuffers(state->display, state->surface);
	assert(eglGetError() == EGL_SUCCESS);
}

// SaveEnd dumps the raster before rendering to the display 
void SaveEnd(const char *filename) {
	FILE *fp;
	assert(vgGetError() == VG_NO_ERROR);
	if (strlen(filename) == 0) {
		dumpscreen(state->screen_width, state->screen_height, stdout);
	} else {
		fp = fopen(filename, "wb");
		if (fp != NULL) {
			dumpscreen(state->screen_width, state->screen_height, fp);
			fclose(fp);
		}
	}
	eglSwapBuffers(state->display, state->surface);
	assert(eglGetError() == EGL_SUCCESS);
}

// Backgroud clears the screen to a solid background color

// WindowOpacity sets the  window opacity
void WindowOpacity(unsigned int a) {
	dispmanChangeWindowOpacity(state, a);
}

// WindowPosition moves the window to given position
void WindowPosition(int x, int y) {
	dispmanMoveWindow(state, x, y);
}


// WeeKit handler functions
typedef void (*WKDrawHandler)(int, int);


// Handler pointers
WKDrawHandler wkDrawHandler;

int WKMain(WKDrawHandler handler) {
  int w, h;
  egl_init(&w, &h);
  printf("%d %d\n", w, h);


  wkDrawHandler = handler;
  wkDrawHandler(w, h);
 eglSwapBuffers(state->display, state->surface);
  sleep(5);
  egl_finish();
  return 0;
}

