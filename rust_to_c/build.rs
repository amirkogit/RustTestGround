extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/math.c")
        .compile("math");
}
