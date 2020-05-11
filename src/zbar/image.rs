use ::libc;
extern {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type zbar_video_s;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    #[no_mangle]
    fn fwrite(__ptr: *const libc::c_void, __size: size_t, __n: size_t, __s: *mut FILE) -> size_t;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn zbar_symbol_set_ref(symbols: *const zbar_symbol_set_t, refs: libc::c_int);
    #[no_mangle]
    static mut _zbar_verbosity: libc::c_int;
    #[no_mangle]
    fn _zbar_refcnt(cnt: *mut refcnt_t, delta: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn _zbar_refcnt_init();
}
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
/* * decoded symbol type. */
pub type zbar_symbol_type_e = libc::c_uint;
/* * add-on flag mask.
 * @deprecated in 0.11, GS1 add-ons are represented using composite
 * symbols of type ::ZBAR_COMPOSITE; add-on components use ::ZBAR_EAN2
 * or ::ZBAR_EAN5
 */
pub const ZBAR_ADDON: zbar_symbol_type_e = 1792;
/* * 5-digit add-on flag.
 * @deprecated in 0.11, a ::ZBAR_EAN5 component is used for
 * 5-digit GS1 add-ons
 */
pub const ZBAR_ADDON5: zbar_symbol_type_e = 1280;
/* * 2-digit add-on flag.
 * @deprecated in 0.11, a ::ZBAR_EAN2 component is used for
 * 2-digit GS1 add-ons
 */
pub const ZBAR_ADDON2: zbar_symbol_type_e = 512;
/* *< Code 128 */
/* * mask for base symbol type.
 * @deprecated in 0.11, remove this from existing code
 */
pub const ZBAR_SYMBOL: zbar_symbol_type_e = 255;
/* *< Code 93. @since 0.11 */
pub const ZBAR_CODE128: zbar_symbol_type_e = 128;
/* *< QR Code. @since 0.10 */
pub const ZBAR_CODE93: zbar_symbol_type_e = 93;
/* *< PDF417. @since 0.6 */
pub const ZBAR_QRCODE: zbar_symbol_type_e = 64;
/* *< Code 39. @since 0.4 */
pub const ZBAR_PDF417: zbar_symbol_type_e = 57;
/* *< Codabar. @since 0.11 */
pub const ZBAR_CODE39: zbar_symbol_type_e = 39;
/* *< GS1 DataBar Expanded. @since 0.11 */
pub const ZBAR_CODABAR: zbar_symbol_type_e = 38;
/* *< GS1 DataBar (RSS). @since 0.11 */
pub const ZBAR_DATABAR_EXP: zbar_symbol_type_e = 35;
/* *< Interleaved 2 of 5. @since 0.4 */
pub const ZBAR_DATABAR: zbar_symbol_type_e = 34;
/* *< EAN/UPC composite */
pub const ZBAR_I25: zbar_symbol_type_e = 25;
/* *< ISBN-13 (from EAN-13). @since 0.4 */
pub const ZBAR_COMPOSITE: zbar_symbol_type_e = 15;
/* *< EAN-13 */
pub const ZBAR_ISBN13: zbar_symbol_type_e = 14;
/* *< UPC-A */
pub const ZBAR_EAN13: zbar_symbol_type_e = 13;
/* *< ISBN-10 (from EAN-13). @since 0.4 */
pub const ZBAR_UPCA: zbar_symbol_type_e = 12;
/* *< UPC-E */
pub const ZBAR_ISBN10: zbar_symbol_type_e = 10;
/* *< EAN-8 */
pub const ZBAR_UPCE: zbar_symbol_type_e = 9;
/* *< GS1 5-digit add-on */
pub const ZBAR_EAN8: zbar_symbol_type_e = 8;
/* *< GS1 2-digit add-on */
pub const ZBAR_EAN5: zbar_symbol_type_e = 5;
/* *< intermediate status */
pub const ZBAR_EAN2: zbar_symbol_type_e = 2;
/* *< no symbol decoded */
pub const ZBAR_PARTIAL: zbar_symbol_type_e = 1;
pub const ZBAR_NONE: zbar_symbol_type_e = 0;
pub type zbar_symbol_type_t = zbar_symbol_type_e;
/* * decoded symbol coarse orientation.
 * @since 0.11
 */
pub type zbar_orientation_e = libc::c_int;
/* *< sideways, read bottom to top */
/* *< upside-down, read right to left */
pub const ZBAR_ORIENT_LEFT: zbar_orientation_e = 3;
/* *< sideways, read top to bottom */
pub const ZBAR_ORIENT_DOWN: zbar_orientation_e = 2;
/* *< upright, read left to right */
pub const ZBAR_ORIENT_RIGHT: zbar_orientation_e = 1;
/* *< unable to determine orientation */
pub const ZBAR_ORIENT_UP: zbar_orientation_e = 0;
pub const ZBAR_ORIENT_UNKNOWN: zbar_orientation_e = -1;
pub type zbar_orientation_t = zbar_orientation_e;
/*------------------------------------------------------------------------
 *  Copyright 2007-2010 (c) Jeff Brown <spadix@users.sourceforge.net>
 *
 *  This file is part of the ZBar Bar Code Reader.
 *
 *  The ZBar Bar Code Reader is free software; you can redistribute it
 *  and/or modify it under the terms of the GNU Lesser Public License as
 *  published by the Free Software Foundation; either version 2.1 of
 *  the License, or (at your option) any later version.
 *
 *  The ZBar Bar Code Reader is distributed in the hope that it will be
 *  useful, but WITHOUT ANY WARRANTY; without even the implied warranty
 *  of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU Lesser Public License for more details.
 *
 *  You should have received a copy of the GNU Lesser Public License
 *  along with the ZBar Bar Code Reader; if not, write to the Free
 *  Software Foundation, Inc., 51 Franklin St, Fifth Floor,
 *  Boston, MA  02110-1301  USA
 *
 *  http://sourceforge.net/projects/zbar
 *------------------------------------------------------------------------ */
/* number of filtered symbols */
/* first of decoded symbol results */
/* last of unfiltered symbol results */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zbar_symbol_s {
    pub type_0: zbar_symbol_type_t,
    pub configs: libc::c_uint,
    pub modifiers: libc::c_uint,
    pub data_alloc: libc::c_uint,
    pub datalen: libc::c_uint,
    pub data: *mut libc::c_char,
    pub pts_alloc: libc::c_uint,
    pub npts: libc::c_uint,
    pub pts: *mut point_t,
    pub orient: zbar_orientation_t,
    pub refcnt: refcnt_t,
    pub next: *mut zbar_symbol_t,
    pub syms: *mut zbar_symbol_set_t,
    pub time: libc::c_ulong,
    pub cache_count: libc::c_int,
    pub quality: libc::c_int,
}
pub type zbar_symbol_set_t = zbar_symbol_set_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zbar_symbol_set_s {
    pub refcnt: refcnt_t,
    pub nsyms: libc::c_int,
    pub head: *mut zbar_symbol_t,
    pub tail: *mut zbar_symbol_t,
}
pub type zbar_symbol_t = zbar_symbol_s;
/*------------------------------------------------------------------------
 *  Copyright 2007-2010 (c) Jeff Brown <spadix@users.sourceforge.net>
 *
 *  This file is part of the ZBar Bar Code Reader.
 *
 *  The ZBar Bar Code Reader is free software; you can redistribute it
 *  and/or modify it under the terms of the GNU Lesser Public License as
 *  published by the Free Software Foundation; either version 2.1 of
 *  the License, or (at your option) any later version.
 *
 *  The ZBar Bar Code Reader is distributed in the hope that it will be
 *  useful, but WITHOUT ANY WARRANTY; without even the implied warranty
 *  of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU Lesser Public License for more details.
 *
 *  You should have received a copy of the GNU Lesser Public License
 *  along with the ZBar Bar Code Reader; if not, write to the Free
 *  Software Foundation, Inc., 51 Franklin St, Fifth Floor,
 *  Boston, MA  02110-1301  USA
 *
 *  http://sourceforge.net/projects/zbar
 *------------------------------------------------------------------------ */
pub type refcnt_t = libc::c_int;
pub type point_t = point_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point_s {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
/*------------------------------------------------------------------------
 *  Copyright 2007-2010 (c) Jeff Brown <spadix@users.sourceforge.net>
 *
 *  This file is part of the ZBar Bar Code Reader.
 *
 *  The ZBar Bar Code Reader is free software; you can redistribute it
 *  and/or modify it under the terms of the GNU Lesser Public License as
 *  published by the Free Software Foundation; either version 2.1 of
 *  the License, or (at your option) any later version.
 *
 *  The ZBar Bar Code Reader is distributed in the hope that it will be
 *  useful, but WITHOUT ANY WARRANTY; without even the implied warranty
 *  of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU Lesser Public License for more details.
 *
 *  You should have received a copy of the GNU Lesser Public License
 *  along with the ZBar Bar Code Reader; if not, write to the Free
 *  Software Foundation, Inc., 51 Franklin St, Fifth Floor,
 *  Boston, MA  02110-1301  USA
 *
 *  http://sourceforge.net/projects/zbar
 *------------------------------------------------------------------------ */
/* unpack size/location of component */
/* coarse image format categorization.
 * to limit conversion variations
 */
/* enum size */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zbar_image_s {
    pub format: uint32_t,
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub data: *const libc::c_void,
    pub datalen: libc::c_ulong,
    pub crop_x: libc::c_uint,
    pub crop_y: libc::c_uint,
    pub crop_w: libc::c_uint,
    pub crop_h: libc::c_uint,
    pub userdata: *mut libc::c_void,
    pub cleanup: Option<zbar_image_cleanup_handler_t>,
    pub refcnt: refcnt_t,
    pub src: *mut zbar_video_t,
    pub srcidx: libc::c_int,
    pub next: *mut zbar_image_t,
    pub seq: libc::c_uint,
    pub syms: *mut zbar_symbol_set_t,
}
pub type zbar_image_t = zbar_image_s;
/* * read back an image in the format written by zbar_image_write()
 * @note TBD
 */
/* @} */
/* ------------------------------------------------------------ */
/* * @name Processor interface
 * @anchor c-processor
 * high-level self-contained image processor.
 * processes video and images for barcodes, optionally displaying
 * images to a library owned output window
 */
/* @{ */
/* * opaque standalone processor object. */
/* * constructor.
 * if threaded is set and threading is available the processor
 * will spawn threads where appropriate to avoid blocking and
 * improve responsiveness
 */
/* * destructor.  cleans up all resources associated with the processor
 */
/* * (re)initialization.
 * opens a video input device and/or prepares to display output
 */
/* * request a preferred size for the video image from the device.
 * the request may be adjusted or completely ignored by the driver.
 * @note must be called before zbar_processor_init()
 * @since 0.6
 */
/* * request a preferred video driver interface version for
 * debug/testing.
 * @note must be called before zbar_processor_init()
 * @since 0.6
 */
/* * request a preferred video I/O mode for debug/testing.  You will
 * get errors if the driver does not support the specified mode.
 * @verbatim
    0 = auto-detect
    1 = force I/O using read()
    2 = force memory mapped I/O using mmap()
    3 = force USERPTR I/O (v4l2 only)
@endverbatim
 * @note must be called before zbar_processor_init()
 * @since 0.7
 */
/* * force specific input and output formats for debug/testing.
 * @note must be called before zbar_processor_init()
 */
/* * setup result handler callback.
 * the specified function will be called by the processor whenever
 * new results are available from the video stream or a static image.
 * pass a NULL value to disable callbacks.
 * @param processor the object on which to set the handler.
 * @param handler the function to call when new results are available.
 * @param userdata is set as with zbar_processor_set_userdata().
 * @returns the previously registered handler
 */
/* * associate user specified data value with the processor.
 * @since 0.6
 */
/* * return user specified data value associated with the processor.
 * @since 0.6
 */
/* * set config for indicated symbology (0 for all) to specified value.
 * @returns 0 for success, non-0 for failure (config does not apply to
 * specified symbology, or value out of range)
 * @see zbar_decoder_set_config()
 * @since 0.4
 */
/* * set video control value
 * @returns 0 for success, non-0 for failure
 * @since 0.20
 * @see zbar_video_set_control()
 */
/* * get video control value
 * @returns 0 for success, non-0 for failure
 * @since 0.20
 * @see zbar_video_get_control()
 */
/* * parse configuration string using zbar_parse_config()
 * and apply to processor using zbar_processor_set_config().
 * @returns 0 for success, non-0 for failure
 * @see zbar_parse_config()
 * @see zbar_processor_set_config()
 * @since 0.4
 */
/* * retrieve the current state of the ouput window.
 * @returns 1 if the output window is currently displayed, 0 if not.
 * @returns -1 if an error occurs
 */
/* * show or hide the display window owned by the library.
 * the size will be adjusted to the input size
 */
/* * control the processor in free running video mode.
 * only works if video input is initialized. if threading is in use,
 * scanning will occur in the background, otherwise this is only
 * useful wrapping calls to zbar_processor_user_wait(). if the
 * library output window is visible, video display will be enabled.
 */
/* * retrieve decode results for last scanned image/frame.
 * @returns the symbol set result container or NULL if no results are
 * available
 * @note the returned symbol set has its reference count incremented;
 * ensure that the count is decremented after use
 * @since 0.10
 */
/* * wait for input to the display window from the user
 * (via mouse or keyboard).
 * @returns >0 when input is received, 0 if timeout ms expired
 * with no input or -1 in case of an error
 */
/* * process from the video stream until a result is available,
 * or the timeout (in milliseconds) expires.
 * specify a timeout of -1 to scan indefinitely
 * (zbar_processor_set_active() may still be used to abort the scan
 * from another thread).
 * if the library window is visible, video display will be enabled.
 * @note that multiple results may still be returned (despite the
 * name).
 * @returns >0 if symbols were successfully decoded,
 * 0 if no symbols were found (ie, the timeout expired)
 * or -1 if an error occurs
 */
/* * process the provided image for barcodes.
 * if the library window is visible, the image will be displayed.
 * @returns >0 if symbols were successfully decoded,
 * 0 if no symbols were found or -1 if an error occurs
 */
/* * display detail for last processor error to stderr.
 * @returns a non-zero value suitable for passing to exit()
 */
/* * retrieve the detail string for the last processor error. */
/* * retrieve the type code for the last processor error. */
/* @} */
/* ------------------------------------------------------------ */
/* * @name Video interface
 * @anchor c-video
 * mid-level video source abstraction.
 * captures images from a video device
 */
/* @{ */
/* * opaque video object. */
pub type zbar_video_t = zbar_video_s;
pub type zbar_image_cleanup_handler_t = unsafe extern fn(_: *mut zbar_image_t) -> ();
pub type zimg_hdr_t = zimg_hdr_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zimg_hdr_s {
    pub magic: uint32_t,
    pub format: uint32_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub size: uint32_t,
}
/* decoded result set */
/* description of an image format */
/* fourcc */
/* coarse categorization */
/* raw bytes */
/* bits per pixel */
/* size/location a la RGB_BITS() */
/* chroma subsampling in each axis */
/* channel ordering flags
 *   bit0: 0=UV, 1=VU
 *   bit1: 0=Y/chroma, 1=chroma/Y
 */
/* quick compare equivalent formats */
#[inline]
unsafe extern fn _zbar_image_refcnt(mut img: *mut zbar_image_t, mut delta: libc::c_int) {
    if _zbar_refcnt(&mut (*img).refcnt, delta) == 0 && delta <= 0 as libc::c_int {
        if (*img).cleanup.is_some() {
            (*img).cleanup.expect("non-null function pointer")(img);
        }
        if (*img).src.is_null() {
            _zbar_image_free(img);
        }
    };
}
#[inline]
unsafe extern fn _zbar_image_copy_size(mut dst: *mut zbar_image_t, mut src: *const zbar_image_t) {
    (*dst).width = (*src).width;
    (*dst).height = (*src).height;
    (*dst).crop_x = (*src).crop_x;
    (*dst).crop_y = (*src).crop_y;
    (*dst).crop_w = (*src).crop_w;
    (*dst).crop_h = (*src).crop_h;
}
/*------------------------------------------------------------------------
 *  Copyright 2007-2010 (c) Jeff Brown <spadix@users.sourceforge.net>
 *
 *  This file is part of the ZBar Bar Code Reader.
 *
 *  The ZBar Bar Code Reader is free software; you can redistribute it
 *  and/or modify it under the terms of the GNU Lesser Public License as
 *  published by the Free Software Foundation; either version 2.1 of
 *  the License, or (at your option) any later version.
 *
 *  The ZBar Bar Code Reader is distributed in the hope that it will be
 *  useful, but WITHOUT ANY WARRANTY; without even the implied warranty
 *  of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU Lesser Public License for more details.
 *
 *  You should have received a copy of the GNU Lesser Public License
 *  along with the ZBar Bar Code Reader; if not, write to the Free
 *  Software Foundation, Inc., 51 Franklin St, Fifth Floor,
 *  Boston, MA  02110-1301  USA
 *
 *  http://sourceforge.net/projects/zbar
 *------------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern fn zbar_image_create() -> *mut zbar_image_t {
    let mut img: *mut zbar_image_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<zbar_image_t>() as libc::c_ulong,
    ) as *mut zbar_image_t;
    _zbar_refcnt_init();
    _zbar_image_refcnt(img, 1 as libc::c_int);
    (*img).srcidx = -(1 as libc::c_int);
    return img;
}
#[no_mangle]
pub unsafe extern fn _zbar_image_free(mut img: *mut zbar_image_t) {
    if !(*img).syms.is_null() {
        zbar_symbol_set_ref((*img).syms, -(1 as libc::c_int));
        (*img).syms = 0 as *mut zbar_symbol_set_t
    }
    free(img as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern fn zbar_image_destroy(mut img: *mut zbar_image_t) {
    _zbar_image_refcnt(img, -(1 as libc::c_int));
}
#[no_mangle]
pub unsafe extern fn zbar_image_ref(mut img: *mut zbar_image_t, mut refs: libc::c_int) {
    _zbar_image_refcnt(img, refs);
}
#[no_mangle]
pub unsafe extern fn zbar_image_get_format(mut img: *const zbar_image_t) -> libc::c_ulong {
    return (*img).format as libc::c_ulong;
}
#[no_mangle]
pub unsafe extern fn zbar_image_get_sequence(mut img: *const zbar_image_t) -> libc::c_uint {
    return (*img).seq;
}
#[no_mangle]
pub unsafe extern fn zbar_image_get_width(mut img: *const zbar_image_t) -> libc::c_uint {
    return (*img).width;
}
#[no_mangle]
pub unsafe extern fn zbar_image_get_height(mut img: *const zbar_image_t) -> libc::c_uint {
    return (*img).height;
}
#[no_mangle]
pub unsafe extern fn zbar_image_get_size(
    mut img: *const zbar_image_t,
    mut w: *mut libc::c_uint,
    mut h: *mut libc::c_uint,
) {
    if !w.is_null() {
        *w = (*img).width
    }
    if !h.is_null() {
        *h = (*img).height
    };
}
#[no_mangle]
pub unsafe extern fn zbar_image_get_crop(
    mut img: *const zbar_image_t,
    mut x: *mut libc::c_uint,
    mut y: *mut libc::c_uint,
    mut w: *mut libc::c_uint,
    mut h: *mut libc::c_uint,
) {
    if !x.is_null() {
        *x = (*img).crop_x
    }
    if !y.is_null() {
        *y = (*img).crop_y
    }
    if !w.is_null() {
        *w = (*img).crop_w
    }
    if !h.is_null() {
        *h = (*img).crop_h
    };
}
#[no_mangle]
pub unsafe extern fn zbar_image_get_data(mut img: *const zbar_image_t) -> *const libc::c_void {
    return (*img).data;
}
#[no_mangle]
pub unsafe extern fn zbar_image_get_data_length(mut img: *const zbar_image_t) -> libc::c_ulong {
    return (*img).datalen;
}
#[no_mangle]
pub unsafe extern fn zbar_image_set_format(mut img: *mut zbar_image_t, mut fmt: libc::c_ulong) {
    (*img).format = fmt as uint32_t;
}
#[no_mangle]
pub unsafe extern fn zbar_image_set_sequence(mut img: *mut zbar_image_t, mut seq: libc::c_uint) {
    (*img).seq = seq;
}
#[no_mangle]
pub unsafe extern fn zbar_image_set_size(
    mut img: *mut zbar_image_t,
    mut w: libc::c_uint,
    mut h: libc::c_uint,
) {
    (*img).crop_y = 0 as libc::c_int as libc::c_uint;
    (*img).crop_x = (*img).crop_y;
    (*img).crop_w = w;
    (*img).width = (*img).crop_w;
    (*img).crop_h = h;
    (*img).height = (*img).crop_h;
}
#[no_mangle]
pub unsafe extern fn zbar_image_set_crop(
    mut img: *mut zbar_image_t,
    mut x: libc::c_uint,
    mut y: libc::c_uint,
    mut w: libc::c_uint,
    mut h: libc::c_uint,
) {
    let mut img_w: libc::c_uint = (*img).width;
    if x > img_w {
        x = img_w
    }
    if x.wrapping_add(w) > img_w {
        w = img_w.wrapping_sub(x)
    }
    (*img).crop_x = x;
    (*img).crop_w = w;
    let mut img_h: libc::c_uint = (*img).height;
    if y > img_h {
        y = img_h
    }
    if y.wrapping_add(h) > img_h {
        h = img_h.wrapping_sub(y)
    }
    (*img).crop_y = y;
    (*img).crop_h = h;
}
// if this function is inline we get an error when testing for the function being defined twice - is that because being external AND inline is stupid?
#[no_mangle]
// #[inline]
#[linkage = "external"]
pub unsafe extern fn zbar_image_free_data(mut img: *mut zbar_image_t) {
    if img.is_null() {
        return;
    }
    if !(*img).src.is_null() {
        let mut newimg: *mut zbar_image_t = 0 as *mut zbar_image_t;
        /* replace video image w/new copy */
        if (*img).refcnt != 0 {
        } else {
            __assert_fail(
                b"img->refcnt\x00" as *const u8 as *const libc::c_char,
                b"zbar/image.c\x00" as *const u8 as *const libc::c_char,
                154 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                    b"void zbar_image_free_data(zbar_image_t *)\x00",
                ))
                .as_ptr(),
            ); /* FIXME needs lock */
        }
        newimg = zbar_image_create();
        memcpy(
            newimg as *mut libc::c_void,
            img as *const libc::c_void,
            ::std::mem::size_of::<zbar_image_t>() as libc::c_ulong,
        );
        /* recycle video image */
        (*newimg).cleanup.expect("non-null function pointer")(newimg);
        /* detach old image from src */
        (*img).cleanup = None;
        (*img).src = 0 as *mut zbar_video_t;
        (*img).srcidx = -(1 as libc::c_int)
    } else if (*img).cleanup.is_some() && !(*img).data.is_null() {
        if (*img).cleanup
            != Some(zbar_image_free_data as unsafe extern fn(_: *mut zbar_image_t) -> ())
        {
            /* using function address to detect this case is a bad idea;
             * windows link libraries add an extra layer of indirection...
             * this works around that problem (bug #2796277)
             */
            let mut cleanup: Option<zbar_image_cleanup_handler_t> = (*img).cleanup;
            (*img).cleanup =
                Some(zbar_image_free_data as unsafe extern fn(_: *mut zbar_image_t) -> ());
            cleanup.expect("non-null function pointer")(img);
        } else {
            free((*img).data as *mut libc::c_void);
        }
    }
    (*img).data = 0 as *const libc::c_void;
}
#[no_mangle]
pub unsafe extern fn zbar_image_set_data(
    mut img: *mut zbar_image_t,
    mut data: *const libc::c_void,
    mut len: libc::c_ulong,
    mut cleanup: Option<zbar_image_cleanup_handler_t>,
) {
    zbar_image_free_data(img);
    (*img).data = data;
    (*img).datalen = len;
    (*img).cleanup = cleanup;
}
#[no_mangle]
pub unsafe extern fn zbar_image_set_userdata(
    mut img: *mut zbar_image_t,
    mut userdata: *mut libc::c_void,
) {
    (*img).userdata = userdata;
}
#[no_mangle]
pub unsafe extern fn zbar_image_get_userdata(mut img: *const zbar_image_t) -> *mut libc::c_void {
    return (*img).userdata;
}
#[no_mangle]
pub unsafe extern fn zbar_image_copy(mut src: *const zbar_image_t) -> *mut zbar_image_t {
    let mut dst: *mut zbar_image_t = zbar_image_create();
    (*dst).format = (*src).format;
    _zbar_image_copy_size(dst, src);
    (*dst).datalen = (*src).datalen;
    (*dst).data = malloc((*src).datalen);
    if !(*dst).data.is_null() {
    } else {
        __assert_fail(
            b"dst->data\x00" as *const u8 as *const libc::c_char,
            b"zbar/image.c\x00" as *const u8 as *const libc::c_char,
            209 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 52], &[libc::c_char; 52]>(
                b"zbar_image_t *zbar_image_copy(const zbar_image_t *)\x00",
            ))
            .as_ptr(),
        );
    }
    memcpy((*dst).data as *mut libc::c_void, (*src).data, (*src).datalen);
    (*dst).cleanup = Some(zbar_image_free_data as unsafe extern fn(_: *mut zbar_image_t) -> ());
    return dst;
}
#[no_mangle]
pub unsafe extern fn zbar_image_get_symbols(
    mut img: *const zbar_image_t,
) -> *const zbar_symbol_set_t {
    return (*img).syms;
}
#[no_mangle]
pub unsafe extern fn zbar_image_set_symbols(
    mut img: *mut zbar_image_t,
    mut syms: *const zbar_symbol_set_t,
) {
    if !syms.is_null() {
        zbar_symbol_set_ref(syms, 1 as libc::c_int);
    }
    if !(*img).syms.is_null() {
        zbar_symbol_set_ref((*img).syms, -(1 as libc::c_int));
    }
    (*img).syms = syms as *mut zbar_symbol_set_t;
}
#[no_mangle]
pub unsafe extern fn zbar_image_first_symbol(mut img: *const zbar_image_t) -> *const zbar_symbol_t {
    return if !(*img).syms.is_null() {
        (*(*img).syms).head
    } else {
        0 as *mut zbar_symbol_t
    };
}
/* @} */
/* ------------------------------------------------------------ */
/* * @name Symbol interface
 * decoded barcode symbol result object.  stores type, data, and image
 * location of decoded symbol.  all memory is owned by the library
 */
