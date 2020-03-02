use libc::{c_char, c_int, c_uint, c_ulong, c_void};

use crate::symbol;
use crate::zbar;
use crate::zbar_image;

extern {
    pub fn zbar_image_scanner_create() -> *mut ZBarImageScanner;
    pub fn zbar_image_scanner_destroy(scanner: *mut ZBarImageScanner);
    pub fn zbar_image_scanner_set_data_handler(
        scanner: *mut c_void,
        handler: *const c_void,
        userdata: *const c_void,
    );
    pub fn zbar_image_scanner_set_config(
        scanner: *mut ZBarImageScanner,
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
    pub fn zbar_scan_image(
        scanner: *mut ZBarImageScanner,
        image: *mut zbar_image::ZBarImage,
    ) -> c_int;
}

#[derive(Debug)]
pub struct ZBarImageScanResult {
    pub symbol_type: zbar::ZBarSymbolType,
    pub data: Vec<u8>,
}

#[repr(C)]
#[derive(Debug)]
struct RecycleBucket {
    nsyms: c_int,
    head: *mut zbar::ZBarSymbolType,
}

const RECYCLE_BUCKETS: usize = 5;
// this will be 2
// I'm confused if this is right
// because ZBAR_CFG_X_DENSITY and ZBAR_CFG_Y_DENSITY start out as 0x100 and 0x101
// but are then changed to 1 and 1 by a CFG call when scanner is made?
const NUM_SCN_CFGS: usize =
    zbar::ZBarConfig::ZBarCfgYDensity as usize - zbar::ZBarConfig::ZBarCfgXDensity as usize + 1;

const NUM_SYMS: usize = 20;

#[repr(C)]
#[derive(Debug)]
pub struct ZBarImageScanner {
    /* associated linear intensity scanner */
    scanner: *mut c_void,
    /* associated symbol decoder */
    decoder: *mut c_void,
    /* QR Code 2D reader */
    // todo: with a macro, hide behind ENABLE_QRCODE flag
    qr_reader: *mut c_void,
    /* application data */
    userdata: *mut c_void,
    handler: *mut c_void,
    /* scan start time */
    time: c_ulong,
    /* currently scanning image *root* */
    img: *mut c_void,
    /* current scan direction */
    dx: c_int,
    dy: c_int,
    du: c_int,
    dumin: c_int,
    v: c_int,
    /* previous decode results */
    syms: *mut c_void,
    /* recycled symbols in 4^n size buckets */
    recycle: [RecycleBucket; RECYCLE_BUCKETS],
    /* current result cache state */
    enable_cache: c_int,
    /* inter-image result cache entries */
    cache: *mut c_void,
    /* configuration settings
    config flags */
    config: c_uint,
    ean_config: c_uint,
    /* int valued configurations */
    configs: [c_int; NUM_SCN_CFGS],
    /* per-symbology configurations */
    sym_configs: [[c_int; NUM_SYMS]; 1],

    // todo: with macro hide behind NO_STATS flag
    stat_syms_new: c_int,
    stat_iscn_syms_inuse: c_int,
    stat_iscn_syms_recycle: c_int,
    stat_img_syms_inuse: c_int,
    stat_img_syms_recycle: c_int,
    stat_sym_new: c_int,
    stat_sym_recycle: [c_int; RECYCLE_BUCKETS],
}

impl ZBarImageScanner {
    pub fn new() -> *mut ZBarImageScanner {
        unsafe { zbar_image_scanner_create() }
    }

    pub fn set_config(
        &mut self,
        symbology: zbar::ZBarSymbolType,
        config: zbar::ZBarConfig,
        value: isize,
    ) -> Result<(), &'static str> {
        let result = unsafe {
            zbar_image_scanner_set_config(
                self,
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

    pub fn destroy(&mut self) {
        //todo: this should be in a wrapper that also
        // nulls the thing pointing to self
        unsafe {
            zbar_image_scanner_destroy(self);
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

        let mut image = zbar_image::ZBarImage::new();

        image.set_size(width, height);
        image.set_format(format);

        unsafe {
            zbar_image::zbar_image_set_data(
                &mut image,
                data.as_ptr() as *const c_void,
                data.len() as c_ulong,
                zbar_image::zbar_image_free_data as *mut c_void,
            );
        }

        let n = unsafe { zbar_scan_image(self, &mut image) };

        if n < 0 {
            return Err("incorrect image");
        }

        let mut result_array = Vec::with_capacity(n as usize);

        let mut symbol = unsafe { zbar_image::zbar_image_first_symbol(&image) };

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

impl Drop for ZBarImageScanner {
    fn drop(&mut self) {
        unsafe {
            zbar_image_scanner_destroy(self);
        }
    }
}
