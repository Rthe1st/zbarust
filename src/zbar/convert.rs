use ::c2rust_bitfields;
use ::libc;
extern {
    pub type video_state_s;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type window_state_s;
    /* * @internal type unsafe error API (don't use) */
    #[no_mangle]
    fn _zbar_error_spew(object: *const libc::c_void, verbosity: libc::c_int) -> libc::c_int;
    /* * new image constructor.
     * @returns a new image object with uninitialized data and format.
     * this image should be destroyed (using zbar_image_destroy()) as
     * soon as the application is finished with it
     */
    #[no_mangle]
    fn zbar_image_create() -> *mut zbar_image_t;
    /* * image destructor.  all images created by or returned to the
     * application should be destroyed using this function.  when an image
     * is destroyed, the associated data cleanup handler will be invoked
     * if available
     * @note make no assumptions about the image or the data buffer.
     * they may not be destroyed/cleaned immediately if the library
     * is still using them.  if necessary, use the cleanup handler hook
     * to keep track of image data buffers
     */
    #[no_mangle]
    fn zbar_image_destroy(image: *mut zbar_image_t);
    /* * specify a rectangular region of the image to scan.
     * the rectangle will be clipped to the image boundaries.
     * defaults to the full image specified by zbar_image_set_size()
     */
    #[no_mangle]
    fn zbar_image_set_crop(
        image: *mut zbar_image_t,
        x: libc::c_uint,
        y: libc::c_uint,
        width: libc::c_uint,
        height: libc::c_uint,
    );
    /* * built-in cleanup handler.
     * passes the image data buffer to free()
     */
    #[no_mangle]
    fn zbar_image_free_data(image: *mut zbar_image_t);
    /* * initialize video using a specific format for debug.
     * use zbar_negotiate_format() to automatically select and initialize
     * the best available format
     */
    #[no_mangle]
    fn zbar_video_init(video: *mut zbar_video_t, format: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut _zbar_verbosity: libc::c_int;
    #[no_mangle]
    fn _zbar_image_free(_: *mut zbar_image_t);
    #[no_mangle]
    fn _zbar_refcnt(cnt: *mut refcnt_t, delta: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
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
pub type zbar_error_e = libc::c_uint;
pub const ZBAR_ERR_NUM: zbar_error_e = 12;
pub const ZBAR_ERR_WINAPI: zbar_error_e = 11;
pub const ZBAR_ERR_CLOSED: zbar_error_e = 10;
pub const ZBAR_ERR_XPROTO: zbar_error_e = 9;
pub const ZBAR_ERR_XDISPLAY: zbar_error_e = 8;
pub const ZBAR_ERR_BUSY: zbar_error_e = 7;
pub const ZBAR_ERR_LOCKING: zbar_error_e = 6;
pub const ZBAR_ERR_SYSTEM: zbar_error_e = 5;
pub const ZBAR_ERR_INVALID: zbar_error_e = 4;
pub const ZBAR_ERR_UNSUPPORTED: zbar_error_e = 3;
pub const ZBAR_ERR_INTERNAL: zbar_error_e = 2;
pub const ZBAR_ERR_NOMEM: zbar_error_e = 1;
pub const ZBAR_OK: zbar_error_e = 0;
/* *< unable to determine orientation */
/* *< upright, read left to right */
/* *< sideways, read top to bottom */
/* *< upside-down, read right to left */
/* *< sideways, read bottom to top */
/* * error codes. */
pub type zbar_error_t = zbar_error_e;
pub type video_control_type_e = libc::c_uint;
pub const VIDEO_CNTL_BOOLEAN: video_control_type_e = 6;
pub const VIDEO_CNTL_STRING: video_control_type_e = 5;
pub const VIDEO_CNTL_INTEGER64: video_control_type_e = 4;
pub const VIDEO_CNTL_BUTTON: video_control_type_e = 3;
pub const VIDEO_CNTL_MENU: video_control_type_e = 2;
pub const VIDEO_CNTL_INTEGER: video_control_type_e = 1;
pub type video_control_type_t = video_control_type_e;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct video_control_menu_s {
    pub name: *mut libc::c_char,
    pub value: int64_t,
}
/* *< no error */
/* *< out of memory */
/* *< internal library error */
/* *< unsupported request */
/* *< invalid request */
/* *< system error */
/* *< locking error */
/* *< all resources busy */
/* *< X11 display error */
/* *< X11 protocol error */
/* *< output window is closed */
/* *< windows system error */
/* *< number of error codes */
/* * store video control menu
 * @param name name of the menu item
 * @param val integer value associated with the item
 */
pub type video_control_menu_t = video_control_menu_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct video_controls_s {
    pub name: *mut libc::c_char,
    pub group: *mut libc::c_char,
    pub type_0: video_control_type_t,
    pub min: int64_t,
    pub max: int64_t,
    pub def: int64_t,
    pub step: uint64_t,
    pub menu_size: libc::c_uint,
    pub menu: *mut video_control_menu_t,
    pub next: *mut libc::c_void,
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
/* @} */
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
/* @} */
/* ------------------------------------------------------------ */
/* * @name Image interface
 * stores image data samples along with associated format and size
 * metadata
 */
/* @{ */
/* * opaque image object. */
pub type zbar_image_t = zbar_image_s;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct zbar_video_s {
    pub err: errinfo_t,
    pub fd: libc::c_int,
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub intf: video_interface_t,
    pub iomode: video_iomode_t,
    #[bitfield(name = "initialized", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "active", ty = "libc::c_uint", bits = "1..=1")]
    pub initialized_active: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub format: uint32_t,
    pub palette: libc::c_uint,
    pub formats: *mut uint32_t,
    pub emu_formats: *mut uint32_t,
    pub controls: *mut video_controls_s,
    pub datalen: libc::c_ulong,
    pub buflen: libc::c_ulong,
    pub buf: *mut libc::c_void,
    pub frame: libc::c_uint,
    pub qlock: zbar_mutex_t,
    pub num_images: libc::c_int,
    pub images: *mut *mut zbar_image_t,
    pub nq_image: *mut zbar_image_t,
    pub dq_image: *mut zbar_image_t,
    pub shadow_image: *mut zbar_image_t,
    pub state: *mut video_state_t,
    pub init: Option<unsafe extern fn(_: *mut zbar_video_t, _: uint32_t) -> libc::c_int>,
    pub cleanup: Option<unsafe extern fn(_: *mut zbar_video_t) -> libc::c_int>,
    pub start: Option<unsafe extern fn(_: *mut zbar_video_t) -> libc::c_int>,
    pub stop: Option<unsafe extern fn(_: *mut zbar_video_t) -> libc::c_int>,
    pub nq: Option<unsafe extern fn(_: *mut zbar_video_t, _: *mut zbar_image_t) -> libc::c_int>,
    pub set_control: Option<
        unsafe extern fn(
            _: *mut zbar_video_t,
            _: *const libc::c_char,
            _: *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub get_control: Option<
        unsafe extern fn(
            _: *mut zbar_video_t,
            _: *const libc::c_char,
            _: *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub free: Option<unsafe extern fn(_: *mut zbar_video_t) -> ()>,
    pub dq: Option<unsafe extern fn(_: *mut zbar_video_t) -> *mut zbar_image_t>,
}
pub type video_state_t = video_state_s;
pub type zbar_mutex_t = pthread_mutex_t;
pub type video_iomode_t = video_iomode_e;
pub type video_iomode_e = libc::c_uint;
pub const VIDEO_USERPTR: video_iomode_e = 3;
pub const VIDEO_MMAP: video_iomode_e = 2;
pub const VIDEO_READWRITE: video_iomode_e = 1;
pub type video_interface_t = video_interface_e;
pub type video_interface_e = libc::c_uint;
pub const VIDEO_VFW: video_interface_e = 3;
pub const VIDEO_V4L2: video_interface_e = 2;
pub const VIDEO_V4L1: video_interface_e = 1;
pub const VIDEO_INVALID: video_interface_e = 0;
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
/* "zERR" (LE) */
/* application must terminate */
/* might be able to recover and continue */
/* unexpected condition */
/* fyi */
pub type errinfo_t = errinfo_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct errinfo_s {
    pub magic: uint32_t,
    pub module: errmodule_t,
    pub buf: *mut libc::c_char,
    pub errnum: libc::c_int,
    pub sev: errsev_t,
    pub type_0: zbar_error_t,
    pub func: *const libc::c_char,
    pub detail: *const libc::c_char,
    pub arg_str: *mut libc::c_char,
    pub arg_int: libc::c_int,
}
pub type errsev_t = errsev_e;
pub type errsev_e = libc::c_int;
pub const SEV_NOTE: errsev_e = 2;
pub const SEV_WARNING: errsev_e = 1;
pub const SEV_OK: errsev_e = 0;
pub const SEV_ERROR: errsev_e = -1;
pub const SEV_FATAL: errsev_e = -2;
pub type errmodule_t = errmodule_e;
pub type errmodule_e = libc::c_uint;
pub const ZBAR_MOD_UNKNOWN: errmodule_e = 4;
pub const ZBAR_MOD_IMAGE_SCANNER: errmodule_e = 3;
pub const ZBAR_MOD_WINDOW: errmodule_e = 2;
pub const ZBAR_MOD_VIDEO: errmodule_e = 1;
pub const ZBAR_MOD_PROCESSOR: errmodule_e = 0;
/* just in case */
/* reporting module */
/* formatted and passed to application */
/* errno for system errors */
/* reporting function */
/* description */
/* single string argument */
/* single integer argument */
/* * cleanup handler callback function.
 * called to free sample data when an image is destroyed.
 */
pub type zbar_image_cleanup_handler_t = unsafe extern fn(_: *mut zbar_image_t) -> ();
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
/* pack bit size and location offset of a component into one byte
 */
pub type conversion_handler_t = unsafe extern fn(
    _: *mut zbar_image_t,
    _: *const zbar_format_def_t,
    _: *const zbar_image_t,
    _: *const zbar_format_def_t,
) -> ();
pub type zbar_format_def_t = zbar_format_def_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zbar_format_def_s {
    pub format: uint32_t,
    pub group: zbar_format_group_t,
    pub p: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub gen: [uint8_t; 4],
    pub rgb: C2RustUnnamed_1,
    pub yuv: C2RustUnnamed_0,
    pub cmp: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub xsub2: uint8_t,
    pub ysub2: uint8_t,
    pub packorder: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub bpp: uint8_t,
    pub red: uint8_t,
    pub green: uint8_t,
    pub blue: uint8_t,
}
pub type zbar_format_group_t = zbar_format_group_e;
pub type zbar_format_group_e = libc::c_uint;
pub const ZBAR_FMT_NUM: zbar_format_group_e = 6;
pub const ZBAR_FMT_JPEG: zbar_format_group_e = 5;
pub const ZBAR_FMT_YUV_NV: zbar_format_group_e = 4;
pub const ZBAR_FMT_RGB_PACKED: zbar_format_group_e = 3;
pub const ZBAR_FMT_YUV_PACKED: zbar_format_group_e = 2;
pub const ZBAR_FMT_YUV_PLANAR: zbar_format_group_e = 1;
pub const ZBAR_FMT_GRAY: zbar_format_group_e = 0;
pub type conversion_def_t = conversion_def_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conversion_def_s {
    pub cost: libc::c_int,
    pub func: Option<conversion_handler_t>,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zbar_window_s {
    pub err: errinfo_t,
    pub image: *mut zbar_image_t,
    pub overlay: libc::c_uint,
    pub format: uint32_t,
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub max_width: libc::c_uint,
    pub max_height: libc::c_uint,
    pub src_format: uint32_t,
    pub src_width: libc::c_uint,
    pub src_height: libc::c_uint,
    pub dst_width: libc::c_uint,
    pub dst_height: libc::c_uint,
    pub scale_num: libc::c_uint,
    pub scale_den: libc::c_uint,
    pub scaled_offset: point_t,
    pub scaled_size: point_t,
    pub formats: *mut uint32_t,
    pub imglock: zbar_mutex_t,
    pub display: *mut libc::c_void,
    pub xwin: libc::c_ulong,
    pub time: libc::c_ulong,
    pub time_avg: libc::c_ulong,
    pub state: *mut window_state_t,
    pub init: Option<
        unsafe extern fn(
            _: *mut zbar_window_t,
            _: *mut zbar_image_t,
            _: libc::c_int,
        ) -> libc::c_int,
    >,
    pub draw_image:
        Option<unsafe extern fn(_: *mut zbar_window_t, _: *mut zbar_image_t) -> libc::c_int>,
    pub cleanup: Option<unsafe extern fn(_: *mut zbar_window_t) -> libc::c_int>,
}
/* conversion "badness" */
/* function that accomplishes it */
/* @} */
/* ------------------------------------------------------------ */
/* * @name Window interface
 * @anchor c-window
 * mid-level output window abstraction.
 * displays images to user-specified platform specific output window
 */
/* @{ */
/* * opaque window object. */
pub type zbar_window_t = zbar_window_s;
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
 *------------------------------------------------------------------------ */
pub type window_state_t = window_state_s;
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
/* FIXME don't we need varargs hacks here? */
/* unused at src, avoid double free */
#[inline]
unsafe extern fn err_capture(
    mut container: *const libc::c_void,
    mut sev: errsev_t,
    mut type_0: zbar_error_t,
    mut func: *const libc::c_char,
    mut detail: *const libc::c_char,
) -> libc::c_int {
    let mut err: *mut errinfo_t = container as *mut errinfo_t;
    if (*err).magic == 0x5252457a as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"err->magic == ERRINFO_MAGIC\x00" as *const u8 as
                          *const libc::c_char,
                      b"./zbar/error.h\x00" as *const u8 as
                          *const libc::c_char,
                      150 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 82],
                                                &[libc::c_char; 82]>(b"int err_capture(const void *, errsev_t, zbar_error_t, const char *, const char *)\x00")).as_ptr());
    }
    if type_0 as libc::c_uint == ZBAR_ERR_SYSTEM as libc::c_int as libc::c_uint {
        (*err).errnum = *__errno_location()
    }
    (*err).sev = sev;
    (*err).type_0 = type_0;
    (*err).func = func;
    (*err).detail = detail;
    if _zbar_verbosity >= 1 as libc::c_int {
        _zbar_error_spew(err as *const libc::c_void, 0 as libc::c_int);
    }
    return -(1 as libc::c_int);
}
#[inline]
unsafe extern fn _zbar_mutex_lock(mut lock: *mut zbar_mutex_t) -> libc::c_int {
    let mut rc: libc::c_int = pthread_mutex_lock(lock);
    /* FIXME save system code */
    /*rc = err_capture(proc, SEV_ERROR, ZBAR_ERR_LOCKING, __func__,
    "unable to lock processor");*/
    return rc;
}
#[inline]
unsafe extern fn _zbar_mutex_unlock(mut lock: *mut zbar_mutex_t) -> libc::c_int {
    let mut rc: libc::c_int = pthread_mutex_unlock(lock);
    /* FIXME save system code */
    return rc;
}
/* window.draw has to be thread safe wrt/other apis
 * FIXME should be a semaphore
 */
#[inline]
unsafe extern fn window_lock(mut w: *mut zbar_window_t) -> libc::c_int {
    let mut rc: libc::c_int = 0 as libc::c_int;
    rc = _zbar_mutex_lock(&mut (*w).imglock);
    if rc != 0 {
        err_capture(
            w as *const libc::c_void,
            SEV_FATAL,
            ZBAR_ERR_LOCKING,
            (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"window_lock\x00")).as_ptr(),
            b"unable to acquire lock\x00" as *const u8 as *const libc::c_char,
        );
        (*w).err.errnum = rc;
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern fn window_unlock(mut w: *mut zbar_window_t) -> libc::c_int {
    let mut rc: libc::c_int = 0 as libc::c_int;
    rc = _zbar_mutex_unlock(&mut (*w).imglock);
    if rc != 0 {
        err_capture(
            w as *const libc::c_void,
            SEV_FATAL,
            ZBAR_ERR_LOCKING,
            (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"window_unlock\x00"))
                .as_ptr(),
            b"unable to release lock\x00" as *const u8 as *const libc::c_char,
        );
        (*w).err.errnum = rc;
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
/* NULL terminated list of known formats, in order of preference
 * (NB Cr=V Cb=U)
 */
#[no_mangle]
pub static mut _zbar_formats: [uint32_t; 38] = [
    ('4' as i32 as libc::c_ulong
        | ('2' as i32 as libc::c_ulong) << 8 as libc::c_int
        | ('2' as i32 as libc::c_ulong) << 16 as libc::c_int
        | ('P' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    ('I' as i32 as libc::c_ulong
        | ('4' as i32 as libc::c_ulong) << 8 as libc::c_int
        | ('2' as i32 as libc::c_ulong) << 16 as libc::c_int
        | ('0' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    ('Y' as i32 as libc::c_ulong
        | ('U' as i32 as libc::c_ulong) << 8 as libc::c_int
        | ('1' as i32 as libc::c_ulong) << 16 as libc::c_int
        | ('2' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    ('Y' as i32 as libc::c_ulong
        | ('V' as i32 as libc::c_ulong) << 8 as libc::c_int
        | ('1' as i32 as libc::c_ulong) << 16 as libc::c_int
        | ('2' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    ('4' as i32 as libc::c_ulong
        | ('1' as i32 as libc::c_ulong) << 8 as libc::c_int
        | ('1' as i32 as libc::c_ulong) << 16 as libc::c_int
        | ('P' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    ('N' as i32 as libc::c_ulong
        | ('V' as i32 as libc::c_ulong) << 8 as libc::c_int
        | ('1' as i32 as libc::c_ulong) << 16 as libc::c_int
        | ('2' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    ('N' as i32 as libc::c_ulong
        | ('V' as i32 as libc::c_ulong) << 8 as libc::c_int
        | ('2' as i32 as libc::c_ulong) << 16 as libc::c_int
        | ('1' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    ('Y' as i32 as libc::c_ulong
        | ('U' as i32 as libc::c_ulong) << 8 as libc::c_int
        | ('Y' as i32 as libc::c_ulong) << 16 as libc::c_int
        | ('V' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    ('U' as i32 as libc::c_ulong
        | ('Y' as i32 as libc::c_ulong) << 8 as libc::c_int
        | ('V' as i32 as libc::c_ulong) << 16 as libc::c_int
        | ('Y' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    ('Y' as i32 as libc::c_ulong
        | ('U' as i32 as libc::c_ulong) << 8 as libc::c_int
        | ('Y' as i32 as libc::c_ulong) << 16 as libc::c_int
        | ('2' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    ('Y' as i32 as libc::c_ulong
        | ('U' as i32 as libc::c_ulong) << 8 as libc::c_int
        | ('V' as i32 as libc::c_ulong) << 16 as libc::c_int
        | ('4' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    ('R' as i32 as libc::c_ulong
        | ('G' as i32 as libc::c_ulong) << 8 as libc::c_int
        | ('B' as i32 as libc::c_ulong) << 16 as libc::c_int
        | ('3' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    (3 as libc::c_int as libc::c_ulong
        | (0 as libc::c_int as libc::c_ulong) << 8 as libc::c_int
        | (0 as libc::c_int as libc::c_ulong) << 16 as libc::c_int
        | (0 as libc::c_int as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    ('B' as i32 as libc::c_ulong
        | ('G' as i32 as libc::c_ulong) << 8 as libc::c_int
        | ('R' as i32 as libc::c_ulong) << 16 as libc::c_int
        | ('3' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    ('R' as i32 as libc::c_ulong
        | ('G' as i32 as libc::c_ulong) << 8 as libc::c_int
        | ('B' as i32 as libc::c_ulong) << 16 as libc::c_int
        | ('4' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    ('B' as i32 as libc::c_ulong
        | ('G' as i32 as libc::c_ulong) << 8 as libc::c_int
        | ('R' as i32 as libc::c_ulong) << 16 as libc::c_int
        | ('4' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    ('R' as i32 as libc::c_ulong
        | ('G' as i32 as libc::c_ulong) << 8 as libc::c_int
        | ('B' as i32 as libc::c_ulong) << 16 as libc::c_int
        | ('P' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    ('R' as i32 as libc::c_ulong
        | ('G' as i32 as libc::c_ulong) << 8 as libc::c_int
        | ('B' as i32 as libc::c_ulong) << 16 as libc::c_int
        | ('O' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    ('R' as i32 as libc::c_ulong
        | ('G' as i32 as libc::c_ulong) << 8 as libc::c_int
        | ('B' as i32 as libc::c_ulong) << 16 as libc::c_int
        | ('R' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    ('R' as i32 as libc::c_ulong
        | ('G' as i32 as libc::c_ulong) << 8 as libc::c_int
        | ('B' as i32 as libc::c_ulong) << 16 as libc::c_int
        | ('Q' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    ('Y' as i32 as libc::c_ulong
        | ('U' as i32 as libc::c_ulong) << 8 as libc::c_int
        | ('V' as i32 as libc::c_ulong) << 16 as libc::c_int
        | ('9' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    ('Y' as i32 as libc::c_ulong
        | ('V' as i32 as libc::c_ulong) << 8 as libc::c_int
        | ('U' as i32 as libc::c_ulong) << 16 as libc::c_int
        | ('9' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    ('G' as i32 as libc::c_ulong
        | ('R' as i32 as libc::c_ulong) << 8 as libc::c_int
        | ('E' as i32 as libc::c_ulong) << 16 as libc::c_int
        | ('Y' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    ('Y' as i32 as libc::c_ulong
        | ('8' as i32 as libc::c_ulong) << 8 as libc::c_int
        | ('0' as i32 as libc::c_ulong) << 16 as libc::c_int
        | ('0' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    ('Y' as i32 as libc::c_ulong
        | ('8' as i32 as libc::c_ulong) << 8 as libc::c_int
        | (' ' as i32 as libc::c_ulong) << 16 as libc::c_int
        | (' ' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    ('Y' as i32 as libc::c_ulong
        | ('8' as i32 as libc::c_ulong) << 8 as libc::c_int
        | (0 as libc::c_int as libc::c_ulong) << 16 as libc::c_int
        | (0 as libc::c_int as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    ('R' as i32 as libc::c_ulong
        | ('G' as i32 as libc::c_ulong) << 8 as libc::c_int
        | ('B' as i32 as libc::c_ulong) << 16 as libc::c_int
        | ('1' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    ('R' as i32 as libc::c_ulong
        | ('4' as i32 as libc::c_ulong) << 8 as libc::c_int
        | ('4' as i32 as libc::c_ulong) << 16 as libc::c_int
        | ('4' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    ('B' as i32 as libc::c_ulong
        | ('A' as i32 as libc::c_ulong) << 8 as libc::c_int
        | ('8' as i32 as libc::c_ulong) << 16 as libc::c_int
        | ('1' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    ('Y' as i32 as libc::c_ulong
        | ('4' as i32 as libc::c_ulong) << 8 as libc::c_int
        | ('1' as i32 as libc::c_ulong) << 16 as libc::c_int
        | ('P' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    ('Y' as i32 as libc::c_ulong
        | ('4' as i32 as libc::c_ulong) << 8 as libc::c_int
        | ('4' as i32 as libc::c_ulong) << 16 as libc::c_int
        | ('4' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    ('Y' as i32 as libc::c_ulong
        | ('U' as i32 as libc::c_ulong) << 8 as libc::c_int
        | ('V' as i32 as libc::c_ulong) << 16 as libc::c_int
        | ('O' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    ('H' as i32 as libc::c_ulong
        | ('M' as i32 as libc::c_ulong) << 8 as libc::c_int
        | ('1' as i32 as libc::c_ulong) << 16 as libc::c_int
        | ('2' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    ('H' as i32 as libc::c_ulong
        | ('I' as i32 as libc::c_ulong) << 8 as libc::c_int
        | ('2' as i32 as libc::c_ulong) << 16 as libc::c_int
        | ('4' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    ('J' as i32 as libc::c_ulong
        | ('P' as i32 as libc::c_ulong) << 8 as libc::c_int
        | ('E' as i32 as libc::c_ulong) << 16 as libc::c_int
        | ('G' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    ('M' as i32 as libc::c_ulong
        | ('J' as i32 as libc::c_ulong) << 8 as libc::c_int
        | ('P' as i32 as libc::c_ulong) << 16 as libc::c_int
        | ('G' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    ('M' as i32 as libc::c_ulong
        | ('P' as i32 as libc::c_ulong) << 8 as libc::c_int
        | ('E' as i32 as libc::c_ulong) << 16 as libc::c_int
        | ('G' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
    0 as libc::c_int as uint32_t,
];
// Initialized in run_static_initializers
#[no_mangle]
pub static mut _zbar_num_formats: libc::c_int = 0;
/* format definitions */
static mut format_defs: [zbar_format_def_t; 31] = [
    {
        let mut init = zbar_format_def_s {
            format: ('R' as i32 as libc::c_ulong
                | ('G' as i32 as libc::c_ulong) << 8 as libc::c_int
                | ('B' as i32 as libc::c_ulong) << 16 as libc::c_int
                | ('4' as i32 as libc::c_ulong) << 24 as libc::c_int)
                as uint32_t,
            group: ZBAR_FMT_RGB_PACKED,
            p: C2RustUnnamed {
                gen: [
                    4 as libc::c_int as uint8_t,
                    ((8 as libc::c_int - 8 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 8 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                    ((8 as libc::c_int - 8 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 16 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                    ((8 as libc::c_int - 8 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 24 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                ],
            },
        };
        init
    },
    {
        let mut init = zbar_format_def_s {
            format: ('B' as i32 as libc::c_ulong
                | ('G' as i32 as libc::c_ulong) << 8 as libc::c_int
                | ('R' as i32 as libc::c_ulong) << 16 as libc::c_int
                | ('1' as i32 as libc::c_ulong) << 24 as libc::c_int)
                as uint32_t,
            group: ZBAR_FMT_RGB_PACKED,
            p: C2RustUnnamed {
                gen: [
                    1 as libc::c_int as uint8_t,
                    ((8 as libc::c_int - 3 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 0 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                    ((8 as libc::c_int - 3 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 3 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                    ((8 as libc::c_int - 2 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 6 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                ],
            },
        };
        init
    },
    {
        let mut init = zbar_format_def_s {
            format: ('4' as i32 as libc::c_ulong
                | ('2' as i32 as libc::c_ulong) << 8 as libc::c_int
                | ('2' as i32 as libc::c_ulong) << 16 as libc::c_int
                | ('P' as i32 as libc::c_ulong) << 24 as libc::c_int)
                as uint32_t,
            group: ZBAR_FMT_YUV_PLANAR,
            p: C2RustUnnamed {
                gen: [
                    1 as libc::c_int as uint8_t,
                    0 as libc::c_int as uint8_t,
                    0 as libc::c_int as uint8_t,
                    0,
                ],
            },
        };
        init
    },
    {
        let mut init = zbar_format_def_s {
            format: ('Y' as i32 as libc::c_ulong
                | ('8' as i32 as libc::c_ulong) << 8 as libc::c_int
                | ('0' as i32 as libc::c_ulong) << 16 as libc::c_int
                | ('0' as i32 as libc::c_ulong) << 24 as libc::c_int)
                as uint32_t,
            group: ZBAR_FMT_GRAY,
            p: C2RustUnnamed {
                gen: [0; 4],
            },
        };
        init
    },
    {
        let mut init = zbar_format_def_s {
            format: ('Y' as i32 as libc::c_ulong
                | ('U' as i32 as libc::c_ulong) << 8 as libc::c_int
                | ('Y' as i32 as libc::c_ulong) << 16 as libc::c_int
                | ('2' as i32 as libc::c_ulong) << 24 as libc::c_int)
                as uint32_t,
            group: ZBAR_FMT_YUV_PACKED,
            p: C2RustUnnamed {
                gen: [
                    1 as libc::c_int as uint8_t,
                    0 as libc::c_int as uint8_t,
                    0 as libc::c_int as uint8_t,
                    0,
                ],
            },
        };
        init
    },
    {
        let mut init = zbar_format_def_s {
            format: ('J' as i32 as libc::c_ulong
                | ('P' as i32 as libc::c_ulong) << 8 as libc::c_int
                | ('E' as i32 as libc::c_ulong) << 16 as libc::c_int
                | ('G' as i32 as libc::c_ulong) << 24 as libc::c_int)
                as uint32_t,
            group: ZBAR_FMT_JPEG,
            p: C2RustUnnamed {
                gen: [0; 4],
            },
        };
        init
    },
    {
        let mut init = zbar_format_def_s {
            format: ('Y' as i32 as libc::c_ulong
                | ('V' as i32 as libc::c_ulong) << 8 as libc::c_int
                | ('Y' as i32 as libc::c_ulong) << 16 as libc::c_int
                | ('U' as i32 as libc::c_ulong) << 24 as libc::c_int)
                as uint32_t,
            group: ZBAR_FMT_YUV_PACKED,
            p: C2RustUnnamed {
                gen: [
                    1 as libc::c_int as uint8_t,
                    0 as libc::c_int as uint8_t,
                    1 as libc::c_int as uint8_t,
                    0,
                ],
            },
        };
        init
    },
    {
        let mut init = zbar_format_def_s {
            format: ('Y' as i32 as libc::c_ulong
                | ('8' as i32 as libc::c_ulong) << 8 as libc::c_int
                | (0 as libc::c_int as libc::c_ulong) << 16 as libc::c_int
                | (0 as libc::c_int as libc::c_ulong) << 24 as libc::c_int)
                as uint32_t,
            group: ZBAR_FMT_GRAY,
            p: C2RustUnnamed {
                gen: [0; 4],
            },
        };
        init
    },
    {
        let mut init = zbar_format_def_s {
            format: ('N' as i32 as libc::c_ulong
                | ('V' as i32 as libc::c_ulong) << 8 as libc::c_int
                | ('2' as i32 as libc::c_ulong) << 16 as libc::c_int
                | ('1' as i32 as libc::c_ulong) << 24 as libc::c_int)
                as uint32_t,
            group: ZBAR_FMT_YUV_NV,
            p: C2RustUnnamed {
                gen: [
                    1 as libc::c_int as uint8_t,
                    1 as libc::c_int as uint8_t,
                    1 as libc::c_int as uint8_t,
                    0,
                ],
            },
        };
        init
    },
    {
        let mut init = zbar_format_def_s {
            format: ('N' as i32 as libc::c_ulong
                | ('V' as i32 as libc::c_ulong) << 8 as libc::c_int
                | ('1' as i32 as libc::c_ulong) << 16 as libc::c_int
                | ('2' as i32 as libc::c_ulong) << 24 as libc::c_int)
                as uint32_t,
            group: ZBAR_FMT_YUV_NV,
            p: C2RustUnnamed {
                gen: [
                    1 as libc::c_int as uint8_t,
                    1 as libc::c_int as uint8_t,
                    0 as libc::c_int as uint8_t,
                    0,
                ],
            },
        };
        init
    },
    {
        let mut init = zbar_format_def_s {
            format: ('B' as i32 as libc::c_ulong
                | ('G' as i32 as libc::c_ulong) << 8 as libc::c_int
                | ('R' as i32 as libc::c_ulong) << 16 as libc::c_int
                | ('3' as i32 as libc::c_ulong) << 24 as libc::c_int)
                as uint32_t,
            group: ZBAR_FMT_RGB_PACKED,
            p: C2RustUnnamed {
                gen: [
                    3 as libc::c_int as uint8_t,
                    ((8 as libc::c_int - 8 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 16 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                    ((8 as libc::c_int - 8 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 8 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                    ((8 as libc::c_int - 8 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 0 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                ],
            },
        };
        init
    },
    {
        let mut init = zbar_format_def_s {
            format: ('Y' as i32 as libc::c_ulong
                | ('V' as i32 as libc::c_ulong) << 8 as libc::c_int
                | ('U' as i32 as libc::c_ulong) << 16 as libc::c_int
                | ('9' as i32 as libc::c_ulong) << 24 as libc::c_int)
                as uint32_t,
            group: ZBAR_FMT_YUV_PLANAR,
            p: C2RustUnnamed {
                gen: [
                    2 as libc::c_int as uint8_t,
                    2 as libc::c_int as uint8_t,
                    1 as libc::c_int as uint8_t,
                    0,
                ],
            },
        };
        init
    },
    {
        let mut init = zbar_format_def_s {
            format: ('R' as i32 as libc::c_ulong
                | ('G' as i32 as libc::c_ulong) << 8 as libc::c_int
                | ('B' as i32 as libc::c_ulong) << 16 as libc::c_int
                | ('O' as i32 as libc::c_ulong) << 24 as libc::c_int)
                as uint32_t,
            group: ZBAR_FMT_RGB_PACKED,
            p: C2RustUnnamed {
                gen: [
                    2 as libc::c_int as uint8_t,
                    ((8 as libc::c_int - 5 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 10 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                    ((8 as libc::c_int - 5 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 5 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                    ((8 as libc::c_int - 5 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 0 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                ],
            },
        };
        init
    },
    {
        let mut init = zbar_format_def_s {
            format: ('R' as i32 as libc::c_ulong
                | ('G' as i32 as libc::c_ulong) << 8 as libc::c_int
                | ('B' as i32 as libc::c_ulong) << 16 as libc::c_int
                | ('Q' as i32 as libc::c_ulong) << 24 as libc::c_int)
                as uint32_t,
            group: ZBAR_FMT_RGB_PACKED,
            p: C2RustUnnamed {
                gen: [
                    2 as libc::c_int as uint8_t,
                    ((8 as libc::c_int - 5 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 2 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                    ((8 as libc::c_int - 5 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 13 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                    ((8 as libc::c_int - 5 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 8 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                ],
            },
        };
        init
    },
    {
        let mut init = zbar_format_def_s {
            format: ('G' as i32 as libc::c_ulong
                | ('R' as i32 as libc::c_ulong) << 8 as libc::c_int
                | ('E' as i32 as libc::c_ulong) << 16 as libc::c_int
                | ('Y' as i32 as libc::c_ulong) << 24 as libc::c_int)
                as uint32_t,
            group: ZBAR_FMT_GRAY,
            p: C2RustUnnamed {
                gen: [0; 4],
            },
        };
        init
    },
    {
        let mut init = zbar_format_def_s {
            format: (3 as libc::c_int as libc::c_ulong
                | (0 as libc::c_int as libc::c_ulong) << 8 as libc::c_int
                | (0 as libc::c_int as libc::c_ulong) << 16 as libc::c_int
                | (0 as libc::c_int as libc::c_ulong) << 24 as libc::c_int)
                as uint32_t,
            group: ZBAR_FMT_RGB_PACKED,
            p: C2RustUnnamed {
                gen: [
                    4 as libc::c_int as uint8_t,
                    ((8 as libc::c_int - 8 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 16 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                    ((8 as libc::c_int - 8 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 8 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                    ((8 as libc::c_int - 8 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 0 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                ],
            },
        };
        init
    },
    {
        let mut init = zbar_format_def_s {
            format: ('Y' as i32 as libc::c_ulong
                | ('8' as i32 as libc::c_ulong) << 8 as libc::c_int
                | (' ' as i32 as libc::c_ulong) << 16 as libc::c_int
                | (' ' as i32 as libc::c_ulong) << 24 as libc::c_int)
                as uint32_t,
            group: ZBAR_FMT_GRAY,
            p: C2RustUnnamed {
                gen: [0; 4],
            },
        };
        init
    },
    {
        let mut init = zbar_format_def_s {
            format: ('I' as i32 as libc::c_ulong
                | ('4' as i32 as libc::c_ulong) << 8 as libc::c_int
                | ('2' as i32 as libc::c_ulong) << 16 as libc::c_int
                | ('0' as i32 as libc::c_ulong) << 24 as libc::c_int)
                as uint32_t,
            group: ZBAR_FMT_YUV_PLANAR,
            p: C2RustUnnamed {
                gen: [
                    1 as libc::c_int as uint8_t,
                    1 as libc::c_int as uint8_t,
                    0 as libc::c_int as uint8_t,
                    0,
                ],
            },
        };
        init
    },
    {
        let mut init = zbar_format_def_s {
            format: ('R' as i32 as libc::c_ulong
                | ('G' as i32 as libc::c_ulong) << 8 as libc::c_int
                | ('B' as i32 as libc::c_ulong) << 16 as libc::c_int
                | ('1' as i32 as libc::c_ulong) << 24 as libc::c_int)
                as uint32_t,
            group: ZBAR_FMT_RGB_PACKED,
            p: C2RustUnnamed {
                gen: [
                    1 as libc::c_int as uint8_t,
                    ((8 as libc::c_int - 3 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 5 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                    ((8 as libc::c_int - 3 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 2 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                    ((8 as libc::c_int - 2 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 0 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                ],
            },
        };
        init
    },
    {
        let mut init = zbar_format_def_s {
            format: ('Y' as i32 as libc::c_ulong
                | ('U' as i32 as libc::c_ulong) << 8 as libc::c_int
                | ('1' as i32 as libc::c_ulong) << 16 as libc::c_int
                | ('2' as i32 as libc::c_ulong) << 24 as libc::c_int)
                as uint32_t,
            group: ZBAR_FMT_YUV_PLANAR,
            p: C2RustUnnamed {
                gen: [
                    1 as libc::c_int as uint8_t,
                    1 as libc::c_int as uint8_t,
                    0 as libc::c_int as uint8_t,
                    0,
                ],
            },
        };
        init
    },
    {
        let mut init = zbar_format_def_s {
            format: ('Y' as i32 as libc::c_ulong
                | ('V' as i32 as libc::c_ulong) << 8 as libc::c_int
                | ('1' as i32 as libc::c_ulong) << 16 as libc::c_int
                | ('2' as i32 as libc::c_ulong) << 24 as libc::c_int)
                as uint32_t,
            group: ZBAR_FMT_YUV_PLANAR,
            p: C2RustUnnamed {
                gen: [
                    1 as libc::c_int as uint8_t,
                    1 as libc::c_int as uint8_t,
                    1 as libc::c_int as uint8_t,
                    0,
                ],
            },
        };
        init
    },
    {
        let mut init = zbar_format_def_s {
            format: ('R' as i32 as libc::c_ulong
                | ('G' as i32 as libc::c_ulong) << 8 as libc::c_int
                | ('B' as i32 as libc::c_ulong) << 16 as libc::c_int
                | ('3' as i32 as libc::c_ulong) << 24 as libc::c_int)
                as uint32_t,
            group: ZBAR_FMT_RGB_PACKED,
            p: C2RustUnnamed {
                gen: [
                    3 as libc::c_int as uint8_t,
                    ((8 as libc::c_int - 8 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 0 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                    ((8 as libc::c_int - 8 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 8 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                    ((8 as libc::c_int - 8 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 16 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                ],
            },
        };
        init
    },
    {
        let mut init = zbar_format_def_s {
            format: ('R' as i32 as libc::c_ulong
                | ('4' as i32 as libc::c_ulong) << 8 as libc::c_int
                | ('4' as i32 as libc::c_ulong) << 16 as libc::c_int
                | ('4' as i32 as libc::c_ulong) << 24 as libc::c_int)
                as uint32_t,
            group: ZBAR_FMT_RGB_PACKED,
            p: C2RustUnnamed {
                gen: [
                    2 as libc::c_int as uint8_t,
                    ((8 as libc::c_int - 4 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 8 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                    ((8 as libc::c_int - 4 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 4 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                    ((8 as libc::c_int - 4 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 0 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                ],
            },
        };
        init
    },
    {
        let mut init = zbar_format_def_s {
            format: ('B' as i32 as libc::c_ulong
                | ('G' as i32 as libc::c_ulong) << 8 as libc::c_int
                | ('R' as i32 as libc::c_ulong) << 16 as libc::c_int
                | ('4' as i32 as libc::c_ulong) << 24 as libc::c_int)
                as uint32_t,
            group: ZBAR_FMT_RGB_PACKED,
            p: C2RustUnnamed {
                gen: [
                    4 as libc::c_int as uint8_t,
                    ((8 as libc::c_int - 8 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 16 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                    ((8 as libc::c_int - 8 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 8 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                    ((8 as libc::c_int - 8 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 0 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                ],
            },
        };
        init
    },
    {
        let mut init = zbar_format_def_s {
            format: ('Y' as i32 as libc::c_ulong
                | ('U' as i32 as libc::c_ulong) << 8 as libc::c_int
                | ('V' as i32 as libc::c_ulong) << 16 as libc::c_int
                | ('9' as i32 as libc::c_ulong) << 24 as libc::c_int)
                as uint32_t,
            group: ZBAR_FMT_YUV_PLANAR,
            p: C2RustUnnamed {
                gen: [
                    2 as libc::c_int as uint8_t,
                    2 as libc::c_int as uint8_t,
                    0 as libc::c_int as uint8_t,
                    0,
                ],
            },
        };
        init
    },
    {
        let mut init = zbar_format_def_s {
            format: ('M' as i32 as libc::c_ulong
                | ('J' as i32 as libc::c_ulong) << 8 as libc::c_int
                | ('P' as i32 as libc::c_ulong) << 16 as libc::c_int
                | ('G' as i32 as libc::c_ulong) << 24 as libc::c_int)
                as uint32_t,
            group: ZBAR_FMT_JPEG,
            p: C2RustUnnamed {
                gen: [0; 4],
            },
        };
        init
    },
    {
        let mut init = zbar_format_def_s {
            format: ('4' as i32 as libc::c_ulong
                | ('1' as i32 as libc::c_ulong) << 8 as libc::c_int
                | ('1' as i32 as libc::c_ulong) << 16 as libc::c_int
                | ('P' as i32 as libc::c_ulong) << 24 as libc::c_int)
                as uint32_t,
            group: ZBAR_FMT_YUV_PLANAR,
            p: C2RustUnnamed {
                gen: [
                    2 as libc::c_int as uint8_t,
                    0 as libc::c_int as uint8_t,
                    0 as libc::c_int as uint8_t,
                    0,
                ],
            },
        };
        init
    },
    {
        let mut init = zbar_format_def_s {
            format: ('R' as i32 as libc::c_ulong
                | ('G' as i32 as libc::c_ulong) << 8 as libc::c_int
                | ('B' as i32 as libc::c_ulong) << 16 as libc::c_int
                | ('P' as i32 as libc::c_ulong) << 24 as libc::c_int)
                as uint32_t,
            group: ZBAR_FMT_RGB_PACKED,
            p: C2RustUnnamed {
                gen: [
                    2 as libc::c_int as uint8_t,
                    ((8 as libc::c_int - 5 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 11 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                    ((8 as libc::c_int - 6 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 5 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                    ((8 as libc::c_int - 5 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 0 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                ],
            },
        };
        init
    },
    {
        let mut init = zbar_format_def_s {
            format: ('R' as i32 as libc::c_ulong
                | ('G' as i32 as libc::c_ulong) << 8 as libc::c_int
                | ('B' as i32 as libc::c_ulong) << 16 as libc::c_int
                | ('R' as i32 as libc::c_ulong) << 24 as libc::c_int)
                as uint32_t,
            group: ZBAR_FMT_RGB_PACKED,
            p: C2RustUnnamed {
                gen: [
                    2 as libc::c_int as uint8_t,
                    ((8 as libc::c_int - 5 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 3 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                    ((8 as libc::c_int - 6 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 13 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                    ((8 as libc::c_int - 5 as libc::c_int & 0x7 as libc::c_int) << 5 as libc::c_int
                        | 8 as libc::c_int & 0x1f as libc::c_int) as uint8_t,
                ],
            },
        };
        init
    },
    {
        let mut init = zbar_format_def_s {
            format: ('Y' as i32 as libc::c_ulong
                | ('U' as i32 as libc::c_ulong) << 8 as libc::c_int
                | ('Y' as i32 as libc::c_ulong) << 16 as libc::c_int
                | ('V' as i32 as libc::c_ulong) << 24 as libc::c_int)
                as uint32_t,
            group: ZBAR_FMT_YUV_PACKED,
            p: C2RustUnnamed {
                gen: [
                    1 as libc::c_int as uint8_t,
                    0 as libc::c_int as uint8_t,
                    0 as libc::c_int as uint8_t,
                    0,
                ],
            },
        };
        init
    },
    {
        let mut init = zbar_format_def_s {
            format: ('U' as i32 as libc::c_ulong
                | ('Y' as i32 as libc::c_ulong) << 8 as libc::c_int
                | ('V' as i32 as libc::c_ulong) << 16 as libc::c_int
                | ('Y' as i32 as libc::c_ulong) << 24 as libc::c_int)
                as uint32_t,
            group: ZBAR_FMT_YUV_PACKED,
            p: C2RustUnnamed {
                gen: [
                    1 as libc::c_int as uint8_t,
                    0 as libc::c_int as uint8_t,
                    2 as libc::c_int as uint8_t,
                    0,
                ],
            },
        };
        init
    },
];
// Initialized in run_static_initializers
static mut num_format_defs: libc::c_int = 0;
/* verify that format list is in required sort order */
#[inline]
unsafe extern fn verify_format_sort() -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < num_format_defs {
        let mut j: libc::c_int = i * 2 as libc::c_int + 1 as libc::c_int;
        if j < num_format_defs && format_defs[i as usize].format < format_defs[j as usize].format
            || (j + 1 as libc::c_int) < num_format_defs
                && format_defs[(j + 1 as libc::c_int) as usize].format
                    < format_defs[i as usize].format
        {
            break;
        }
        i += 1
    }
    if i == num_format_defs {
        return 0 as libc::c_int;
    }
    /* spew correct order for fix */
    fprintf(
        stderr,
        b"ERROR: image format list is not sorted!?\n\x00" as *const u8 as *const libc::c_char,
    );
    return -(1 as libc::c_int);
}
#[inline]
unsafe extern fn uv_roundup(mut img: *mut zbar_image_t, mut fmt: *const zbar_format_def_t) {
    let mut xmask: libc::c_uint = 0;
    let mut ymask: libc::c_uint = 0;
    if (*fmt).group as libc::c_uint == ZBAR_FMT_GRAY as libc::c_int as libc::c_uint {
        return;
    }
    xmask = (((1 as libc::c_int) << (*fmt).p.yuv.xsub2 as libc::c_int) - 1 as libc::c_int)
        as libc::c_uint;
    if (*img).width & xmask != 0 {
        (*img).width = (*img).width.wrapping_add(xmask) & !xmask
    }
    ymask = (((1 as libc::c_int) << (*fmt).p.yuv.ysub2 as libc::c_int) - 1 as libc::c_int)
        as libc::c_uint;
    if (*img).height & ymask != 0 {
        (*img).height = (*img).height.wrapping_add(ymask) & !ymask
    };
}
#[inline]
unsafe extern fn uvp_size(
    mut img: *const zbar_image_t,
    mut fmt: *const zbar_format_def_t,
) -> libc::c_ulong {
    if (*fmt).group as libc::c_uint == ZBAR_FMT_GRAY as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as libc::c_ulong;
    }
    return ((*img).width >> (*fmt).p.yuv.xsub2 as libc::c_int)
        .wrapping_mul((*img).height >> (*fmt).p.yuv.ysub2 as libc::c_int)
        as libc::c_ulong;
}
#[inline]
unsafe extern fn convert_read_rgb(mut srcp: *const uint8_t, mut bpp: libc::c_int) -> uint32_t {
    let mut p: uint32_t = 0;
    if bpp == 3 as libc::c_int {
        p = *srcp as uint32_t;
        p |= ((*srcp.offset(1 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int)
            as libc::c_uint;
        p |= ((*srcp.offset(2 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int)
            as libc::c_uint
    } else if bpp == 4 as libc::c_int {
        p = *(srcp as *mut uint32_t)
    } else if bpp == 2 as libc::c_int {
        p = *(srcp as *mut uint16_t) as uint32_t
    } else {
        p = *srcp as uint32_t
    }
    return p;
}
#[inline]
unsafe extern fn convert_write_rgb(mut dstp: *mut uint8_t, mut p: uint32_t, mut bpp: libc::c_int) {
    if bpp == 3 as libc::c_int {
        *dstp = (p & 0xff as libc::c_int as libc::c_uint) as uint8_t;
        *dstp.offset(1 as libc::c_int as isize) =
            (p >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as uint8_t;
        *dstp.offset(2 as libc::c_int as isize) =
            (p >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as uint8_t
    } else if bpp == 4 as libc::c_int {
        *(dstp as *mut uint32_t) = p
    } else if bpp == 2 as libc::c_int {
        *(dstp as *mut uint16_t) = p as uint16_t
    } else {
        *dstp = p as uint8_t
    };
}
/* cleanup linked image by unrefing */
unsafe extern fn cleanup_ref(mut img: *mut zbar_image_t) {
    if !(*img).next.is_null() {
        _zbar_image_refcnt((*img).next, -(1 as libc::c_int));
    };
}
/* resize y plane, drop extra columns/rows from the right/bottom,
 * or duplicate last column/row to pad missing data
 */
#[inline]
unsafe extern fn convert_y_resize(
    mut dst: *mut zbar_image_t,
    mut dstfmt: *const zbar_format_def_t,
    mut src: *const zbar_image_t,
    mut srcfmt: *const zbar_format_def_t,
    mut n: size_t,
) {
    let mut psrc: *mut uint8_t = 0 as *mut uint8_t;
    let mut pdst: *mut uint8_t = 0 as *mut uint8_t;
    let mut width: libc::c_uint = 0;
    let mut height: libc::c_uint = 0;
    let mut xpad: libc::c_uint = 0;
    let mut y: libc::c_uint = 0;
    if (*dst).width == (*src).width && (*dst).height == (*src).height {
        memcpy((*dst).data as *mut libc::c_void, (*src).data, n);
        return;
    }
    psrc = (*src).data as *mut libc::c_void as *mut uint8_t;
    pdst = (*dst).data as *mut libc::c_void as *mut uint8_t;
    width = if (*dst).width > (*src).width {
        (*src).width
    } else {
        (*dst).width
    };
    xpad = if (*dst).width > (*src).width {
        (*dst).width.wrapping_sub((*src).width)
    } else {
        0 as libc::c_int as libc::c_uint
    };
    height = if (*dst).height > (*src).height {
        (*src).height
    } else {
        (*dst).height
    };
    y = 0 as libc::c_int as libc::c_uint;
    while y < height {
        memcpy(pdst as *mut libc::c_void, psrc as *const libc::c_void, width as libc::c_ulong);
        pdst = pdst.offset(width as isize);
        psrc = psrc.offset((*src).width as isize);
        if xpad != 0 {
            memset(
                pdst as *mut libc::c_void,
                *psrc.offset(-(1 as libc::c_int as isize)) as libc::c_int,
                xpad as libc::c_ulong,
            );
            pdst = pdst.offset(xpad as isize)
        }
        y = y.wrapping_add(1)
    }
    psrc = psrc.offset(-((*src).width as isize));
    while y < (*dst).height {
        memcpy(pdst as *mut libc::c_void, psrc as *const libc::c_void, width as libc::c_ulong);
        pdst = pdst.offset(width as isize);
        if xpad != 0 {
            memset(
                pdst as *mut libc::c_void,
                *psrc.offset(-(1 as libc::c_int as isize)) as libc::c_int,
                xpad as libc::c_ulong,
            );
            pdst = pdst.offset(xpad as isize)
        }
        y = y.wrapping_add(1)
    }
}
/* make new image w/reference to the same image data */
unsafe extern fn convert_copy(
    mut dst: *mut zbar_image_t,
    mut dstfmt: *const zbar_format_def_t,
    mut src: *const zbar_image_t,
    mut srcfmt: *const zbar_format_def_t,
) {
    if (*src).width == (*dst).width && (*src).height == (*dst).height {
        let mut s: *mut zbar_image_t = src as *mut zbar_image_t;
        (*dst).data = (*src).data;
        (*dst).datalen = (*src).datalen;
        (*dst).cleanup = Some(cleanup_ref as unsafe extern fn(_: *mut zbar_image_t) -> ());
        (*dst).next = s;
        _zbar_image_refcnt(s, 1 as libc::c_int);
    } else {
        /* NB only for GRAY/YUV_PLANAR formats */
        convert_y_resize(
            dst,
            dstfmt,
            src,
            srcfmt,
            (*dst).width.wrapping_mul((*dst).height) as size_t,
        );
    };
}
/* append neutral UV plane to grayscale image */
unsafe extern fn convert_uvp_append(
    mut dst: *mut zbar_image_t,
    mut dstfmt: *const zbar_format_def_t,
    mut src: *const zbar_image_t,
    mut srcfmt: *const zbar_format_def_t,
) {
    let mut n: libc::c_ulong = 0;
    uv_roundup(dst, dstfmt);
    (*dst).datalen = uvp_size(dst, dstfmt).wrapping_mul(2 as libc::c_int as libc::c_ulong);
    n = (*dst).width.wrapping_mul((*dst).height) as libc::c_ulong;
    (*dst).datalen = (*dst).datalen.wrapping_add(n);
    if (*src).datalen >= (*src).width.wrapping_mul((*src).height) as libc::c_ulong {
    } else {
        __assert_fail(b"src->datalen >= src->width * src->height\x00" as
                          *const u8 as *const libc::c_char,
                      b"zbar/convert.c\x00" as *const u8 as
                          *const libc::c_char,
                      373 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 116],
                                                &[libc::c_char; 116]>(b"void convert_uvp_append(zbar_image_t *, const zbar_format_def_t *, const zbar_image_t *, const zbar_format_def_t *)\x00")).as_ptr());
    }
    if _zbar_verbosity >= 24 as libc::c_int {
        fprintf(
            stderr,
            b"%s: dst=%dx%d (%lx) %lx src=%dx%d %lx\n\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"convert_uvp_append\x00"))
                .as_ptr(),
            (*dst).width,
            (*dst).height,
            n,
            (*dst).datalen,
            (*src).width,
            (*src).height,
            (*src).datalen,
        );
    }
    (*dst).data = malloc((*dst).datalen);
    if (*dst).data.is_null() {
        return;
    }
    convert_y_resize(dst, dstfmt, src, srcfmt, n);
    memset(
        ((*dst).data as *mut uint8_t).offset(n as isize) as *mut libc::c_void,
        0x80 as libc::c_int,
        (*dst).datalen.wrapping_sub(n),
    );
}
/* interleave YUV planes into packed YUV */
unsafe extern fn convert_yuv_pack(
    mut dst: *mut zbar_image_t,
    mut dstfmt: *const zbar_format_def_t,
    mut src: *const zbar_image_t,
    mut srcfmt: *const zbar_format_def_t,
) {
    let mut srcm: libc::c_ulong = 0;
    let mut srcn: libc::c_ulong = 0;
    let mut flags: uint8_t = 0;
    let mut srcy: *mut uint8_t = 0 as *mut uint8_t;
    let mut dstp: *mut uint8_t = 0 as *mut uint8_t;
    let mut srcu: *const uint8_t = 0 as *const uint8_t;
    let mut srcv: *const uint8_t = 0 as *const uint8_t;
    let mut srcl: libc::c_uint = 0;
    let mut xmask: libc::c_uint = 0;
    let mut ymask: libc::c_uint = 0;
    let mut x: libc::c_uint = 0;
    let mut y: libc::c_uint = 0;
    let mut y0: uint8_t = 0 as libc::c_int as uint8_t;
    let mut y1: uint8_t = 0 as libc::c_int as uint8_t;
    let mut u: uint8_t = 0x80 as libc::c_int as uint8_t;
    let mut v: uint8_t = 0x80 as libc::c_int as uint8_t;
    uv_roundup(dst, dstfmt);
    (*dst).datalen = ((*dst).width.wrapping_mul((*dst).height) as libc::c_ulong)
        .wrapping_add(uvp_size(dst, dstfmt).wrapping_mul(2 as libc::c_int as libc::c_ulong));
    (*dst).data = malloc((*dst).datalen);
    if (*dst).data.is_null() {
        return;
    }
    dstp = (*dst).data as *mut libc::c_void as *mut uint8_t;
    srcm = uvp_size(src, srcfmt);
    srcn = (*src).width.wrapping_mul((*src).height) as libc::c_ulong;
    if (*src).datalen >= srcn.wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(srcn)) {
    } else {
        __assert_fail(b"src->datalen >= srcn + 2 * srcn\x00" as *const u8 as
                          *const libc::c_char,
                      b"zbar/convert.c\x00" as *const u8 as
                          *const libc::c_char,
                      403 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 114],
                                                &[libc::c_char; 114]>(b"void convert_yuv_pack(zbar_image_t *, const zbar_format_def_t *, const zbar_image_t *, const zbar_format_def_t *)\x00")).as_ptr());
    }
    flags = ((*dstfmt).p.yuv.packorder as libc::c_int ^ (*srcfmt).p.yuv.packorder as libc::c_int)
        as uint8_t;
    srcy = (*src).data as *mut libc::c_void as *mut uint8_t;
    if flags as libc::c_int & 1 as libc::c_int != 0 {
        srcv = ((*src).data as *mut uint8_t).offset(srcn as isize);
        srcu = srcv.offset(srcm as isize)
    } else {
        srcu = ((*src).data as *mut uint8_t).offset(srcn as isize);
        srcv = srcu.offset(srcm as isize)
    }
    flags = ((*dstfmt).p.yuv.packorder as libc::c_int & 2 as libc::c_int) as uint8_t;
    srcl = (*src).width >> (*srcfmt).p.yuv.xsub2 as libc::c_int;
    xmask = (((1 as libc::c_int) << (*srcfmt).p.yuv.xsub2 as libc::c_int) - 1 as libc::c_int)
        as libc::c_uint;
    ymask = (((1 as libc::c_int) << (*srcfmt).p.yuv.ysub2 as libc::c_int) - 1 as libc::c_int)
        as libc::c_uint;
    y = 0 as libc::c_int as libc::c_uint;
    while y < (*dst).height {
        if y >= (*src).height {
            srcy = srcy.offset(-((*src).width as isize));
            srcu = srcu.offset(-(srcl as isize));
            srcv = srcv.offset(-(srcl as isize))
        } else if y & ymask != 0 {
            srcu = srcu.offset(-(srcl as isize));
            srcv = srcv.offset(-(srcl as isize))
        }
        x = 0 as libc::c_int as libc::c_uint;
        while x < (*dst).width {
            if x < (*src).width {
                let fresh0 = srcy;
                srcy = srcy.offset(1);
                y0 = *fresh0;
                let fresh1 = srcy;
                srcy = srcy.offset(1);
                y1 = *fresh1;
                if x & xmask == 0 {
                    let fresh2 = srcu;
                    srcu = srcu.offset(1);
                    u = *fresh2;
                    let fresh3 = srcv;
                    srcv = srcv.offset(1);
                    v = *fresh3
                }
            }
            if flags != 0 {
                let fresh4 = dstp;
                dstp = dstp.offset(1);
                *fresh4 = u;
                let fresh5 = dstp;
                dstp = dstp.offset(1);
                *fresh5 = y0;
                let fresh6 = dstp;
                dstp = dstp.offset(1);
                *fresh6 = v;
                let fresh7 = dstp;
                dstp = dstp.offset(1);
                *fresh7 = y1
            } else {
                let fresh8 = dstp;
                dstp = dstp.offset(1);
                *fresh8 = y0;
                let fresh9 = dstp;
                dstp = dstp.offset(1);
                *fresh9 = u;
                let fresh10 = dstp;
                dstp = dstp.offset(1);
                *fresh10 = y1;
                let fresh11 = dstp;
                dstp = dstp.offset(1);
                *fresh11 = v
            }
            x = x.wrapping_add(2 as libc::c_int as libc::c_uint)
        }
        while x < (*src).width {
            srcy = srcy.offset(2 as libc::c_int as isize);
            if x & xmask == 0 {
                srcu = srcu.offset(1);
                srcv = srcv.offset(1)
            }
            x = x.wrapping_add(2 as libc::c_int as libc::c_uint)
        }
        y = y.wrapping_add(1)
    }
}
/* split packed YUV samples and join into YUV planes
 * FIXME currently ignores color and grayscales the image
 */
unsafe extern fn convert_yuv_unpack(
    mut dst: *mut zbar_image_t,
    mut dstfmt: *const zbar_format_def_t,
    mut src: *const zbar_image_t,
    mut srcfmt: *const zbar_format_def_t,
) {
    let mut dstn: libc::c_ulong = 0;
    let mut dstm2: libc::c_ulong = 0;
    let mut dsty: *mut uint8_t = 0 as *mut uint8_t;
    let mut flags: uint8_t = 0;
    let mut srcp: *const uint8_t = 0 as *const uint8_t;
    let mut srcl: libc::c_uint = 0;
    let mut x: libc::c_uint = 0;
    let mut y: libc::c_uint = 0;
    let mut y0: uint8_t = 0 as libc::c_int as uint8_t;
    let mut y1: uint8_t = 0 as libc::c_int as uint8_t;
    uv_roundup(dst, dstfmt);
    dstn = (*dst).width.wrapping_mul((*dst).height) as libc::c_ulong;
    dstm2 = uvp_size(dst, dstfmt).wrapping_mul(2 as libc::c_int as libc::c_ulong);
    (*dst).datalen = dstn.wrapping_add(dstm2);
    (*dst).data = malloc((*dst).datalen);
    if (*dst).data.is_null() {
        return;
    }
    if dstm2 != 0 {
        memset(
            ((*dst).data as *mut uint8_t).offset(dstn as isize) as *mut libc::c_void,
            0x80 as libc::c_int,
            dstm2,
        );
    }
    dsty = (*dst).data as *mut uint8_t;
    flags = ((*srcfmt).p.yuv.packorder as libc::c_int ^ (*dstfmt).p.yuv.packorder as libc::c_int)
        as uint8_t;
    flags = (flags as libc::c_int & 2 as libc::c_int) as uint8_t;
    srcp = (*src).data as *const uint8_t;
    if flags != 0 {
        srcp = srcp.offset(1)
    }
    srcl = (*src).width.wrapping_add((*src).width >> (*srcfmt).p.yuv.xsub2 as libc::c_int);
    y = 0 as libc::c_int as libc::c_uint;
    while y < (*dst).height {
        if y >= (*src).height {
            srcp = srcp.offset(-(srcl as isize))
        }
        x = 0 as libc::c_int as libc::c_uint;
        while x < (*dst).width {
            if x < (*src).width {
                let fresh12 = srcp;
                srcp = srcp.offset(1);
                y0 = *fresh12;
                srcp = srcp.offset(1);
                let fresh13 = srcp;
                srcp = srcp.offset(1);
                y1 = *fresh13;
                srcp = srcp.offset(1)
            }
            let fresh14 = dsty;
            dsty = dsty.offset(1);
            *fresh14 = y0;
            let fresh15 = dsty;
            dsty = dsty.offset(1);
            *fresh15 = y1;
            x = x.wrapping_add(2 as libc::c_int as libc::c_uint)
        }
        if x < (*src).width {
            srcp = srcp
                .offset((*src).width.wrapping_sub(x).wrapping_mul(2 as libc::c_int as libc::c_uint)
                    as isize)
        }
        y = y.wrapping_add(1)
    }
}
/* resample and resize UV plane(s)
 * FIXME currently ignores color and grayscales the image
 */
unsafe extern fn convert_uvp_resample(
    mut dst: *mut zbar_image_t,
    mut dstfmt: *const zbar_format_def_t,
    mut src: *const zbar_image_t,
    mut srcfmt: *const zbar_format_def_t,
) {
    let mut dstn: libc::c_ulong = 0;
    let mut dstm2: libc::c_ulong = 0;
    uv_roundup(dst, dstfmt);
    dstn = (*dst).width.wrapping_mul((*dst).height) as libc::c_ulong;
    dstm2 = uvp_size(dst, dstfmt).wrapping_mul(2 as libc::c_int as libc::c_ulong);
    (*dst).datalen = dstn.wrapping_add(dstm2);
    (*dst).data = malloc((*dst).datalen);
    if (*dst).data.is_null() {
        return;
    }
    convert_y_resize(dst, dstfmt, src, srcfmt, dstn);
    if dstm2 != 0 {
        memset(
            ((*dst).data as *mut uint8_t).offset(dstn as isize) as *mut libc::c_void,
            0x80 as libc::c_int,
            dstm2,
        );
    };
}
/* rearrange interleaved UV componets */
unsafe extern fn convert_uv_resample(
    mut dst: *mut zbar_image_t,
    mut dstfmt: *const zbar_format_def_t,
    mut src: *const zbar_image_t,
    mut srcfmt: *const zbar_format_def_t,
) {
    let mut dstn: libc::c_ulong = 0;
    let mut dstp: *mut uint8_t = 0 as *mut uint8_t;
    let mut flags: uint8_t = 0;
    let mut srcp: *const uint8_t = 0 as *const uint8_t;
    let mut srcl: libc::c_uint = 0;
    let mut x: libc::c_uint = 0;
    let mut y: libc::c_uint = 0;
    let mut y0: uint8_t = 0 as libc::c_int as uint8_t;
    let mut y1: uint8_t = 0 as libc::c_int as uint8_t;
    let mut u: uint8_t = 0x80 as libc::c_int as uint8_t;
    let mut v: uint8_t = 0x80 as libc::c_int as uint8_t;
    uv_roundup(dst, dstfmt);
    dstn = (*dst).width.wrapping_mul((*dst).height) as libc::c_ulong;
    (*dst).datalen =
        dstn.wrapping_add(uvp_size(dst, dstfmt).wrapping_mul(2 as libc::c_int as libc::c_ulong));
    (*dst).data = malloc((*dst).datalen);
    if (*dst).data.is_null() {
        return;
    }
    dstp = (*dst).data as *mut libc::c_void as *mut uint8_t;
    flags = (((*srcfmt).p.yuv.packorder as libc::c_int ^ (*dstfmt).p.yuv.packorder as libc::c_int)
        & 1 as libc::c_int) as uint8_t;
    srcp = (*src).data as *const uint8_t;
    srcl = (*src).width.wrapping_add((*src).width >> (*srcfmt).p.yuv.xsub2 as libc::c_int);
    y = 0 as libc::c_int as libc::c_uint;
    while y < (*dst).height {
        if y >= (*src).height {
            srcp = srcp.offset(-(srcl as isize))
        }
        x = 0 as libc::c_int as libc::c_uint;
        while x < (*dst).width {
            if x < (*src).width {
                if (*srcfmt).p.yuv.packorder as libc::c_int & 2 as libc::c_int == 0 {
                    let fresh16 = srcp;
                    srcp = srcp.offset(1);
                    y0 = *fresh16;
                    let fresh17 = srcp;
                    srcp = srcp.offset(1);
                    u = *fresh17;
                    let fresh18 = srcp;
                    srcp = srcp.offset(1);
                    y1 = *fresh18;
                    let fresh19 = srcp;
                    srcp = srcp.offset(1);
                    v = *fresh19
                } else {
                    let fresh20 = srcp;
                    srcp = srcp.offset(1);
                    u = *fresh20;
                    let fresh21 = srcp;
                    srcp = srcp.offset(1);
                    y0 = *fresh21;
                    let fresh22 = srcp;
                    srcp = srcp.offset(1);
                    v = *fresh22;
                    let fresh23 = srcp;
                    srcp = srcp.offset(1);
                    y1 = *fresh23
                }
                if flags != 0 {
                    let mut tmp: uint8_t = u;
                    u = v;
                    v = tmp
                }
            }
            if (*dstfmt).p.yuv.packorder as libc::c_int & 2 as libc::c_int == 0 {
                let fresh24 = dstp;
                dstp = dstp.offset(1);
                *fresh24 = y0;
                let fresh25 = dstp;
                dstp = dstp.offset(1);
                *fresh25 = u;
                let fresh26 = dstp;
                dstp = dstp.offset(1);
                *fresh26 = y1;
                let fresh27 = dstp;
                dstp = dstp.offset(1);
                *fresh27 = v
            } else {
                let fresh28 = dstp;
                dstp = dstp.offset(1);
                *fresh28 = u;
                let fresh29 = dstp;
                dstp = dstp.offset(1);
                *fresh29 = y0;
                let fresh30 = dstp;
                dstp = dstp.offset(1);
                *fresh30 = v;
                let fresh31 = dstp;
                dstp = dstp.offset(1);
                *fresh31 = y1
            }
            x = x.wrapping_add(2 as libc::c_int as libc::c_uint)
        }
        if x < (*src).width {
            srcp = srcp
                .offset((*src).width.wrapping_sub(x).wrapping_mul(2 as libc::c_int as libc::c_uint)
                    as isize)
        }
        y = y.wrapping_add(1)
    }
}
/* YUV planes to packed RGB
 * FIXME currently ignores color and grayscales the image
 */
unsafe extern fn convert_yuvp_to_rgb(
    mut dst: *mut zbar_image_t,
    mut dstfmt: *const zbar_format_def_t,
    mut src: *const zbar_image_t,
    mut srcfmt: *const zbar_format_def_t,
) {
    let mut dstp: *mut uint8_t = 0 as *mut uint8_t;
    let mut srcy: *mut uint8_t = 0 as *mut uint8_t;
    let mut drbits: libc::c_int = 0;
    let mut drbit0: libc::c_int = 0;
    let mut dgbits: libc::c_int = 0;
    let mut dgbit0: libc::c_int = 0;
    let mut dbbits: libc::c_int = 0;
    let mut dbbit0: libc::c_int = 0;
    let mut srcm: libc::c_ulong = 0;
    let mut srcn: libc::c_ulong = 0;
    let mut x: libc::c_uint = 0;
    let mut y: libc::c_uint = 0;
    let mut p: uint32_t = 0 as libc::c_int as uint32_t;
    (*dst).datalen =
        (*dst).width.wrapping_mul((*dst).height).wrapping_mul((*dstfmt).p.rgb.bpp as libc::c_uint)
            as libc::c_ulong;
    (*dst).data = malloc((*dst).datalen);
    if (*dst).data.is_null() {
        return;
    }
    dstp = (*dst).data as *mut libc::c_void as *mut uint8_t;
    drbits = (*dstfmt).p.rgb.red as libc::c_int >> 5 as libc::c_int;
    drbit0 = (*dstfmt).p.rgb.red as libc::c_int & 0x1f as libc::c_int;
    dgbits = (*dstfmt).p.rgb.green as libc::c_int >> 5 as libc::c_int;
    dgbit0 = (*dstfmt).p.rgb.green as libc::c_int & 0x1f as libc::c_int;
    dbbits = (*dstfmt).p.rgb.blue as libc::c_int >> 5 as libc::c_int;
    dbbit0 = (*dstfmt).p.rgb.blue as libc::c_int & 0x1f as libc::c_int;
    srcm = uvp_size(src, srcfmt);
    srcn = (*src).width.wrapping_mul((*src).height) as libc::c_ulong;
    if (*src).datalen >= srcn.wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(srcm)) {
    } else {
        __assert_fail(b"src->datalen >= srcn + 2 * srcm\x00" as *const u8 as
                          *const libc::c_char,
                      b"zbar/convert.c\x00" as *const u8 as
                          *const libc::c_char,
                      599 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 117],
                                                &[libc::c_char; 117]>(b"void convert_yuvp_to_rgb(zbar_image_t *, const zbar_format_def_t *, const zbar_image_t *, const zbar_format_def_t *)\x00")).as_ptr());
    }
    srcy = (*src).data as *mut libc::c_void as *mut uint8_t;
    y = 0 as libc::c_int as libc::c_uint;
    while y < (*dst).height {
        if y >= (*src).height {
            srcy = srcy.offset(-((*src).width as isize))
        }
        x = 0 as libc::c_int as libc::c_uint;
        while x < (*dst).width {
            if x < (*src).width {
                /* FIXME color space? */
                let fresh32 = srcy;
                srcy = srcy.offset(1);
                let mut y0: libc::c_uint = *fresh32 as libc::c_uint;
                p = y0 >> drbits << drbit0 | y0 >> dgbits << dgbit0 | y0 >> dbbits << dbbit0
            }
            convert_write_rgb(dstp, p, (*dstfmt).p.rgb.bpp as libc::c_int);
            dstp = dstp.offset((*dstfmt).p.rgb.bpp as libc::c_int as isize);
            x = x.wrapping_add(1)
        }
        if x < (*src).width {
            srcy = srcy.offset((*src).width.wrapping_sub(x) as isize)
        }
        y = y.wrapping_add(1)
    }
}
/* packed RGB to YUV planes
 * FIXME currently ignores color and grayscales the image
 */
unsafe extern fn convert_rgb_to_yuvp(
    mut dst: *mut zbar_image_t,
    mut dstfmt: *const zbar_format_def_t,
    mut src: *const zbar_image_t,
    mut srcfmt: *const zbar_format_def_t,
) {
    let mut dstn: libc::c_ulong = 0;
    let mut dstm2: libc::c_ulong = 0;
    let mut dsty: *mut uint8_t = 0 as *mut uint8_t;
    let mut srcp: *const uint8_t = 0 as *const uint8_t;
    let mut rbits: libc::c_int = 0;
    let mut rbit0: libc::c_int = 0;
    let mut gbits: libc::c_int = 0;
    let mut gbit0: libc::c_int = 0;
    let mut bbits: libc::c_int = 0;
    let mut bbit0: libc::c_int = 0;
    let mut srcl: libc::c_uint = 0;
    let mut x: libc::c_uint = 0;
    let mut y: libc::c_uint = 0;
    let mut y0: uint16_t = 0 as libc::c_int as uint16_t;
    uv_roundup(dst, dstfmt);
    dstn = (*dst).width.wrapping_mul((*dst).height) as libc::c_ulong;
    dstm2 = uvp_size(dst, dstfmt).wrapping_mul(2 as libc::c_int as libc::c_ulong);
    (*dst).datalen = dstn.wrapping_add(dstm2);
    (*dst).data = malloc((*dst).datalen);
    if (*dst).data.is_null() {
        return;
    }
    if dstm2 != 0 {
        memset(
            ((*dst).data as *mut uint8_t).offset(dstn as isize) as *mut libc::c_void,
            0x80 as libc::c_int,
            dstm2,
        );
    }
    dsty = (*dst).data as *mut libc::c_void as *mut uint8_t;
    if (*src).datalen
        >= (*src)
            .width
            .wrapping_mul((*src).height)
            .wrapping_mul((*srcfmt).p.rgb.bpp as libc::c_uint) as libc::c_ulong
    {
    } else {
        __assert_fail(b"src->datalen >= (src->width * src->height * srcfmt->p.rgb.bpp)\x00"
                          as *const u8 as *const libc::c_char,
                      b"zbar/convert.c\x00" as *const u8 as
                          *const libc::c_char,
                      646 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 117],
                                                &[libc::c_char; 117]>(b"void convert_rgb_to_yuvp(zbar_image_t *, const zbar_format_def_t *, const zbar_image_t *, const zbar_format_def_t *)\x00")).as_ptr());
    }
    srcp = (*src).data as *const uint8_t;
    rbits = (*srcfmt).p.rgb.red as libc::c_int >> 5 as libc::c_int;
    rbit0 = (*srcfmt).p.rgb.red as libc::c_int & 0x1f as libc::c_int;
    gbits = (*srcfmt).p.rgb.green as libc::c_int >> 5 as libc::c_int;
    gbit0 = (*srcfmt).p.rgb.green as libc::c_int & 0x1f as libc::c_int;
    bbits = (*srcfmt).p.rgb.blue as libc::c_int >> 5 as libc::c_int;
    bbit0 = (*srcfmt).p.rgb.blue as libc::c_int & 0x1f as libc::c_int;
    srcl = (*src).width.wrapping_mul((*srcfmt).p.rgb.bpp as libc::c_uint);
    y = 0 as libc::c_int as libc::c_uint;
    while y < (*dst).height {
        if y >= (*src).height {
            srcp = srcp.offset(-(srcl as isize))
        }
        x = 0 as libc::c_int as libc::c_uint;
        while x < (*dst).width {
            if x < (*src).width {
                let mut r: uint8_t = 0;
                let mut g: uint8_t = 0;
                let mut b: uint8_t = 0;
                let mut p: uint32_t = convert_read_rgb(srcp, (*srcfmt).p.rgb.bpp as libc::c_int);
                srcp = srcp.offset((*srcfmt).p.rgb.bpp as libc::c_int as isize);
                /* FIXME endianness? */
                r = (p >> rbit0 << rbits & 0xff as libc::c_int as libc::c_uint) as uint8_t;
                g = (p >> gbit0 << gbits & 0xff as libc::c_int as libc::c_uint) as uint8_t;
                b = (p >> bbit0 << bbits & 0xff as libc::c_int as libc::c_uint) as uint8_t;
                /* FIXME color space? */
                y0 = (77 as libc::c_int * r as libc::c_int
                    + 150 as libc::c_int * g as libc::c_int
                    + 29 as libc::c_int * b as libc::c_int
                    + 0x80 as libc::c_int
                    >> 8 as libc::c_int) as uint16_t
            }
            let fresh33 = dsty;
            dsty = dsty.offset(1);
            *fresh33 = y0 as uint8_t;
            x = x.wrapping_add(1)
        }
        if x < (*src).width {
            srcp = srcp.offset(
                (*src).width.wrapping_sub(x).wrapping_mul((*srcfmt).p.rgb.bpp as libc::c_uint)
                    as isize,
            )
        }
        y = y.wrapping_add(1)
    }
}
/* packed YUV to packed RGB */
unsafe extern fn convert_yuv_to_rgb(
    mut dst: *mut zbar_image_t,
    mut dstfmt: *const zbar_format_def_t,
    mut src: *const zbar_image_t,
    mut srcfmt: *const zbar_format_def_t,
) {
    let mut dstp: *mut uint8_t = 0 as *mut uint8_t;
    let mut dstn: libc::c_ulong = (*dst).width.wrapping_mul((*dst).height) as libc::c_ulong;
    let mut drbits: libc::c_int = 0;
    let mut drbit0: libc::c_int = 0;
    let mut dgbits: libc::c_int = 0;
    let mut dgbit0: libc::c_int = 0;
    let mut dbbits: libc::c_int = 0;
    let mut dbbit0: libc::c_int = 0;
    let mut srcp: *const uint8_t = 0 as *const uint8_t;
    let mut srcl: libc::c_uint = 0;
    let mut x: libc::c_uint = 0;
    let mut y: libc::c_uint = 0;
    let mut p: uint32_t = 0 as libc::c_int as uint32_t;
    (*dst).datalen = dstn.wrapping_mul((*dstfmt).p.rgb.bpp as libc::c_ulong);
    (*dst).data = malloc((*dst).datalen);
    if (*dst).data.is_null() {
        return;
    }
    dstp = (*dst).data as *mut libc::c_void as *mut uint8_t;
    drbits = (*dstfmt).p.rgb.red as libc::c_int >> 5 as libc::c_int;
    drbit0 = (*dstfmt).p.rgb.red as libc::c_int & 0x1f as libc::c_int;
    dgbits = (*dstfmt).p.rgb.green as libc::c_int >> 5 as libc::c_int;
    dgbit0 = (*dstfmt).p.rgb.green as libc::c_int & 0x1f as libc::c_int;
    dbbits = (*dstfmt).p.rgb.blue as libc::c_int >> 5 as libc::c_int;
    dbbit0 = (*dstfmt).p.rgb.blue as libc::c_int & 0x1f as libc::c_int;
    if (*src).datalen
        >= ((*src).width.wrapping_mul((*src).height) as libc::c_ulong)
            .wrapping_add(uvp_size(src, srcfmt).wrapping_mul(2 as libc::c_int as libc::c_ulong))
    {
    } else {
        __assert_fail(b"src->datalen >= (src->width * src->height + uvp_size(src, srcfmt) * 2)\x00"
                          as *const u8 as *const libc::c_char,
                      b"zbar/convert.c\x00" as *const u8 as
                          *const libc::c_char,
                      707 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 116],
                                                &[libc::c_char; 116]>(b"void convert_yuv_to_rgb(zbar_image_t *, const zbar_format_def_t *, const zbar_image_t *, const zbar_format_def_t *)\x00")).as_ptr());
    }
    srcp = (*src).data as *const uint8_t;
    if (*srcfmt).p.yuv.packorder as libc::c_int & 2 as libc::c_int != 0 {
        srcp = srcp.offset(1)
    }
    if (*srcfmt).p.yuv.xsub2 as libc::c_int == 1 as libc::c_int {
    } else {
        __assert_fail(b"srcfmt->p.yuv.xsub2 == 1\x00" as *const u8 as
                          *const libc::c_char,
                      b"zbar/convert.c\x00" as *const u8 as
                          *const libc::c_char,
                      712 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 116],
                                                &[libc::c_char; 116]>(b"void convert_yuv_to_rgb(zbar_image_t *, const zbar_format_def_t *, const zbar_image_t *, const zbar_format_def_t *)\x00")).as_ptr());
    }
    srcl = (*src).width.wrapping_add((*src).width >> 1 as libc::c_int);
    y = 0 as libc::c_int as libc::c_uint;
    while y < (*dst).height {
        if y >= (*src).height {
            srcp = srcp.offset(-(srcl as isize))
        }
        x = 0 as libc::c_int as libc::c_uint;
        while x < (*dst).width {
            if x < (*src).width {
                let fresh34 = srcp;
                srcp = srcp.offset(1);
                let mut y0: uint8_t = *fresh34;
                srcp = srcp.offset(1);
                if y0 as libc::c_int <= 16 as libc::c_int {
                    y0 = 0 as libc::c_int as uint8_t
                } else if y0 as libc::c_int >= 235 as libc::c_int {
                    y0 = 255 as libc::c_int as uint8_t
                } else {
                    y0 = ((y0 as libc::c_int - 16 as libc::c_int) as uint16_t as libc::c_int
                        * 255 as libc::c_int
                        / 219 as libc::c_int) as uint8_t
                }
                p = (y0 as libc::c_int >> drbits << drbit0
                    | y0 as libc::c_int >> dgbits << dgbit0
                    | y0 as libc::c_int >> dbbits << dbbit0) as uint32_t
            }
            convert_write_rgb(dstp, p, (*dstfmt).p.rgb.bpp as libc::c_int);
            dstp = dstp.offset((*dstfmt).p.rgb.bpp as libc::c_int as isize);
            x = x.wrapping_add(1)
        }
        if x < (*src).width {
            srcp = srcp
                .offset((*src).width.wrapping_sub(x).wrapping_mul(2 as libc::c_int as libc::c_uint)
                    as isize)
        }
        y = y.wrapping_add(1)
    }
}
/* packed RGB to packed YUV
 * FIXME currently ignores color and grayscales the image
 */
unsafe extern fn convert_rgb_to_yuv(
    mut dst: *mut zbar_image_t,
    mut dstfmt: *const zbar_format_def_t,
    mut src: *const zbar_image_t,
    mut srcfmt: *const zbar_format_def_t,
) {
    let mut dstp: *mut uint8_t = 0 as *mut uint8_t;
    let mut flags: uint8_t = 0;
    let mut srcp: *const uint8_t = 0 as *const uint8_t;
    let mut rbits: libc::c_int = 0;
    let mut rbit0: libc::c_int = 0;
    let mut gbits: libc::c_int = 0;
    let mut gbit0: libc::c_int = 0;
    let mut bbits: libc::c_int = 0;
    let mut bbit0: libc::c_int = 0;
    let mut srcl: libc::c_uint = 0;
    let mut x: libc::c_uint = 0;
    let mut y: libc::c_uint = 0;
    let mut y0: uint16_t = 0 as libc::c_int as uint16_t;
    uv_roundup(dst, dstfmt);
    (*dst).datalen = ((*dst).width.wrapping_mul((*dst).height) as libc::c_ulong)
        .wrapping_add(uvp_size(dst, dstfmt).wrapping_mul(2 as libc::c_int as libc::c_ulong));
    (*dst).data = malloc((*dst).datalen);
    if (*dst).data.is_null() {
        return;
    }
    dstp = (*dst).data as *mut libc::c_void as *mut uint8_t;
    flags = ((*dstfmt).p.yuv.packorder as libc::c_int & 2 as libc::c_int) as uint8_t;
    if (*src).datalen
        >= (*src)
            .width
            .wrapping_mul((*src).height)
            .wrapping_mul((*srcfmt).p.rgb.bpp as libc::c_uint) as libc::c_ulong
    {
    } else {
        __assert_fail(b"src->datalen >= (src->width * src->height * srcfmt->p.rgb.bpp)\x00"
                          as *const u8 as *const libc::c_char,
                      b"zbar/convert.c\x00" as *const u8 as
                          *const libc::c_char,
                      762 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 116],
                                                &[libc::c_char; 116]>(b"void convert_rgb_to_yuv(zbar_image_t *, const zbar_format_def_t *, const zbar_image_t *, const zbar_format_def_t *)\x00")).as_ptr());
    }
    srcp = (*src).data as *const uint8_t;
    rbits = (*srcfmt).p.rgb.red as libc::c_int >> 5 as libc::c_int;
    rbit0 = (*srcfmt).p.rgb.red as libc::c_int & 0x1f as libc::c_int;
    gbits = (*srcfmt).p.rgb.green as libc::c_int >> 5 as libc::c_int;
    gbit0 = (*srcfmt).p.rgb.green as libc::c_int & 0x1f as libc::c_int;
    bbits = (*srcfmt).p.rgb.blue as libc::c_int >> 5 as libc::c_int;
    bbit0 = (*srcfmt).p.rgb.blue as libc::c_int & 0x1f as libc::c_int;
    srcl = (*src).width.wrapping_mul((*srcfmt).p.rgb.bpp as libc::c_uint);
    y = 0 as libc::c_int as libc::c_uint;
    while y < (*dst).height {
        if y >= (*src).height {
            srcp = srcp.offset(-(srcl as isize))
        }
        x = 0 as libc::c_int as libc::c_uint;
        while x < (*dst).width {
            if x < (*src).width {
                let mut r: uint8_t = 0;
                let mut g: uint8_t = 0;
                let mut b: uint8_t = 0;
                let mut p: uint32_t = convert_read_rgb(srcp, (*srcfmt).p.rgb.bpp as libc::c_int);
                srcp = srcp.offset((*srcfmt).p.rgb.bpp as libc::c_int as isize);
                /* FIXME endianness? */
                r = (p >> rbit0 << rbits & 0xff as libc::c_int as libc::c_uint) as uint8_t;
                g = (p >> gbit0 << gbits & 0xff as libc::c_int as libc::c_uint) as uint8_t;
                b = (p >> bbit0 << bbits & 0xff as libc::c_int as libc::c_uint) as uint8_t;
                /* FIXME color space? */
                y0 = (77 as libc::c_int * r as libc::c_int
                    + 150 as libc::c_int * g as libc::c_int
                    + 29 as libc::c_int * b as libc::c_int
                    + 0x80 as libc::c_int
                    >> 8 as libc::c_int) as uint16_t
            }
            if flags != 0 {
                let fresh35 = dstp;
                dstp = dstp.offset(1);
                *fresh35 = 0x80 as libc::c_int as uint8_t;
                let fresh36 = dstp;
                dstp = dstp.offset(1);
                *fresh36 = y0 as uint8_t
            } else {
                let fresh37 = dstp;
                dstp = dstp.offset(1);
                *fresh37 = y0 as uint8_t;
                let fresh38 = dstp;
                dstp = dstp.offset(1);
                *fresh38 = 0x80 as libc::c_int as uint8_t
            }
            x = x.wrapping_add(1)
        }
        if x < (*src).width {
            srcp = srcp.offset(
                (*src).width.wrapping_sub(x).wrapping_mul((*srcfmt).p.rgb.bpp as libc::c_uint)
                    as isize,
            )
        }
        y = y.wrapping_add(1)
    }
}
/* resample and resize packed RGB components */
unsafe extern fn convert_rgb_resample(
    mut dst: *mut zbar_image_t,
    mut dstfmt: *const zbar_format_def_t,
    mut src: *const zbar_image_t,
    mut srcfmt: *const zbar_format_def_t,
) {
    let mut dstn: libc::c_ulong = (*dst).width.wrapping_mul((*dst).height) as libc::c_ulong;
    let mut dstp: *mut uint8_t = 0 as *mut uint8_t;
    let mut drbits: libc::c_int = 0;
    let mut drbit0: libc::c_int = 0;
    let mut dgbits: libc::c_int = 0;
    let mut dgbit0: libc::c_int = 0;
    let mut dbbits: libc::c_int = 0;
    let mut dbbit0: libc::c_int = 0;
    let mut srbits: libc::c_int = 0;
    let mut srbit0: libc::c_int = 0;
    let mut sgbits: libc::c_int = 0;
    let mut sgbit0: libc::c_int = 0;
    let mut sbbits: libc::c_int = 0;
    let mut sbbit0: libc::c_int = 0;
    let mut srcp: *const uint8_t = 0 as *const uint8_t;
    let mut srcl: libc::c_uint = 0;
    let mut x: libc::c_uint = 0;
    let mut y: libc::c_uint = 0;
    let mut p: uint32_t = 0 as libc::c_int as uint32_t;
    (*dst).datalen = dstn.wrapping_mul((*dstfmt).p.rgb.bpp as libc::c_ulong);
    (*dst).data = malloc((*dst).datalen);
    if (*dst).data.is_null() {
        return;
    }
    dstp = (*dst).data as *mut libc::c_void as *mut uint8_t;
    drbits = (*dstfmt).p.rgb.red as libc::c_int >> 5 as libc::c_int;
    drbit0 = (*dstfmt).p.rgb.red as libc::c_int & 0x1f as libc::c_int;
    dgbits = (*dstfmt).p.rgb.green as libc::c_int >> 5 as libc::c_int;
    dgbit0 = (*dstfmt).p.rgb.green as libc::c_int & 0x1f as libc::c_int;
    dbbits = (*dstfmt).p.rgb.blue as libc::c_int >> 5 as libc::c_int;
    dbbit0 = (*dstfmt).p.rgb.blue as libc::c_int & 0x1f as libc::c_int;
    if (*src).datalen
        >= (*src)
            .width
            .wrapping_mul((*src).height)
            .wrapping_mul((*srcfmt).p.rgb.bpp as libc::c_uint) as libc::c_ulong
    {
    } else {
        __assert_fail(b"src->datalen >= (src->width * src->height * srcfmt->p.rgb.bpp)\x00"
                          as *const u8 as *const libc::c_char,
                      b"zbar/convert.c\x00" as *const u8 as
                          *const libc::c_char,
                      828 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 118],
                                                &[libc::c_char; 118]>(b"void convert_rgb_resample(zbar_image_t *, const zbar_format_def_t *, const zbar_image_t *, const zbar_format_def_t *)\x00")).as_ptr());
    }
    srcp = (*src).data as *const uint8_t;
    srbits = (*srcfmt).p.rgb.red as libc::c_int >> 5 as libc::c_int;
    srbit0 = (*srcfmt).p.rgb.red as libc::c_int & 0x1f as libc::c_int;
    sgbits = (*srcfmt).p.rgb.green as libc::c_int >> 5 as libc::c_int;
    sgbit0 = (*srcfmt).p.rgb.green as libc::c_int & 0x1f as libc::c_int;
    sbbits = (*srcfmt).p.rgb.blue as libc::c_int >> 5 as libc::c_int;
    sbbit0 = (*srcfmt).p.rgb.blue as libc::c_int & 0x1f as libc::c_int;
    srcl = (*src).width.wrapping_mul((*srcfmt).p.rgb.bpp as libc::c_uint);
    y = 0 as libc::c_int as libc::c_uint;
    while y < (*dst).height {
        if y >= (*src).height {
            y = y.wrapping_sub(srcl)
        }
        x = 0 as libc::c_int as libc::c_uint;
        while x < (*dst).width {
            if x < (*src).width {
                let mut r: uint8_t = 0;
                let mut g: uint8_t = 0;
                let mut b: uint8_t = 0;
                p = convert_read_rgb(srcp, (*srcfmt).p.rgb.bpp as libc::c_int);
                srcp = srcp.offset((*srcfmt).p.rgb.bpp as libc::c_int as isize);
                /* FIXME endianness? */
                r = (p >> srbit0 << srbits) as uint8_t;
                g = (p >> sgbit0 << sgbits) as uint8_t;
                b = (p >> sbbit0 << sbbits) as uint8_t;
                p = (r as libc::c_int >> drbits << drbit0
                    | g as libc::c_int >> dgbits << dgbit0
                    | b as libc::c_int >> dbbits << dbbit0) as uint32_t
            }
            convert_write_rgb(dstp, p, (*dstfmt).p.rgb.bpp as libc::c_int);
            dstp = dstp.offset((*dstfmt).p.rgb.bpp as libc::c_int as isize);
            x = x.wrapping_add(1)
        }
        if x < (*src).width {
            srcp = srcp.offset(
                (*src).width.wrapping_sub(x).wrapping_mul((*srcfmt).p.rgb.bpp as libc::c_uint)
                    as isize,
            )
        }
        y = y.wrapping_add(1)
    }
}
/* group conversion matrix */
static mut conversions: [[conversion_def_t; 6]; 6] = unsafe {
    [
        [
            {
                let mut init = conversion_def_s {
                    cost: 0 as libc::c_int,
                    func: Some(
                        convert_copy
                            as unsafe extern fn(
                                _: *mut zbar_image_t,
                                _: *const zbar_format_def_t,
                                _: *const zbar_image_t,
                                _: *const zbar_format_def_t,
                            ) -> (),
                    ),
                };
                init
            },
            {
                let mut init = conversion_def_s {
                    cost: 8 as libc::c_int,
                    func: Some(
                        convert_uvp_append
                            as unsafe extern fn(
                                _: *mut zbar_image_t,
                                _: *const zbar_format_def_t,
                                _: *const zbar_image_t,
                                _: *const zbar_format_def_t,
                            ) -> (),
                    ),
                };
                init
            },
            {
                let mut init = conversion_def_s {
                    cost: 24 as libc::c_int,
                    func: Some(
                        convert_yuv_pack
                            as unsafe extern fn(
                                _: *mut zbar_image_t,
                                _: *const zbar_format_def_t,
                                _: *const zbar_image_t,
                                _: *const zbar_format_def_t,
                            ) -> (),
                    ),
                };
                init
            },
            {
                let mut init = conversion_def_s {
                    cost: 32 as libc::c_int,
                    func: Some(
                        convert_yuvp_to_rgb
                            as unsafe extern fn(
                                _: *mut zbar_image_t,
                                _: *const zbar_format_def_t,
                                _: *const zbar_image_t,
                                _: *const zbar_format_def_t,
                            ) -> (),
                    ),
                };
                init
            },
            {
                let mut init = conversion_def_s {
                    cost: 8 as libc::c_int,
                    func: Some(
                        convert_uvp_append
                            as unsafe extern fn(
                                _: *mut zbar_image_t,
                                _: *const zbar_format_def_t,
                                _: *const zbar_image_t,
                                _: *const zbar_format_def_t,
                            ) -> (),
                    ),
                };
                init
            },
            {
                let mut init = conversion_def_s {
                    cost: -(1 as libc::c_int),
                    func: None,
                };
                init
            },
        ],
        [
            {
                let mut init = conversion_def_s {
                    cost: 1 as libc::c_int,
                    func: Some(
                        convert_copy
                            as unsafe extern fn(
                                _: *mut zbar_image_t,
                                _: *const zbar_format_def_t,
                                _: *const zbar_image_t,
                                _: *const zbar_format_def_t,
                            ) -> (),
                    ),
                };
                init
            },
            {
                let mut init = conversion_def_s {
                    cost: 48 as libc::c_int,
                    func: Some(
                        convert_uvp_resample
                            as unsafe extern fn(
                                _: *mut zbar_image_t,
                                _: *const zbar_format_def_t,
                                _: *const zbar_image_t,
                                _: *const zbar_format_def_t,
                            ) -> (),
                    ),
                };
                init
            },
            {
                let mut init = conversion_def_s {
                    cost: 64 as libc::c_int,
                    func: Some(
                        convert_yuv_pack
                            as unsafe extern fn(
                                _: *mut zbar_image_t,
                                _: *const zbar_format_def_t,
                                _: *const zbar_image_t,
                                _: *const zbar_format_def_t,
                            ) -> (),
                    ),
                };
                init
            },
            {
                let mut init = conversion_def_s {
                    cost: 128 as libc::c_int,
                    func: Some(
                        convert_yuvp_to_rgb
                            as unsafe extern fn(
                                _: *mut zbar_image_t,
                                _: *const zbar_format_def_t,
                                _: *const zbar_image_t,
                                _: *const zbar_format_def_t,
                            ) -> (),
                    ),
                };
                init
            },
            {
                let mut init = conversion_def_s {
                    cost: 40 as libc::c_int,
                    func: Some(
                        convert_uvp_append
                            as unsafe extern fn(
                                _: *mut zbar_image_t,
                                _: *const zbar_format_def_t,
                                _: *const zbar_image_t,
                                _: *const zbar_format_def_t,
                            ) -> (),
                    ),
                };
                init
            },
            {
                let mut init = conversion_def_s {
                    cost: -(1 as libc::c_int),
                    func: None,
                };
                init
            },
        ],
        [
            {
                let mut init = conversion_def_s {
                    cost: 24 as libc::c_int,
                    func: Some(
                        convert_yuv_unpack
                            as unsafe extern fn(
                                _: *mut zbar_image_t,
                                _: *const zbar_format_def_t,
                                _: *const zbar_image_t,
                                _: *const zbar_format_def_t,
                            ) -> (),
                    ),
                };
                init
            },
            {
                let mut init = conversion_def_s {
                    cost: 52 as libc::c_int,
                    func: Some(
                        convert_yuv_unpack
                            as unsafe extern fn(
                                _: *mut zbar_image_t,
                                _: *const zbar_format_def_t,
                                _: *const zbar_image_t,
                                _: *const zbar_format_def_t,
                            ) -> (),
                    ),
                };
                init
            },
            {
                let mut init = conversion_def_s {
                    cost: 20 as libc::c_int,
                    func: Some(
                        convert_uv_resample
                            as unsafe extern fn(
                                _: *mut zbar_image_t,
                                _: *const zbar_format_def_t,
                                _: *const zbar_image_t,
                                _: *const zbar_format_def_t,
                            ) -> (),
                    ),
                };
                init
            },
            {
                let mut init = conversion_def_s {
                    cost: 144 as libc::c_int,
                    func: Some(
                        convert_yuv_to_rgb
                            as unsafe extern fn(
                                _: *mut zbar_image_t,
                                _: *const zbar_format_def_t,
                                _: *const zbar_image_t,
                                _: *const zbar_format_def_t,
                            ) -> (),
                    ),
                };
                init
            },
            {
                let mut init = conversion_def_s {
                    cost: 18 as libc::c_int,
                    func: Some(
                        convert_yuv_unpack
                            as unsafe extern fn(
                                _: *mut zbar_image_t,
                                _: *const zbar_format_def_t,
                                _: *const zbar_image_t,
                                _: *const zbar_format_def_t,
                            ) -> (),
                    ),
                };
                init
            },
            {
                let mut init = conversion_def_s {
                    cost: -(1 as libc::c_int),
                    func: None,
                };
                init
            },
        ],
        [
            {
                let mut init = conversion_def_s {
                    cost: 112 as libc::c_int,
                    func: Some(
                        convert_rgb_to_yuvp
                            as unsafe extern fn(
                                _: *mut zbar_image_t,
                                _: *const zbar_format_def_t,
                                _: *const zbar_image_t,
                                _: *const zbar_format_def_t,
                            ) -> (),
                    ),
                };
                init
            },
            {
                let mut init = conversion_def_s {
                    cost: 160 as libc::c_int,
                    func: Some(
                        convert_rgb_to_yuvp
                            as unsafe extern fn(
                                _: *mut zbar_image_t,
                                _: *const zbar_format_def_t,
                                _: *const zbar_image_t,
                                _: *const zbar_format_def_t,
                            ) -> (),
                    ),
                };
                init
            },
            {
                let mut init = conversion_def_s {
                    cost: 144 as libc::c_int,
                    func: Some(
                        convert_rgb_to_yuv
                            as unsafe extern fn(
                                _: *mut zbar_image_t,
                                _: *const zbar_format_def_t,
                                _: *const zbar_image_t,
                                _: *const zbar_format_def_t,
                            ) -> (),
                    ),
                };
                init
            },
            {
                let mut init = conversion_def_s {
                    cost: 120 as libc::c_int,
                    func: Some(
                        convert_rgb_resample
                            as unsafe extern fn(
                                _: *mut zbar_image_t,
                                _: *const zbar_format_def_t,
                                _: *const zbar_image_t,
                                _: *const zbar_format_def_t,
                            ) -> (),
                    ),
                };
                init
            },
            {
                let mut init = conversion_def_s {
                    cost: 152 as libc::c_int,
                    func: Some(
                        convert_rgb_to_yuvp
                            as unsafe extern fn(
                                _: *mut zbar_image_t,
                                _: *const zbar_format_def_t,
                                _: *const zbar_image_t,
                                _: *const zbar_format_def_t,
                            ) -> (),
                    ),
                };
                init
            },
            {
                let mut init = conversion_def_s {
                    cost: -(1 as libc::c_int),
                    func: None,
                };
                init
            },
        ],
        [
            {
                let mut init = conversion_def_s {
                    cost: 1 as libc::c_int,
                    func: Some(
                        convert_copy
                            as unsafe extern fn(
                                _: *mut zbar_image_t,
                                _: *const zbar_format_def_t,
                                _: *const zbar_image_t,
                                _: *const zbar_format_def_t,
                            ) -> (),
                    ),
                };
                init
            },
            {
                let mut init = conversion_def_s {
                    cost: 8 as libc::c_int,
                    func: Some(
                        convert_uvp_append
                            as unsafe extern fn(
                                _: *mut zbar_image_t,
                                _: *const zbar_format_def_t,
                                _: *const zbar_image_t,
                                _: *const zbar_format_def_t,
                            ) -> (),
                    ),
                };
                init
            },
            {
                let mut init = conversion_def_s {
                    cost: 24 as libc::c_int,
                    func: Some(
                        convert_yuv_pack
                            as unsafe extern fn(
                                _: *mut zbar_image_t,
                                _: *const zbar_format_def_t,
                                _: *const zbar_image_t,
                                _: *const zbar_format_def_t,
                            ) -> (),
                    ),
                };
                init
            },
            {
                let mut init = conversion_def_s {
                    cost: 32 as libc::c_int,
                    func: Some(
                        convert_yuvp_to_rgb
                            as unsafe extern fn(
                                _: *mut zbar_image_t,
                                _: *const zbar_format_def_t,
                                _: *const zbar_image_t,
                                _: *const zbar_format_def_t,
                            ) -> (),
                    ),
                };
                init
            },
            {
                let mut init = conversion_def_s {
                    cost: 8 as libc::c_int,
                    func: Some(
                        convert_uvp_append
                            as unsafe extern fn(
                                _: *mut zbar_image_t,
                                _: *const zbar_format_def_t,
                                _: *const zbar_image_t,
                                _: *const zbar_format_def_t,
                            ) -> (),
                    ),
                };
                init
            },
            {
                let mut init = conversion_def_s {
                    cost: -(1 as libc::c_int),
                    func: None,
                };
                init
            },
        ],
        [
            {
                let mut init = conversion_def_s {
                    cost: -(1 as libc::c_int),
                    func: None,
                };
                init
            },
            {
                let mut init = conversion_def_s {
                    cost: -(1 as libc::c_int),
                    func: None,
                };
                init
            },
            {
                let mut init = conversion_def_s {
                    cost: -(1 as libc::c_int),
                    func: None,
                };
                init
            },
            {
                let mut init = conversion_def_s {
                    cost: -(1 as libc::c_int),
                    func: None,
                };
                init
            },
            {
                let mut init = conversion_def_s {
                    cost: -(1 as libc::c_int),
                    func: None,
                };
                init
            },
            {
                let mut init = conversion_def_s {
                    cost: -(1 as libc::c_int),
                    func: None,
                };
                init
            },
        ],
    ]
};
#[no_mangle]
pub unsafe extern fn _zbar_format_lookup(mut fmt: uint32_t) -> *const zbar_format_def_t {
    let mut def: *const zbar_format_def_t = 0 as *const zbar_format_def_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < num_format_defs {
        def = &*format_defs.as_ptr().offset(i as isize) as *const zbar_format_def_t;
        if fmt == (*def).format {
            return def;
        }
        i = i * 2 as libc::c_int + 1 as libc::c_int;
        if fmt > (*def).format {
            i += 1
        }
    }
    return 0 as *const zbar_format_def_t;
}
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
#[no_mangle]
pub unsafe extern fn zbar_image_convert_resize(
    mut src: *const zbar_image_t,
    mut fmt: libc::c_ulong,
    mut width: libc::c_uint,
    mut height: libc::c_uint,
) -> *mut zbar_image_t {
    let mut srcfmt: *const zbar_format_def_t = 0 as *const zbar_format_def_t;
    let mut dstfmt: *const zbar_format_def_t = 0 as *const zbar_format_def_t;
    let mut func: Option<conversion_handler_t> = None;
    let mut dst: *mut zbar_image_t = zbar_image_create();
    (*dst).format = fmt as uint32_t;
    (*dst).width = width;
    (*dst).height = height;
    zbar_image_set_crop(dst, (*src).crop_x, (*src).crop_y, (*src).crop_w, (*src).crop_h);
    if (*src).format as libc::c_ulong == fmt && (*src).width == width && (*src).height == height {
        convert_copy(dst, 0 as *const zbar_format_def_t, src, 0 as *const zbar_format_def_t);
        return dst;
    }
    srcfmt = _zbar_format_lookup((*src).format);
    dstfmt = _zbar_format_lookup((*dst).format);
    if srcfmt.is_null() || dstfmt.is_null() {
        /* FIXME free dst */
        return 0 as *mut zbar_image_t;
    }
    if (*srcfmt).group as libc::c_uint == (*dstfmt).group as libc::c_uint
        && (*srcfmt).p.cmp == (*dstfmt).p.cmp
        && (*src).width == width
        && (*src).height == height
    {
        convert_copy(dst, 0 as *const zbar_format_def_t, src, 0 as *const zbar_format_def_t);
        return dst;
    }
    func = conversions[(*srcfmt).group as usize][(*dstfmt).group as usize].func;
    (*dst).cleanup = Some(zbar_image_free_data as unsafe extern fn(_: *mut zbar_image_t) -> ());
    func.expect("non-null function pointer")(dst, dstfmt, src, srcfmt);
    if (*dst).data.is_null() {
        /* conversion failed */
        zbar_image_destroy(dst);
        return 0 as *mut zbar_image_t;
    }
    return dst;
}
/* * image format conversion.  refer to the documentation for supported
 * image formats
 * @returns a @em new image with the sample data from the original image
 * converted to the requested format.  the original image is
 * unaffected.
 * @note the converted image size may be rounded (up) due to format
 * constraints
 */
#[no_mangle]
pub unsafe extern fn zbar_image_convert(
    mut src: *const zbar_image_t,
    mut fmt: libc::c_ulong,
) -> *mut zbar_image_t {
    return zbar_image_convert_resize(src, fmt, (*src).width, (*src).height);
}
#[inline]
unsafe extern fn has_format(mut fmt: uint32_t, mut fmts: *const uint32_t) -> libc::c_int {
    while *fmts != 0 {
        if *fmts == fmt {
            return 1 as libc::c_int;
        }
        fmts = fmts.offset(1)
    }
    return 0 as libc::c_int;
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
/* select least cost conversion from src format to available dsts */
#[no_mangle]
pub unsafe extern fn _zbar_best_format(
    mut src: uint32_t,
    mut dst: *mut uint32_t,
    mut dsts: *const uint32_t,
) -> libc::c_int {
    let mut srcfmt: *const zbar_format_def_t = 0 as *const zbar_format_def_t;
    let mut min_cost: libc::c_uint = -(1 as libc::c_int) as libc::c_uint;
    if !dst.is_null() {
        *dst = 0 as libc::c_int as uint32_t
    }
    if dsts.is_null() {
        return -(1 as libc::c_int);
    }
    if has_format(src, dsts) != 0 {
        if _zbar_verbosity >= 8 as libc::c_int {
            fprintf(
                stderr,
                b"%s: shared format: %4.4s\n\x00" as *const u8 as *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(
                    b"_zbar_best_format\x00",
                ))
                .as_ptr(),
                &mut src as *mut uint32_t as *mut libc::c_char,
            );
        }
        if !dst.is_null() {
            *dst = src
        }
        return 0 as libc::c_int;
    }
    srcfmt = _zbar_format_lookup(src);
    if srcfmt.is_null() {
        return -(1 as libc::c_int);
    }
    if _zbar_verbosity >= 8 as libc::c_int {
        fprintf(
            stderr,
            b"%s: from %.4s(%08x) to\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"_zbar_best_format\x00"))
                .as_ptr(),
            &mut src as *mut uint32_t as *mut libc::c_char,
            src,
        );
    }
    while *dsts != 0 {
        let mut dstfmt: *const zbar_format_def_t = _zbar_format_lookup(*dsts);
        let mut cost: libc::c_int = 0;
        if !dstfmt.is_null() {
            if (*srcfmt).group as libc::c_uint == (*dstfmt).group as libc::c_uint
                && (*srcfmt).p.cmp == (*dstfmt).p.cmp
            {
                cost = 0 as libc::c_int
            } else {
                cost = conversions[(*srcfmt).group as usize][(*dstfmt).group as usize].cost
            }
            if _zbar_verbosity >= 8 as libc::c_int {
                fprintf(
                    stderr,
                    b" %.4s(%08x)=%d\x00" as *const u8 as *const libc::c_char,
                    dsts as *mut libc::c_char,
                    *dsts,
                    cost,
                );
            }
            if cost >= 0 as libc::c_int && min_cost > cost as libc::c_uint {
                min_cost = cost as libc::c_uint;
                if !dst.is_null() {
                    *dst = *dsts
                }
            }
        }
        dsts = dsts.offset(1)
    }
    if _zbar_verbosity >= 8 as libc::c_int {
        fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
    }
    return min_cost as libc::c_int;
}
/* * select a compatible format between video input and output window.
 * the selection algorithm attempts to use a format shared by
 * video input and window output which is also most useful for
 * barcode scanning.  if a format conversion is necessary, it will
 * heuristically attempt to minimize the cost of the conversion
 */
#[no_mangle]
pub unsafe extern fn zbar_negotiate_format(
    mut vdo: *mut zbar_video_t,
    mut win: *mut zbar_window_t,
) -> libc::c_int {
    static mut y800: [uint32_t; 2] = [
        ('Y' as i32 as libc::c_ulong
            | ('8' as i32 as libc::c_ulong) << 8 as libc::c_int
            | ('0' as i32 as libc::c_ulong) << 16 as libc::c_int
            | ('0' as i32 as libc::c_ulong) << 24 as libc::c_int) as uint32_t,
        0 as libc::c_int as uint32_t,
    ];
    let mut errdst: *mut errinfo_t = 0 as *mut errinfo_t;
    let mut srcs: *const uint32_t = 0 as *const uint32_t;
    let mut dsts: *const uint32_t = 0 as *const uint32_t;
    let mut min_cost: libc::c_uint = -(1 as libc::c_int) as libc::c_uint;
    let mut min_fmt: uint32_t = 0 as libc::c_int as uint32_t;
    let mut fmt: *const uint32_t = 0 as *const uint32_t;
    if vdo.is_null() && win.is_null() {
        return 0 as libc::c_int;
    }
    if !win.is_null() {
        window_lock(win);
    }
    errdst = if !vdo.is_null() {
        &mut (*vdo).err
    } else {
        &mut (*win).err
    };
    if verify_format_sort() != 0 {
        if !win.is_null() {
            window_unlock(win);
        }
        return err_capture(
            errdst as *const libc::c_void,
            SEV_FATAL,
            ZBAR_ERR_INTERNAL,
            (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                b"zbar_negotiate_format\x00",
            ))
            .as_ptr(),
            b"image format list is not sorted!?\x00" as *const u8 as *const libc::c_char,
        );
    }
    if !vdo.is_null() && (*vdo).formats.is_null() || !win.is_null() && (*win).formats.is_null() {
        if !win.is_null() {
            window_unlock(win);
        }
        return err_capture(
            errdst as *const libc::c_void,
            SEV_ERROR,
            ZBAR_ERR_UNSUPPORTED,
            (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                b"zbar_negotiate_format\x00",
            ))
            .as_ptr(),
            b"no input or output formats available\x00" as *const u8 as *const libc::c_char,
        );
    }
    srcs = if !vdo.is_null() {
        (*vdo).formats
    } else {
        y800.as_ptr()
    };
    dsts = if !win.is_null() {
        (*win).formats
    } else {
        y800.as_ptr()
    };
    fmt = _zbar_formats.as_ptr();
    while *fmt != 0 {
        /* only consider formats supported by video device */
        let mut win_fmt: uint32_t = 0 as libc::c_int as uint32_t;
        let mut cost: libc::c_int = 0;
        if !(has_format(*fmt, srcs) == 0) {
            cost = _zbar_best_format(*fmt, &mut win_fmt, dsts);
            if cost < 0 as libc::c_int {
                if _zbar_verbosity >= 4 as libc::c_int {
                    fprintf(
                        stderr,
                        b"%s: %.4s(%08x) -> ? (unsupported)\n\x00" as *const u8
                            as *const libc::c_char,
                        (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                            b"zbar_negotiate_format\x00",
                        ))
                        .as_ptr(),
                        fmt as *mut libc::c_char,
                        *fmt,
                    );
                }
            } else {
                if _zbar_verbosity >= 4 as libc::c_int {
                    fprintf(
                        stderr,
                        b"%s: %.4s(%08x) -> %.4s(%08x) (%d)\n\x00" as *const u8
                            as *const libc::c_char,
                        (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                            b"zbar_negotiate_format\x00",
                        ))
                        .as_ptr(),
                        fmt as *mut libc::c_char,
                        *fmt,
                        &mut win_fmt as *mut uint32_t as *mut libc::c_char,
                        win_fmt,
                        cost,
                    );
                }
                if min_cost > cost as libc::c_uint {
                    min_cost = cost as libc::c_uint;
                    min_fmt = *fmt;
                    if cost == 0 {
                        break;
                    }
                }
            }
        }
        fmt = fmt.offset(1)
    }
    if min_fmt == 0 && !(*vdo).emu_formats.is_null() {
        /* As vdo->formats aren't compatible, just free them */
        free((*vdo).formats as *mut libc::c_void);
        (*vdo).formats = (*vdo).emu_formats;
        (*vdo).emu_formats = 0 as *mut uint32_t;
        /*
         * Use the same cost algorithm to select emulated formats.
         * This might select a sub-optimal conversion, but, in practice,
         * it will select a conversion to YUV at libv4l, and a YUY->Y8
         * in zbar, with it is OK. Yet, it is better to not select the
         * most performatic conversion than to not support the webcam.
         */
        fmt = _zbar_formats.as_ptr();
        while *fmt != 0 {
            /* only consider formats supported by video device */
            let mut win_fmt_0: uint32_t = 0 as libc::c_int as uint32_t;
            let mut cost_0: libc::c_int = 0;
            if !(has_format(*fmt, srcs) == 0) {
                cost_0 = _zbar_best_format(*fmt, &mut win_fmt_0, dsts);
                if cost_0 < 0 as libc::c_int {
                    if _zbar_verbosity >= 4 as libc::c_int {
                        fprintf(
                            stderr,
                            b"%s: %.4s(%08x) -> ? (unsupported)\n\x00" as *const u8
                                as *const libc::c_char,
                            (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                                b"zbar_negotiate_format\x00",
                            ))
                            .as_ptr(),
                            fmt as *mut libc::c_char,
                            *fmt,
                        );
                    }
                } else {
                    if _zbar_verbosity >= 4 as libc::c_int {
                        fprintf(
                            stderr,
                            b"%s: %.4s(%08x) -> %.4s(%08x) (%d)\n\x00" as *const u8
                                as *const libc::c_char,
                            (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                                b"zbar_negotiate_format\x00",
                            ))
                            .as_ptr(),
                            fmt as *mut libc::c_char,
                            *fmt,
                            &mut win_fmt_0 as *mut uint32_t as *mut libc::c_char,
                            win_fmt_0,
                            cost_0,
                        );
                    }
                    if min_cost > cost_0 as libc::c_uint {
                        min_cost = cost_0 as libc::c_uint;
                        min_fmt = *fmt;
                        if cost_0 == 0 {
                            break;
                        }
                    }
                }
            }
            fmt = fmt.offset(1)
        }
    }
    if !win.is_null() {
        window_unlock(win);
    }
    if min_fmt == 0 {
        return err_capture(
            errdst as *const libc::c_void,
            SEV_ERROR,
            ZBAR_ERR_UNSUPPORTED,
            (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                b"zbar_negotiate_format\x00",
            ))
            .as_ptr(),
            b"no supported image formats available\x00" as *const u8 as *const libc::c_char,
        );
    }
    if vdo.is_null() {
        return 0 as libc::c_int;
    }
    if _zbar_verbosity >= 2 as libc::c_int {
        fprintf(
            stderr,
            b"%s: setting best format %.4s(%08x) (%d)\n\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                b"zbar_negotiate_format\x00",
            ))
            .as_ptr(),
            &mut min_fmt as *mut uint32_t as *mut libc::c_char,
            min_fmt,
            min_cost,
        );
    }
    return zbar_video_init(vdo, min_fmt as libc::c_ulong);
}
unsafe extern fn run_static_initializers() {
    _zbar_num_formats = (::std::mem::size_of::<[uint32_t; 38]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
        as libc::c_int;
    num_format_defs = (::std::mem::size_of::<[zbar_format_def_t; 31]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<zbar_format_def_t>() as libc::c_ulong)
        as libc::c_int
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern fn(); 1] = [run_static_initializers];
