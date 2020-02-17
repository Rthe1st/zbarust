# ZBar Rust

![Rust](https://github.com/Rthe1st/zbarust/workflows/Rust/badge.svg?branch=master)

A fork of [magiclen's project](https://github.com/magiclen/zbar-rust) to provide high-level and low-level ZBar binding for the Rust language.

This fork aims to eventually remove the need for bindings to the ZBar library by gradually rewriting it in rust.

This whole thing is revenge for the fact that I spent several dozen hours of my life trying to get libzbar and rust to build on windows. Root cause of blame may or may not lie in my own incompetency.

## Compilation

To compile this crate, you need to compile the ZBar library first. 

By default the build script will compile and link to version of ZBar in the project submodule,

However you can install ZBar in your operating system, or in somewhere in your file system. As for the latter, you need to set the following environment variables to link the ZBar library:

* `ZBAR_LIB_DIRS`: The directories of library files, like `-L`. Use `:` to separate.
* `ZBAR_LIBS`: The library names that you want to link, like `-l`. Use `:` to separate. Typically, it is **iconv:zbar**.
* `ZBAR_INCLUDE_DIRS`: The directories of header files, like `-i`. Use `:` to separate.

Note: Only ZBar version 0.10-0.20 are supported.

## Examples

```rust
extern crate zbarust;
extern crate image;

use zbarust::ZBarImageScanner;

use image::GenericImageView;

let img = image::open(INPUT_IMAGE_PATH).unwrap();

let (width, height) = img.dimensions();

let luma_img = img.to_luma();

let luma_img_data: Vec<u8> = luma_img.to_vec();

let mut scanner = ZBarImageScanner::new();

let results = scanner.scan_y800(&luma_img_data, width, height).unwrap();

for result in results {
    println!("{}", String::from_utf8(result.data).unwrap())
}
```

More examples are in the `examples` folder.

## License

[LGPL-2.1](LICENSE)