use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type video_state_s;
    pub type zbar_window_s;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    /* * @internal type unsafe error API (don't use) */
    #[no_mangle]
    fn _zbar_error_spew(object: *const libc::c_void, verbosity: libc::c_int)
     -> libc::c_int;
    /* * new image constructor.
 * @returns a new image object with uninitialized data and format.
 * this image should be destroyed (using zbar_image_destroy()) as
 * soon as the application is finished with it
 */
    #[no_mangle]
    fn zbar_image_create() -> *mut zbar_image_t;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn zbar_image_set_size(image: *mut zbar_image_t, width: libc::c_uint,
                           height: libc::c_uint);
    #[no_mangle]
    static mut _zbar_verbosity: libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn _zbar_image_free(_: *mut zbar_image_t);
    /* PAL interface */
    #[no_mangle]
    fn _zbar_video_open(_: *mut zbar_video_t, _: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    /* * constructor. */
    /* * destructor. */
    /* * associate reader with an existing platform window.
 * This can be any "Drawable" for X Windows or a "HWND" for windows.
 * input images will be scaled into the output window.
 * pass NULL to detach from the resource, further input will be
 * ignored
 */
    /* * control content level of the reader overlay.
 * the overlay displays graphical data for informational or debug
 * purposes.  higher values increase the level of annotation (possibly
 * decreasing performance). @verbatim
    0 = disable overlay
    1 = outline decoded symbols (default)
    2 = also track and display input frame rate
@endverbatim
 */
    /* * retrieve current content level of reader overlay.
 * @see zbar_window_set_overlay()
 * @since 0.10
 */
    /* * draw a new image into the output window. */
    /* * redraw the last image (exposure handler). */
    /* * resize the image window (reconfigure handler).
 * this does @em not update the contents of the window
 * @since 0.3, changed in 0.4 to not redraw window
 */
    /* * display detail for last window error to stderr.
 * @returns a non-zero value suitable for passing to exit()
 */
    /* * retrieve the detail string for the last window error. */
    /* * retrieve the type code for the last window error. */
    /* * select a compatible format between video input and output window.
 * the selection algorithm attempts to use a format shared by
 * video input and window output which is also most useful for
 * barcode scanning.  if a format conversion is necessary, it will
 * heuristically attempt to minimize the cost of the conversion
 */
    #[no_mangle]
    fn zbar_negotiate_format(video: *mut zbar_video_t,
                             window: *mut zbar_window_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_init(__mutex: *mut pthread_mutex_t,
                          __mutexattr: *const pthread_mutexattr_t)
     -> libc::c_int;
    #[no_mangle]
    fn _zbar_refcnt(cnt: *mut refcnt_t, delta: libc::c_int) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
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
 *------------------------------------------------------------------------*/
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
/*@}*/
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
 *------------------------------------------------------------------------*/
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
    pub init: Option<unsafe extern "C" fn(_: *mut zbar_video_t, _: uint32_t)
                         -> libc::c_int>,
    pub cleanup: Option<unsafe extern "C" fn(_: *mut zbar_video_t)
                            -> libc::c_int>,
    pub start: Option<unsafe extern "C" fn(_: *mut zbar_video_t)
                          -> libc::c_int>,
    pub stop: Option<unsafe extern "C" fn(_: *mut zbar_video_t)
                         -> libc::c_int>,
    pub nq: Option<unsafe extern "C" fn(_: *mut zbar_video_t,
                                        _: *mut zbar_image_t) -> libc::c_int>,
    pub set_control: Option<unsafe extern "C" fn(_: *mut zbar_video_t,
                                                 _: *const libc::c_char,
                                                 _: *mut libc::c_void)
                                -> libc::c_int>,
    pub get_control: Option<unsafe extern "C" fn(_: *mut zbar_video_t,
                                                 _: *const libc::c_char,
                                                 _: *mut libc::c_void)
                                -> libc::c_int>,
    pub free: Option<unsafe extern "C" fn(_: *mut zbar_video_t) -> ()>,
    pub dq: Option<unsafe extern "C" fn(_: *mut zbar_video_t)
                       -> *mut zbar_image_t>,
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
 *------------------------------------------------------------------------*/
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
 *------------------------------------------------------------------------*/
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
pub type zbar_image_cleanup_handler_t
    =
    unsafe extern "C" fn(_: *mut zbar_image_t) -> ();
/* * display detail for last video error to stderr.
 * @returns a non-zero value suitable for passing to exit()
 */
/* * retrieve the detail string for the last video error. */
/* * retrieve the type code for the last video error. */
/*@}*/
/*------------------------------------------------------------*/
/* * @name Window interface
 * @anchor c-window
 * mid-level output window abstraction.
 * displays images to user-specified platform specific output window
 */
/*@{*/
/* * opaque window object. */
pub type zbar_window_t = zbar_window_s;
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
#[inline]
unsafe extern "C" fn video_unlock(mut vdo: *mut zbar_video_t) -> libc::c_int {
    let mut rc: libc::c_int = 0 as libc::c_int;
    rc = _zbar_mutex_unlock(&mut (*vdo).qlock);
    if rc != 0 {
        err_capture(vdo as *const libc::c_void, SEV_FATAL, ZBAR_ERR_LOCKING,
                    (*::std::mem::transmute::<&[u8; 13],
                                              &[libc::c_char; 13]>(b"video_unlock\x00")).as_ptr(),
                    b"unable to release lock\x00" as *const u8 as
                        *const libc::c_char);
        (*vdo).err.errnum = rc;
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
/* FIXME don't we need varargs hacks here? */
/* unused at src, avoid double free */
#[inline]
unsafe extern "C" fn err_capture(mut container: *const libc::c_void,
                                 mut sev: errsev_t, mut type_0: zbar_error_t,
                                 mut func: *const libc::c_char,
                                 mut detail: *const libc::c_char)
 -> libc::c_int {
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
    if type_0 as libc::c_uint ==
           ZBAR_ERR_SYSTEM as libc::c_int as libc::c_uint {
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
/* FIXME save system code */
    /*rc = err_capture(proc, SEV_ERROR, ZBAR_ERR_LOCKING, __func__,
                       "unable to lock processor");*/
#[inline]
unsafe extern "C" fn _zbar_mutex_unlock(mut lock: *mut zbar_mutex_t)
 -> libc::c_int {
    let mut rc: libc::c_int = pthread_mutex_unlock(lock);
    /* FIXME save system code */
    return rc;
}
#[inline]
unsafe extern "C" fn video_lock(mut vdo: *mut zbar_video_t) -> libc::c_int {
    let mut rc: libc::c_int = 0 as libc::c_int;
    rc = _zbar_mutex_lock(&mut (*vdo).qlock);
    if rc != 0 {
        err_capture(vdo as *const libc::c_void, SEV_FATAL, ZBAR_ERR_LOCKING,
                    (*::std::mem::transmute::<&[u8; 11],
                                              &[libc::c_char; 11]>(b"video_lock\x00")).as_ptr(),
                    b"unable to acquire lock\x00" as *const u8 as
                        *const libc::c_char);
        (*vdo).err.errnum = rc;
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn _zbar_mutex_lock(mut lock: *mut zbar_mutex_t)
 -> libc::c_int {
    let mut rc: libc::c_int = pthread_mutex_lock(lock);
    return rc;
}
#[inline]
unsafe extern "C" fn _zbar_mutex_destroy(mut lock: *mut zbar_mutex_t) {
    pthread_mutex_destroy(lock);
}
#[inline]
unsafe extern "C" fn err_cleanup(mut err: *mut errinfo_t) {
    if (*err).magic == 0x5252457a as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"err->magic == ERRINFO_MAGIC\x00" as *const u8 as
                          *const libc::c_char,
                      b"./zbar/error.h\x00" as *const u8 as
                          *const libc::c_char,
                      218 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 30],
                                                &[libc::c_char; 30]>(b"void err_cleanup(errinfo_t *)\x00")).as_ptr());
    }
    if !(*err).buf.is_null() {
        free((*err).buf as *mut libc::c_void);
        (*err).buf = 0 as *mut libc::c_char
    }
    if !(*err).arg_str.is_null() {
        free((*err).arg_str as *mut libc::c_void);
        (*err).arg_str = 0 as *mut libc::c_char
    };
}
#[inline]
unsafe extern "C" fn _zbar_mutex_init(mut lock: *mut zbar_mutex_t)
 -> libc::c_int {
    return pthread_mutex_init(lock, 0 as *const pthread_mutexattr_t);
}
#[inline]
unsafe extern "C" fn err_init(mut err: *mut errinfo_t,
                              mut module: errmodule_t) {
    (*err).magic = 0x5252457a as libc::c_int as uint32_t;
    (*err).module = module;
}
#[inline]
unsafe extern "C" fn _zbar_image_refcnt(mut img: *mut zbar_image_t,
                                        mut delta: libc::c_int) {
    if _zbar_refcnt(&mut (*img).refcnt, delta) == 0 &&
           delta <= 0 as libc::c_int {
        if (*img).cleanup.is_some() {
            (*img).cleanup.expect("non-null function pointer")(img);
        }
        if (*img).src.is_null() { _zbar_image_free(img); }
    };
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
unsafe extern "C" fn _zbar_video_recycle_image(mut img: *mut zbar_image_t) {
    let mut vdo: *mut zbar_video_t = (*img).src;
    if !vdo.is_null() {
    } else {
        __assert_fail(b"vdo\x00" as *const u8 as *const libc::c_char,
                      b"zbar/video.c\x00" as *const u8 as *const libc::c_char,
                      36 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 47],
                                                &[libc::c_char; 47]>(b"void _zbar_video_recycle_image(zbar_image_t *)\x00")).as_ptr());
    }
    if (*img).srcidx >= 0 as libc::c_int {
    } else {
        __assert_fail(b"img->srcidx >= 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"zbar/video.c\x00" as *const u8 as *const libc::c_char,
                      37 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 47],
                                                &[libc::c_char; 47]>(b"void _zbar_video_recycle_image(zbar_image_t *)\x00")).as_ptr());
    }
    video_lock(vdo);
    if *(*vdo).images.offset((*img).srcidx as isize) != img {
        let ref mut fresh0 = *(*vdo).images.offset((*img).srcidx as isize);
        *fresh0 = img
    }
    if (*vdo).active() != 0 {
        (*vdo).nq.expect("non-null function pointer")(vdo, img);
    } else { video_unlock(vdo); };
}
unsafe extern "C" fn _zbar_video_recycle_shadow(mut img: *mut zbar_image_t) {
    let mut vdo: *mut zbar_video_t = (*img).src;
    if !vdo.is_null() {
    } else {
        __assert_fail(b"vdo\x00" as *const u8 as *const libc::c_char,
                      b"zbar/video.c\x00" as *const u8 as *const libc::c_char,
                      50 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 48],
                                                &[libc::c_char; 48]>(b"void _zbar_video_recycle_shadow(zbar_image_t *)\x00")).as_ptr());
    }
    if (*img).srcidx == -(1 as libc::c_int) {
    } else {
        __assert_fail(b"img->srcidx == -1\x00" as *const u8 as
                          *const libc::c_char,
                      b"zbar/video.c\x00" as *const u8 as *const libc::c_char,
                      51 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 48],
                                                &[libc::c_char; 48]>(b"void _zbar_video_recycle_shadow(zbar_image_t *)\x00")).as_ptr());
    }
    video_lock(vdo);
    (*img).next = (*vdo).shadow_image;
    (*vdo).shadow_image = img;
    video_unlock(vdo);
}
#[no_mangle]
pub unsafe extern "C" fn zbar_video_create() -> *mut zbar_video_t {
    let mut vdo: *mut zbar_video_t =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<zbar_video_t>() as libc::c_ulong) as
            *mut zbar_video_t;
    let mut i: libc::c_int = 0;
    if vdo.is_null() { return 0 as *mut zbar_video_t }
    err_init(&mut (*vdo).err, ZBAR_MOD_VIDEO);
    (*vdo).fd = -(1 as libc::c_int);
    _zbar_mutex_init(&mut (*vdo).qlock);
    /* pre-allocate images */
    (*vdo).num_images = 4 as libc::c_int;
    (*vdo).images =
        calloc(4 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<*mut zbar_image_t>() as libc::c_ulong) as
            *mut *mut zbar_image_t;
    if (*vdo).images.is_null() {
        zbar_video_destroy(vdo);
        return 0 as *mut zbar_video_t
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let ref mut fresh1 = *(*vdo).images.offset(i as isize);
        *fresh1 = zbar_image_create();
        let mut img: *mut zbar_image_t = *fresh1;
        if img.is_null() {
            zbar_video_destroy(vdo);
            return 0 as *mut zbar_video_t
        }
        (*img).refcnt = 0 as libc::c_int;
        (*img).cleanup =
            Some(_zbar_video_recycle_image as
                     unsafe extern "C" fn(_: *mut zbar_image_t) -> ());
        (*img).srcidx = i;
        (*img).src = vdo;
        i += 1
    }
    return vdo;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_video_destroy(mut vdo: *mut zbar_video_t) {
    if (*vdo).intf as libc::c_uint !=
           VIDEO_INVALID as libc::c_int as libc::c_uint {
        zbar_video_open(vdo, 0 as *const libc::c_char);
    }
    if !(*vdo).images.is_null() {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            if !(*(*vdo).images.offset(i as isize)).is_null() {
                _zbar_image_free(*(*vdo).images.offset(i as isize));
            }
            i += 1
        }
        free((*vdo).images as *mut libc::c_void);
    }
    while !(*vdo).shadow_image.is_null() {
        let mut img: *mut zbar_image_t = (*vdo).shadow_image;
        (*vdo).shadow_image = (*img).next;
        free((*img).data as *mut libc::c_void);
        (*img).data = 0 as *const libc::c_void;
        free(img as *mut libc::c_void);
    }
    if !(*vdo).buf.is_null() { free((*vdo).buf); }
    if !(*vdo).formats.is_null() {
        free((*vdo).formats as *mut libc::c_void);
    }
    if !(*vdo).emu_formats.is_null() {
        free((*vdo).emu_formats as *mut libc::c_void);
    }
    if (*vdo).free.is_some() {
        (*vdo).free.expect("non-null function pointer")(vdo);
    }
    err_cleanup(&mut (*vdo).err);
    _zbar_mutex_destroy(&mut (*vdo).qlock);
    free(vdo as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn zbar_video_open(mut vdo: *mut zbar_video_t,
                                         mut dev: *const libc::c_char)
 -> libc::c_int {
    let mut ldev: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rc: libc::c_int = 0;
    zbar_video_enable(vdo, 0 as libc::c_int);
    video_lock(vdo);
    if (*vdo).intf as libc::c_uint !=
           VIDEO_INVALID as libc::c_int as libc::c_uint {
        if (*vdo).cleanup.is_some() {
            (*vdo).cleanup.expect("non-null function pointer")(vdo);
            (*vdo).cleanup = None
        }
        if _zbar_verbosity >= 1 as libc::c_int {
            fprintf(stderr,
                    b"%s: closed camera (fd=%d)\n\x00" as *const u8 as
                        *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 16],
                                              &[libc::c_char; 16]>(b"zbar_video_open\x00")).as_ptr(),
                    (*vdo).fd);
        }
        (*vdo).intf = VIDEO_INVALID
    }
    video_unlock(vdo);
    if dev.is_null() { return 0 as libc::c_int }
    if (*dev.offset(0 as libc::c_int as isize) as libc::c_uchar as
            libc::c_int) < 0x10 as libc::c_int {
        /* default linux device, overloaded for other platforms */
        let mut id: libc::c_int =
            *dev.offset(0 as libc::c_int as isize) as libc::c_int;
        ldev = strdup(b"/dev/video0\x00" as *const u8 as *const libc::c_char);
        dev = ldev;
        *ldev.offset(10 as libc::c_int as isize) =
            ('0' as i32 + id) as libc::c_char
    }
    rc = _zbar_video_open(vdo, dev);
    if !ldev.is_null() { free(ldev as *mut libc::c_void); }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_video_get_fd(mut vdo: *const zbar_video_t)
 -> libc::c_int {
    if (*vdo).intf as libc::c_uint ==
           VIDEO_INVALID as libc::c_int as libc::c_uint {
        return err_capture(vdo as *const libc::c_void, SEV_ERROR,
                           ZBAR_ERR_INVALID,
                           (*::std::mem::transmute::<&[u8; 18],
                                                     &[libc::c_char; 18]>(b"zbar_video_get_fd\x00")).as_ptr(),
                           b"video device not opened\x00" as *const u8 as
                               *const libc::c_char)
    }
    if (*vdo).intf as libc::c_uint !=
           VIDEO_V4L2 as libc::c_int as libc::c_uint {
        return err_capture(vdo as *const libc::c_void, SEV_WARNING,
                           ZBAR_ERR_UNSUPPORTED,
                           (*::std::mem::transmute::<&[u8; 18],
                                                     &[libc::c_char; 18]>(b"zbar_video_get_fd\x00")).as_ptr(),
                           b"video driver does not support polling\x00" as
                               *const u8 as *const libc::c_char)
    }
    return (*vdo).fd;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_video_request_size(mut vdo: *mut zbar_video_t,
                                                 mut width: libc::c_uint,
                                                 mut height: libc::c_uint)
 -> libc::c_int {
    if (*vdo).initialized() != 0 {
        /* FIXME re-init different format? */
        return err_capture(vdo as *const libc::c_void, SEV_ERROR,
                           ZBAR_ERR_INVALID,
                           (*::std::mem::transmute::<&[u8; 24],
                                                     &[libc::c_char; 24]>(b"zbar_video_request_size\x00")).as_ptr(),
                           b"already initialized, unable to resize\x00" as
                               *const u8 as *const libc::c_char)
    }
    (*vdo).width = width;
    (*vdo).height = height;
    if _zbar_verbosity >= 1 as libc::c_int {
        fprintf(stderr,
                b"%s: request size: %d x %d\n\x00" as *const u8 as
                    *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 24],
                                          &[libc::c_char; 24]>(b"zbar_video_request_size\x00")).as_ptr(),
                width, height);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_video_request_interface(mut vdo:
                                                          *mut zbar_video_t,
                                                      mut ver: libc::c_int)
 -> libc::c_int {
    if (*vdo).intf as libc::c_uint !=
           VIDEO_INVALID as libc::c_int as libc::c_uint {
        return err_capture(vdo as *const libc::c_void, SEV_ERROR,
                           ZBAR_ERR_INVALID,
                           (*::std::mem::transmute::<&[u8; 29],
                                                     &[libc::c_char; 29]>(b"zbar_video_request_interface\x00")).as_ptr(),
                           b"device already opened, unable to change interface\x00"
                               as *const u8 as *const libc::c_char)
    }
    (*vdo).intf = ver as video_interface_t;
    if _zbar_verbosity >= 1 as libc::c_int {
        fprintf(stderr,
                b"%s: request interface version %d\n\x00" as *const u8 as
                    *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 29],
                                          &[libc::c_char; 29]>(b"zbar_video_request_interface\x00")).as_ptr(),
                (*vdo).intf as libc::c_uint);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_video_request_iomode(mut vdo: *mut zbar_video_t,
                                                   mut iomode: libc::c_int)
 -> libc::c_int {
    if (*vdo).intf as libc::c_uint !=
           VIDEO_INVALID as libc::c_int as libc::c_uint {
        return err_capture(vdo as *const libc::c_void, SEV_ERROR,
                           ZBAR_ERR_INVALID,
                           (*::std::mem::transmute::<&[u8; 26],
                                                     &[libc::c_char; 26]>(b"zbar_video_request_iomode\x00")).as_ptr(),
                           b"device already opened, unable to change iomode\x00"
                               as *const u8 as *const libc::c_char)
    }
    if iomode < 0 as libc::c_int || iomode > VIDEO_USERPTR as libc::c_int {
        return err_capture(vdo as *const libc::c_void, SEV_ERROR,
                           ZBAR_ERR_INVALID,
                           (*::std::mem::transmute::<&[u8; 26],
                                                     &[libc::c_char; 26]>(b"zbar_video_request_iomode\x00")).as_ptr(),
                           b"invalid iomode requested\x00" as *const u8 as
                               *const libc::c_char)
    }
    (*vdo).iomode = iomode as video_iomode_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_video_get_width(mut vdo: *const zbar_video_t)
 -> libc::c_int {
    return (*vdo).width as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_video_get_height(mut vdo: *const zbar_video_t)
 -> libc::c_int {
    return (*vdo).height as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_video_get_format(mut vdo: *const zbar_video_t)
 -> uint32_t {
    return (*vdo).format;
}
#[inline]
unsafe extern "C" fn video_init_images(mut vdo: *mut zbar_video_t)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    if (*vdo).datalen != 0 {
    } else {
        __assert_fail(b"vdo->datalen\x00" as *const u8 as *const libc::c_char,
                      b"zbar/video.c\x00" as *const u8 as *const libc::c_char,
                      238 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 38],
                                                &[libc::c_char; 38]>(b"int video_init_images(zbar_video_t *)\x00")).as_ptr());
    }
    if (*vdo).iomode as libc::c_uint !=
           VIDEO_MMAP as libc::c_int as libc::c_uint {
        if (*vdo).buf.is_null() {
        } else {
            __assert_fail(b"!vdo->buf\x00" as *const u8 as
                              *const libc::c_char,
                          b"zbar/video.c\x00" as *const u8 as
                              *const libc::c_char,
                          240 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 38],
                                                    &[libc::c_char; 38]>(b"int video_init_images(zbar_video_t *)\x00")).as_ptr());
        }
        (*vdo).buflen =
            ((*vdo).num_images as libc::c_ulong).wrapping_mul((*vdo).datalen);
        (*vdo).buf = calloc(1 as libc::c_int as libc::c_ulong, (*vdo).buflen);
        if (*vdo).buf.is_null() {
            return err_capture(vdo as *const libc::c_void, SEV_FATAL,
                               ZBAR_ERR_NOMEM,
                               (*::std::mem::transmute::<&[u8; 18],
                                                         &[libc::c_char; 18]>(b"video_init_images\x00")).as_ptr(),
                               b"unable to allocate image buffers\x00" as
                                   *const u8 as *const libc::c_char)
        }
        if _zbar_verbosity >= 1 as libc::c_int {
            fprintf(stderr,
                    b"%s: pre-allocated %d %s buffers size=0x%lx\n\x00" as
                        *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 18],
                                              &[libc::c_char; 18]>(b"video_init_images\x00")).as_ptr(),
                    (*vdo).num_images,
                    if (*vdo).iomode as libc::c_uint ==
                           VIDEO_READWRITE as libc::c_int as libc::c_uint {
                        b"READ\x00" as *const u8 as *const libc::c_char
                    } else {
                        b"USERPTR\x00" as *const u8 as *const libc::c_char
                    }, (*vdo).buflen);
        }
    }
    i = 0 as libc::c_int;
    while i < (*vdo).num_images {
        let mut img: *mut zbar_image_t = *(*vdo).images.offset(i as isize);
        (*img).format = (*vdo).format;
        zbar_image_set_size(img, (*vdo).width, (*vdo).height);
        if (*vdo).iomode as libc::c_uint !=
               VIDEO_MMAP as libc::c_int as libc::c_uint {
            let mut offset: libc::c_ulong =
                (i as libc::c_ulong).wrapping_mul((*vdo).datalen);
            (*img).datalen = (*vdo).datalen;
            (*img).data =
                ((*vdo).buf as *mut uint8_t).offset(offset as isize) as
                    *const libc::c_void;
            if _zbar_verbosity >= 2 as libc::c_int {
                fprintf(stderr,
                        b"%s:     [%02d] @%08lx\n\x00" as *const u8 as
                            *const libc::c_char,
                        (*::std::mem::transmute::<&[u8; 18],
                                                  &[libc::c_char; 18]>(b"video_init_images\x00")).as_ptr(),
                        i, offset);
            }
        }
        i += 1
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_video_init(mut vdo: *mut zbar_video_t,
                                         mut fmt: libc::c_ulong)
 -> libc::c_int {
    if (*vdo).initialized() != 0 {
        /* FIXME re-init different format? */
        return err_capture(vdo as *const libc::c_void, SEV_ERROR,
                           ZBAR_ERR_INVALID,
                           (*::std::mem::transmute::<&[u8; 16],
                                                     &[libc::c_char; 16]>(b"zbar_video_init\x00")).as_ptr(),
                           b"already initialized, re-init unimplemented\x00"
                               as *const u8 as *const libc::c_char)
    }
    if (*vdo).init.expect("non-null function pointer")(vdo, fmt as uint32_t)
           != 0 {
        return -(1 as libc::c_int)
    }
    (*vdo).format = fmt as uint32_t;
    if video_init_images(vdo) != 0 { return -(1 as libc::c_int) }
    (*vdo).set_initialized(1 as libc::c_int as libc::c_uint);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_video_enable(mut vdo: *mut zbar_video_t,
                                           mut enable: libc::c_int)
 -> libc::c_int {
    if (*vdo).active() as libc::c_int == enable { return 0 as libc::c_int }
    if enable != 0 {
        if (*vdo).intf as libc::c_uint ==
               VIDEO_INVALID as libc::c_int as libc::c_uint {
            return err_capture(vdo as *const libc::c_void, SEV_ERROR,
                               ZBAR_ERR_INVALID,
                               (*::std::mem::transmute::<&[u8; 18],
                                                         &[libc::c_char; 18]>(b"zbar_video_enable\x00")).as_ptr(),
                               b"video device not opened\x00" as *const u8 as
                                   *const libc::c_char)
        }
        if (*vdo).initialized() == 0 &&
               zbar_negotiate_format(vdo, 0 as *mut zbar_window_t) != 0 {
            return -(1 as libc::c_int)
        }
    }
    if video_lock(vdo) != 0 { return -(1 as libc::c_int) }
    (*vdo).set_active(enable as libc::c_uint);
    if enable != 0 {
        /* enqueue all buffers */
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < (*vdo).num_images {
            if (*vdo).nq.expect("non-null function pointer")(vdo,
                                                             *(*vdo).images.offset(i
                                                                                       as
                                                                                       isize))
                   != 0 ||
                   (i + 1 as libc::c_int) < (*vdo).num_images &&
                       video_lock(vdo) != 0 {
                return -(1 as libc::c_int)
            }
            i += 1
        }
        return (*vdo).start.expect("non-null function pointer")(vdo)
    } else {
        let mut i_0: libc::c_int = 0;
        i_0 = 0 as libc::c_int;
        while i_0 < (*vdo).num_images {
            let ref mut fresh2 = (**(*vdo).images.offset(i_0 as isize)).next;
            *fresh2 = 0 as *mut zbar_image_t;
            i_0 += 1
        }
        (*vdo).dq_image = 0 as *mut zbar_image_t;
        (*vdo).nq_image = (*vdo).dq_image;
        if video_unlock(vdo) != 0 { return -(1 as libc::c_int) }
        return (*vdo).stop.expect("non-null function pointer")(vdo)
    };
}
#[no_mangle]
pub unsafe extern "C" fn zbar_video_next_image(mut vdo: *mut zbar_video_t)
 -> *mut zbar_image_t {
    let mut frame: libc::c_uint = 0;
    let mut img: *mut zbar_image_t = 0 as *mut zbar_image_t;
    if video_lock(vdo) != 0 { return 0 as *mut zbar_image_t }
    if (*vdo).active() == 0 {
        video_unlock(vdo);
        return 0 as *mut zbar_image_t
    }
    let fresh3 = (*vdo).frame;
    (*vdo).frame = (*vdo).frame.wrapping_add(1);
    frame = fresh3;
    img = (*vdo).dq.expect("non-null function pointer")(vdo);
    if !img.is_null() {
        (*img).seq = frame;
        if (*vdo).num_images < 2 as libc::c_int {
            /* return a *copy* of the video image and immediately recycle
             * the driver's buffer to avoid deadlocking the resources
             */
            let mut tmp: *mut zbar_image_t = img;
            video_lock(vdo);
            img = (*vdo).shadow_image;
            (*vdo).shadow_image =
                if !img.is_null() {
                    (*img).next
                } else { 0 as *mut zbar_image_t };
            video_unlock(vdo);
            if img.is_null() {
                img = zbar_image_create();
                if !img.is_null() {
                } else {
                    __assert_fail(b"img\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"zbar/video.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  370 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 52],
                                                            &[libc::c_char; 52]>(b"zbar_image_t *zbar_video_next_image(zbar_video_t *)\x00")).as_ptr());
                }
                (*img).refcnt = 0 as libc::c_int;
                (*img).src = vdo;
                /* recycle the shadow images */
                (*img).format = (*vdo).format;
                zbar_image_set_size(img, (*vdo).width, (*vdo).height);
                (*img).datalen = (*vdo).datalen;
                (*img).data = malloc((*vdo).datalen)
            }
            (*img).cleanup =
                Some(_zbar_video_recycle_shadow as
                         unsafe extern "C" fn(_: *mut zbar_image_t) -> ());
            (*img).seq = frame;
            memcpy((*img).data as *mut libc::c_void, (*tmp).data,
                   (*img).datalen);
            _zbar_video_recycle_image(tmp);
        } else {
            (*img).cleanup =
                Some(_zbar_video_recycle_image as
                         unsafe extern "C" fn(_: *mut zbar_image_t) -> ())
        }
        _zbar_image_refcnt(img, 1 as libc::c_int);
    }
    return img;
}
/* * @brief return if fun unsupported, otherwise continue */
#[no_mangle]
pub unsafe extern "C" fn zbar_video_set_control(mut vdo: *mut zbar_video_t,
                                                mut control_name:
                                                    *const libc::c_char,
                                                mut value: libc::c_int)
 -> libc::c_int {
    let mut loc_value: libc::c_int = 0;
    let mut rv: libc::c_int = 0;
    if (*vdo).set_control.is_none() {
        if _zbar_verbosity >= 1 as libc::c_int {
            fprintf(stderr,
                    b"%s: video driver does not implement %s\n\x00" as
                        *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 23],
                                              &[libc::c_char; 23]>(b"zbar_video_set_control\x00")).as_ptr(),
                    b"set_control\x00" as *const u8 as *const libc::c_char);
        }
        return ZBAR_ERR_UNSUPPORTED as libc::c_int
    }
    loc_value = value;
    rv =
        (*vdo).set_control.expect("non-null function pointer")(vdo,
                                                               control_name,
                                                               &mut loc_value
                                                                   as
                                                                   *mut libc::c_int
                                                                   as
                                                                   *mut libc::c_void);
    if rv == 0 as libc::c_int {
        if _zbar_verbosity >= 1 as libc::c_int {
            fprintf(stderr,
                    b"%s: value of %s set to: %d\n\x00" as *const u8 as
                        *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 23],
                                              &[libc::c_char; 23]>(b"zbar_video_set_control\x00")).as_ptr(),
                    control_name, loc_value);
        }
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_video_get_control(mut vdo: *mut zbar_video_t,
                                                mut control_name:
                                                    *const libc::c_char,
                                                mut value: *mut libc::c_int)
 -> libc::c_int {
    if (*vdo).get_control.is_none() {
        if _zbar_verbosity >= 1 as libc::c_int {
            fprintf(stderr,
                    b"%s: video driver does not implement %s\n\x00" as
                        *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 23],
                                              &[libc::c_char; 23]>(b"zbar_video_get_control\x00")).as_ptr(),
                    b"get_control\x00" as *const u8 as *const libc::c_char);
        }
        return ZBAR_ERR_UNSUPPORTED as libc::c_int
    }
    return (*vdo).get_control.expect("non-null function pointer")(vdo,
                                                                  control_name,
                                                                  value as
                                                                      *mut libc::c_void);
}
/* * display detail for last processor error to stderr.
 * @returns a non-zero value suitable for passing to exit()
 */
