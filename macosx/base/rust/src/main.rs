
#[link(name = "app")]
extern {
  pub fn mymain(f: extern fn(i32,i32) -> ()) -> i64;
}

extern fn do_something(x:i32, y:i32) {
  println!("do_something: {} {}", x, y);
}

fn main() {
    println!("Hello, world!");
    unsafe {
	mymain(do_something);
    }
}
