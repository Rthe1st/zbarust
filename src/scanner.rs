use libc::{c_char, c_int, c_uint, c_ulong, c_void};

use crate::zbar;
use std::mem;

use zbar::img_scanner::{
    zbar_image_scanner_s, zbar_image_scanner_create, zbar_scan_image
};
use zbar::img_scanner;

use zbar::image::{
    zbar_image_create, zbar_image_set_data, zbar_image_set_format, zbar_image_set_size
};

use zbar::image as zbar_image;
use zbar::symbol as zbar_symbol;

#[derive(Debug)]
pub struct ZBarImageScanResult {
    pub symbol_type: zbar_symbol::zbar_symbol_type_t,
    pub data: Vec<u8>,
}

pub fn scan_y800<D: AsRef<[u8]>>(
    scanner: *mut zbar_image_scanner_s,
    data: D,
    width: u32,
    height: u32,
) -> Result<Vec<ZBarImageScanResult>, &'static str> {
    //        let format: u32 = unsafe { transmute([b'Y', b'8', b'0', b'0']) };
    scan(scanner, data, width, height, 808_466_521)
}

pub fn scan_gray<D: AsRef<[u8]>>(
    scanner: *mut zbar_image_scanner_s,
    data: D,
    width: u32,
    height: u32,
) -> Result<Vec<ZBarImageScanResult>, &'static str> {
    //        let format: u32 = unsafe { transmute([b'G', b'R', b'A', b'Y']) };
    scan(scanner, data, width, height, 1_497_453_127)
}

pub fn scan<D: AsRef<[u8]>>(
    scanner: *mut zbar_image_scanner_s,
    data: D,
    width: libc::c_uint,
    height: libc::c_uint,
    format: libc::c_ulong,
) -> Result<Vec<ZBarImageScanResult>, &'static str> {

    let data = data.as_ref();

    let mut image = unsafe{ zbar_image_create() };

    unsafe{

        zbar_image_set_size(image, width, height);
        zbar_image_set_format(image, format);
        zbar_image_set_data(image, data.as_ptr() as *const c_void,
            data.len() as c_ulong,
            Some(zbar_image::zbar_image_free_data),
        );

    };

    let n = unsafe {
        zbar_scan_image(
            scanner,
            mem::transmute::<*mut zbar_image::zbar_image_s,*mut img_scanner::zbar_image_s>(
                image
            )
        )
    };

    if n < 0 {
        return Err("incorrect image");
    }

    let mut result_array = Vec::with_capacity(n as usize);

    let mut symbol = unsafe { zbar_image::zbar_image_first_symbol(image) };

    let mut symbol = unsafe{
        mem::transmute::<*const zbar_image::zbar_symbol_s,*const zbar_symbol::zbar_symbol_s>(symbol)
    };

    while !symbol.is_null() {
        let symbol_type = unsafe { zbar_symbol::zbar_symbol_get_type(symbol) };
        // let symbol_type = unsafe { mem::transmute::<i32, zbar::ZBarSymbolType>(symbol_type) };
        let data = unsafe {
            let data = zbar_symbol::zbar_symbol_get_data(symbol);
            let data_length = zbar_symbol::zbar_symbol_get_data_length(symbol) as usize;
            Vec::from_raw_parts(data as *mut u8, data_length, data_length)
        };

        let result = ZBarImageScanResult {
            symbol_type,
            data,
        };

        result_array.push(result);

        symbol = unsafe { zbar_symbol::zbar_symbol_next(symbol) };
    }

    Ok(result_array)
}

