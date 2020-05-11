use ::libc;
extern "C" {
    /*@}*/
    /*------------------------------------------------------------*/
/* * @name Video interface
 * @anchor c-video
 * mid-level video source abstraction.
 * captures images from a video device
 */
/*@{*/
    pub type zbar_video_s;
    pub type qr_reader;
    /*@}*/
    /*------------------------------------------------------------*/
/* * @name Decoder interface
 * @anchor c-decoder
 * low-level bar width stream decoder interface.
 * identifies symbols and extracts encoded data
 */
/*@{*/
    pub type zbar_decoder_s;
    /*@}*/
    /*------------------------------------------------------------*/
/* * @name Scanner interface
 * @anchor c-scanner
 * low-level linear intensity sample stream scanner interface.
 * identifies "bar" edges and measures width between them.
 * optionally passes to bar width decoder
 */
/*@{*/
    pub type zbar_scanner_s;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    /* * retrieve string name for symbol encoding.
 * @param sym symbol type encoding
 * @returns the static string name for the specified symbol type,
 * or "UNKNOWN" if the encoding is not recognized
 */
    #[no_mangle]
    fn zbar_get_symbol_name(sym: zbar_symbol_type_t) -> *const libc::c_char;
    /*@}*/
    /*------------------------------------------------------------*/
/* * @name Symbol Set interface
 * container for decoded result symbols associated with an image
 * or a composite symbol.
 * @since 0.10
 */
/*@{*/
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
    #[no_mangle]
    fn zbar_symbol_set_ref(symbols: *const zbar_symbol_set_t,
                           refs: libc::c_int);
    /* * constructor. */
    #[no_mangle]
    fn zbar_decoder_create() -> *mut zbar_decoder_t;
    /* * destructor. */
    #[no_mangle]
    fn zbar_decoder_destroy(decoder: *mut zbar_decoder_t);
    /* * set config for indicated symbology (0 for all) to specified value.
 * @returns 0 for success, non-0 for failure (config does not apply to
 * specified symbology, or value out of range)
 * @since 0.4
 */
    #[no_mangle]
    fn zbar_decoder_set_config(decoder: *mut zbar_decoder_t,
                               symbology: zbar_symbol_type_t,
                               config: zbar_config_t, value: libc::c_int)
     -> libc::c_int;
    /* * retrieve symbology boolean config settings.
 * @returns a bitmask indicating which configs are currently set for the
 * specified symbology.
 * @since 0.11
 */
    #[no_mangle]
    fn zbar_decoder_get_configs(decoder: *const zbar_decoder_t,
                                symbology: zbar_symbol_type_t)
     -> libc::c_uint;
    /* * retrieve last decoded data.
 * @returns the data string or NULL if no new data available.
 * the returned data buffer is owned by library, contents are only
 * valid between non-0 return from zbar_decode_width and next library
 * call
 */
    #[no_mangle]
    fn zbar_decoder_get_data(decoder: *const zbar_decoder_t)
     -> *const libc::c_char;
    /* * retrieve length of binary data.
 * @returns the length of the decoded data or 0 if no new data
 * available.
 */
    #[no_mangle]
    fn zbar_decoder_get_data_length(decoder: *const zbar_decoder_t)
     -> libc::c_uint;
    /* * retrieve last decoded symbol type.
 * @returns the type or ::ZBAR_NONE if no new data available
 */
    #[no_mangle]
    fn zbar_decoder_get_type(decoder: *const zbar_decoder_t)
     -> zbar_symbol_type_t;
    /* * retrieve modifier flags for the last decoded symbol.
 * @returns a bitmask indicating which characteristics were detected
 * during decoding.
 * @since 0.11
 */
    #[no_mangle]
    fn zbar_decoder_get_modifiers(decoder: *const zbar_decoder_t)
     -> libc::c_uint;
    /* * retrieve last decode direction.
 * @returns 1 for forward and -1 for reverse
 * @returns 0 if the decode direction is unknown or does not apply
 * @since 0.11
 */
    #[no_mangle]
    fn zbar_decoder_get_direction(decoder: *const zbar_decoder_t)
     -> libc::c_int;
    /* * setup data handler callback.
 * the registered function will be called by the decoder
 * just before zbar_decode_width() returns a non-zero value.
 * pass a NULL value to disable callbacks.
 * @returns the previously registered handler
 */
    #[no_mangle]
    fn zbar_decoder_set_handler(decoder: *mut zbar_decoder_t,
                                handler: Option<zbar_decoder_handler_t>)
     -> Option<zbar_decoder_handler_t>;
    /* * associate user specified data value with the decoder. */
    #[no_mangle]
    fn zbar_decoder_set_userdata(decoder: *mut zbar_decoder_t,
                                 userdata: *mut libc::c_void);
    /* * return user specified data value associated with the decoder. */
    #[no_mangle]
    fn zbar_decoder_get_userdata(decoder: *const zbar_decoder_t)
     -> *mut libc::c_void;
    /* * constructor.
 * if decoder is non-NULL it will be attached to scanner
 * and called automatically at each new edge
 * current color is initialized to ::ZBAR_SPACE
 * (so an initial BAR->SPACE transition may be discarded)
 */
    #[no_mangle]
    fn zbar_scanner_create(decoder: *mut zbar_decoder_t)
     -> *mut zbar_scanner_t;
    /* * destructor. */
    #[no_mangle]
    fn zbar_scanner_destroy(scanner: *mut zbar_scanner_t);
    /* * mark start of a new scan pass. resets color to ::ZBAR_SPACE.
 * also updates an associated decoder.
 * @returns any decode results flushed from the pipeline
 * @note when not using callback handlers, the return value should
 * be checked the same as zbar_scan_y()
 * @note call zbar_scanner_flush() at least twice before calling this
 * method to ensure no decode results are lost
 */
    #[no_mangle]
    fn zbar_scanner_new_scan(scanner: *mut zbar_scanner_t)
     -> zbar_symbol_type_t;
    /* * flush scanner processing pipeline.
 * forces current scanner position to be a scan boundary.
 * call multiple times (max 3) to completely flush decoder.
 * @returns any decode/scan results flushed from the pipeline
 * @note when not using callback handlers, the return value should
 * be checked the same as zbar_scan_y()
 * @since 0.9
 */
    #[no_mangle]
    fn zbar_scanner_flush(scanner: *mut zbar_scanner_t) -> zbar_symbol_type_t;
    /* * process next sample intensity value.
 * intensity (y) is in arbitrary relative units.
 * @returns result of zbar_decode_width() if a decoder is attached,
 * otherwise @returns (::ZBAR_PARTIAL) when new edge is detected
 * or 0 (::ZBAR_NONE) if no new edge is detected
 */
    #[no_mangle]
    fn zbar_scan_y(scanner: *mut zbar_scanner_t, y: libc::c_int)
     -> zbar_symbol_type_t;
    /* * retrieve last scanned width. */
    #[no_mangle]
    fn zbar_scanner_get_width(scanner: *const zbar_scanner_t) -> libc::c_uint;
    /* * retrieve sample position of last edge.
 * @since 0.10
 */
    #[no_mangle]
    fn zbar_scanner_get_edge(scn: *const zbar_scanner_t, offset: libc::c_uint,
                             prec: libc::c_int) -> libc::c_uint;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut _zbar_verbosity: libc::c_int;
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
 *------------------------------------------------------------------------*/
    #[no_mangle]
    fn _zbar_refcnt(cnt: *mut refcnt_t, delta: libc::c_int) -> libc::c_int;
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
 *------------------------------------------------------------------------*/
    /* number of filtered symbols */
    /* first of decoded symbol results */
    /* last of unfiltered symbol results */
    /* symbol type */
    /* symbology boolean config bitmask */
    /* symbology modifier bitmask */
    /* allocation size of data */
    /* length of binary symbol data */
    /* symbol data */
    /* allocation size of pts */
    /* number of points in location polygon */
    /* list of points in location polygon */
    /* coarse orientation */
    /* reference count */
    /* linked list of results (or siblings) */
    /* components of composite result */
    /* relative symbol capture time */
    /* cache state */
    /* relative symbol reliability metric */
    #[no_mangle]
    fn _zbar_symbol_free(_: *mut zbar_symbol_t);
    #[no_mangle]
    fn _zbar_get_symbol_hash(_: zbar_symbol_type_t) -> libc::c_int;
    #[no_mangle]
    fn _zbar_symbol_set_free(_: *mut zbar_symbol_set_t);
    #[no_mangle]
    fn _zbar_symbol_set_create() -> *mut zbar_symbol_set_t;
    #[no_mangle]
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec)
     -> libc::c_int;
    #[no_mangle]
    fn _zbar_qr_create() -> *mut qr_reader;
    #[no_mangle]
    fn _zbar_qr_destroy(reader: *mut qr_reader);
    #[no_mangle]
    fn _zbar_qr_reset(reader: *mut qr_reader);
    #[no_mangle]
    fn _zbar_qr_found_line(reader: *mut qr_reader, direction: libc::c_int,
                           line: *const qr_finder_line) -> libc::c_int;
    #[no_mangle]
    fn _zbar_qr_decode(reader: *mut qr_reader,
                       iscn: *mut zbar_image_scanner_t,
                       img: *mut zbar_image_t) -> libc::c_int;
    #[no_mangle]
    fn _zbar_decoder_get_qr_finder_line(_: *mut zbar_decoder_t)
     -> *mut qr_finder_line;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
