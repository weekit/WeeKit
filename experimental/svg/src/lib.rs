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

extern crate quick_xml;

use std::fs::File;
use std::io::prelude::*;
use std::str;

pub use self::quick_xml::Reader as XMLReader;
pub use self::quick_xml::events::Event;

pub type Length = f32;
pub type Coordinate = f32;
pub type Color = String;

#[derive(Debug)]
pub enum Element {
    Unknown,
    SVG {
        width: Option<Length>,
        height: Option<Length>,
        view_box: String,
        xmlns: Option<String>,
        version: String,
        base_profile: String,
        _children: Vec<Element>,
    },
    Title {
        _text: Option<String>,
    },
    Desc {
        _text: Option<String>,
    },
    Rect {
        x: Option<Coordinate>,
        y: Option<Coordinate>,
        width: Option<Length>,
        height: Option<Length>,
        rx: Option<Coordinate>,
        ry: Option<Coordinate>,
        fill: Option<Color>,
        stroke: Option<Color>,
        stroke_width: Option<String>,
    },
    Circle {
        cx: Option<Coordinate>,
        cy: Option<Coordinate>,
        r: Option<Length>,
    },
    Ellipse {},
    Line {
        x1: Coordinate,
        y1: Coordinate,
        x2: Coordinate,
        y2: Coordinate,
    },
    Polyline {},
    Polygon {},
    Path {
        d: Option<String>,
        fill: Option<String>,
        stroke: Option<String>,
        stroke_width: Option<String>,
    },
    Group {
        _children: Vec<Element>,
    },
}

#[derive(Debug)]
struct Partial {
    name: String,
    text: String,
    children: Vec<Element>,
    keys: Vec<String>,
    values: Vec<String>,
}

impl Partial {
    fn new(name: &str) -> Partial {
        Partial {
            name: name.to_string(),
            text: "".to_string(),
            children: Vec::new(),
            keys: Vec::new(),
            values: Vec::new(),
        }
    }
    fn attribute_string(&self, name: &str) -> Option<String> {
        for i in 0..self.keys.len() {
            if self.keys[i] == name {
                return Some(self.values[i].clone());
            }
        }
        None
    }
    fn attribute_coordinate(&self, name: &str) -> Option<Coordinate> {
        for i in 0..self.keys.len() {
            if self.keys[i] == name {
                return Some(self.values[i].parse::<Coordinate>().unwrap());
            }
        }
        None
    }
    fn attribute_length(&self, name: &str) -> Option<Length> {
        for i in 0..self.keys.len() {
            if self.keys[i] == name {
                return Some(self.values[i].parse::<Length>().unwrap());
            }
        }
        None
    }
    fn make_svg(self) -> Element {
        Element::SVG {
            width: None,
            height: None,
            view_box: "".to_string(),
            xmlns: None,
            version: "".to_string(),
            base_profile: "".to_string(),
            _children: self.children,
        }
    }
    fn make_path(self) -> Element {
        Element::Path {
            d: self.attribute_string("d"),
            fill: self.attribute_string("fill"),
            stroke: self.attribute_string("stroke"),
            stroke_width: self.attribute_string("stroke_width"),
        }
    }
    fn make_rect(self) -> Element {
        Element::Rect {
            x: self.attribute_coordinate("x"),
            y: self.attribute_coordinate("y"),
            width: self.attribute_length("width"),
            height: self.attribute_length("height"),
            rx: self.attribute_length("rx"),
            ry: self.attribute_length("ry"),
            fill: self.attribute_string("fill"),
            stroke: self.attribute_string("stroke"),
            stroke_width: self.attribute_string("stroke_width"),
        }
    }
}

pub struct Reader {
    stack: Vec<Partial>,
}

impl Reader {
    pub fn new() -> Reader {
        let mut s: Vec<Partial> = Vec::new();
        s.push(Partial::new(""));
        return Reader { stack: s };
    }

    pub fn read(&mut self, f: &mut File) -> Element {
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
        println!("{}", contents);

        let mut reader = XMLReader::from_str(&contents);
        reader.trim_text(true);

        let mut buf = Vec::new();

        // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    self.open_element(e);
                }
                Ok(Event::Empty(ref e)) => {
                    self.open_element(e);
                    self.close_element();
                }
                Ok(Event::End(_e)) => {
                    self.close_element();
                }
                Ok(Event::Text(e)) => {
                    let text = e.unescape_and_decode(&reader).unwrap();
                    self.add_text(&text);
                }
                Ok(Event::Eof) => {
                    break; // exits the loop when reaching end of file
                }
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // There are several other `Event`s we do not consider here
            }
            // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
            buf.clear();
        }

        println!("{:?}", self.stack.last().unwrap().children.last().unwrap());

        let mut partial = self.stack.pop().unwrap();
        let child = partial.children.pop().unwrap();
        child
    }

    fn open_element(&mut self, e: &quick_xml::events::BytesStart) {
        let name = str::from_utf8(e.name()).unwrap();
        let mut partial = Partial::new(name);
        e.attributes().for_each(|a| {
            let item = a.unwrap();
            let key = str::from_utf8(item.key).unwrap();
            let value = str::from_utf8(&item.value).unwrap();
            partial.keys.push(key.to_string());
            partial.values.push(value.to_string());
        });
        self.stack.push(partial);
    }

    fn close_element(&mut self) {
        let partial = self.stack.pop().unwrap();
        let child = match partial.name.as_ref() {
            "svg" => partial.make_svg(),
            "title" => Element::Title {
                _text: Some(partial.text),
            },
            "desc" => Element::Desc {
                _text: Some(partial.text),
            },
            "rect" => partial.make_rect(),
            "path" => partial.make_path(),
            _ => Element::Unknown,
        };
        self.add_child(child);
    }

    fn add_child(&mut self, child: Element) {
        match self.stack.last_mut() {
            Some(ref mut node) => {
                let mut c = &mut node.children;
                c.push(child);
            }
            None => {}
        }
    }

    fn add_text(&mut self, text: &str) {
        let partial = self.stack.last_mut();
        partial.unwrap().text = text.to_string();
    }
}