/* @{ */
/* * @typedef zbar_symbol_t
 * opaque decoded symbol object.
 */
/* * symbol reference count manipulation.
 * increment the reference count when you store a new reference to the
 * symbol.  decrement when the reference is no longer used.  do not
 * refer to the symbol once the count is decremented and the
 * containing image has been recycled or destroyed.
 * @note the containing image holds a reference to the symbol, so you
 * only need to use this if you keep a symbol after the image has been
 * destroyed or reused.
 * @since 0.9
 */
/* * retrieve type of decoded symbol.
 * @returns the symbol type
 */
/* * retrieve symbology boolean config settings.
 * @returns a bitmask indicating which configs were set for the detected
 * symbology during decoding.
 * @since 0.11
 */
/* * retrieve symbology modifier flag settings.
 * @returns a bitmask indicating which characteristics were detected
 * during decoding.
 * @since 0.11
 */
/* * retrieve data decoded from symbol.
 * @returns the data string
 */
/* * retrieve length of binary data.
 * @returns the length of the decoded data
 */
/* * retrieve a symbol confidence metric.
 * @returns an unscaled, relative quantity: larger values are better
 * than smaller values, where "large" and "small" are application
 * dependent.
 * @note expect the exact definition of this quantity to change as the
 * metric is refined.  currently, only the ordered relationship
 * between two values is defined and will remain stable in the future
 * @since 0.9
 */
