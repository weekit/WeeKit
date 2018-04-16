fn main() {
    println!("cargo:rustc-link-search=native=../Demo.app/Contents/dylibs");
    println!("cargo:rustc-link-lib=AmanithVG.4");
}
