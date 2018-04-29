extern crate svgkit;

use std::fs::File;

fn main() {
    let filename = "08_01.svg";
    let mut f = File::open(filename).expect("file not found");
    let mut reader = svgkit::Reader::new();
    let svg = reader.read(&mut f);
    println!("{:?}", svg);
}
