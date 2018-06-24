//
// derived from
//
// libshapes: high-level OpenVG API
// Anthony Starks (ajstarks@gmail.com)
//
// Additional outline / windowing functions
// Paeryn (github.com/paeryn)
//
#include <assert.h>
#include "VG/openvg.h"
#include "VG/vgu.h"
#include "EGL/egl.h"
#include "bcm_host.h"

typedef struct {
	// screen dimensions
	uint32_t width;
	uint32_t height;

	// dispman window 
	EGL_DISPMANX_WINDOW_T window;

	// EGL data
	EGLDisplay display;
	EGLSurface surface;
	EGLContext context;
} STATE_T;

static STATE_T _state, *state = &_state;	// global graphics state

static VC_DISPMANX_ALPHA_T alpha = {
	DISPMANX_FLAGS_ALPHA_FIXED_ALL_PIXELS,
	255, 0
};

static const EGLint attribute_list[] = {
	EGL_RED_SIZE, 8,
	EGL_GREEN_SIZE, 8,
	EGL_BLUE_SIZE, 8,
	EGL_ALPHA_SIZE, 8,
	EGL_SURFACE_TYPE, EGL_WINDOW_BIT,
	EGL_NONE
};
 
void egl_init(uint32_t *w, uint32_t *h) {
	memset(state, 0, sizeof(STATE_T));

	// get the screen size
	int32_t success = graphics_get_display_size(
		0 /* LCD */ , 
		&state->width,
		&state->height);
	assert(success >= 0);

	// get an EGL display connection
	state->display = eglGetDisplay(EGL_DEFAULT_DISPLAY);
	assert(state->display != EGL_NO_DISPLAY);

	// initialize the EGL display connection
	EGLBoolean result = eglInitialize(state->display, NULL, NULL);
	assert(EGL_FALSE != result);

	// bind OpenVG API
	eglBindAPI(EGL_OPENVG_API);

	// get an appropriate EGL frame buffer configuration
	EGLConfig config;
	EGLint num_config;
	result = eglChooseConfig(state->display, attribute_list, &config, 1, &num_config);
	assert(EGL_FALSE != result);

	// create an EGL rendering context
	state->context = eglCreateContext(state->display, config, EGL_NO_CONTEXT, NULL);
	assert(state->context != EGL_NO_CONTEXT);

	// create an EGL window surface
	DISPMANX_DISPLAY_HANDLE_T dispman_display = vc_dispmanx_display_open(0 /* LCD */ );
	DISPMANX_UPDATE_HANDLE_T dispman_update = vc_dispmanx_update_start(0);
	VC_RECT_T dst_rect;
	VC_RECT_T src_rect;
	DISPMANX_ELEMENT_HANDLE_T dispman_element = vc_dispmanx_element_add(
		dispman_update, 
		dispman_display, 
		0 /*layer */ , 
		&dst_rect, 
		0 /*src */ ,
		&src_rect, 
		DISPMANX_PROTECTION_NONE,
		&alpha, 
                0 /*clamp */ ,
		0 /*transform */ );

	state->window.element = dispman_element;
	state->window.width = state->width;
	state->window.height = state->height;
	vc_dispmanx_update_submit_sync(dispman_update);

	state->surface = eglCreateWindowSurface(state->display, config, &(state->window), NULL);
	assert(state->surface != EGL_NO_SURFACE);

	// preserve the buffers on swap
	result = eglSurfaceAttrib(state->display, state->surface, EGL_SWAP_BEHAVIOR, EGL_BUFFER_PRESERVED);
	assert(EGL_FALSE != result);

	// connect the context to the surface
	result = eglMakeCurrent(state->display, state->surface, state->surface, state->context);
	assert(EGL_FALSE != result);

	// return the screen size
	*w = state->width;
	*h = state->height;
}

void egl_finish() {
	eglSwapBuffers(state->display, state->surface);
	eglMakeCurrent(state->display, EGL_NO_SURFACE, EGL_NO_SURFACE, EGL_NO_CONTEXT);
	eglDestroySurface(state->display, state->surface);
	eglDestroyContext(state->display, state->context);
	eglTerminate(state->display);
}

void egl_swap_buffers() {
	eglSwapBuffers(state->display, state->surface);
}
