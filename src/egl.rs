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
