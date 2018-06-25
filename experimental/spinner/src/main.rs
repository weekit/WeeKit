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
