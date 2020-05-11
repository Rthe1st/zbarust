/*!
High-level and low-level ZBar binding for the Rust language.

## Compilation

To compile this crate, you need to compile the ZBar library first. You can install ZBar in your operating system, or in somewhere in your file system. As for the latter, you need to set the following environment variables to link the ZBar library:

* `ZBAR_LIB_DIRS`: The directories of library files, like `-L`. Use `:` to separate.
* `ZBAR_LIBS`: The library names that you want to link, like `-l`. Use `:` to separate. Typically, it is **iconv:zbar**.
* `ZBAR_INCLUDE_DIRS`: The directories of header files, like `-i`. Use `:` to separate.

## Examples

```rust
extern crate zbarust;
extern crate image;

use zbarust::zbar::img_scanner::{
    zbar_image_scanner_create
};

use zbarust::scanner;

use image::GenericImageView;

let img = image::open("examples/data/magiclen.org.png").unwrap();

let (width, height) = img.dimensions();

let luma_img = img.to_luma();

let mut luma_img_data: Vec<u8> = luma_img.to_vec();

let scanner = unsafe { zbar_image_scanner_create() };

let mut results = unsafe { scanner::scan_y800(scanner, &mut luma_img_data, width, height).unwrap() };

for result in results {
    println!("{}", String::from_utf8(result.data).unwrap())
}
```

More examples are in the `examples` folder.
*/

#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(const_transmute)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(linkage)]
#![feature(main)]
#![feature(ptr_wrapping_offset_from)]
#![feature(register_tool)]
#![register_tool(c2rust)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;

pub mod scanner;

pub mod python {
    pub mod decoder;
    pub mod r#enum;
    pub mod exception;
    pub mod image;
    pub mod imagescanner;
    pub mod processor;
    pub mod scanner;
    pub mod symbol;
    pub mod symboliter;
    pub mod symbolset;
    pub mod zbarmodule;
} // mod python
pub mod zbar {
    pub mod config;
    pub mod convert;
    pub mod decoder2;
    pub mod decoder {
        pub mod codabar;
        pub mod code128;
        pub mod code39;
        pub mod code93;
        pub mod databar;
        pub mod ean;
        pub mod i25;
        pub mod qr_finder;
    } // mod decoder
    pub mod error;
    pub mod image;
    pub mod img_scanner;
    pub mod processor {
        pub mod lock;
        pub mod null;
        pub mod posix;
    } // mod processor
    pub mod qrcode {
        pub mod bch15_5;
        pub mod binarize;
        pub mod isaac;
        pub mod qrdec;
        pub mod qrdectxt;
        pub mod rs;
        pub mod util;
    } // mod qrcode
    pub mod refcnt;
    pub mod scanner;
    pub mod symbol;
    pub mod video {
        pub mod v4l;
        pub mod v4l2;
    } // mod video
    pub mod window {
        pub mod null;
    } // mod window
} // mod zbar
pub mod zbarcam {
    pub mod zbarcam;
} // mod zbarcam