/* * retrieve current cache count.  when the cache is enabled for the
 * image_scanner this provides inter-frame reliability and redundancy
 * information for video streams.
 * @returns < 0 if symbol is still uncertain.
 * @returns 0 if symbol is newly verified.
 * @returns > 0 for duplicate symbols
 */
/* * retrieve the number of points in the location polygon.  the
 * location polygon defines the image area that the symbol was
 * extracted from.
 * @returns the number of points in the location polygon
 * @note this is currently not a polygon, but the scan locations
 * where the symbol was decoded
 */
/* * retrieve location polygon x-coordinates.
 * points are specified by 0-based index.
 * @returns the x-coordinate for a point in the location polygon.
 * @returns -1 if index is out of range
 */
/* * retrieve location polygon y-coordinates.
 * points are specified by 0-based index.
 * @returns the y-coordinate for a point in the location polygon.
 * @returns -1 if index is out of range
 */
/* * retrieve general orientation of decoded symbol.
 * @returns a coarse, axis-aligned indication of symbol orientation or
 * ::ZBAR_ORIENT_UNKNOWN if unknown
 * @since 0.11
 */
/* * iterate the set to which this symbol belongs (there can be only one).
 * @returns the next symbol in the set, or
 * @returns NULL when no more results are available
 */
/* * retrieve components of a composite result.
 * @returns the symbol set containing the components
 * @returns NULL if the symbol is already a physical symbol
 * @since 0.10
 */
