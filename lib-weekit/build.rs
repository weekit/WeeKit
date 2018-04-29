use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    if target.contains("apple") {
        println!("cargo:rustc-link-search=native=../Demo.app/Contents/dylibs");
        println!("cargo:rustc-link-lib=platform");
        println!("cargo:rustc-link-lib=AmanithVG.4");
    } else {
        println!("cargo:rustc-link-search=native=../platform");
        println!("cargo:rustc-link-search=native=/opt/vc/lib");
        println!("cargo:rustc-link-lib=platform");
        println!("cargo:rustc-link-lib=brcmEGL");
        println!("cargo:rustc-link-lib=brcmGLESv2");
        println!("cargo:rustc-link-lib=brcmOpenVG");
    }
}
