fn main() {
    println!("cargo:rustc-link-search=native=..");
    println!("cargo:rustc-link-search=native=/opt/vc/lib");
    println!("cargo:rustc-link-lib=brcmEGL");
    println!("cargo:rustc-link-lib=brcmGLESv2");
    println!("cargo:rustc-link-lib=brcmOpenVG");
}