/* * iterate components of a composite result.
 * @returns the first physical component symbol of a composite result
 * @returns NULL if the symbol is already a physical symbol
 * @since 0.10
 */
/* * print XML symbol element representation to user result buffer.
 * @see http://zbar.sourceforge.net/2008/barcode.xsd for the schema.
 * @param symbol is the symbol to print
 * @param buffer is the inout result pointer, it will be reallocated
 * with a larger size if necessary.
 * @param buflen is inout length of the result buffer.
 * @returns the buffer pointer
 * @since 0.6
 */
/* @} */
/* ------------------------------------------------------------ */
/* * @name Symbol Set interface
 * container for decoded result symbols associated with an image
 * or a composite symbol.
 * @since 0.10
 */
/* @{ */
/* * @typedef zbar_symbol_set_t
 * opaque symbol iterator object.
 * @since 0.10
 */
/* * reference count manipulation.
 * increment the reference count when you store a new reference.
 * decrement when the reference is no longer used.  do not refer to
 * the object any longer once references have been released.
 * @since 0.10
 */
/* * retrieve set size.
 * @returns the number of symbols in the set.
 * @since 0.10
 */
/* * set iterator.
 * @returns the first decoded symbol result in a set
 * @returns NULL if the set is empty
 * @since 0.10
 */