pub type __intptr_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type intptr_t = __intptr_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uintptr_t = libc::c_ulong;
pub type clockid_t = __clockid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type zbar_symbol_type_e = libc::c_uint;
pub const ZBAR_ADDON: zbar_symbol_type_e = 1792;
pub const ZBAR_ADDON5: zbar_symbol_type_e = 1280;
pub const ZBAR_ADDON2: zbar_symbol_type_e = 512;
pub const ZBAR_SYMBOL: zbar_symbol_type_e = 255;
pub const ZBAR_CODE128: zbar_symbol_type_e = 128;
pub const ZBAR_CODE93: zbar_symbol_type_e = 93;
pub const ZBAR_QRCODE: zbar_symbol_type_e = 64;
pub const ZBAR_PDF417: zbar_symbol_type_e = 57;
pub const ZBAR_CODE39: zbar_symbol_type_e = 39;
pub const ZBAR_CODABAR: zbar_symbol_type_e = 38;
pub const ZBAR_DATABAR_EXP: zbar_symbol_type_e = 35;
pub const ZBAR_DATABAR: zbar_symbol_type_e = 34;
pub const ZBAR_I25: zbar_symbol_type_e = 25;
pub const ZBAR_COMPOSITE: zbar_symbol_type_e = 15;
pub const ZBAR_ISBN13: zbar_symbol_type_e = 14;
pub const ZBAR_EAN13: zbar_symbol_type_e = 13;
pub const ZBAR_UPCA: zbar_symbol_type_e = 12;
pub const ZBAR_ISBN10: zbar_symbol_type_e = 10;
pub const ZBAR_UPCE: zbar_symbol_type_e = 9;
pub const ZBAR_EAN8: zbar_symbol_type_e = 8;
pub const ZBAR_EAN5: zbar_symbol_type_e = 5;
pub const ZBAR_EAN2: zbar_symbol_type_e = 2;
pub const ZBAR_PARTIAL: zbar_symbol_type_e = 1;
pub const ZBAR_NONE: zbar_symbol_type_e = 0;
/* * decoded symbol type. */
pub type zbar_symbol_type_t = zbar_symbol_type_e;
pub type zbar_orientation_e = libc::c_int;
pub const ZBAR_ORIENT_LEFT: zbar_orientation_e = 3;
pub const ZBAR_ORIENT_DOWN: zbar_orientation_e = 2;
pub const ZBAR_ORIENT_RIGHT: zbar_orientation_e = 1;
pub const ZBAR_ORIENT_UP: zbar_orientation_e = 0;
pub const ZBAR_ORIENT_UNKNOWN: zbar_orientation_e = -1;
/* *< no symbol decoded */
/* *< intermediate status */
/* *< GS1 2-digit add-on */
/* *< GS1 5-digit add-on */
/* *< EAN-8 */
/* *< UPC-E */
/* *< ISBN-10 (from EAN-13). @since 0.4 */
/* *< UPC-A */
/* *< EAN-13 */
/* *< ISBN-13 (from EAN-13). @since 0.4 */
/* *< EAN/UPC composite */
/* *< Interleaved 2 of 5. @since 0.4 */
/* *< GS1 DataBar (RSS). @since 0.11 */
/* *< GS1 DataBar Expanded. @since 0.11 */
/* *< Codabar. @since 0.11 */
/* *< Code 39. @since 0.4 */
/* *< PDF417. @since 0.6 */
/* *< QR Code. @since 0.10 */
/* *< Code 93. @since 0.11 */
/* *< Code 128 */
/* * mask for base symbol type.
     * @deprecated in 0.11, remove this from existing code
     */
/* * 2-digit add-on flag.
     * @deprecated in 0.11, a ::ZBAR_EAN2 component is used for
     * 2-digit GS1 add-ons
     */
/* * 5-digit add-on flag.
     * @deprecated in 0.11, a ::ZBAR_EAN5 component is used for
     * 5-digit GS1 add-ons
     */
/* * add-on flag mask.
     * @deprecated in 0.11, GS1 add-ons are represented using composite
     * symbols of type ::ZBAR_COMPOSITE; add-on components use ::ZBAR_EAN2
     * or ::ZBAR_EAN5
     */
/* * decoded symbol coarse orientation.
 * @since 0.11
 */
pub type zbar_orientation_t = zbar_orientation_e;
pub type zbar_config_e = libc::c_uint;
pub const ZBAR_CFG_Y_DENSITY: zbar_config_e = 257;
pub const ZBAR_CFG_X_DENSITY: zbar_config_e = 256;
pub const ZBAR_CFG_POSITION: zbar_config_e = 128;
pub const ZBAR_CFG_UNCERTAINTY: zbar_config_e = 64;
pub const ZBAR_CFG_MAX_LEN: zbar_config_e = 33;
pub const ZBAR_CFG_MIN_LEN: zbar_config_e = 32;
pub const ZBAR_CFG_NUM: zbar_config_e = 4;
pub const ZBAR_CFG_ASCII: zbar_config_e = 3;
pub const ZBAR_CFG_EMIT_CHECK: zbar_config_e = 2;
pub const ZBAR_CFG_ADD_CHECK: zbar_config_e = 1;
pub const ZBAR_CFG_ENABLE: zbar_config_e = 0;
/* *< unable to determine orientation */
/* *< upright, read left to right */
/* *< sideways, read top to bottom */
/* *< upside-down, read right to left */
/* *< sideways, read bottom to top */
/* * decoder configuration options.
 * @since 0.4
 */
pub type zbar_config_t = zbar_config_e;
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
/* *< enable symbology/feature */
/* *< enable check digit when optional */
/* *< return check digit when present */
/* *< enable full ASCII character set */
/* *< number of boolean decoder configs */
/* *< minimum data length for valid decode */
/* *< maximum data length for valid decode */
/* *< required video consistency frames */
/* *< enable scanner to collect position data */
/* *< image scanner vertical scan density */
/* *< image scanner horizontal scan density */
/*@}*/
pub type zbar_symbol_t = zbar_symbol_s;
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
 *------------------------------------------------------------------------*/
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
/*@}*/
/*------------------------------------------------------------*/
/* * @name Image interface
 * stores image data samples along with associated format and size
 * metadata
 */
/*@{*/
/* * opaque image object. */
pub type zbar_image_t = zbar_image_s;
/* * opaque video object. */
pub type zbar_video_t = zbar_video_s;
/* * cleanup handler callback function.
 * called to free sample data when an image is destroyed.
 */
pub type zbar_image_cleanup_handler_t
    =
    unsafe extern "C" fn(_: *mut zbar_image_t) -> ();
/* * data handler callback function.
 * called when decoded symbol results are available for an image
 */
pub type zbar_image_data_handler_t
    =
    unsafe extern "C" fn(_: *mut zbar_image_t, _: *const libc::c_void) -> ();
/* image scanner state */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zbar_image_scanner_s {
    pub scn: *mut zbar_scanner_t,
    pub dcode: *mut zbar_decoder_t,
    pub qr: *mut qr_reader,
    pub userdata: *const libc::c_void,
    pub handler: Option<zbar_image_data_handler_t>,
    pub time: libc::c_ulong,
    pub img: *mut zbar_image_t,
    pub dx: libc::c_int,
    pub dy: libc::c_int,
    pub du: libc::c_int,
    pub umin: libc::c_int,
    pub v: libc::c_int,
    pub syms: *mut zbar_symbol_set_t,
    pub recycle: [recycle_bucket_t; 5],
    pub enable_cache: libc::c_int,
    pub cache: *mut zbar_symbol_t,
    pub config: libc::c_uint,
    pub ean_config: libc::c_uint,
    pub configs: [libc::c_int; 2],
    pub sym_configs: [[libc::c_int; 20]; 1],
    pub stat_syms_new: libc::c_int,
    pub stat_iscn_syms_inuse: libc::c_int,
    pub stat_iscn_syms_recycle: libc::c_int,
    pub stat_img_syms_inuse: libc::c_int,
    pub stat_img_syms_recycle: libc::c_int,
    pub stat_sym_new: libc::c_int,
    pub stat_sym_recycle: [libc::c_int; 5],
}
pub type recycle_bucket_t = recycle_bucket_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct recycle_bucket_s {
    pub nsyms: libc::c_int,
    pub head: *mut zbar_symbol_t,
}
/* * opaque decoder object. */
pub type zbar_decoder_t = zbar_decoder_s;
/* * opaque scanner object. */
pub type zbar_scanner_t = zbar_scanner_s;
/*@}*/
/*------------------------------------------------------------*/
/* * @name Image Scanner interface
 * @anchor c-imagescanner
 * mid-level image scanner interface.
 * reads barcodes from 2-D images
 */
/*@{*/
/* * opaque image scanner object. */
pub type zbar_image_scanner_t = zbar_image_scanner_s;
/* * decoder data handler callback function.
 * called by decoder when new data has just been decoded
 */
pub type zbar_decoder_handler_t
    =
    unsafe extern "C" fn(_: *mut zbar_decoder_t) -> ();
