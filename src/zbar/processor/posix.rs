use ::libc;
extern "C" {
    pub type zbar_symbol_set_s;
    /*@}*/
    /*------------------------------------------------------------*/
/* * @name Image interface
 * stores image data samples along with associated format and size
 * metadata
 */
/*@{*/
    pub type zbar_image_s;
    pub type zbar_image_scanner_s;
    pub type zbar_window_s;
    pub type zbar_video_s;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    /* * @internal type unsafe error API (don't use) */
    #[no_mangle]
    fn _zbar_error_spew(object: *const libc::c_void, verbosity: libc::c_int)
     -> libc::c_int;
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
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t)
     -> ssize_t;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t)
     -> ssize_t;
    #[no_mangle]
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
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
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn zbar_video_get_fd(video: *const zbar_video_t) -> libc::c_int;
    #[no_mangle]
    fn zbar_video_next_image(video: *mut zbar_video_t) -> *mut zbar_image_t;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn perror(__s: *const libc::c_char);
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    static mut _zbar_verbosity: libc::c_int;
    #[no_mangle]
    fn nanosleep(__requested_time: *const timespec,
                 __remaining: *mut timespec) -> libc::c_int;
    #[no_mangle]
    fn pthread_create(__newthread: *mut pthread_t,
                      __attr: *const pthread_attr_t,
                      __start_routine:
                          Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                     -> *mut libc::c_void>,
                      __arg: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn pthread_join(__th: pthread_t, __thread_return: *mut *mut libc::c_void)
     -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_cond_init(__cond: *mut pthread_cond_t,
                         __cond_attr: *const pthread_condattr_t)
     -> libc::c_int;
    #[no_mangle]
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_cond_wait(__cond: *mut pthread_cond_t,
                         __mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_cond_timedwait(__cond: *mut pthread_cond_t,
                              __mutex: *mut pthread_mutex_t,
                              __abstime: *const timespec) -> libc::c_int;
    #[no_mangle]
    fn _zbar_process_image(_: *mut zbar_processor_t, _: *mut zbar_image_t)
     -> libc::c_int;
    #[no_mangle]
    fn _zbar_processor_unlock(_: *mut zbar_processor_t, _: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn _zbar_processor_lock(_: *mut zbar_processor_t) -> libc::c_int;
    #[no_mangle]
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int)
     -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type uint32_t = __uint32_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
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
pub union pthread_condattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
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
/* * error codes. */
pub type zbar_error_t = zbar_error_e;
pub type zbar_symbol_set_t = zbar_symbol_set_s;
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
/* * opaque image object. */
pub type zbar_image_t = zbar_image_s;
/* * data handler callback function.
 * called when decoded symbol results are available for an image
 */
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct processor_state_s {
    pub polling: poll_desc_t,
    pub thr_polling: poll_desc_t,
    pub kick_fds: [libc::c_int; 2],
    pub pre_poll_handler: Option<poll_handler_t>,
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
pub type poll_handler_t
    =
    unsafe extern "C" fn(_: *mut zbar_processor_t, _: libc::c_int)
        -> libc::c_int;
pub type zbar_processor_t = zbar_processor_s;
/* poll information */
pub type poll_desc_t = poll_desc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct poll_desc_s {
    pub num: libc::c_int,
    pub fds: *mut pollfd,
    pub handlers: *mut Option<poll_handler_t>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
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
/* number of descriptors */
/* poll descriptors */
/* poll handlers */
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
pub type zbar_video_t = zbar_video_s;
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
/* just in case */
/* reporting module */
/* formatted and passed to application */
/* errno for system errors */
/* reporting function */
/* description */
/* single string argument */
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
/* gettimeofday */
/* platform timer abstraction
 *
 * zbar_timer_t stores the absolute expiration of a delay from
 * when the timer was initialized.
 *
 * _zbar_timer_init() initialized timer with specified ms delay.
 *     returns timer or NULL if timeout < 0 (no/infinite timeout)
 * _zbar_timer_check() returns ms remaining until expiration.
 *     will be <= 0 if timer has expired
 */
pub type zbar_timer_t = timespec;
pub type zbar_thread_proc_t
    =
    unsafe extern "C" fn(_: *mut libc::c_void) -> *mut libc::c_void;
pub type nfds_t = libc::c_ulong;
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
unsafe extern "C" fn _zbar_mutex_lock(mut lock: *mut zbar_mutex_t)
 -> libc::c_int {
    let mut rc: libc::c_int = pthread_mutex_lock(lock);
    /* FIXME save system code */
    /*rc = err_capture(proc, SEV_ERROR, ZBAR_ERR_LOCKING, __func__,
                       "unable to lock processor");*/
    return rc;
}
#[inline]
unsafe extern "C" fn _zbar_mutex_unlock(mut lock: *mut zbar_mutex_t)
 -> libc::c_int {
    let mut rc: libc::c_int = pthread_mutex_unlock(lock);
    /* FIXME save system code */
    return rc;
}
/* special case */
#[inline]
unsafe extern "C" fn alloc_polls(mut p: *mut poll_desc_t) -> libc::c_int {
    ::std::ptr::write_volatile(&mut (*p).fds as *mut *mut pollfd,
                               realloc((*p).fds as *mut libc::c_void,
                                       ((*p).num as
                                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<pollfd>()
                                                                            as
                                                                            libc::c_ulong))
                                   as *mut pollfd);
    ::std::ptr::write_volatile(&mut (*p).handlers as
                                   *mut *mut Option<poll_handler_t>,
                               realloc((*p).handlers as *mut libc::c_void,
                                       ((*p).num as
                                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<Option<poll_handler_t>>()
                                                                            as
                                                                            libc::c_ulong))
                                   as *mut Option<poll_handler_t>);
    /* FIXME should check for ENOMEM */
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn add_poll(mut proc_0: *mut zbar_processor_t,
                              mut fd: libc::c_int,
                              mut handler: Option<poll_handler_t>)
 -> libc::c_int {
    let mut state: *mut processor_state_t = (*proc_0).state;
    _zbar_mutex_lock(&mut (*proc_0).mutex);
    let mut polling: *mut poll_desc_t = &mut (*state).polling;
    let fresh0 = (*polling).num;
    (*polling).num = (*polling).num + 1;
    let mut i: libc::c_uint = fresh0 as libc::c_uint;
    if _zbar_verbosity >= 5 as libc::c_int {
        fprintf(stderr,
                b"%s: [%d] fd=%d handler=%p\n\x00" as *const u8 as
                    *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 9],
                                          &[libc::c_char; 9]>(b"add_poll\x00")).as_ptr(),
                i, fd, handler);
    }
    if alloc_polls(polling as *mut poll_desc_t) == 0 {
        memset(&mut *(*polling).fds.offset(i as isize) as *mut pollfd as
                   *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<pollfd>() as libc::c_ulong);
        (*(*polling).fds.offset(i as isize)).fd = fd;
        (*(*polling).fds.offset(i as isize)).events =
            0x1 as libc::c_int as libc::c_short;
        let ref mut fresh1 = *(*polling).handlers.offset(i as isize);
        *fresh1 = handler
    } else { i = -(1 as libc::c_int) as libc::c_uint }
    _zbar_mutex_unlock(&mut (*proc_0).mutex);
    if (*proc_0).input_thread.started != 0 {
        if (*state).kick_fds[1 as libc::c_int as usize] >= 0 as libc::c_int {
        } else {
            __assert_fail(b"state->kick_fds[1] >= 0\x00" as *const u8 as
                              *const libc::c_char,
                          b"zbar/processor/posix.h\x00" as *const u8 as
                              *const libc::c_char,
                          85 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 56],
                                                    &[libc::c_char; 56]>(b"int add_poll(zbar_processor_t *, int, poll_handler_t *)\x00")).as_ptr());
        }
        if write((*state).kick_fds[1 as libc::c_int as usize],
                 &mut i as *mut libc::c_uint as *const libc::c_void,
                 ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong) <
               0 as libc::c_int as libc::c_long {
            return -(1 as libc::c_int)
        }
    } else if (*proc_0).threaded == 0 {
        (*state).thr_polling.num = (*polling).num;
        (*state).thr_polling.fds = (*polling).fds;
        (*state).thr_polling.handlers = (*polling).handlers
    }
    return i as libc::c_int;
}
#[inline]
unsafe extern "C" fn remove_poll(mut proc_0: *mut zbar_processor_t,
                                 mut fd: libc::c_int) -> libc::c_int {
    let mut state: *mut processor_state_t = (*proc_0).state;
    _zbar_mutex_lock(&mut (*proc_0).mutex);
    let mut polling: *mut poll_desc_t = &mut (*state).polling;
    let mut i: libc::c_int = 0;
    i = (*polling).num - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if (*(*polling).fds.offset(i as isize)).fd == fd { break ; }
        i -= 1
    }
    if _zbar_verbosity >= 5 as libc::c_int {
        fprintf(stderr,
                b"%s: [%d] fd=%d n=%d\n\x00" as *const u8 as
                    *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 12],
                                          &[libc::c_char; 12]>(b"remove_poll\x00")).as_ptr(),
                i, fd, (*polling).num);
    }
    if i >= 0 as libc::c_int {
        if (i + 1 as libc::c_int) < (*polling).num {
            let mut n: libc::c_int = (*polling).num - i - 1 as libc::c_int;
            memmove(&mut *(*polling).fds.offset(i as isize) as *mut pollfd as
                        *mut libc::c_void,
                    &mut *(*polling).fds.offset((i + 1 as libc::c_int) as
                                                    isize) as *mut pollfd as
                        *const libc::c_void,
                    (n as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<pollfd>()
                                                         as libc::c_ulong));
            memmove(&mut *(*polling).handlers.offset(i as isize) as
                        *mut Option<poll_handler_t> as *mut libc::c_void,
                    &mut *(*polling).handlers.offset((i + 1 as libc::c_int) as
                                                         isize) as
                        *mut Option<poll_handler_t> as *const libc::c_void,
                    (n as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<poll_handler_t>()
                                                         as libc::c_ulong));
        }
        (*polling).num -= 1;
        i = alloc_polls(polling as *mut poll_desc_t)
    }
    _zbar_mutex_unlock(&mut (*proc_0).mutex);
    if (*proc_0).input_thread.started != 0 {
        if write((*state).kick_fds[1 as libc::c_int as usize],
                 &mut i as *mut libc::c_int as *const libc::c_void,
                 ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong) <
               0 as libc::c_int as libc::c_long {
            return -(1 as libc::c_int)
        }
    } else if (*proc_0).threaded == 0 {
        (*state).thr_polling.num = (*polling).num;
        (*state).thr_polling.fds = (*polling).fds;
        (*state).thr_polling.handlers = (*polling).handlers
    }
    return i;
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
#[inline]
unsafe extern "C" fn proc_sleep(mut timeout: libc::c_int) -> libc::c_int {
    if timeout > 0 as libc::c_int {
    } else {
        __assert_fail(b"timeout > 0\x00" as *const u8 as *const libc::c_char,
                      b"zbar/processor/posix.c\x00" as *const u8 as
                          *const libc::c_char,
                      33 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 20],
                                                &[libc::c_char; 20]>(b"int proc_sleep(int)\x00")).as_ptr());
    }
    let mut sleepns: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut remns: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    sleepns.tv_sec = (timeout / 1000 as libc::c_int) as __time_t;
    sleepns.tv_nsec =
        (timeout % 1000 as libc::c_int * 1000000 as libc::c_int) as
            __syscall_slong_t;
    while nanosleep(&mut sleepns, &mut remns) != 0 &&
              *__errno_location() == 4 as libc::c_int {
        sleepns = remns
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _zbar_event_init(mut event: *mut zbar_event_t)
 -> libc::c_int {
    (*event).state = 0 as libc::c_int;
    (*event).pollfd = -(1 as libc::c_int);
    pthread_cond_init(&mut (*event).cond, 0 as *const pthread_condattr_t);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _zbar_event_destroy(mut event: *mut zbar_event_t) {
    (*event).state = -(1 as libc::c_int);
    (*event).pollfd = -(1 as libc::c_int);
    pthread_cond_destroy(&mut (*event).cond);
}
/* lock must be held */
#[no_mangle]
pub unsafe extern "C" fn _zbar_event_trigger(mut event: *mut zbar_event_t) {
    (*event).state = 1 as libc::c_int; /* unused */
    pthread_cond_broadcast(&mut (*event).cond);
    if (*event).pollfd >= 0 as libc::c_int {
        let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        if write((*event).pollfd,
                 &mut i as *mut libc::c_uint as *const libc::c_void,
                 ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong) <
               0 as libc::c_int as libc::c_long {
            perror(b"\x00" as *const u8 as *const libc::c_char);
        }
        (*event).pollfd = -(1 as libc::c_int)
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
/* platform synchronization "event" abstraction
 */
/* lock must be held */
#[no_mangle]
pub unsafe extern "C" fn _zbar_event_wait(mut event: *mut zbar_event_t,
                                          mut lock: *mut zbar_mutex_t,
                                          mut timeout: *mut zbar_timer_t)
 -> libc::c_int {
    let mut rc: libc::c_int = 0 as libc::c_int;
    while rc == 0 && (*event).state == 0 {
        if timeout.is_null() {
            rc = pthread_cond_wait(&mut (*event).cond, lock)
        } else {
            let mut timer: *mut timespec = 0 as *mut timespec;
            timer = timeout;
            rc = pthread_cond_timedwait(&mut (*event).cond, lock, timer)
        }
    }
    /* consume/reset event */
    (*event).state = 0 as libc::c_int; /* got event */
    if rc == 0 { return 1 as libc::c_int } /* timed out */
    if rc == 110 as libc::c_int { return 0 as libc::c_int }
    return -(1 as libc::c_int);
    /* error (FIXME save info) */
}
#[no_mangle]
pub unsafe extern "C" fn _zbar_thread_start(mut thr: *mut zbar_thread_t,
                                            mut proc_0:
                                                Option<zbar_thread_proc_t>,
                                            mut arg: *mut libc::c_void,
                                            mut lock: *mut zbar_mutex_t)
 -> libc::c_int {
    if (*thr).started != 0 || (*thr).running != 0 {
        return -(1 as libc::c_int)
        /*FIXME*/
    }
    (*thr).started = 1 as libc::c_int;
    _zbar_event_init(&mut (*thr).notify);
    _zbar_event_init(&mut (*thr).activity);
    let mut rc: libc::c_int = 0 as libc::c_int;
    _zbar_mutex_lock(lock);
    if pthread_create(&mut (*thr).tid, 0 as *const pthread_attr_t, proc_0,
                      arg) != 0 ||
           _zbar_event_wait(&mut (*thr).activity, lock,
                            0 as *mut zbar_timer_t) < 0 as libc::c_int ||
           (*thr).running == 0 {
        (*thr).started = 0 as libc::c_int;
        _zbar_event_destroy(&mut (*thr).notify);
        _zbar_event_destroy(&mut (*thr).activity);
        /*FIXME*/
        rc = -(1 as libc::c_int)
    }
    _zbar_mutex_unlock(lock);
    return rc;
}
/*rc = err_capture_num(proc, SEV_ERROR, ZBAR_ERR_SYSTEM,
          __func__, "spawning thread", rc);*/
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
/* simple platform thread abstraction
 */
#[no_mangle]
pub unsafe extern "C" fn _zbar_thread_stop(mut thr: *mut zbar_thread_t,
                                           mut lock: *mut zbar_mutex_t)
 -> libc::c_int {
    if (*thr).started != 0 {
        (*thr).started = 0 as libc::c_int;
        _zbar_event_trigger(&mut (*thr).notify);
        while (*thr).running != 0 {
            /* FIXME time out and abandon? */
            _zbar_event_wait(&mut (*thr).activity, lock,
                             0 as *mut zbar_timer_t);
        }
        pthread_join((*thr).tid, 0 as *mut *mut libc::c_void);
        _zbar_event_destroy(&mut (*thr).notify);
        _zbar_event_destroy(&mut (*thr).activity);
    }
    return 0 as libc::c_int;
}
/* used by poll interface.  lock is already held */
unsafe extern "C" fn proc_video_handler(mut proc_0: *mut zbar_processor_t,
                                        mut i: libc::c_int) -> libc::c_int {
    _zbar_mutex_lock(&mut (*proc_0).mutex);
    _zbar_processor_lock(proc_0);
    _zbar_mutex_unlock(&mut (*proc_0).mutex);
    let mut img: *mut zbar_image_t = 0 as *mut zbar_image_t;
    if (*proc_0).streaming != 0 {
        /* not expected to block */
        img = zbar_video_next_image((*proc_0).video);
        if !img.is_null() { _zbar_process_image(proc_0, img); }
    }
    _zbar_mutex_lock(&mut (*proc_0).mutex);
    _zbar_processor_unlock(proc_0, 0 as libc::c_int);
    _zbar_mutex_unlock(&mut (*proc_0).mutex);
    if !img.is_null() { zbar_image_destroy(img); }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn proc_cache_polling(mut state: *mut processor_state_t) {
    /* make a thread-local copy of polling data */
    (*state).thr_polling.num = (*state).polling.num;
    let mut n: libc::c_int = (*state).thr_polling.num;
    alloc_polls(&mut (*state).thr_polling as *mut poll_desc_t as
                    *mut poll_desc_t);
    memcpy((*state).thr_polling.fds as *mut libc::c_void,
           (*state).polling.fds as *const libc::c_void,
           (n as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<pollfd>() as
                                                libc::c_ulong));
    memcpy((*state).thr_polling.handlers as *mut libc::c_void,
           (*state).polling.handlers as *const libc::c_void,
           (n as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<Option<poll_handler_t>>()
                                                as libc::c_ulong));
}
unsafe extern "C" fn proc_kick_handler(mut proc_0: *mut zbar_processor_t,
                                       mut i: libc::c_int) -> libc::c_int {
    let mut state: *mut processor_state_t = (*proc_0).state;
    if _zbar_verbosity >= 5 as libc::c_int {
        fprintf(stderr,
                b"%s: kicking %d fds\n\x00" as *const u8 as
                    *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 18],
                                          &[libc::c_char; 18]>(b"proc_kick_handler\x00")).as_ptr(),
                (*state).polling.num);
    }
    let mut junk: [libc::c_uint; 2] = [0; 2];
    let mut rc: libc::c_int =
        read((*state).kick_fds[0 as libc::c_int as usize],
             junk.as_mut_ptr() as *mut libc::c_void,
             (2 as libc::c_int as
                  libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_uint>()
                                                  as libc::c_ulong)) as
            libc::c_int;
    if (*proc_0).threaded != 0 {
    } else {
        __assert_fail(b"proc->threaded\x00" as *const u8 as
                          *const libc::c_char,
                      b"zbar/processor/posix.c\x00" as *const u8 as
                          *const libc::c_char,
                      225 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 47],
                                                &[libc::c_char; 47]>(b"int proc_kick_handler(zbar_processor_t *, int)\x00")).as_ptr());
    }
    _zbar_mutex_lock(&mut (*proc_0).mutex);
    proc_cache_polling((*proc_0).state);
    _zbar_mutex_unlock(&mut (*proc_0).mutex);
    return rc;
}
#[inline]
unsafe extern "C" fn proc_poll_inputs(mut proc_0: *mut zbar_processor_t,
                                      mut timeout: libc::c_int)
 -> libc::c_int {
    let mut state: *mut processor_state_t = (*proc_0).state;
    if (*state).pre_poll_handler.is_some() {
        (*state).pre_poll_handler.expect("non-null function pointer")(proc_0,
                                                                      -(1 as
                                                                            libc::c_int));
    }
    let mut p: *mut poll_desc_t = &mut (*state).thr_polling;
    if (*p).num != 0 {
    } else {
        __assert_fail(b"p->num\x00" as *const u8 as *const libc::c_char,
                      b"zbar/processor/posix.c\x00" as *const u8 as
                          *const libc::c_char,
                      240 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 46],
                                                &[libc::c_char; 46]>(b"int proc_poll_inputs(zbar_processor_t *, int)\x00")).as_ptr());
    }
    let mut rc: libc::c_int = poll((*p).fds, (*p).num as nfds_t, timeout);
    if rc <= 0 as libc::c_int {
        /* FIXME detect and handle fatal errors (somehow) */
        return rc
    } /* debug */
    let mut i: libc::c_int = 0;
    i = (*p).num - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if (*(*p).fds.offset(i as isize)).revents != 0 {
            if (*(*p).handlers.offset(i as isize)).is_some() {
                (*(*p).handlers.offset(i as
                                           isize)).expect("non-null function pointer")(proc_0,
                                                                                       i);
            }
            (*(*p).fds.offset(i as isize)).revents =
                0 as libc::c_int as libc::c_short;
            rc -= 1
        }
        i -= 1
    }
    if rc == 0 {
    } else {
        __assert_fail(b"!rc\x00" as *const u8 as *const libc::c_char,
                      b"zbar/processor/posix.c\x00" as *const u8 as
                          *const libc::c_char,
                      253 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 46],
                                                &[libc::c_char; 46]>(b"int proc_poll_inputs(zbar_processor_t *, int)\x00")).as_ptr());
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _zbar_processor_input_wait(mut proc_0:
                                                        *mut zbar_processor_t,
                                                    mut event:
                                                        *mut zbar_event_t,
                                                    mut timeout: libc::c_int)
 -> libc::c_int {
    let mut state: *mut processor_state_t = (*proc_0).state;
    if (*state).thr_polling.num != 0 {
        if !event.is_null() {
            _zbar_mutex_lock(&mut (*proc_0).mutex);
            (*event).pollfd = (*state).kick_fds[1 as libc::c_int as usize];
            _zbar_mutex_unlock(&mut (*proc_0).mutex);
        }
        return proc_poll_inputs(proc_0, timeout)
    } else { if timeout != 0 { return proc_sleep(timeout) } }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn _zbar_processor_init(mut proc_0:
                                                  *mut zbar_processor_t)
 -> libc::c_int {
    (*proc_0).state =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<processor_state_t>() as libc::c_ulong) as
            *mut processor_state_t;
    let mut state: *mut processor_state_t = (*proc_0).state;
    (*state).kick_fds[1 as libc::c_int as usize] = -(1 as libc::c_int);
    (*state).kick_fds[0 as libc::c_int as usize] =
        (*state).kick_fds[1 as libc::c_int as usize];
    if (*proc_0).threaded != 0 {
        /* FIXME check errors */
        if pipe((*state).kick_fds.as_mut_ptr()) != 0 {
            return err_capture(proc_0 as *const libc::c_void, SEV_FATAL,
                               ZBAR_ERR_SYSTEM,
                               (*::std::mem::transmute::<&[u8; 21],
                                                         &[libc::c_char; 21]>(b"_zbar_processor_init\x00")).as_ptr(),
                               b"failed to open pipe\x00" as *const u8 as
                                   *const libc::c_char)
        }
        add_poll(proc_0, (*state).kick_fds[0 as libc::c_int as usize],
                 Some(proc_kick_handler as
                          unsafe extern "C" fn(_: *mut zbar_processor_t,
                                               _: libc::c_int)
                              -> libc::c_int));
        proc_cache_polling((*proc_0).state);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _zbar_processor_cleanup(mut proc_0:
                                                     *mut zbar_processor_t)
 -> libc::c_int {
    let mut state: *mut processor_state_t = (*proc_0).state;
    if (*proc_0).threaded != 0 {
        close((*state).kick_fds[0 as libc::c_int as usize]);
        close((*state).kick_fds[1 as libc::c_int as usize]);
        (*state).kick_fds[1 as libc::c_int as usize] = -(1 as libc::c_int);
        (*state).kick_fds[0 as libc::c_int as usize] =
            (*state).kick_fds[1 as libc::c_int as usize]
    }
    if !(*state).polling.fds.is_null() {
        free((*state).polling.fds as *mut libc::c_void);
        (*state).polling.fds = 0 as *mut pollfd;
        if (*proc_0).threaded == 0 {
            (*state).thr_polling.fds = 0 as *mut pollfd
        }
    }
    if !(*state).polling.handlers.is_null() {
        free((*state).polling.handlers as *mut libc::c_void);
        (*state).polling.handlers = 0 as *mut Option<poll_handler_t>;
        if (*proc_0).threaded == 0 {
            (*state).thr_polling.handlers = 0 as *mut Option<poll_handler_t>
        }
    }
    if !(*state).thr_polling.fds.is_null() {
        free((*state).thr_polling.fds as *mut libc::c_void);
        (*state).thr_polling.fds = 0 as *mut pollfd
    }
    if !(*state).thr_polling.handlers.is_null() {
        free((*state).thr_polling.handlers as *mut libc::c_void);
        (*state).thr_polling.handlers = 0 as *mut Option<poll_handler_t>
    }
    free((*proc_0).state as *mut libc::c_void);
    (*proc_0).state = 0 as *mut processor_state_t;
    return 0 as libc::c_int;
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
pub unsafe extern "C" fn _zbar_processor_enable(mut proc_0:
                                                    *mut zbar_processor_t)
 -> libc::c_int {
    let mut vid_fd: libc::c_int = zbar_video_get_fd((*proc_0).video);
    if vid_fd < 0 as libc::c_int { return 0 as libc::c_int }
    if (*proc_0).streaming != 0 {
        add_poll(proc_0, vid_fd,
                 Some(proc_video_handler as
                          unsafe extern "C" fn(_: *mut zbar_processor_t,
                                               _: libc::c_int)
                              -> libc::c_int));
    } else { remove_poll(proc_0, vid_fd); }
    /* FIXME failure recovery? */
    return 0 as libc::c_int;
}
