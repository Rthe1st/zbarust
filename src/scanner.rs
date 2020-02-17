use libc::{c_char, c_int, c_ulong, c_void};

use crate::symbol;
use crate::zbar;
use crate::zbar_image;

use std::ptr;

extern {
    pub fn zbar_image_scanner_create() -> *mut c_void;
    pub fn zbar_image_scanner_destroy(scanner: *mut c_void);
    pub fn zbar_image_scanner_set_data_handler(
        scanner: *mut c_void,
        handler: *const c_void,
        userdata: *const c_void,
    );
    pub fn zbar_image_scanner_set_config(
        scanner: *mut c_void,
        symbology: c_int,
        config: c_int,
        value: c_int,
    ) -> c_int;
    pub fn zbar_image_scanner_parse_config(
        scanner: *mut c_void,
        config_string: *const c_char,
    ) -> c_int;
    pub fn zbar_image_scanner_enable_cache(scanner: *mut c_void, enable: c_int);
    pub fn zbar_image_scanner_recycle_image(
        scanner: *mut c_void,
        image: *mut zbar_image::ZBarImage,
    );
    pub fn zbar_image_scanner_get_results(scanner: *const c_void) -> *const c_void;
    pub fn zbar_scan_image(scanner: *mut c_void, image: *mut zbar_image::ZBarImage) -> c_int;
}

#[derive(Debug)]
pub struct ZBarImageScanResult {
    pub symbol_type: zbar::ZBarSymbolType,
    pub data: Vec<u8>,
}

pub struct ZBarImageScanner {
    scanner: *mut c_void,
}

impl ZBarImageScanner {
    pub fn new() -> ZBarImageScanner {
        let scanner = unsafe { zbar_image_scanner_create() };

        ZBarImageScanner {
            scanner,
        }
    }

    pub fn set_config(
        &mut self,
        symbology: zbar::ZBarSymbolType,
        config: zbar::ZBarConfig,
        value: isize,
    ) -> Result<(), &'static str> {
        let result = unsafe {
            zbar_image_scanner_set_config(
                self.scanner,
                symbology.ordinal() as c_int,
                config.ordinal() as c_int,
                value as c_int,
            )
        };
        if result == 0 {
            Ok(())
        } else {
            Err("unsuccessfully")
        }
    }

    pub fn destroy(mut self) {
        unsafe {
            zbar_image_scanner_destroy(self.scanner);
            self.scanner = ptr::null_mut();
        }
    }

    pub fn scan_y800<D: AsRef<[u8]>>(
        &mut self,
        data: D,
        width: u32,
        height: u32,
    ) -> Result<Vec<ZBarImageScanResult>, &'static str> {
        //        let format: u32 = unsafe { transmute([b'Y', b'8', b'0', b'0']) };
        self.scan(data, width, height, 808_466_521)
    }

    pub fn scan_gray<D: AsRef<[u8]>>(
        &mut self,
        data: D,
        width: u32,
        height: u32,
    ) -> Result<Vec<ZBarImageScanResult>, &'static str> {
        //        let format: u32 = unsafe { transmute([b'G', b'R', b'A', b'Y']) };
        self.scan(data, width, height, 1_497_453_127)
    }

    pub fn scan<D: AsRef<[u8]>>(
        &mut self,
        data: D,
        width: u32,
        height: u32,
        format: u32,
    ) -> Result<Vec<ZBarImageScanResult>, &'static str> {
        let data = data.as_ref();

        let image: *mut zbar_image::ZBarImage = zbar_image::ZBarImage::new();

        unsafe {
            (*image).set_size(width, height);
            (*image).set_format(format);
            zbar_image::zbar_image_set_data(
                image,
                data.as_ptr() as *const c_void,
                data.len() as c_ulong,
                zbar_image::zbar_image_free_data as *mut c_void,
            );
        }

        let n = unsafe { zbar_scan_image(self.scanner, image) };

        if n < 0 {
            return Err("incorrect image");
        }

        let mut result_array = Vec::with_capacity(n as usize);

        let mut symbol = unsafe { zbar_image::zbar_image_first_symbol(image) };

        while !symbol.is_null() {
            let symbol_type = unsafe { symbol::zbar_symbol_get_type(symbol) };
            let symbol_type =
                unsafe { zbar::ZBarSymbolType::from_ordinal_unsafe(symbol_type as isize) };
            let data = unsafe {
                let data = symbol::zbar_symbol_get_data(symbol);
                let data_length = symbol::zbar_symbol_get_data_length(symbol) as usize;
                Vec::from_raw_parts(data as *mut u8, data_length, data_length)
            };

            let result = ZBarImageScanResult {
                symbol_type,
                data,
            };

            result_array.push(result);

            symbol = unsafe { symbol::zbar_symbol_next(symbol) };
        }

        Ok(result_array)
    }
}

impl Default for ZBarImageScanner {
    #[inline]
    fn default() -> Self {
        ZBarImageScanner::new()
    }
}

impl Drop for ZBarImageScanner {
    fn drop(&mut self) {
        if !self.scanner.is_null() {
            unsafe {
                zbar_image_scanner_destroy(self.scanner);
            }
        }
    }
}
