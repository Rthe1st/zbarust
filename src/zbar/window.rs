use ::libc;
extern "C" {
    pub type zbar_video_s;
    pub type window_state_s;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn _zbar_error_spew(object: *const libc::c_void, verbosity: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_init(__mutex: *mut pthread_mutex_t,
                          __mutexattr: *const pthread_mutexattr_t)
     -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    /* window.draw has to be thread safe wrt/other apis
 * FIXME should be a semaphore
 */
    /* PAL interface */
    #[no_mangle]
    fn _zbar_window_attach(_: *mut zbar_window_t, _: *mut libc::c_void,
                           _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    static mut _zbar_verbosity: libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn _zbar_refcnt(cnt: *mut refcnt_t, delta: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn _zbar_window_resize(_: *mut zbar_window_t) -> libc::c_int;
    #[no_mangle]
    fn _zbar_image_free(_: *mut zbar_image_t);
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
pub type zbar_video_t = zbar_video_s;
pub type zbar_image_cleanup_handler_t
    =
    unsafe extern "C" fn(_: *mut zbar_image_t) -> ();
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
 *------------------------------------------------------------------------*/
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
pub struct timezone {
    pub tz_minuteswest: libc::c_int,
    pub tz_dsttime: libc::c_int,
}
#[inline]
unsafe extern "C" fn _zbar_mutex_init(mut lock: *mut zbar_mutex_t)
 -> libc::c_int {
    return pthread_mutex_init(lock, 0 as *const pthread_mutexattr_t);
}
/* just in case */
/* reporting module */
/* formatted and passed to application */
/* errno for system errors */
/* reporting function */
/* description */
/* single string argument */
/* single integer argument */
/* FIXME don't we need varargs hacks here? */
/* unused at src, avoid double free */
#[inline]
unsafe extern "C" fn err_init(mut err: *mut errinfo_t,
                              mut module: errmodule_t) {
    (*err).magic = 0x5252457a as libc::c_int as uint32_t;
    (*err).module = module;
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
unsafe extern "C" fn window_unlock(mut w: *mut zbar_window_t) -> libc::c_int {
    let mut rc: libc::c_int = 0 as libc::c_int;
    rc = _zbar_mutex_unlock(&mut (*w).imglock);
    if rc != 0 {
        err_capture(w as *const libc::c_void, SEV_FATAL, ZBAR_ERR_LOCKING,
                    (*::std::mem::transmute::<&[u8; 14],
                                              &[libc::c_char; 14]>(b"window_unlock\x00")).as_ptr(),
                    b"unable to release lock\x00" as *const u8 as
                        *const libc::c_char);
        (*w).err.errnum = rc;
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
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
unsafe extern "C" fn window_lock(mut w: *mut zbar_window_t) -> libc::c_int {
    let mut rc: libc::c_int = 0 as libc::c_int;
    rc = _zbar_mutex_lock(&mut (*w).imglock);
    if rc != 0 {
        err_capture(w as *const libc::c_void, SEV_FATAL, ZBAR_ERR_LOCKING,
                    (*::std::mem::transmute::<&[u8; 12],
                                              &[libc::c_char; 12]>(b"window_lock\x00")).as_ptr(),
                    b"unable to acquire lock\x00" as *const u8 as
                        *const libc::c_char);
        (*w).err.errnum = rc;
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
/* clock_gettime */
/* gettimeofday */
#[no_mangle]
pub unsafe extern "C" fn zbar_window_create() -> *mut zbar_window_t {
    let mut w: *mut zbar_window_t =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<zbar_window_t>() as libc::c_ulong) as
            *mut zbar_window_t;
    if w.is_null() { return 0 as *mut zbar_window_t }
    err_init(&mut (*w).err, ZBAR_MOD_WINDOW);
    (*w).overlay = 1 as libc::c_int as libc::c_uint;
    _zbar_mutex_init(&mut (*w).imglock);
    return w;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_window_destroy(mut w: *mut zbar_window_t) {
    /* detach */
    zbar_window_attach(w, 0 as *mut libc::c_void,
                       0 as libc::c_int as libc::c_ulong);
    err_cleanup(&mut (*w).err);
    _zbar_mutex_destroy(&mut (*w).imglock);
    free(w as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn zbar_window_attach(mut w: *mut zbar_window_t,
                                            mut display: *mut libc::c_void,
                                            mut drawable: libc::c_ulong)
 -> libc::c_int {
    /* release image */
    zbar_window_draw(w, 0 as *mut zbar_image_t);
    if (*w).cleanup.is_some() {
        (*w).cleanup.expect("non-null function pointer")(w);
        (*w).cleanup = None;
        (*w).draw_image = None
    }
    if !(*w).formats.is_null() {
        free((*w).formats as *mut libc::c_void);
        (*w).formats = 0 as *mut uint32_t
    }
    (*w).src_format = 0 as libc::c_int as uint32_t;
    (*w).src_height = 0 as libc::c_int as libc::c_uint;
    (*w).src_width = (*w).src_height;
    (*w).scaled_size.y = 0 as libc::c_int;
    (*w).scaled_size.x = (*w).scaled_size.y;
    (*w).dst_height = 0 as libc::c_int as libc::c_uint;
    (*w).dst_width = (*w).dst_height;
    (*w).max_height =
        ((1 as libc::c_int) << 15 as libc::c_int) as libc::c_uint;
    (*w).max_width = (*w).max_height;
    (*w).scale_den = 1 as libc::c_int as libc::c_uint;
    (*w).scale_num = (*w).scale_den;
    return _zbar_window_attach(w, display, drawable);
}
#[no_mangle]
pub unsafe extern "C" fn zbar_window_draw(mut w: *mut zbar_window_t,
                                          mut img: *mut zbar_image_t)
 -> libc::c_int {
    if window_lock(w) != 0 { return -(1 as libc::c_int) }
    if (*w).draw_image.is_none() { img = 0 as *mut zbar_image_t }
    if !img.is_null() {
        _zbar_image_refcnt(img, 1 as libc::c_int);
        if (*img).width != (*w).src_width || (*img).height != (*w).src_height
           {
            (*w).dst_width = 0 as libc::c_int as libc::c_uint
        }
    }
    if !(*w).image.is_null() {
        _zbar_image_refcnt((*w).image, -(1 as libc::c_int));
    }
    (*w).image = img;
    return window_unlock(w);
}
#[no_mangle]
pub unsafe extern "C" fn zbar_window_set_overlay(mut w: *mut zbar_window_t,
                                                 mut lvl: libc::c_int) {
    if lvl < 0 as libc::c_int { lvl = 0 as libc::c_int }
    if lvl > 2 as libc::c_int { lvl = 2 as libc::c_int }
    if window_lock(w) != 0 { return }
    if (*w).overlay != lvl as libc::c_uint {
        (*w).overlay = lvl as libc::c_uint
    }
    window_unlock(w);
}
#[no_mangle]
pub unsafe extern "C" fn zbar_window_get_overlay(mut w: *const zbar_window_t)
 -> libc::c_int {
    let mut ncw: *mut zbar_window_t = w as *mut zbar_window_t;
    let mut lvl: libc::c_int = 0;
    if window_lock(ncw) != 0 { return -(1 as libc::c_int) }
    lvl = (*w).overlay as libc::c_int;
    window_unlock(ncw);
    return lvl;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_window_resize(mut w: *mut zbar_window_t,
                                            mut width: libc::c_uint,
                                            mut height: libc::c_uint)
 -> libc::c_int {
    if window_lock(w) != 0 { return -(1 as libc::c_int) }
    (*w).width = width;
    (*w).height = height;
    (*w).scaled_size.x = 0 as libc::c_int;
    _zbar_window_resize(w);
    return window_unlock(w);
}
