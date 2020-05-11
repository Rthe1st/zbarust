use ::libc;
extern {
    /* @} */
    /* ------------------------------------------------------------ */
    /* * @name Image interface
     * stores image data samples along with associated format and size
     * metadata
     */
    /* @{ */
    pub type zbar_image_s;
    pub type window_state_s;
    /* * @internal type unsafe error API (don't use) */
    #[no_mangle]
    fn _zbar_error_spew(object: *const libc::c_void, verbosity: libc::c_int) -> libc::c_int;
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
    static mut _zbar_verbosity: libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
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
pub type point_t = point_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point_s {
    pub x: libc::c_int,
    pub y: libc::c_int,
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
/* * opaque image object. */
pub type zbar_image_t = zbar_image_s;
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
pub type window_state_t = window_state_s;
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
/* just in case */
/* reporting module */
/* formatted and passed to application */
/* errno for system errors */
/* reporting function */
/* description */
/* single string argument */
/* single integer argument */
/*------------------------------------------------------------------------
 *  Copyright 2008-2009 (c) Jeff Brown <spadix@users.sourceforge.net>
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
#[inline]
unsafe extern fn null_error(
    mut m: *mut libc::c_void,
    mut func: *const libc::c_char,
) -> libc::c_int {
    return err_capture(
        m,
        SEV_ERROR,
        ZBAR_ERR_UNSUPPORTED,
        func,
        b"not compiled with output window support\x00" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern fn _zbar_window_attach(
    mut w: *mut zbar_window_t,
    mut display: *mut libc::c_void,
    mut win: libc::c_ulong,
) -> libc::c_int {
    return null_error(
        w as *mut libc::c_void,
        (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"_zbar_window_attach\x00"))
            .as_ptr(),
    );
}
#[no_mangle]
pub unsafe extern fn _zbar_window_expose(
    mut w: *mut zbar_window_t,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> libc::c_int {
    return null_error(
        w as *mut libc::c_void,
        (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"_zbar_window_expose\x00"))
            .as_ptr(),
    );
}
#[no_mangle]
pub unsafe extern fn _zbar_window_resize(mut w: *mut zbar_window_t) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern fn _zbar_window_clear(mut w: *mut zbar_window_t) -> libc::c_int {
    return null_error(
        w as *mut libc::c_void,
        (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"_zbar_window_clear\x00"))
            .as_ptr(),
    );
}
#[no_mangle]
pub unsafe extern fn _zbar_window_begin(mut w: *mut zbar_window_t) -> libc::c_int {
    return null_error(
        w as *mut libc::c_void,
        (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"_zbar_window_begin\x00"))
            .as_ptr(),
    );
}
#[no_mangle]
pub unsafe extern fn _zbar_window_end(mut w: *mut zbar_window_t) -> libc::c_int {
    return null_error(
        w as *mut libc::c_void,
        (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"_zbar_window_end\x00"))
            .as_ptr(),
    );
}
#[no_mangle]
pub unsafe extern fn _zbar_window_draw_marker(
    mut w: *mut zbar_window_t,
    mut rgb: uint32_t,
    mut p: point_t,
) -> libc::c_int {
    return null_error(
        w as *mut libc::c_void,
        (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(b"_zbar_window_draw_marker\x00"))
            .as_ptr(),
    );
}
#[no_mangle]
pub unsafe extern fn _zbar_window_draw_polygon(
    mut w: *mut zbar_window_t,
    mut rgb: uint32_t,
    mut pts: *const point_t,
    mut npts: libc::c_int,
) -> libc::c_int {
    return null_error(
        w as *mut libc::c_void,
        (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
            b"_zbar_window_draw_polygon\x00",
        ))
        .as_ptr(),
    );
}
#[no_mangle]
pub unsafe extern fn _zbar_window_draw_text(
    mut w: *mut zbar_window_t,
    mut rgb: uint32_t,
    mut p: point_t,
    mut text: *const libc::c_char,
) -> libc::c_int {
    return null_error(
        w as *mut libc::c_void,
        (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(b"_zbar_window_draw_text\x00"))
            .as_ptr(),
    );
}
#[no_mangle]
pub unsafe extern fn _zbar_window_fill_rect(
    mut w: *mut zbar_window_t,
    mut rgb: uint32_t,
    mut org: point_t,
    mut size: point_t,
) -> libc::c_int {
    return null_error(
        w as *mut libc::c_void,
        (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(b"_zbar_window_fill_rect\x00"))
            .as_ptr(),
    );
}
/* window.draw has to be thread safe wrt/other apis
 * FIXME should be a semaphore
 */
/* PAL interface */
#[no_mangle]
pub unsafe extern fn _zbar_window_draw_logo(mut w: *mut zbar_window_t) -> libc::c_int {
    return null_error(
        w as *mut libc::c_void,
        (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(b"_zbar_window_draw_logo\x00"))
            .as_ptr(),
    );
}