/* * retrieve the detail string for the last processor error. */
/* * retrieve the type code for the last processor error. */
/*@}*/
/*------------------------------------------------------------*/
/* * @name Video interface
 * @anchor c-video
 * mid-level video source abstraction.
 * captures images from a video device
 */
/*@{*/
/* * opaque video object. */
/* * constructor. */
/* * destructor. */
/* * open and probe a video device.
 * the device specified by platform specific unique name
 * (v4l device node path in *nix eg "/dev/video",
 *  DirectShow DevicePath property in windows).
 * @returns 0 if successful or -1 if an error occurs
 */
/* * retrieve file descriptor associated with open *nix video device
 * useful for using select()/poll() to tell when new images are
 * available (NB v4l2 only!!).
 * @returns the file descriptor or -1 if the video device is not open
 * or the driver only supports v4l1
 */
/* * request a preferred size for the video image from the device.
 * the request may be adjusted or completely ignored by the driver.
 * @returns 0 if successful or -1 if the video device is already
 * initialized
 * @since 0.6
 */
/* * request a preferred driver interface version for debug/testing.
 * @note must be called before zbar_video_open()
 * @since 0.6
 */
/* * request a preferred I/O mode for debug/testing.  You will get
 * errors if the driver does not support the specified mode.
 * @verbatim
    0 = auto-detect
    1 = force I/O using read()
    2 = force memory mapped I/O using mmap()
    3 = force USERPTR I/O (v4l2 only)
@endverbatim
 * @note must be called before zbar_video_open()
 * @since 0.7
 */