/* * raw result iterator.
 * @returns the first decoded symbol result in a set, *before* filtering
 * @returns NULL if the set is empty
 * @since 0.11
 */
/* @} */
/* ------------------------------------------------------------ */
/* * @name Image interface
 * stores image data samples along with associated format and size
 * metadata
 */
/* @{ */
/* * opaque image object. */
/* * cleanup handler callback function.
 * called to free sample data when an image is destroyed.
 */
/* * data handler callback function.
 * called when decoded symbol results are available for an image
 */
/* * new image constructor.
 * @returns a new image object with uninitialized data and format.
 * this image should be destroyed (using zbar_image_destroy()) as
 * soon as the application is finished with it
 */
/* * image destructor.  all images created by or returned to the
 * application should be destroyed using this function.  when an image
 * is destroyed, the associated data cleanup handler will be invoked
 * if available
 * @note make no assumptions about the image or the data buffer.
 * they may not be destroyed/cleaned immediately if the library
 * is still using them.  if necessary, use the cleanup handler hook
 * to keep track of image data buffers
 */
/* * image reference count manipulation.
 * increment the reference count when you store a new reference to the
 * image.  decrement when the reference is no longer used.  do not
 * refer to the image any longer once the count is decremented.
 * zbar_image_ref(image, -1) is the same as zbar_image_destroy(image)
 * @since 0.5
 */
