use ::libc;
extern "C" {
    pub type zbar_video_s;
    pub type processor_state_s;
    pub type zbar_image_scanner_s;
    pub type window_state_s;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn zbar_get_symbol_name(sym: zbar_symbol_type_t) -> *const libc::c_char;
    #[no_mangle]
    fn _zbar_error_spew(object: *const libc::c_void, verbosity: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn zbar_symbol_get_type(symbol: *const zbar_symbol_t)
     -> zbar_symbol_type_t;
    #[no_mangle]
    fn zbar_symbol_get_data(symbol: *const zbar_symbol_t)
     -> *const libc::c_char;
    #[no_mangle]
    fn zbar_symbol_get_quality(symbol: *const zbar_symbol_t) -> libc::c_int;
    #[no_mangle]
    fn zbar_symbol_get_count(symbol: *const zbar_symbol_t) -> libc::c_int;
    #[no_mangle]
    fn zbar_symbol_get_loc_size(symbol: *const zbar_symbol_t) -> libc::c_uint;
    #[no_mangle]
    fn zbar_symbol_get_orientation(symbol: *const zbar_symbol_t)
     -> zbar_orientation_t;
    #[no_mangle]
    fn zbar_symbol_next(symbol: *const zbar_symbol_t) -> *const zbar_symbol_t;
    #[no_mangle]
    fn zbar_symbol_set_ref(symbols: *const zbar_symbol_set_t,
                           refs: libc::c_int);
    #[no_mangle]
    fn zbar_image_destroy(image: *mut zbar_image_t);
    #[no_mangle]
    fn zbar_image_convert(image: *const zbar_image_t, format: libc::c_ulong)
     -> *mut zbar_image_t;
    #[no_mangle]
    fn zbar_image_get_format(image: *const zbar_image_t) -> libc::c_ulong;
    #[no_mangle]
    fn zbar_image_get_width(image: *const zbar_image_t) -> libc::c_uint;
    #[no_mangle]
    fn zbar_image_get_height(image: *const zbar_image_t) -> libc::c_uint;
    #[no_mangle]
    fn zbar_image_get_data(image: *const zbar_image_t) -> *const libc::c_void;
    #[no_mangle]
    fn zbar_image_first_symbol(image: *const zbar_image_t)
     -> *const zbar_symbol_t;
    #[no_mangle]
    fn zbar_image_write(image: *const zbar_image_t,
                        filebase: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn _zbar_processor_init(_: *mut zbar_processor_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_init(__mutex: *mut pthread_mutex_t,
                          __mutexattr: *const pthread_mutexattr_t)
     -> libc::c_int;
    #[no_mangle]
    fn zbar_image_scanner_create() -> *mut zbar_image_scanner_t;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn _zbar_event_destroy(_: *mut zbar_event_t);
    #[no_mangle]
    fn _zbar_processor_cleanup(_: *mut zbar_processor_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn zbar_image_scanner_destroy(scanner: *mut zbar_image_scanner_t);
    #[no_mangle]
    fn _zbar_processor_unlock(_: *mut zbar_processor_t, _: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    static mut _zbar_verbosity: libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn zbar_negotiate_format(video: *mut zbar_video_t,
                             window: *mut zbar_window_t) -> libc::c_int;
    #[no_mangle]
    fn zbar_video_init(video: *mut zbar_video_t, format: libc::c_ulong)
     -> libc::c_int;
    /* windowing platform API */
    #[no_mangle]
    fn _zbar_processor_open(_: *mut zbar_processor_t, _: *mut libc::c_char,
                            _: libc::c_uint, _: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn zbar_video_get_height(video: *const zbar_video_t) -> libc::c_int;
    #[no_mangle]
    fn zbar_video_get_width(video: *const zbar_video_t) -> libc::c_int;
    #[no_mangle]
    fn zbar_video_get_fd(video: *const zbar_video_t) -> libc::c_int;
    #[no_mangle]
    fn _zbar_event_trigger(_: *mut zbar_event_t);
    #[no_mangle]
    fn _zbar_processor_close(_: *mut zbar_processor_t) -> libc::c_int;
    #[no_mangle]
    fn _zbar_processor_input_wait(_: *mut zbar_processor_t,
                                  _: *mut zbar_event_t, _: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn _zbar_thread_start(_: *mut zbar_thread_t,
                          _: Option<zbar_thread_proc_t>, _: *mut libc::c_void,
                          _: *mut zbar_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn zbar_video_next_image(video: *mut zbar_video_t) -> *mut zbar_image_t;
    #[no_mangle]
    fn _zbar_processor_invalidate(_: *mut zbar_processor_t) -> libc::c_int;
    #[no_mangle]
    fn zbar_window_draw(window: *mut zbar_window_t, image: *mut zbar_image_t)
     -> libc::c_int;
    #[no_mangle]
    fn _zbar_processor_notify(_: *mut zbar_processor_t, _: libc::c_uint);
    #[no_mangle]
    fn zbar_scan_image(scanner: *mut zbar_image_scanner_t,
                       image: *mut zbar_image_t) -> libc::c_int;
    #[no_mangle]
    fn zbar_image_scanner_get_results(scanner: *const zbar_image_scanner_t)
     -> *const zbar_symbol_set_t;
    #[no_mangle]
    fn zbar_image_scanner_recycle_image(scanner: *mut zbar_image_scanner_t,
                                        image: *mut zbar_image_t);
    #[no_mangle]
    fn _zbar_processor_lock(_: *mut zbar_processor_t) -> libc::c_int;
    #[no_mangle]
    fn _zbar_event_wait(_: *mut zbar_event_t, _: *mut zbar_mutex_t,
                        _: *mut zbar_timer_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_sigmask(__how: libc::c_int, __newmask: *const __sigset_t,
                       __oldmask: *mut __sigset_t) -> libc::c_int;
    #[no_mangle]
    fn sigfillset(__set: *mut sigset_t) -> libc::c_int;
    #[no_mangle]
    fn zbar_video_open(video: *mut zbar_video_t, device: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn zbar_video_request_iomode(video: *mut zbar_video_t,
                                 iomode: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn zbar_video_request_interface(video: *mut zbar_video_t,
                                    version: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn zbar_video_request_size(video: *mut zbar_video_t, width: libc::c_uint,
                               height: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn zbar_video_create() -> *mut zbar_video_t;
    #[no_mangle]
    fn zbar_window_create() -> *mut zbar_window_t;
    #[no_mangle]
    fn zbar_video_destroy(video: *mut zbar_video_t);
    #[no_mangle]
    fn zbar_window_destroy(window: *mut zbar_window_t);
    #[no_mangle]
    fn _zbar_thread_stop(_: *mut zbar_thread_t, _: *mut zbar_mutex_t)
     -> libc::c_int;
    #[no_mangle]
    fn _zbar_processor_enable(_: *mut zbar_processor_t) -> libc::c_int;
    #[no_mangle]
    fn zbar_video_enable(video: *mut zbar_video_t, enable: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn zbar_image_scanner_enable_cache(scanner: *mut zbar_image_scanner_t,
                                       enable: libc::c_int);
    #[no_mangle]
    fn zbar_image_scanner_set_config(scanner: *mut zbar_image_scanner_t,
                                     symbology: zbar_symbol_type_t,
                                     config: zbar_config_t,
                                     value: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn zbar_video_set_control(video: *mut zbar_video_t,
                              control_name: *const libc::c_char,
                              value: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn zbar_video_get_control(video: *mut zbar_video_t,
                              control_name: *const libc::c_char,
                              value: *mut libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn _zbar_processor_set_visible(_: *mut zbar_processor_t, _: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn _zbar_processor_set_size(_: *mut zbar_processor_t, _: libc::c_uint,
                                _: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec)
     -> libc::c_int;
    #[no_mangle]
    fn _zbar_processor_wait(_: *mut zbar_processor_t, _: libc::c_uint,
                            _: *mut zbar_timer_t) -> libc::c_int;
    #[no_mangle]
    fn zbar_window_set_overlay(window: *mut zbar_window_t,
                               level: libc::c_int);
    #[no_mangle]
    fn zbar_window_get_overlay(window: *const zbar_window_t) -> libc::c_int;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
}
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type clockid_t = __clockid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub struct __pthread_cond_s {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub c2rust_unnamed_0: C2RustUnnamed,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
pub type pthread_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
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
pub type zbar_symbol_type_t = zbar_symbol_type_e;
pub type zbar_orientation_e = libc::c_int;
pub const ZBAR_ORIENT_LEFT: zbar_orientation_e = 3;
pub const ZBAR_ORIENT_DOWN: zbar_orientation_e = 2;
pub const ZBAR_ORIENT_RIGHT: zbar_orientation_e = 1;
pub const ZBAR_ORIENT_UP: zbar_orientation_e = 0;
pub const ZBAR_ORIENT_UNKNOWN: zbar_orientation_e = -1;
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
pub type zbar_error_t = zbar_error_e;
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
pub type zbar_config_t = zbar_config_e;
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
 *------------------------------------------------------------------------*/
pub type refcnt_t = libc::c_int;
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
pub type point_t = point_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point_s {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
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
pub type zbar_video_t = zbar_video_s;
pub type zbar_image_cleanup_handler_t
    =
    unsafe extern "C" fn(_: *mut zbar_image_t) -> ();
pub type zbar_image_data_handler_t
    =
    unsafe extern "C" fn(_: *mut zbar_image_t, _: *const libc::c_void) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zbar_processor_s {
    pub err: errinfo_t,
    pub userdata: *const libc::c_void,
    pub video: *mut zbar_video_t,
    pub window: *mut zbar_window_t,
    pub scanner: *mut zbar_image_scanner_t,
    pub handler: Option<zbar_image_data_handler_t>,
    pub req_width: libc::c_uint,
    pub req_height: libc::c_uint,
    pub req_intf: libc::c_int,
    pub req_iomode: libc::c_int,
    pub force_input: uint32_t,
    pub force_output: uint32_t,
    pub input: libc::c_int,
    pub threaded: libc::c_int,
    pub visible: libc::c_int,
    pub streaming: libc::c_int,
    pub dumping: libc::c_int,
    pub display: *mut libc::c_void,
    pub xwin: libc::c_ulong,
    pub input_thread: zbar_thread_t,
    pub video_thread: zbar_thread_t,
    pub syms: *const zbar_symbol_set_t,
    pub mutex: zbar_mutex_t,
    pub lock_level: libc::c_int,
    pub lock_owner: zbar_thread_id_t,
    pub wait_head: *mut proc_waiter_t,
    pub wait_tail: *mut proc_waiter_t,
    pub wait_next: *mut proc_waiter_t,
    pub free_waiter: *mut proc_waiter_t,
    pub state: *mut processor_state_t,
}
pub type processor_state_t = processor_state_s;
pub type proc_waiter_t = proc_waiter_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct proc_waiter_s {
    pub next: *mut proc_waiter_s,
    pub notify: zbar_event_t,
    pub requester: zbar_thread_id_t,
    pub events: libc::c_uint,
}
pub type zbar_thread_id_t = pthread_t;
pub type zbar_event_t = zbar_event_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zbar_event_s {
    pub state: libc::c_int,
    pub cond: pthread_cond_t,
    pub pollfd: libc::c_int,
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
/* simple platform mutex abstraction
 */
pub type zbar_mutex_t = pthread_mutex_t;
pub type zbar_thread_t = zbar_thread_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zbar_thread_s {
    pub tid: zbar_thread_id_t,
    pub started: libc::c_int,
    pub running: libc::c_int,
    pub notify: zbar_event_t,
    pub activity: zbar_event_t,
}
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
pub type zbar_window_t = zbar_window_s;
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
    pub init: Option<unsafe extern "C" fn(_: *mut zbar_window_t,
                                          _: *mut zbar_image_t,
                                          _: libc::c_int) -> libc::c_int>,
    pub draw_image: Option<unsafe extern "C" fn(_: *mut zbar_window_t,
                                                _: *mut zbar_image_t)
                               -> libc::c_int>,
    pub cleanup: Option<unsafe extern "C" fn(_: *mut zbar_window_t)
                            -> libc::c_int>,
}
pub type window_state_t = window_state_s;
pub type errinfo_t = errinfo_s;
/* single integer argument */
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
pub type zbar_processor_t = zbar_processor_s;
pub type zbar_timer_t = timespec;
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
pub type zbar_thread_proc_t
    =
    unsafe extern "C" fn(_: *mut libc::c_void) -> *mut libc::c_void;
#[inline]
unsafe extern "C" fn _zbar_mutex_init(mut lock: *mut zbar_mutex_t)
 -> libc::c_int {
    return pthread_mutex_init(lock, 0 as *const pthread_mutexattr_t);
}
/* FIXME don't we need varargs hacks here? */
/* unused at src, avoid double free */
#[inline]
unsafe extern "C" fn err_init(mut err: *mut errinfo_t,
                              mut module: errmodule_t) {
    (*err).magic = 0x5252457a as libc::c_int as uint32_t;
    (*err).module = module;
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
unsafe extern "C" fn _zbar_mutex_destroy(mut lock: *mut zbar_mutex_t) {
    pthread_mutex_destroy(lock);
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
unsafe extern "C" fn _zbar_mutex_lock(mut lock: *mut zbar_mutex_t)
 -> libc::c_int {
    let mut rc: libc::c_int = pthread_mutex_lock(lock);
    return rc;
}
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
#[inline]
unsafe extern "C" fn err_copy(mut dst_c: *mut libc::c_void,
                              mut src_c: *mut libc::c_void) -> libc::c_int {
    let mut dst: *mut errinfo_t = dst_c as *mut errinfo_t;
    let mut src: *mut errinfo_t = src_c as *mut errinfo_t;
    if (*dst).magic == 0x5252457a as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"dst->magic == ERRINFO_MAGIC\x00" as *const u8 as
                          *const libc::c_char,
                      b"./zbar/error.h\x00" as *const u8 as
                          *const libc::c_char,
                      129 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 29],
                                                &[libc::c_char; 29]>(b"int err_copy(void *, void *)\x00")).as_ptr());
    }
    if (*src).magic == 0x5252457a as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"src->magic == ERRINFO_MAGIC\x00" as *const u8 as
                          *const libc::c_char,
                      b"./zbar/error.h\x00" as *const u8 as
                          *const libc::c_char,
                      130 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 29],
                                                &[libc::c_char; 29]>(b"int err_copy(void *, void *)\x00")).as_ptr());
    }
    (*dst).errnum = (*src).errnum;
    (*dst).sev = (*src).sev;
    (*dst).type_0 = (*src).type_0;
    (*dst).func = (*src).func;
    (*dst).detail = (*src).detail;
    (*dst).arg_str = (*src).arg_str;
    (*src).arg_str = 0 as *mut libc::c_char;
    (*dst).arg_int = (*src).arg_int;
    return -(1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn _zbar_thread_init(mut thr: *mut zbar_thread_t) {
    let mut sigs: sigset_t = sigset_t{__val: [0; 16],};
    sigfillset(&mut sigs);
    pthread_sigmask(0 as libc::c_int, &mut sigs, 0 as *mut __sigset_t);
    (*thr).running = 1 as libc::c_int;
    _zbar_event_trigger(&mut (*thr).activity);
}
#[inline]
unsafe extern "C" fn _zbar_timer_init(mut timer: *mut zbar_timer_t,
                                      mut delay: libc::c_int)
 -> *mut zbar_timer_t {
    if delay < 0 as libc::c_int { return 0 as *mut zbar_timer_t }
    clock_gettime(0 as libc::c_int, timer);
    (*timer).tv_nsec +=
        (delay % 1000 as libc::c_int * 1000000 as libc::c_int) as
            libc::c_long;
    (*timer).tv_sec +=
        (delay / 1000 as libc::c_int) as libc::c_long +
            (*timer).tv_nsec / 1000000000 as libc::c_int as libc::c_long;
    (*timer).tv_nsec %= 1000000000 as libc::c_int as libc::c_long;
    return timer;
}
#[inline]
unsafe extern "C" fn _zbar_image_swap_symbols(mut a: *mut zbar_image_t,
                                              mut b: *mut zbar_image_t) {
    let mut tmp: *mut zbar_symbol_set_t = (*a).syms;
    (*a).syms = (*b).syms;
    (*b).syms = tmp;
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
#[inline]
unsafe extern "C" fn proc_enter(mut proc_0: *mut zbar_processor_t)
 -> libc::c_int {
    _zbar_mutex_lock(&mut (*proc_0).mutex);
    return _zbar_processor_lock(proc_0);
}
#[inline]
unsafe extern "C" fn proc_leave(mut proc_0: *mut zbar_processor_t)
 -> libc::c_int {
    let mut rc: libc::c_int =
        _zbar_processor_unlock(proc_0, 0 as libc::c_int);
    _zbar_mutex_unlock(&mut (*proc_0).mutex);
    return rc;
}
#[inline]
unsafe extern "C" fn proc_open(mut proc_0: *mut zbar_processor_t)
 -> libc::c_int {
    /* arbitrary default */
    let mut width: libc::c_int = 640 as libc::c_int;
    let mut height: libc::c_int = 480 as libc::c_int;
    if !(*proc_0).video.is_null() {
        width = zbar_video_get_width((*proc_0).video);
        height = zbar_video_get_height((*proc_0).video)
    }
    return _zbar_processor_open(proc_0,
                                b"zbar barcode reader\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                                width as libc::c_uint,
                                height as libc::c_uint);
}
/* API lock is already held */
#[no_mangle]
pub unsafe extern "C" fn _zbar_process_image(mut proc_0:
                                                 *mut zbar_processor_t,
                                             mut img: *mut zbar_image_t)
 -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut current_block: u64;
    let mut force_fmt: uint32_t = (*proc_0).force_output;
    if !img.is_null() {
        if (*proc_0).dumping != 0 {
            zbar_image_write((*(*proc_0).window).image,
                             b"zbar\x00" as *const u8 as *const libc::c_char);
            (*proc_0).dumping = 0 as libc::c_int
        }
        let mut format: uint32_t = zbar_image_get_format(img) as uint32_t;
        if _zbar_verbosity >= 16 as libc::c_int {
            fprintf(stderr,
                    b"%s: processing: %.4s(%08x) %dx%d @%p\n\x00" as *const u8
                        as *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 20],
                                              &[libc::c_char; 20]>(b"_zbar_process_image\x00")).as_ptr(),
                    &mut format as *mut uint32_t as *mut libc::c_char, format,
                    zbar_image_get_width(img), zbar_image_get_height(img),
                    zbar_image_get_data(img));
        }
        /* FIXME locking all other interfaces while processing is conservative
         * but easier for now and we don't expect this to take long...
         */
        let mut tmp: *mut zbar_image_t =
            zbar_image_convert(img,
                               'Y' as i32 as libc::c_ulong |
                                   ('8' as i32 as libc::c_ulong) <<
                                       8 as libc::c_int |
                                   ('0' as i32 as libc::c_ulong) <<
                                       16 as libc::c_int |
                                   ('0' as i32 as libc::c_ulong) <<
                                       24 as libc::c_int);
        if tmp.is_null() {
            current_block = 9662623937613065487;
        } else {
            if !(*proc_0).syms.is_null() {
                zbar_symbol_set_ref((*proc_0).syms, -(1 as libc::c_int));
                (*proc_0).syms = 0 as *const zbar_symbol_set_t
            }
            zbar_image_scanner_recycle_image((*proc_0).scanner, img);
            let mut nsyms: libc::c_int =
                zbar_scan_image((*proc_0).scanner, tmp);
            _zbar_image_swap_symbols(img, tmp);
            zbar_image_destroy(tmp);
            tmp = 0 as *mut zbar_image_t;
            if nsyms < 0 as libc::c_int {
                current_block = 9662623937613065487;
            } else {
                (*proc_0).syms =
                    zbar_image_scanner_get_results((*proc_0).scanner);
                if !(*proc_0).syms.is_null() {
                    zbar_symbol_set_ref((*proc_0).syms, 1 as libc::c_int);
                }
                if _zbar_verbosity >= 8 as libc::c_int {
                    let mut sym: *const zbar_symbol_t =
                        zbar_image_first_symbol(img);
                    while !sym.is_null() {
                        let mut type_0: zbar_symbol_type_t =
                            zbar_symbol_get_type(sym);
                        let mut count: libc::c_int =
                            zbar_symbol_get_count(sym);
                        if _zbar_verbosity >= 8 as libc::c_int {
                            fprintf(stderr,
                                    b"%s: %s: %s (%d pts) (dir=%d) (q=%d) (%s)\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    (*::std::mem::transmute::<&[u8; 20],
                                                              &[libc::c_char; 20]>(b"_zbar_process_image\x00")).as_ptr(),
                                    zbar_get_symbol_name(type_0),
                                    zbar_symbol_get_data(sym),
                                    zbar_symbol_get_loc_size(sym),
                                    zbar_symbol_get_orientation(sym) as
                                        libc::c_int,
                                    zbar_symbol_get_quality(sym),
                                    if count < 0 as libc::c_int {
                                        b"uncertain\x00" as *const u8 as
                                            *const libc::c_char
                                    } else if count > 0 as libc::c_int {
                                        b"duplicate\x00" as *const u8 as
                                            *const libc::c_char
                                    } else {
                                        b"new\x00" as *const u8 as
                                            *const libc::c_char
                                    });
                        }
                        sym = zbar_symbol_next(sym)
                    }
                }
                if nsyms != 0 {
                    /* FIXME only call after filtering */
                    _zbar_mutex_lock(&mut (*proc_0).mutex);
                    _zbar_processor_notify(proc_0,
                                           0x2 as libc::c_int as
                                               libc::c_uint);
                    _zbar_mutex_unlock(&mut (*proc_0).mutex);
                    if (*proc_0).handler.is_some() {
                        (*proc_0).handler.expect("non-null function pointer")(img,
                                                                              (*proc_0).userdata);
                    }
                }
                if force_fmt != 0 {
                    let mut syms: *mut zbar_symbol_set_t = (*img).syms;
                    img = zbar_image_convert(img, force_fmt as libc::c_ulong);
                    if img.is_null() {
                        current_block = 9662623937613065487;
                    } else {
                        (*img).syms = syms;
                        zbar_symbol_set_ref(syms, 1 as libc::c_int);
                        current_block = 12497913735442871383;
                    }
                } else { current_block = 12497913735442871383; }
            }
        }
        match current_block {
            12497913735442871383 => { }
            _ => {
                return err_capture(proc_0 as *const libc::c_void, SEV_ERROR,
                                   ZBAR_ERR_UNSUPPORTED,
                                   (*::std::mem::transmute::<&[u8; 20],
                                                             &[libc::c_char; 20]>(b"_zbar_process_image\x00")).as_ptr(),
                                   b"unknown image format\x00" as *const u8 as
                                       *const libc::c_char)
            }
        }
    }
    /* display to window if enabled */
    rc = 0 as libc::c_int;
    if !(*proc_0).window.is_null() {
        rc = zbar_window_draw((*proc_0).window, img);
        if rc != 0 {
            err_copy(proc_0 as *mut libc::c_void,
                     (*proc_0).window as *mut libc::c_void);
        }
        _zbar_processor_invalidate(proc_0);
    }
    if force_fmt != 0 && !img.is_null() { zbar_image_destroy(img); }
    return rc;
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
/* max time to wait for input before looking for the next frame.
 * only used in unthreaded mode with blocking (non-pollable) video.
 * NB subject to precision of whatever timer is in use
 */
/*ms*/
/* platform specific state wrapper */
/* specific notification tracking */
/* high-level API events */
/* user input */
/* decoded output data available */
/* cancelation flag */
/* error reporting */
/* application data */
/* input video device abstraction */
/* output window abstraction */
/* barcode scanner */
/* application data handler */
/* application requested video size */
/* application requested interface */
/* force input format (debug) */
/* force format conversion (debug) */
/* user input status */
/* state flags */
/* output window mapped to display */
/* video enabled */
/* debug image dump */
/* X display connection */
/* toplevel window */
/* video input handler */
/* window event handler */
/* previous decode results */
/* shared data mutex */
/* API serialization lock */
/* processor lock API */
/* platform API */
#[no_mangle]
pub unsafe extern "C" fn _zbar_processor_handle_input(mut proc_0:
                                                          *mut zbar_processor_t,
                                                      mut input: libc::c_int)
 -> libc::c_int {
    let mut event: libc::c_int = 0x1 as libc::c_int;
    match input {
        -1 => {
            event |= 0x80 as libc::c_int;
            _zbar_processor_set_visible(proc_0, 0 as libc::c_int);
            err_capture(proc_0 as *const libc::c_void, SEV_WARNING,
                        ZBAR_ERR_CLOSED,
                        (*::std::mem::transmute::<&[u8; 29],
                                                  &[libc::c_char; 29]>(b"_zbar_processor_handle_input\x00")).as_ptr(),
                        b"user closed display window\x00" as *const u8 as
                            *const libc::c_char);
        }
        100 => {
            (*proc_0).dumping = 1 as libc::c_int;
            return 0 as libc::c_int
        }
        43 | 61 => {
            if !(*proc_0).window.is_null() {
                let mut ovl: libc::c_int =
                    zbar_window_get_overlay((*proc_0).window);
                zbar_window_set_overlay((*proc_0).window,
                                        ovl + 1 as libc::c_int);
            }
        }
        45 => {
            if !(*proc_0).window.is_null() {
                let mut ovl_0: libc::c_int =
                    zbar_window_get_overlay((*proc_0).window);
                zbar_window_set_overlay((*proc_0).window,
                                        ovl_0 - 1 as libc::c_int);
            }
        }
        _ => { }
    }
    _zbar_mutex_lock(&mut (*proc_0).mutex);
    (*proc_0).input = input;
    if input == -(1 as libc::c_int) && (*proc_0).visible != 0 &&
           (*proc_0).streaming != 0 {
        /* also cancel outstanding output waiters */
        event |= 0x2 as libc::c_int
    }
    _zbar_processor_notify(proc_0, event as libc::c_uint);
    _zbar_mutex_unlock(&mut (*proc_0).mutex);
    return input;
}
unsafe extern "C" fn proc_video_thread(mut arg: *mut libc::c_void)
 -> *mut libc::c_void {
    let mut proc_0: *mut zbar_processor_t = arg as *mut zbar_processor_t;
    let mut thread: *mut zbar_thread_t = &mut (*proc_0).video_thread;
    _zbar_mutex_lock(&mut (*proc_0).mutex);
    _zbar_thread_init(thread);
    if _zbar_verbosity >= 4 as libc::c_int {
        fprintf(stderr,
                b"%s: spawned video thread\n\x00" as *const u8 as
                    *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 18],
                                          &[libc::c_char; 18]>(b"proc_video_thread\x00")).as_ptr());
    }
    while (*thread).started != 0 {
        /* wait for video stream to be active */
        while (*thread).started != 0 && (*proc_0).streaming == 0 {
            _zbar_event_wait(&mut (*thread).notify, &mut (*proc_0).mutex,
                             0 as *mut zbar_timer_t);
        }
        if (*thread).started == 0 { break ; }
        /* blocking capture image from video */
        _zbar_mutex_unlock(&mut (*proc_0).mutex);
        let mut img: *mut zbar_image_t =
            zbar_video_next_image((*proc_0).video);
        _zbar_mutex_lock(&mut (*proc_0).mutex);
        if img.is_null() && (*proc_0).streaming == 0 { continue ; }
        if img.is_null() { break ; }
        /* acquire API lock */
        _zbar_processor_lock(proc_0);
        _zbar_mutex_unlock(&mut (*proc_0).mutex);
        if (*thread).started != 0 && (*proc_0).streaming != 0 {
            _zbar_process_image(proc_0, img);
        }
        zbar_image_destroy(img);
        _zbar_mutex_lock(&mut (*proc_0).mutex);
        /* release API lock */
        _zbar_processor_unlock(proc_0, 0 as libc::c_int);
    }
    (*thread).running = 0 as libc::c_int;
    _zbar_event_trigger(&mut (*thread).activity);
    _zbar_mutex_unlock(&mut (*proc_0).mutex);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn proc_input_thread(mut arg: *mut libc::c_void)
 -> *mut libc::c_void {
    let mut rc: libc::c_int = 0;
    let mut proc_0: *mut zbar_processor_t = arg as *mut zbar_processor_t;
    let mut thread: *mut zbar_thread_t = &mut (*proc_0).input_thread;
    if !(!(*proc_0).window.is_null() && proc_open(proc_0) != 0) {
        _zbar_mutex_lock(&mut (*proc_0).mutex);
        (*thread).running = 1 as libc::c_int;
        _zbar_event_trigger(&mut (*thread).activity);
        if _zbar_verbosity >= 4 as libc::c_int {
            fprintf(stderr,
                    b"%s: spawned input thread\n\x00" as *const u8 as
                        *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 18],
                                              &[libc::c_char; 18]>(b"proc_input_thread\x00")).as_ptr());
        }
        rc = 0 as libc::c_int;
        while (*thread).started != 0 && rc >= 0 as libc::c_int {
            _zbar_mutex_unlock(&mut (*proc_0).mutex);
            rc =
                _zbar_processor_input_wait(proc_0, &mut (*thread).notify,
                                           -(1 as libc::c_int));
            _zbar_mutex_lock(&mut (*proc_0).mutex);
        }
        _zbar_mutex_unlock(&mut (*proc_0).mutex);
        _zbar_processor_close(proc_0);
        _zbar_mutex_lock(&mut (*proc_0).mutex);
    }
    (*thread).running = 0 as libc::c_int;
    _zbar_event_trigger(&mut (*thread).activity);
    _zbar_mutex_unlock(&mut (*proc_0).mutex);
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_processor_create(mut threaded: libc::c_int)
 -> *mut zbar_processor_t {
    let mut proc_0: *mut zbar_processor_t =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<zbar_processor_t>() as libc::c_ulong) as
            *mut zbar_processor_t;
    if proc_0.is_null() { return 0 as *mut zbar_processor_t }
    err_init(&mut (*proc_0).err, ZBAR_MOD_PROCESSOR);
    (*proc_0).scanner = zbar_image_scanner_create();
    if (*proc_0).scanner.is_null() {
        free(proc_0 as *mut libc::c_void);
        return 0 as *mut zbar_processor_t
    }
    (*proc_0).threaded =
        (_zbar_mutex_init(&mut (*proc_0).mutex) == 0 && threaded != 0) as
            libc::c_int;
    _zbar_processor_init(proc_0);
    return proc_0;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_processor_destroy(mut proc_0:
                                                    *mut zbar_processor_t) {
    zbar_processor_init(proc_0, 0 as *const libc::c_char, 0 as libc::c_int);
    if !(*proc_0).syms.is_null() {
        zbar_symbol_set_ref((*proc_0).syms, -(1 as libc::c_int));
        (*proc_0).syms = 0 as *const zbar_symbol_set_t
    }
    if !(*proc_0).scanner.is_null() {
        zbar_image_scanner_destroy((*proc_0).scanner);
        (*proc_0).scanner = 0 as *mut zbar_image_scanner_t
    }
    _zbar_mutex_destroy(&mut (*proc_0).mutex);
    _zbar_processor_cleanup(proc_0);
    if (*proc_0).wait_head.is_null() {
    } else {
        __assert_fail(b"!proc->wait_head\x00" as *const u8 as
                          *const libc::c_char,
                      b"zbar/processor.c\x00" as *const u8 as
                          *const libc::c_char,
                      303 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 48],
                                                &[libc::c_char; 48]>(b"void zbar_processor_destroy(zbar_processor_t *)\x00")).as_ptr());
    }
    if (*proc_0).wait_tail.is_null() {
    } else {
        __assert_fail(b"!proc->wait_tail\x00" as *const u8 as
                          *const libc::c_char,
                      b"zbar/processor.c\x00" as *const u8 as
                          *const libc::c_char,
                      304 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 48],
                                                &[libc::c_char; 48]>(b"void zbar_processor_destroy(zbar_processor_t *)\x00")).as_ptr());
    }
    if (*proc_0).wait_next.is_null() {
    } else {
        __assert_fail(b"!proc->wait_next\x00" as *const u8 as
                          *const libc::c_char,
                      b"zbar/processor.c\x00" as *const u8 as
                          *const libc::c_char,
                      305 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 48],
                                                &[libc::c_char; 48]>(b"void zbar_processor_destroy(zbar_processor_t *)\x00")).as_ptr());
    }
    let mut w: *mut proc_waiter_t = 0 as *mut proc_waiter_t;
    let mut next: *mut proc_waiter_t = 0 as *mut proc_waiter_t;
    w = (*proc_0).free_waiter;
    while !w.is_null() {
        next = (*w).next;
        _zbar_event_destroy(&mut (*w).notify);
        free(w as *mut libc::c_void);
        w = next
    }
    err_cleanup(&mut (*proc_0).err);
    free(proc_0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn zbar_processor_init(mut proc_0:
                                                 *mut zbar_processor_t,
                                             mut dev: *const libc::c_char,
                                             mut enable_display: libc::c_int)
 -> libc::c_int {
    let mut video_threaded: libc::c_int = 0;
    let mut input_threaded: libc::c_int = 0;
    let mut current_block: u64;
    if !(*proc_0).video.is_null() {
        zbar_processor_set_active(proc_0, 0 as libc::c_int);
    }
    if !(*proc_0).window.is_null() && (*proc_0).input_thread.started == 0 {
        _zbar_processor_close(proc_0);
    }
    _zbar_mutex_lock(&mut (*proc_0).mutex);
    _zbar_thread_stop(&mut (*proc_0).input_thread, &mut (*proc_0).mutex);
    _zbar_thread_stop(&mut (*proc_0).video_thread, &mut (*proc_0).mutex);
    _zbar_processor_lock(proc_0);
    _zbar_mutex_unlock(&mut (*proc_0).mutex);
    if !(*proc_0).window.is_null() {
        zbar_window_destroy((*proc_0).window);
        (*proc_0).window = 0 as *mut zbar_window_t
    }
    let mut rc: libc::c_int = 0 as libc::c_int;
    if !(*proc_0).video.is_null() {
        zbar_video_destroy((*proc_0).video);
        (*proc_0).video = 0 as *mut zbar_video_t
    }
    if !(dev.is_null() && enable_display == 0) {
        if enable_display != 0 {
            (*proc_0).window = zbar_window_create();
            if (*proc_0).window.is_null() {
                rc =
                    err_capture(proc_0 as *const libc::c_void, SEV_FATAL,
                                ZBAR_ERR_NOMEM,
                                (*::std::mem::transmute::<&[u8; 20],
                                                          &[libc::c_char; 20]>(b"zbar_processor_init\x00")).as_ptr(),
                                b"allocating window resources\x00" as
                                    *const u8 as *const libc::c_char);
                current_block = 2012931665687488966;
            } else { current_block = 4495394744059808450; }
        } else { current_block = 4495394744059808450; }
        match current_block {
            2012931665687488966 => { }
            _ => {
                if !dev.is_null() {
                    (*proc_0).video = zbar_video_create();
                    if (*proc_0).video.is_null() {
                        rc =
                            err_capture(proc_0 as *const libc::c_void,
                                        SEV_FATAL, ZBAR_ERR_NOMEM,
                                        (*::std::mem::transmute::<&[u8; 20],
                                                                  &[libc::c_char; 20]>(b"zbar_processor_init\x00")).as_ptr(),
                                        b"allocating video resources\x00" as
                                            *const u8 as *const libc::c_char);
                        current_block = 2012931665687488966;
                    } else {
                        if (*proc_0).req_width != 0 ||
                               (*proc_0).req_height != 0 {
                            zbar_video_request_size((*proc_0).video,
                                                    (*proc_0).req_width,
                                                    (*proc_0).req_height);
                        }
                        if (*proc_0).req_intf != 0 {
                            zbar_video_request_interface((*proc_0).video,
                                                         (*proc_0).req_intf);
                        }
                        if (*proc_0).req_iomode != 0 &&
                               zbar_video_request_iomode((*proc_0).video,
                                                         (*proc_0).req_iomode)
                                   != 0 ||
                               zbar_video_open((*proc_0).video, dev) != 0 {
                            rc =
                                err_copy(proc_0 as *mut libc::c_void,
                                         (*proc_0).video as
                                             *mut libc::c_void);
                            current_block = 2012931665687488966;
                        } else { current_block = 18377268871191777778; }
                    }
                } else { current_block = 18377268871191777778; }
                match current_block {
                    2012931665687488966 => { }
                    _ =>
                    /* spawn blocking video thread */
                    {
                        video_threaded =
                            ((*proc_0).threaded != 0 &&
                                 !(*proc_0).video.is_null() &&
                                 zbar_video_get_fd((*proc_0).video) <
                                     0 as libc::c_int) as libc::c_int;
                        if video_threaded != 0 &&
                               _zbar_thread_start(&mut (*proc_0).video_thread,
                                                  Some(proc_video_thread as
                                                           unsafe extern "C" fn(_:
                                                                                    *mut libc::c_void)
                                                               ->
                                                                   *mut libc::c_void),
                                                  proc_0 as *mut libc::c_void,
                                                  &mut (*proc_0).mutex) != 0 {
                            rc =
                                err_capture(proc_0 as *const libc::c_void,
                                            SEV_ERROR, ZBAR_ERR_SYSTEM,
                                            (*::std::mem::transmute::<&[u8; 20],
                                                                      &[libc::c_char; 20]>(b"zbar_processor_init\x00")).as_ptr(),
                                            b"spawning video thread\x00" as
                                                *const u8 as
                                                *const libc::c_char)
                        } else {
                            /* spawn input monitor thread */
                            input_threaded =
                                ((*proc_0).threaded != 0 &&
                                     (!(*proc_0).window.is_null() ||
                                          !(*proc_0).video.is_null() &&
                                              video_threaded == 0)) as
                                    libc::c_int;
                            if input_threaded != 0 &&
                                   _zbar_thread_start(&mut (*proc_0).input_thread,
                                                      Some(proc_input_thread
                                                               as
                                                               unsafe extern "C" fn(_:
                                                                                        *mut libc::c_void)
                                                                   ->
                                                                       *mut libc::c_void),
                                                      proc_0 as
                                                          *mut libc::c_void,
                                                      &mut (*proc_0).mutex) !=
                                       0 {
                                rc =
                                    err_capture(proc_0 as *const libc::c_void,
                                                SEV_ERROR, ZBAR_ERR_SYSTEM,
                                                (*::std::mem::transmute::<&[u8; 20],
                                                                          &[libc::c_char; 20]>(b"zbar_processor_init\x00")).as_ptr(),
                                                b"spawning input thread\x00"
                                                    as *const u8 as
                                                    *const libc::c_char)
                            } else if !(!(*proc_0).window.is_null() &&
                                            input_threaded == 0 &&
                                            {
                                                rc = proc_open(proc_0);
                                                (rc) != 0
                                            }) {
                                if !(*proc_0).video.is_null() &&
                                       (*proc_0).force_input != 0 {
                                    if zbar_video_init((*proc_0).video,
                                                       (*proc_0).force_input
                                                           as libc::c_ulong)
                                           != 0 {
                                        rc =
                                            err_copy(proc_0 as
                                                         *mut libc::c_void,
                                                     (*proc_0).video as
                                                         *mut libc::c_void)
                                    }
                                } else if !(*proc_0).video.is_null() {
                                    let mut retry: libc::c_int =
                                        -(1 as libc::c_int);
                                    if !(*proc_0).window.is_null() {
                                        retry =
                                            zbar_negotiate_format((*proc_0).video,
                                                                  (*proc_0).window);
                                        if retry != 0 {
                                            fprintf(stderr,
                                                    b"WARNING: no compatible input to output format\n...trying again with output disabled\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                        }
                                    }
                                    if retry != 0 {
                                        retry =
                                            zbar_negotiate_format((*proc_0).video,
                                                                  0 as
                                                                      *mut zbar_window_t)
                                    }
                                    if retry != 0 {
                                        if _zbar_verbosity >= 1 as libc::c_int
                                           {
                                            fprintf(stderr,
                                                    b"%s: ERROR: no compatible %s format\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    (*::std::mem::transmute::<&[u8; 20],
                                                                              &[libc::c_char; 20]>(b"zbar_processor_init\x00")).as_ptr(),
                                                    if !(*proc_0).video.is_null()
                                                       {
                                                        b"video input\x00" as
                                                            *const u8 as
                                                            *const libc::c_char
                                                    } else {
                                                        b"window output\x00"
                                                            as *const u8 as
                                                            *const libc::c_char
                                                    });
                                        }
                                        rc =
                                            err_capture(proc_0 as
                                                            *const libc::c_void,
                                                        SEV_ERROR,
                                                        ZBAR_ERR_UNSUPPORTED,
                                                        (*::std::mem::transmute::<&[u8; 20],
                                                                                  &[libc::c_char; 20]>(b"zbar_processor_init\x00")).as_ptr(),
                                                        b"no compatible image format\x00"
                                                            as *const u8 as
                                                            *const libc::c_char)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    /* nothing to do */
    _zbar_mutex_lock(&mut (*proc_0).mutex);
    proc_leave(proc_0);
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_processor_set_data_handler(mut proc_0:
                                                             *mut zbar_processor_t,
                                                         mut handler:
                                                             Option<zbar_image_data_handler_t>,
                                                         mut userdata:
                                                             *const libc::c_void)
 -> Option<zbar_image_data_handler_t> {
    let mut result: Option<zbar_image_data_handler_t> = None;
    proc_enter(proc_0);
    result = (*proc_0).handler;
    (*proc_0).handler = handler;
    (*proc_0).userdata = userdata;
    proc_leave(proc_0);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_processor_set_userdata(mut proc_0:
                                                         *mut zbar_processor_t,
                                                     mut userdata:
                                                         *mut libc::c_void) {
    _zbar_mutex_lock(&mut (*proc_0).mutex);
    (*proc_0).userdata = userdata;
    _zbar_mutex_unlock(&mut (*proc_0).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn zbar_processor_get_userdata(mut proc_0:
                                                         *const zbar_processor_t)
 -> *mut libc::c_void {
    let mut ncproc: *mut zbar_processor_t = proc_0 as *mut zbar_processor_t;
    _zbar_mutex_lock(&mut (*ncproc).mutex);
    let mut userdata: *mut libc::c_void =
        (*ncproc).userdata as *mut libc::c_void;
    _zbar_mutex_unlock(&mut (*ncproc).mutex);
    return userdata;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_processor_set_config(mut proc_0:
                                                       *mut zbar_processor_t,
                                                   mut sym:
                                                       zbar_symbol_type_t,
                                                   mut cfg: zbar_config_t,
                                                   mut val: libc::c_int)
 -> libc::c_int {
    proc_enter(proc_0);
    let mut rc: libc::c_int =
        zbar_image_scanner_set_config((*proc_0).scanner, sym, cfg, val);
    proc_leave(proc_0);
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_processor_set_control(mut proc_0:
                                                        *mut zbar_processor_t,
                                                    mut control_name:
                                                        *const libc::c_char,
                                                    mut value: libc::c_int)
 -> libc::c_int {
    proc_enter(proc_0);
    let mut value_before: libc::c_int = 0;
    let mut value_after: libc::c_int = 0;
    if _zbar_verbosity >= 4 as libc::c_int {
        if zbar_video_get_control((*proc_0).video, control_name,
                                  &mut value_before) == 0 as libc::c_int {
            if _zbar_verbosity >= 0 as libc::c_int {
                fprintf(stderr,
                        b"%s: value of %s before a set: %d\n\x00" as *const u8
                            as *const libc::c_char,
                        (*::std::mem::transmute::<&[u8; 27],
                                                  &[libc::c_char; 27]>(b"zbar_processor_set_control\x00")).as_ptr(),
                        control_name, value_before);
            }
        }
    }
    let mut rc: libc::c_int =
        zbar_video_set_control((*proc_0).video, control_name, value);
    if _zbar_verbosity >= 4 as libc::c_int {
        if zbar_video_get_control((*proc_0).video, control_name,
                                  &mut value_after) == 0 as libc::c_int {
            if _zbar_verbosity >= 0 as libc::c_int {
                fprintf(stderr,
                        b"%s: value of %s after a set: %d\n\x00" as *const u8
                            as *const libc::c_char,
                        (*::std::mem::transmute::<&[u8; 27],
                                                  &[libc::c_char; 27]>(b"zbar_processor_set_control\x00")).as_ptr(),
                        control_name, value_after);
            }
        }
    }
    proc_leave(proc_0);
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_processor_get_control(mut proc_0:
                                                        *mut zbar_processor_t,
                                                    mut control_name:
                                                        *const libc::c_char,
                                                    mut value:
                                                        *mut libc::c_int)
 -> libc::c_int {
    proc_enter(proc_0);
    let mut rc: libc::c_int =
        zbar_video_get_control((*proc_0).video, control_name, value);
    proc_leave(proc_0);
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_processor_request_size(mut proc_0:
                                                         *mut zbar_processor_t,
                                                     mut width: libc::c_uint,
                                                     mut height: libc::c_uint)
 -> libc::c_int {
    proc_enter(proc_0);
    (*proc_0).req_width = width;
    (*proc_0).req_height = height;
    proc_leave(proc_0);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_processor_request_interface(mut proc_0:
                                                              *mut zbar_processor_t,
                                                          mut ver:
                                                              libc::c_int)
 -> libc::c_int {
    proc_enter(proc_0);
    (*proc_0).req_intf = ver;
    proc_leave(proc_0);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_processor_request_iomode(mut proc_0:
                                                           *mut zbar_processor_t,
                                                       mut iomode:
                                                           libc::c_int)
 -> libc::c_int {
    proc_enter(proc_0);
    (*proc_0).req_iomode = iomode;
    proc_leave(proc_0);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_processor_force_format(mut proc_0:
                                                         *mut zbar_processor_t,
                                                     mut input: libc::c_ulong,
                                                     mut output:
                                                         libc::c_ulong)
 -> libc::c_int {
    proc_enter(proc_0);
    (*proc_0).force_input = input as uint32_t;
    (*proc_0).force_output = output as uint32_t;
    proc_leave(proc_0);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_processor_is_visible(mut proc_0:
                                                       *mut zbar_processor_t)
 -> libc::c_int {
    proc_enter(proc_0);
    let mut visible: libc::c_int =
        (!(*proc_0).window.is_null() && (*proc_0).visible != 0) as
            libc::c_int;
    proc_leave(proc_0);
    return visible;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_processor_set_visible(mut proc_0:
                                                        *mut zbar_processor_t,
                                                    mut visible: libc::c_int)
 -> libc::c_int {
    proc_enter(proc_0);
    _zbar_mutex_unlock(&mut (*proc_0).mutex);
    let mut rc: libc::c_int = 0 as libc::c_int;
    if !(*proc_0).window.is_null() {
        if !(*proc_0).video.is_null() {
            rc =
                _zbar_processor_set_size(proc_0,
                                         zbar_video_get_width((*proc_0).video)
                                             as libc::c_uint,
                                         zbar_video_get_height((*proc_0).video)
                                             as libc::c_uint)
        }
        if rc == 0 { rc = _zbar_processor_set_visible(proc_0, visible) }
        if rc == 0 {
            (*proc_0).visible = (visible != 0 as libc::c_int) as libc::c_int
        }
    } else if visible != 0 {
        rc =
            err_capture(proc_0 as *const libc::c_void, SEV_ERROR,
                        ZBAR_ERR_INVALID,
                        (*::std::mem::transmute::<&[u8; 27],
                                                  &[libc::c_char; 27]>(b"zbar_processor_set_visible\x00")).as_ptr(),
                        b"processor display window not initialized\x00" as
                            *const u8 as *const libc::c_char)
    }
    _zbar_mutex_lock(&mut (*proc_0).mutex);
    proc_leave(proc_0);
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_processor_get_results(mut proc_0:
                                                        *const zbar_processor_t)
 -> *const zbar_symbol_set_t {
    let mut ncproc: *mut zbar_processor_t = proc_0 as *mut zbar_processor_t;
    proc_enter(ncproc);
    let mut syms: *const zbar_symbol_set_t = (*proc_0).syms;
    if !syms.is_null() { zbar_symbol_set_ref(syms, 1 as libc::c_int); }
    proc_leave(ncproc);
    return syms;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_processor_user_wait(mut proc_0:
                                                      *mut zbar_processor_t,
                                                  mut timeout: libc::c_int)
 -> libc::c_int {
    proc_enter(proc_0);
    _zbar_mutex_unlock(&mut (*proc_0).mutex);
    let mut rc: libc::c_int = -(1 as libc::c_int);
    if (*proc_0).visible != 0 || (*proc_0).streaming != 0 ||
           timeout >= 0 as libc::c_int {
        let mut timer: zbar_timer_t = zbar_timer_t{tv_sec: 0, tv_nsec: 0,};
        rc =
            _zbar_processor_wait(proc_0, 0x1 as libc::c_int as libc::c_uint,
                                 _zbar_timer_init(&mut timer, timeout))
    }
    if (*proc_0).visible == 0 {
        rc =
            err_capture(proc_0 as *const libc::c_void, SEV_WARNING,
                        ZBAR_ERR_CLOSED,
                        (*::std::mem::transmute::<&[u8; 25],
                                                  &[libc::c_char; 25]>(b"zbar_processor_user_wait\x00")).as_ptr(),
                        b"display window not available for input\x00" as
                            *const u8 as *const libc::c_char)
    }
    if rc > 0 as libc::c_int { rc = (*proc_0).input }
    _zbar_mutex_lock(&mut (*proc_0).mutex);
    proc_leave(proc_0);
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_processor_set_active(mut proc_0:
                                                       *mut zbar_processor_t,
                                                   mut active: libc::c_int)
 -> libc::c_int {
    proc_enter(proc_0);
    let mut rc: libc::c_int = 0;
    if (*proc_0).video.is_null() {
        rc =
            err_capture(proc_0 as *const libc::c_void, SEV_ERROR,
                        ZBAR_ERR_INVALID,
                        (*::std::mem::transmute::<&[u8; 26],
                                                  &[libc::c_char; 26]>(b"zbar_processor_set_active\x00")).as_ptr(),
                        b"video input not initialized\x00" as *const u8 as
                            *const libc::c_char)
    } else {
        _zbar_mutex_unlock(&mut (*proc_0).mutex);
        zbar_image_scanner_enable_cache((*proc_0).scanner, active);
        rc = zbar_video_enable((*proc_0).video, active);
        if rc == 0 {
            _zbar_mutex_lock(&mut (*proc_0).mutex);
            (*proc_0).streaming = active;
            _zbar_mutex_unlock(&mut (*proc_0).mutex);
            rc = _zbar_processor_enable(proc_0)
        } else {
            err_copy(proc_0 as *mut libc::c_void,
                     (*proc_0).video as *mut libc::c_void);
        }
        if (*proc_0).streaming == 0 && !(*proc_0).window.is_null() {
            if zbar_window_draw((*proc_0).window, 0 as *mut zbar_image_t) != 0
                   && rc == 0 {
                rc =
                    err_copy(proc_0 as *mut libc::c_void,
                             (*proc_0).window as *mut libc::c_void)
            }
            _zbar_processor_invalidate(proc_0);
        }
        _zbar_mutex_lock(&mut (*proc_0).mutex);
        if (*proc_0).video_thread.started != 0 {
            _zbar_event_trigger(&mut (*proc_0).video_thread.notify);
        }
    }
    proc_leave(proc_0);
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_process_one(mut proc_0: *mut zbar_processor_t,
                                          mut timeout: libc::c_int)
 -> libc::c_int {
    let mut timer: zbar_timer_t = zbar_timer_t{tv_sec: 0, tv_nsec: 0,};
    let mut current_block: u64;
    proc_enter(proc_0);
    let mut streaming: libc::c_int = (*proc_0).streaming;
    _zbar_mutex_unlock(&mut (*proc_0).mutex);
    let mut rc: libc::c_int = 0 as libc::c_int;
    if (*proc_0).video.is_null() {
        rc =
            err_capture(proc_0 as *const libc::c_void, SEV_ERROR,
                        ZBAR_ERR_INVALID,
                        (*::std::mem::transmute::<&[u8; 17],
                                                  &[libc::c_char; 17]>(b"zbar_process_one\x00")).as_ptr(),
                        b"video input not initialized\x00" as *const u8 as
                            *const libc::c_char)
    } else {
        if streaming == 0 {
            rc = zbar_processor_set_active(proc_0, 1 as libc::c_int);
            if rc != 0 {
                current_block = 14487944739627291418;
            } else { current_block = 13513818773234778473; }
        } else { current_block = 13513818773234778473; }
        match current_block {
            14487944739627291418 => { }
            _ => {
                timer = zbar_timer_t{tv_sec: 0, tv_nsec: 0,};
                rc =
                    _zbar_processor_wait(proc_0,
                                         0x2 as libc::c_int as libc::c_uint,
                                         _zbar_timer_init(&mut timer,
                                                          timeout));
                if streaming == 0 &&
                       zbar_processor_set_active(proc_0, 0 as libc::c_int) !=
                           0 {
                    rc = -(1 as libc::c_int)
                }
            }
        }
    }
    _zbar_mutex_lock(&mut (*proc_0).mutex);
    proc_leave(proc_0);
    return rc;
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
/* * @file
 * ZBar Barcode Reader C API definition
 */
/* * @mainpage
 *
 * interface to the barcode reader is available at several levels.
 * most applications will want to use the high-level interfaces:
 *
 * @section high-level High-Level Interfaces
 *
 * these interfaces wrap all library functionality into an easy-to-use
 * package for a specific toolkit:
 * - the "GTK+ 2.x widget" may be used with GTK GUI applications.  a
 *   Python wrapper is included for PyGtk
 * - the @ref zbar::QZBar "Qt4 widget" may be used with Qt GUI
 *   applications
 * - the Processor interface (in @ref c-processor "C" or @ref
 *   zbar::Processor "C++") adds a scanning window to an application
 *   with no GUI.
 *
 * @section mid-level Intermediate Interfaces
 *
 * building blocks used to construct high-level interfaces:
 * - the ImageScanner (in @ref c-imagescanner "C" or @ref
 *   zbar::ImageScanner "C++") looks for barcodes in a library defined
 *   image object
 * - the Window abstraction (in @ref c-window "C" or @ref
 *   zbar::Window "C++") sinks library images, displaying them on the
 *   platform display
 * - the Video abstraction (in @ref c-video "C" or @ref zbar::Video
 *   "C++") sources library images from a video device
 *
 * @section low-level Low-Level Interfaces
 *
 * direct interaction with barcode scanning and decoding:
 * - the Scanner (in @ref c-scanner "C" or @ref zbar::Scanner "C++")
 *   looks for barcodes in a linear intensity sample stream
 * - the Decoder (in @ref c-decoder "C" or @ref zbar::Decoder "C++")
 *   extracts barcodes from a stream of bar and space widths
 */
/* * @name Global library interfaces */
/*@{*/
/* * "color" of element: bar or space. */
/* *< light area or space between bars */
/* *< dark area or colored bar segment */
/* * decoded symbol type. */
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
/* *< unable to determine orientation */
/* *< upright, read left to right */
/* *< sideways, read top to bottom */
/* *< upside-down, read right to left */
/* *< sideways, read bottom to top */
/* * error codes. */
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
/* * decoder configuration options.
 * @since 0.4
 */
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
/* * decoder symbology modifier flags.
 * @since 0.11
 */
/* * barcode tagged as GS1 (EAN.UCC) reserved
     * (eg, FNC1 before first data character).
     * data may be parsed as a sequence of GS1 AIs
     */
/* * barcode tagged as AIM reserved
     * (eg, FNC1 after first character or digit pair)
     */
/* * number of modifiers */
/* * store video control menu
 * @param name name of the menu item
 * @param val integer value associated with the item
 */
/* * store video controls
 * @param name name of the control
 * @param group name of the control group/class
 * @param type type of the control
 * @param min minimum value of control (if control is integer)
 * @param max maximum value of control (if control is integer)
 * @param def default value of control (if control is integer)
 * @param step increment steps (if control is integer)
 * @param menu menu array
 * @param menu_size menu size
 * @since 0.20
 */
// video drivers may add extra private data in the end of this struct
/* * retrieve runtime library version information.
 * @param major set to the running major version (unless NULL)
 * @param minor set to the running minor version (unless NULL)
 * @returns 0
 */
/* * set global library debug level.
 * @param verbosity desired debug level.  higher values create more spew
 */
/* * increase global library debug level.
 * eg, for -vvvv
 */
/* * retrieve string name for symbol encoding.
 * @param sym symbol type encoding
 * @returns the static string name for the specified symbol type,
 * or "UNKNOWN" if the encoding is not recognized
 */
/* * retrieve string name for addon encoding.
 * @param sym symbol type encoding
 * @returns static string name for any addon, or the empty string
 * if no addons were decoded
 * @deprecated in 0.11
 */
/* * retrieve string name for configuration setting.
 * @param config setting to name
 * @returns static string name for config,
 * or the empty string if value is not a known config
 */
/* * retrieve string name for modifier.
 * @param modifier flag to name
 * @returns static string name for modifier,
 * or the empty string if the value is not a known flag
 */
/* * retrieve string name for orientation.
 * @param orientation orientation encoding
 * @returns the static string name for the specified orientation,
 * or "UNKNOWN" if the orientation is not recognized
 * @since 0.11
 */
/* * parse a configuration string of the form "[symbology.]config[=value]".
 * the config must match one of the recognized names.
 * the symbology, if present, must match one of the recognized names.
 * if symbology is unspecified, it will be set to 0.
 * if value is unspecified it will be set to 1.
 * @returns 0 if the config is parsed successfully, 1 otherwise
 * @since 0.4
 */
/* * consistently compute fourcc values across architectures
 * (adapted from v4l2 specification)
 * @since 0.11
 */
/* * parse a fourcc string into its encoded integer value.
 * @since 0.11
 */
/* * @internal type unsafe error API (don't use) */
/*@}*/
/*------------------------------------------------------------*/
/* * @name Symbol interface
 * decoded barcode symbol result object.  stores type, data, and image
 * location of decoded symbol.  all memory is owned by the library
 */
/*@{*/
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
/*@}*/
/*------------------------------------------------------------*/
/* * @name Image interface
 * stores image data samples along with associated format and size
 * metadata
 */
/*@{*/
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
/* * read back an image in the format written by zbar_image_write()
 * @note TBD
 */
/*@}*/
/*------------------------------------------------------------*/
/* * @name Processor interface
 * @anchor c-processor
 * high-level self-contained image processor.
 * processes video and images for barcodes, optionally displaying
 * images to a library owned output window
 */
/*@{*/
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
#[no_mangle]
pub unsafe extern "C" fn zbar_process_image(mut proc_0: *mut zbar_processor_t,
                                            mut img: *mut zbar_image_t)
 -> libc::c_int {
    proc_enter(proc_0);
    _zbar_mutex_unlock(&mut (*proc_0).mutex);
    let mut rc: libc::c_int = 0 as libc::c_int;
    if !img.is_null() && !(*proc_0).window.is_null() {
        rc =
            _zbar_processor_set_size(proc_0, zbar_image_get_width(img),
                                     zbar_image_get_height(img))
    }
    if rc == 0 {
        zbar_image_scanner_enable_cache((*proc_0).scanner, 0 as libc::c_int);
        rc = _zbar_process_image(proc_0, img);
        if (*proc_0).streaming != 0 {
            zbar_image_scanner_enable_cache((*proc_0).scanner,
                                            1 as libc::c_int);
        }
    }
    _zbar_mutex_lock(&mut (*proc_0).mutex);
    proc_leave(proc_0);
    return rc;
}
