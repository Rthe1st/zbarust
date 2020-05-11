use ::c2rust_bitfields;
use ::libc;
extern {
    pub type video_state_s;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    /* * @internal type unsafe error API (don't use) */
    #[no_mangle]
    fn _zbar_error_spew(object: *const libc::c_void, verbosity: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    static mut _zbar_verbosity: libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn _zbar_v4l2_probe(_: *mut zbar_video_t) -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type int64_t = __int64_t;
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
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
/* number of images to preallocate */
/* uninitialized */
/* v4l protocol version 1 */
/* v4l protocol version 2 */
/* video for windows */
/* standard system calls */
/* mmap interface */
/* userspace buffers */
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
/* simple platform mutex abstraction
 */
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
unsafe extern fn err_capture_str(
    mut container: *const libc::c_void,
    mut sev: errsev_t,
    mut type_0: zbar_error_t,
    mut func: *const libc::c_char,
    mut detail: *const libc::c_char,
    mut arg: *const libc::c_char,
) -> libc::c_int {
    let mut err: *mut errinfo_t = container as *mut errinfo_t;
    if (*err).magic == 0x5252457a as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"err->magic == ERRINFO_MAGIC\x00" as *const u8 as
                          *const libc::c_char,
                      b"./zbar/error.h\x00" as *const u8 as
                          *const libc::c_char,
                      176 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 100],
                                                &[libc::c_char; 100]>(b"int err_capture_str(const void *, errsev_t, zbar_error_t, const char *, const char *, const char *)\x00")).as_ptr());
    }
    if !(*err).arg_str.is_null() {
        free((*err).arg_str as *mut libc::c_void);
    }
    (*err).arg_str = strdup(arg);
    return err_capture(container, sev, type_0, func, detail);
}
/* video.next_image and video.recycle_image have to be thread safe
 * wrt/other apis
 */
/* maintains queued buffers in order */
/* FIXME reclaim image */
/* PAL interface */
#[no_mangle]
pub unsafe extern fn _zbar_video_open(
    mut vdo: *mut zbar_video_t,
    mut dev: *const libc::c_char,
) -> libc::c_int {
    (*vdo).fd = open(dev, 0o2 as libc::c_int);
    if (*vdo).fd < 0 as libc::c_int {
        return err_capture_str(
            vdo as *const libc::c_void,
            SEV_ERROR,
            ZBAR_ERR_SYSTEM,
            (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"_zbar_video_open\x00"))
                .as_ptr(),
            b"opening video device \'%s\'\x00" as *const u8 as *const libc::c_char,
            dev,
        );
    }
    if _zbar_verbosity >= 1 as libc::c_int {
        fprintf(
            stderr,
            b"%s: opened camera device %s (fd=%d)\n\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"_zbar_video_open\x00"))
                .as_ptr(),
            dev,
            (*vdo).fd,
        );
    }
    let mut rc: libc::c_int = -(1 as libc::c_int);
    if (*vdo).intf as libc::c_uint != VIDEO_V4L1 as libc::c_int as libc::c_uint {
        rc = _zbar_v4l2_probe(vdo)
    }
    if rc != 0 && (*vdo).fd >= 0 as libc::c_int {
        close((*vdo).fd);
        (*vdo).fd = -(1 as libc::c_int)
    }
    return rc;
}
