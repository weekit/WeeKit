
#[link(name = "app")]
extern {
  pub fn mymain() -> i64;
}

fn main() {
    println!("Hello, world!");
    unsafe {
	mymain();
    }
}
