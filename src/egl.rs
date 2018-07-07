extern "C" {
    fn bcm_host_init();
    fn egl_init(w: &mut u32, h: &mut u32);
    fn egl_finish();
    fn egl_swap_buffers();
}

pub fn init(w: &mut u32, h: &mut u32) {
    unsafe {
        bcm_host_init();
        egl_init(w, h);
    }
}

pub fn swap_buffers() {
    unsafe {
        egl_swap_buffers();
    }
}

pub fn finish() {
    unsafe {
        egl_finish();
    }
}