/* * retrieve current output image width.
 * @returns the width or 0 if the video device is not open
 */
/* * retrieve current output image height.
 * @returns the height or 0 if the video device is not open
 */
/* * initialize video using a specific format for debug.
 * use zbar_negotiate_format() to automatically select and initialize
 * the best available format
 */
/* * start/stop video capture.
 * all buffered images are retired when capture is disabled.
 * @returns 0 if successful or -1 if an error occurs
 */
/* * retrieve next captured image.  blocks until an image is available.
 * @returns NULL if video is not enabled or an error occurs
 */
/* * set video control value (integer).
 * @returns 0 for success, non-0 for failure
 * @since 0.20
 * @see zbar_processor_set_control()
 */
/* * get video control value (integer).
 * @returns 0 for success, non-0 for failure
 * @since 0.20
 * @see zbar_processor_get_control()
 */
/* * get available controls from video source
 * @returns 0 for success, non-0 for failure
 * @since 0.20
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_video_get_controls(mut vdo: *const zbar_video_t,
                                                 mut index: libc::c_int)
 -> *mut video_controls_s {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut p: *mut video_controls_s = (*vdo).controls;
    while !p.is_null() && i != index {
        i += 1;
        p = (*p).next as *mut video_controls_s
    }
    if p.is_null() { return 0 as *mut video_controls_s }
    return p;
}
