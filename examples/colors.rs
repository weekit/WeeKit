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

extern crate weekit;

use weekit::*;

struct Demo<'a> {
    touch_count: u32,
    sans_typeface: Option<font::Font<'a>>,
}

impl<'a> Demo<'a> {
    fn new() -> Demo<'a> {
        Demo {
            touch_count: 0,
            sans_typeface: None,
        }
    }
    fn load_fonts(&mut self) -> () {
        self.sans_typeface = Some(font::Font::sans());
    }
}

impl<'a> Application for Demo<'a> {
    fn draw(&mut self, width: u32, height: u32) -> () {
        match self.sans_typeface {
            Some(_) => (),
            None => self.load_fonts(),
        }

        let canvas = draw::Canvas::new(width, height);
        canvas.background((self.touch_count * 4) % 256, 0, 0);

        draw::fill(44, 77, (128 + 4 * self.touch_count) % 256, 1.0); // Big blue marble
        draw::circle(width as f32 / 2.0, 0 as f32, width as f32); // The "world"

        draw::fill(255, 255, 255, 1.0); // White text

        match self.sans_typeface {
            Some(ref font) => draw::text_mid(
                width as f32 / 2.0,
                height as f32 * 0.5,
                "hello, world",
                font,
                width / 15,
            ),
            None => {}
        }

        draw::stroke_width(1.0);
        draw::fill(255, 0, 0, 1.0);
        draw::stroke(255, 255, 255, 1.0);
    }

    fn handle(&mut self, ev: &event::Event) {
        match ev {
            &event::Event::Touch(_t, _) => self.touch_count += 1,
            _default => {}
        }
    }
}

fn main() {
    weekit::main(Demo::new());
}
