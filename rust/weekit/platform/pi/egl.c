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

static EGLDisplay display;
static EGLContext context;
static EGLSurface surface;

static EGL_DISPMANX_WINDOW_T window;

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

	// get an EGL display connection
	display = eglGetDisplay(EGL_DEFAULT_DISPLAY);
	assert(display != EGL_NO_DISPLAY);

	// initialize the EGL display connection
	EGLBoolean result = eglInitialize(display, NULL, NULL);
	assert(EGL_FALSE != result);

	// bind OpenVG API
	eglBindAPI(EGL_OPENVG_API);

	// get an appropriate EGL frame buffer configuration
	EGLConfig config;
	EGLint num_config;
	result = eglChooseConfig(display, attribute_list, &config, 1, &num_config);
	assert(EGL_FALSE != result);

	// create an EGL rendering context
	context = eglCreateContext(display, config, EGL_NO_CONTEXT, NULL);
	assert(context != EGL_NO_CONTEXT);

	DISPMANX_DISPLAY_HANDLE_T dispman_display = vc_dispmanx_display_open(0 /* LCD */ );

	// get the display size
	DISPMANX_MODEINFO_T mode_info;
	int32_t success = vc_dispmanx_display_get_info(dispman_display, &mode_info);
	*w = mode_info.width;
	*h = mode_info.height;

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

	window.element = dispman_element;
	window.width = *w;
	window.height = *h;
	vc_dispmanx_update_submit_sync(dispman_update);

	// create an EGL window surface
	surface = eglCreateWindowSurface(display, config, &window, NULL);
	assert(surface != EGL_NO_SURFACE);

	// preserve the buffers on swap
	result = eglSurfaceAttrib(display, surface, EGL_SWAP_BEHAVIOR, EGL_BUFFER_PRESERVED);
	assert(EGL_FALSE != result);

	// connect the context to the surface
	result = eglMakeCurrent(display, surface, surface, context);
	assert(EGL_FALSE != result);
}

void egl_finish() {
	eglSwapBuffers(display, surface);
	eglMakeCurrent(display, EGL_NO_SURFACE, EGL_NO_SURFACE, EGL_NO_CONTEXT);
	eglDestroySurface(display, surface);
	eglDestroyContext(display, context);
	eglTerminate(display);
}

void egl_swap_buffers() {
	eglSwapBuffers(display, surface);
}