/* * image format conversion.  refer to the documentation for supported
 * image formats
 * @returns a @em new image with the sample data from the original image
 * converted to the requested format.  the original image is
 * unaffected.
 * @note the converted image size may be rounded (up) due to format
 * constraints
 */
/* * image format conversion with crop/pad.
 * if the requested size is larger than the image, the last row/column
 * are duplicated to cover the difference.  if the requested size is
 * smaller than the image, the extra rows/columns are dropped from the
 * right/bottom.
 * @returns a @em new image with the sample data from the original
 * image converted to the requested format and size.
 * @note the image is @em not scaled
 * @see zbar_image_convert()
 * @since 0.4
 */
/* * retrieve the image format.
 * @returns the fourcc describing the format of the image sample data
 */
/* * retrieve a "sequence" (page/frame) number associated with this image.
 * @since 0.6
 */
/* * retrieve the width of the image.
 * @returns the width in sample columns
 */
/* * retrieve the height of the image.
 * @returns the height in sample rows
 */
/* * retrieve both dimensions of the image.
 * fills in the width and height in samples
 */
/* * retrieve the crop rectangle.
 * fills in the image coordinates of the upper left corner and size
 * of an axis-aligned rectangular area of the image that will be scanned.
 * defaults to the full image
 * @since 0.11
 */
