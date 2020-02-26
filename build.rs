extern crate autotools;

use autotools::Config;

fn main() {
    let dst = Config::new("zbar")
        .reconf("-ivf")
        .without("gtk", None)
        .without("python", None)
        .without("qt", None)
        .without("java", None)
        .without("imagemagick", None)
        .with("jpeg", Some("no"))
        .with("x", Some("no"))
        .enable("video", Some("no"))
        .build();

    println!("cargo:rustc-link-search=native={}", format!("{}/lib", dst.display()));
    println!("cargo:rustc-link-lib=static=zbar");
}
