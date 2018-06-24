use std::thread;
use std::env;

fn spin() {
    thread::spawn(move || {
	loop {}
    });
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut threads = 1;
    if args.len() > 1 {
	threads = args[1].parse().unwrap();
    }
    println!("Hello! Running with {} threads.", threads);
    for _ in 1..threads {
        spin();
        println!("launching");
    }
    println!("launching");
    loop {}
}