/* * return the image sample data.  the returned data buffer is only
 * valid until zbar_image_destroy() is called
 */
/* * return the size of image data.
 * @since 0.6
 */
/* * retrieve the decoded results.
 * @returns the (possibly empty) set of decoded symbols
 * @returns NULL if the image has not been scanned
 * @since 0.10
 */
/* * associate the specified symbol set with the image, replacing any
 * existing results.  use NULL to release the current results from the
 * image.
 * @see zbar_image_scanner_recycle_image()
 * @since 0.10
 */
/* * image_scanner decode result iterator.
 * @returns the first decoded symbol result for an image
 * or NULL if no results are available
 */
/* * specify the fourcc image format code for image sample data.
 * refer to the documentation for supported formats.
 * @note this does not convert the data!
 * (see zbar_image_convert() for that)
 */
/* * associate a "sequence" (page/frame) number with this image.
 * @since 0.6
 */
/* * specify the pixel size of the image.
 * @note this also resets the crop rectangle to the full image
 * (0, 0, width, height)
 * @note this does not affect the data!
 */
/* * specify a rectangular region of the image to scan.
 * the rectangle will be clipped to the image boundaries.
 * defaults to the full image specified by zbar_image_set_size()
 */
/* * specify image sample data.  when image data is no longer needed by
 * the library the specific data cleanup handler will be called
 * (unless NULL)
 * @note application image data will not be modified by the library
 */
