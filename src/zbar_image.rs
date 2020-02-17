use libc::{c_char, c_int, c_uint, c_ulong, c_void};

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
    pub fn zbar_image_set_format(image: *mut ZBarImage, format: c_ulong);
    pub fn zbar_image_set_sequence(image: *mut ZBarImage, sequence_num: c_ulong);
    pub fn zbar_image_set_size(image: *mut ZBarImage, width: c_ulong, height: c_ulong);
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
    format: u32,
    /* image size */
    width: u32,
    height: u32,
    /* image sample data */
    data: *mut c_void,
    /* allocated/mapped size of data */
    datalen: u64,
    /* crop rectangle */
    crop_x: u32,
    crop_y: u32,
    crop_w: u32,
    crop_h: u32,
    /* user specified data associated w/image */
    userdata: *mut c_void,
    /* cleanup handler */
    zbar_image_cleanup_handler_t: *mut c_void,
    /* reference count */
    refcnt: u32,
    /* originator */
    src: *mut c_void,
    /* index used by originator */
    srcidx: u32,
    /* internal image lists */
    next: *mut c_void,
    /* page/frame sequence number */
    seq: u32,
    /* decoded result set */
    syms: *mut c_void,
}

impl ZBarImage {
    pub fn new() -> *mut ZBarImage {
        unsafe { zbar_image_create() }
    }

    pub fn set_format(&mut self, format: u32) {
        unsafe {
            zbar_image_set_format(self, c_ulong::from(format));
        }
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        unsafe {
            zbar_image_set_size(self, c_ulong::from(width), c_ulong::from(height));
        }
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
