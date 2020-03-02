use libc::{c_char, c_int, c_uint, c_ulong, c_void};

use crate::symbol;
use crate::threads;
use std::ptr;

extern {
    pub fn zbar_image_create() -> *mut ZBarImage;
    pub fn zbar_image_destroy(image: *mut ZBarImage);
    pub fn zbar_image_ref(image: *mut ZBarImage, refs: c_int);
    pub fn zbar_image_convert(image: *const ZBarImage, format: c_ulong) -> *mut c_void;
    pub fn zbar_image_convert_resize(
        image: *const ZBarImage,
        format: c_ulong,
        width: c_uint,
        height: c_int,
    ) -> *mut c_void;
    pub fn zbar_image_get_format(image: *const ZBarImage) -> c_ulong;
    pub fn zbar_image_get_sequence(image: *const ZBarImage) -> c_uint;
    pub fn zbar_image_get_width(image: *const ZBarImage) -> c_uint;
    pub fn zbar_image_get_height(image: *const ZBarImage) -> c_uint;
    pub fn zbar_image_get_size(image: *const ZBarImage, width: *mut c_uint, height: *mut c_uint);
    pub fn zbar_image_get_crop(
        image: *const ZBarImage,
        x: *mut c_uint,
        y: *mut c_uint,
        width: *mut c_uint,
        height: *mut c_uint,
    );
    pub fn zbar_image_get_data(image: *const ZBarImage) -> *const c_void;
    pub fn zbar_image_get_data_length(image: *const ZBarImage) -> c_ulong;
    pub fn zbar_image_get_symbols(image: *const ZBarImage) -> *const c_void;
    pub fn zbar_image_set_symbols(image: *mut ZBarImage, symbols: *const c_void);
    pub fn zbar_image_first_symbol(image: *const ZBarImage) -> *const c_void;
    pub fn zbar_image_set_sequence(image: *mut ZBarImage, sequence_num: c_ulong);
    pub fn zbar_image_set_crop(
        image: *mut ZBarImage,
        x: c_ulong,
        y: c_ulong,
        width: c_ulong,
        height: c_ulong,
    );
    pub fn zbar_image_set_data(
        image: *mut ZBarImage,
        data: *const c_void,
        data_byte_length: c_ulong,
        handler: *mut c_void,
    );
    pub fn zbar_image_free_data(image: *mut ZBarImage);
    pub fn zbar_image_set_userdata(image: *mut ZBarImage, userdata: *const c_void);
    pub fn zbar_image_get_userdata(image: *const ZBarImage) -> *const c_void;
    pub fn zbar_image_write(image: *const ZBarImage, filebase: *const c_char) -> c_uint;
    pub fn zbar_image_read(filename: *mut c_char) -> *const c_void;
}

#[repr(C)]
pub struct ZBarImage {
    /* fourcc image format code */
    format: c_uint,
    /* image size */
    width: c_uint,
    height: c_uint,
    /* image sample data */
    data: *mut c_void,
    /* allocated/mapped size of data */
    datalen: u64,
    /* crop rectangle */
    crop_x: c_uint,
    crop_y: c_uint,
    crop_w: c_uint,
    crop_h: c_uint,
    /* user specified data associated w/image */
    userdata: *mut c_void,
    /* cleanup handler */
    cleanup: Option<Cleanup>,
    /* reference count */
    refcnt: c_int,
    /* originator */
    src: *mut c_void,
    /* index used by originator */
    srcidx: c_int,
    /* internal image lists */
    next: *mut c_void,
    /* page/frame sequence number */
    seq: c_int,
    /* decoded result set */
    syms: *mut c_void,
}

type Cleanup = extern fn(*mut ZBarImage);

impl ZBarImage {
    pub fn new() -> ZBarImage {
        let mut image = ZBarImage {
            format: 0,
            width: 0,
            height: 0,
            data: ptr::null_mut(),
            datalen: 0,
            crop_x: 0,
            crop_y: 0,
            crop_w: 0,
            crop_h: 0,
            userdata: ptr::null_mut(),
            cleanup: None,
            refcnt: 0,
            src: ptr::null_mut(),
            srcidx: -1,
            next: ptr::null_mut(),
            seq: 0,
            syms: ptr::null_mut(),
        };

        unsafe {
            threads::_zbar_refcnt_init();
        }

        image.refcnt(1);

        image
    }

    fn refcnt(&mut self, delta: c_int) {
        let refcnt_res;
        unsafe { refcnt_res = threads::_zbar_refcnt(&mut (self.refcnt), delta) }
        if refcnt_res == 0 && delta <= 0 {
            if self.cleanup.is_some() {
                (self.cleanup.unwrap())(self as *mut ZBarImage);
            }
            if !self.src.is_null() {
                self.zbar_image_free();
            }
        }
    }

    fn zbar_image_free(&mut self) {
        if !self.syms.is_null() {
            unsafe {
                symbol::zbar_symbol_set_ref(self.syms, -1);
            }
            self.syms = ptr::null_mut();
        }
        //this shouldn't be need because we create the memory in rust now
        //free(img);
    }

    pub fn set_format(&mut self, format: u32) {
        self.format = format;
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.crop_x = 0;
        self.crop_y = 0;
        self.crop_h = height;
        self.crop_w = width;
        self.width = width;
        self.height = height;
    }

    pub fn set_ref(&mut self, r: isize) {
        unsafe {
            zbar_image_ref(self, r as c_int);
        }
    }

    pub fn destroy(mut self) {
        unsafe {
            zbar_image_destroy(&mut self);
        }
    }
}

impl Default for ZBarImage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    static mut CLEANUP_EXECUTED: bool = false;

    extern fn cleanup_dummy(_img: *mut ZBarImage) {
        unsafe {
            CLEANUP_EXECUTED = true;
        }
    }

    #[test]
    fn test_refcnt_cleanup() {
        let mut img = ZBarImage::new();
        img.cleanup = Some(cleanup_dummy);
        img.refcnt(-1);
        unsafe {
            assert!(CLEANUP_EXECUTED);
        }
    }
}