/* * built-in cleanup handler.
 * passes the image data buffer to free()
 */
/* * associate user specified data value with an image.
 * @since 0.5
 */
/* * return user specified data value associated with the image.
 * @since 0.5
 */
/* * dump raw image data to a file for debug.
 * the data will be prefixed with a 16 byte header consisting of:
 *   - 4 bytes uint = 0x676d697a ("zimg")
 *   - 4 bytes format fourcc
 *   - 2 bytes width
 *   - 2 bytes height
 *   - 4 bytes size of following image data in bytes
 * this header can be dumped w/eg:
 * @verbatim
       od -Ax -tx1z -N16 -w4 [file]
@endverbatim
 * for some formats the image can be displayed/converted using
 * ImageMagick, eg:
 * @verbatim
       display -size 640x480+16 [-depth ?] [-sampling-factor ?x?] \
           {GRAY,RGB,UYVY,YUV}:[file]
@endverbatim
 *
 * @param image the image object to dump
 * @param filebase base filename, appended with ".XXXX.zimg" where
 * XXXX is the format fourcc
 * @returns 0 on success or a system error code on failure
 */
#[no_mangle]
pub unsafe extern fn zbar_image_write(
    mut img: *const zbar_image_t,
    mut filebase: *const libc::c_char,
) -> libc::c_int {
    let mut len: libc::c_int =
        strlen(filebase).wrapping_add(16 as libc::c_int as libc::c_ulong) as libc::c_int;
    let mut filename: *mut libc::c_char = malloc(len as libc::c_ulong) as *mut libc::c_char;
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut hdr: zimg_hdr_t = zimg_hdr_t {
        magic: 0,
        format: 0,
        width: 0,
        height: 0,
        size: 0,
    };
    strcpy(filename, filebase);
    if (*img).format & 0xff as libc::c_int as libc::c_uint >= ' ' as i32 as libc::c_uint {
        n = snprintf(
            filename,
            len as libc::c_ulong,
            b"%s.%.4s.zimg\x00" as *const u8 as *const libc::c_char,
            filebase,
            &(*img).format as *const uint32_t as *mut libc::c_char,
        )
    } else {
        n = snprintf(
            filename,
            len as libc::c_ulong,
            b"%s.%08x.zimg\x00" as *const u8 as *const libc::c_char,
            filebase,
            (*img).format,
        )
    }
    if n < len - 1 as libc::c_int {
    } else {
        __assert_fail(
            b"n < len - 1\x00" as *const u8 as *const libc::c_char,
            b"zbar/image.c\x00" as *const u8 as *const libc::c_char,
            256 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                b"int zbar_image_write(const zbar_image_t *, const char *)\x00",
            ))
            .as_ptr(),
        );
    }
    *filename.offset((len - 1 as libc::c_int) as isize) = '\u{0}' as i32 as libc::c_char;
    if _zbar_verbosity >= 1 as libc::c_int {
        fprintf(
            stderr,
            b"%s: dumping %.4s(%08x) image to %s\n\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"zbar_image_write\x00"))
                .as_ptr(),
            &(*img).format as *const uint32_t as *mut libc::c_char,
            (*img).format,
            filename,
        );
    }
    f = fopen(filename, b"w\x00" as *const u8 as *const libc::c_char);
    if f.is_null() {
        rc = *__errno_location();
        if _zbar_verbosity >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"%s: ERROR opening %s: %s\n\x00" as *const u8 as *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"zbar_image_write\x00"))
                    .as_ptr(),
                filename,
                strerror(rc),
            );
        }
    } else {
        hdr.magic = 0x676d697a as libc::c_int as uint32_t;
        hdr.format = (*img).format;
        hdr.width = (*img).width as uint16_t;
        hdr.height = (*img).height as uint16_t;
        hdr.size = (*img).datalen as uint32_t;
        if fwrite(
            &mut hdr as *mut zimg_hdr_t as *const libc::c_void,
            ::std::mem::size_of::<zimg_hdr_t>() as libc::c_ulong,
            1 as libc::c_int as size_t,
            f,
        ) != 1 as libc::c_int as libc::c_ulong
            || fwrite((*img).data, 1 as libc::c_int as size_t, (*img).datalen, f) != (*img).datalen
        {
            rc = *__errno_location();
            if _zbar_verbosity >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"%s: ERROR writing %s: %s\n\x00" as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(
                        b"zbar_image_write\x00",
                    ))
                    .as_ptr(),
                    filename,
                    strerror(rc),
                );
            }
            fclose(f);
        } else {
            rc = fclose(f)
        }
    }
    free(filename as *mut libc::c_void);
    return rc;
}