pub type FILE = _IO_FILE;
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
/*A line crossing a finder pattern.
  Whether the line is horizontal or vertical is determined by context.
  The offsts to various parts of the finder pattern are as follows:
    |*****|     |*****|*****|*****|     |*****|
    |*****|     |*****|*****|*****|     |*****|
       ^        ^                 ^        ^
       |        |                 |        |
       |        |                 |       pos[v]+len+eoffs
       |        |                pos[v]+len
       |       pos[v]
      pos[v]-boffs
  Here v is 0 for horizontal and 1 for vertical lines.*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qr_finder_line {
    pub pos: qr_point,
    pub len: libc::c_int,
    pub boffs: libc::c_int,
    pub eoffs: libc::c_int,
}
pub type qr_point = [libc::c_int; 2];
#[inline]
unsafe extern "C" fn _zbar_symbol_refcnt(mut sym: *mut zbar_symbol_t,
                                         mut delta: libc::c_int) {
    if _zbar_refcnt(&mut (*sym).refcnt, delta) == 0 &&
           delta <= 0 as libc::c_int {
        _zbar_symbol_free(sym);
    };
}
#[inline]
unsafe extern "C" fn sym_add_point(mut sym: *mut zbar_symbol_t,
                                   mut x: libc::c_int, mut y: libc::c_int) {
    let mut i: libc::c_int = (*sym).npts as libc::c_int;
    (*sym).npts = (*sym).npts.wrapping_add(1);
    if (*sym).npts >= (*sym).pts_alloc {
        (*sym).pts_alloc = (*sym).pts_alloc.wrapping_add(1);
        (*sym).pts =
            realloc((*sym).pts as *mut libc::c_void,
                    ((*sym).pts_alloc as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<point_t>()
                                                         as libc::c_ulong)) as
                *mut point_t
    }
    (*(*sym).pts.offset(i as isize)).x = x;
    (*(*sym).pts.offset(i as isize)).y = y;
}
#[inline]
unsafe extern "C" fn _zbar_timer_now() -> libc::c_int {
    let mut now: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    clock_gettime(0 as libc::c_int, &mut now);
    return (now.tv_sec * 1000 as libc::c_int as libc::c_long +
                now.tv_nsec / 1000000 as libc::c_int as libc::c_long) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _zbar_image_scanner_recycle_syms(mut iscn:
                                                              *mut zbar_image_scanner_t,
                                                          mut sym:
                                                              *mut zbar_symbol_t) {
    let mut next: *mut zbar_symbol_t = 0 as *mut zbar_symbol_t;
    while !sym.is_null() {
        next = (*sym).next;
        if (*sym).refcnt != 0 &&
               _zbar_refcnt(&mut (*sym).refcnt, -(1 as libc::c_int)) != 0 {
            /* unlink referenced symbol */
            /* FIXME handle outstanding component refs (currently unsupported)
             */
            if (*sym).data_alloc != 0 {
            } else {
                __assert_fail(b"sym->data_alloc\x00" as *const u8 as
                                  *const libc::c_char,
                              b"zbar/img_scanner.c\x00" as *const u8 as
                                  *const libc::c_char,
                              133 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 79],
                                                        &[libc::c_char; 79]>(b"void _zbar_image_scanner_recycle_syms(zbar_image_scanner_t *, zbar_symbol_t *)\x00")).as_ptr());
            }
            (*sym).next = 0 as *mut zbar_symbol_t
        } else {
            let mut i: libc::c_int = 0;
            let mut bucket: *mut recycle_bucket_t =
                0 as *mut recycle_bucket_t;
            /* recycle unreferenced symbol */
            if (*sym).data_alloc == 0 {
                (*sym).data = 0 as *mut libc::c_char;
                (*sym).datalen = 0 as libc::c_int as libc::c_uint
            }
            if !(*sym).syms.is_null() {
                if _zbar_refcnt(&mut (*(*sym).syms).refcnt,
                                -(1 as libc::c_int)) != 0 {
                    __assert_fail(b"0\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"zbar/img_scanner.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  146 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 79],
                                                            &[libc::c_char; 79]>(b"void _zbar_image_scanner_recycle_syms(zbar_image_scanner_t *, zbar_symbol_t *)\x00")).as_ptr());
                }
                _zbar_image_scanner_recycle_syms(iscn, (*(*sym).syms).head);
                (*(*sym).syms).head = 0 as *mut zbar_symbol_t;
                _zbar_symbol_set_free((*sym).syms);
                (*sym).syms = 0 as *mut zbar_symbol_set_t
            }
            i = 0 as libc::c_int;
            while i < 5 as libc::c_int {
                if (*sym).data_alloc <
                       ((1 as libc::c_int) << i * 2 as libc::c_int) as
                           libc::c_uint {
                    break ;
                }
                i += 1
            }
            if i == 5 as libc::c_int {
                if !(*sym).data.is_null() {
                } else {
                    __assert_fail(b"sym->data\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"zbar/img_scanner.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  156 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 79],
                                                            &[libc::c_char; 79]>(b"void _zbar_image_scanner_recycle_syms(zbar_image_scanner_t *, zbar_symbol_t *)\x00")).as_ptr());
                }
                free((*sym).data as *mut libc::c_void);
                (*sym).data = 0 as *mut libc::c_char;
                (*sym).data_alloc = 0 as libc::c_int as libc::c_uint;
                i = 0 as libc::c_int
            }
            bucket =
                &mut *(*iscn).recycle.as_mut_ptr().offset(i as isize) as
                    *mut recycle_bucket_t;
            /* FIXME cap bucket fill */
            (*bucket).nsyms += 1;
            (*sym).next = (*bucket).head;
            (*bucket).head = sym
        }
        sym = next
    };
}
#[inline]
unsafe extern "C" fn recycle_syms(mut iscn: *mut zbar_image_scanner_t,
                                  mut syms: *mut zbar_symbol_set_t)
 -> libc::c_int {
    if _zbar_refcnt(&mut (*syms).refcnt, -(1 as libc::c_int)) != 0 {
        return 1 as libc::c_int
    }
    _zbar_image_scanner_recycle_syms(iscn, (*syms).head);
    (*syms).tail = 0 as *mut zbar_symbol_t;
    (*syms).head = (*syms).tail;
    (*syms).nsyms = 0 as libc::c_int;
    return 0 as libc::c_int;
}
/* * remove any previously decoded results from the image scanner and the
 * specified image.  somewhat more efficient version of
 * zbar_image_set_symbols(image, NULL) which may retain memory for
 * subsequent decodes
 * @since 0.10
 */
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn zbar_image_scanner_recycle_image(mut iscn:
                                                              *mut zbar_image_scanner_t,
                                                          mut img:
                                                              *mut zbar_image_t) {
    let mut syms: *mut zbar_symbol_set_t = (*iscn).syms;
    if !syms.is_null() && (*syms).refcnt != 0 {
        if recycle_syms(iscn, syms) != 0 {
            (*iscn).stat_iscn_syms_inuse += 1;
            (*iscn).syms = 0 as *mut zbar_symbol_set_t
        } else { (*iscn).stat_iscn_syms_recycle += 1 }
    }
    syms = (*img).syms;
    (*img).syms = 0 as *mut zbar_symbol_set_t;
    if !syms.is_null() && recycle_syms(iscn, syms) != 0 {
        (*iscn).stat_img_syms_inuse += 1
    } else if !syms.is_null() {
        (*iscn).stat_img_syms_recycle += 1;
        /* select one set to resurrect, destroy the other */
        if !(*iscn).syms.is_null() {
            _zbar_symbol_set_free(syms);
        } else { (*iscn).syms = syms }
    };
}
/*------------------------------------------------------------------------
 *  Copyright 2007-2009 (c) Jeff Brown <spadix@users.sourceforge.net>
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
 *------------------------------------------------------------------------*/
