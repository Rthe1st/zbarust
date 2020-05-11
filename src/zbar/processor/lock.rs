use ::libc;
extern {
    pub type zbar_symbol_set_s;
    /* @} */
    /* ------------------------------------------------------------ */
    /* * @name Image interface
     * stores image data samples along with associated format and size
     * metadata
     */
    /* @{ */
    pub type zbar_image_s;
    pub type processor_state_s;
    pub type zbar_image_scanner_s;
    pub type zbar_window_s;
    pub type zbar_video_s;
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
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn _zbar_process_image(_: *mut zbar_processor_t, _: *mut zbar_image_t) -> libc::c_int;
    #[no_mangle]
    fn _zbar_processor_input_wait(
        _: *mut zbar_processor_t,
        _: *mut zbar_event_t,
        _: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn zbar_video_get_fd(video: *const zbar_video_t) -> libc::c_int;
    #[no_mangle]
    fn zbar_video_next_image(video: *mut zbar_video_t) -> *mut zbar_image_t;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    #[no_mangle]
    fn pthread_self() -> pthread_t;
    #[no_mangle]
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn _zbar_event_init(_: *mut zbar_event_t) -> libc::c_int;
    #[no_mangle]
    fn _zbar_event_trigger(_: *mut zbar_event_t);
    #[no_mangle]
    fn _zbar_event_wait(
        _: *mut zbar_event_t,
        _: *mut zbar_mutex_t,
        _: *mut zbar_timer_t,
    ) -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
pub type uint32_t = __uint32_t;
pub type clockid_t = __clockid_t;
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
pub type zbar_image_data_handler_t =
    unsafe extern fn(_: *mut zbar_image_t, _: *const libc::c_void) -> ();
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
/* platform synchronization "event" abstraction
 */
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
 *------------------------------------------------------------------------ */
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
/* @} */
/* ------------------------------------------------------------ */
/* * @name Window interface
 * @anchor c-window
 * mid-level output window abstraction.
 * displays images to user-specified platform specific output window
 */
/* @{ */
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
/* @} */
/* ------------------------------------------------------------ */
/* * @name Image Scanner interface
 * @anchor c-imagescanner
 * mid-level image scanner interface.
 * reads barcodes from 2-D images
 */
/* @{ */
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
pub type zbar_processor_t = zbar_processor_s;
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
 *------------------------------------------------------------------------ */
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
#[inline]
unsafe extern fn _zbar_thread_is_self(mut tid: zbar_thread_id_t) -> libc::c_int {
    return pthread_equal(tid, pthread_self());
}
#[inline]
unsafe extern fn _zbar_thread_self() -> zbar_thread_id_t {
    return pthread_self();
}
#[inline]
unsafe extern fn pthread_equal(mut __thread1: pthread_t, mut __thread2: pthread_t) -> libc::c_int {
    return (__thread1 == __thread2) as libc::c_int;
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
#[inline]
unsafe extern fn _zbar_timer_check(mut timer: *mut zbar_timer_t) -> libc::c_int {
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut delay: libc::c_int = 0;
    if timer.is_null() {
        return -(1 as libc::c_int);
    }
    clock_gettime(0 as libc::c_int, &mut now);
    delay = (((*timer).tv_sec - now.tv_sec) * 1000 as libc::c_int as libc::c_long
        + ((*timer).tv_nsec - now.tv_nsec) / 1000000 as libc::c_int as libc::c_long)
        as libc::c_int;
    return if delay >= 0 as libc::c_int {
        delay
    } else {
        0 as libc::c_int
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
 *------------------------------------------------------------------------ */
/* the processor api lock is a recursive mutex with added capabilities
 * to completely drop all lock levels before blocking and atomically
 * unblock a waiting set.  the lock is implemented using a variation
 * of the "specific notification pattern" [cargill], which makes it
 * easy to provide these features across platforms with consistent,
 * predictable semantics.  probably overkill, but additional overhead
 * associated with this mechanism should fall in the noise, as locks
 * are only exchanged O(frame/image)
 *
 * [cargill]
 * http://www.profcon.com/profcon/cargill/jgf/9809/SpecificNotification.html
 */
#[inline]
unsafe extern fn proc_waiter_queue(mut proc_0: *mut zbar_processor_t) -> *mut proc_waiter_t {
    let mut waiter: *mut proc_waiter_t = (*proc_0).free_waiter;
    if !waiter.is_null() {
        (*proc_0).free_waiter = (*waiter).next;
        (*waiter).events = 0 as libc::c_int as libc::c_uint
    } else {
        waiter = calloc(
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<proc_waiter_t>() as libc::c_ulong,
        ) as *mut proc_waiter_t;
        _zbar_event_init(&mut (*waiter).notify);
    }
    (*waiter).next = 0 as *mut proc_waiter_s;
    (*waiter).requester = _zbar_thread_self();
    if !(*proc_0).wait_head.is_null() {
        (*(*proc_0).wait_tail).next = waiter
    } else {
        (*proc_0).wait_head = waiter
    }
    (*proc_0).wait_tail = waiter;
    return waiter;
}
#[inline]
unsafe extern fn proc_waiter_dequeue(mut proc_0: *mut zbar_processor_t) -> *mut proc_waiter_t {
    let mut prev: *mut proc_waiter_t = (*proc_0).wait_next;
    let mut waiter: *mut proc_waiter_t = 0 as *mut proc_waiter_t;
    if !prev.is_null() {
        waiter = (*prev).next
    } else {
        waiter = (*proc_0).wait_head
    }
    while !waiter.is_null()
        && (*waiter).events & (0x1 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint != 0
    {
        prev = waiter;
        (*proc_0).wait_next = waiter;
        waiter = (*waiter).next
    }
    if !waiter.is_null() {
        if !prev.is_null() {
            (*prev).next = (*waiter).next
        } else {
            (*proc_0).wait_head = (*waiter).next
        }
        if (*waiter).next.is_null() {
            (*proc_0).wait_tail = prev
        }
        (*waiter).next = 0 as *mut proc_waiter_s;
        (*proc_0).lock_level = 1 as libc::c_int;
        (*proc_0).lock_owner = (*waiter).requester
    }
    return waiter;
}
#[inline]
unsafe extern fn proc_waiter_release(
    mut proc_0: *mut zbar_processor_t,
    mut waiter: *mut proc_waiter_t,
) {
    if !waiter.is_null() {
        (*waiter).next = (*proc_0).free_waiter;
        (*proc_0).free_waiter = waiter
    };
}
#[no_mangle]
pub unsafe extern fn _zbar_processor_lock(mut proc_0: *mut zbar_processor_t) -> libc::c_int {
    if (*proc_0).lock_level == 0 {
        (*proc_0).lock_owner = _zbar_thread_self();
        (*proc_0).lock_level = 1 as libc::c_int;
        return 0 as libc::c_int;
    }
    if _zbar_thread_is_self((*proc_0).lock_owner) != 0 {
        (*proc_0).lock_level += 1;
        return 0 as libc::c_int;
    }
    let mut waiter: *mut proc_waiter_t = proc_waiter_queue(proc_0);
    _zbar_event_wait(&mut (*waiter).notify, &mut (*proc_0).mutex, 0 as *mut zbar_timer_t);
    if (*proc_0).lock_level == 1 as libc::c_int {
    } else {
        __assert_fail(
            b"proc->lock_level == 1\x00" as *const u8 as *const libc::c_char,
            b"zbar/processor/lock.c\x00" as *const u8 as *const libc::c_char,
            116 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                b"int _zbar_processor_lock(zbar_processor_t *)\x00",
            ))
            .as_ptr(),
        );
    }
    if _zbar_thread_is_self((*proc_0).lock_owner) != 0 {
    } else {
        __assert_fail(
            b"_zbar_thread_is_self(proc->lock_owner)\x00" as *const u8 as *const libc::c_char,
            b"zbar/processor/lock.c\x00" as *const u8 as *const libc::c_char,
            117 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                b"int _zbar_processor_lock(zbar_processor_t *)\x00",
            ))
            .as_ptr(),
        );
    }
    proc_waiter_release(proc_0, waiter);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern fn _zbar_processor_unlock(
    mut proc_0: *mut zbar_processor_t,
    mut all: libc::c_int,
) -> libc::c_int {
    if (*proc_0).lock_level > 0 as libc::c_int {
    } else {
        __assert_fail(
            b"proc->lock_level > 0\x00" as *const u8 as *const libc::c_char,
            b"zbar/processor/lock.c\x00" as *const u8 as *const libc::c_char,
            126 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 52], &[libc::c_char; 52]>(
                b"int _zbar_processor_unlock(zbar_processor_t *, int)\x00",
            ))
            .as_ptr(),
        );
    }
    if _zbar_thread_is_self((*proc_0).lock_owner) != 0 {
    } else {
        __assert_fail(
            b"_zbar_thread_is_self(proc->lock_owner)\x00" as *const u8 as *const libc::c_char,
            b"zbar/processor/lock.c\x00" as *const u8 as *const libc::c_char,
            127 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 52], &[libc::c_char; 52]>(
                b"int _zbar_processor_unlock(zbar_processor_t *, int)\x00",
            ))
            .as_ptr(),
        );
    }
    if all != 0 {
        (*proc_0).lock_level = 0 as libc::c_int
    } else {
        (*proc_0).lock_level -= 1
    }
    if (*proc_0).lock_level == 0 {
        let mut waiter: *mut proc_waiter_t = proc_waiter_dequeue(proc_0);
        if !waiter.is_null() {
            _zbar_event_trigger(&mut (*waiter).notify);
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern fn _zbar_processor_notify(
    mut proc_0: *mut zbar_processor_t,
    mut events: libc::c_uint,
) {
    (*proc_0).wait_next = 0 as *mut proc_waiter_t;
    let mut waiter: *mut proc_waiter_t = 0 as *mut proc_waiter_t;
    waiter = (*proc_0).wait_head;
    while !waiter.is_null() {
        (*waiter).events =
            (*waiter).events & !events | events & 0x80 as libc::c_int as libc::c_uint;
        waiter = (*waiter).next
    }
    if (*proc_0).lock_level == 0 {
        waiter = proc_waiter_dequeue(proc_0);
        if !waiter.is_null() {
            _zbar_event_trigger(&mut (*waiter).notify);
        }
    };
}
#[inline]
unsafe extern fn proc_wait_unthreaded(
    mut proc_0: *mut zbar_processor_t,
    mut waiter: *mut proc_waiter_t,
    mut timeout: *mut zbar_timer_t,
) -> libc::c_int {
    let mut blocking: libc::c_int = ((*proc_0).streaming != 0
        && zbar_video_get_fd((*proc_0).video) < 0 as libc::c_int)
        as libc::c_int;
    _zbar_mutex_unlock(&mut (*proc_0).mutex);
    let mut rc: libc::c_int = 1 as libc::c_int;
    while rc > 0 as libc::c_int
        && (*waiter).events & (0x1 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint != 0
    {
        /* FIXME lax w/the locking (though shouldn't matter...) */
        if blocking != 0 {
            let mut img: *mut zbar_image_t = zbar_video_next_image((*proc_0).video);
            if img.is_null() {
                rc = -(1 as libc::c_int);
                break;
            } else {
                /* FIXME reacquire API lock! (refactor w/video thread?) */
                _zbar_mutex_lock(&mut (*proc_0).mutex);
                _zbar_process_image(proc_0, img);
                zbar_image_destroy(img);
                _zbar_mutex_unlock(&mut (*proc_0).mutex);
            }
        }
        let mut reltime: libc::c_int = _zbar_timer_check(timeout);
        if blocking != 0 && (reltime < 0 as libc::c_int || reltime > 15 as libc::c_int) {
            reltime = 15 as libc::c_int
        }
        rc = _zbar_processor_input_wait(proc_0, 0 as *mut zbar_event_t, reltime)
    }
    _zbar_mutex_lock(&mut (*proc_0).mutex);
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
 *------------------------------------------------------------------------ */
/* max time to wait for input before looking for the next frame.
 * only used in unthreaded mode with blocking (non-pollable) video.
 * NB subject to precision of whatever timer is in use
 */
/* ms */
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
#[no_mangle]
pub unsafe extern fn _zbar_processor_wait(
    mut proc_0: *mut zbar_processor_t,
    mut events: libc::c_uint,
    mut timeout: *mut zbar_timer_t,
) -> libc::c_int {
    _zbar_mutex_lock(&mut (*proc_0).mutex);
    let mut save_level: libc::c_int = (*proc_0).lock_level;
    let mut waiter: *mut proc_waiter_t = proc_waiter_queue(proc_0);
    (*waiter).events = events & (0x1 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint;
    _zbar_processor_unlock(proc_0, 1 as libc::c_int);
    let mut rc: libc::c_int = 0;
    if (*proc_0).threaded != 0 {
        rc = _zbar_event_wait(&mut (*waiter).notify, &mut (*proc_0).mutex, timeout)
    } else {
        rc = proc_wait_unthreaded(proc_0, waiter, timeout)
    }
    if rc <= 0 as libc::c_int || (*proc_0).threaded == 0 {
        /* reacquire api lock */
        (*waiter).events &= 0x80 as libc::c_int as libc::c_uint;
        (*proc_0).wait_next = 0 as *mut proc_waiter_t;
        if (*proc_0).lock_level == 0 {
            let mut w: *mut proc_waiter_t = proc_waiter_dequeue(proc_0);
            if w == waiter {
            } else {
                __assert_fail(b"w == waiter\x00" as *const u8 as
                                  *const libc::c_char,
                              b"zbar/processor/lock.c\x00" as *const u8 as
                                  *const libc::c_char,
                              212 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 75],
                                                        &[libc::c_char; 75]>(b"int _zbar_processor_wait(zbar_processor_t *, unsigned int, zbar_timer_t *)\x00")).as_ptr());
            }
        } else {
            _zbar_event_wait(&mut (*waiter).notify, &mut (*proc_0).mutex, 0 as *mut zbar_timer_t);
        }
    }
    if rc > 0 as libc::c_int && (*waiter).events & 0x80 as libc::c_int as libc::c_uint != 0 {
        rc = -(1 as libc::c_int)
    }
    if (*proc_0).lock_level == 1 as libc::c_int {
    } else {
        __assert_fail(
            b"proc->lock_level == 1\x00" as *const u8 as *const libc::c_char,
            b"zbar/processor/lock.c\x00" as *const u8 as *const libc::c_char,
            220 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 75], &[libc::c_char; 75]>(
                b"int _zbar_processor_wait(zbar_processor_t *, unsigned int, zbar_timer_t *)\x00",
            ))
            .as_ptr(),
        );
    }
    if _zbar_thread_is_self((*proc_0).lock_owner) != 0 {
    } else {
        __assert_fail(
            b"_zbar_thread_is_self(proc->lock_owner)\x00" as *const u8 as *const libc::c_char,
            b"zbar/processor/lock.c\x00" as *const u8 as *const libc::c_char,
            221 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 75], &[libc::c_char; 75]>(
                b"int _zbar_processor_wait(zbar_processor_t *, unsigned int, zbar_timer_t *)\x00",
            ))
            .as_ptr(),
        );
    }
    (*proc_0).lock_level = save_level;
    proc_waiter_release(proc_0, waiter);
    _zbar_mutex_unlock(&mut (*proc_0).mutex);
    return rc;
}
