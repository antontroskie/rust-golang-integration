fn main() {
    println!("cargo:rustc-link-search=native=./build");
    println!("cargo:rustc-link-lib=static=glang");
    println!("cargo:rustc-link-lib=static=bridge");
}
