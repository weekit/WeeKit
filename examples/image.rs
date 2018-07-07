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

extern crate reqwest;
extern crate weekit;

use std::io::Read;
use weekit::openvg::{vgSetPixels, VGImage, VGubyte};
use weekit::*;

const IMAGE_PATH: &str = "https://picsum.photos/800/480";

extern "C" {
    fn createImageFromJpegData(data: *const VGubyte, length: usize) -> VGImage;
}

struct Demo<'a> {
    serif_typeface: Option<font::Font<'a>>,
    image: VGImage,
}

impl<'a> Demo<'a> {
    fn new() -> Demo<'a> {
        Demo {
            serif_typeface: None,
            image: 0,
        }
    }
    fn load_fonts(&mut self) -> () {
        self.serif_typeface = Some(font::Font::serif());
        self.load_image();
    }
    fn load_image(&mut self) -> () {
        let mut resp = reqwest::get(IMAGE_PATH).unwrap();
        assert!(resp.status().is_success());
        let mut buffer = Vec::new();
        resp.read_to_end(&mut buffer).unwrap();
        unsafe {
            self.image = createImageFromJpegData(buffer.as_ptr(), buffer.len());
        }
    }
}

impl<'a> Application for Demo<'a> {
    fn draw(&mut self, width: u32, height: u32) -> () {
        match self.serif_typeface {
            Some(_) => (),
            None => self.load_fonts(),
        }

        let canvas = draw::Canvas::new(width, height);
        canvas.background(192, 0, 0);

        unsafe {
            vgSetPixels(0, 0, self.image, 0, 0, 800, 480);
        }

        draw::fill(255, 255, 255, 1.0); // White text
        match self.serif_typeface {
            Some(ref font) => draw::text_mid(
                width as f32 / 2.0,
                height as f32 * 0.1,
                IMAGE_PATH,
                font,
                width / 30,
            ),
            None => {}
        }
    }

    fn handle(&mut self, ev: &event::Event) {
        match ev {
            _default => {}
        }
    }
}

fn main() {
    weekit::main(Demo::new());
}
