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

use std::env;
use std::thread;

fn spin(i: i32) {
    loop {
        let mut sum: f64 = 0.0;
        for i in 1..10000000 {
            sum += (i as f64).sqrt();
        }
        println!("{}: {}", i, sum);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut threads = 1;
    if args.len() > 1 {
        threads = args[1].parse().unwrap();
    }
    println!("Hello! Running with {} threads.", threads);
    for i in 1..threads {
        thread::spawn(move || {
            spin(i);
        });
        println!("launching");
    }
    println!("launching");
    spin(0);
}
