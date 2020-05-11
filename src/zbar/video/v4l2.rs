use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type video_state_s;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t)
     -> ssize_t;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn mmap(__addr: *mut libc::c_void, __len: size_t, __prot: libc::c_int,
            __flags: libc::c_int, __fd: libc::c_int, __offset: __off_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut _zbar_verbosity: libc::c_int;
    #[no_mangle]
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn _zbar_error_spew(object: *const libc::c_void, verbosity: libc::c_int)
     -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
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
pub type __u8 = libc::c_uchar;
pub type __u16 = libc::c_ushort;
pub type __s32 = libc::c_int;
pub type __u32 = libc::c_uint;
pub type __s64 = libc::c_longlong;
pub type __u64 = libc::c_ulonglong;
pub type v4l2_field = libc::c_uint;
pub const V4L2_FIELD_INTERLACED_BT: v4l2_field = 9;
pub const V4L2_FIELD_INTERLACED_TB: v4l2_field = 8;
pub const V4L2_FIELD_ALTERNATE: v4l2_field = 7;
pub const V4L2_FIELD_SEQ_BT: v4l2_field = 6;
pub const V4L2_FIELD_SEQ_TB: v4l2_field = 5;
pub const V4L2_FIELD_INTERLACED: v4l2_field = 4;
pub const V4L2_FIELD_BOTTOM: v4l2_field = 3;
pub const V4L2_FIELD_TOP: v4l2_field = 2;
pub const V4L2_FIELD_NONE: v4l2_field = 1;
pub const V4L2_FIELD_ANY: v4l2_field = 0;
pub type v4l2_buf_type = libc::c_uint;
pub const V4L2_BUF_TYPE_PRIVATE: v4l2_buf_type = 128;
pub const V4L2_BUF_TYPE_META_OUTPUT: v4l2_buf_type = 14;
pub const V4L2_BUF_TYPE_META_CAPTURE: v4l2_buf_type = 13;
pub const V4L2_BUF_TYPE_SDR_OUTPUT: v4l2_buf_type = 12;
pub const V4L2_BUF_TYPE_SDR_CAPTURE: v4l2_buf_type = 11;
pub const V4L2_BUF_TYPE_VIDEO_OUTPUT_MPLANE: v4l2_buf_type = 10;
pub const V4L2_BUF_TYPE_VIDEO_CAPTURE_MPLANE: v4l2_buf_type = 9;
pub const V4L2_BUF_TYPE_VIDEO_OUTPUT_OVERLAY: v4l2_buf_type = 8;
pub const V4L2_BUF_TYPE_SLICED_VBI_OUTPUT: v4l2_buf_type = 7;
pub const V4L2_BUF_TYPE_SLICED_VBI_CAPTURE: v4l2_buf_type = 6;
pub const V4L2_BUF_TYPE_VBI_OUTPUT: v4l2_buf_type = 5;
pub const V4L2_BUF_TYPE_VBI_CAPTURE: v4l2_buf_type = 4;
pub const V4L2_BUF_TYPE_VIDEO_OVERLAY: v4l2_buf_type = 3;
pub const V4L2_BUF_TYPE_VIDEO_OUTPUT: v4l2_buf_type = 2;
pub const V4L2_BUF_TYPE_VIDEO_CAPTURE: v4l2_buf_type = 1;
pub type v4l2_memory = libc::c_uint;
pub const V4L2_MEMORY_DMABUF: v4l2_memory = 4;
pub const V4L2_MEMORY_OVERLAY: v4l2_memory = 3;
pub const V4L2_MEMORY_USERPTR: v4l2_memory = 2;
pub const V4L2_MEMORY_MMAP: v4l2_memory = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct v4l2_rect {
    pub left: __s32,
    pub top: __s32,
    pub width: __u32,
    pub height: __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct v4l2_fract {
    pub numerator: __u32,
    pub denominator: __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct v4l2_capability {
    pub driver: [__u8; 16],
    pub card: [__u8; 32],
    pub bus_info: [__u8; 32],
    pub version: __u32,
    pub capabilities: __u32,
    pub device_caps: __u32,
    pub reserved: [__u32; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct v4l2_pix_format {
    pub width: __u32,
    pub height: __u32,
    pub pixelformat: __u32,
    pub field: __u32,
    pub bytesperline: __u32,
    pub sizeimage: __u32,
    pub colorspace: __u32,
    pub priv_0: __u32,
    pub flags: __u32,
    pub c2rust_unnamed: C2RustUnnamed,
    pub quantization: __u32,
    pub xfer_func: __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ycbcr_enc: __u32,
    pub hsv_enc: __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct v4l2_fmtdesc {
    pub index: __u32,
    pub type_0: __u32,
    pub flags: __u32,
    pub description: [__u8; 32],
    pub pixelformat: __u32,
    pub reserved: [__u32; 4],
}
pub type v4l2_frmsizetypes = libc::c_uint;
pub const V4L2_FRMSIZE_TYPE_STEPWISE: v4l2_frmsizetypes = 3;
pub const V4L2_FRMSIZE_TYPE_CONTINUOUS: v4l2_frmsizetypes = 2;
pub const V4L2_FRMSIZE_TYPE_DISCRETE: v4l2_frmsizetypes = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct v4l2_frmsize_discrete {
    pub width: __u32,
    pub height: __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct v4l2_frmsize_stepwise {
    pub min_width: __u32,
    pub max_width: __u32,
    pub step_width: __u32,
    pub min_height: __u32,
    pub max_height: __u32,
    pub step_height: __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct v4l2_frmsizeenum {
    pub index: __u32,
    pub pixel_format: __u32,
    pub type_0: __u32,
    pub c2rust_unnamed: C2RustUnnamed_0,
    pub reserved: [__u32; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub discrete: v4l2_frmsize_discrete,
    pub stepwise: v4l2_frmsize_stepwise,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct v4l2_timecode {
    pub type_0: __u32,
    pub flags: __u32,
    pub frames: __u8,
    pub seconds: __u8,
    pub minutes: __u8,
    pub hours: __u8,
    pub userbits: [__u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct v4l2_requestbuffers {
    pub count: __u32,
    pub type_0: __u32,
    pub memory: __u32,
    pub capabilities: __u32,
    pub reserved: [__u32; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct v4l2_plane {
    pub bytesused: __u32,
    pub length: __u32,
    pub m: C2RustUnnamed_1,
    pub data_offset: __u32,
    pub reserved: [__u32; 11],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub mem_offset: __u32,
    pub userptr: libc::c_ulong,
    pub fd: __s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct v4l2_buffer {
    pub index: __u32,
    pub type_0: __u32,
    pub bytesused: __u32,
    pub flags: __u32,
    pub field: __u32,
    pub timestamp: timeval,
    pub timecode: v4l2_timecode,
    pub sequence: __u32,
    pub memory: __u32,
    pub m: C2RustUnnamed_3,
    pub length: __u32,
    pub reserved2: __u32,
    pub c2rust_unnamed: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub request_fd: __s32,
    pub reserved: __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub offset: __u32,
    pub userptr: libc::c_ulong,
    pub planes: *mut v4l2_plane,
    pub fd: __s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct v4l2_clip {
    pub c: v4l2_rect,
    pub next: *mut v4l2_clip,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct v4l2_window {
    pub w: v4l2_rect,
    pub field: __u32,
    pub chromakey: __u32,
    pub clips: *mut v4l2_clip,
    pub clipcount: __u32,
    pub bitmap: *mut libc::c_void,
    pub global_alpha: __u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct v4l2_cropcap {
    pub type_0: __u32,
    pub bounds: v4l2_rect,
    pub defrect: v4l2_rect,
    pub pixelaspect: v4l2_fract,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct v4l2_crop {
    pub type_0: __u32,
    pub c: v4l2_rect,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct v4l2_ext_control {
    pub id: __u32,
    pub size: __u32,
    pub reserved2: [__u32; 1],
    pub c2rust_unnamed: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub value: __s32,
    pub value64: __s64,
    pub string: *mut libc::c_char,
    pub p_u8: *mut __u8,
    pub p_u16: *mut __u16,
    pub p_u32: *mut __u32,
    pub ptr: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct v4l2_ext_controls {
    pub c2rust_unnamed: C2RustUnnamed_5,
    pub count: __u32,
    pub error_idx: __u32,
    pub request_fd: __s32,
    pub reserved: [__u32; 1],
    pub controls: *mut v4l2_ext_control,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub ctrl_class: __u32,
    pub which: __u32,
}
pub type v4l2_ctrl_type = libc::c_uint;
pub const V4L2_CTRL_TYPE_U32: v4l2_ctrl_type = 258;
pub const V4L2_CTRL_TYPE_U16: v4l2_ctrl_type = 257;
pub const V4L2_CTRL_TYPE_U8: v4l2_ctrl_type = 256;
pub const V4L2_CTRL_COMPOUND_TYPES: v4l2_ctrl_type = 256;
pub const V4L2_CTRL_TYPE_INTEGER_MENU: v4l2_ctrl_type = 9;
pub const V4L2_CTRL_TYPE_BITMASK: v4l2_ctrl_type = 8;
pub const V4L2_CTRL_TYPE_STRING: v4l2_ctrl_type = 7;
pub const V4L2_CTRL_TYPE_CTRL_CLASS: v4l2_ctrl_type = 6;
pub const V4L2_CTRL_TYPE_INTEGER64: v4l2_ctrl_type = 5;
pub const V4L2_CTRL_TYPE_BUTTON: v4l2_ctrl_type = 4;
pub const V4L2_CTRL_TYPE_MENU: v4l2_ctrl_type = 3;
pub const V4L2_CTRL_TYPE_BOOLEAN: v4l2_ctrl_type = 2;
pub const V4L2_CTRL_TYPE_INTEGER: v4l2_ctrl_type = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct v4l2_query_ext_ctrl {
    pub id: __u32,
    pub type_0: __u32,
    pub name: [libc::c_char; 32],
    pub minimum: __s64,
    pub maximum: __s64,
    pub step: __u64,
    pub default_value: __s64,
    pub flags: __u32,
    pub elem_size: __u32,
    pub elems: __u32,
    pub nr_of_dims: __u32,
    pub dims: [__u32; 4],
    pub reserved: [__u32; 32],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct v4l2_querymenu {
    pub id: __u32,
    pub index: __u32,
    pub c2rust_unnamed: C2RustUnnamed_6,
    pub reserved: __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub name: [__u8; 32],
    pub value: __s64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct v4l2_vbi_format {
    pub sampling_rate: __u32,
    pub offset: __u32,
    pub samples_per_line: __u32,
    pub sample_format: __u32,
    pub start: [__s32; 2],
    pub count: [__u32; 2],
    pub flags: __u32,
    pub reserved: [__u32; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct v4l2_sliced_vbi_format {
    pub service_set: __u16,
    pub service_lines: [[__u16; 24]; 2],
    pub io_size: __u32,
    pub reserved: [__u32; 2],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct v4l2_plane_pix_format {
    pub sizeimage: __u32,
    pub bytesperline: __u32,
    pub reserved: [__u16; 6],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct v4l2_pix_format_mplane {
    pub width: __u32,
    pub height: __u32,
    pub pixelformat: __u32,
    pub field: __u32,
    pub colorspace: __u32,
    pub plane_fmt: [v4l2_plane_pix_format; 8],
    pub num_planes: __u8,
    pub flags: __u8,
    pub c2rust_unnamed: C2RustUnnamed_7,
    pub quantization: __u8,
    pub xfer_func: __u8,
    pub reserved: [__u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub ycbcr_enc: __u8,
    pub hsv_enc: __u8,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct v4l2_sdr_format {
    pub pixelformat: __u32,
    pub buffersize: __u32,
    pub reserved: [__u8; 24],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct v4l2_meta_format {
    pub dataformat: __u32,
    pub buffersize: __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct v4l2_format {
    pub type_0: __u32,
    pub fmt: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub pix: v4l2_pix_format,
    pub pix_mp: v4l2_pix_format_mplane,
    pub win: v4l2_window,
    pub vbi: v4l2_vbi_format,
    pub sliced: v4l2_sliced_vbi_format,
    pub sdr: v4l2_sdr_format,
    pub meta: v4l2_meta_format,
    pub raw_data: [__u8; 200],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zbar_symbol_set_s {
    pub refcnt: refcnt_t,
    pub nsyms: libc::c_int,
    pub head: *mut zbar_symbol_t,
    pub tail: *mut zbar_symbol_t,
}
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
/* userspace buffers */
/* mmap interface */
pub const VIDEO_USERPTR: video_iomode_e = 3;
/* standard system calls */
pub const VIDEO_MMAP: video_iomode_e = 2;
pub const VIDEO_READWRITE: video_iomode_e = 1;
pub type video_interface_t = video_interface_e;
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
/* number of images to preallocate */
pub type video_interface_e = libc::c_uint;
/* video for windows */
/* v4l protocol version 2 */
pub const VIDEO_VFW: video_interface_e = 3;
/* v4l protocol version 1 */
pub const VIDEO_V4L2: video_interface_e = 2;
/* uninitialized */
pub const VIDEO_V4L1: video_interface_e = 1;
pub const VIDEO_INVALID: video_interface_e = 0;
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
pub type errmodule_e = libc::c_uint;
pub const ZBAR_MOD_UNKNOWN: errmodule_e = 4;
pub const ZBAR_MOD_IMAGE_SCANNER: errmodule_e = 3;
pub const ZBAR_MOD_WINDOW: errmodule_e = 2;
pub const ZBAR_MOD_VIDEO: errmodule_e = 1;
pub const ZBAR_MOD_PROCESSOR: errmodule_e = 0;
pub type zbar_image_cleanup_handler_t
    =
    unsafe extern "C" fn(_: *mut zbar_image_t) -> ();
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct video_controls_priv_s {
    pub s: video_controls_s,
    pub id: __u32,
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
unsafe extern "C" fn err_capture_int(mut container: *const libc::c_void,
                                     mut sev: errsev_t,
                                     mut type_0: zbar_error_t,
                                     mut func: *const libc::c_char,
                                     mut detail: *const libc::c_char,
                                     mut arg: libc::c_int) -> libc::c_int {
    let mut err: *mut errinfo_t = container as *mut errinfo_t;
    if (*err).magic == 0x5252457a as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"err->magic == ERRINFO_MAGIC\x00" as *const u8 as
                          *const libc::c_char,
                      b"./zbar/error.h\x00" as *const u8 as
                          *const libc::c_char,
                      191 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 91],
                                                &[libc::c_char; 91]>(b"int err_capture_int(const void *, errsev_t, zbar_error_t, const char *, const char *, int)\x00")).as_ptr());
    }
    (*err).arg_int = arg;
    return err_capture(container, sev, type_0, func, detail);
}
#[inline]
unsafe extern "C" fn _zbar_mutex_unlock(mut lock: *mut zbar_mutex_t)
 -> libc::c_int {
    let mut rc: libc::c_int = pthread_mutex_unlock(lock);
    /* FIXME save system code */
    return rc;
}
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
#[inline]
unsafe extern "C" fn video_nq_image(mut vdo: *mut zbar_video_t,
                                    mut img: *mut zbar_image_t)
 -> libc::c_int {
    /* maintains queued buffers in order */
    (*img).next = 0 as *mut zbar_image_t;
    if !(*vdo).nq_image.is_null() { (*(*vdo).nq_image).next = img }
    (*vdo).nq_image = img;
    if (*vdo).dq_image.is_null() { (*vdo).dq_image = img }
    return video_unlock(vdo);
}
#[inline]
unsafe extern "C" fn video_dq_image(mut vdo: *mut zbar_video_t)
 -> *mut zbar_image_t {
    let mut img: *mut zbar_image_t = (*vdo).dq_image;
    if !img.is_null() {
        (*vdo).dq_image = (*img).next;
        (*img).next = 0 as *mut zbar_image_t
    }
    if video_unlock(vdo) != 0 {
        /* FIXME reclaim image */
        return 0 as *mut zbar_image_t
    }
    return img;
}
unsafe extern "C" fn v4l2_nq(mut vdo: *mut zbar_video_t,
                             mut img: *mut zbar_image_t) -> libc::c_int {
    if (*vdo).iomode as libc::c_uint ==
           VIDEO_READWRITE as libc::c_int as libc::c_uint {
        return video_nq_image(vdo, img)
    }
    if video_unlock(vdo) != 0 { return -(1 as libc::c_int) }
    let mut vbuf: v4l2_buffer =
        v4l2_buffer{index: 0,
                    type_0: 0,
                    bytesused: 0,
                    flags: 0,
                    field: 0,
                    timestamp: timeval{tv_sec: 0, tv_usec: 0,},
                    timecode:
                        v4l2_timecode{type_0: 0,
                                      flags: 0,
                                      frames: 0,
                                      seconds: 0,
                                      minutes: 0,
                                      hours: 0,
                                      userbits: [0; 4],},
                    sequence: 0,
                    memory: 0,
                    m: C2RustUnnamed_3{offset: 0,},
                    length: 0,
                    reserved2: 0,
                    c2rust_unnamed: C2RustUnnamed_2{request_fd: 0,},};
    memset(&mut vbuf as *mut v4l2_buffer as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<v4l2_buffer>() as libc::c_ulong);
    vbuf.type_0 = V4L2_BUF_TYPE_VIDEO_CAPTURE as libc::c_int as __u32;
    if (*vdo).iomode as libc::c_uint ==
           VIDEO_MMAP as libc::c_int as libc::c_uint {
        vbuf.memory = V4L2_MEMORY_MMAP as libc::c_int as __u32;
        vbuf.index = (*img).srcidx as __u32
    } else {
        vbuf.memory = V4L2_MEMORY_USERPTR as libc::c_int as __u32;
        vbuf.m.userptr = (*img).data as libc::c_ulong;
        vbuf.length = (*img).datalen as __u32;
        vbuf.index = (*img).srcidx as __u32
        /* FIXME workaround broken drivers */
    }
    if ioctl((*vdo).fd,
             ((2 as libc::c_uint | 1 as libc::c_uint) <<
                  0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int +
                      14 as libc::c_int |
                  (('V' as i32) << 0 as libc::c_int + 8 as libc::c_int) as
                      libc::c_uint |
                  ((15 as libc::c_int) << 0 as libc::c_int) as libc::c_uint)
                 as libc::c_ulong |
                 (::std::mem::size_of::<v4l2_buffer>() as libc::c_ulong) <<
                     0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
             &mut vbuf as *mut v4l2_buffer) < 0 as libc::c_int {
        return err_capture(vdo as *const libc::c_void, SEV_ERROR,
                           ZBAR_ERR_SYSTEM,
                           (*::std::mem::transmute::<&[u8; 8],
                                                     &[libc::c_char; 8]>(b"v4l2_nq\x00")).as_ptr(),
                           b"queuing video buffer (VIDIOC_QBUF)\x00" as
                               *const u8 as *const libc::c_char)
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn v4l2_dq(mut vdo: *mut zbar_video_t)
 -> *mut zbar_image_t {
    let mut img: *mut zbar_image_t = 0 as *mut zbar_image_t;
    let mut fd: libc::c_int = (*vdo).fd;
    if (*vdo).iomode as libc::c_uint !=
           VIDEO_READWRITE as libc::c_int as libc::c_uint {
        let mut iomode: video_iomode_t = (*vdo).iomode;
        if video_unlock(vdo) != 0 { return 0 as *mut zbar_image_t }
        let mut vbuf: v4l2_buffer =
            v4l2_buffer{index: 0,
                        type_0: 0,
                        bytesused: 0,
                        flags: 0,
                        field: 0,
                        timestamp: timeval{tv_sec: 0, tv_usec: 0,},
                        timecode:
                            v4l2_timecode{type_0: 0,
                                          flags: 0,
                                          frames: 0,
                                          seconds: 0,
                                          minutes: 0,
                                          hours: 0,
                                          userbits: [0; 4],},
                        sequence: 0,
                        memory: 0,
                        m: C2RustUnnamed_3{offset: 0,},
                        length: 0,
                        reserved2: 0,
                        c2rust_unnamed: C2RustUnnamed_2{request_fd: 0,},};
        memset(&mut vbuf as *mut v4l2_buffer as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<v4l2_buffer>() as libc::c_ulong);
        vbuf.type_0 = V4L2_BUF_TYPE_VIDEO_CAPTURE as libc::c_int as __u32;
        if iomode as libc::c_uint == VIDEO_MMAP as libc::c_int as libc::c_uint
           {
            vbuf.memory = V4L2_MEMORY_MMAP as libc::c_int as __u32
        } else { vbuf.memory = V4L2_MEMORY_USERPTR as libc::c_int as __u32 }
        if ioctl(fd,
                 ((2 as libc::c_uint | 1 as libc::c_uint) <<
                      0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int +
                          14 as libc::c_int |
                      (('V' as i32) << 0 as libc::c_int + 8 as libc::c_int) as
                          libc::c_uint |
                      ((17 as libc::c_int) << 0 as libc::c_int) as
                          libc::c_uint) as libc::c_ulong |
                     (::std::mem::size_of::<v4l2_buffer>() as libc::c_ulong)
                         <<
                         0 as libc::c_int + 8 as libc::c_int +
                             8 as libc::c_int, &mut vbuf as *mut v4l2_buffer)
               < 0 as libc::c_int {
            return 0 as *mut zbar_image_t
        }
        if iomode as libc::c_uint == VIDEO_MMAP as libc::c_int as libc::c_uint
           {
            if vbuf.index >= 0 as libc::c_int as libc::c_uint {
            } else {
                __assert_fail(b"vbuf.index >= 0\x00" as *const u8 as
                                  *const libc::c_char,
                              b"zbar/video/v4l2.c\x00" as *const u8 as
                                  *const libc::c_char,
                              115 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 38],
                                                        &[libc::c_char; 38]>(b"zbar_image_t *v4l2_dq(zbar_video_t *)\x00")).as_ptr());
            }
            if vbuf.index < (*vdo).num_images as libc::c_uint {
            } else {
                __assert_fail(b"vbuf.index < vdo->num_images\x00" as *const u8
                                  as *const libc::c_char,
                              b"zbar/video/v4l2.c\x00" as *const u8 as
                                  *const libc::c_char,
                              116 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 38],
                                                        &[libc::c_char; 38]>(b"zbar_image_t *v4l2_dq(zbar_video_t *)\x00")).as_ptr());
            }
            img = *(*vdo).images.offset(vbuf.index as isize)
        } else {
            /* reverse map pointer back to image (FIXME) */
            if vbuf.m.userptr >= (*vdo).buf as libc::c_ulong {
            } else {
                __assert_fail(b"vbuf.m.userptr >= (unsigned long)vdo->buf\x00"
                                  as *const u8 as *const libc::c_char,
                              b"zbar/video/v4l2.c\x00" as *const u8 as
                                  *const libc::c_char,
                              121 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 38],
                                                        &[libc::c_char; 38]>(b"zbar_image_t *v4l2_dq(zbar_video_t *)\x00")).as_ptr());
            }
            if vbuf.m.userptr <
                   (*vdo).buf.offset((*vdo).buflen as isize) as libc::c_ulong
               {
            } else {
                __assert_fail(b"vbuf.m.userptr < (unsigned long)(vdo->buf + vdo->buflen)\x00"
                                  as *const u8 as *const libc::c_char,
                              b"zbar/video/v4l2.c\x00" as *const u8 as
                                  *const libc::c_char,
                              122 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 38],
                                                        &[libc::c_char; 38]>(b"zbar_image_t *v4l2_dq(zbar_video_t *)\x00")).as_ptr());
            }
            let mut i: libc::c_int =
                vbuf.m.userptr.wrapping_sub((*vdo).buf as
                                                libc::c_ulong).wrapping_div((*vdo).datalen)
                    as libc::c_int;
            if i >= 0 as libc::c_int {
            } else {
                __assert_fail(b"i >= 0\x00" as *const u8 as
                                  *const libc::c_char,
                              b"zbar/video/v4l2.c\x00" as *const u8 as
                                  *const libc::c_char,
                              124 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 38],
                                                        &[libc::c_char; 38]>(b"zbar_image_t *v4l2_dq(zbar_video_t *)\x00")).as_ptr());
            }
            if i < (*vdo).num_images {
            } else {
                __assert_fail(b"i < vdo->num_images\x00" as *const u8 as
                                  *const libc::c_char,
                              b"zbar/video/v4l2.c\x00" as *const u8 as
                                  *const libc::c_char,
                              125 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 38],
                                                        &[libc::c_char; 38]>(b"zbar_image_t *v4l2_dq(zbar_video_t *)\x00")).as_ptr());
            }
            img = *(*vdo).images.offset(i as isize);
            if vbuf.m.userptr == (*img).data as libc::c_ulong {
            } else {
                __assert_fail(b"vbuf.m.userptr == (unsigned long)img->data\x00"
                                  as *const u8 as *const libc::c_char,
                              b"zbar/video/v4l2.c\x00" as *const u8 as
                                  *const libc::c_char,
                              127 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 38],
                                                        &[libc::c_char; 38]>(b"zbar_image_t *v4l2_dq(zbar_video_t *)\x00")).as_ptr());
            }
        }
    } else {
        img = video_dq_image(vdo);
        if img.is_null() { return 0 as *mut zbar_image_t }
        /* FIXME should read entire image */
        let mut datalen: libc::c_ulong =
            read(fd, (*img).data as *mut libc::c_void, (*img).datalen) as
                libc::c_ulong;
        if datalen < 0 as libc::c_int as libc::c_ulong {
            return 0 as *mut zbar_image_t
        } else {
            if datalen != (*img).datalen {
                if _zbar_verbosity >= 0 as libc::c_int {
                    fprintf(stderr,
                            b"%s: WARNING: read() size mismatch: 0x%lx != 0x%lx\n\x00"
                                as *const u8 as *const libc::c_char,
                            (*::std::mem::transmute::<&[u8; 8],
                                                      &[libc::c_char; 8]>(b"v4l2_dq\x00")).as_ptr(),
                            datalen, (*img).datalen);
                }
            }
        }
    }
    return img;
}
unsafe extern "C" fn v4l2_start(mut vdo: *mut zbar_video_t) -> libc::c_int {
    if (*vdo).iomode as libc::c_uint ==
           VIDEO_READWRITE as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    let mut type_0: v4l2_buf_type = V4L2_BUF_TYPE_VIDEO_CAPTURE;
    if ioctl((*vdo).fd,
             ((1 as libc::c_uint) <<
                  0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int +
                      14 as libc::c_int |
                  (('V' as i32) << 0 as libc::c_int + 8 as libc::c_int) as
                      libc::c_uint |
                  ((18 as libc::c_int) << 0 as libc::c_int) as libc::c_uint)
                 as libc::c_ulong |
                 (::std::mem::size_of::<libc::c_int>() as libc::c_ulong) <<
                     0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
             &mut type_0 as *mut v4l2_buf_type) < 0 as libc::c_int {
        return err_capture(vdo as *const libc::c_void, SEV_ERROR,
                           ZBAR_ERR_SYSTEM,
                           (*::std::mem::transmute::<&[u8; 11],
                                                     &[libc::c_char; 11]>(b"v4l2_start\x00")).as_ptr(),
                           b"starting video stream (VIDIOC_STREAMON)\x00" as
                               *const u8 as *const libc::c_char)
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn v4l2_stop(mut vdo: *mut zbar_video_t) -> libc::c_int {
    if (*vdo).iomode as libc::c_uint ==
           VIDEO_READWRITE as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    let mut type_0: v4l2_buf_type = V4L2_BUF_TYPE_VIDEO_CAPTURE;
    if ioctl((*vdo).fd,
             ((1 as libc::c_uint) <<
                  0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int +
                      14 as libc::c_int |
                  (('V' as i32) << 0 as libc::c_int + 8 as libc::c_int) as
                      libc::c_uint |
                  ((19 as libc::c_int) << 0 as libc::c_int) as libc::c_uint)
                 as libc::c_ulong |
                 (::std::mem::size_of::<libc::c_int>() as libc::c_ulong) <<
                     0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
             &mut type_0 as *mut v4l2_buf_type) < 0 as libc::c_int {
        return err_capture(vdo as *const libc::c_void, SEV_ERROR,
                           ZBAR_ERR_SYSTEM,
                           (*::std::mem::transmute::<&[u8; 10],
                                                     &[libc::c_char; 10]>(b"v4l2_stop\x00")).as_ptr(),
                           b"stopping video stream (VIDIOC_STREAMOFF)\x00" as
                               *const u8 as *const libc::c_char)
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn v4l2_cleanup(mut vdo: *mut zbar_video_t) -> libc::c_int {
    if (*vdo).iomode as libc::c_uint ==
           VIDEO_READWRITE as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    let mut rb: v4l2_requestbuffers =
        v4l2_requestbuffers{count: 0,
                            type_0: 0,
                            memory: 0,
                            capabilities: 0,
                            reserved: [0; 1],};
    memset(&mut rb as *mut v4l2_requestbuffers as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<v4l2_requestbuffers>() as libc::c_ulong);
    rb.type_0 = V4L2_BUF_TYPE_VIDEO_CAPTURE as libc::c_int as __u32;
    if (*vdo).iomode as libc::c_uint ==
           VIDEO_MMAP as libc::c_int as libc::c_uint {
        rb.memory = V4L2_MEMORY_MMAP as libc::c_int as __u32;
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < (*vdo).num_images {
            let mut img: *mut zbar_image_t =
                *(*vdo).images.offset(i as isize);
            if !(*img).data.is_null() &&
                   munmap((*img).data as *mut libc::c_void, (*img).datalen) !=
                       0 {
                err_capture(vdo as *const libc::c_void, SEV_WARNING,
                            ZBAR_ERR_SYSTEM,
                            (*::std::mem::transmute::<&[u8; 13],
                                                      &[libc::c_char; 13]>(b"v4l2_cleanup\x00")).as_ptr(),
                            b"unmapping video frame buffers\x00" as *const u8
                                as *const libc::c_char);
            }
            (*img).data = 0 as *const libc::c_void;
            (*img).datalen = 0 as libc::c_int as libc::c_ulong;
            i += 1
        }
    } else { rb.memory = V4L2_MEMORY_USERPTR as libc::c_int as __u32 }
    /* requesting 0 buffers
     * should implicitly disable streaming
     */
    if ioctl((*vdo).fd,
             ((2 as libc::c_uint | 1 as libc::c_uint) <<
                  0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int +
                      14 as libc::c_int |
                  (('V' as i32) << 0 as libc::c_int + 8 as libc::c_int) as
                      libc::c_uint |
                  ((8 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as
                 libc::c_ulong |
                 (::std::mem::size_of::<v4l2_requestbuffers>() as
                      libc::c_ulong) <<
                     0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
             &mut rb as *mut v4l2_requestbuffers) < 0 as libc::c_int {
        err_capture(vdo as *const libc::c_void, SEV_WARNING, ZBAR_ERR_SYSTEM,
                    (*::std::mem::transmute::<&[u8; 13],
                                              &[libc::c_char; 13]>(b"v4l2_cleanup\x00")).as_ptr(),
                    b"releasing video frame buffers (VIDIOC_REQBUFS)\x00" as
                        *const u8 as *const libc::c_char);
    }
    /* v4l2_close v4l2_open device */
    if (*vdo).fd >= 0 as libc::c_int {
        close((*vdo).fd);
        (*vdo).fd = -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn v4l2_mmap_buffers(mut vdo: *mut zbar_video_t)
 -> libc::c_int {
    let mut vbuf: v4l2_buffer =
        v4l2_buffer{index: 0,
                    type_0: 0,
                    bytesused: 0,
                    flags: 0,
                    field: 0,
                    timestamp: timeval{tv_sec: 0, tv_usec: 0,},
                    timecode:
                        v4l2_timecode{type_0: 0,
                                      flags: 0,
                                      frames: 0,
                                      seconds: 0,
                                      minutes: 0,
                                      hours: 0,
                                      userbits: [0; 4],},
                    sequence: 0,
                    memory: 0,
                    m: C2RustUnnamed_3{offset: 0,},
                    length: 0,
                    reserved2: 0,
                    c2rust_unnamed: C2RustUnnamed_2{request_fd: 0,},};
    memset(&mut vbuf as *mut v4l2_buffer as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<v4l2_buffer>() as libc::c_ulong);
    vbuf.type_0 = V4L2_BUF_TYPE_VIDEO_CAPTURE as libc::c_int as __u32;
    vbuf.memory = V4L2_MEMORY_MMAP as libc::c_int as __u32;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*vdo).num_images {
        vbuf.index = i as __u32;
        if ioctl((*vdo).fd,
                 ((2 as libc::c_uint | 1 as libc::c_uint) <<
                      0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int +
                          14 as libc::c_int |
                      (('V' as i32) << 0 as libc::c_int + 8 as libc::c_int) as
                          libc::c_uint |
                      ((9 as libc::c_int) << 0 as libc::c_int) as
                          libc::c_uint) as libc::c_ulong |
                     (::std::mem::size_of::<v4l2_buffer>() as libc::c_ulong)
                         <<
                         0 as libc::c_int + 8 as libc::c_int +
                             8 as libc::c_int, &mut vbuf as *mut v4l2_buffer)
               < 0 as libc::c_int {
            /* FIXME cleanup */
            return err_capture(vdo as *const libc::c_void, SEV_ERROR,
                               ZBAR_ERR_SYSTEM,
                               (*::std::mem::transmute::<&[u8; 18],
                                                         &[libc::c_char; 18]>(b"v4l2_mmap_buffers\x00")).as_ptr(),
                               b"querying video buffer (VIDIOC_QUERYBUF)\x00"
                                   as *const u8 as *const libc::c_char)
        }
        if (vbuf.length as libc::c_ulong) < (*vdo).datalen {
            fprintf(stderr,
                    b"WARNING: insufficient v4l2 video buffer size:\n\tvbuf[%d].length=%x datalen=%lx image=%d x %d %.4s(%08x)\n\x00"
                        as *const u8 as *const libc::c_char, i, vbuf.length,
                    (*vdo).datalen, (*vdo).width, (*vdo).height,
                    &mut (*vdo).format as *mut uint32_t as *mut libc::c_char,
                    (*vdo).format);
        }
        let mut img: *mut zbar_image_t = *(*vdo).images.offset(i as isize);
        (*img).datalen = vbuf.length as libc::c_ulong;
        (*img).data =
            mmap(0 as *mut libc::c_void, vbuf.length as size_t,
                 0x1 as libc::c_int | 0x2 as libc::c_int, 0x1 as libc::c_int,
                 (*vdo).fd, vbuf.m.offset as __off_t);
        if (*img).data == -(1 as libc::c_int) as *mut libc::c_void {
            /* FIXME cleanup */
            return err_capture(vdo as *const libc::c_void, SEV_ERROR,
                               ZBAR_ERR_SYSTEM,
                               (*::std::mem::transmute::<&[u8; 18],
                                                         &[libc::c_char; 18]>(b"v4l2_mmap_buffers\x00")).as_ptr(),
                               b"mapping video frame buffers\x00" as *const u8
                                   as *const libc::c_char)
        }
        if _zbar_verbosity >= 2 as libc::c_int {
            fprintf(stderr,
                    b"%s:     buf[%d] 0x%lx bytes @%p\n\x00" as *const u8 as
                        *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 18],
                                              &[libc::c_char; 18]>(b"v4l2_mmap_buffers\x00")).as_ptr(),
                    i, (*img).datalen, (*img).data);
        }
        i += 1
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn v4l2_set_format(mut vdo: *mut zbar_video_t,
                                     mut fmt: uint32_t) -> libc::c_int {
    let mut vfmt: v4l2_format =
        v4l2_format{type_0: 0,
                    fmt:
                        C2RustUnnamed_8{pix:
                                            v4l2_pix_format{width: 0,
                                                            height: 0,
                                                            pixelformat: 0,
                                                            field: 0,
                                                            bytesperline: 0,
                                                            sizeimage: 0,
                                                            colorspace: 0,
                                                            priv_0: 0,
                                                            flags: 0,
                                                            c2rust_unnamed:
                                                                C2RustUnnamed{ycbcr_enc:
                                                                                  0,},
                                                            quantization: 0,
                                                            xfer_func:
                                                                0,},},};
    let mut vpix: *mut v4l2_pix_format = &mut vfmt.fmt.pix;
    memset(&mut vfmt as *mut v4l2_format as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<v4l2_format>() as libc::c_ulong);
    vfmt.type_0 = V4L2_BUF_TYPE_VIDEO_CAPTURE as libc::c_int as __u32;
    (*vpix).width = (*vdo).width;
    (*vpix).height = (*vdo).height;
    (*vpix).pixelformat = fmt;
    (*vpix).field = V4L2_FIELD_NONE as libc::c_int as __u32;
    let mut rc: libc::c_int = 0 as libc::c_int;
    rc =
        ioctl((*vdo).fd,
              ((2 as libc::c_uint | 1 as libc::c_uint) <<
                   0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int +
                       14 as libc::c_int |
                   (('V' as i32) << 0 as libc::c_int + 8 as libc::c_int) as
                       libc::c_uint |
                   ((5 as libc::c_int) << 0 as libc::c_int) as libc::c_uint)
                  as libc::c_ulong |
                  (::std::mem::size_of::<v4l2_format>() as libc::c_ulong) <<
                      0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
              &mut vfmt as *mut v4l2_format);
    if rc < 0 as libc::c_int {
        /* several broken drivers return an error if we request
         * no interlacing (NB v4l2 spec violation)
         * ...try again with an interlaced request
         */
        if _zbar_verbosity >= 1 as libc::c_int {
            fprintf(stderr,
                    b"%s: VIDIOC_S_FMT returned %d(%d), trying interlaced...\n\x00"
                        as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 16],
                                              &[libc::c_char; 16]>(b"v4l2_set_format\x00")).as_ptr(),
                    rc, *__errno_location());
        }
        /* FIXME this might be _ANY once we can de-interlace */
        (*vpix).field = V4L2_FIELD_INTERLACED as libc::c_int as __u32;
        if ioctl((*vdo).fd,
                 ((2 as libc::c_uint | 1 as libc::c_uint) <<
                      0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int +
                          14 as libc::c_int |
                      (('V' as i32) << 0 as libc::c_int + 8 as libc::c_int) as
                          libc::c_uint |
                      ((5 as libc::c_int) << 0 as libc::c_int) as
                          libc::c_uint) as libc::c_ulong |
                     (::std::mem::size_of::<v4l2_format>() as libc::c_ulong)
                         <<
                         0 as libc::c_int + 8 as libc::c_int +
                             8 as libc::c_int, &mut vfmt as *mut v4l2_format)
               < 0 as libc::c_int {
            return err_capture_int(vdo as *const libc::c_void, SEV_ERROR,
                                   ZBAR_ERR_SYSTEM,
                                   (*::std::mem::transmute::<&[u8; 16],
                                                             &[libc::c_char; 16]>(b"v4l2_set_format\x00")).as_ptr(),
                                   b"setting format %x (VIDIOC_S_FMT)\x00" as
                                       *const u8 as *const libc::c_char,
                                   fmt as libc::c_int)
        }
        if _zbar_verbosity >= 0 as libc::c_int {
            fprintf(stderr,
                    b"%s: WARNING: broken driver returned error when non-interlaced format requested\n\x00"
                        as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 16],
                                              &[libc::c_char; 16]>(b"v4l2_set_format\x00")).as_ptr());
        }
    }
    let mut newfmt: v4l2_format =
        v4l2_format{type_0: 0,
                    fmt:
                        C2RustUnnamed_8{pix:
                                            v4l2_pix_format{width: 0,
                                                            height: 0,
                                                            pixelformat: 0,
                                                            field: 0,
                                                            bytesperline: 0,
                                                            sizeimage: 0,
                                                            colorspace: 0,
                                                            priv_0: 0,
                                                            flags: 0,
                                                            c2rust_unnamed:
                                                                C2RustUnnamed{ycbcr_enc:
                                                                                  0,},
                                                            quantization: 0,
                                                            xfer_func:
                                                                0,},},};
    let mut newpix: *mut v4l2_pix_format = &mut newfmt.fmt.pix;
    memset(&mut newfmt as *mut v4l2_format as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<v4l2_format>() as libc::c_ulong);
    newfmt.type_0 = V4L2_BUF_TYPE_VIDEO_CAPTURE as libc::c_int as __u32;
    if ioctl((*vdo).fd,
             ((2 as libc::c_uint | 1 as libc::c_uint) <<
                  0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int +
                      14 as libc::c_int |
                  (('V' as i32) << 0 as libc::c_int + 8 as libc::c_int) as
                      libc::c_uint |
                  ((4 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as
                 libc::c_ulong |
                 (::std::mem::size_of::<v4l2_format>() as libc::c_ulong) <<
                     0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
             &mut newfmt as *mut v4l2_format) < 0 as libc::c_int {
        return err_capture(vdo as *const libc::c_void, SEV_ERROR,
                           ZBAR_ERR_SYSTEM,
                           (*::std::mem::transmute::<&[u8; 16],
                                                     &[libc::c_char; 16]>(b"v4l2_set_format\x00")).as_ptr(),
                           b"querying format (VIDIOC_G_FMT)\x00" as *const u8
                               as *const libc::c_char)
    }
    if (*newpix).field != V4L2_FIELD_NONE as libc::c_int as libc::c_uint {
        err_capture(vdo as *const libc::c_void, SEV_WARNING, ZBAR_ERR_INVALID,
                    (*::std::mem::transmute::<&[u8; 16],
                                              &[libc::c_char; 16]>(b"v4l2_set_format\x00")).as_ptr(),
                    b"video driver only supports interlaced format, vertical scanning may not work\x00"
                        as *const u8 as *const libc::c_char);
    }
    if (*newpix).pixelformat != fmt {
        /* FIXME bpl/bpp checks? */
        return err_capture(vdo as *const libc::c_void, SEV_ERROR,
                           ZBAR_ERR_INVALID,
                           (*::std::mem::transmute::<&[u8; 16],
                                                     &[libc::c_char; 16]>(b"v4l2_set_format\x00")).as_ptr(),
                           b"video driver can\'t provide compatible format\x00"
                               as *const u8 as *const libc::c_char)
    }
    (*vdo).format = fmt;
    (*vdo).width = (*newpix).width;
    (*vdo).height = (*newpix).height;
    (*vdo).datalen = (*newpix).sizeimage as libc::c_ulong;
    if _zbar_verbosity >= 1 as libc::c_int {
        fprintf(stderr,
                b"%s: set new format: %.4s(%08x) %u x %u (0x%lx)\n\x00" as
                    *const u8 as *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 16],
                                          &[libc::c_char; 16]>(b"v4l2_set_format\x00")).as_ptr(),
                &mut (*vdo).format as *mut uint32_t as *mut libc::c_char,
                (*vdo).format, (*vdo).width, (*vdo).height, (*vdo).datalen);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn v4l2_init(mut vdo: *mut zbar_video_t, mut fmt: uint32_t)
 -> libc::c_int {
    let mut rb: v4l2_requestbuffers =
        v4l2_requestbuffers{count: 0,
                            type_0: 0,
                            memory: 0,
                            capabilities: 0,
                            reserved: [0; 1],};
    if v4l2_set_format(vdo, fmt) != 0 { return -(1 as libc::c_int) }
    memset(&mut rb as *mut v4l2_requestbuffers as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<v4l2_requestbuffers>() as libc::c_ulong);
    rb.count = (*vdo).num_images as __u32;
    rb.type_0 = V4L2_BUF_TYPE_VIDEO_CAPTURE as libc::c_int as __u32;
    if (*vdo).iomode as libc::c_uint ==
           VIDEO_MMAP as libc::c_int as libc::c_uint {
        rb.memory = V4L2_MEMORY_MMAP as libc::c_int as __u32
    } else { rb.memory = V4L2_MEMORY_USERPTR as libc::c_int as __u32 }
    if ioctl((*vdo).fd,
             ((2 as libc::c_uint | 1 as libc::c_uint) <<
                  0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int +
                      14 as libc::c_int |
                  (('V' as i32) << 0 as libc::c_int + 8 as libc::c_int) as
                      libc::c_uint |
                  ((8 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as
                 libc::c_ulong |
                 (::std::mem::size_of::<v4l2_requestbuffers>() as
                      libc::c_ulong) <<
                     0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
             &mut rb as *mut v4l2_requestbuffers) < 0 as libc::c_int {
        return err_capture(vdo as *const libc::c_void, SEV_ERROR,
                           ZBAR_ERR_SYSTEM,
                           (*::std::mem::transmute::<&[u8; 10],
                                                     &[libc::c_char; 10]>(b"v4l2_init\x00")).as_ptr(),
                           b"requesting video frame buffers (VIDIOC_REQBUFS)\x00"
                               as *const u8 as *const libc::c_char)
    }
    if rb.count == 0 {
        return err_capture(vdo as *const libc::c_void, SEV_ERROR,
                           ZBAR_ERR_INVALID,
                           (*::std::mem::transmute::<&[u8; 10],
                                                     &[libc::c_char; 10]>(b"v4l2_init\x00")).as_ptr(),
                           b"driver returned 0 buffers\x00" as *const u8 as
                               *const libc::c_char)
    }
    if (*vdo).num_images as libc::c_uint > rb.count {
        (*vdo).num_images = rb.count as libc::c_int
    }
    if _zbar_verbosity >= 1 as libc::c_int {
        fprintf(stderr,
                b"%s: using %u buffers (of %d requested)\n\x00" as *const u8
                    as *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 10],
                                          &[libc::c_char; 10]>(b"v4l2_init\x00")).as_ptr(),
                rb.count, (*vdo).num_images);
    }
    if (*vdo).iomode as libc::c_uint ==
           VIDEO_MMAP as libc::c_int as libc::c_uint {
        return v4l2_mmap_buffers(vdo)
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn v4l2_probe_iomode(mut vdo: *mut zbar_video_t)
 -> libc::c_int {
    let mut rb: v4l2_requestbuffers =
        v4l2_requestbuffers{count: 0,
                            type_0: 0,
                            memory: 0,
                            capabilities: 0,
                            reserved: [0; 1],};
    memset(&mut rb as *mut v4l2_requestbuffers as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<v4l2_requestbuffers>() as libc::c_ulong);
    rb.count = (*vdo).num_images as __u32;
    rb.type_0 = V4L2_BUF_TYPE_VIDEO_CAPTURE as libc::c_int as __u32;
    if (*vdo).iomode as libc::c_uint ==
           VIDEO_MMAP as libc::c_int as libc::c_uint {
        rb.memory = V4L2_MEMORY_MMAP as libc::c_int as __u32
    } else { rb.memory = V4L2_MEMORY_USERPTR as libc::c_int as __u32 }
    if ioctl((*vdo).fd,
             ((2 as libc::c_uint | 1 as libc::c_uint) <<
                  0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int +
                      14 as libc::c_int |
                  (('V' as i32) << 0 as libc::c_int + 8 as libc::c_int) as
                      libc::c_uint |
                  ((8 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as
                 libc::c_ulong |
                 (::std::mem::size_of::<v4l2_requestbuffers>() as
                      libc::c_ulong) <<
                     0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
             &mut rb as *mut v4l2_requestbuffers) < 0 as libc::c_int {
        if (*vdo).iomode as u64 != 0 {
            return err_capture_int(vdo as *const libc::c_void, SEV_ERROR,
                                   ZBAR_ERR_INVALID,
                                   (*::std::mem::transmute::<&[u8; 18],
                                                             &[libc::c_char; 18]>(b"v4l2_probe_iomode\x00")).as_ptr(),
                                   b"unsupported iomode requested (%d)\x00" as
                                       *const u8 as *const libc::c_char,
                                   (*vdo).iomode as libc::c_int)
        } else {
            if *__errno_location() != 22 as libc::c_int {
                return err_capture(vdo as *const libc::c_void, SEV_ERROR,
                                   ZBAR_ERR_SYSTEM,
                                   (*::std::mem::transmute::<&[u8; 18],
                                                             &[libc::c_char; 18]>(b"v4l2_probe_iomode\x00")).as_ptr(),
                                   b"querying streaming mode (VIDIOC_REQBUFS)\x00"
                                       as *const u8 as *const libc::c_char)
            }
        }
        err_capture(vdo as *const libc::c_void, SEV_WARNING, ZBAR_ERR_SYSTEM,
                    (*::std::mem::transmute::<&[u8; 18],
                                              &[libc::c_char; 18]>(b"v4l2_probe_iomode\x00")).as_ptr(),
                    b"USERPTR failed. Falling back to mmap\x00" as *const u8
                        as *const libc::c_char);
        (*vdo).iomode = VIDEO_MMAP
    } else {
        if (*vdo).iomode as u64 == 0 {
            rb.memory = V4L2_MEMORY_USERPTR as libc::c_int as __u32
        }
        /* Update the num_images with the max supported by the driver */
        if rb.count != 0 {
            (*vdo).num_images = rb.count as libc::c_int
        } else {
            err_capture(vdo as *const libc::c_void, SEV_WARNING,
                        ZBAR_ERR_SYSTEM,
                        (*::std::mem::transmute::<&[u8; 18],
                                                  &[libc::c_char; 18]>(b"v4l2_probe_iomode\x00")).as_ptr(),
                        b"Something is wrong: number of buffers returned by REQBUF is zero!\x00"
                            as *const u8 as *const libc::c_char);
        }
        /* requesting 0 buffers
         * This cleans up the buffers allocated previously on probe
         */
        rb.count = 0 as libc::c_int as __u32;
        if ioctl((*vdo).fd,
                 ((2 as libc::c_uint | 1 as libc::c_uint) <<
                      0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int +
                          14 as libc::c_int |
                      (('V' as i32) << 0 as libc::c_int + 8 as libc::c_int) as
                          libc::c_uint |
                      ((8 as libc::c_int) << 0 as libc::c_int) as
                          libc::c_uint) as libc::c_ulong |
                     (::std::mem::size_of::<v4l2_requestbuffers>() as
                          libc::c_ulong) <<
                         0 as libc::c_int + 8 as libc::c_int +
                             8 as libc::c_int,
                 &mut rb as *mut v4l2_requestbuffers) < 0 as libc::c_int {
            err_capture(vdo as *const libc::c_void, SEV_WARNING,
                        ZBAR_ERR_SYSTEM,
                        (*::std::mem::transmute::<&[u8; 18],
                                                  &[libc::c_char; 18]>(b"v4l2_probe_iomode\x00")).as_ptr(),
                        b"releasing video frame buffers (VIDIOC_REQBUFS)\x00"
                            as *const u8 as *const libc::c_char);
        }
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn v4l2_max_size(mut vdo: *mut zbar_video_t,
                                   mut pixfmt: uint32_t,
                                   mut max_width: *mut uint32_t,
                                   mut max_height: *mut uint32_t) {
    let mut mwidth: libc::c_int = 0 as libc::c_int;
    let mut mheight: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut frm: v4l2_frmsizeenum =
        v4l2_frmsizeenum{index: 0,
                         pixel_format: 0,
                         type_0: 0,
                         c2rust_unnamed:
                             C2RustUnnamed_0{discrete:
                                                 v4l2_frmsize_discrete{width:
                                                                           0,
                                                                       height:
                                                                           0,},},
                         reserved: [0; 2],};
    let mut current_block_11: u64;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        memset(&mut frm as *mut v4l2_frmsizeenum as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<v4l2_frmsizeenum>() as libc::c_ulong);
        frm.index = i as __u32;
        frm.pixel_format = pixfmt;
        if ioctl((*vdo).fd,
                 ((2 as libc::c_uint | 1 as libc::c_uint) <<
                      0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int +
                          14 as libc::c_int |
                      (('V' as i32) << 0 as libc::c_int + 8 as libc::c_int) as
                          libc::c_uint |
                      ((74 as libc::c_int) << 0 as libc::c_int) as
                          libc::c_uint) as libc::c_ulong |
                     (::std::mem::size_of::<v4l2_frmsizeenum>() as
                          libc::c_ulong) <<
                         0 as libc::c_int + 8 as libc::c_int +
                             8 as libc::c_int,
                 &mut frm as *mut v4l2_frmsizeenum) != 0 {
            break ;
        }
        match frm.type_0 {
            1 => {
                mwidth = frm.c2rust_unnamed.discrete.width as libc::c_int;
                mheight = frm.c2rust_unnamed.discrete.height as libc::c_int;
                current_block_11 = 4166486009154926805;
            }
            2 | 3 => {
                mwidth = frm.c2rust_unnamed.stepwise.max_width as libc::c_int;
                mheight =
                    frm.c2rust_unnamed.stepwise.max_height as libc::c_int;
                current_block_11 = 4166486009154926805;
            }
            _ => { current_block_11 = 16559507199688588974; }
        }
        match current_block_11 {
            4166486009154926805 => {
                if mwidth as libc::c_uint > *max_width {
                    *max_width = mwidth as uint32_t
                }
                if mheight as libc::c_uint > *max_height {
                    *max_height = mheight as uint32_t
                }
            }
            _ => { }
        }
        i += 1
    };
}
#[inline]
unsafe extern "C" fn v4l2_probe_formats(mut vdo: *mut zbar_video_t)
 -> libc::c_int {
    let mut n_formats: libc::c_int = 0 as libc::c_int;
    let mut n_emu_formats: libc::c_int = 0 as libc::c_int;
    let mut max_width: uint32_t = 0 as libc::c_int as uint32_t;
    let mut max_height: uint32_t = 0 as libc::c_int as uint32_t;
    if (*vdo).width != 0 && (*vdo).height != 0 {
        if _zbar_verbosity >= 1 as libc::c_int {
            fprintf(stderr,
                    b"%s: Caller requested an specific size: %d x %d\n\x00" as
                        *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 19],
                                              &[libc::c_char; 19]>(b"v4l2_probe_formats\x00")).as_ptr(),
                    (*vdo).width, (*vdo).height);
        }
    }
    if _zbar_verbosity >= 2 as libc::c_int {
        fprintf(stderr,
                b"%s: enumerating supported formats:\n\x00" as *const u8 as
                    *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 19],
                                          &[libc::c_char; 19]>(b"v4l2_probe_formats\x00")).as_ptr());
    }
    let mut desc: v4l2_fmtdesc =
        v4l2_fmtdesc{index: 0,
                     type_0: 0,
                     flags: 0,
                     description: [0; 32],
                     pixelformat: 0,
                     reserved: [0; 4],};
    memset(&mut desc as *mut v4l2_fmtdesc as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<v4l2_fmtdesc>() as libc::c_ulong);
    desc.type_0 = V4L2_BUF_TYPE_VIDEO_CAPTURE as libc::c_int as __u32;
    desc.index = 0 as libc::c_int as __u32;
    while desc.index < 64 as libc::c_int as libc::c_uint {
        if ioctl((*vdo).fd,
                 ((2 as libc::c_uint | 1 as libc::c_uint) <<
                      0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int +
                          14 as libc::c_int |
                      (('V' as i32) << 0 as libc::c_int + 8 as libc::c_int) as
                          libc::c_uint |
                      ((2 as libc::c_int) << 0 as libc::c_int) as
                          libc::c_uint) as libc::c_ulong |
                     (::std::mem::size_of::<v4l2_fmtdesc>() as libc::c_ulong)
                         <<
                         0 as libc::c_int + 8 as libc::c_int +
                             8 as libc::c_int, &mut desc as *mut v4l2_fmtdesc)
               < 0 as libc::c_int {
            break ;
        }
        if _zbar_verbosity >= 2 as libc::c_int {
            fprintf(stderr,
                    b"%s:     [%d] %.4s : %s%s%s\n\x00" as *const u8 as
                        *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 19],
                                              &[libc::c_char; 19]>(b"v4l2_probe_formats\x00")).as_ptr(),
                    desc.index,
                    &mut desc.pixelformat as *mut __u32 as *mut libc::c_char,
                    desc.description.as_mut_ptr(),
                    if desc.flags & 0x1 as libc::c_int as libc::c_uint != 0 {
                        b" COMPRESSED\x00" as *const u8 as *const libc::c_char
                    } else { b"\x00" as *const u8 as *const libc::c_char },
                    if desc.flags & 0x2 as libc::c_int as libc::c_uint != 0 {
                        b" EMULATED\x00" as *const u8 as *const libc::c_char
                    } else { b"\x00" as *const u8 as *const libc::c_char });
        }
        if desc.flags & 0x2 as libc::c_int as libc::c_uint != 0 {
            (*vdo).emu_formats =
                realloc((*vdo).emu_formats as *mut libc::c_void,
                        ((n_emu_formats + 2 as libc::c_int) as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint32_t>()
                                                             as
                                                             libc::c_ulong))
                    as *mut uint32_t;
            let fresh0 = n_emu_formats;
            n_emu_formats = n_emu_formats + 1;
            *(*vdo).emu_formats.offset(fresh0 as isize) = desc.pixelformat
        } else {
            (*vdo).formats =
                realloc((*vdo).formats as *mut libc::c_void,
                        ((n_formats + 2 as libc::c_int) as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint32_t>()
                                                             as
                                                             libc::c_ulong))
                    as *mut uint32_t;
            let fresh1 = n_formats;
            n_formats = n_formats + 1;
            *(*vdo).formats.offset(fresh1 as isize) = desc.pixelformat
        }
        if (*vdo).width == 0 || (*vdo).height == 0 {
            v4l2_max_size(vdo, desc.pixelformat, &mut max_width,
                          &mut max_height);
        }
        desc.index = desc.index.wrapping_add(1)
    }
    if (*vdo).width == 0 || (*vdo).height == 0 {
        if _zbar_verbosity >= 1 as libc::c_int {
            fprintf(stderr,
                    b"%s: Max supported size: %d x %d\n\x00" as *const u8 as
                        *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 19],
                                              &[libc::c_char; 19]>(b"v4l2_probe_formats\x00")).as_ptr(),
                    max_width, max_height);
        }
        if max_width != 0 && max_height != 0 {
            (*vdo).width = max_width;
            (*vdo).height = max_height
        } else {
            /* fallback to large size, driver reduces to max available */
            (*vdo).width =
                (640 as libc::c_int * 64 as libc::c_int) as libc::c_uint;
            (*vdo).height =
                (480 as libc::c_int * 64 as libc::c_int) as libc::c_uint
        }
    }
    if desc.index == 0 {
        return err_capture(vdo as *const libc::c_void, SEV_ERROR,
                           ZBAR_ERR_SYSTEM,
                           (*::std::mem::transmute::<&[u8; 19],
                                                     &[libc::c_char; 19]>(b"v4l2_probe_formats\x00")).as_ptr(),
                           b"enumerating video formats (VIDIOC_ENUM_FMT)\x00"
                               as *const u8 as *const libc::c_char)
    }
    if !(*vdo).formats.is_null() {
        *(*vdo).formats.offset(n_formats as isize) =
            0 as libc::c_int as uint32_t
    }
    if !(*vdo).emu_formats.is_null() {
        *(*vdo).emu_formats.offset(n_emu_formats as isize) =
            0 as libc::c_int as uint32_t
    }
    if (*vdo).formats.is_null() && !(*vdo).emu_formats.is_null() {
        /*
        * If only emu formats are available, just move them to vdo->formats.
        * This happens when libv4l detects that the only available fourcc
        * formats are webcam proprietary formats or bayer formats.
        */
        (*vdo).formats = (*vdo).emu_formats;
        (*vdo).emu_formats = 0 as *mut uint32_t
    }
    if _zbar_verbosity >= 2 as libc::c_int {
        fprintf(stderr,
                b"%s: Found %d formats and %d emulated formats.\n\x00" as
                    *const u8 as *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 19],
                                          &[libc::c_char; 19]>(b"v4l2_probe_formats\x00")).as_ptr(),
                n_formats, n_emu_formats);
    }
    let mut fmt: v4l2_format =
        v4l2_format{type_0: 0,
                    fmt:
                        C2RustUnnamed_8{pix:
                                            v4l2_pix_format{width: 0,
                                                            height: 0,
                                                            pixelformat: 0,
                                                            field: 0,
                                                            bytesperline: 0,
                                                            sizeimage: 0,
                                                            colorspace: 0,
                                                            priv_0: 0,
                                                            flags: 0,
                                                            c2rust_unnamed:
                                                                C2RustUnnamed{ycbcr_enc:
                                                                                  0,},
                                                            quantization: 0,
                                                            xfer_func:
                                                                0,},},};
    let mut pix: *mut v4l2_pix_format = &mut fmt.fmt.pix;
    memset(&mut fmt as *mut v4l2_format as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<v4l2_format>() as libc::c_ulong);
    fmt.type_0 = V4L2_BUF_TYPE_VIDEO_CAPTURE as libc::c_int as __u32;
    if ioctl((*vdo).fd,
             ((2 as libc::c_uint | 1 as libc::c_uint) <<
                  0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int +
                      14 as libc::c_int |
                  (('V' as i32) << 0 as libc::c_int + 8 as libc::c_int) as
                      libc::c_uint |
                  ((4 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as
                 libc::c_ulong |
                 (::std::mem::size_of::<v4l2_format>() as libc::c_ulong) <<
                     0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
             &mut fmt as *mut v4l2_format) < 0 as libc::c_int {
        return err_capture(vdo as *const libc::c_void, SEV_ERROR,
                           ZBAR_ERR_SYSTEM,
                           (*::std::mem::transmute::<&[u8; 19],
                                                     &[libc::c_char; 19]>(b"v4l2_probe_formats\x00")).as_ptr(),
                           b"querying current video format (VIDIO_G_FMT)\x00"
                               as *const u8 as *const libc::c_char)
    }
    if _zbar_verbosity >= 1 as libc::c_int {
        fprintf(stderr,
                b"%s: current format: %.4s(%08x) %u x %u%s (line=0x%x size=0x%x)\n\x00"
                    as *const u8 as *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 19],
                                          &[libc::c_char; 19]>(b"v4l2_probe_formats\x00")).as_ptr(),
                &mut (*pix).pixelformat as *mut __u32 as *mut libc::c_char,
                (*pix).pixelformat, (*pix).width, (*pix).height,
                if (*pix).field !=
                       V4L2_FIELD_NONE as libc::c_int as libc::c_uint {
                    b" INTERLACED\x00" as *const u8 as *const libc::c_char
                } else { b"\x00" as *const u8 as *const libc::c_char },
                (*pix).bytesperline, (*pix).sizeimage);
    }
    (*vdo).format = (*pix).pixelformat;
    (*vdo).datalen = (*pix).sizeimage as libc::c_ulong;
    if (*pix).width == (*vdo).width && (*pix).height == (*vdo).height {
        return 0 as libc::c_int
    }
    let mut maxfmt: v4l2_format =
        v4l2_format{type_0: 0,
                    fmt:
                        C2RustUnnamed_8{pix:
                                            v4l2_pix_format{width: 0,
                                                            height: 0,
                                                            pixelformat: 0,
                                                            field: 0,
                                                            bytesperline: 0,
                                                            sizeimage: 0,
                                                            colorspace: 0,
                                                            priv_0: 0,
                                                            flags: 0,
                                                            c2rust_unnamed:
                                                                C2RustUnnamed{ycbcr_enc:
                                                                                  0,},
                                                            quantization: 0,
                                                            xfer_func:
                                                                0,},},};
    let mut maxpix: *mut v4l2_pix_format = &mut maxfmt.fmt.pix;
    memcpy(&mut maxfmt as *mut v4l2_format as *mut libc::c_void,
           &mut fmt as *mut v4l2_format as *const libc::c_void,
           ::std::mem::size_of::<v4l2_format>() as libc::c_ulong);
    (*maxpix).width = (*vdo).width;
    (*maxpix).height = (*vdo).height;
    if _zbar_verbosity >= 1 as libc::c_int {
        fprintf(stderr,
                b"%s: setting requested size: %d x %d\n\x00" as *const u8 as
                    *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 19],
                                          &[libc::c_char; 19]>(b"v4l2_probe_formats\x00")).as_ptr(),
                (*vdo).width, (*vdo).height);
    }
    if ioctl((*vdo).fd,
             ((2 as libc::c_uint | 1 as libc::c_uint) <<
                  0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int +
                      14 as libc::c_int |
                  (('V' as i32) << 0 as libc::c_int + 8 as libc::c_int) as
                      libc::c_uint |
                  ((5 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as
                 libc::c_ulong |
                 (::std::mem::size_of::<v4l2_format>() as libc::c_ulong) <<
                     0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
             &mut maxfmt as *mut v4l2_format) < 0 as libc::c_int {
        if _zbar_verbosity >= 1 as libc::c_int {
            fprintf(stderr,
                    b"%s: set FAILED...trying to recover original format\n\x00"
                        as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 19],
                                              &[libc::c_char; 19]>(b"v4l2_probe_formats\x00")).as_ptr());
        }
        /* ignore errors (driver broken anyway) */
        ioctl((*vdo).fd,
              ((2 as libc::c_uint | 1 as libc::c_uint) <<
                   0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int +
                       14 as libc::c_int |
                   (('V' as i32) << 0 as libc::c_int + 8 as libc::c_int) as
                       libc::c_uint |
                   ((5 as libc::c_int) << 0 as libc::c_int) as libc::c_uint)
                  as libc::c_ulong |
                  (::std::mem::size_of::<v4l2_format>() as libc::c_ulong) <<
                      0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
              &mut fmt as *mut v4l2_format);
    }
    memset(&mut fmt as *mut v4l2_format as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<v4l2_format>() as libc::c_ulong);
    fmt.type_0 = V4L2_BUF_TYPE_VIDEO_CAPTURE as libc::c_int as __u32;
    if ioctl((*vdo).fd,
             ((2 as libc::c_uint | 1 as libc::c_uint) <<
                  0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int +
                      14 as libc::c_int |
                  (('V' as i32) << 0 as libc::c_int + 8 as libc::c_int) as
                      libc::c_uint |
                  ((4 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as
                 libc::c_ulong |
                 (::std::mem::size_of::<v4l2_format>() as libc::c_ulong) <<
                     0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
             &mut fmt as *mut v4l2_format) < 0 as libc::c_int {
        return err_capture(vdo as *const libc::c_void, SEV_ERROR,
                           ZBAR_ERR_SYSTEM,
                           (*::std::mem::transmute::<&[u8; 19],
                                                     &[libc::c_char; 19]>(b"v4l2_probe_formats\x00")).as_ptr(),
                           b"querying current video format (VIDIOC_G_FMT)\x00"
                               as *const u8 as *const libc::c_char)
    }
    if _zbar_verbosity >= 1 as libc::c_int {
        fprintf(stderr,
                b"%s: final format: %.4s(%08x) %u x %u%s (line=0x%x size=0x%x)\n\x00"
                    as *const u8 as *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 19],
                                          &[libc::c_char; 19]>(b"v4l2_probe_formats\x00")).as_ptr(),
                &mut (*pix).pixelformat as *mut __u32 as *mut libc::c_char,
                (*pix).pixelformat, (*pix).width, (*pix).height,
                if (*pix).field !=
                       V4L2_FIELD_NONE as libc::c_int as libc::c_uint {
                    b" INTERLACED\x00" as *const u8 as *const libc::c_char
                } else { b"\x00" as *const u8 as *const libc::c_char },
                (*pix).bytesperline, (*pix).sizeimage);
    }
    (*vdo).width = (*pix).width;
    (*vdo).height = (*pix).height;
    (*vdo).datalen = (*pix).sizeimage as libc::c_ulong;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn v4l2_reset_crop(mut vdo: *mut zbar_video_t)
 -> libc::c_int {
    /* check cropping */
    let mut ccap: v4l2_cropcap =
        v4l2_cropcap{type_0: 0,
                     bounds: v4l2_rect{left: 0, top: 0, width: 0, height: 0,},
                     defrect:
                         v4l2_rect{left: 0, top: 0, width: 0, height: 0,},
                     pixelaspect: v4l2_fract{numerator: 0, denominator: 0,},};
    memset(&mut ccap as *mut v4l2_cropcap as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<v4l2_cropcap>() as libc::c_ulong);
    ccap.type_0 = V4L2_BUF_TYPE_VIDEO_CAPTURE as libc::c_int as __u32;
    if ioctl((*vdo).fd,
             ((2 as libc::c_uint | 1 as libc::c_uint) <<
                  0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int +
                      14 as libc::c_int |
                  (('V' as i32) << 0 as libc::c_int + 8 as libc::c_int) as
                      libc::c_uint |
                  ((58 as libc::c_int) << 0 as libc::c_int) as libc::c_uint)
                 as libc::c_ulong |
                 (::std::mem::size_of::<v4l2_cropcap>() as libc::c_ulong) <<
                     0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
             &mut ccap as *mut v4l2_cropcap) < 0 as libc::c_int {
        return err_capture(vdo as *const libc::c_void, SEV_ERROR,
                           ZBAR_ERR_SYSTEM,
                           (*::std::mem::transmute::<&[u8; 16],
                                                     &[libc::c_char; 16]>(b"v4l2_reset_crop\x00")).as_ptr(),
                           b"querying crop support (VIDIOC_CROPCAP)\x00" as
                               *const u8 as *const libc::c_char)
    }
    if _zbar_verbosity >= 1 as libc::c_int {
        fprintf(stderr,
                b"%s: crop bounds: %d x %d @ (%d, %d)\n\x00" as *const u8 as
                    *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 16],
                                          &[libc::c_char; 16]>(b"v4l2_reset_crop\x00")).as_ptr(),
                ccap.bounds.width, ccap.bounds.height, ccap.bounds.left,
                ccap.bounds.top);
    }
    if _zbar_verbosity >= 1 as libc::c_int {
        fprintf(stderr,
                b"%s: current crop win: %d x %d @ (%d, %d) aspect %d / %d\n\x00"
                    as *const u8 as *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 16],
                                          &[libc::c_char; 16]>(b"v4l2_reset_crop\x00")).as_ptr(),
                ccap.defrect.width, ccap.defrect.height, ccap.defrect.left,
                ccap.defrect.top, ccap.pixelaspect.numerator,
                ccap.pixelaspect.denominator);
    }
    /* reset crop parameters */
    let mut crop: v4l2_crop =
        v4l2_crop{type_0: 0,
                  c: v4l2_rect{left: 0, top: 0, width: 0, height: 0,},};
    memset(&mut crop as *mut v4l2_crop as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<v4l2_crop>() as libc::c_ulong);
    crop.type_0 = V4L2_BUF_TYPE_VIDEO_CAPTURE as libc::c_int as __u32;
    crop.c = ccap.defrect;
    if ioctl((*vdo).fd,
             ((1 as libc::c_uint) <<
                  0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int +
                      14 as libc::c_int |
                  (('V' as i32) << 0 as libc::c_int + 8 as libc::c_int) as
                      libc::c_uint |
                  ((60 as libc::c_int) << 0 as libc::c_int) as libc::c_uint)
                 as libc::c_ulong |
                 (::std::mem::size_of::<v4l2_crop>() as libc::c_ulong) <<
                     0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
             &mut crop as *mut v4l2_crop) < 0 as libc::c_int &&
           *__errno_location() != 22 as libc::c_int {
        return err_capture(vdo as *const libc::c_void, SEV_ERROR,
                           ZBAR_ERR_SYSTEM,
                           (*::std::mem::transmute::<&[u8; 16],
                                                     &[libc::c_char; 16]>(b"v4l2_reset_crop\x00")).as_ptr(),
                           b"setting default crop window (VIDIOC_S_CROP)\x00"
                               as *const u8 as *const libc::c_char)
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn v4l2_ctrl_type(mut type_0: uint32_t)
 -> *const libc::c_char {
    match type_0 {
        1 => {
            // All controls below are available since, at least, Kernel 2.6.31
            return b"int\x00" as *const u8 as *const libc::c_char
        }
        2 => { return b"bool\x00" as *const u8 as *const libc::c_char }
        3 => { return b"menu\x00" as *const u8 as *const libc::c_char }
        4 => { return b"button\x00" as *const u8 as *const libc::c_char }
        5 => { return b"int64\x00" as *const u8 as *const libc::c_char }
        6 => { return b"ctrl class\x00" as *const u8 as *const libc::c_char }
        7 => { return b"string\x00" as *const u8 as *const libc::c_char }
        _ => { return b"unknown\x00" as *const u8 as *const libc::c_char }
    };
}
unsafe extern "C" fn v4l2_ctrl_class(mut class: uint32_t)
 -> *const libc::c_char {
    match class {
        9961472 => {
            // All classes below are available since, at least, Kernel 2.6.31
            return b"User\x00" as *const u8 as *const libc::c_char
        }
        10027008 => {
            return b"MPEG-compression\x00" as *const u8 as *const libc::c_char
        }
        10092544 => {
            return b"Camera\x00" as *const u8 as *const libc::c_char
        }
        10158080 => {
            return b"FM Modulator\x00" as *const u8 as *const libc::c_char
        }
        10223616 => {
            // Newer classes added up to Kernel 3.16
            return b"Camera flash\x00" as *const u8 as *const libc::c_char
        }
        10289152 => {
            return b"JPEG-compression\x00" as *const u8 as *const libc::c_char
        }
        10354688 => {
            return b"Image source\x00" as *const u8 as *const libc::c_char
        }
        10420224 => {
            return b"Image processing\x00" as *const u8 as *const libc::c_char
        }
        10485760 => {
            return b"Digital Video\x00" as *const u8 as *const libc::c_char
        }
        10551296 => {
            return b"FM Receiver\x00" as *const u8 as *const libc::c_char
        }
        10616832 => {
            return b"RF tuner\x00" as *const u8 as *const libc::c_char
        }
        10682368 => {
            return b"Detection\x00" as *const u8 as *const libc::c_char
        }
        _ => { return b"Unknown\x00" as *const u8 as *const libc::c_char }
    };
}
// return values: 1: ignore, 0: added, -1: silently ignore
unsafe extern "C" fn v4l2_add_control(mut vdo: *mut zbar_video_t,
                                      mut query: *mut v4l2_query_ext_ctrl,
                                      mut ptr:
                                          *mut *mut video_controls_priv_s)
 -> libc::c_int {
    // Control is disabled, ignore it. Please notice that disabled controls
    // can be re-enabled. The right thing here would be to get those too,
    // and add a logic to
    if (*query).flags & 0x1 as libc::c_int as libc::c_uint != 0 {
        return 1 as libc::c_int
    }
    /* Silently ignore control classes */
    if (*query).type_0 ==
           V4L2_CTRL_TYPE_CTRL_CLASS as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int)
    }
    // There's not much sense on displaying permanent read-only controls
    if (*query).flags & 0x4 as libc::c_int as libc::c_uint != 0 {
        return 1 as libc::c_int
    }
    // Allocate a new element on the linked list
    if (*vdo).controls.is_null() {
        *ptr =
            calloc(1 as libc::c_int as libc::c_ulong,
                   ::std::mem::size_of::<video_controls_priv_s>() as
                       libc::c_ulong) as *mut video_controls_priv_s;
        (*vdo).controls = *ptr as *mut libc::c_void as *mut video_controls_s
    } else {
        (**ptr).s.next =
            calloc(1 as libc::c_int as libc::c_ulong,
                   ::std::mem::size_of::<video_controls_priv_s>() as
                       libc::c_ulong);
        *ptr = (**ptr).s.next as *mut video_controls_priv_s
    }
    // Fill control data
    (**ptr).id = (*query).id;
    (**ptr).s.name =
        strdup((*query).name.as_mut_ptr() as *const libc::c_char);
    (**ptr).s.group =
        strdup(v4l2_ctrl_class(((*query).id as libc::c_ulong &
                                    0xfff0000 as libc::c_ulong) as uint32_t));
    match (*query).type_0 {
        1 => {
            (**ptr).s.type_0 = VIDEO_CNTL_INTEGER;
            (**ptr).s.min = (*query).minimum as int64_t;
            (**ptr).s.max = (*query).maximum as int64_t;
            (**ptr).s.def = (*query).default_value as int64_t;
            (**ptr).s.step = (*query).step as uint64_t;
            return 0 as libc::c_int
        }
        5 => {
            (**ptr).s.type_0 = VIDEO_CNTL_INTEGER64;
            (**ptr).s.min = (*query).minimum as int64_t;
            (**ptr).s.max = (*query).maximum as int64_t;
            (**ptr).s.def = (*query).default_value as int64_t;
            (**ptr).s.step = (*query).step as uint64_t;
            return 0 as libc::c_int
        }
        2 => {
            (**ptr).s.type_0 = VIDEO_CNTL_BOOLEAN;
            return 0 as libc::c_int
        }
        4 => { (**ptr).s.type_0 = VIDEO_CNTL_BUTTON; return 0 as libc::c_int }
        7 => { (**ptr).s.type_0 = VIDEO_CNTL_STRING; return 0 as libc::c_int }
        3 => {
            let mut menu: v4l2_querymenu =
                v4l2_querymenu{id: 0,
                               index: 0,
                               c2rust_unnamed:
                                   C2RustUnnamed_6{name: [0; 32],},
                               reserved: 0,};
            let mut first: *mut video_control_menu_s =
                0 as *mut video_control_menu_s;
            let mut p: *mut video_control_menu_s =
                0 as *mut video_control_menu_s;
            let mut n_menu: libc::c_int = 0 as libc::c_int;
            memset(&mut menu as *mut v4l2_querymenu as *mut libc::c_void,
                   0 as libc::c_int,
                   ::std::mem::size_of::<v4l2_querymenu>() as libc::c_ulong);
            menu.id = (*query).id;
            menu.index = (*query).minimum as __u32;
            while menu.index as libc::c_longlong <= (*query).maximum {
                if ioctl((*vdo).fd,
                         ((2 as libc::c_uint | 1 as libc::c_uint) <<
                              0 as libc::c_int + 8 as libc::c_int +
                                  8 as libc::c_int + 14 as libc::c_int |
                              (('V' as i32) <<
                                   0 as libc::c_int + 8 as libc::c_int) as
                                  libc::c_uint |
                              ((37 as libc::c_int) << 0 as libc::c_int) as
                                  libc::c_uint) as libc::c_ulong |
                             (::std::mem::size_of::<v4l2_querymenu>() as
                                  libc::c_ulong) <<
                                 0 as libc::c_int + 8 as libc::c_int +
                                     8 as libc::c_int,
                         &mut menu as *mut v4l2_querymenu) == 0 {
                    first =
                        realloc(first as *mut libc::c_void,
                                ((n_menu + 1 as libc::c_int) as
                                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<video_control_menu_t>()
                                                                     as
                                                                     libc::c_ulong))
                            as *mut video_control_menu_s;
                    p =
                        &mut *first.offset(n_menu as isize) as
                            *mut video_control_menu_s;
                    (*p).value = menu.index as int64_t;
                    (*p).name =
                        strdup(menu.c2rust_unnamed.name.as_mut_ptr() as
                                   *const libc::c_char);
                    n_menu += 1
                }
                menu.index = menu.index.wrapping_add(1)
            }
            (**ptr).s.menu = first;
            (**ptr).s.menu_size = n_menu as libc::c_uint;
            (**ptr).s.min = (*query).minimum as int64_t;
            (**ptr).s.max = (*query).maximum as int64_t;
            (**ptr).s.def = (*query).default_value as int64_t;
            (**ptr).s.type_0 = VIDEO_CNTL_MENU;
            return 0 as libc::c_int
        }
        _ => { return 1 as libc::c_int }
    };
}
#[no_mangle]
pub unsafe extern "C" fn v4l2_free_controls(mut vdo: *mut zbar_video_t) {
    let mut i: libc::c_int = 0;
    if !(*vdo).controls.is_null() {
        let mut p: *mut video_controls_s = (*vdo).controls;
        while !p.is_null() {
            free((*p).name as *mut libc::c_void);
            free((*p).group as *mut libc::c_void);
            if !(*p).menu.is_null() {
                i = 0 as libc::c_int;
                while (i as libc::c_uint) < (*p).menu_size {
                    free((*(*p).menu.offset(i as isize)).name as
                             *mut libc::c_void);
                    i += 1
                }
                free((*p).menu as *mut libc::c_void);
            }
            p = (*p).next as *mut video_controls_s
        }
        free((*vdo).controls as *mut libc::c_void);
    }
    (*vdo).controls = 0 as *mut video_controls_s;
}
unsafe extern "C" fn v4l2_query_controls(mut vdo: *mut zbar_video_t)
 -> libc::c_int {
    let mut ptr: *mut video_controls_priv_s = 0 as *mut video_controls_priv_s;
    let mut query: v4l2_query_ext_ctrl =
        v4l2_query_ext_ctrl{id: 0,
                            type_0: 0,
                            name: [0; 32],
                            minimum: 0,
                            maximum: 0,
                            step: 0,
                            default_value: 0,
                            flags: 0,
                            elem_size: 0,
                            elems: 0,
                            nr_of_dims: 0,
                            dims: [0; 4],
                            reserved: [0; 32],};
    let mut ignore: libc::c_int = 0;
    let mut old_class: *const libc::c_char = 0 as *const libc::c_char;
    // Free controls list if not NULL
    v4l2_free_controls(vdo);
    memset(&mut query as *mut v4l2_query_ext_ctrl as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<v4l2_query_ext_ctrl>() as libc::c_ulong);
    query.id = 0x80000000 as libc::c_uint;
    while ioctl((*vdo).fd,
                ((2 as libc::c_uint | 1 as libc::c_uint) <<
                     0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int +
                         14 as libc::c_int |
                     (('V' as i32) << 0 as libc::c_int + 8 as libc::c_int) as
                         libc::c_uint |
                     ((103 as libc::c_int) << 0 as libc::c_int) as
                         libc::c_uint) as libc::c_ulong |
                    (::std::mem::size_of::<v4l2_query_ext_ctrl>() as
                         libc::c_ulong) <<
                        0 as libc::c_int + 8 as libc::c_int +
                            8 as libc::c_int,
                &mut query as *mut v4l2_query_ext_ctrl) == 0 {
        ignore = v4l2_add_control(vdo, &mut query, &mut ptr);
        if ignore >= 0 as libc::c_int && _zbar_verbosity != 0 {
            let mut i: libc::c_int = 0;
            let mut class: *const libc::c_char =
                v4l2_ctrl_class((query.id as libc::c_ulong &
                                     0xfff0000 as libc::c_ulong) as uint32_t);
            if class != old_class {
                if _zbar_verbosity >= 1 as libc::c_int {
                    fprintf(stderr,
                            b"%s: Control class %s:\n\x00" as *const u8 as
                                *const libc::c_char,
                            (*::std::mem::transmute::<&[u8; 20],
                                                      &[libc::c_char; 20]>(b"v4l2_query_controls\x00")).as_ptr(),
                            class);
                }
            }
            if _zbar_verbosity >= 1 as libc::c_int {
                fprintf(stderr,
                        b"%s: %-10s %-32s - 0x%x%s\n\x00" as *const u8 as
                            *const libc::c_char,
                        (*::std::mem::transmute::<&[u8; 20],
                                                  &[libc::c_char; 20]>(b"v4l2_query_controls\x00")).as_ptr(),
                        v4l2_ctrl_type(query.type_0), query.name.as_mut_ptr(),
                        query.id,
                        if ignore != 0 {
                            b" - Ignored\x00" as *const u8 as
                                *const libc::c_char
                        } else {
                            b"\x00" as *const u8 as *const libc::c_char
                        });
            }
            i = 0 as libc::c_int;
            while (i as libc::c_uint) < (*ptr).s.menu_size {
                if _zbar_verbosity >= 1 as libc::c_int {
                    fprintf(stderr,
                            b"%s:            %ld: %s\n\x00" as *const u8 as
                                *const libc::c_char,
                            (*::std::mem::transmute::<&[u8; 20],
                                                      &[libc::c_char; 20]>(b"v4l2_query_controls\x00")).as_ptr(),
                            (*(*ptr).s.menu.offset(i as isize)).value,
                            (*(*ptr).s.menu.offset(i as isize)).name);
                }
                i += 1
            }
            old_class = class
        }
        query.id |= 0x80000000 as libc::c_uint
    }
    return 0 as libc::c_int;
}
/* * locate a control entry
 */
unsafe extern "C" fn v4l2_g_control_def(mut vdo: *mut zbar_video_t,
                                        mut name: *const libc::c_char)
 -> *mut video_controls_priv_s {
    let mut p: *mut video_controls_priv_s =
        (*vdo).controls as *mut libc::c_void as
            *mut video_controls_priv_s; // we have no such a control on the list
    while !p.is_null() {
        if strcasecmp((*p).s.name, name) == 0 {
            break ; // we have no such a control on the list
        }
        p = (*p).s.next as *mut video_controls_priv_s
    }
    if (*p).s.name.is_null() {
        if _zbar_verbosity >= 1 as libc::c_int {
            fprintf(stderr,
                    b"%s: Control not found: %s\x00" as *const u8 as
                        *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 19],
                                              &[libc::c_char; 19]>(b"v4l2_g_control_def\x00")).as_ptr(),
                    name);
        }
        return 0 as *mut video_controls_priv_s
    }
    return p;
}
unsafe extern "C" fn v4l2_s_control(mut vdo: *mut zbar_video_t,
                                    mut name: *const libc::c_char,
                                    mut value: *mut libc::c_void)
 -> libc::c_int {
    let mut ctrls: v4l2_ext_controls =
        v4l2_ext_controls{c2rust_unnamed: C2RustUnnamed_5{ctrl_class: 0,},
                          count: 0,
                          error_idx: 0,
                          request_fd: 0,
                          reserved: [0; 1],
                          controls: 0 as *mut v4l2_ext_control,};
    let mut c: v4l2_ext_control =
        v4l2_ext_control{id: 0,
                         size: 0,
                         reserved2: [0; 1],
                         c2rust_unnamed: C2RustUnnamed_4{value: 0,},};
    let mut p: *mut video_controls_priv_s = 0 as *mut video_controls_priv_s;
    p = v4l2_g_control_def(vdo, name);
    if p.is_null() { return ZBAR_ERR_UNSUPPORTED as libc::c_int }
    memset(&mut ctrls as *mut v4l2_ext_controls as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<v4l2_ext_controls>() as libc::c_ulong);
    ctrls.count = 1 as libc::c_int as __u32;
    ctrls.c2rust_unnamed.which = 0 as libc::c_int as __u32;
    ctrls.controls = &mut c;
    memset(&mut c as *mut v4l2_ext_control as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<v4l2_ext_control>() as libc::c_ulong);
    c.id = (*p).id;
    match (*p).s.type_0 as libc::c_uint {
        1 | 6 | 3 | 2 => {
            c.c2rust_unnamed.value = *(value as *mut libc::c_int)
        }
        _ => { return ZBAR_ERR_UNSUPPORTED as libc::c_int }
    }
    let mut rv: libc::c_int =
        ioctl((*vdo).fd,
              ((2 as libc::c_uint | 1 as libc::c_uint) <<
                   0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int +
                       14 as libc::c_int |
                   (('V' as i32) << 0 as libc::c_int + 8 as libc::c_int) as
                       libc::c_uint |
                   ((72 as libc::c_int) << 0 as libc::c_int) as libc::c_uint)
                  as libc::c_ulong |
                  (::std::mem::size_of::<v4l2_ext_controls>() as
                       libc::c_ulong) <<
                      0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
              &mut ctrls as *mut v4l2_ext_controls);
    if rv != 0 {
        if _zbar_verbosity >= 1 as libc::c_int {
            fprintf(stderr,
                    b"%s: v4l2 set user control \"%s\" returned %d\n\x00" as
                        *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 15],
                                              &[libc::c_char; 15]>(b"v4l2_s_control\x00")).as_ptr(),
                    (*p).s.name, rv);
        }
        rv = ZBAR_ERR_INVALID as libc::c_int
    }
    if _zbar_verbosity >= 1 as libc::c_int {
        fprintf(stderr,
                b"%s: %-32s id: 0x%x set to value %d\n\x00" as *const u8 as
                    *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 15],
                                          &[libc::c_char; 15]>(b"v4l2_s_control\x00")).as_ptr(),
                name, (*p).id, *(value as *mut libc::c_int));
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn v4l2_g_control(mut vdo: *mut zbar_video_t,
                                    mut name: *const libc::c_char,
                                    mut value: *mut libc::c_void)
 -> libc::c_int {
    let mut ctrls: v4l2_ext_controls =
        v4l2_ext_controls{c2rust_unnamed: C2RustUnnamed_5{ctrl_class: 0,},
                          count: 0,
                          error_idx: 0,
                          request_fd: 0,
                          reserved: [0; 1],
                          controls: 0 as *mut v4l2_ext_control,};
    let mut c: v4l2_ext_control =
        v4l2_ext_control{id: 0,
                         size: 0,
                         reserved2: [0; 1],
                         c2rust_unnamed: C2RustUnnamed_4{value: 0,},};
    let mut p: *mut video_controls_priv_s = 0 as *mut video_controls_priv_s;
    p = v4l2_g_control_def(vdo, name);
    if p.is_null() { return ZBAR_ERR_UNSUPPORTED as libc::c_int }
    memset(&mut ctrls as *mut v4l2_ext_controls as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<v4l2_ext_controls>() as libc::c_ulong);
    ctrls.count = 1 as libc::c_int as __u32;
    ctrls.c2rust_unnamed.which = 0 as libc::c_int as __u32;
    ctrls.controls = &mut c;
    memset(&mut c as *mut v4l2_ext_control as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<v4l2_ext_control>() as libc::c_ulong);
    c.id = (*p).id;
    let mut rv: libc::c_int =
        ioctl((*vdo).fd,
              ((2 as libc::c_uint | 1 as libc::c_uint) <<
                   0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int +
                       14 as libc::c_int |
                   (('V' as i32) << 0 as libc::c_int + 8 as libc::c_int) as
                       libc::c_uint |
                   ((71 as libc::c_int) << 0 as libc::c_int) as libc::c_uint)
                  as libc::c_ulong |
                  (::std::mem::size_of::<v4l2_ext_controls>() as
                       libc::c_ulong) <<
                      0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
              &mut ctrls as *mut v4l2_ext_controls);
    if rv != 0 {
        if _zbar_verbosity >= 1 as libc::c_int {
            fprintf(stderr,
                    b"%s: v4l2 get user control \"%s\" returned %d\n\x00" as
                        *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 15],
                                              &[libc::c_char; 15]>(b"v4l2_g_control\x00")).as_ptr(),
                    (*p).s.name, rv);
        }
        return ZBAR_ERR_UNSUPPORTED as libc::c_int
    }
    match (*p).s.type_0 as libc::c_uint {
        1 | 6 | 3 | 2 => {
            *(value as *mut libc::c_int) = c.c2rust_unnamed.value;
            if _zbar_verbosity >= 1 as libc::c_int {
                fprintf(stderr,
                        b"%s: v4l2 get user control \"%s\" = %d\n\x00" as
                            *const u8 as *const libc::c_char,
                        (*::std::mem::transmute::<&[u8; 15],
                                                  &[libc::c_char; 15]>(b"v4l2_g_control\x00")).as_ptr(),
                        (*p).s.name, c.c2rust_unnamed.value);
            }
            return 0 as libc::c_int
        }
        _ => { return ZBAR_ERR_UNSUPPORTED as libc::c_int }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _zbar_v4l2_probe(mut vdo: *mut zbar_video_t)
 -> libc::c_int {
    /* check capabilities */
    let mut vcap: v4l2_capability =
        v4l2_capability{driver: [0; 16],
                        card: [0; 32],
                        bus_info: [0; 32],
                        version: 0,
                        capabilities: 0,
                        device_caps: 0,
                        reserved: [0; 3],};
    memset(&mut vcap as *mut v4l2_capability as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<v4l2_capability>() as libc::c_ulong);
    if ioctl((*vdo).fd,
             ((2 as libc::c_uint) <<
                  0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int +
                      14 as libc::c_int |
                  (('V' as i32) << 0 as libc::c_int + 8 as libc::c_int) as
                      libc::c_uint |
                  ((0 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as
                 libc::c_ulong |
                 (::std::mem::size_of::<v4l2_capability>() as libc::c_ulong)
                     <<
                     0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
             &mut vcap as *mut v4l2_capability) < 0 as libc::c_int {
        return err_capture(vdo as *const libc::c_void, SEV_WARNING,
                           ZBAR_ERR_UNSUPPORTED,
                           (*::std::mem::transmute::<&[u8; 17],
                                                     &[libc::c_char; 17]>(b"_zbar_v4l2_probe\x00")).as_ptr(),
                           b"video4linux version 2 not supported (VIDIOC_QUERYCAP)\x00"
                               as *const u8 as *const libc::c_char)
    }
    if _zbar_verbosity >= 1 as libc::c_int {
        fprintf(stderr,
                b"%s: %.32s on %.32s driver %.16s (version %u.%u.%u)\n\x00" as
                    *const u8 as *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 17],
                                          &[libc::c_char; 17]>(b"_zbar_v4l2_probe\x00")).as_ptr(),
                vcap.card.as_mut_ptr(),
                if vcap.bus_info[0 as libc::c_int as usize] as libc::c_int !=
                       0 {
                    vcap.bus_info.as_mut_ptr() as *mut libc::c_char
                } else {
                    b"<unknown>\x00" as *const u8 as *const libc::c_char
                }, vcap.driver.as_mut_ptr(),
                vcap.version >> 16 as libc::c_int &
                    0xff as libc::c_int as libc::c_uint,
                vcap.version >> 8 as libc::c_int &
                    0xff as libc::c_int as libc::c_uint,
                vcap.version & 0xff as libc::c_int as libc::c_uint);
    }
    if _zbar_verbosity >= 1 as libc::c_int {
        fprintf(stderr,
                b"%s:     capabilities:%s%s%s%s\n\x00" as *const u8 as
                    *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 17],
                                          &[libc::c_char; 17]>(b"_zbar_v4l2_probe\x00")).as_ptr(),
                if vcap.capabilities & 0x1 as libc::c_int as libc::c_uint != 0
                   {
                    b" CAPTURE\x00" as *const u8 as *const libc::c_char
                } else { b"\x00" as *const u8 as *const libc::c_char },
                if vcap.capabilities & 0x4 as libc::c_int as libc::c_uint != 0
                   {
                    b" OVERLAY\x00" as *const u8 as *const libc::c_char
                } else { b"\x00" as *const u8 as *const libc::c_char },
                if vcap.capabilities &
                       0x1000000 as libc::c_int as libc::c_uint != 0 {
                    b" READWRITE\x00" as *const u8 as *const libc::c_char
                } else { b"\x00" as *const u8 as *const libc::c_char },
                if vcap.capabilities &
                       0x4000000 as libc::c_int as libc::c_uint != 0 {
                    b" STREAMING\x00" as *const u8 as *const libc::c_char
                } else { b"\x00" as *const u8 as *const libc::c_char });
    }
    if vcap.capabilities & 0x1 as libc::c_int as libc::c_uint == 0 ||
           vcap.capabilities &
               (0x1000000 as libc::c_int | 0x4000000 as libc::c_int) as
                   libc::c_uint == 0 {
        return err_capture(vdo as *const libc::c_void, SEV_WARNING,
                           ZBAR_ERR_UNSUPPORTED,
                           (*::std::mem::transmute::<&[u8; 17],
                                                     &[libc::c_char; 17]>(b"_zbar_v4l2_probe\x00")).as_ptr(),
                           b"v4l2 device does not support usable CAPTURE\x00"
                               as *const u8 as *const libc::c_char)
    }
    (v4l2_reset_crop(vdo)) != 0;
    if v4l2_probe_formats(vdo) != 0 { return -(1 as libc::c_int) }
    if v4l2_query_controls(vdo) != 0 { return -(1 as libc::c_int) }
    /* FIXME report error and fallback to readwrite? (if supported...) */
    if (*vdo).iomode as libc::c_uint !=
           VIDEO_READWRITE as libc::c_int as libc::c_uint &&
           vcap.capabilities & 0x4000000 as libc::c_int as libc::c_uint != 0
           && v4l2_probe_iomode(vdo) != 0 {
        return -(1 as libc::c_int)
    }
    if (*vdo).iomode as u64 == 0 { (*vdo).iomode = VIDEO_READWRITE }
    if _zbar_verbosity >= 1 as libc::c_int {
        fprintf(stderr,
                b"%s: using I/O mode: %s\n\x00" as *const u8 as
                    *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 17],
                                          &[libc::c_char; 17]>(b"_zbar_v4l2_probe\x00")).as_ptr(),
                if (*vdo).iomode as libc::c_uint ==
                       VIDEO_READWRITE as libc::c_int as libc::c_uint {
                    b"READWRITE\x00" as *const u8 as *const libc::c_char
                } else if (*vdo).iomode as libc::c_uint ==
                              VIDEO_MMAP as libc::c_int as libc::c_uint {
                    b"MMAP\x00" as *const u8 as *const libc::c_char
                } else if (*vdo).iomode as libc::c_uint ==
                              VIDEO_USERPTR as libc::c_int as libc::c_uint {
                    b"USERPTR\x00" as *const u8 as *const libc::c_char
                } else {
                    b"<UNKNOWN>\x00" as *const u8 as *const libc::c_char
                });
    }
    (*vdo).intf = VIDEO_V4L2;
    (*vdo).init =
        Some(v4l2_init as
                 unsafe extern "C" fn(_: *mut zbar_video_t, _: uint32_t)
                     -> libc::c_int);
    (*vdo).cleanup =
        Some(v4l2_cleanup as
                 unsafe extern "C" fn(_: *mut zbar_video_t) -> libc::c_int);
    (*vdo).start =
        Some(v4l2_start as
                 unsafe extern "C" fn(_: *mut zbar_video_t) -> libc::c_int);
    (*vdo).stop =
        Some(v4l2_stop as
                 unsafe extern "C" fn(_: *mut zbar_video_t) -> libc::c_int);
    (*vdo).nq =
        Some(v4l2_nq as
                 unsafe extern "C" fn(_: *mut zbar_video_t,
                                      _: *mut zbar_image_t) -> libc::c_int);
    (*vdo).dq =
        Some(v4l2_dq as
                 unsafe extern "C" fn(_: *mut zbar_video_t)
                     -> *mut zbar_image_t);
    (*vdo).set_control =
        Some(v4l2_s_control as
                 unsafe extern "C" fn(_: *mut zbar_video_t,
                                      _: *const libc::c_char,
                                      _: *mut libc::c_void) -> libc::c_int);
    (*vdo).get_control =
        Some(v4l2_g_control as
                 unsafe extern "C" fn(_: *mut zbar_video_t,
                                      _: *const libc::c_char,
                                      _: *mut libc::c_void) -> libc::c_int);
    (*vdo).free =
        Some(v4l2_free_controls as
                 unsafe extern "C" fn(_: *mut zbar_video_t) -> ());
    return 0 as libc::c_int;
}
