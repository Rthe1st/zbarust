# Zbarust

![Rust](https://github.com/Rthe1st/zbarust/workflows/Rust/badge.svg?branch=master)

A port of the libzbar C library to Rust.

This whole attempt is because I have a project I want to build in rust that needs

1) To scan barcodes
2) Needs to run on windows

I spent several dozen hours of my life trying to get libzbar and rust to build together on windows and couldn't make it work.

The primary goal is to make this a function Rust crate for scanning barcodes on windows/linux/mac.

This started of as a fork of [magiclen's project](https://github.com/magiclen/zbar-rust) to provide high-level and low-level ZBar binding for the Rust language. But then c2rust was used to convert the external zbar libary to rust and embed it in the project directly.

## Compilation

```bash
export RUSTFLAGS='-Z force-overflow-checks=no'
cargo build
cargo test
```

This is needed because the generated rust code triggers an overflow error.
Not sure if a bug in the C or intentional - but library seems to function either way.

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