/* internal image scanner APIs for 2D readers */
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn _zbar_image_scanner_alloc_sym(mut iscn:
                                                           *mut zbar_image_scanner_t,
                                                       mut type_0:
                                                           zbar_symbol_type_t,
                                                       mut datalen:
                                                           libc::c_int)
 -> *mut zbar_symbol_t {
    /* recycle old or alloc new symbol */
    let mut sym: *mut zbar_symbol_t = 0 as *mut zbar_symbol_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int - 1 as libc::c_int {
        if datalen <= (1 as libc::c_int) << i * 2 as libc::c_int { break ; }
        i += 1
    }
    while i > 0 as libc::c_int {
        sym = (*iscn).recycle[i as usize].head;
        if !sym.is_null() {
            (*iscn).stat_sym_recycle[i as usize] += 1;
            break ;
        } else { i -= 1 }
    }
    if !sym.is_null() {
        (*iscn).recycle[i as usize].head = (*sym).next;
        (*sym).next = 0 as *mut zbar_symbol_t;
        if (*iscn).recycle[i as usize].nsyms != 0 {
        } else {
            __assert_fail(b"iscn->recycle[i].nsyms\x00" as *const u8 as
                              *const libc::c_char,
                          b"zbar/img_scanner.c\x00" as *const u8 as
                              *const libc::c_char,
                          232 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 94],
                                                    &[libc::c_char; 94]>(b"zbar_symbol_t *_zbar_image_scanner_alloc_sym(zbar_image_scanner_t *, zbar_symbol_type_t, int)\x00")).as_ptr());
        }
        (*iscn).recycle[i as usize].nsyms -= 1
    } else {
        sym =
            calloc(1 as libc::c_int as libc::c_ulong,
                   ::std::mem::size_of::<zbar_symbol_t>() as libc::c_ulong) as
                *mut zbar_symbol_t;
        (*iscn).stat_sym_new += 1
    }
    /* init new symbol */
    (*sym).type_0 = type_0;
    (*sym).quality = 1 as libc::c_int;
    (*sym).npts = 0 as libc::c_int as libc::c_uint;
    (*sym).orient = ZBAR_ORIENT_UNKNOWN;
    (*sym).cache_count = 0 as libc::c_int;
    (*sym).time = (*iscn).time;
    if (*sym).syms.is_null() {
    } else {
        __assert_fail(b"!sym->syms\x00" as *const u8 as *const libc::c_char,
                      b"zbar/img_scanner.c\x00" as *const u8 as
                          *const libc::c_char,
                      247 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 94],
                                                &[libc::c_char; 94]>(b"zbar_symbol_t *_zbar_image_scanner_alloc_sym(zbar_image_scanner_t *, zbar_symbol_type_t, int)\x00")).as_ptr());
    }
    if datalen > 0 as libc::c_int {
        (*sym).datalen = (datalen - 1 as libc::c_int) as libc::c_uint;
        if (*sym).data_alloc < datalen as libc::c_uint {
            if !(*sym).data.is_null() {
                free((*sym).data as *mut libc::c_void);
            }
            (*sym).data_alloc = datalen as libc::c_uint;
            (*sym).data =
                malloc(datalen as libc::c_ulong) as *mut libc::c_char
        }
    } else {
        if !(*sym).data.is_null() { free((*sym).data as *mut libc::c_void); }
        (*sym).data = 0 as *mut libc::c_char;
        (*sym).data_alloc = 0 as libc::c_int as libc::c_uint;
        (*sym).datalen = (*sym).data_alloc
    }
    return sym;
}
#[inline]
unsafe extern "C" fn cache_lookup(mut iscn: *mut zbar_image_scanner_t,
                                  mut sym: *mut zbar_symbol_t)
 -> *mut zbar_symbol_t {
    /* search for matching entry in cache */
    let mut entry: *mut *mut zbar_symbol_t = &mut (*iscn).cache;
    while !(*entry).is_null() {
        if (**entry).type_0 as libc::c_uint == (*sym).type_0 as libc::c_uint
               && (**entry).datalen == (*sym).datalen &&
               memcmp((**entry).data as *const libc::c_void,
                      (*sym).data as *const libc::c_void,
                      (*sym).datalen as libc::c_ulong) == 0 {
            break ;
        }
        if (*sym).time.wrapping_sub((**entry).time) >
               (2000 as libc::c_int * 2 as libc::c_int) as libc::c_ulong {
            /* recycle stale cache entry */
            let mut next: *mut zbar_symbol_t = (**entry).next;
            (**entry).next = 0 as *mut zbar_symbol_t;
            _zbar_image_scanner_recycle_syms(iscn, *entry);
            *entry = next
        } else { entry = &mut (**entry).next }
    }
    return *entry;
}
#[inline]
unsafe extern "C" fn cache_sym(mut iscn: *mut zbar_image_scanner_t,
                               mut sym: *mut zbar_symbol_t) {
    if (*iscn).enable_cache != 0 {
        let mut age: uint32_t = 0;
        let mut near_thresh: uint32_t = 0;
        let mut far_thresh: uint32_t = 0;
        let mut dup: uint32_t = 0;
        let mut entry: *mut zbar_symbol_t = cache_lookup(iscn, sym);
        if entry.is_null() {
            /* FIXME reuse sym */
            entry =
                _zbar_image_scanner_alloc_sym(iscn, (*sym).type_0,
                                              (*sym).datalen.wrapping_add(1 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_uint)
                                                  as libc::c_int);
            (*entry).configs = (*sym).configs;
            (*entry).modifiers = (*sym).modifiers;
            memcpy((*entry).data as *mut libc::c_void,
                   (*sym).data as *const libc::c_void,
                   (*sym).datalen as libc::c_ulong);
            (*entry).time =
                (*sym).time.wrapping_sub(2000 as libc::c_int as
                                             libc::c_ulong);
            (*entry).cache_count = 0 as libc::c_int;
            /* add to cache */
            (*entry).next = (*iscn).cache;
            (*iscn).cache = entry
        }
        /* consistency check and hysteresis */
        age = (*sym).time.wrapping_sub((*entry).time) as uint32_t;
        (*entry).time = (*sym).time;
        near_thresh =
            (age < 1000 as libc::c_int as libc::c_uint) as libc::c_int as
                uint32_t;
        far_thresh =
            (age >= 2000 as libc::c_int as libc::c_uint) as libc::c_int as
                uint32_t;
        dup =
            ((*entry).cache_count >= 0 as libc::c_int) as libc::c_int as
                uint32_t;
        if dup == 0 && near_thresh == 0 || far_thresh != 0 {
            let mut type_0: libc::c_int = (*sym).type_0 as libc::c_int;
            let mut h: libc::c_int =
                _zbar_get_symbol_hash(type_0 as zbar_symbol_type_t);
            (*entry).cache_count =
                -(*iscn).sym_configs[0 as libc::c_int as usize][h as usize]
        } else if dup != 0 || near_thresh != 0 { (*entry).cache_count += 1 }
        (*sym).cache_count = (*entry).cache_count
    } else { (*sym).cache_count = 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn _zbar_image_scanner_add_sym(mut iscn:
                                                         *mut zbar_image_scanner_t,
                                                     mut sym:
                                                         *mut zbar_symbol_t) {
    let mut syms: *mut zbar_symbol_set_t = 0 as *mut zbar_symbol_set_t;
    cache_sym(iscn, sym);
    syms = (*iscn).syms;
    if (*sym).cache_count != 0 || (*syms).tail.is_null() {
        (*sym).next = (*syms).head;
        (*syms).head = sym
    } else { (*sym).next = (*(*syms).tail).next; (*(*syms).tail).next = sym }
    if (*sym).cache_count == 0 {
        (*syms).nsyms += 1
    } else if (*syms).tail.is_null() { (*syms).tail = sym }
    _zbar_symbol_refcnt(sym, 1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn qr_handler(mut iscn: *mut zbar_image_scanner_t) {
    let mut u: libc::c_uint = 0;
    let mut vert: libc::c_int = 0;
    let mut line: *mut qr_finder_line =
        _zbar_decoder_get_qr_finder_line((*iscn).dcode);
    if !line.is_null() {
    } else {
        __assert_fail(b"line\x00" as *const u8 as *const libc::c_char,
                      b"zbar/img_scanner.c\x00" as *const u8 as
                          *const libc::c_char,
                      367 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 40],
                                                &[libc::c_char; 40]>(b"void qr_handler(zbar_image_scanner_t *)\x00")).as_ptr());
    }
    u =
        zbar_scanner_get_edge((*iscn).scn,
                              (*line).pos[0 as libc::c_int as usize] as
                                  libc::c_uint, 2 as libc::c_int);
    (*line).boffs =
        u.wrapping_sub(zbar_scanner_get_edge((*iscn).scn,
                                             (*line).boffs as libc::c_uint,
                                             2 as libc::c_int)) as
            libc::c_int;
    (*line).len =
        zbar_scanner_get_edge((*iscn).scn, (*line).len as libc::c_uint,
                              2 as libc::c_int) as libc::c_int;
    (*line).eoffs =
        zbar_scanner_get_edge((*iscn).scn, (*line).eoffs as libc::c_uint,
                              2 as
                                  libc::c_int).wrapping_sub((*line).len as
                                                                libc::c_uint)
            as libc::c_int;
    (*line).len =
        ((*line).len as libc::c_uint).wrapping_sub(u) as libc::c_int as
            libc::c_int;
    u =
        (((((*iscn).umin << 1 as libc::c_int) + 0 as libc::c_int) <<
              2 as libc::c_int - 1 as libc::c_int) as
             libc::c_uint).wrapping_add(((*iscn).du as
                                             libc::c_uint).wrapping_mul(u));
    if (*iscn).du < 0 as libc::c_int {
        let mut tmp: libc::c_int = (*line).boffs;
        (*line).boffs = (*line).eoffs;
        (*line).eoffs = tmp;
        u = u.wrapping_sub((*line).len as libc::c_uint)
    }
    vert = ((*iscn).dx == 0) as libc::c_int;
    (*line).pos[vert as usize] = u as libc::c_int;
    (*line).pos[(vert == 0) as libc::c_int as usize] =
        (((*iscn).v << 1 as libc::c_int) + 1 as libc::c_int) <<
            2 as libc::c_int - 1 as libc::c_int;
    _zbar_qr_found_line((*iscn).qr, vert, line);
}
unsafe extern "C" fn symbol_handler(mut dcode: *mut zbar_decoder_t) {
    let mut iscn: *mut zbar_image_scanner_t =
        zbar_decoder_get_userdata(dcode) as *mut zbar_image_scanner_t;
    let mut type_0: zbar_symbol_type_t = zbar_decoder_get_type(dcode);
    let mut x: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    let mut dir: libc::c_int = 0;
    let mut data: *const libc::c_char = 0 as *const libc::c_char;
    let mut datalen: libc::c_uint = 0;
    let mut sym: *mut zbar_symbol_t = 0 as *mut zbar_symbol_t;
    if type_0 as libc::c_uint == ZBAR_QRCODE as libc::c_int as libc::c_uint {
        qr_handler(iscn);
        return
    }
    if (*iscn).config >>
           ZBAR_CFG_POSITION as libc::c_int - ZBAR_CFG_POSITION as libc::c_int
           & 1 as libc::c_int as libc::c_uint != 0 {
        /* tmp position fixup */
        let mut w: libc::c_int =
            zbar_scanner_get_width((*iscn).scn) as libc::c_int;
        let mut u: libc::c_int =
            ((*iscn).umin as
                 libc::c_uint).wrapping_add(((*iscn).du as
                                                 libc::c_uint).wrapping_mul(zbar_scanner_get_edge((*iscn).scn,
                                                                                                  w
                                                                                                      as
                                                                                                      libc::c_uint,
                                                                                                  0
                                                                                                      as
                                                                                                      libc::c_int)))
                as libc::c_int;
        if (*iscn).dx != 0 {
            x = u;
            y = (*iscn).v
        } else { x = (*iscn).v; y = u }
    }
    /* FIXME debug flag to save/display all PARTIALs */
    if type_0 as libc::c_uint <= ZBAR_PARTIAL as libc::c_int as libc::c_uint {
        if _zbar_verbosity >= 256 as libc::c_int {
            fprintf(stderr,
                    b"%s: partial symbol @(%d,%d)\n\x00" as *const u8 as
                        *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 15],
                                              &[libc::c_char; 15]>(b"symbol_handler\x00")).as_ptr(),
                    x, y);
        }
        return
    }
    data = zbar_decoder_get_data(dcode);
    datalen = zbar_decoder_get_data_length(dcode);
    /* FIXME need better symbol matching */
    sym = (*(*iscn).syms).head;
    while !sym.is_null() {
        if (*sym).type_0 as libc::c_uint == type_0 as libc::c_uint &&
               (*sym).datalen == datalen &&
               memcmp((*sym).data as *const libc::c_void,
                      data as *const libc::c_void, datalen as libc::c_ulong)
                   == 0 {
            (*sym).quality += 1;
            if _zbar_verbosity >= 224 as libc::c_int {
                fprintf(stderr,
                        b"%s: dup symbol @(%d,%d): dup %s: %.20s\n\x00" as
                            *const u8 as *const libc::c_char,
                        (*::std::mem::transmute::<&[u8; 15],
                                                  &[libc::c_char; 15]>(b"symbol_handler\x00")).as_ptr(),
                        x, y, zbar_get_symbol_name(type_0), data);
            }
            if (*iscn).config >>
                   ZBAR_CFG_POSITION as libc::c_int -
                       ZBAR_CFG_POSITION as libc::c_int &
                   1 as libc::c_int as libc::c_uint != 0 {
                /* add new point to existing set */
                /* FIXME should be polygon */
                sym_add_point(sym, x, y);
            }
            return
        }
        sym = (*sym).next
    }
    sym =
        _zbar_image_scanner_alloc_sym(iscn, type_0,
                                      datalen.wrapping_add(1 as libc::c_int as
                                                               libc::c_uint)
                                          as libc::c_int);
    (*sym).configs = zbar_decoder_get_configs(dcode, type_0);
    (*sym).modifiers = zbar_decoder_get_modifiers(dcode);
    /* FIXME grab decoder buffer */
    memcpy((*sym).data as *mut libc::c_void, data as *const libc::c_void,
           datalen.wrapping_add(1 as libc::c_int as libc::c_uint) as
               libc::c_ulong);
    /* initialize first point */
    if (*iscn).config >>
           ZBAR_CFG_POSITION as libc::c_int - ZBAR_CFG_POSITION as libc::c_int
           & 1 as libc::c_int as libc::c_uint != 0 {
        if _zbar_verbosity >= 192 as libc::c_int {
            fprintf(stderr,
                    b"%s: new symbol @(%d,%d): %s: %.20s\n\x00" as *const u8
                        as *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 15],
                                              &[libc::c_char; 15]>(b"symbol_handler\x00")).as_ptr(),
                    x, y, zbar_get_symbol_name(type_0), data);
        }
        sym_add_point(sym, x, y);
    }
    dir = zbar_decoder_get_direction(dcode);
    if dir != 0 {
        (*sym).orient =
            (((*iscn).dy != 0 as libc::c_int) as libc::c_int +
                 (((*iscn).du ^ dir) & 2 as libc::c_int)) as
                zbar_orientation_t
    }
    _zbar_image_scanner_add_sym(iscn, sym);
}
/* * constructor. */
#[no_mangle]
pub unsafe extern "C" fn zbar_image_scanner_create()
 -> *mut zbar_image_scanner_t {
    let mut iscn: *mut zbar_image_scanner_t =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<zbar_image_scanner_t>() as libc::c_ulong)
            as *mut zbar_image_scanner_t;
    if iscn.is_null() { return 0 as *mut zbar_image_scanner_t }
    (*iscn).dcode = zbar_decoder_create();
    (*iscn).scn = zbar_scanner_create((*iscn).dcode);
    if (*iscn).dcode.is_null() || (*iscn).scn.is_null() {
        zbar_image_scanner_destroy(iscn);
        return 0 as *mut zbar_image_scanner_t
    }
    zbar_decoder_set_userdata((*iscn).dcode, iscn as *mut libc::c_void);
    zbar_decoder_set_handler((*iscn).dcode,
                             Some(symbol_handler as
                                      unsafe extern "C" fn(_:
                                                               *mut zbar_decoder_t)
                                          -> ()));
    (*iscn).qr = _zbar_qr_create();
    /* apply default configuration */
    (*iscn).configs[(ZBAR_CFG_X_DENSITY as libc::c_int -
                         ZBAR_CFG_X_DENSITY as libc::c_int) as usize] =
        1 as libc::c_int;
    (*iscn).configs[(ZBAR_CFG_Y_DENSITY as libc::c_int -
                         ZBAR_CFG_X_DENSITY as libc::c_int) as usize] =
        1 as libc::c_int;
    zbar_image_scanner_set_config(iscn, ZBAR_NONE, ZBAR_CFG_POSITION,
                                  1 as libc::c_int);
    zbar_image_scanner_set_config(iscn, ZBAR_NONE, ZBAR_CFG_UNCERTAINTY,
                                  2 as libc::c_int);
    zbar_image_scanner_set_config(iscn, ZBAR_QRCODE, ZBAR_CFG_UNCERTAINTY,
                                  0 as libc::c_int);
    zbar_image_scanner_set_config(iscn, ZBAR_CODE128, ZBAR_CFG_UNCERTAINTY,
                                  0 as libc::c_int);
    zbar_image_scanner_set_config(iscn, ZBAR_CODE93, ZBAR_CFG_UNCERTAINTY,
                                  0 as libc::c_int);
    zbar_image_scanner_set_config(iscn, ZBAR_CODE39, ZBAR_CFG_UNCERTAINTY,
                                  0 as libc::c_int);
    zbar_image_scanner_set_config(iscn, ZBAR_CODABAR, ZBAR_CFG_UNCERTAINTY,
                                  1 as libc::c_int);
    zbar_image_scanner_set_config(iscn, ZBAR_COMPOSITE, ZBAR_CFG_UNCERTAINTY,
                                  0 as libc::c_int);
    return iscn;
}
#[inline]
unsafe extern "C" fn dump_stats(mut iscn: *const zbar_image_scanner_t) {
    let mut i: libc::c_int = 0;
    if _zbar_verbosity >= 1 as libc::c_int {
        fprintf(stderr,
                b"%s: symbol sets allocated   = %-4d\n\x00" as *const u8 as
                    *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 11],
                                          &[libc::c_char; 11]>(b"dump_stats\x00")).as_ptr(),
                (*iscn).stat_syms_new);
    }
    if _zbar_verbosity >= 1 as libc::c_int {
        fprintf(stderr,
                b"%s:     scanner syms in use = %-4d\trecycled  = %-4d\n\x00"
                    as *const u8 as *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 11],
                                          &[libc::c_char; 11]>(b"dump_stats\x00")).as_ptr(),
                (*iscn).stat_iscn_syms_inuse, (*iscn).stat_iscn_syms_recycle);
    }
    if _zbar_verbosity >= 1 as libc::c_int {
        fprintf(stderr,
                b"%s:     image syms in use   = %-4d\trecycled  = %-4d\n\x00"
                    as *const u8 as *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 11],
                                          &[libc::c_char; 11]>(b"dump_stats\x00")).as_ptr(),
                (*iscn).stat_img_syms_inuse, (*iscn).stat_img_syms_recycle);
    }
    if _zbar_verbosity >= 1 as libc::c_int {
        fprintf(stderr,
                b"%s: symbols allocated       = %-4d\n\x00" as *const u8 as
                    *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 11],
                                          &[libc::c_char; 11]>(b"dump_stats\x00")).as_ptr(),
                (*iscn).stat_sym_new);
    }
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        if _zbar_verbosity >= 1 as libc::c_int {
            fprintf(stderr,
                    b"%s:      recycled[%d]        = %-4d\n\x00" as *const u8
                        as *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 11],
                                              &[libc::c_char; 11]>(b"dump_stats\x00")).as_ptr(),
                    i, (*iscn).stat_sym_recycle[i as usize]);
        }
        i += 1
    };
}
/* * destructor. */
#[no_mangle]
pub unsafe extern "C" fn zbar_image_scanner_destroy(mut iscn:
                                                        *mut zbar_image_scanner_t) {
    let mut i: libc::c_int = 0;
    dump_stats(iscn);
    if !(*iscn).syms.is_null() {
        if (*(*iscn).syms).refcnt != 0 {
            zbar_symbol_set_ref((*iscn).syms, -(1 as libc::c_int));
        } else { _zbar_symbol_set_free((*iscn).syms); }
        (*iscn).syms = 0 as *mut zbar_symbol_set_t
    }
    if !(*iscn).scn.is_null() { zbar_scanner_destroy((*iscn).scn); }
    (*iscn).scn = 0 as *mut zbar_scanner_t;
    if !(*iscn).dcode.is_null() { zbar_decoder_destroy((*iscn).dcode); }
    (*iscn).dcode = 0 as *mut zbar_decoder_t;
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        let mut sym: *mut zbar_symbol_t = 0 as *mut zbar_symbol_t;
        let mut next: *mut zbar_symbol_t = 0 as *mut zbar_symbol_t;
        sym = (*iscn).recycle[i as usize].head;
        while !sym.is_null() {
            next = (*sym).next;
            _zbar_symbol_free(sym);
            sym = next
        }
        i += 1
    }
    if !(*iscn).qr.is_null() {
        _zbar_qr_destroy((*iscn).qr);
        (*iscn).qr = 0 as *mut qr_reader
    }
    free(iscn as *mut libc::c_void);
}
/* * setup result handler callback.
 * the specified function will be called by the scanner whenever
 * new results are available from a decoded image.
 * pass a NULL value to disable callbacks.
 * @returns the previously registered handler
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_image_scanner_set_data_handler(mut iscn:
                                                                 *mut zbar_image_scanner_t,
                                                             mut handler:
                                                                 Option<zbar_image_data_handler_t>,
                                                             mut userdata:
                                                                 *const libc::c_void)
 -> Option<zbar_image_data_handler_t> {
    let mut result: Option<zbar_image_data_handler_t> = (*iscn).handler;
    (*iscn).handler = handler;
    (*iscn).userdata = userdata;
    return result;
}
/* * set config for indicated symbology (0 for all) to specified value.
 * @returns 0 for success, non-0 for failure (config does not apply to
 * specified symbology, or value out of range)
 * @see zbar_decoder_set_config()
 * @since 0.4
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_image_scanner_set_config(mut iscn:
                                                           *mut zbar_image_scanner_t,
                                                       mut sym:
                                                           zbar_symbol_type_t,
                                                       mut cfg: zbar_config_t,
                                                       mut val: libc::c_int)
 -> libc::c_int {
    if (sym as libc::c_uint == 0 as libc::c_int as libc::c_uint ||
            sym as libc::c_uint ==
                ZBAR_COMPOSITE as libc::c_int as libc::c_uint) &&
           cfg as libc::c_uint ==
               ZBAR_CFG_ENABLE as libc::c_int as libc::c_uint {
        (*iscn).ean_config = (val != 0) as libc::c_int as libc::c_uint;
        if sym as u64 != 0 { return 0 as libc::c_int }
    }
    if (cfg as libc::c_uint) <
           ZBAR_CFG_UNCERTAINTY as libc::c_int as libc::c_uint {
        return zbar_decoder_set_config((*iscn).dcode, sym, cfg, val)
    }
    if (cfg as libc::c_uint) <
           ZBAR_CFG_POSITION as libc::c_int as libc::c_uint {
        let mut c: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        if cfg as libc::c_uint >
               ZBAR_CFG_UNCERTAINTY as libc::c_int as libc::c_uint {
            return 1 as libc::c_int
        }
        c =
            (cfg as
                 libc::c_uint).wrapping_sub(ZBAR_CFG_UNCERTAINTY as
                                                libc::c_int as libc::c_uint)
                as libc::c_int;
        if sym as libc::c_uint > ZBAR_PARTIAL as libc::c_int as libc::c_uint {
            i = _zbar_get_symbol_hash(sym);
            (*iscn).sym_configs[c as usize][i as usize] = val
        } else {
            i = 0 as libc::c_int;
            while i < 20 as libc::c_int {
                (*iscn).sym_configs[c as usize][i as usize] = val;
                i += 1
            }
        }
        return 0 as libc::c_int
    }
    if sym as libc::c_uint > ZBAR_PARTIAL as libc::c_int as libc::c_uint {
        return 1 as libc::c_int
    }
    if cfg as libc::c_uint >=
           ZBAR_CFG_X_DENSITY as libc::c_int as libc::c_uint &&
           cfg as libc::c_uint <=
               ZBAR_CFG_Y_DENSITY as libc::c_int as libc::c_uint {
        (*iscn).configs[(cfg as
                             libc::c_uint).wrapping_sub(ZBAR_CFG_X_DENSITY as
                                                            libc::c_int as
                                                            libc::c_uint) as
                            usize] = val;
        return 0 as libc::c_int
    }
    if cfg as libc::c_uint > ZBAR_CFG_POSITION as libc::c_int as libc::c_uint
       {
        return 1 as libc::c_int
    }
    cfg =
        ::std::mem::transmute::<libc::c_uint,
                                zbar_config_t>((cfg as
                                                    libc::c_uint).wrapping_sub(ZBAR_CFG_POSITION
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint))
            as zbar_config_t;
    if val == 0 {
        (*iscn).config &=
            !((1 as libc::c_int) << cfg as libc::c_uint) as libc::c_uint
    } else if val == 1 as libc::c_int {
        (*iscn).config |=
            ((1 as libc::c_int) << cfg as libc::c_uint) as libc::c_uint
    } else { return 1 as libc::c_int }
    return 0 as libc::c_int;
}
/* * enable or disable the inter-image result cache (default disabled).
 * mostly useful for scanning video frames, the cache filters
 * duplicate results from consecutive images, while adding some
 * consistency checking and hysteresis to the results.
 * this interface also clears the cache
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_image_scanner_enable_cache(mut iscn:
                                                             *mut zbar_image_scanner_t,
                                                         mut enable:
                                                             libc::c_int) {
    if !(*iscn).cache.is_null() {
        /* recycle all cached syms */
        _zbar_image_scanner_recycle_syms(iscn, (*iscn).cache);
        (*iscn).cache = 0 as *mut zbar_symbol_t
    }
    (*iscn).enable_cache =
        if enable != 0 { 1 as libc::c_int } else { 0 as libc::c_int };
}
/* * retrieve decode results for last scanned image.
 * @returns the symbol set result container or NULL if no results are
 * available
 * @note the symbol set does not have its reference count adjusted;
 * ensure that the count is incremented if the results may be kept
 * after the next image is scanned
 * @since 0.10
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_image_scanner_get_results(mut iscn:
                                                            *const zbar_image_scanner_t)
 -> *const zbar_symbol_set_t {
    return (*iscn).syms;
}
#[inline]
unsafe extern "C" fn quiet_border(mut iscn: *mut zbar_image_scanner_t) {
    /* flush scanner pipeline */
    let mut scn: *mut zbar_scanner_t = (*iscn).scn;
    zbar_scanner_flush(scn);
    zbar_scanner_flush(scn);
    zbar_scanner_new_scan(scn);
}
/* * scan for symbols in provided image.  The image format must be
 * "Y800" or "GRAY".
 * @returns >0 if symbols were successfully decoded from the image,
 * 0 if no symbols were found or -1 if an error occurs
 * @see zbar_image_convert()
 * @since 0.9 - changed to only accept grayscale images
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_scan_image(mut iscn: *mut zbar_image_scanner_t,
                                         mut img: *mut zbar_image_t)
 -> libc::c_int {
    let mut syms: *mut zbar_symbol_set_t = 0 as *mut zbar_symbol_set_t;
    let mut data: *const uint8_t = 0 as *const uint8_t;
    let mut scn: *mut zbar_scanner_t = (*iscn).scn;
    let mut w: libc::c_uint = 0;
    let mut h: libc::c_uint = 0;
    let mut cx1: libc::c_uint = 0;
    let mut cy1: libc::c_uint = 0;
    let mut density: libc::c_int = 0;
    /* timestamp image
     * FIXME prefer video timestamp
     */
    (*iscn).time = _zbar_timer_now() as libc::c_ulong;
    _zbar_qr_reset((*iscn).qr);
    /* image must be in grayscale format */
    if (*img).format as libc::c_ulong !=
           'Y' as i32 as libc::c_ulong |
               ('8' as i32 as libc::c_ulong) << 8 as libc::c_int |
               ('0' as i32 as libc::c_ulong) << 16 as libc::c_int |
               ('0' as i32 as libc::c_ulong) << 24 as libc::c_int &&
           (*img).format as libc::c_ulong !=
               'G' as i32 as libc::c_ulong |
                   ('R' as i32 as libc::c_ulong) << 8 as libc::c_int |
                   ('E' as i32 as libc::c_ulong) << 16 as libc::c_int |
                   ('Y' as i32 as libc::c_ulong) << 24 as libc::c_int {
        return -(1 as libc::c_int)
    }
    (*iscn).img = img;
    /* recycle previous scanner and image results */
    zbar_image_scanner_recycle_image(iscn, img);
    syms = (*iscn).syms;
    if syms.is_null() {
        (*iscn).syms = _zbar_symbol_set_create();
        syms = (*iscn).syms;
        (*iscn).stat_syms_new += 1;
        zbar_symbol_set_ref(syms, 1 as libc::c_int);
    } else { zbar_symbol_set_ref(syms, 2 as libc::c_int); }
    (*img).syms = syms;
    w = (*img).width;
    h = (*img).height;
    cx1 = (*img).crop_x.wrapping_add((*img).crop_w);
    if cx1 <= w {
    } else {
        __assert_fail(b"cx1 <= w\x00" as *const u8 as *const libc::c_char,
                      b"zbar/img_scanner.c\x00" as *const u8 as
                          *const libc::c_char,
                      683 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 60],
                                                &[libc::c_char; 60]>(b"int zbar_scan_image(zbar_image_scanner_t *, zbar_image_t *)\x00")).as_ptr());
    }
    cy1 = (*img).crop_y.wrapping_add((*img).crop_h);
    if cy1 <= h {
    } else {
        __assert_fail(b"cy1 <= h\x00" as *const u8 as *const libc::c_char,
                      b"zbar/img_scanner.c\x00" as *const u8 as
                          *const libc::c_char,
                      685 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 60],
                                                &[libc::c_char; 60]>(b"int zbar_scan_image(zbar_image_scanner_t *, zbar_image_t *)\x00")).as_ptr());
    }
    data = (*img).data as *const uint8_t;
    zbar_scanner_new_scan(scn);
    density =
        (*iscn).configs[(ZBAR_CFG_Y_DENSITY as libc::c_int -
                             ZBAR_CFG_X_DENSITY as libc::c_int) as usize];
    if density > 0 as libc::c_int {
        let mut p: *const uint8_t = data;
        let mut x: libc::c_int = 0 as libc::c_int;
        let mut y: libc::c_int = 0 as libc::c_int;
        let mut border: libc::c_int =
            (*img).crop_h.wrapping_sub(1 as libc::c_int as
                                           libc::c_uint).wrapping_rem(density
                                                                          as
                                                                          libc::c_uint).wrapping_add(1
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_uint).wrapping_div(2
                                                                                                                                        as
                                                                                                                                        libc::c_int
                                                                                                                                        as
                                                                                                                                        libc::c_uint)
                as libc::c_int;
        if border as libc::c_uint >
               (*img).crop_h.wrapping_div(2 as libc::c_int as libc::c_uint) {
            border =
                (*img).crop_h.wrapping_div(2 as libc::c_int as libc::c_uint)
                    as libc::c_int
        }
        border =
            (border as libc::c_uint).wrapping_add((*img).crop_y) as
                libc::c_int as libc::c_int;
        if border as libc::c_uint <= h {
        } else {
            __assert_fail(b"border <= h\x00" as *const u8 as
                              *const libc::c_char,
                          b"zbar/img_scanner.c\x00" as *const u8 as
                              *const libc::c_char,
                          703 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 60],
                                                    &[libc::c_char; 60]>(b"int zbar_scan_image(zbar_image_scanner_t *, zbar_image_t *)\x00")).as_ptr());
        }
        (*iscn).dy = 0 as libc::c_int;
        x =
            (x as libc::c_uint).wrapping_add((*img).crop_x) as libc::c_int as
                libc::c_int;
        y += border;
        p =
            p.offset(((*img).crop_x as
                          libc::c_ulong).wrapping_add((border as
                                                           uintptr_t).wrapping_mul(w
                                                                                       as
                                                                                       libc::c_ulong))
                         as isize);
        (*iscn).v = y;
        while (y as libc::c_uint) < cy1 {
            let mut cx0: libc::c_int = (*img).crop_x as libc::c_int;
            if _zbar_verbosity >= 128 as libc::c_int {
                fprintf(stderr,
                        b"%s: img_x+: %04d,%04d @%p\n\x00" as *const u8 as
                            *const libc::c_char,
                        (*::std::mem::transmute::<&[u8; 16],
                                                  &[libc::c_char; 16]>(b"zbar_scan_image\x00")).as_ptr(),
                        x, y, p);
            }
            (*iscn).du = 1 as libc::c_int;
            (*iscn).dx = (*iscn).du;
            (*iscn).umin = cx0;
            while (x as libc::c_uint) < cx1 {
                let mut d: uint8_t = *p;
                x += 1 as libc::c_int;
                y += 0 as libc::c_int;
                p =
                    p.offset((1 as libc::c_int as
                                  libc::c_ulong).wrapping_add((0 as
                                                                   libc::c_int
                                                                   as
                                                                   uintptr_t).wrapping_mul(w
                                                                                               as
                                                                                               libc::c_ulong))
                                 as isize);
                zbar_scan_y(scn, d as libc::c_int);
            }
            if p ==
                   data.offset(x as
                                   isize).offset((y as libc::c_long *
                                                      w as intptr_t) as isize)
               {
            } else {
                __assert_fail(b"p == data + x + y * (intptr_t)w\x00" as
                                  *const u8 as *const libc::c_char,
                              b"zbar/img_scanner.c\x00" as *const u8 as
                                  *const libc::c_char,
                              721 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 60],
                                                        &[libc::c_char; 60]>(b"int zbar_scan_image(zbar_image_scanner_t *, zbar_image_t *)\x00")).as_ptr());
            }
            quiet_border(iscn);
            x += -(1 as libc::c_int);
            y += density;
            p =
                p.offset((-(1 as libc::c_int) as
                              libc::c_ulong).wrapping_add((density as
                                                               uintptr_t).wrapping_mul(w
                                                                                           as
                                                                                           libc::c_ulong))
                             as isize);
            (*iscn).v = y;
            if y as libc::c_uint >= cy1 { break ; }
            if _zbar_verbosity >= 128 as libc::c_int {
                fprintf(stderr,
                        b"%s: img_x-: %04d,%04d @%p\n\x00" as *const u8 as
                            *const libc::c_char,
                        (*::std::mem::transmute::<&[u8; 16],
                                                  &[libc::c_char; 16]>(b"zbar_scan_image\x00")).as_ptr(),
                        x, y, p);
            }
            (*iscn).du = -(1 as libc::c_int);
            (*iscn).dx = (*iscn).du;
            (*iscn).umin = cx1 as libc::c_int;
            while x >= cx0 {
                let mut d_0: uint8_t = *p;
                x += -(1 as libc::c_int);
                y += 0 as libc::c_int;
                p =
                    p.offset((-(1 as libc::c_int) as
                                  libc::c_ulong).wrapping_add((0 as
                                                                   libc::c_int
                                                                   as
                                                                   uintptr_t).wrapping_mul(w
                                                                                               as
                                                                                               libc::c_ulong))
                                 as isize);
                zbar_scan_y(scn, d_0 as libc::c_int);
            }
            if p ==
                   data.offset(x as
                                   isize).offset((y as libc::c_long *
                                                      w as intptr_t) as isize)
               {
            } else {
                __assert_fail(b"p == data + x + y * (intptr_t)w\x00" as
                                  *const u8 as *const libc::c_char,
                              b"zbar/img_scanner.c\x00" as *const u8 as
                                  *const libc::c_char,
                              739 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 60],
                                                        &[libc::c_char; 60]>(b"int zbar_scan_image(zbar_image_scanner_t *, zbar_image_t *)\x00")).as_ptr());
            }
            quiet_border(iscn);
            x += 1 as libc::c_int;
            y += density;
            p =
                p.offset((1 as libc::c_int as
                              libc::c_ulong).wrapping_add((density as
                                                               uintptr_t).wrapping_mul(w
                                                                                           as
                                                                                           libc::c_ulong))
                             as isize);
            (*iscn).v = y
        }
    }
    (*iscn).dx = 0 as libc::c_int;
    density =
        (*iscn).configs[(ZBAR_CFG_X_DENSITY as libc::c_int -
                             ZBAR_CFG_X_DENSITY as libc::c_int) as usize];
    if density > 0 as libc::c_int {
        let mut p_0: *const uint8_t = data;
        let mut x_0: libc::c_int = 0 as libc::c_int;
        let mut y_0: libc::c_int = 0 as libc::c_int;
        let mut border_0: libc::c_int =
            (*img).crop_w.wrapping_sub(1 as libc::c_int as
                                           libc::c_uint).wrapping_rem(density
                                                                          as
                                                                          libc::c_uint).wrapping_add(1
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_uint).wrapping_div(2
                                                                                                                                        as
                                                                                                                                        libc::c_int
                                                                                                                                        as
                                                                                                                                        libc::c_uint)
                as libc::c_int;
        if border_0 as libc::c_uint >
               (*img).crop_w.wrapping_div(2 as libc::c_int as libc::c_uint) {
            border_0 =
                (*img).crop_w.wrapping_div(2 as libc::c_int as libc::c_uint)
                    as libc::c_int
        }
        border_0 =
            (border_0 as libc::c_uint).wrapping_add((*img).crop_x) as
                libc::c_int as libc::c_int;
        if border_0 as libc::c_uint <= w {
        } else {
            __assert_fail(b"border <= w\x00" as *const u8 as
                              *const libc::c_char,
                          b"zbar/img_scanner.c\x00" as *const u8 as
                              *const libc::c_char,
                          759 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 60],
                                                    &[libc::c_char; 60]>(b"int zbar_scan_image(zbar_image_scanner_t *, zbar_image_t *)\x00")).as_ptr());
        }
        x_0 += border_0;
        y_0 =
            (y_0 as libc::c_uint).wrapping_add((*img).crop_y) as libc::c_int
                as libc::c_int;
        p_0 =
            p_0.offset((border_0 as
                            libc::c_ulong).wrapping_add(((*img).crop_y as
                                                             uintptr_t).wrapping_mul(w
                                                                                         as
                                                                                         libc::c_ulong))
                           as isize);
        (*iscn).v = x_0;
        while (x_0 as libc::c_uint) < cx1 {
            let mut cy0: libc::c_int = (*img).crop_y as libc::c_int;
            if _zbar_verbosity >= 128 as libc::c_int {
                fprintf(stderr,
                        b"%s: img_y+: %04d,%04d @%p\n\x00" as *const u8 as
                            *const libc::c_char,
                        (*::std::mem::transmute::<&[u8; 16],
                                                  &[libc::c_char; 16]>(b"zbar_scan_image\x00")).as_ptr(),
                        x_0, y_0, p_0);
            }
            (*iscn).du = 1 as libc::c_int;
            (*iscn).dy = (*iscn).du;
            (*iscn).umin = cy0;
            while (y_0 as libc::c_uint) < cy1 {
                let mut d_1: uint8_t = *p_0;
                x_0 += 0 as libc::c_int;
                y_0 += 1 as libc::c_int;
                p_0 =
                    p_0.offset((0 as libc::c_int as
                                    libc::c_ulong).wrapping_add((1 as
                                                                     libc::c_int
                                                                     as
                                                                     uintptr_t).wrapping_mul(w
                                                                                                 as
                                                                                                 libc::c_ulong))
                                   as isize);
                zbar_scan_y(scn, d_1 as libc::c_int);
            }
            if p_0 ==
                   data.offset(x_0 as
                                   isize).offset((y_0 as libc::c_long *
                                                      w as intptr_t) as isize)
               {
            } else {
                __assert_fail(b"p == data + x + y * (intptr_t)w\x00" as
                                  *const u8 as *const libc::c_char,
                              b"zbar/img_scanner.c\x00" as *const u8 as
                                  *const libc::c_char,
                              775 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 60],
                                                        &[libc::c_char; 60]>(b"int zbar_scan_image(zbar_image_scanner_t *, zbar_image_t *)\x00")).as_ptr());
            }
            quiet_border(iscn);
            x_0 += density;
            y_0 += -(1 as libc::c_int);
            p_0 =
                p_0.offset((density as
                                libc::c_ulong).wrapping_add((-(1 as
                                                                   libc::c_int)
                                                                 as
                                                                 uintptr_t).wrapping_mul(w
                                                                                             as
                                                                                             libc::c_ulong))
                               as isize);
            (*iscn).v = x_0;
            if x_0 as libc::c_uint >= cx1 { break ; }
            if _zbar_verbosity >= 128 as libc::c_int {
                fprintf(stderr,
                        b"%s: img_y-: %04d,%04d @%p\n\x00" as *const u8 as
                            *const libc::c_char,
                        (*::std::mem::transmute::<&[u8; 16],
                                                  &[libc::c_char; 16]>(b"zbar_scan_image\x00")).as_ptr(),
                        x_0, y_0, p_0);
            }
            (*iscn).du = -(1 as libc::c_int);
            (*iscn).dy = (*iscn).du;
            (*iscn).umin = cy1 as libc::c_int;
            while y_0 >= cy0 {
                let mut d_2: uint8_t = *p_0;
                x_0 += 0 as libc::c_int;
                y_0 += -(1 as libc::c_int);
                p_0 =
                    p_0.offset((0 as libc::c_int as
                                    libc::c_ulong).wrapping_add((-(1 as
                                                                       libc::c_int)
                                                                     as
                                                                     uintptr_t).wrapping_mul(w
                                                                                                 as
                                                                                                 libc::c_ulong))
                                   as isize);
                zbar_scan_y(scn, d_2 as libc::c_int);
            }
            if p_0 ==
                   data.offset(x_0 as
                                   isize).offset((y_0 as libc::c_long *
                                                      w as intptr_t) as isize)
               {
            } else {
                __assert_fail(b"p == data + x + y * (intptr_t)w\x00" as
                                  *const u8 as *const libc::c_char,
                              b"zbar/img_scanner.c\x00" as *const u8 as
                                  *const libc::c_char,
                              793 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 60],
                                                        &[libc::c_char; 60]>(b"int zbar_scan_image(zbar_image_scanner_t *, zbar_image_t *)\x00")).as_ptr());
            }
            quiet_border(iscn);
            x_0 += density;
            y_0 += 1 as libc::c_int;
            p_0 =
                p_0.offset((density as
                                libc::c_ulong).wrapping_add((1 as libc::c_int
                                                                 as
                                                                 uintptr_t).wrapping_mul(w
                                                                                             as
                                                                                             libc::c_ulong))
                               as isize);
            (*iscn).v = x_0
        }
    }
    (*iscn).dy = 0 as libc::c_int;
    (*iscn).img = 0 as *mut zbar_image_t;
    _zbar_qr_decode((*iscn).qr, iscn, img);
    /* FIXME tmp hack to filter bad EAN results */
    /* FIXME tmp hack to merge simple case EAN add-ons */
    let mut filter: libc::c_char =
        ((*iscn).enable_cache == 0 &&
             (density == 1 as libc::c_int ||
                  (*iscn).configs[(ZBAR_CFG_Y_DENSITY as libc::c_int -
                                       ZBAR_CFG_X_DENSITY as libc::c_int) as
                                      usize] == 1 as libc::c_int)) as
            libc::c_int as libc::c_char;
    let mut nean: libc::c_int = 0 as libc::c_int;
    let mut naddon: libc::c_int = 0 as libc::c_int;
    if (*syms).nsyms != 0 {
        let mut symp: *mut *mut zbar_symbol_t = 0 as *mut *mut zbar_symbol_t;
        symp = &mut (*syms).head;
        while !(*symp).is_null() {
            let mut sym: *mut zbar_symbol_t = *symp;
            if (*sym).cache_count <= 0 as libc::c_int &&
                   (((*sym).type_0 as libc::c_uint) <
                        ZBAR_COMPOSITE as libc::c_int as libc::c_uint &&
                        (*sym).type_0 as libc::c_uint >
                            ZBAR_PARTIAL as libc::c_int as libc::c_uint ||
                        (*sym).type_0 as libc::c_uint ==
                            ZBAR_DATABAR as libc::c_int as libc::c_uint ||
                        (*sym).type_0 as libc::c_uint ==
                            ZBAR_DATABAR_EXP as libc::c_int as libc::c_uint ||
                        (*sym).type_0 as libc::c_uint ==
                            ZBAR_CODABAR as libc::c_int as libc::c_uint) {
                if ((*sym).type_0 as libc::c_uint ==
                        ZBAR_CODABAR as libc::c_int as libc::c_uint ||
                        filter as libc::c_int != 0) &&
                       (*sym).quality < 4 as libc::c_int {
                    if (*iscn).enable_cache != 0 {
                        /* revert cache update */
                        let mut entry: *mut zbar_symbol_t =
                            cache_lookup(iscn, sym);
                        if !entry.is_null() {
                            (*entry).cache_count -= 1
                        } else {
                            __assert_fail(b"0\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"zbar/img_scanner.c\x00" as
                                              *const u8 as
                                              *const libc::c_char,
                                          831 as libc::c_int as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 60],
                                                                    &[libc::c_char; 60]>(b"int zbar_scan_image(zbar_image_scanner_t *, zbar_image_t *)\x00")).as_ptr());
                        }
                    }
                    /* recycle */
                    *symp = (*sym).next;
                    (*syms).nsyms -= 1;
                    (*sym).next = 0 as *mut zbar_symbol_t;
                    _zbar_image_scanner_recycle_syms(iscn, sym);
                    continue ;
                } else if ((*sym).type_0 as libc::c_uint) <
                              ZBAR_COMPOSITE as libc::c_int as libc::c_uint &&
                              (*sym).type_0 as libc::c_uint !=
                                  ZBAR_ISBN10 as libc::c_int as libc::c_uint {
                    if (*sym).type_0 as libc::c_uint >
                           ZBAR_EAN5 as libc::c_int as libc::c_uint {
                        nean += 1
                    } else { naddon += 1 }
                }
            }
            symp = &mut (*sym).next
        }
        if nean == 1 as libc::c_int && naddon == 1 as libc::c_int &&
               (*iscn).ean_config != 0 {
            /* create container symbol for composite result */
            let mut ean: *mut zbar_symbol_t = 0 as *mut zbar_symbol_t;
            let mut addon: *mut zbar_symbol_t = 0 as *mut zbar_symbol_t;
            symp = &mut (*syms).head;
            while !(*symp).is_null() {
                let mut sym_0: *mut zbar_symbol_t = *symp;
                if ((*sym_0).type_0 as libc::c_uint) <
                       ZBAR_COMPOSITE as libc::c_int as libc::c_uint &&
                       (*sym_0).type_0 as libc::c_uint >
                           ZBAR_PARTIAL as libc::c_int as libc::c_uint {
                    /* move to composite */
                    *symp = (*sym_0).next;
                    (*syms).nsyms -= 1;
                    (*sym_0).next = 0 as *mut zbar_symbol_t;
                    if (*sym_0).type_0 as libc::c_uint <=
                           ZBAR_EAN5 as libc::c_int as libc::c_uint {
                        addon = sym_0
                    } else { ean = sym_0 }
                } else { symp = &mut (*sym_0).next }
            }
            if !ean.is_null() {
            } else {
                __assert_fail(b"ean\x00" as *const u8 as *const libc::c_char,
                              b"zbar/img_scanner.c\x00" as *const u8 as
                                  *const libc::c_char,
                              871 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 60],
                                                        &[libc::c_char; 60]>(b"int zbar_scan_image(zbar_image_scanner_t *, zbar_image_t *)\x00")).as_ptr());
            }
            if !addon.is_null() {
            } else {
                __assert_fail(b"addon\x00" as *const u8 as
                                  *const libc::c_char,
                              b"zbar/img_scanner.c\x00" as *const u8 as
                                  *const libc::c_char,
                              872 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 60],
                                                        &[libc::c_char; 60]>(b"int zbar_scan_image(zbar_image_scanner_t *, zbar_image_t *)\x00")).as_ptr());
            }
            let mut datalen: libc::c_int =
                (*ean).datalen.wrapping_add((*addon).datalen).wrapping_add(1
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint)
                    as libc::c_int;
            let mut ean_sym: *mut zbar_symbol_t =
                _zbar_image_scanner_alloc_sym(iscn, ZBAR_COMPOSITE, datalen);
            (*ean_sym).orient = (*ean).orient;
            (*ean_sym).syms = _zbar_symbol_set_create();
            memcpy((*ean_sym).data as *mut libc::c_void,
                   (*ean).data as *const libc::c_void,
                   (*ean).datalen as libc::c_ulong);
            memcpy((*ean_sym).data.offset((*ean).datalen as isize) as
                       *mut libc::c_void,
                   (*addon).data as *const libc::c_void,
                   (*addon).datalen.wrapping_add(1 as libc::c_int as
                                                     libc::c_uint) as
                       libc::c_ulong);
            (*(*ean_sym).syms).head = ean;
            (*ean).next = addon;
            (*(*ean_sym).syms).nsyms = 2 as libc::c_int;
            _zbar_image_scanner_add_sym(iscn, ean_sym);
        }
    }
    if (*syms).nsyms != 0 && (*iscn).handler.is_some() {
        (*iscn).handler.expect("non-null function pointer")(img,
                                                            (*iscn).userdata);
    }
    return (*syms).nsyms;
}
