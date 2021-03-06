use ::libc;
extern {
    /* @} */
    /* ------------------------------------------------------------ */
    /* * @name Video interface
     * @anchor c-video
     * mid-level video source abstraction.
     * captures images from a video device
     */
    /* @{ */
    pub type zbar_video_s;
    /* @} */
    /* ------------------------------------------------------------ */
    /* * @name Image Scanner interface
     * @anchor c-imagescanner
     * mid-level image scanner interface.
     * reads barcodes from 2-D images
     */
    /* @{ */
    pub type zbar_image_scanner_s;
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
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    /*Extract symbol data from a list of QR codes and attach to the image.
    All text is converted to UTF-8.
    Any structured-append group that does not have all of its members is decoded
     as ZBAR_PARTIAL with ZBAR_PARTIAL components for the discontinuities.
    Note that isolated members of a structured-append group may be decoded with
     the wrong character set, since the correct setting cannot be propagated
     between codes.
    Return: The number of symbols which were successfully extracted from the
     codes; this will be at most the number of codes.*/
    #[no_mangle]
    fn qr_code_data_list_extract_text(
        _qrlist: *const qr_code_data_list,
        iscn: *mut zbar_image_scanner_t,
        img: *mut zbar_image_t,
    ) -> libc::c_int;
    /*Corrects the received code *_y, if possible.
    The original data is located in the top five bits.
    Returns the number of errors corrected, or a negative value if decoding
     failed due to too many bit errors, in which case *_y is left unchanged.*/
    #[no_mangle]
    fn bch15_5_correct(_y: *mut libc::c_uint) -> libc::c_int;
    /*Initialize discrete logarithm tables for GF(2**8) using a given primitive
    irreducible polynomial.*/
    #[no_mangle]
    fn rs_gf256_init(_gf: *mut rs_gf256, _ppoly: libc::c_uint);
    /*Corrects a codeword with _ndata<256 bytes, of which the last _npar are parity
     bytes.
    Known locations of errors can be passed in the _erasures array.
    Twice as many (up to _npar) errors with a known location can be corrected
     compared to errors with an unknown location.
    Returns the number of errors corrected if successful, or a negative number if
     the message could not be corrected because too many errors were detected.*/
    #[no_mangle]
    fn rs_correct(
        _gf: *const rs_gf256,
        _m0: libc::c_int,
        _data: *mut libc::c_uchar,
        _ndata: libc::c_int,
        _npar: libc::c_int,
        _erasures: *const libc::c_uchar,
        _nerasures: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn isaac_init(_ctx: *mut isaac_ctx, _seed: *const libc::c_void, _nseed: libc::c_int);
    #[no_mangle]
    fn isaac_next_uint(_ctx: *mut isaac_ctx, _n: libc::c_uint) -> libc::c_uint;
    /*Copyright (C) 2008-2009  Timothy B. Terriberry (tterribe@xiph.org)
    You can redistribute this library and/or modify it under the terms of the
     GNU Lesser General Public License as published by the Free Software
     Foundation; either version 2.1 of the License, or (at your option) any later
     version.*/
    /* Unlike copysign(), simply inverts the sign of _a if _b is negative. */
    /* Divides a signed integer by a positive value with exact rounding. */
    /* Swaps two integers _a and _b if _a>_b. */
    /* Computes the integer logarithm of a (positive, 32-bit) constant. */
    /*Multiplies 32-bit numbers _a and _b, adds (possibly 64-bit) number _r, and
    takes bits [_s,_s+31] of the result.*/
    /*Multiplies 32-bit numbers _a and _b, adds (possibly 64-bit) number _r, and
    gives all 64 bits of the result.*/
    #[no_mangle]
    fn qr_isqrt(_val: libc::c_uint) -> libc::c_uint;
    #[no_mangle]
    fn qr_ihypot(_x: libc::c_int, _y: libc::c_int) -> libc::c_uint;
    #[no_mangle]
    fn qr_ilog(_val: libc::c_uint) -> libc::c_int;
    /* Binarizes a grayscale image. */
    #[no_mangle]
    fn qr_binarize(
        _img: *const libc::c_uchar,
        _width: libc::c_int,
        _height: libc::c_int,
    ) -> *mut libc::c_uchar;
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
    /* just in case */
    /* reporting module */
    /* formatted and passed to application */
    /* errno for system errors */
    /* reporting function */
    /* description */
    /* single string argument */
    /* single integer argument */
    #[no_mangle]
    static mut _zbar_verbosity: libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut stderr: *mut FILE;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __compar_fn_t =
    Option<unsafe extern fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int>;
pub type uint32_t = __uint32_t;
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
/* *< unable to determine orientation */
/* *< upright, read left to right */
/* *< sideways, read top to bottom */
/* *< upside-down, read right to left */
/* *< sideways, read bottom to top */
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
/* * opaque video object. */
pub type zbar_video_t = zbar_video_s;
/* * cleanup handler callback function.
 * called to free sample data when an image is destroyed.
 */
pub type zbar_image_cleanup_handler_t = unsafe extern fn(_: *mut zbar_image_t) -> ();
/* * opaque image scanner object. */
pub type zbar_image_scanner_t = zbar_image_scanner_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qr_reader {
    pub gf: rs_gf256,
    pub isaac: isaac_ctx,
    pub finder_lines: [qr_finder_lines; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qr_finder_lines {
    pub lines: *mut qr_finder_line,
    pub nlines: libc::c_int,
    pub clines: libc::c_int,
}
/*Copyright (C) 2008-2009  Timothy B. Terriberry (tterribe@xiph.org)
You can redistribute this library and/or modify it under the terms of the
 GNU Lesser General Public License as published by the Free Software
 Foundation; either version 2.1 of the License, or (at your option) any later
 version.*/
/*The number of bits of subpel precision to store image coordinates in.
This helps when estimating positions in low-resolution images, which may have
 a module pitch only a pixel or two wide, making rounding errors matter a
 great deal.*/
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
/*ISAAC is the most advanced of a series of Pseudo-Random Number Generators
 designed by Robert J. Jenkins Jr. in 1996.
http://www.burtleburtle.net/bob/rand/isaac.html
To quote:
  No efficient method is known for deducing their internal states.
  ISAAC requires an amortized 18.75 instructions to produce a 32-bit value.
  There are no cycles in ISAAC shorter than 2**40 values.
  The expected cycle length is 2**8295 values.*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct isaac_ctx {
    pub n: libc::c_uint,
    pub r: [libc::c_uint; 256],
    pub m: [libc::c_uint; 256],
    pub a: libc::c_uint,
    pub b: libc::c_uint,
    pub c: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rs_gf256 {
    pub log: [libc::c_uchar; 256],
    pub exp: [libc::c_uchar; 511],
}
/*The center of a finder pattern obtained from the crossing of one or more
clusters of horizontal finder lines with one or more clusters of vertical
finder lines.*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qr_finder_center {
    pub pos: qr_point,
    pub edge_pts: *mut qr_finder_edge_pt,
    pub nedge_pts: libc::c_int,
}
/*A point on the edge of a finder pattern.
These are obtained from the endpoints of the lines crossing this particular
 pattern.*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qr_finder_edge_pt {
    pub pos: qr_point,
    pub edge: libc::c_int,
    pub extent: libc::c_int,
}
/* Low-level QR code data. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qr_code_data {
    pub entries: *mut qr_code_data_entry,
    pub nentries: libc::c_int,
    pub version: libc::c_uchar,
    pub ecc_level: libc::c_uchar,
    pub sa_index: libc::c_uchar,
    pub sa_size: libc::c_uchar,
    pub sa_parity: libc::c_uchar,
    pub self_parity: libc::c_uchar,
    pub bbox: [qr_point; 4],
}
/* A single unit of parsed QR code data. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qr_code_data_entry {
    pub mode: qr_mode,
    pub payload: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub data: C2RustUnnamed_1,
    pub eci: libc::c_uint,
    pub ai: libc::c_int,
    pub sa: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub sa_index: libc::c_uchar,
    pub sa_size: libc::c_uchar,
    pub sa_parity: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub buf: *mut libc::c_uchar,
    pub len: libc::c_int,
}
pub type qr_mode = libc::c_uint;
pub const QR_MODE_FNC1_2ND: qr_mode = 9;
pub const QR_MODE_KANJI: qr_mode = 8;
pub const QR_MODE_ECI: qr_mode = 7;
pub const QR_MODE_FNC1_1ST: qr_mode = 5;
pub const QR_MODE_BYTE: qr_mode = 4;
pub const QR_MODE_STRUCT: qr_mode = 3;
pub const QR_MODE_ALNUM: qr_mode = 2;
pub const QR_MODE_NUM: qr_mode = 1;
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
pub struct qr_code_data_list {
    pub qrdata: *mut qr_code_data,
    pub nqrdata: libc::c_int,
    pub cqrdata: libc::c_int,
}
/*All the information we've collected about a finder pattern in the current
configuration.*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qr_finder {
    pub size: [libc::c_int; 2],
    pub eversion: [libc::c_int; 2],
    pub edge_pts: [*mut qr_finder_edge_pt; 4],
    pub nedge_pts: [libc::c_int; 4],
    pub ninliers: [libc::c_int; 4],
    pub o: qr_point,
    pub c: *mut qr_finder_center,
}
/*Bit reading code blatantly stolen^W^Wadapted from libogg/libtheora (because
 I've already debugged it and I know it works).
Portions (C) Xiph.Org Foundation 1994-2008, BSD-style license.*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qr_pack_buf {
    pub buf: *const libc::c_uchar,
    pub endbyte: libc::c_int,
    pub endbit: libc::c_int,
    pub storage: libc::c_int,
}
/*The grid used to sample the image bits.
The grid is divided into separate cells bounded by finder patterns and/or
 alignment patterns, and a separate map back to the original image is
 constructed for each cell.
All of these structural elements, as well as the timing patterns, version
 info, and format info, are marked in fpmask so they can easily be skipped
 during decode.*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qr_sampling_grid {
    pub cells: [*mut qr_hom_cell; 6],
    pub fpmask: *mut libc::c_uint,
    pub cell_limits: [libc::c_int; 6],
    pub ncells: libc::c_int,
}
/*A homography from one region of the grid back to the image.
Unlike a qr_hom, this does not include an inverse transform and maps directly
 from the grid points, not a square with power-of-two sides.*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qr_hom_cell {
    pub fwd: [[libc::c_int; 3]; 3],
    pub x0: libc::c_int,
    pub y0: libc::c_int,
    pub u0: libc::c_int,
    pub v0: libc::c_int,
}
/*A full homography.
Like the affine homography, this maps from the image (at subpel resolution)
 to a square domain with power-of-two sides (of res bits) and back.*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qr_hom {
    pub fwd: [[libc::c_int; 2]; 3],
    pub inv: [[libc::c_int; 2]; 3],
    pub fwd22: libc::c_int,
    pub inv22: libc::c_int,
    pub x0: libc::c_int,
    pub y0: libc::c_int,
    pub res: libc::c_int,
}
/*An affine homography.
This maps from the image (at subpel resolution) to a square domain with
 power-of-two sides (of res bits) and back.*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qr_aff {
    pub fwd: [[libc::c_int; 2]; 2],
    pub inv: [[libc::c_int; 2]; 2],
    pub x0: libc::c_int,
    pub y0: libc::c_int,
    pub res: libc::c_int,
    pub ires: libc::c_int,
}
/*Copyright (C) 2008-2009  Timothy B. Terriberry (tterribe@xiph.org)
You can redistribute this library and/or modify it under the terms of the
 GNU Lesser General Public License as published by the Free Software
 Foundation; either version 2.1 of the License, or (at your option) any later
 version.*/
pub type qr_line = [libc::c_int; 3];
/* A cluster of lines crossing a finder pattern (all in the same direction). */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qr_finder_cluster {
    pub lines: *mut *mut qr_finder_line,
    pub nlines: libc::c_int,
}
/* Initializes a client reader handle. */
unsafe extern fn qr_reader_init(mut reader: *mut qr_reader) {
    /*time_t now;
    now=time(NULL);
    isaac_init(&_reader->isaac,&now,sizeof(now));*/
    isaac_init(&mut (*reader).isaac, 0 as *const libc::c_void, 0 as libc::c_int);
    rs_gf256_init(&mut (*reader).gf, 0x1d as libc::c_int as libc::c_uint);
}
/* Allocates a client reader handle. */
#[no_mangle]
pub unsafe extern fn _zbar_qr_create() -> *mut qr_reader {
    let mut reader: *mut qr_reader = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<qr_reader>() as libc::c_ulong,
    ) as *mut qr_reader;
    qr_reader_init(reader);
    return reader;
}
/* Frees a client reader handle. */
#[no_mangle]
pub unsafe extern fn _zbar_qr_destroy(mut reader: *mut qr_reader) {
    if _zbar_verbosity >= 1 as libc::c_int {
        fprintf(
            stderr,
            b"%s: max finder lines = %dx%d\n\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"_zbar_qr_destroy\x00"))
                .as_ptr(),
            (*reader).finder_lines[0 as libc::c_int as usize].clines,
            (*reader).finder_lines[1 as libc::c_int as usize].clines,
        );
    }
    if !(*reader).finder_lines[0 as libc::c_int as usize].lines.is_null() {
        free((*reader).finder_lines[0 as libc::c_int as usize].lines as *mut libc::c_void);
    }
    if !(*reader).finder_lines[1 as libc::c_int as usize].lines.is_null() {
        free((*reader).finder_lines[1 as libc::c_int as usize].lines as *mut libc::c_void);
    }
    free(reader as *mut libc::c_void);
}
/* reset finder state between scans */
#[no_mangle]
pub unsafe extern fn _zbar_qr_reset(mut reader: *mut qr_reader) {
    (*reader).finder_lines[0 as libc::c_int as usize].nlines = 0 as libc::c_int;
    (*reader).finder_lines[1 as libc::c_int as usize].nlines = 0 as libc::c_int;
}
unsafe extern fn qr_finder_vline_cmp(
    mut _a: *const libc::c_void,
    mut _b: *const libc::c_void,
) -> libc::c_int {
    let mut a: *const qr_finder_line = 0 as *const qr_finder_line;
    let mut b: *const qr_finder_line = 0 as *const qr_finder_line;
    a = _a as *const qr_finder_line;
    b = _b as *const qr_finder_line;
    return ((((*a).pos[0 as libc::c_int as usize] > (*b).pos[0 as libc::c_int as usize])
        as libc::c_int
        - ((*a).pos[0 as libc::c_int as usize] < (*b).pos[0 as libc::c_int as usize])
            as libc::c_int)
        << 1 as libc::c_int)
        + ((*a).pos[1 as libc::c_int as usize] > (*b).pos[1 as libc::c_int as usize])
            as libc::c_int
        - ((*a).pos[1 as libc::c_int as usize] < (*b).pos[1 as libc::c_int as usize])
            as libc::c_int;
}
/*Clusters adjacent lines into groups that are large enough to be crossing a
 finder pattern (relative to their length).
_clusters:  The buffer in which to store the clusters found.
_neighbors: The buffer used to store the lists of lines in each cluster.
_lines:     The list of lines to cluster.
            Horizontal lines must be sorted in ascending order by Y
             coordinate, with ties broken by X coordinate.
            Vertical lines must be sorted in ascending order by X coordinate,
             with ties broken by Y coordinate.
_nlines:    The number of lines in the set of lines to cluster.
_v:         0 for horizontal lines, or 1 for vertical lines.
Return: The number of clusters.*/
unsafe extern fn qr_finder_cluster_lines(
    mut _clusters: *mut qr_finder_cluster,
    mut _neighbors: *mut *mut qr_finder_line,
    mut _lines: *mut qr_finder_line,
    mut _nlines: libc::c_int,
    mut _v: libc::c_int,
) -> libc::c_int {
    let mut mark: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut neighbors: *mut *mut qr_finder_line = 0 as *mut *mut qr_finder_line;
    let mut nneighbors: libc::c_int = 0;
    let mut nclusters: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    /* TODO: Kalman filters! */
    mark = calloc(_nlines as libc::c_ulong, ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
        as *mut libc::c_uchar;
    neighbors = _neighbors;
    nclusters = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < _nlines - 1 as libc::c_int {
        if *mark.offset(i as isize) == 0 {
            let mut len: libc::c_int = 0;
            let mut j: libc::c_int = 0;
            nneighbors = 1 as libc::c_int;
            let ref mut fresh0 = *neighbors.offset(0 as libc::c_int as isize);
            *fresh0 = _lines.offset(i as isize);
            len = (*_lines.offset(i as isize)).len;
            j = i + 1 as libc::c_int;
            while j < _nlines {
                if *mark.offset(j as isize) == 0 {
                    let mut a: *const qr_finder_line = 0 as *const qr_finder_line;
                    let mut b: *const qr_finder_line = 0 as *const qr_finder_line;
                    let mut thresh: libc::c_int = 0;
                    a = *neighbors.offset((nneighbors - 1 as libc::c_int) as isize);
                    b = _lines.offset(j as isize);
                    /*The clustering threshold is proportional to the size of the lines,
                    since minor noise in large areas can interrupt patterns more easily
                    at high resolutions.*/
                    thresh = (*a).len + 7 as libc::c_int >> 2 as libc::c_int;
                    if abs((*a).pos[(1 as libc::c_int - _v) as usize]
                        - (*b).pos[(1 as libc::c_int - _v) as usize])
                        > thresh
                    {
                        break;
                    }
                    if !(abs((*a).pos[_v as usize] - (*b).pos[_v as usize]) > thresh) {
                        if !(abs((*a).pos[_v as usize] + (*a).len
                            - (*b).pos[_v as usize]
                            - (*b).len)
                            > thresh)
                        {
                            if !((*a).boffs > 0 as libc::c_int
                                && (*b).boffs > 0 as libc::c_int
                                && abs((*a).pos[_v as usize] - (*a).boffs - (*b).pos[_v as usize]
                                    + (*b).boffs)
                                    > thresh)
                            {
                                if !((*a).eoffs > 0 as libc::c_int
                                    && (*b).eoffs > 0 as libc::c_int
                                    && abs((*a).pos[_v as usize] + (*a).len + (*a).eoffs
                                        - (*b).pos[_v as usize]
                                        - (*b).len
                                        - (*b).eoffs)
                                        > thresh)
                                {
                                    let fresh1 = nneighbors;
                                    nneighbors = nneighbors + 1;
                                    let ref mut fresh2 = *neighbors.offset(fresh1 as isize);
                                    *fresh2 = _lines.offset(j as isize);
                                    len += (*b).len
                                }
                            }
                        }
                    }
                }
                j += 1
            }
            /*We require at least three lines to form a cluster, which eliminates a
             large number of false positives, saving considerable decoding time.
            This should still be sufficient for 1-pixel codes with no noise.*/
            if !(nneighbors < 3 as libc::c_int) {
                /*The expected number of lines crossing a finder pattern is equal to their
                 average length.
                We accept the cluster if size is at least 1/3 their average length (this
                 is a very small threshold, but was needed for some test images).*/
                len = ((len << 1 as libc::c_int) + nneighbors) / (nneighbors << 1 as libc::c_int);
                if nneighbors * ((5 as libc::c_int) << 2 as libc::c_int) >= len {
                    let ref mut fresh3 = (*_clusters.offset(nclusters as isize)).lines;
                    *fresh3 = neighbors;
                    (*_clusters.offset(nclusters as isize)).nlines = nneighbors;
                    j = 0 as libc::c_int;
                    while j < nneighbors {
                        *mark
                            .offset((*neighbors.offset(j as isize)).wrapping_offset_from(_lines)
                                as libc::c_long as isize) = 1 as libc::c_int as libc::c_uchar;
                        j += 1
                    }
                    neighbors = neighbors.offset(nneighbors as isize);
                    nclusters += 1
                }
            }
        }
        i += 1
    }
    free(mark as *mut libc::c_void);
    return nclusters;
}
/*Adds the coordinates of the edge points from the lines contained in the
 given list of clusters to the list of edge points for a finder center.
Only the edge point position is initialized.
The edge label and extent are set by qr_finder_edge_pts_aff_classify()
 or qr_finder_edge_pts_hom_classify().
_edge_pts:   The buffer in which to store the edge points.
_nedge_pts:  The current number of edge points in the buffer.
_neighbors:  The list of lines in the cluster.
_nneighbors: The number of lines in the list of lines in the cluster.
_v:          0 for horizontal lines and 1 for vertical lines.
Return: The new total number of edge points.*/
unsafe extern fn qr_finder_edge_pts_fill(
    mut _edge_pts: *mut qr_finder_edge_pt,
    mut _nedge_pts: libc::c_int,
    mut _neighbors: *mut *mut qr_finder_cluster,
    mut _nneighbors: libc::c_int,
    mut _v: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < _nneighbors {
        let mut c: *mut qr_finder_cluster = 0 as *mut qr_finder_cluster;
        let mut j: libc::c_int = 0;
        c = *_neighbors.offset(i as isize);
        j = 0 as libc::c_int;
        while j < (*c).nlines {
            let mut l: *mut qr_finder_line = 0 as *mut qr_finder_line;
            l = *(*c).lines.offset(j as isize);
            if (*l).boffs > 0 as libc::c_int {
                (*_edge_pts.offset(_nedge_pts as isize)).pos[0 as libc::c_int as usize] =
                    (*l).pos[0 as libc::c_int as usize];
                (*_edge_pts.offset(_nedge_pts as isize)).pos[1 as libc::c_int as usize] =
                    (*l).pos[1 as libc::c_int as usize];
                (*_edge_pts.offset(_nedge_pts as isize)).pos[_v as usize] -= (*l).boffs;
                _nedge_pts += 1
            }
            if (*l).eoffs > 0 as libc::c_int {
                (*_edge_pts.offset(_nedge_pts as isize)).pos[0 as libc::c_int as usize] =
                    (*l).pos[0 as libc::c_int as usize];
                (*_edge_pts.offset(_nedge_pts as isize)).pos[1 as libc::c_int as usize] =
                    (*l).pos[1 as libc::c_int as usize];
                (*_edge_pts.offset(_nedge_pts as isize)).pos[_v as usize] += (*l).len + (*l).eoffs;
                _nedge_pts += 1
            }
            j += 1
        }
        i += 1
    }
    return _nedge_pts;
}
unsafe extern fn qr_finder_center_cmp(
    mut _a: *const libc::c_void,
    mut _b: *const libc::c_void,
) -> libc::c_int {
    let mut a: *const qr_finder_center = 0 as *const qr_finder_center;
    let mut b: *const qr_finder_center = 0 as *const qr_finder_center;
    a = _a as *const qr_finder_center;
    b = _b as *const qr_finder_center;
    return ((((*b).nedge_pts > (*a).nedge_pts) as libc::c_int
        - ((*b).nedge_pts < (*a).nedge_pts) as libc::c_int)
        << 2 as libc::c_int)
        + ((((*a).pos[1 as libc::c_int as usize] > (*b).pos[1 as libc::c_int as usize])
            as libc::c_int
            - ((*a).pos[1 as libc::c_int as usize] < (*b).pos[1 as libc::c_int as usize])
                as libc::c_int)
            << 1 as libc::c_int)
        + ((*a).pos[0 as libc::c_int as usize] > (*b).pos[0 as libc::c_int as usize])
            as libc::c_int
        - ((*a).pos[0 as libc::c_int as usize] < (*b).pos[0 as libc::c_int as usize])
            as libc::c_int;
}
/*Determine if a horizontal line crosses a vertical line.
_hline: The horizontal line.
_vline: The vertical line.
Return: A non-zero value if the lines cross, or zero if they do not.*/
unsafe extern fn qr_finder_lines_are_crossing(
    mut _hline: *const qr_finder_line,
    mut _vline: *const qr_finder_line,
) -> libc::c_int {
    return ((*_hline).pos[0 as libc::c_int as usize] <= (*_vline).pos[0 as libc::c_int as usize]
        && (*_vline).pos[0 as libc::c_int as usize]
            < (*_hline).pos[0 as libc::c_int as usize] + (*_hline).len
        && (*_vline).pos[1 as libc::c_int as usize] <= (*_hline).pos[1 as libc::c_int as usize]
        && (*_hline).pos[1 as libc::c_int as usize]
            < (*_vline).pos[1 as libc::c_int as usize] + (*_vline).len) as libc::c_int;
}
/*Finds horizontal clusters that cross corresponding vertical clusters,
 presumably corresponding to a finder center.
_center:     The buffer in which to store putative finder centers.
_edge_pts:   The buffer to use for the edge point lists for each finder
              center.
_hclusters:  The clusters of horizontal lines crossing finder patterns.
_nhclusters: The number of horizontal line clusters.
_vclusters:  The clusters of vertical lines crossing finder patterns.
_nvclusters: The number of vertical line clusters.
Return: The number of putative finder centers.*/
unsafe extern fn qr_finder_find_crossings(
    mut _centers: *mut qr_finder_center,
    mut _edge_pts: *mut qr_finder_edge_pt,
    mut _hclusters: *mut qr_finder_cluster,
    mut _nhclusters: libc::c_int,
    mut _vclusters: *mut qr_finder_cluster,
    mut _nvclusters: libc::c_int,
) -> libc::c_int {
    let mut hneighbors: *mut *mut qr_finder_cluster = 0 as *mut *mut qr_finder_cluster;
    let mut vneighbors: *mut *mut qr_finder_cluster = 0 as *mut *mut qr_finder_cluster;
    let mut hmark: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut vmark: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ncenters: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    hneighbors = malloc(
        (_nhclusters as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut qr_finder_cluster>() as libc::c_ulong),
    ) as *mut *mut qr_finder_cluster;
    vneighbors = malloc(
        (_nvclusters as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut qr_finder_cluster>() as libc::c_ulong),
    ) as *mut *mut qr_finder_cluster;
    hmark = calloc(
        _nhclusters as libc::c_ulong,
        ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
    ) as *mut libc::c_uchar;
    vmark = calloc(
        _nvclusters as libc::c_ulong,
        ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
    ) as *mut libc::c_uchar;
    ncenters = 0 as libc::c_int;
    /*TODO: This may need some re-working.
    We should be finding groups of clusters such that _all_ horizontal lines in
     _all_ horizontal clusters in the group cross _all_ vertical lines in _all_
     vertical clusters in the group.
    This is equivalent to finding the maximum bipartite clique in the
     connectivity graph, which requires linear progamming to solve efficiently.
    In principle, that is easy to do, but a realistic implementation without
     floating point is a lot of work (and computationally expensive).
    Right now we are relying on a sufficient border around the finder patterns
     to prevent false positives.*/
    i = 0 as libc::c_int;
    while i < _nhclusters {
        if *hmark.offset(i as isize) == 0 {
            let mut a: *mut qr_finder_line = 0 as *mut qr_finder_line;
            let mut b: *mut qr_finder_line = 0 as *mut qr_finder_line;
            let mut nvneighbors: libc::c_int = 0;
            let mut nedge_pts: libc::c_int = 0;
            let mut y: libc::c_int = 0;
            a = *(*_hclusters.offset(i as isize))
                .lines
                .offset(((*_hclusters.offset(i as isize)).nlines >> 1 as libc::c_int) as isize);
            nvneighbors = 0 as libc::c_int;
            y = nvneighbors;
            j = 0 as libc::c_int;
            while j < _nvclusters {
                if *vmark.offset(j as isize) == 0 {
                    b = *(*_vclusters.offset(j as isize)).lines.offset(
                        ((*_vclusters.offset(j as isize)).nlines >> 1 as libc::c_int) as isize,
                    );
                    if qr_finder_lines_are_crossing(a, b) != 0 {
                        *vmark.offset(j as isize) = 1 as libc::c_int as libc::c_uchar;
                        y += ((*b).pos[1 as libc::c_int as usize] << 1 as libc::c_int) + (*b).len;
                        if (*b).boffs > 0 as libc::c_int && (*b).eoffs > 0 as libc::c_int {
                            y += (*b).eoffs - (*b).boffs
                        }
                        let fresh4 = nvneighbors;
                        nvneighbors = nvneighbors + 1;
                        let ref mut fresh5 = *vneighbors.offset(fresh4 as isize);
                        *fresh5 = _vclusters.offset(j as isize)
                    }
                }
                j += 1
            }
            if nvneighbors > 0 as libc::c_int {
                let mut c: *mut qr_finder_center = 0 as *mut qr_finder_center;
                let mut nhneighbors: libc::c_int = 0;
                let mut x: libc::c_int = 0;
                x = ((*a).pos[0 as libc::c_int as usize] << 1 as libc::c_int) + (*a).len;
                if (*a).boffs > 0 as libc::c_int && (*a).eoffs > 0 as libc::c_int {
                    x += (*a).eoffs - (*a).boffs
                }
                let ref mut fresh6 = *hneighbors.offset(0 as libc::c_int as isize);
                *fresh6 = _hclusters.offset(i as isize);
                nhneighbors = 1 as libc::c_int;
                j = nvneighbors >> 1 as libc::c_int;
                b = *(**vneighbors.offset(j as isize)).lines.offset(
                    ((**vneighbors.offset(j as isize)).nlines >> 1 as libc::c_int) as isize,
                );
                j = i + 1 as libc::c_int;
                while j < _nhclusters {
                    if *hmark.offset(j as isize) == 0 {
                        a = *(*_hclusters.offset(j as isize)).lines.offset(
                            ((*_hclusters.offset(j as isize)).nlines >> 1 as libc::c_int) as isize,
                        );
                        if qr_finder_lines_are_crossing(a, b) != 0 {
                            *hmark.offset(j as isize) = 1 as libc::c_int as libc::c_uchar;
                            x += ((*a).pos[0 as libc::c_int as usize] << 1 as libc::c_int)
                                + (*a).len;
                            if (*a).boffs > 0 as libc::c_int && (*a).eoffs > 0 as libc::c_int {
                                x += (*a).eoffs - (*a).boffs
                            }
                            let fresh7 = nhneighbors;
                            nhneighbors = nhneighbors + 1;
                            let ref mut fresh8 = *hneighbors.offset(fresh7 as isize);
                            *fresh8 = _hclusters.offset(j as isize)
                        }
                    }
                    j += 1
                }
                let fresh9 = ncenters;
                ncenters = ncenters + 1;
                c = _centers.offset(fresh9 as isize);
                (*c).pos[0 as libc::c_int as usize] =
                    (x + nhneighbors) / (nhneighbors << 1 as libc::c_int);
                (*c).pos[1 as libc::c_int as usize] =
                    (y + nvneighbors) / (nvneighbors << 1 as libc::c_int);
                (*c).edge_pts = _edge_pts;
                nedge_pts = qr_finder_edge_pts_fill(
                    _edge_pts,
                    0 as libc::c_int,
                    hneighbors,
                    nhneighbors,
                    0 as libc::c_int,
                );
                nedge_pts = qr_finder_edge_pts_fill(
                    _edge_pts,
                    nedge_pts,
                    vneighbors,
                    nvneighbors,
                    1 as libc::c_int,
                );
                (*c).nedge_pts = nedge_pts;
                _edge_pts = _edge_pts.offset(nedge_pts as isize)
            }
        }
        i += 1
    }
    free(vmark as *mut libc::c_void);
    free(hmark as *mut libc::c_void);
    free(vneighbors as *mut libc::c_void);
    free(hneighbors as *mut libc::c_void);
    /* Sort the centers by decreasing numbers of edge points. */
    qsort(
        _centers as *mut libc::c_void,
        ncenters as size_t,
        ::std::mem::size_of::<qr_finder_center>() as libc::c_ulong,
        Some(
            qr_finder_center_cmp
                as unsafe extern fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int,
        ),
    );
    return ncenters;
}
/*Locates a set of putative finder centers in the image.
First we search for horizontal and vertical lines that have
 (dark:light:dark:light:dark) runs with size ratios of roughly (1:1:3:1:1).
Then we cluster them into groups such that each subsequent pair of endpoints
 is close to the line before it in the cluster.
This will locate many line clusters that don't cross a finder pattern, but
 qr_finder_find_crossings() will filter most of them out.
Where horizontal and vertical clusters cross, a prospective finder center is
 returned.
_centers:  Returns a pointer to a freshly-allocated list of finder centers.
           This must be freed by the caller.
_edge_pts: Returns a pointer to a freshly-allocated list of edge points
            around those centers.
           This must be freed by the caller.
_img:      The binary image to search.
_width:    The width of the image.
_height:   The height of the image.
Return: The number of putative finder centers located.*/
unsafe extern fn qr_finder_centers_locate(
    mut _centers: *mut *mut qr_finder_center,
    mut _edge_pts: *mut *mut qr_finder_edge_pt,
    mut reader: *mut qr_reader,
    mut _width: libc::c_int,
    mut _height: libc::c_int,
) -> libc::c_int {
    let mut hlines: *mut qr_finder_line = (*reader).finder_lines[0 as libc::c_int as usize].lines;
    let mut nhlines: libc::c_int = (*reader).finder_lines[0 as libc::c_int as usize].nlines;
    let mut vlines: *mut qr_finder_line = (*reader).finder_lines[1 as libc::c_int as usize].lines;
    let mut nvlines: libc::c_int = (*reader).finder_lines[1 as libc::c_int as usize].nlines;
    let mut hneighbors: *mut *mut qr_finder_line = 0 as *mut *mut qr_finder_line;
    let mut hclusters: *mut qr_finder_cluster = 0 as *mut qr_finder_cluster;
    let mut nhclusters: libc::c_int = 0;
    let mut vneighbors: *mut *mut qr_finder_line = 0 as *mut *mut qr_finder_line;
    let mut vclusters: *mut qr_finder_cluster = 0 as *mut qr_finder_cluster;
    let mut nvclusters: libc::c_int = 0;
    let mut ncenters: libc::c_int = 0;
    /* Cluster the detected lines. */
    hneighbors = malloc(
        (nhlines as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut qr_finder_line>() as libc::c_ulong),
    ) as *mut *mut qr_finder_line;
    /* We require more than one line per cluster, so there are at most nhlines/2. */
    hclusters = malloc(
        ((nhlines >> 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<qr_finder_cluster>() as libc::c_ulong),
    ) as *mut qr_finder_cluster;
    nhclusters = qr_finder_cluster_lines(hclusters, hneighbors, hlines, nhlines, 0 as libc::c_int);
    /*We need vertical lines to be sorted by X coordinate, with ties broken by Y
     coordinate, for clustering purposes.
    We scan the image in the opposite order for cache efficiency, so sort the
     lines we found here.*/
    qsort(
        vlines as *mut libc::c_void,
        nvlines as size_t,
        ::std::mem::size_of::<qr_finder_line>() as libc::c_ulong,
        Some(
            qr_finder_vline_cmp
                as unsafe extern fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int,
        ),
    );
    vneighbors = malloc(
        (nvlines as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut qr_finder_line>() as libc::c_ulong),
    ) as *mut *mut qr_finder_line;
    /* We require more than one line per cluster, so there are at most nvlines/2. */
    vclusters = malloc(
        ((nvlines >> 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<qr_finder_cluster>() as libc::c_ulong),
    ) as *mut qr_finder_cluster;
    nvclusters = qr_finder_cluster_lines(vclusters, vneighbors, vlines, nvlines, 1 as libc::c_int);
    /* Find line crossings among the clusters. */
    if nhclusters >= 3 as libc::c_int && nvclusters >= 3 as libc::c_int {
        let mut edge_pts: *mut qr_finder_edge_pt = 0 as *mut qr_finder_edge_pt;
        let mut centers: *mut qr_finder_center = 0 as *mut qr_finder_center;
        let mut nedge_pts: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        nedge_pts = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < nhclusters {
            nedge_pts += (*hclusters.offset(i as isize)).nlines;
            i += 1
        }
        i = 0 as libc::c_int;
        while i < nvclusters {
            nedge_pts += (*vclusters.offset(i as isize)).nlines;
            i += 1
        }
        nedge_pts <<= 1 as libc::c_int;
        edge_pts = malloc(
            (nedge_pts as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<qr_finder_edge_pt>() as libc::c_ulong),
        ) as *mut qr_finder_edge_pt;
        centers = malloc(
            ((nhclusters + (nvclusters - nhclusters & -((nvclusters < nhclusters) as libc::c_int)))
                as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<qr_finder_center>() as libc::c_ulong),
        ) as *mut qr_finder_center;
        ncenters = qr_finder_find_crossings(
            centers, edge_pts, hclusters, nhclusters, vclusters, nvclusters,
        );
        *_centers = centers;
        *_edge_pts = edge_pts
    } else {
        ncenters = 0 as libc::c_int
    }
    free(vclusters as *mut libc::c_void);
    free(vneighbors as *mut libc::c_void);
    free(hclusters as *mut libc::c_void);
    free(hneighbors as *mut libc::c_void);
    return ncenters;
}
unsafe extern fn qr_point_translate(
    mut _point: *mut libc::c_int,
    mut _dx: libc::c_int,
    mut _dy: libc::c_int,
) {
    *_point.offset(0 as libc::c_int as isize) += _dx;
    *_point.offset(1 as libc::c_int as isize) += _dy;
}
unsafe extern fn qr_point_distance2(
    mut _p1: *const libc::c_int,
    mut _p2: *const libc::c_int,
) -> libc::c_uint {
    return ((*_p1.offset(0 as libc::c_int as isize) - *_p2.offset(0 as libc::c_int as isize))
        * (*_p1.offset(0 as libc::c_int as isize) - *_p2.offset(0 as libc::c_int as isize))
        + (*_p1.offset(1 as libc::c_int as isize) - *_p2.offset(1 as libc::c_int as isize))
            * (*_p1.offset(1 as libc::c_int as isize) - *_p2.offset(1 as libc::c_int as isize)))
        as libc::c_uint;
}
/*Returns the cross product of the three points, which is positive if they are
in CCW order (in a right-handed coordinate system), and 0 if they're
colinear.*/
unsafe extern fn qr_point_ccw(
    mut _p0: *const libc::c_int,
    mut _p1: *const libc::c_int,
    mut _p2: *const libc::c_int,
) -> libc::c_int {
    return (*_p1.offset(0 as libc::c_int as isize) - *_p0.offset(0 as libc::c_int as isize))
        * (*_p2.offset(1 as libc::c_int as isize) - *_p0.offset(1 as libc::c_int as isize))
        - (*_p1.offset(1 as libc::c_int as isize) - *_p0.offset(1 as libc::c_int as isize))
            * (*_p2.offset(0 as libc::c_int as isize) - *_p0.offset(0 as libc::c_int as isize));
}
/*Evaluates a line equation at a point.
_line: The line to evaluate.
_x:    The X coordinate of the point.
_y:    The y coordinate of the point.
Return: The value of the line equation _line[0]*_x+_line[1]*_y+_line[2].*/
unsafe extern fn qr_line_eval(
    mut _line: *mut libc::c_int,
    mut _x: libc::c_int,
    mut _y: libc::c_int,
) -> libc::c_int {
    return *_line.offset(0 as libc::c_int as isize) * _x
        + *_line.offset(1 as libc::c_int as isize) * _y
        + *_line.offset(2 as libc::c_int as isize);
}
/*Computes a line passing through the given point using the specified second
 order statistics.
Given a line defined by the equation
  A*x+B*y+C = 0 ,
 the least squares fit to n points (x_i,y_i) must satisfy the two equations
  A^2 + (Syy - Sxx)/Sxy*A*B - B^2 = 0 ,
  C = -(xbar*A+ybar*B) ,
 where
  xbar = sum(x_i)/n ,
  ybar = sum(y_i)/n ,
  Sxx = sum((x_i-xbar)**2) ,
  Sxy = sum((x_i-xbar)*(y_i-ybar)) ,
  Syy = sum((y_i-ybar)**2) .
The quadratic can be solved for the ratio (A/B) or (B/A):
  A/B = (Syy + sqrt((Sxx-Syy)**2 + 4*Sxy**2) - Sxx)/(-2*Sxy) ,
  B/A = (Sxx + sqrt((Sxx-Syy)**2 + 4*Sxy**2) - Syy)/(-2*Sxy) .
We pick the one that leads to the larger ratio to avoid destructive
 cancellation (and e.g., 0/0 for horizontal or vertical lines).
The above solutions correspond to the actual minimum.
The other solution of the quadratic corresponds to a saddle point of the
 least squares objective function.
_l:   Returns the fitted line values A, B, and C.
_x0:  The X coordinate of the point the line is supposed to pass through.
_y0:  The Y coordinate of the point the line is supposed to pass through.
_sxx: The sum Sxx.
_sxy: The sum Sxy.
_syy: The sum Syy.
_res: The maximum number of bits occupied by the product of any two of
       _l[0] or _l[1].
      Smaller numbers give less angular resolution, but allow more overhead
       room for computations.*/
unsafe extern fn qr_line_fit(
    mut _l: *mut libc::c_int,
    mut _x0: libc::c_int,
    mut _y0: libc::c_int,
    mut _sxx: libc::c_int,
    mut _sxy: libc::c_int,
    mut _syy: libc::c_int,
    mut _res: libc::c_int,
) {
    let mut dshift: libc::c_int = 0;
    let mut dround: libc::c_int = 0;
    let mut u: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    u = abs(_sxx - _syy);
    v = -_sxy << 1 as libc::c_int;
    w = qr_ihypot(u, v) as libc::c_int;
    /*Computations in later stages can easily overflow with moderate sizes, so we
     compute a shift factor to scale things down into a managable range.
    We ensure that the product of any two of _l[0] and _l[1] fits within _res
     bits, which allows computation of line intersections without overflow.*/
    dshift = 0 as libc::c_int
        - (0 as libc::c_int
            - (qr_ilog(u as libc::c_uint)
                - (qr_ilog(u as libc::c_uint) - qr_ilog(abs(v) as libc::c_uint)
                    & -((qr_ilog(abs(v) as libc::c_uint) > qr_ilog(u as libc::c_uint))
                        as libc::c_int))
                + 1 as libc::c_int
                - (_res + 1 as libc::c_int >> 1 as libc::c_int))
            & -((qr_ilog(u as libc::c_uint)
                - (qr_ilog(u as libc::c_uint) - qr_ilog(abs(v) as libc::c_uint)
                    & -((qr_ilog(abs(v) as libc::c_uint) > qr_ilog(u as libc::c_uint))
                        as libc::c_int))
                + 1 as libc::c_int
                - (_res + 1 as libc::c_int >> 1 as libc::c_int)
                > 0 as libc::c_int) as libc::c_int));
    dround = (1 as libc::c_int) << dshift >> 1 as libc::c_int;
    if _sxx > _syy {
        *_l.offset(0 as libc::c_int as isize) = v + dround >> dshift;
        *_l.offset(1 as libc::c_int as isize) = u + w + dround >> dshift
    } else {
        *_l.offset(0 as libc::c_int as isize) = u + w + dround >> dshift;
        *_l.offset(1 as libc::c_int as isize) = v + dround >> dshift
    }
    *_l.offset(2 as libc::c_int as isize) = -(_x0 * *_l.offset(0 as libc::c_int as isize)
        + _y0 * *_l.offset(1 as libc::c_int as isize));
}
/*Perform a least-squares line fit to a list of points.
At least two points are required.*/
unsafe extern fn qr_line_fit_points(
    mut _l: *mut libc::c_int,
    mut _p: *mut qr_point,
    mut _np: libc::c_int,
    mut _res: libc::c_int,
) {
    let mut sx: libc::c_int = 0;
    let mut sy: libc::c_int = 0;
    let mut xmin: libc::c_int = 0;
    let mut xmax: libc::c_int = 0;
    let mut ymin: libc::c_int = 0;
    let mut ymax: libc::c_int = 0;
    let mut xbar: libc::c_int = 0;
    let mut ybar: libc::c_int = 0;
    let mut dx: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    let mut sxx: libc::c_int = 0;
    let mut sxy: libc::c_int = 0;
    let mut syy: libc::c_int = 0;
    let mut sshift: libc::c_int = 0;
    let mut sround: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    sy = 0 as libc::c_int;
    sx = sy;
    xmax = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    ymax = xmax;
    xmin = 2147483647 as libc::c_int;
    ymin = xmin;
    i = 0 as libc::c_int;
    while i < _np {
        sx += (*_p.offset(i as isize))[0 as libc::c_int as usize];
        xmin = xmin
            + ((*_p.offset(i as isize))[0 as libc::c_int as usize] - xmin
                & -(((*_p.offset(i as isize))[0 as libc::c_int as usize] < xmin) as libc::c_int));
        xmax = xmax
            - (xmax - (*_p.offset(i as isize))[0 as libc::c_int as usize]
                & -(((*_p.offset(i as isize))[0 as libc::c_int as usize] > xmax) as libc::c_int));
        sy += (*_p.offset(i as isize))[1 as libc::c_int as usize];
        ymin = ymin
            + ((*_p.offset(i as isize))[1 as libc::c_int as usize] - ymin
                & -(((*_p.offset(i as isize))[1 as libc::c_int as usize] < ymin) as libc::c_int));
        ymax = ymax
            - (ymax - (*_p.offset(i as isize))[1 as libc::c_int as usize]
                & -(((*_p.offset(i as isize))[1 as libc::c_int as usize] > ymax) as libc::c_int));
        i += 1
    }
    xbar = (sx + (_np >> 1 as libc::c_int)) / _np;
    ybar = (sy + (_np >> 1 as libc::c_int)) / _np;
    sshift = 0 as libc::c_int
        - (0 as libc::c_int
            - (qr_ilog(
                (_np * (xmax
                    - xbar
                    - (xmax - xbar - (xbar - xmin)
                        & -((xbar - xmin > xmax - xbar) as libc::c_int))
                    - (xmax
                        - xbar
                        - (xmax - xbar - (xbar - xmin)
                            & -((xbar - xmin > xmax - xbar) as libc::c_int))
                        - (ymax
                            - ybar
                            - (ymax - ybar - (ybar - ymin)
                                & -((ybar - ymin > ymax - ybar) as libc::c_int)))
                        & -((ymax
                            - ybar
                            - (ymax - ybar - (ybar - ymin)
                                & -((ybar - ymin > ymax - ybar) as libc::c_int))
                            > xmax
                                - xbar
                                - (xmax - xbar - (xbar - xmin)
                                    & -((xbar - xmin > xmax - xbar) as libc::c_int)))
                            as libc::c_int)))) as libc::c_uint,
            ) - (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                * 8 as libc::c_int
                - 1 as libc::c_int
                >> 1 as libc::c_int))
            & -((qr_ilog(
                (_np * (xmax
                    - xbar
                    - (xmax - xbar - (xbar - xmin)
                        & -((xbar - xmin > xmax - xbar) as libc::c_int))
                    - (xmax
                        - xbar
                        - (xmax - xbar - (xbar - xmin)
                            & -((xbar - xmin > xmax - xbar) as libc::c_int))
                        - (ymax
                            - ybar
                            - (ymax - ybar - (ybar - ymin)
                                & -((ybar - ymin > ymax - ybar) as libc::c_int)))
                        & -((ymax
                            - ybar
                            - (ymax - ybar - (ybar - ymin)
                                & -((ybar - ymin > ymax - ybar) as libc::c_int))
                            > xmax
                                - xbar
                                - (xmax - xbar - (xbar - xmin)
                                    & -((xbar - xmin > xmax - xbar) as libc::c_int)))
                            as libc::c_int)))) as libc::c_uint,
            ) - (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                * 8 as libc::c_int
                - 1 as libc::c_int
                >> 1 as libc::c_int)
                > 0 as libc::c_int) as libc::c_int));
    sround = (1 as libc::c_int) << sshift >> 1 as libc::c_int;
    syy = 0 as libc::c_int;
    sxy = syy;
    sxx = sxy;
    i = 0 as libc::c_int;
    while i < _np {
        dx = (*_p.offset(i as isize))[0 as libc::c_int as usize] - xbar + sround >> sshift;
        dy = (*_p.offset(i as isize))[1 as libc::c_int as usize] - ybar + sround >> sshift;
        sxx += dx * dx;
        sxy += dx * dy;
        syy += dy * dy;
        i += 1
    }
    qr_line_fit(_l, xbar, ybar, sxx, sxy, syy, _res);
}
unsafe extern fn qr_line_orient(
    mut _l: *mut libc::c_int,
    mut _x: libc::c_int,
    mut _y: libc::c_int,
) {
    if qr_line_eval(_l, _x, _y) < 0 as libc::c_int {
        *_l.offset(0 as libc::c_int as isize) = -*_l.offset(0 as libc::c_int as isize);
        *_l.offset(1 as libc::c_int as isize) = -*_l.offset(1 as libc::c_int as isize);
        *_l.offset(2 as libc::c_int as isize) = -*_l.offset(2 as libc::c_int as isize)
    };
}
unsafe extern fn qr_line_isect(
    mut _p: *mut libc::c_int,
    mut _l0: *const libc::c_int,
    mut _l1: *const libc::c_int,
) -> libc::c_int {
    let mut d: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    d = *_l0.offset(0 as libc::c_int as isize) * *_l1.offset(1 as libc::c_int as isize)
        - *_l0.offset(1 as libc::c_int as isize) * *_l1.offset(0 as libc::c_int as isize);
    if d == 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    x = *_l0.offset(1 as libc::c_int as isize) * *_l1.offset(2 as libc::c_int as isize)
        - *_l1.offset(1 as libc::c_int as isize) * *_l0.offset(2 as libc::c_int as isize);
    y = *_l1.offset(0 as libc::c_int as isize) * *_l0.offset(2 as libc::c_int as isize)
        - *_l0.offset(0 as libc::c_int as isize) * *_l1.offset(2 as libc::c_int as isize);
    if d < 0 as libc::c_int {
        x = -x;
        y = -y;
        d = -d
    }
    *_p.offset(0 as libc::c_int as isize) = (x
        + ((d >> 1 as libc::c_int) + -((x < 0 as libc::c_int) as libc::c_int)
            ^ -((x < 0 as libc::c_int) as libc::c_int)))
        / d;
    *_p.offset(1 as libc::c_int as isize) = (y
        + ((d >> 1 as libc::c_int) + -((y < 0 as libc::c_int) as libc::c_int)
            ^ -((y < 0 as libc::c_int) as libc::c_int)))
        / d;
    return 0 as libc::c_int;
}
unsafe extern fn qr_aff_init(
    mut _aff: *mut qr_aff,
    mut _p0: *const libc::c_int,
    mut _p1: *const libc::c_int,
    mut _p2: *const libc::c_int,
    mut _res: libc::c_int,
) {
    let mut det: libc::c_int = 0;
    let mut ires: libc::c_int = 0;
    let mut dx1: libc::c_int = 0;
    let mut dy1: libc::c_int = 0;
    let mut dx2: libc::c_int = 0;
    let mut dy2: libc::c_int = 0;
    /* det is ensured to be positive by our caller. */
    dx1 = *_p1.offset(0 as libc::c_int as isize) - *_p0.offset(0 as libc::c_int as isize);
    dx2 = *_p2.offset(0 as libc::c_int as isize) - *_p0.offset(0 as libc::c_int as isize);
    dy1 = *_p1.offset(1 as libc::c_int as isize) - *_p0.offset(1 as libc::c_int as isize);
    dy2 = *_p2.offset(1 as libc::c_int as isize) - *_p0.offset(1 as libc::c_int as isize);
    det = dx1 * dy2 - dy1 * dx2;
    ires = (qr_ilog(abs(det) as libc::c_uint) >> 1 as libc::c_int)
        - 2 as libc::c_int
        - ((qr_ilog(abs(det) as libc::c_uint) >> 1 as libc::c_int)
            - 2 as libc::c_int
            - 0 as libc::c_int
            & -((0 as libc::c_int
                > (qr_ilog(abs(det) as libc::c_uint) >> 1 as libc::c_int) - 2 as libc::c_int)
                as libc::c_int));
    (*_aff).fwd[0 as libc::c_int as usize][0 as libc::c_int as usize] = dx1;
    (*_aff).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize] = dx2;
    (*_aff).fwd[1 as libc::c_int as usize][0 as libc::c_int as usize] = dy1;
    (*_aff).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize] = dy2;
    (*_aff).inv[0 as libc::c_int as usize][0 as libc::c_int as usize] = ((dy2 << _res)
        + ((det >> ires >> 1 as libc::c_int) + -((dy2 << _res < 0 as libc::c_int) as libc::c_int)
            ^ -((dy2 << _res < 0 as libc::c_int) as libc::c_int)))
        / (det >> ires);
    (*_aff).inv[0 as libc::c_int as usize][1 as libc::c_int as usize] = ((-dx2 << _res)
        + ((det >> ires >> 1 as libc::c_int)
            + -((-dx2 << _res < 0 as libc::c_int) as libc::c_int)
            ^ -((-dx2 << _res < 0 as libc::c_int) as libc::c_int)))
        / (det >> ires);
    (*_aff).inv[1 as libc::c_int as usize][0 as libc::c_int as usize] = ((-dy1 << _res)
        + ((det >> ires >> 1 as libc::c_int)
            + -((-dy1 << _res < 0 as libc::c_int) as libc::c_int)
            ^ -((-dy1 << _res < 0 as libc::c_int) as libc::c_int)))
        / (det >> ires);
    (*_aff).inv[1 as libc::c_int as usize][1 as libc::c_int as usize] = ((dx1 << _res)
        + ((det >> ires >> 1 as libc::c_int) + -((dx1 << _res < 0 as libc::c_int) as libc::c_int)
            ^ -((dx1 << _res < 0 as libc::c_int) as libc::c_int)))
        / (det >> ires);
    (*_aff).x0 = *_p0.offset(0 as libc::c_int as isize);
    (*_aff).y0 = *_p0.offset(1 as libc::c_int as isize);
    (*_aff).res = _res;
    (*_aff).ires = ires;
}
/* Map from the image (at subpel resolution) into the square domain. */
unsafe extern fn qr_aff_unproject(
    mut _q: *mut libc::c_int,
    mut _aff: *const qr_aff,
    mut _x: libc::c_int,
    mut _y: libc::c_int,
) {
    *_q.offset(0 as libc::c_int as isize) =
        (*_aff).inv[0 as libc::c_int as usize][0 as libc::c_int as usize] * (_x - (*_aff).x0)
            + (*_aff).inv[0 as libc::c_int as usize][1 as libc::c_int as usize] * (_y - (*_aff).y0)
            + ((1 as libc::c_int) << (*_aff).ires >> 1 as libc::c_int)
            >> (*_aff).ires;
    *_q.offset(1 as libc::c_int as isize) =
        (*_aff).inv[1 as libc::c_int as usize][0 as libc::c_int as usize] * (_x - (*_aff).x0)
            + (*_aff).inv[1 as libc::c_int as usize][1 as libc::c_int as usize] * (_y - (*_aff).y0)
            + ((1 as libc::c_int) << (*_aff).ires >> 1 as libc::c_int)
            >> (*_aff).ires;
}
/* Map from the square domain into the image (at subpel resolution). */
unsafe extern fn qr_aff_project(
    mut _p: *mut libc::c_int,
    mut _aff: *const qr_aff,
    mut _u: libc::c_int,
    mut _v: libc::c_int,
) {
    *_p.offset(0 as libc::c_int as isize) =
        ((*_aff).fwd[0 as libc::c_int as usize][0 as libc::c_int as usize] * _u
            + (*_aff).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize] * _v
            + ((1 as libc::c_int) << (*_aff).res - 1 as libc::c_int)
            >> (*_aff).res)
            + (*_aff).x0;
    *_p.offset(1 as libc::c_int as isize) =
        ((*_aff).fwd[1 as libc::c_int as usize][0 as libc::c_int as usize] * _u
            + (*_aff).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize] * _v
            + ((1 as libc::c_int) << (*_aff).res - 1 as libc::c_int)
            >> (*_aff).res)
            + (*_aff).y0;
}
unsafe extern fn qr_hom_init(
    mut _hom: *mut qr_hom,
    mut _x0: libc::c_int,
    mut _y0: libc::c_int,
    mut _x1: libc::c_int,
    mut _y1: libc::c_int,
    mut _x2: libc::c_int,
    mut _y2: libc::c_int,
    mut _x3: libc::c_int,
    mut _y3: libc::c_int,
    mut _res: libc::c_int,
) {
    let mut dx10: libc::c_int = 0;
    let mut dx20: libc::c_int = 0;
    let mut dx30: libc::c_int = 0;
    let mut dx31: libc::c_int = 0;
    let mut dx32: libc::c_int = 0;
    let mut dy10: libc::c_int = 0;
    let mut dy20: libc::c_int = 0;
    let mut dy30: libc::c_int = 0;
    let mut dy31: libc::c_int = 0;
    let mut dy32: libc::c_int = 0;
    let mut a20: libc::c_int = 0;
    let mut a21: libc::c_int = 0;
    let mut a22: libc::c_int = 0;
    let mut b0: libc::c_int = 0;
    let mut b1: libc::c_int = 0;
    let mut b2: libc::c_int = 0;
    let mut s1: libc::c_int = 0;
    let mut s2: libc::c_int = 0;
    let mut r1: libc::c_int = 0;
    let mut r2: libc::c_int = 0;
    dx10 = _x1 - _x0;
    dx20 = _x2 - _x0;
    dx30 = _x3 - _x0;
    dx31 = _x3 - _x1;
    dx32 = _x3 - _x2;
    dy10 = _y1 - _y0;
    dy20 = _y2 - _y0;
    dy30 = _y3 - _y0;
    dy31 = _y3 - _y1;
    dy32 = _y3 - _y2;
    a20 = dx32 * dy10 - dx10 * dy32;
    a21 = dx20 * dy31 - dx31 * dy20;
    a22 = dx32 * dy31 - dx31 * dy32;
    /* Figure out if we need to downscale anything. */
    b0 = qr_ilog(
        (abs(dx10) - (abs(dx10) - abs(dy10) & -((abs(dy10) > abs(dx10)) as libc::c_int)))
            as libc::c_uint,
    ) + qr_ilog(abs(a20 + a22) as libc::c_uint);
    b1 = qr_ilog(
        (abs(dx20) - (abs(dx20) - abs(dy20) & -((abs(dy20) > abs(dx20)) as libc::c_int)))
            as libc::c_uint,
    ) + qr_ilog(abs(a21 + a22) as libc::c_uint);
    b2 = qr_ilog(
        (abs(a20)
            - (abs(a20) - abs(a21) & -((abs(a21) > abs(a20)) as libc::c_int))
            - (abs(a20)
                - (abs(a20) - abs(a21) & -((abs(a21) > abs(a20)) as libc::c_int))
                - abs(a22)
                & -((abs(a22)
                    > abs(a20) - (abs(a20) - abs(a21) & -((abs(a21) > abs(a20)) as libc::c_int)))
                    as libc::c_int))) as libc::c_uint,
    );
    s1 = 0 as libc::c_int
        - (0 as libc::c_int
            - (_res
                + (b0
                    - (b0 - b1 & -((b1 > b0) as libc::c_int))
                    - (b0 - (b0 - b1 & -((b1 > b0) as libc::c_int)) - b2
                        & -((b2 > b0 - (b0 - b1 & -((b1 > b0) as libc::c_int))) as libc::c_int)))
                - (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                    * 8 as libc::c_int
                    - 2 as libc::c_int))
            & -((_res
                + (b0
                    - (b0 - b1 & -((b1 > b0) as libc::c_int))
                    - (b0 - (b0 - b1 & -((b1 > b0) as libc::c_int)) - b2
                        & -((b2 > b0 - (b0 - b1 & -((b1 > b0) as libc::c_int))) as libc::c_int)))
                - (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                    * 8 as libc::c_int
                    - 2 as libc::c_int)
                > 0 as libc::c_int) as libc::c_int));
    r1 = (1 as libc::c_int) << s1 >> 1 as libc::c_int;
    /*Compute the final coefficients of the forward transform.
    The 32x32->64 bit multiplies are really needed for accuracy with large
     versions.*/
    (*_hom).fwd[0 as libc::c_int as usize][0 as libc::c_int as usize] =
        (dx10 as libc::c_longlong * (a20 + a22) as libc::c_longlong + r1 as libc::c_longlong >> s1)
            as libc::c_int;
    (*_hom).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize] =
        (dx20 as libc::c_longlong * (a21 + a22) as libc::c_longlong + r1 as libc::c_longlong >> s1)
            as libc::c_int;
    (*_hom).x0 = _x0;
    (*_hom).fwd[1 as libc::c_int as usize][0 as libc::c_int as usize] =
        (dy10 as libc::c_longlong * (a20 + a22) as libc::c_longlong + r1 as libc::c_longlong >> s1)
            as libc::c_int;
    (*_hom).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize] =
        (dy20 as libc::c_longlong * (a21 + a22) as libc::c_longlong + r1 as libc::c_longlong >> s1)
            as libc::c_int;
    (*_hom).y0 = _y0;
    (*_hom).fwd[2 as libc::c_int as usize][0 as libc::c_int as usize] = a20 + r1 >> s1;
    (*_hom).fwd[2 as libc::c_int as usize][1 as libc::c_int as usize] = a21 + r1 >> s1;
    (*_hom).fwd22 = if s1 > _res {
        (a22 + (r1 >> _res)) >> s1 - _res
    } else {
        (a22) << _res - s1
    };
    /* Now compute the inverse transform. */
    b0 = qr_ilog(
        (abs(dx10)
            - (abs(dx10) - abs(dx20) & -((abs(dx20) > abs(dx10)) as libc::c_int))
            - (abs(dx10)
                - (abs(dx10) - abs(dx20) & -((abs(dx20) > abs(dx10)) as libc::c_int))
                - abs(dx30)
                & -((abs(dx30)
                    > abs(dx10)
                        - (abs(dx10) - abs(dx20) & -((abs(dx20) > abs(dx10)) as libc::c_int)))
                    as libc::c_int))) as libc::c_uint,
    ) + qr_ilog(
        (abs((*_hom).fwd[0 as libc::c_int as usize][0 as libc::c_int as usize])
            - (abs((*_hom).fwd[0 as libc::c_int as usize][0 as libc::c_int as usize])
                - abs((*_hom).fwd[1 as libc::c_int as usize][0 as libc::c_int as usize])
                & -((abs((*_hom).fwd[1 as libc::c_int as usize][0 as libc::c_int as usize])
                    > abs((*_hom).fwd[0 as libc::c_int as usize][0 as libc::c_int as usize]))
                    as libc::c_int))) as libc::c_uint,
    );
    b1 = qr_ilog(
        (abs(dy10)
            - (abs(dy10) - abs(dy20) & -((abs(dy20) > abs(dy10)) as libc::c_int))
            - (abs(dy10)
                - (abs(dy10) - abs(dy20) & -((abs(dy20) > abs(dy10)) as libc::c_int))
                - abs(dy30)
                & -((abs(dy30)
                    > abs(dy10)
                        - (abs(dy10) - abs(dy20) & -((abs(dy20) > abs(dy10)) as libc::c_int)))
                    as libc::c_int))) as libc::c_uint,
    ) + qr_ilog(
        (abs((*_hom).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize])
            - (abs((*_hom).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize])
                - abs((*_hom).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize])
                & -((abs((*_hom).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize])
                    > abs((*_hom).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize]))
                    as libc::c_int))) as libc::c_uint,
    );
    b2 = qr_ilog(abs(a22) as libc::c_uint) - s1;
    s2 = 0 as libc::c_int
        - (0 as libc::c_int
            - (b0 - (b0 - b1 & -((b1 > b0) as libc::c_int)) + b2
                - (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                    * 8 as libc::c_int
                    - 3 as libc::c_int))
            & -((b0 - (b0 - b1 & -((b1 > b0) as libc::c_int)) + b2
                - (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                    * 8 as libc::c_int
                    - 3 as libc::c_int)
                > 0 as libc::c_int) as libc::c_int));
    r2 = (1 as libc::c_int) << s2 >> 1 as libc::c_int;
    s1 += s2;
    r1 <<= s2;
    /*The 32x32->64 bit multiplies are really needed for accuracy with large
    versions.*/
    (*_hom).inv[0 as libc::c_int as usize][0 as libc::c_int as usize] =
        ((*_hom).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize] as libc::c_longlong
            * a22 as libc::c_longlong
            + r1 as libc::c_longlong
            >> s1) as libc::c_int;
    (*_hom).inv[0 as libc::c_int as usize][1 as libc::c_int as usize] =
        (-(*_hom).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize] as libc::c_longlong
            * a22 as libc::c_longlong
            + r1 as libc::c_longlong
            >> s1) as libc::c_int;
    (*_hom).inv[1 as libc::c_int as usize][0 as libc::c_int as usize] =
        (-(*_hom).fwd[1 as libc::c_int as usize][0 as libc::c_int as usize] as libc::c_longlong
            * a22 as libc::c_longlong
            + r1 as libc::c_longlong
            >> s1) as libc::c_int;
    (*_hom).inv[1 as libc::c_int as usize][1 as libc::c_int as usize] =
        ((*_hom).fwd[0 as libc::c_int as usize][0 as libc::c_int as usize] as libc::c_longlong
            * a22 as libc::c_longlong
            + r1 as libc::c_longlong
            >> s1) as libc::c_int;
    (*_hom).inv[2 as libc::c_int as usize][0 as libc::c_int as usize] =
        ((*_hom).fwd[1 as libc::c_int as usize][0 as libc::c_int as usize] as libc::c_longlong
            * (*_hom).fwd[2 as libc::c_int as usize][1 as libc::c_int as usize] as libc::c_longlong
            + -((*_hom).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize]
                as libc::c_longlong
                * (*_hom).fwd[2 as libc::c_int as usize][0 as libc::c_int as usize]
                    as libc::c_longlong
                + r2 as libc::c_longlong)
            >> s2) as libc::c_int;
    (*_hom).inv[2 as libc::c_int as usize][1 as libc::c_int as usize] =
        ((*_hom).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize] as libc::c_longlong
            * (*_hom).fwd[2 as libc::c_int as usize][0 as libc::c_int as usize] as libc::c_longlong
            + -((*_hom).fwd[0 as libc::c_int as usize][0 as libc::c_int as usize]
                as libc::c_longlong
                * (*_hom).fwd[2 as libc::c_int as usize][1 as libc::c_int as usize]
                    as libc::c_longlong
                + r2 as libc::c_longlong)
            >> s2) as libc::c_int;
    (*_hom).inv22 = ((*_hom).fwd[0 as libc::c_int as usize][0 as libc::c_int as usize]
        as libc::c_longlong
        * (*_hom).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize] as libc::c_longlong
        + -((*_hom).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize] as libc::c_longlong
            * (*_hom).fwd[1 as libc::c_int as usize][0 as libc::c_int as usize]
                as libc::c_longlong
            + r2 as libc::c_longlong)
        >> s2) as libc::c_int;
    (*_hom).res = _res;
}
/*Map from the image (at subpel resolution) into the square domain.
Returns a negative value if the point went to infinity.*/
unsafe extern fn qr_hom_unproject(
    mut _q: *mut libc::c_int,
    mut _hom: *const qr_hom,
    mut _x: libc::c_int,
    mut _y: libc::c_int,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    _x -= (*_hom).x0;
    _y -= (*_hom).y0;
    x = (*_hom).inv[0 as libc::c_int as usize][0 as libc::c_int as usize] * _x
        + (*_hom).inv[0 as libc::c_int as usize][1 as libc::c_int as usize] * _y;
    y = (*_hom).inv[1 as libc::c_int as usize][0 as libc::c_int as usize] * _x
        + (*_hom).inv[1 as libc::c_int as usize][1 as libc::c_int as usize] * _y;
    w = (*_hom).inv[2 as libc::c_int as usize][0 as libc::c_int as usize] * _x
        + (*_hom).inv[2 as libc::c_int as usize][1 as libc::c_int as usize] * _y
        + (*_hom).inv22
        + ((1 as libc::c_int) << (*_hom).res - 1 as libc::c_int)
        >> (*_hom).res;
    if w == 0 as libc::c_int {
        *_q.offset(0 as libc::c_int as isize) = if x < 0 as libc::c_int {
            (-(2147483647 as libc::c_int)) - 1 as libc::c_int
        } else {
            2147483647 as libc::c_int
        };
        *_q.offset(1 as libc::c_int as isize) = if y < 0 as libc::c_int {
            (-(2147483647 as libc::c_int)) - 1 as libc::c_int
        } else {
            2147483647 as libc::c_int
        };
        return -(1 as libc::c_int);
    } else {
        if w < 0 as libc::c_int {
            x = -x;
            y = -y;
            w = -w
        }
        *_q.offset(0 as libc::c_int as isize) = (x
            + ((w >> 1 as libc::c_int) + -((x < 0 as libc::c_int) as libc::c_int)
                ^ -((x < 0 as libc::c_int) as libc::c_int)))
            / w;
        *_q.offset(1 as libc::c_int as isize) = (y
            + ((w >> 1 as libc::c_int) + -((y < 0 as libc::c_int) as libc::c_int)
                ^ -((y < 0 as libc::c_int) as libc::c_int)))
            / w
    }
    return 0 as libc::c_int;
}
/*Finish a partial projection, converting from homogeneous coordinates to the
 normal 2-D representation.
In loops, we can avoid many multiplies by computing the homogeneous _x, _y,
 and _w incrementally, but we cannot avoid the divisions, done here.*/
unsafe extern fn qr_hom_fproject(
    mut _p: *mut libc::c_int,
    mut _hom: *const qr_hom,
    mut _x: libc::c_int,
    mut _y: libc::c_int,
    mut _w: libc::c_int,
) {
    if _w == 0 as libc::c_int {
        *_p.offset(0 as libc::c_int as isize) = if _x < 0 as libc::c_int {
            (-(2147483647 as libc::c_int)) - 1 as libc::c_int
        } else {
            2147483647 as libc::c_int
        };
        *_p.offset(1 as libc::c_int as isize) = if _y < 0 as libc::c_int {
            (-(2147483647 as libc::c_int)) - 1 as libc::c_int
        } else {
            2147483647 as libc::c_int
        }
    } else {
        if _w < 0 as libc::c_int {
            _x = -_x;
            _y = -_y;
            _w = -_w
        }
        *_p.offset(0 as libc::c_int as isize) = (_x
            + ((_w >> 1 as libc::c_int) + -((_x < 0 as libc::c_int) as libc::c_int)
                ^ -((_x < 0 as libc::c_int) as libc::c_int)))
            / _w
            + (*_hom).x0;
        *_p.offset(1 as libc::c_int as isize) = (_y
            + ((_w >> 1 as libc::c_int) + -((_y < 0 as libc::c_int) as libc::c_int)
                ^ -((_y < 0 as libc::c_int) as libc::c_int)))
            / _w
            + (*_hom).y0
    };
}
unsafe extern fn qr_cmp_edge_pt(
    mut _a: *const libc::c_void,
    mut _b: *const libc::c_void,
) -> libc::c_int {
    let mut a: *const qr_finder_edge_pt = 0 as *const qr_finder_edge_pt;
    let mut b: *const qr_finder_edge_pt = 0 as *const qr_finder_edge_pt;
    a = _a as *const qr_finder_edge_pt;
    b = _b as *const qr_finder_edge_pt;
    return ((((*a).edge > (*b).edge) as libc::c_int - ((*a).edge < (*b).edge) as libc::c_int)
        << 1 as libc::c_int)
        + ((*a).extent > (*b).extent) as libc::c_int
        - ((*a).extent < (*b).extent) as libc::c_int;
}
/*Computes the index of the edge each edge point belongs to, and its (signed)
 distance along the corresponding axis from the center of the finder pattern
 (in the square domain).
The resulting list of edge points is sorted by edge index, with ties broken
 by extent.*/
unsafe extern fn qr_finder_edge_pts_aff_classify(mut _f: *mut qr_finder, mut _aff: *const qr_aff) {
    let mut c: *mut qr_finder_center = 0 as *mut qr_finder_center;
    let mut i: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    c = (*_f).c;
    e = 0 as libc::c_int;
    while e < 4 as libc::c_int {
        (*_f).nedge_pts[e as usize] = 0 as libc::c_int;
        e += 1
    }
    i = 0 as libc::c_int;
    while i < (*c).nedge_pts {
        let mut q: qr_point = [0; 2];
        let mut d: libc::c_int = 0;
        qr_aff_unproject(
            q.as_mut_ptr(),
            _aff,
            (*(*c).edge_pts.offset(i as isize)).pos[0 as libc::c_int as usize],
            (*(*c).edge_pts.offset(i as isize)).pos[1 as libc::c_int as usize],
        );
        qr_point_translate(
            q.as_mut_ptr(),
            -(*_f).o[0 as libc::c_int as usize],
            -(*_f).o[1 as libc::c_int as usize],
        );
        d = (abs(q[1 as libc::c_int as usize]) > abs(q[0 as libc::c_int as usize])) as libc::c_int;
        e = d << 1 as libc::c_int | (q[d as usize] >= 0 as libc::c_int) as libc::c_int;
        (*_f).nedge_pts[e as usize] += 1;
        (*(*c).edge_pts.offset(i as isize)).edge = e;
        (*(*c).edge_pts.offset(i as isize)).extent = q[d as usize];
        i += 1
    }
    qsort(
        (*c).edge_pts as *mut libc::c_void,
        (*c).nedge_pts as size_t,
        ::std::mem::size_of::<qr_finder_edge_pt>() as libc::c_ulong,
        Some(
            qr_cmp_edge_pt
                as unsafe extern fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int,
        ),
    );
    (*_f).edge_pts[0 as libc::c_int as usize] = (*c).edge_pts;
    e = 1 as libc::c_int;
    while e < 4 as libc::c_int {
        (*_f).edge_pts[e as usize] = (*_f).edge_pts[(e - 1 as libc::c_int) as usize]
            .offset((*_f).nedge_pts[(e - 1 as libc::c_int) as usize] as isize);
        e += 1
    }
}
/*Computes the index of the edge each edge point belongs to, and its (signed)
 distance along the corresponding axis from the center of the finder pattern
 (in the square domain).
The resulting list of edge points is sorted by edge index, with ties broken
 by extent.*/
unsafe extern fn qr_finder_edge_pts_hom_classify(mut _f: *mut qr_finder, mut _hom: *const qr_hom) {
    let mut c: *mut qr_finder_center = 0 as *mut qr_finder_center;
    let mut i: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    c = (*_f).c;
    e = 0 as libc::c_int;
    while e < 4 as libc::c_int {
        (*_f).nedge_pts[e as usize] = 0 as libc::c_int;
        e += 1
    }
    i = 0 as libc::c_int;
    while i < (*c).nedge_pts {
        let mut q: qr_point = [0; 2];
        let mut d: libc::c_int = 0;
        if qr_hom_unproject(
            q.as_mut_ptr(),
            _hom,
            (*(*c).edge_pts.offset(i as isize)).pos[0 as libc::c_int as usize],
            (*(*c).edge_pts.offset(i as isize)).pos[1 as libc::c_int as usize],
        ) >= 0 as libc::c_int
        {
            qr_point_translate(
                q.as_mut_ptr(),
                -(*_f).o[0 as libc::c_int as usize],
                -(*_f).o[1 as libc::c_int as usize],
            );
            d = (abs(q[1 as libc::c_int as usize]) > abs(q[0 as libc::c_int as usize]))
                as libc::c_int;
            e = d << 1 as libc::c_int | (q[d as usize] >= 0 as libc::c_int) as libc::c_int;
            (*_f).nedge_pts[e as usize] += 1;
            (*(*c).edge_pts.offset(i as isize)).edge = e;
            (*(*c).edge_pts.offset(i as isize)).extent = q[d as usize]
        } else {
            (*(*c).edge_pts.offset(i as isize)).edge = 4 as libc::c_int;
            (*(*c).edge_pts.offset(i as isize)).extent = q[0 as libc::c_int as usize]
        }
        i += 1
    }
    qsort(
        (*c).edge_pts as *mut libc::c_void,
        (*c).nedge_pts as size_t,
        ::std::mem::size_of::<qr_finder_edge_pt>() as libc::c_ulong,
        Some(
            qr_cmp_edge_pt
                as unsafe extern fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int,
        ),
    );
    (*_f).edge_pts[0 as libc::c_int as usize] = (*c).edge_pts;
    e = 1 as libc::c_int;
    while e < 4 as libc::c_int {
        (*_f).edge_pts[e as usize] = (*_f).edge_pts[(e - 1 as libc::c_int) as usize]
            .offset((*_f).nedge_pts[(e - 1 as libc::c_int) as usize] as isize);
        e += 1
    }
}
/*Estimates the size of a module after classifying the edge points.
_width:  The distance between UL and UR in the square domain.
_height: The distance between UL and DL in the square domain.*/
unsafe extern fn qr_finder_estimate_module_size_and_version(
    mut _f: *mut qr_finder,
    mut _width: libc::c_int,
    mut _height: libc::c_int,
) -> libc::c_int {
    let mut offs: qr_point = [0; 2];
    let mut sums: [libc::c_int; 4] = [0; 4];
    let mut nsums: [libc::c_int; 4] = [0; 4];
    let mut usize: libc::c_int = 0;
    let mut nusize: libc::c_int = 0;
    let mut vsize: libc::c_int = 0;
    let mut nvsize: libc::c_int = 0;
    let mut uversion: libc::c_int = 0;
    let mut vversion: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    offs[1 as libc::c_int as usize] = 0 as libc::c_int;
    offs[0 as libc::c_int as usize] = offs[1 as libc::c_int as usize];
    e = 0 as libc::c_int;
    while e < 4 as libc::c_int {
        if (*_f).nedge_pts[e as usize] > 0 as libc::c_int {
            let mut edge_pts: *mut qr_finder_edge_pt = 0 as *mut qr_finder_edge_pt;
            let mut sum: libc::c_int = 0;
            let mut mean: libc::c_int = 0;
            let mut n: libc::c_int = 0;
            let mut i: libc::c_int = 0;
            /* Average the samples for this edge, dropping the top and bottom 25%. */
            edge_pts = (*_f).edge_pts[e as usize];
            n = (*_f).nedge_pts[e as usize];
            sum = 0 as libc::c_int;
            i = n >> 2 as libc::c_int;
            while i < n - (n >> 2 as libc::c_int) {
                sum += (*edge_pts.offset(i as isize)).extent;
                i += 1
            }
            n = n - ((n >> 2 as libc::c_int) << 1 as libc::c_int);
            mean = (sum
                + ((n >> 1 as libc::c_int) + -((sum < 0 as libc::c_int) as libc::c_int)
                    ^ -((sum < 0 as libc::c_int) as libc::c_int)))
                / n;
            offs[(e >> 1 as libc::c_int) as usize] += mean;
            sums[e as usize] = sum;
            nsums[e as usize] = n
        } else {
            sums[e as usize] = 0 as libc::c_int;
            nsums[e as usize] = sums[e as usize]
        }
        e += 1
    }
    /*If we have samples on both sides of an axis, refine our idea of where the
    unprojected finder center is located.*/
    if (*_f).nedge_pts[0 as libc::c_int as usize] > 0 as libc::c_int
        && (*_f).nedge_pts[1 as libc::c_int as usize] > 0 as libc::c_int
    {
        (*_f).o[0 as libc::c_int as usize] -= offs[0 as libc::c_int as usize] >> 1 as libc::c_int;
        sums[0 as libc::c_int as usize] -=
            offs[0 as libc::c_int as usize] * nsums[0 as libc::c_int as usize] >> 1 as libc::c_int;
        sums[1 as libc::c_int as usize] -=
            offs[0 as libc::c_int as usize] * nsums[1 as libc::c_int as usize] >> 1 as libc::c_int
    }
    if (*_f).nedge_pts[2 as libc::c_int as usize] > 0 as libc::c_int
        && (*_f).nedge_pts[3 as libc::c_int as usize] > 0 as libc::c_int
    {
        (*_f).o[1 as libc::c_int as usize] -= offs[1 as libc::c_int as usize] >> 1 as libc::c_int;
        sums[2 as libc::c_int as usize] -=
            offs[1 as libc::c_int as usize] * nsums[2 as libc::c_int as usize] >> 1 as libc::c_int;
        sums[3 as libc::c_int as usize] -=
            offs[1 as libc::c_int as usize] * nsums[3 as libc::c_int as usize] >> 1 as libc::c_int
    }
    /*We must have _some_ samples along each axis... if we don't, our transform
    must be pretty severely distorting the original square (e.g., with
    coordinates so large as to cause overflow).*/
    nusize = nsums[0 as libc::c_int as usize] + nsums[1 as libc::c_int as usize];
    if nusize <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    /* The module size is 1/3 the average edge extent. */
    nusize *= 3 as libc::c_int;
    usize = sums[1 as libc::c_int as usize] - sums[0 as libc::c_int as usize];
    usize = ((usize << 1 as libc::c_int) + nusize) / (nusize << 1 as libc::c_int);
    if usize <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    /*Now estimate the version directly from the module size and the distance
     between the finder patterns.
    This is done independently using the extents along each axis.
    If either falls significantly outside the valid range (1 to 40), reject the
     configuration.*/
    uversion = (_width - 8 as libc::c_int * usize) / (usize << 2 as libc::c_int);
    if uversion < 1 as libc::c_int || uversion > 40 as libc::c_int + 3 as libc::c_int {
        return -(1 as libc::c_int);
    }
    /* Now do the same for the other axis. */
    nvsize = nsums[2 as libc::c_int as usize] + nsums[3 as libc::c_int as usize];
    if nvsize <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    nvsize *= 3 as libc::c_int;
    vsize = sums[3 as libc::c_int as usize] - sums[2 as libc::c_int as usize];
    vsize = ((vsize << 1 as libc::c_int) + nvsize) / (nvsize << 1 as libc::c_int);
    if vsize <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    vversion = (_height - 8 as libc::c_int * vsize) / (vsize << 2 as libc::c_int);
    if vversion < 1 as libc::c_int || vversion > 40 as libc::c_int + 3 as libc::c_int {
        return -(1 as libc::c_int);
    }
    /*If the estimated version using extents along one axis is significantly
     different than the estimated version along the other axis, then the axes
     have significantly different scalings (relative to the grid).
    This can happen, e.g., when we have multiple adjacent QR codes, and we've
     picked two finder patterns from one and the third finder pattern from
     another, e.g.:
      X---DL UL---X
      |....   |....
      X....  UR....
    Such a configuration might even pass any other geometric checks if we
     didn't reject it here.*/
    if abs(uversion - vversion) > 3 as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*_f).size[0 as libc::c_int as usize] = usize;
    (*_f).size[1 as libc::c_int as usize] = vsize;
    /*We intentionally do not compute an average version from the sizes along
     both axes.
    In the presence of projective distortion, one of them will be much more
     accurate than the other.*/
    (*_f).eversion[0 as libc::c_int as usize] = uversion;
    (*_f).eversion[1 as libc::c_int as usize] = vversion;
    return 0 as libc::c_int;
}
/* Eliminate outliers from the classified edge points with RANSAC. */
unsafe extern fn qr_finder_ransac(
    mut _f: *mut qr_finder,
    mut _hom: *const qr_aff,
    mut _isaac: *mut isaac_ctx,
    mut _e: libc::c_int,
) {
    let mut edge_pts: *mut qr_finder_edge_pt = 0 as *mut qr_finder_edge_pt;
    let mut best_ninliers: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    edge_pts = (*_f).edge_pts[_e as usize];
    n = (*_f).nedge_pts[_e as usize];
    best_ninliers = 0 as libc::c_int;
    if n > 1 as libc::c_int {
        let mut max_iters: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        /*17 iterations is enough to guarantee an outlier-free sample with more
        than 99% probability given as many as 50% outliers.*/
        max_iters = 17 as libc::c_int;
        i = 0 as libc::c_int;
        while i < max_iters {
            let mut q0: qr_point = [0; 2];
            let mut q1: qr_point = [0; 2];
            let mut ninliers: libc::c_int = 0;
            let mut thresh: libc::c_int = 0;
            let mut p0i: libc::c_int = 0;
            let mut p1i: libc::c_int = 0;
            let mut p0: *mut libc::c_int = 0 as *mut libc::c_int;
            let mut p1: *mut libc::c_int = 0 as *mut libc::c_int;
            let mut j_0: libc::c_int = 0;
            /* Pick two random points on this edge. */
            p0i = isaac_next_uint(_isaac, n as libc::c_uint) as libc::c_int;
            p1i = isaac_next_uint(_isaac, (n - 1 as libc::c_int) as libc::c_uint) as libc::c_int;
            if p1i >= p0i {
                p1i += 1
            }
            p0 = (*edge_pts.offset(p0i as isize)).pos.as_mut_ptr();
            p1 = (*edge_pts.offset(p1i as isize)).pos.as_mut_ptr();
            /*If the corresponding line is not within 45 degrees of the proper
             orientation in the square domain, reject it outright.
            This can happen, e.g., when highly skewed orientations cause points to
             be misclassified into the wrong edge.
            The irony is that using such points might produce a line which _does_
             pass the corresponding validity checks.*/
            qr_aff_unproject(
                q0.as_mut_ptr(),
                _hom,
                *p0.offset(0 as libc::c_int as isize),
                *p0.offset(1 as libc::c_int as isize),
            );
            qr_aff_unproject(
                q1.as_mut_ptr(),
                _hom,
                *p1.offset(0 as libc::c_int as isize),
                *p1.offset(1 as libc::c_int as isize),
            );
            qr_point_translate(
                q0.as_mut_ptr(),
                -(*_f).o[0 as libc::c_int as usize],
                -(*_f).o[1 as libc::c_int as usize],
            );
            qr_point_translate(
                q1.as_mut_ptr(),
                -(*_f).o[0 as libc::c_int as usize],
                -(*_f).o[1 as libc::c_int as usize],
            );
            if !(abs(q0[(_e >> 1 as libc::c_int) as usize] - q1[(_e >> 1 as libc::c_int) as usize])
                > abs(q0[(1 as libc::c_int - (_e >> 1 as libc::c_int)) as usize]
                    - q1[(1 as libc::c_int - (_e >> 1 as libc::c_int)) as usize]))
            {
                /*Identify the other edge points which are inliers.
                The squared distance should be distributed as a \Chi^2 distribution
                 with one degree of freedom, which means for a 95% confidence the
                 point should lie within a factor 3.8414588 ~= 4 times the expected
                 variance of the point locations.
                We grossly approximate the standard deviation as 1 pixel in one
                 direction, and 0.5 pixels in the other (because we average two
                 coordinates).*/
                thresh = qr_isqrt(
                    qr_point_distance2(p0 as *const libc::c_int, p1 as *const libc::c_int)
                        << 2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int,
                ) as libc::c_int;
                ninliers = 0 as libc::c_int;
                j_0 = 0 as libc::c_int;
                while j_0 < n {
                    if abs(qr_point_ccw(
                        p0 as *const libc::c_int,
                        p1 as *const libc::c_int,
                        (*edge_pts.offset(j_0 as isize)).pos.as_mut_ptr() as *const libc::c_int,
                    )) <= thresh
                    {
                        (*edge_pts.offset(j_0 as isize)).extent |= 1 as libc::c_int;
                        ninliers += 1
                    } else {
                        (*edge_pts.offset(j_0 as isize)).extent &= !(1 as libc::c_int)
                    }
                    j_0 += 1
                }
                if ninliers > best_ninliers {
                    j_0 = 0 as libc::c_int;
                    while j_0 < n {
                        (*edge_pts.offset(j_0 as isize)).extent <<= 1 as libc::c_int;
                        j_0 += 1
                    }
                    best_ninliers = ninliers;
                    /*The actual number of iterations required is
                      log(1-\alpha)/log(1-r*r),
                     where \alpha is the required probability of taking a sample with
                      no outliers (e.g., 0.99) and r is the estimated ratio of inliers
                      (e.g. ninliers/n).
                    This is just a rough (but conservative) approximation, but it
                     should be good enough to stop the iteration early when we find
                     a good set of inliers.*/
                    if ninliers > n >> 1 as libc::c_int {
                        max_iters = (67 as libc::c_int * n
                            - 63 as libc::c_int * ninliers
                            - 1 as libc::c_int)
                            / (n << 1 as libc::c_int)
                    }
                }
            }
            i += 1
        }
        /* Now collect all the inliers at the beginning of the list. */
        j = 0 as libc::c_int;
        i = j;
        while j < best_ninliers {
            if (*edge_pts.offset(i as isize)).extent & 2 as libc::c_int != 0 {
                if j < i {
                    let mut tmp: qr_finder_edge_pt = qr_finder_edge_pt {
                        pos: [0; 2],
                        edge: 0,
                        extent: 0,
                    };
                    tmp = *edge_pts.offset(i as isize);
                    *edge_pts.offset(j as isize) = *edge_pts.offset(i as isize);
                    *edge_pts.offset(i as isize) = tmp
                }
                j += 1
            }
            i += 1
        }
    }
    (*_f).ninliers[_e as usize] = best_ninliers;
}
/*Perform a least-squares line fit to an edge of a finder pattern using the
inliers found by RANSAC.*/
unsafe extern fn qr_line_fit_finder_edge(
    mut _l: *mut libc::c_int,
    mut _f: *const qr_finder,
    mut _e: libc::c_int,
    mut _res: libc::c_int,
) -> libc::c_int {
    let mut edge_pts: *mut qr_finder_edge_pt = 0 as *mut qr_finder_edge_pt;
    let mut pts: *mut qr_point = 0 as *mut qr_point;
    let mut npts: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    npts = (*_f).ninliers[_e as usize];
    if npts < 2 as libc::c_int {
        return -(1 as libc::c_int);
    }
    /*We could write a custom version of qr_line_fit_points that accesses
    edge_pts directly, but this saves on code size and doesn't measurably slow
    things down.*/
    pts = malloc(
        (npts as libc::c_ulong).wrapping_mul(::std::mem::size_of::<qr_point>() as libc::c_ulong),
    ) as *mut qr_point;
    edge_pts = (*_f).edge_pts[_e as usize];
    i = 0 as libc::c_int;
    while i < npts {
        (*pts.offset(i as isize))[0 as libc::c_int as usize] =
            (*edge_pts.offset(i as isize)).pos[0 as libc::c_int as usize];
        (*pts.offset(i as isize))[1 as libc::c_int as usize] =
            (*edge_pts.offset(i as isize)).pos[1 as libc::c_int as usize];
        i += 1
    }
    qr_line_fit_points(_l, pts, npts, _res);
    /*Make sure the center of the finder pattern lies in the positive halfspace
    of the line.*/
    qr_line_orient(
        _l,
        (*(*_f).c).pos[0 as libc::c_int as usize],
        (*(*_f).c).pos[1 as libc::c_int as usize],
    );
    free(pts as *mut libc::c_void);
    return 0 as libc::c_int;
}
/*Perform a least-squares line fit to a pair of common finder edges using the
 inliers found by RANSAC.
Unlike a normal edge fit, we guarantee that this one succeeds by creating at
 least one point on each edge using the estimated module size if it has no
 inliers.*/
unsafe extern fn qr_line_fit_finder_pair(
    mut _l: *mut libc::c_int,
    mut _aff: *const qr_aff,
    mut _f0: *const qr_finder,
    mut _f1: *const qr_finder,
    mut _e: libc::c_int,
) {
    let mut pts: *mut qr_point = 0 as *mut qr_point;
    let mut npts: libc::c_int = 0;
    let mut edge_pts: *mut qr_finder_edge_pt = 0 as *mut qr_finder_edge_pt;
    let mut q: qr_point = [0; 2];
    let mut n0: libc::c_int = 0;
    let mut n1: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    n0 = (*_f0).ninliers[_e as usize];
    n1 = (*_f1).ninliers[_e as usize];
    /*We could write a custom version of qr_line_fit_points that accesses
    edge_pts directly, but this saves on code size and doesn't measurably slow
    things down.*/
    npts = n0 - (n0 - 1 as libc::c_int & -((1 as libc::c_int > n0) as libc::c_int))
        + (n1 - (n1 - 1 as libc::c_int & -((1 as libc::c_int > n1) as libc::c_int)));
    pts = malloc(
        (npts as libc::c_ulong).wrapping_mul(::std::mem::size_of::<qr_point>() as libc::c_ulong),
    ) as *mut qr_point;
    if n0 > 0 as libc::c_int {
        edge_pts = (*_f0).edge_pts[_e as usize];
        i = 0 as libc::c_int;
        while i < n0 {
            (*pts.offset(i as isize))[0 as libc::c_int as usize] =
                (*edge_pts.offset(i as isize)).pos[0 as libc::c_int as usize];
            (*pts.offset(i as isize))[1 as libc::c_int as usize] =
                (*edge_pts.offset(i as isize)).pos[1 as libc::c_int as usize];
            i += 1
        }
    } else {
        q[0 as libc::c_int as usize] = (*_f0).o[0 as libc::c_int as usize];
        q[1 as libc::c_int as usize] = (*_f0).o[1 as libc::c_int as usize];
        q[(_e >> 1 as libc::c_int) as usize] += (*_f0).size[(_e >> 1 as libc::c_int) as usize]
            * (2 as libc::c_int * (_e & 1 as libc::c_int) - 1 as libc::c_int);
        qr_aff_project(
            (*pts.offset(0 as libc::c_int as isize)).as_mut_ptr(),
            _aff,
            q[0 as libc::c_int as usize],
            q[1 as libc::c_int as usize],
        );
        n0 += 1
    }
    if n1 > 0 as libc::c_int {
        edge_pts = (*_f1).edge_pts[_e as usize];
        i = 0 as libc::c_int;
        while i < n1 {
            (*pts.offset((n0 + i) as isize))[0 as libc::c_int as usize] =
                (*edge_pts.offset(i as isize)).pos[0 as libc::c_int as usize];
            (*pts.offset((n0 + i) as isize))[1 as libc::c_int as usize] =
                (*edge_pts.offset(i as isize)).pos[1 as libc::c_int as usize];
            i += 1
        }
    } else {
        q[0 as libc::c_int as usize] = (*_f1).o[0 as libc::c_int as usize];
        q[1 as libc::c_int as usize] = (*_f1).o[1 as libc::c_int as usize];
        q[(_e >> 1 as libc::c_int) as usize] += (*_f1).size[(_e >> 1 as libc::c_int) as usize]
            * (2 as libc::c_int * (_e & 1 as libc::c_int) - 1 as libc::c_int);
        qr_aff_project(
            (*pts.offset(n0 as isize)).as_mut_ptr(),
            _aff,
            q[0 as libc::c_int as usize],
            q[1 as libc::c_int as usize],
        );
        n1 += 1
    }
    qr_line_fit_points(_l, pts, npts, (*_aff).res);
    /* Make sure at least one finder center lies in the positive halfspace. */
    qr_line_orient(
        _l,
        (*(*_f0).c).pos[0 as libc::c_int as usize],
        (*(*_f0).c).pos[1 as libc::c_int as usize],
    );
    free(pts as *mut libc::c_void);
}
unsafe extern fn qr_finder_quick_crossing_check(
    mut _img: *const libc::c_uchar,
    mut _width: libc::c_int,
    mut _height: libc::c_int,
    mut _x0: libc::c_int,
    mut _y0: libc::c_int,
    mut _x1: libc::c_int,
    mut _y1: libc::c_int,
    mut _v: libc::c_int,
) -> libc::c_int {
    /*The points must be inside the image, and have a !_v:_v:!_v pattern.
    We don't scan the whole line initially, but quickly reject if the endpoints
     aren't !_v, or the midpoint isn't _v.
    If either end point is out of the image, or we don't encounter a _v pixel,
     we return a negative value, indicating the region should be considered
     empty.
    Otherwise, we return a positive value to indicate it is non-empty.*/
    if _x0 < 0 as libc::c_int
        || _x0 >= _width
        || _y0 < 0 as libc::c_int
        || _y0 >= _height
        || _x1 < 0 as libc::c_int
        || _x1 >= _width
        || _y1 < 0 as libc::c_int
        || _y1 >= _height
    {
        return -(1 as libc::c_int);
    }
    if (*_img.offset((_y0 * _width + _x0) as isize) == 0) as libc::c_int != _v
        || (*_img.offset((_y1 * _width + _x1) as isize) == 0) as libc::c_int != _v
    {
        return 1 as libc::c_int;
    }
    if (*_img.offset(
        ((_y0 + _y1 >> 1 as libc::c_int) * _width + (_x0 + _x1 >> 1 as libc::c_int)) as isize,
    ) == 0) as libc::c_int
        == _v
    {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
/*Locate the midpoint of a _v segment along a !_v:_v:!_v line from (_x0,_y0) to
 (_x1,_y1).
All coordinates, which are NOT in subpel resolution, must lie inside the
 image, and the endpoints are already assumed to have the value !_v.
The returned value is in subpel resolution.*/
unsafe extern fn qr_finder_locate_crossing(
    mut _img: *const libc::c_uchar,
    mut _width: libc::c_int,
    mut _height: libc::c_int,
    mut _x0: libc::c_int,
    mut _y0: libc::c_int,
    mut _x1: libc::c_int,
    mut _y1: libc::c_int,
    mut _v: libc::c_int,
    mut _p: *mut libc::c_int,
) -> libc::c_int {
    let mut x0: qr_point = [0; 2];
    let mut x1: qr_point = [0; 2];
    let mut dx: qr_point = [0; 2];
    let mut step: [libc::c_int; 2] = [0; 2];
    let mut steep: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    let mut derr: libc::c_int = 0;
    /*Use Bresenham's algorithm to trace along the line and find the exact
    transitions from !_v to _v and back.*/
    x0[0 as libc::c_int as usize] = _x0;
    x0[1 as libc::c_int as usize] = _y0;
    x1[0 as libc::c_int as usize] = _x1;
    x1[1 as libc::c_int as usize] = _y1;
    dx[0 as libc::c_int as usize] = abs(_x1 - _x0);
    dx[1 as libc::c_int as usize] = abs(_y1 - _y0);
    steep = (dx[1 as libc::c_int as usize] > dx[0 as libc::c_int as usize]) as libc::c_int;
    err = 0 as libc::c_int;
    derr = dx[(1 as libc::c_int - steep) as usize];
    step[0 as libc::c_int as usize] =
        (((_x0 < _x1) as libc::c_int) << 1 as libc::c_int) - 1 as libc::c_int;
    step[1 as libc::c_int as usize] =
        (((_y0 < _y1) as libc::c_int) << 1 as libc::c_int) - 1 as libc::c_int;
    loop
         /*Find the first crossing from !_v to _v.*/
         /*If we make it all the way to the other side, there's no crossing.*/
         {
        if x0[steep as usize] == x1[steep as usize] {
            return -(1 as libc::c_int)
        }
        x0[steep as usize] += step[steep as usize];
        err += derr;
        if err << 1 as libc::c_int > dx[steep as usize] {
            x0[(1 as libc::c_int - steep) as usize] +=
                step[(1 as libc::c_int - steep) as usize];
            err -= dx[steep as usize]
        }
        if (*_img.offset((x0[1 as libc::c_int as usize] * _width +
                              x0[0 as libc::c_int as usize]) as isize) == 0)
               as libc::c_int != _v {
            break ;
        }
    }
    /* Find the last crossing from _v to !_v. */
    err = 0 as libc::c_int;
    while !(x0[steep as usize] == x1[steep as usize]) {
        x1[steep as usize] -= step[steep as usize];
        err += derr;
        if err << 1 as libc::c_int > dx[steep as usize] {
            x1[(1 as libc::c_int - steep) as usize] -= step[(1 as libc::c_int - steep) as usize];
            err -= dx[steep as usize]
        }
        if (*_img.offset(
            (x1[1 as libc::c_int as usize] * _width + x1[0 as libc::c_int as usize]) as isize,
        ) == 0) as libc::c_int
            != _v
        {
            break;
        }
    }
    /* Return the midpoint of the _v segment. */
    *_p.offset(0 as libc::c_int as isize) =
        (x0[0 as libc::c_int as usize] + x1[0 as libc::c_int as usize] + 1 as libc::c_int)
            << 2 as libc::c_int
            >> 1 as libc::c_int;
    *_p.offset(1 as libc::c_int as isize) =
        (x0[1 as libc::c_int as usize] + x1[1 as libc::c_int as usize] + 1 as libc::c_int)
            << 2 as libc::c_int
            >> 1 as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern fn qr_aff_line_step(
    mut _aff: *const qr_aff,
    mut _l: *mut libc::c_int,
    mut _v: libc::c_int,
    mut _du: libc::c_int,
    mut _dv: *mut libc::c_int,
) -> libc::c_int {
    let mut shift: libc::c_int = 0;
    let mut round: libc::c_int = 0;
    let mut dv: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    n = (*_aff).fwd[0 as libc::c_int as usize][_v as usize] * *_l.offset(0 as libc::c_int as isize)
        + (*_aff).fwd[1 as libc::c_int as usize][_v as usize]
            * *_l.offset(1 as libc::c_int as isize);
    d = (*_aff).fwd[0 as libc::c_int as usize][(1 as libc::c_int - _v) as usize]
        * *_l.offset(0 as libc::c_int as isize)
        + (*_aff).fwd[1 as libc::c_int as usize][(1 as libc::c_int - _v) as usize]
            * *_l.offset(1 as libc::c_int as isize);
    if d < 0 as libc::c_int {
        n = -n;
        d = -d
    }
    shift = 0 as libc::c_int
        - (0 as libc::c_int
            - (qr_ilog(_du as libc::c_uint) + qr_ilog(abs(n) as libc::c_uint) + 3 as libc::c_int
                - ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                    * 8 as libc::c_int)
            & -((qr_ilog(_du as libc::c_uint) + qr_ilog(abs(n) as libc::c_uint) + 3 as libc::c_int
                - ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                    * 8 as libc::c_int
                > 0 as libc::c_int) as libc::c_int));
    round = (1 as libc::c_int) << shift >> 1 as libc::c_int;
    n = n + round >> shift;
    d = d + round >> shift;
    /*The line should not be outside 45 degrees of horizontal/vertical.
    TODO: We impose this restriction to help ensure the loop below terminates,
     but it should not technically be required.
    It also, however, ensures we avoid division by zero.*/
    if abs(n) >= d {
        return -(1 as libc::c_int);
    }
    n = -_du * n;
    dv = (n
        + ((d >> 1 as libc::c_int) + -((n < 0 as libc::c_int) as libc::c_int)
            ^ -((n < 0 as libc::c_int) as libc::c_int)))
        / d;
    if abs(dv) >= _du {
        return -(1 as libc::c_int);
    }
    *_dv = dv;
    return 0 as libc::c_int;
}
/*Computes the Hamming distance between two bit patterns (the number of bits
 that differ).
May stop counting after _maxdiff differences.*/
unsafe extern fn qr_hamming_dist(
    mut _y1: libc::c_uint,
    mut _y2: libc::c_uint,
    mut _maxdiff: libc::c_int,
) -> libc::c_int {
    let mut y: libc::c_uint = 0;
    let mut ret: libc::c_int = 0;
    y = _y1 ^ _y2;
    ret = 0 as libc::c_int;
    while ret < _maxdiff && y != 0 {
        y &= y.wrapping_sub(1 as libc::c_int as libc::c_uint);
        ret += 1
    }
    return ret;
}
/*Retrieve a bit (guaranteed to be 0 or 1) from the image, given coordinates in
subpel resolution which have not been bounds checked.*/
unsafe extern fn qr_img_get_bit(
    mut _img: *const libc::c_uchar,
    mut _width: libc::c_int,
    mut _height: libc::c_int,
    mut _x: libc::c_int,
    mut _y: libc::c_int,
) -> libc::c_int {
    _x >>= 2 as libc::c_int;
    _y >>= 2 as libc::c_int;
    return (*_img.offset(
        ((0 as libc::c_int
            - (0 as libc::c_int
                - (_y
                    + (_height - 1 as libc::c_int - _y
                        & -(((_height - 1 as libc::c_int) < _y) as libc::c_int)))
                & -((_y
                    + (_height - 1 as libc::c_int - _y
                        & -(((_height - 1 as libc::c_int) < _y) as libc::c_int))
                    > 0 as libc::c_int) as libc::c_int)))
            * _width
            + (0 as libc::c_int
                - (0 as libc::c_int
                    - (_x
                        + (_width - 1 as libc::c_int - _x
                            & -(((_width - 1 as libc::c_int) < _x) as libc::c_int)))
                    & -((_x
                        + (_width - 1 as libc::c_int - _x
                            & -(((_width - 1 as libc::c_int) < _x) as libc::c_int))
                        > 0 as libc::c_int) as libc::c_int)))) as isize,
    ) as libc::c_int
        != 0 as libc::c_int) as libc::c_int;
}
unsafe extern fn qr_hom_cell_init(
    mut _cell: *mut qr_hom_cell,
    mut _u0: libc::c_int,
    mut _v0: libc::c_int,
    mut _u1: libc::c_int,
    mut _v1: libc::c_int,
    mut _u2: libc::c_int,
    mut _v2: libc::c_int,
    mut _u3: libc::c_int,
    mut _v3: libc::c_int,
    mut _x0: libc::c_int,
    mut _y0: libc::c_int,
    mut _x1: libc::c_int,
    mut _y1: libc::c_int,
    mut _x2: libc::c_int,
    mut _y2: libc::c_int,
    mut _x3: libc::c_int,
    mut _y3: libc::c_int,
) {
    let mut du10: libc::c_int = 0;
    let mut du20: libc::c_int = 0;
    let mut du30: libc::c_int = 0;
    let mut du31: libc::c_int = 0;
    let mut du32: libc::c_int = 0;
    let mut dv10: libc::c_int = 0;
    let mut dv20: libc::c_int = 0;
    let mut dv30: libc::c_int = 0;
    let mut dv31: libc::c_int = 0;
    let mut dv32: libc::c_int = 0;
    let mut dx10: libc::c_int = 0;
    let mut dx20: libc::c_int = 0;
    let mut dx30: libc::c_int = 0;
    let mut dx31: libc::c_int = 0;
    let mut dx32: libc::c_int = 0;
    let mut dy10: libc::c_int = 0;
    let mut dy20: libc::c_int = 0;
    let mut dy30: libc::c_int = 0;
    let mut dy31: libc::c_int = 0;
    let mut dy32: libc::c_int = 0;
    let mut a00: libc::c_int = 0;
    let mut a01: libc::c_int = 0;
    let mut a02: libc::c_int = 0;
    let mut a10: libc::c_int = 0;
    let mut a11: libc::c_int = 0;
    let mut a12: libc::c_int = 0;
    let mut a20: libc::c_int = 0;
    let mut a21: libc::c_int = 0;
    let mut a22: libc::c_int = 0;
    let mut i00: libc::c_int = 0;
    let mut i01: libc::c_int = 0;
    let mut i10: libc::c_int = 0;
    let mut i11: libc::c_int = 0;
    let mut i20: libc::c_int = 0;
    let mut i21: libc::c_int = 0;
    let mut i22: libc::c_int = 0;
    let mut b0: libc::c_int = 0;
    let mut b1: libc::c_int = 0;
    let mut b2: libc::c_int = 0;
    let mut shift: libc::c_int = 0;
    let mut round: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    /*First, correct for the arrangement of the source points.
    We take advantage of the fact that we know the source points have a very
     limited dynamic range (so there will never be overflow) and a small amount
     of projective distortion.*/
    du10 = _u1 - _u0;
    du20 = _u2 - _u0;
    du30 = _u3 - _u0;
    du31 = _u3 - _u1;
    du32 = _u3 - _u2;
    dv10 = _v1 - _v0;
    dv20 = _v2 - _v0;
    dv30 = _v3 - _v0;
    dv31 = _v3 - _v1;
    dv32 = _v3 - _v2;
    /*Compute the coefficients of the forward transform from the unit square to
    the source point configuration.*/
    a20 = du32 * dv10 - du10 * dv32;
    a21 = du20 * dv31 - du31 * dv20;
    if a20 != 0 || a21 != 0 {
        a22 = du32 * dv31 - du31 * dv32
    } else {
        /*If the source grid points aren't in a non-affine arrangement, there's no
         reason to scale everything by du32*dv31-du31*dv32.
        Not doing so allows a much larger dynamic range, and is the only way we can
         initialize a base cell that covers the whole grid.*/
        a22 = 1 as libc::c_int
    }
    a00 = du10 * (a20 + a22);
    a01 = du20 * (a21 + a22);
    a10 = dv10 * (a20 + a22);
    a11 = dv20 * (a21 + a22);
    /* Now compute the inverse transform. */
    i00 = a11 * a22;
    i01 = -a01 * a22;
    i10 = -a10 * a22;
    i11 = a00 * a22;
    i20 = a10 * a21 - a11 * a20;
    i21 = a01 * a20 - a00 * a21;
    i22 = a00 * a11 - a01 * a10;
    /*Invert the coefficients.
    Since i22 is the largest, we divide it by all the others.
    The quotient is often exact (e.g., when the source points contain no
     projective distortion), and is never zero.
    Hence we can use zero to signal "infinity" when the divisor is zero.*/
    if i00 != 0 {
        i00 = (i22
            + ((abs(i00) >> 1 as libc::c_int) + -((i22 < 0 as libc::c_int) as libc::c_int)
                ^ -((i22 < 0 as libc::c_int) as libc::c_int)))
            / abs(i00)
            + -((i00 < 0 as libc::c_int) as libc::c_int)
            ^ -((i00 < 0 as libc::c_int) as libc::c_int)
    }
    if i01 != 0 {
        i01 = (i22
            + ((abs(i01) >> 1 as libc::c_int) + -((i22 < 0 as libc::c_int) as libc::c_int)
                ^ -((i22 < 0 as libc::c_int) as libc::c_int)))
            / abs(i01)
            + -((i01 < 0 as libc::c_int) as libc::c_int)
            ^ -((i01 < 0 as libc::c_int) as libc::c_int)
    }
    if i10 != 0 {
        i10 = (i22
            + ((abs(i10) >> 1 as libc::c_int) + -((i22 < 0 as libc::c_int) as libc::c_int)
                ^ -((i22 < 0 as libc::c_int) as libc::c_int)))
            / abs(i10)
            + -((i10 < 0 as libc::c_int) as libc::c_int)
            ^ -((i10 < 0 as libc::c_int) as libc::c_int)
    }
    if i11 != 0 {
        i11 = (i22
            + ((abs(i11) >> 1 as libc::c_int) + -((i22 < 0 as libc::c_int) as libc::c_int)
                ^ -((i22 < 0 as libc::c_int) as libc::c_int)))
            / abs(i11)
            + -((i11 < 0 as libc::c_int) as libc::c_int)
            ^ -((i11 < 0 as libc::c_int) as libc::c_int)
    }
    if i20 != 0 {
        i20 = (i22
            + ((abs(i20) >> 1 as libc::c_int) + -((i22 < 0 as libc::c_int) as libc::c_int)
                ^ -((i22 < 0 as libc::c_int) as libc::c_int)))
            / abs(i20)
            + -((i20 < 0 as libc::c_int) as libc::c_int)
            ^ -((i20 < 0 as libc::c_int) as libc::c_int)
    }
    if i21 != 0 {
        i21 = (i22
            + ((abs(i21) >> 1 as libc::c_int) + -((i22 < 0 as libc::c_int) as libc::c_int)
                ^ -((i22 < 0 as libc::c_int) as libc::c_int)))
            / abs(i21)
            + -((i21 < 0 as libc::c_int) as libc::c_int)
            ^ -((i21 < 0 as libc::c_int) as libc::c_int)
    }
    /* Now compute the map from the unit square into the image. */
    dx10 = _x1 - _x0;
    dx20 = _x2 - _x0;
    dx30 = _x3 - _x0;
    dx31 = _x3 - _x1;
    dx32 = _x3 - _x2;
    dy10 = _y1 - _y0;
    dy20 = _y2 - _y0;
    dy30 = _y3 - _y0;
    dy31 = _y3 - _y1;
    dy32 = _y3 - _y2;
    a20 = dx32 * dy10 - dx10 * dy32;
    a21 = dx20 * dy31 - dx31 * dy20;
    a22 = dx32 * dy31 - dx31 * dy32;
    /* Figure out if we need to downscale anything. */
    b0 = qr_ilog(
        (abs(dx10) - (abs(dx10) - abs(dy10) & -((abs(dy10) > abs(dx10)) as libc::c_int)))
            as libc::c_uint,
    ) + qr_ilog(abs(a20 + a22) as libc::c_uint);
    b1 = qr_ilog(
        (abs(dx20) - (abs(dx20) - abs(dy20) & -((abs(dy20) > abs(dx20)) as libc::c_int)))
            as libc::c_uint,
    ) + qr_ilog(abs(a21 + a22) as libc::c_uint);
    b2 = qr_ilog(
        (abs(a20)
            - (abs(a20) - abs(a21) & -((abs(a21) > abs(a20)) as libc::c_int))
            - (abs(a20)
                - (abs(a20) - abs(a21) & -((abs(a21) > abs(a20)) as libc::c_int))
                - abs(a22)
                & -((abs(a22)
                    > abs(a20) - (abs(a20) - abs(a21) & -((abs(a21) > abs(a20)) as libc::c_int)))
                    as libc::c_int))) as libc::c_uint,
    );
    shift = 0 as libc::c_int
        - (0 as libc::c_int
            - (b0
                - (b0 - b1 & -((b1 > b0) as libc::c_int))
                - (b0 - (b0 - b1 & -((b1 > b0) as libc::c_int)) - b2
                    & -((b2 > b0 - (b0 - b1 & -((b1 > b0) as libc::c_int))) as libc::c_int))
                - (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                    * 8 as libc::c_int
                    - 3 as libc::c_int
                    - 2 as libc::c_int))
            & -((b0
                - (b0 - b1 & -((b1 > b0) as libc::c_int))
                - (b0 - (b0 - b1 & -((b1 > b0) as libc::c_int)) - b2
                    & -((b2 > b0 - (b0 - b1 & -((b1 > b0) as libc::c_int))) as libc::c_int))
                - (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                    * 8 as libc::c_int
                    - 3 as libc::c_int
                    - 2 as libc::c_int)
                > 0 as libc::c_int) as libc::c_int));
    round = (1 as libc::c_int) << shift >> 1 as libc::c_int;
    /* Compute the final coefficients of the forward transform. */
    a00 = (dx10 as libc::c_longlong * (a20 + a22) as libc::c_longlong + round as libc::c_longlong
        >> shift) as libc::c_int;
    a01 = (dx20 as libc::c_longlong * (a21 + a22) as libc::c_longlong + round as libc::c_longlong
        >> shift) as libc::c_int;
    a10 = (dy10 as libc::c_longlong * (a20 + a22) as libc::c_longlong + round as libc::c_longlong
        >> shift) as libc::c_int;
    a11 = (dy20 as libc::c_longlong * (a21 + a22) as libc::c_longlong + round as libc::c_longlong
        >> shift) as libc::c_int;
    /*And compose the two transforms.
    Since we inverted the coefficients above, we divide by them here instead
     of multiplying.
    This lets us take advantage of the full dynamic range.
    Note a zero divisor is really "infinity", and thus the quotient should also
     be zero.*/
    (*_cell).fwd[0 as libc::c_int as usize][0 as libc::c_int as usize] = (if i00 != 0 {
        (a00 + ((i00 >> 1 as libc::c_int) + -((a00 < 0 as libc::c_int) as libc::c_int)
            ^ -((a00 < 0 as libc::c_int) as libc::c_int)))
            / i00
    } else {
        0 as libc::c_int
    }) + (if i10 != 0 {
        (a01 + ((i10 >> 1 as libc::c_int) + -((a01 < 0 as libc::c_int) as libc::c_int)
            ^ -((a01 < 0 as libc::c_int) as libc::c_int)))
            / i10
    } else {
        0 as libc::c_int
    });
    (*_cell).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize] = (if i01 != 0 {
        (a00 + ((i01 >> 1 as libc::c_int) + -((a00 < 0 as libc::c_int) as libc::c_int)
            ^ -((a00 < 0 as libc::c_int) as libc::c_int)))
            / i01
    } else {
        0 as libc::c_int
    }) + (if i11 != 0 {
        (a01 + ((i11 >> 1 as libc::c_int) + -((a01 < 0 as libc::c_int) as libc::c_int)
            ^ -((a01 < 0 as libc::c_int) as libc::c_int)))
            / i11
    } else {
        0 as libc::c_int
    });
    (*_cell).fwd[1 as libc::c_int as usize][0 as libc::c_int as usize] = (if i00 != 0 {
        (a10 + ((i00 >> 1 as libc::c_int) + -((a10 < 0 as libc::c_int) as libc::c_int)
            ^ -((a10 < 0 as libc::c_int) as libc::c_int)))
            / i00
    } else {
        0 as libc::c_int
    }) + (if i10 != 0 {
        (a11 + ((i10 >> 1 as libc::c_int) + -((a11 < 0 as libc::c_int) as libc::c_int)
            ^ -((a11 < 0 as libc::c_int) as libc::c_int)))
            / i10
    } else {
        0 as libc::c_int
    });
    (*_cell).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize] = (if i01 != 0 {
        (a10 + ((i01 >> 1 as libc::c_int) + -((a10 < 0 as libc::c_int) as libc::c_int)
            ^ -((a10 < 0 as libc::c_int) as libc::c_int)))
            / i01
    } else {
        0 as libc::c_int
    }) + (if i11 != 0 {
        (a11 + ((i11 >> 1 as libc::c_int) + -((a11 < 0 as libc::c_int) as libc::c_int)
            ^ -((a11 < 0 as libc::c_int) as libc::c_int)))
            / i11
    } else {
        0 as libc::c_int
    });
    (*_cell).fwd[2 as libc::c_int as usize][0 as libc::c_int as usize] = (if i00 != 0 {
        (a20 + ((i00 >> 1 as libc::c_int) + -((a20 < 0 as libc::c_int) as libc::c_int)
            ^ -((a20 < 0 as libc::c_int) as libc::c_int)))
            / i00
    } else {
        0 as libc::c_int
    }) + (if i10 != 0 {
        (a21 + ((i10 >> 1 as libc::c_int) + -((a21 < 0 as libc::c_int) as libc::c_int)
            ^ -((a21 < 0 as libc::c_int) as libc::c_int)))
            / i10
    } else {
        0 as libc::c_int
    }) + (if i20 != 0 {
        (a22 + ((i20 >> 1 as libc::c_int) + -((a22 < 0 as libc::c_int) as libc::c_int)
            ^ -((a22 < 0 as libc::c_int) as libc::c_int)))
            / i20
    } else {
        0 as libc::c_int
    }) + round
        >> shift;
    (*_cell).fwd[2 as libc::c_int as usize][1 as libc::c_int as usize] = (if i01 != 0 {
        (a20 + ((i01 >> 1 as libc::c_int) + -((a20 < 0 as libc::c_int) as libc::c_int)
            ^ -((a20 < 0 as libc::c_int) as libc::c_int)))
            / i01
    } else {
        0 as libc::c_int
    }) + (if i11 != 0 {
        (a21 + ((i11 >> 1 as libc::c_int) + -((a21 < 0 as libc::c_int) as libc::c_int)
            ^ -((a21 < 0 as libc::c_int) as libc::c_int)))
            / i11
    } else {
        0 as libc::c_int
    }) + (if i21 != 0 {
        (a22 + ((i21 >> 1 as libc::c_int) + -((a22 < 0 as libc::c_int) as libc::c_int)
            ^ -((a22 < 0 as libc::c_int) as libc::c_int)))
            / i21
    } else {
        0 as libc::c_int
    }) + round
        >> shift;
    (*_cell).fwd[2 as libc::c_int as usize][2 as libc::c_int as usize] = a22 + round >> shift;
    /*Mathematically, a02 and a12 are exactly zero.
    However, that concentrates all of the rounding error in the (_u3,_v3)
     corner; we compute offsets which distribute it over the whole range.*/
    x = (*_cell).fwd[0 as libc::c_int as usize][0 as libc::c_int as usize] * du10
        + (*_cell).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize] * dv10;
    y = (*_cell).fwd[1 as libc::c_int as usize][0 as libc::c_int as usize] * du10
        + (*_cell).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize] * dv10;
    w = (*_cell).fwd[2 as libc::c_int as usize][0 as libc::c_int as usize] * du10
        + (*_cell).fwd[2 as libc::c_int as usize][1 as libc::c_int as usize] * dv10
        + (*_cell).fwd[2 as libc::c_int as usize][2 as libc::c_int as usize];
    a02 = dx10 * w - x;
    a12 = dy10 * w - y;
    x = (*_cell).fwd[0 as libc::c_int as usize][0 as libc::c_int as usize] * du20
        + (*_cell).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize] * dv20;
    y = (*_cell).fwd[1 as libc::c_int as usize][0 as libc::c_int as usize] * du20
        + (*_cell).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize] * dv20;
    w = (*_cell).fwd[2 as libc::c_int as usize][0 as libc::c_int as usize] * du20
        + (*_cell).fwd[2 as libc::c_int as usize][1 as libc::c_int as usize] * dv20
        + (*_cell).fwd[2 as libc::c_int as usize][2 as libc::c_int as usize];
    a02 += dx20 * w - x;
    a12 += dy20 * w - y;
    x = (*_cell).fwd[0 as libc::c_int as usize][0 as libc::c_int as usize] * du30
        + (*_cell).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize] * dv30;
    y = (*_cell).fwd[1 as libc::c_int as usize][0 as libc::c_int as usize] * du30
        + (*_cell).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize] * dv30;
    w = (*_cell).fwd[2 as libc::c_int as usize][0 as libc::c_int as usize] * du30
        + (*_cell).fwd[2 as libc::c_int as usize][1 as libc::c_int as usize] * dv30
        + (*_cell).fwd[2 as libc::c_int as usize][2 as libc::c_int as usize];
    a02 += dx30 * w - x;
    a12 += dy30 * w - y;
    (*_cell).fwd[0 as libc::c_int as usize][2 as libc::c_int as usize] =
        a02 + 2 as libc::c_int >> 2 as libc::c_int;
    (*_cell).fwd[1 as libc::c_int as usize][2 as libc::c_int as usize] =
        a12 + 2 as libc::c_int >> 2 as libc::c_int;
    (*_cell).x0 = _x0;
    (*_cell).y0 = _y0;
    (*_cell).u0 = _u0;
    (*_cell).v0 = _v0;
}
/*Finish a partial projection, converting from homogeneous coordinates to the
 normal 2-D representation.
In loops, we can avoid many multiplies by computing the homogeneous _x, _y,
 and _w incrementally, but we cannot avoid the divisions, done here.*/
unsafe extern fn qr_hom_cell_fproject(
    mut _p: *mut libc::c_int,
    mut _cell: *const qr_hom_cell,
    mut _x: libc::c_int,
    mut _y: libc::c_int,
    mut _w: libc::c_int,
) {
    if _w == 0 as libc::c_int {
        *_p.offset(0 as libc::c_int as isize) = if _x < 0 as libc::c_int {
            (-(2147483647 as libc::c_int)) - 1 as libc::c_int
        } else {
            2147483647 as libc::c_int
        };
        *_p.offset(1 as libc::c_int as isize) = if _y < 0 as libc::c_int {
            (-(2147483647 as libc::c_int)) - 1 as libc::c_int
        } else {
            2147483647 as libc::c_int
        }
    } else {
        if _w < 0 as libc::c_int {
            _x = -_x;
            _y = -_y;
            _w = -_w
        }
        *_p.offset(0 as libc::c_int as isize) = (_x
            + ((_w >> 1 as libc::c_int) + -((_x < 0 as libc::c_int) as libc::c_int)
                ^ -((_x < 0 as libc::c_int) as libc::c_int)))
            / _w
            + (*_cell).x0;
        *_p.offset(1 as libc::c_int as isize) = (_y
            + ((_w >> 1 as libc::c_int) + -((_y < 0 as libc::c_int) as libc::c_int)
                ^ -((_y < 0 as libc::c_int) as libc::c_int)))
            / _w
            + (*_cell).y0
    };
}
unsafe extern fn qr_hom_cell_project(
    mut _p: *mut libc::c_int,
    mut _cell: *const qr_hom_cell,
    mut _u: libc::c_int,
    mut _v: libc::c_int,
    mut _res: libc::c_int,
) {
    _u -= (*_cell).u0 << _res;
    _v -= (*_cell).v0 << _res;
    qr_hom_cell_fproject(
        _p,
        _cell,
        (*_cell).fwd[0 as libc::c_int as usize][0 as libc::c_int as usize] * _u
            + (*_cell).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize] * _v
            + ((*_cell).fwd[0 as libc::c_int as usize][2 as libc::c_int as usize] << _res),
        (*_cell).fwd[1 as libc::c_int as usize][0 as libc::c_int as usize] * _u
            + (*_cell).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize] * _v
            + ((*_cell).fwd[1 as libc::c_int as usize][2 as libc::c_int as usize] << _res),
        (*_cell).fwd[2 as libc::c_int as usize][0 as libc::c_int as usize] * _u
            + (*_cell).fwd[2 as libc::c_int as usize][1 as libc::c_int as usize] * _v
            + ((*_cell).fwd[2 as libc::c_int as usize][2 as libc::c_int as usize] << _res),
    );
}
/*Retrieves the bits corresponding to the alignment pattern template centered
at the given location in the original image (at subpel precision).*/
unsafe extern fn qr_alignment_pattern_fetch(
    mut _p: *mut [qr_point; 5],
    mut _x0: libc::c_int,
    mut _y0: libc::c_int,
    mut _img: *const libc::c_uchar,
    mut _width: libc::c_int,
    mut _height: libc::c_int,
) -> libc::c_uint {
    let mut v: libc::c_uint = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut dx: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    dx = _x0
        - (*_p.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize]
            [0 as libc::c_int as usize];
    dy = _y0
        - (*_p.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize]
            [1 as libc::c_int as usize];
    v = 0 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int;
    k = i;
    while i < 5 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 5 as libc::c_int {
            v |= (qr_img_get_bit(
                _img,
                _width,
                _height,
                (*_p.offset(i as isize))[j as usize][0 as libc::c_int as usize] + dx,
                (*_p.offset(i as isize))[j as usize][1 as libc::c_int as usize] + dy,
            ) << k) as libc::c_uint;
            j += 1;
            k += 1
        }
        i += 1
    }
    return v;
}
/* Searches for an alignment pattern near the given location. */
unsafe extern fn qr_alignment_pattern_search(
    mut _p: *mut libc::c_int,
    mut _cell: *const qr_hom_cell,
    mut _u: libc::c_int,
    mut _v: libc::c_int,
    mut _r: libc::c_int,
    mut _img: *const libc::c_uchar,
    mut _width: libc::c_int,
    mut _height: libc::c_int,
) -> libc::c_int {
    let mut c: [qr_point; 4] = [[0; 2]; 4];
    let mut nc: [libc::c_int; 4] = [0; 4];
    let mut p: [[qr_point; 5]; 5] = [[[0; 2]; 5]; 5];
    let mut pc: qr_point = [0; 2];
    let mut best_match: libc::c_uint = 0;
    let mut best_dist: libc::c_int = 0;
    let mut bestx: libc::c_int = 0;
    let mut besty: libc::c_int = 0;
    let mut match_0: libc::c_uint = 0;
    let mut dist: libc::c_int = 0;
    let mut u: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut x0: libc::c_int = 0;
    let mut y0: libc::c_int = 0;
    let mut w0: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut dxdu: libc::c_int = 0;
    let mut dydu: libc::c_int = 0;
    let mut dwdu: libc::c_int = 0;
    let mut dxdv: libc::c_int = 0;
    let mut dydv: libc::c_int = 0;
    let mut dwdv: libc::c_int = 0;
    let mut dx: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    /*Build up a basic template using _cell to control shape and scale.
    We project the points in the template back to the image just once, since if
     the alignment pattern has moved, we don't really know why.
    If it's because of radial distortion, or the code wasn't flat, or something
     else, there's no reason to expect that a re-projection around each
     subsequent search point would be any closer to the actual shape than our
     first projection.
    Therefore we simply slide this template around, as is.*/
    u = _u - 2 as libc::c_int - (*_cell).u0;
    v = _v - 2 as libc::c_int - (*_cell).v0;
    x0 = (*_cell).fwd[0 as libc::c_int as usize][0 as libc::c_int as usize] * u
        + (*_cell).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize] * v
        + (*_cell).fwd[0 as libc::c_int as usize][2 as libc::c_int as usize];
    y0 = (*_cell).fwd[1 as libc::c_int as usize][0 as libc::c_int as usize] * u
        + (*_cell).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize] * v
        + (*_cell).fwd[1 as libc::c_int as usize][2 as libc::c_int as usize];
    w0 = (*_cell).fwd[2 as libc::c_int as usize][0 as libc::c_int as usize] * u
        + (*_cell).fwd[2 as libc::c_int as usize][1 as libc::c_int as usize] * v
        + (*_cell).fwd[2 as libc::c_int as usize][2 as libc::c_int as usize];
    dxdu = (*_cell).fwd[0 as libc::c_int as usize][0 as libc::c_int as usize];
    dydu = (*_cell).fwd[1 as libc::c_int as usize][0 as libc::c_int as usize];
    dwdu = (*_cell).fwd[2 as libc::c_int as usize][0 as libc::c_int as usize];
    dxdv = (*_cell).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize];
    dydv = (*_cell).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize];
    dwdv = (*_cell).fwd[2 as libc::c_int as usize][1 as libc::c_int as usize];
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        x = x0;
        y = y0;
        w = w0;
        j = 0 as libc::c_int;
        while j < 5 as libc::c_int {
            qr_hom_cell_fproject(p[i as usize][j as usize].as_mut_ptr(), _cell, x, y, w);
            x += dxdu;
            y += dydu;
            w += dwdu;
            j += 1
        }
        x0 += dxdv;
        y0 += dydv;
        w0 += dwdv;
        i += 1
    }
    bestx = p[2 as libc::c_int as usize][2 as libc::c_int as usize][0 as libc::c_int as usize];
    besty = p[2 as libc::c_int as usize][2 as libc::c_int as usize][1 as libc::c_int as usize];
    best_match = qr_alignment_pattern_fetch(p.as_mut_ptr(), bestx, besty, _img, _width, _height);
    best_dist =
        qr_hamming_dist(best_match, 0x1f8d63f as libc::c_int as libc::c_uint, 25 as libc::c_int);
    if best_dist > 0 as libc::c_int {
        u = _u - (*_cell).u0;
        v = _v - (*_cell).v0;
        x = (*_cell).fwd[0 as libc::c_int as usize][0 as libc::c_int as usize] * u
            + (*_cell).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize] * v
            + (*_cell).fwd[0 as libc::c_int as usize][2 as libc::c_int as usize]
            << 2 as libc::c_int;
        y = (*_cell).fwd[1 as libc::c_int as usize][0 as libc::c_int as usize] * u
            + (*_cell).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize] * v
            + (*_cell).fwd[1 as libc::c_int as usize][2 as libc::c_int as usize]
            << 2 as libc::c_int;
        w = (*_cell).fwd[2 as libc::c_int as usize][0 as libc::c_int as usize] * u
            + (*_cell).fwd[2 as libc::c_int as usize][1 as libc::c_int as usize] * v
            + (*_cell).fwd[2 as libc::c_int as usize][2 as libc::c_int as usize]
            << 2 as libc::c_int;
        /*Search an area at most _r modules around the target location, in
        concentric squares..*/
        i = 1 as libc::c_int;
        while i < _r << 2 as libc::c_int {
            let mut side_len: libc::c_int = 0;
            side_len = (i << 1 as libc::c_int) - 1 as libc::c_int;
            x -= dxdu + dxdv;
            y -= dydu + dydv;
            w -= dwdu + dwdv;
            j = 0 as libc::c_int;
            while j < 4 as libc::c_int * side_len {
                let mut dir: libc::c_int = 0;
                qr_hom_cell_fproject(pc.as_mut_ptr(), _cell, x, y, w);
                match_0 = qr_alignment_pattern_fetch(
                    p.as_mut_ptr(),
                    pc[0 as libc::c_int as usize],
                    pc[1 as libc::c_int as usize],
                    _img,
                    _width,
                    _height,
                );
                dist = qr_hamming_dist(
                    match_0,
                    0x1f8d63f as libc::c_int as libc::c_uint,
                    best_dist + 1 as libc::c_int,
                );
                if dist < best_dist {
                    best_match = match_0;
                    best_dist = dist;
                    bestx = pc[0 as libc::c_int as usize];
                    besty = pc[1 as libc::c_int as usize]
                }
                if j < 2 as libc::c_int * side_len {
                    dir = (j >= side_len) as libc::c_int;
                    x += (*_cell).fwd[0 as libc::c_int as usize][dir as usize];
                    y += (*_cell).fwd[1 as libc::c_int as usize][dir as usize];
                    w += (*_cell).fwd[2 as libc::c_int as usize][dir as usize]
                } else {
                    dir = (j >= 3 as libc::c_int * side_len) as libc::c_int;
                    x -= (*_cell).fwd[0 as libc::c_int as usize][dir as usize];
                    y -= (*_cell).fwd[1 as libc::c_int as usize][dir as usize];
                    w -= (*_cell).fwd[2 as libc::c_int as usize][dir as usize]
                }
                if best_dist == 0 {
                    break;
                }
                j += 1
            }
            if best_dist == 0 {
                break;
            }
            i += 1
        }
    }
    /*If the best result we got was sufficiently bad, reject the match.
    If we're wrong and we include it, we can grossly distort the nearby
     region, whereas using the initial starting point should at least be
     consistent with the geometry we already have.*/
    if best_dist > 6 as libc::c_int {
        *_p.offset(0 as libc::c_int as isize) =
            p[2 as libc::c_int as usize][2 as libc::c_int as usize][0 as libc::c_int as usize];
        *_p.offset(1 as libc::c_int as isize) =
            p[2 as libc::c_int as usize][2 as libc::c_int as usize][1 as libc::c_int as usize];
        return -(1 as libc::c_int);
    }
    /* Now try to get a more accurate location of the pattern center. */
    dx = bestx - p[2 as libc::c_int as usize][2 as libc::c_int as usize][0 as libc::c_int as usize];
    dy = besty - p[2 as libc::c_int as usize][2 as libc::c_int as usize][1 as libc::c_int as usize];
    memset(
        nc.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_int; 4]>() as libc::c_ulong,
    );
    memset(
        c.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[qr_point; 4]>() as libc::c_ulong,
    );
    /*We consider 8 lines across the finder pattern in turn.
    If we actually found a symmetric pattern along that line, search for its
     exact center in the image.
    There are plenty more lines we could use if these don't work, but if we've
     found anything remotely close to an alignment pattern, we should be able
     to use most of these.*/
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        static mut MASK_TESTS: [[libc::c_uint; 2]; 8] = [
            [0x1040041 as libc::c_int as libc::c_uint, 0x1000001 as libc::c_int as libc::c_uint],
            [0x41040 as libc::c_int as libc::c_uint, 0x1000 as libc::c_int as libc::c_uint],
            [0x110110 as libc::c_int as libc::c_uint, 0x100010 as libc::c_int as libc::c_uint],
            [0x11100 as libc::c_int as libc::c_uint, 0x1000 as libc::c_int as libc::c_uint],
            [0x420084 as libc::c_int as libc::c_uint, 0x400004 as libc::c_int as libc::c_uint],
            [0x21080 as libc::c_int as libc::c_uint, 0x1000 as libc::c_int as libc::c_uint],
            [0x6c00 as libc::c_int as libc::c_uint, 0x4400 as libc::c_int as libc::c_uint],
            [0x3800 as libc::c_int as libc::c_uint, 0x1000 as libc::c_int as libc::c_uint],
        ];
        static mut MASK_COORDS: [[libc::c_uchar; 2]; 8] = [
            [0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar],
            [1 as libc::c_int as libc::c_uchar, 1 as libc::c_int as libc::c_uchar],
            [4 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar],
            [3 as libc::c_int as libc::c_uchar, 1 as libc::c_int as libc::c_uchar],
            [2 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar],
            [2 as libc::c_int as libc::c_uchar, 1 as libc::c_int as libc::c_uchar],
            [0 as libc::c_int as libc::c_uchar, 2 as libc::c_int as libc::c_uchar],
            [1 as libc::c_int as libc::c_uchar, 2 as libc::c_int as libc::c_uchar],
        ];
        if best_match & MASK_TESTS[i as usize][0 as libc::c_int as usize]
            == MASK_TESTS[i as usize][1 as libc::c_int as usize]
        {
            let mut x0_0: libc::c_int = 0;
            let mut y0_0: libc::c_int = 0;
            let mut x1: libc::c_int = 0;
            let mut y1: libc::c_int = 0;
            x0_0 = p[MASK_COORDS[i as usize][1 as libc::c_int as usize] as usize]
                [MASK_COORDS[i as usize][0 as libc::c_int as usize] as usize]
                [0 as libc::c_int as usize]
                + dx
                >> 2 as libc::c_int;
            if !(x0_0 < 0 as libc::c_int || x0_0 >= _width) {
                y0_0 = p[MASK_COORDS[i as usize][1 as libc::c_int as usize] as usize]
                    [MASK_COORDS[i as usize][0 as libc::c_int as usize] as usize]
                    [1 as libc::c_int as usize]
                    + dy
                    >> 2 as libc::c_int;
                if !(y0_0 < 0 as libc::c_int || y0_0 >= _height) {
                    x1 = p[(4 as libc::c_int
                        - MASK_COORDS[i as usize][1 as libc::c_int as usize] as libc::c_int)
                        as usize][(4 as libc::c_int
                        - MASK_COORDS[i as usize][0 as libc::c_int as usize] as libc::c_int)
                        as usize][0 as libc::c_int as usize]
                        + dx
                        >> 2 as libc::c_int;
                    if !(x1 < 0 as libc::c_int || x1 >= _width) {
                        y1 = p[(4 as libc::c_int
                            - MASK_COORDS[i as usize][1 as libc::c_int as usize] as libc::c_int)
                            as usize][(4 as libc::c_int
                            - MASK_COORDS[i as usize][0 as libc::c_int as usize] as libc::c_int)
                            as usize][1 as libc::c_int as usize]
                            + dy
                            >> 2 as libc::c_int;
                        if !(y1 < 0 as libc::c_int || y1 >= _height) {
                            if qr_finder_locate_crossing(
                                _img,
                                _width,
                                _height,
                                x0_0,
                                y0_0,
                                x1,
                                y1,
                                i & 1 as libc::c_int,
                                pc.as_mut_ptr(),
                            ) == 0
                            {
                                let mut w_0: libc::c_int = 0;
                                let mut cx: libc::c_int = 0;
                                let mut cy: libc::c_int = 0;
                                cx = pc[0 as libc::c_int as usize] - bestx;
                                cy = pc[1 as libc::c_int as usize] - besty;
                                if i & 1 as libc::c_int != 0 {
                                    /*Weight crossings around the center dot more highly, as they are
                                    generally more reliable.*/
                                    w_0 = 3 as libc::c_int;
                                    cx += cx << 1 as libc::c_int;
                                    cy += cy << 1 as libc::c_int
                                } else {
                                    w_0 = 1 as libc::c_int
                                }
                                nc[(i >> 1 as libc::c_int) as usize] += w_0;
                                c[(i >> 1 as libc::c_int) as usize][0 as libc::c_int as usize] +=
                                    cx;
                                c[(i >> 1 as libc::c_int) as usize][1 as libc::c_int as usize] += cy
                            }
                        }
                    }
                }
            }
        }
        i += 1
    }
    /* Sum offsets from lines in orthogonal directions. */
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        let mut a: libc::c_int = 0;
        let mut b: libc::c_int = 0;
        a = nc[(i << 1 as libc::c_int) as usize];
        b = nc[(i << 1 as libc::c_int | 1 as libc::c_int) as usize];
        if a != 0 && b != 0 {
            let mut w_1: libc::c_int = 0;
            w_1 = a - (a - b & -((b > a) as libc::c_int));
            c[(i << 1 as libc::c_int) as usize][0 as libc::c_int as usize] = (w_1
                * (b * c[(i << 1 as libc::c_int) as usize][0 as libc::c_int as usize]
                    + a * c[(i << 1 as libc::c_int | 1 as libc::c_int) as usize]
                        [0 as libc::c_int as usize])
                + ((a * b >> 1 as libc::c_int)
                    + -((w_1
                        * (b * c[(i << 1 as libc::c_int) as usize][0 as libc::c_int as usize]
                            + a * c[(i << 1 as libc::c_int | 1 as libc::c_int) as usize]
                                [0 as libc::c_int as usize])
                        < 0 as libc::c_int) as libc::c_int)
                    ^ -((w_1
                        * (b * c[(i << 1 as libc::c_int) as usize][0 as libc::c_int as usize]
                            + a * c[(i << 1 as libc::c_int | 1 as libc::c_int) as usize]
                                [0 as libc::c_int as usize])
                        < 0 as libc::c_int) as libc::c_int)))
                / (a * b);
            c[(i << 1 as libc::c_int) as usize][1 as libc::c_int as usize] = (w_1
                * (b * c[(i << 1 as libc::c_int) as usize][1 as libc::c_int as usize]
                    + a * c[(i << 1 as libc::c_int | 1 as libc::c_int) as usize]
                        [1 as libc::c_int as usize])
                + ((a * b >> 1 as libc::c_int)
                    + -((w_1
                        * (b * c[(i << 1 as libc::c_int) as usize][1 as libc::c_int as usize]
                            + a * c[(i << 1 as libc::c_int | 1 as libc::c_int) as usize]
                                [1 as libc::c_int as usize])
                        < 0 as libc::c_int) as libc::c_int)
                    ^ -((w_1
                        * (b * c[(i << 1 as libc::c_int) as usize][1 as libc::c_int as usize]
                            + a * c[(i << 1 as libc::c_int | 1 as libc::c_int) as usize]
                                [1 as libc::c_int as usize])
                        < 0 as libc::c_int) as libc::c_int)))
                / (a * b);
            nc[(i << 1 as libc::c_int) as usize] = w_1 << 1 as libc::c_int
        } else {
            c[(i << 1 as libc::c_int) as usize][0 as libc::c_int as usize] +=
                c[(i << 1 as libc::c_int | 1 as libc::c_int) as usize][0 as libc::c_int as usize];
            c[(i << 1 as libc::c_int) as usize][1 as libc::c_int as usize] +=
                c[(i << 1 as libc::c_int | 1 as libc::c_int) as usize][1 as libc::c_int as usize];
            nc[(i << 1 as libc::c_int) as usize] += b
        }
        i += 1
    }
    /* Average offsets from pairs of orthogonal lines. */
    c[0 as libc::c_int as usize][0 as libc::c_int as usize] +=
        c[2 as libc::c_int as usize][0 as libc::c_int as usize];
    c[0 as libc::c_int as usize][1 as libc::c_int as usize] +=
        c[2 as libc::c_int as usize][1 as libc::c_int as usize];
    nc[0 as libc::c_int as usize] += nc[2 as libc::c_int as usize];
    /* If we actually found any such lines, apply the adjustment. */
    if nc[0 as libc::c_int as usize] != 0 {
        dx = (c[0 as libc::c_int as usize][0 as libc::c_int as usize]
            + ((nc[0 as libc::c_int as usize] >> 1 as libc::c_int)
                + -((c[0 as libc::c_int as usize][0 as libc::c_int as usize] < 0 as libc::c_int)
                    as libc::c_int)
                ^ -((c[0 as libc::c_int as usize][0 as libc::c_int as usize] < 0 as libc::c_int)
                    as libc::c_int)))
            / nc[0 as libc::c_int as usize];
        dy = (c[0 as libc::c_int as usize][1 as libc::c_int as usize]
            + ((nc[0 as libc::c_int as usize] >> 1 as libc::c_int)
                + -((c[0 as libc::c_int as usize][1 as libc::c_int as usize] < 0 as libc::c_int)
                    as libc::c_int)
                ^ -((c[0 as libc::c_int as usize][1 as libc::c_int as usize] < 0 as libc::c_int)
                    as libc::c_int)))
            / nc[0 as libc::c_int as usize];
        /* But only if it doesn't make things too much worse. */
        match_0 = qr_alignment_pattern_fetch(
            p.as_mut_ptr(),
            bestx + dx,
            besty + dy,
            _img,
            _width,
            _height,
        );
        dist = qr_hamming_dist(
            match_0,
            0x1f8d63f as libc::c_int as libc::c_uint,
            best_dist + 1 as libc::c_int,
        );
        if dist <= best_dist + 1 as libc::c_int {
            bestx += dx;
            besty += dy
        }
    }
    *_p.offset(0 as libc::c_int as isize) = bestx;
    *_p.offset(1 as libc::c_int as isize) = besty;
    return 0 as libc::c_int;
}
unsafe extern fn qr_hom_fit(
    mut _hom: *mut qr_hom,
    mut _ul: *mut qr_finder,
    mut _ur: *mut qr_finder,
    mut _dl: *mut qr_finder,
    mut _p: *mut qr_point,
    mut _aff: *const qr_aff,
    mut _isaac: *mut isaac_ctx,
    mut _img: *const libc::c_uchar,
    mut _width: libc::c_int,
    mut _height: libc::c_int,
) -> libc::c_int {
    let mut b: *mut qr_point = 0 as *mut qr_point;
    let mut nb: libc::c_int = 0;
    let mut cb: libc::c_int = 0;
    let mut r: *mut qr_point = 0 as *mut qr_point;
    let mut nr: libc::c_int = 0;
    let mut cr: libc::c_int = 0;
    let mut l: [qr_line; 4] = [[0; 3]; 4];
    let mut q: qr_point = [0; 2];
    let mut p: qr_point = [0; 2];
    let mut ox: libc::c_int = 0;
    let mut oy: libc::c_int = 0;
    let mut ru: libc::c_int = 0;
    let mut rv: libc::c_int = 0;
    let mut dru: libc::c_int = 0;
    let mut drv: libc::c_int = 0;
    let mut bu: libc::c_int = 0;
    let mut bv: libc::c_int = 0;
    let mut dbu: libc::c_int = 0;
    let mut dbv: libc::c_int = 0;
    let mut rx: libc::c_int = 0;
    let mut ry: libc::c_int = 0;
    let mut drxi: libc::c_int = 0;
    let mut dryi: libc::c_int = 0;
    let mut drxj: libc::c_int = 0;
    let mut dryj: libc::c_int = 0;
    let mut rdone: libc::c_int = 0;
    let mut nrempty: libc::c_int = 0;
    let mut rlastfit: libc::c_int = 0;
    let mut bx: libc::c_int = 0;
    let mut by: libc::c_int = 0;
    let mut dbxi: libc::c_int = 0;
    let mut dbyi: libc::c_int = 0;
    let mut dbxj: libc::c_int = 0;
    let mut dbyj: libc::c_int = 0;
    let mut bdone: libc::c_int = 0;
    let mut nbempty: libc::c_int = 0;
    let mut blastfit: libc::c_int = 0;
    let mut shift: libc::c_int = 0;
    let mut round: libc::c_int = 0;
    let mut version4: libc::c_int = 0;
    let mut brx: libc::c_int = 0;
    let mut bry: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    /*We attempt to correct large-scale perspective distortion by fitting lines
     to the edge of the code area.
    We could also look for an alignment pattern now, but that wouldn't work for
     version 1 codes, which have no alignment pattern.
    Even if the code is supposed to have one, there's go guarantee we'd find it
     intact.*/
    /*Fitting lines is easy for the edges on which we have two finder patterns.
    After the fit, UL is guaranteed to be on the proper side, but if either of
     the other two finder patterns aren't, something is wrong.*/
    qr_finder_ransac(_ul, _aff, _isaac, 0 as libc::c_int);
    qr_finder_ransac(_dl, _aff, _isaac, 0 as libc::c_int);
    qr_line_fit_finder_pair(
        l[0 as libc::c_int as usize].as_mut_ptr(),
        _aff,
        _ul,
        _dl,
        0 as libc::c_int,
    );
    if qr_line_eval(
        l[0 as libc::c_int as usize].as_mut_ptr(),
        (*(*_dl).c).pos[0 as libc::c_int as usize],
        (*(*_dl).c).pos[1 as libc::c_int as usize],
    ) < 0 as libc::c_int
        || qr_line_eval(
            l[0 as libc::c_int as usize].as_mut_ptr(),
            (*(*_ur).c).pos[0 as libc::c_int as usize],
            (*(*_ur).c).pos[1 as libc::c_int as usize],
        ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    qr_finder_ransac(_ul, _aff, _isaac, 2 as libc::c_int);
    qr_finder_ransac(_ur, _aff, _isaac, 2 as libc::c_int);
    qr_line_fit_finder_pair(
        l[2 as libc::c_int as usize].as_mut_ptr(),
        _aff,
        _ul,
        _ur,
        2 as libc::c_int,
    );
    if qr_line_eval(
        l[2 as libc::c_int as usize].as_mut_ptr(),
        (*(*_dl).c).pos[0 as libc::c_int as usize],
        (*(*_dl).c).pos[1 as libc::c_int as usize],
    ) < 0 as libc::c_int
        || qr_line_eval(
            l[2 as libc::c_int as usize].as_mut_ptr(),
            (*(*_ur).c).pos[0 as libc::c_int as usize],
            (*(*_ur).c).pos[1 as libc::c_int as usize],
        ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    /*The edges which only have one finder pattern are more difficult.
    We start by fitting a line to the edge of the one finder pattern we do
     have.
    This can fail due to an insufficient number of sample points, and even if
     it succeeds can be fairly inaccurate, because all of the points are
     clustered in one corner of the QR code.
    If it fails, we just use an axis-aligned line in the affine coordinate
     system.
    Then we walk along the edge of the entire code, looking for
     light:dark:light patterns perpendicular to the edge.
    Wherever we find one, we take the center of the dark portion as an
     additional sample point.
    At the end, we re-fit the line using all such sample points found.*/
    drv = (*_ur).size[1 as libc::c_int as usize] >> 1 as libc::c_int;
    qr_finder_ransac(_ur, _aff, _isaac, 1 as libc::c_int);
    if qr_line_fit_finder_edge(
        l[1 as libc::c_int as usize].as_mut_ptr(),
        _ur,
        1 as libc::c_int,
        (*_aff).res,
    ) >= 0 as libc::c_int
    {
        if qr_line_eval(
            l[1 as libc::c_int as usize].as_mut_ptr(),
            (*(*_ul).c).pos[0 as libc::c_int as usize],
            (*(*_ul).c).pos[1 as libc::c_int as usize],
        ) < 0 as libc::c_int
            || qr_line_eval(
                l[1 as libc::c_int as usize].as_mut_ptr(),
                (*(*_dl).c).pos[0 as libc::c_int as usize],
                (*(*_dl).c).pos[1 as libc::c_int as usize],
            ) < 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        /*Figure out the change in ru for a given change in rv when stepping along
        the fitted line.*/
        if qr_aff_line_step(
            _aff,
            l[1 as libc::c_int as usize].as_mut_ptr(),
            1 as libc::c_int,
            drv,
            &mut dru,
        ) < 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
    } else {
        dru = 0 as libc::c_int
    }
    ru = (*_ur).o[0 as libc::c_int as usize]
        + 3 as libc::c_int * (*_ur).size[0 as libc::c_int as usize]
        - 2 as libc::c_int * dru;
    rv = (*_ur).o[1 as libc::c_int as usize] - 2 as libc::c_int * drv;
    dbu = (*_dl).size[0 as libc::c_int as usize] >> 1 as libc::c_int;
    qr_finder_ransac(_dl, _aff, _isaac, 3 as libc::c_int);
    if qr_line_fit_finder_edge(
        l[3 as libc::c_int as usize].as_mut_ptr(),
        _dl,
        3 as libc::c_int,
        (*_aff).res,
    ) >= 0 as libc::c_int
    {
        if qr_line_eval(
            l[3 as libc::c_int as usize].as_mut_ptr(),
            (*(*_ul).c).pos[0 as libc::c_int as usize],
            (*(*_ul).c).pos[1 as libc::c_int as usize],
        ) < 0 as libc::c_int
            || qr_line_eval(
                l[3 as libc::c_int as usize].as_mut_ptr(),
                (*(*_ur).c).pos[0 as libc::c_int as usize],
                (*(*_ur).c).pos[1 as libc::c_int as usize],
            ) < 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        /*Figure out the change in bv for a given change in bu when stepping along
        the fitted line.*/
        if qr_aff_line_step(
            _aff,
            l[3 as libc::c_int as usize].as_mut_ptr(),
            0 as libc::c_int,
            dbu,
            &mut dbv,
        ) < 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
    } else {
        dbv = 0 as libc::c_int
    }
    bu = (*_dl).o[0 as libc::c_int as usize] - 2 as libc::c_int * dbu;
    bv = (*_dl).o[1 as libc::c_int as usize]
        + 3 as libc::c_int * (*_dl).size[1 as libc::c_int as usize]
        - 2 as libc::c_int * dbv;
    /* Set up the initial point lists. */
    rlastfit = (*_ur).ninliers[1 as libc::c_int as usize];
    nr = rlastfit;
    cr = nr + ((*_dl).o[1 as libc::c_int as usize] - rv + drv - 1 as libc::c_int) / drv;
    r = malloc(
        (cr as libc::c_ulong).wrapping_mul(::std::mem::size_of::<qr_point>() as libc::c_ulong),
    ) as *mut qr_point;
    i = 0 as libc::c_int;
    while i < (*_ur).ninliers[1 as libc::c_int as usize] {
        memcpy(
            (*r.offset(i as isize)).as_mut_ptr() as *mut libc::c_void,
            (*(*_ur).edge_pts[1 as libc::c_int as usize].offset(i as isize)).pos.as_mut_ptr()
                as *const libc::c_void,
            ::std::mem::size_of::<qr_point>() as libc::c_ulong,
        );
        i += 1
    }
    blastfit = (*_dl).ninliers[3 as libc::c_int as usize];
    nb = blastfit;
    cb = nb + ((*_ur).o[0 as libc::c_int as usize] - bu + dbu - 1 as libc::c_int) / dbu;
    b = malloc(
        (cb as libc::c_ulong).wrapping_mul(::std::mem::size_of::<qr_point>() as libc::c_ulong),
    ) as *mut qr_point;
    i = 0 as libc::c_int;
    while i < (*_dl).ninliers[3 as libc::c_int as usize] {
        memcpy(
            (*b.offset(i as isize)).as_mut_ptr() as *mut libc::c_void,
            (*(*_dl).edge_pts[3 as libc::c_int as usize].offset(i as isize)).pos.as_mut_ptr()
                as *const libc::c_void,
            ::std::mem::size_of::<qr_point>() as libc::c_ulong,
        );
        i += 1
    }
    /* Set up the step parameters for the affine projection. */
    ox = ((*_aff).x0 << (*_aff).res) + ((1 as libc::c_int) << (*_aff).res - 1 as libc::c_int);
    oy = ((*_aff).y0 << (*_aff).res) + ((1 as libc::c_int) << (*_aff).res - 1 as libc::c_int);
    rx = (*_aff).fwd[0 as libc::c_int as usize][0 as libc::c_int as usize] * ru
        + (*_aff).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize] * rv
        + ox;
    ry = (*_aff).fwd[1 as libc::c_int as usize][0 as libc::c_int as usize] * ru
        + (*_aff).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize] * rv
        + oy;
    drxi = (*_aff).fwd[0 as libc::c_int as usize][0 as libc::c_int as usize] * dru
        + (*_aff).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize] * drv;
    dryi = (*_aff).fwd[1 as libc::c_int as usize][0 as libc::c_int as usize] * dru
        + (*_aff).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize] * drv;
    drxj = (*_aff).fwd[0 as libc::c_int as usize][0 as libc::c_int as usize]
        * (*_ur).size[0 as libc::c_int as usize];
    dryj = (*_aff).fwd[1 as libc::c_int as usize][0 as libc::c_int as usize]
        * (*_ur).size[0 as libc::c_int as usize];
    bx = (*_aff).fwd[0 as libc::c_int as usize][0 as libc::c_int as usize] * bu
        + (*_aff).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize] * bv
        + ox;
    by = (*_aff).fwd[1 as libc::c_int as usize][0 as libc::c_int as usize] * bu
        + (*_aff).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize] * bv
        + oy;
    dbxi = (*_aff).fwd[0 as libc::c_int as usize][0 as libc::c_int as usize] * dbu
        + (*_aff).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize] * dbv;
    dbyi = (*_aff).fwd[1 as libc::c_int as usize][0 as libc::c_int as usize] * dbu
        + (*_aff).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize] * dbv;
    dbxj = (*_aff).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize]
        * (*_dl).size[1 as libc::c_int as usize];
    dbyj = (*_aff).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize]
        * (*_dl).size[1 as libc::c_int as usize];
    /* Now step along the lines, looking for new sample points. */
    nbempty = 0 as libc::c_int;
    nrempty = nbempty;
    loop {
        let mut ret: libc::c_int = 0;
        let mut x0: libc::c_int = 0;
        let mut y0: libc::c_int = 0;
        let mut x1: libc::c_int = 0;
        let mut y1: libc::c_int = 0;
        /*If we take too many steps without encountering a non-zero pixel, assume
         we have wandered off the edge and stop looking before we hit the other
         side of the quiet region.
        Otherwise, stop when the lines cross (if they do so inside the affine
         region) or come close to crossing (outside the affine region).
        TODO: We don't have any way of detecting when we've wandered into the
         code interior; we could stop if the outside sample ever shows up dark,
         but this could happen because of noise in the quiet region, too.*/
        rdone = (rv
            >= bv
                + (((*_dl).o[1 as libc::c_int as usize] + bv >> 1 as libc::c_int) - bv
                    & -((((*_dl).o[1 as libc::c_int as usize] + bv >> 1 as libc::c_int) < bv)
                        as libc::c_int))
            || nrempty > 14 as libc::c_int) as libc::c_int;
        bdone = (bu
            >= ru
                + (((*_ur).o[0 as libc::c_int as usize] + ru >> 1 as libc::c_int) - ru
                    & -((((*_ur).o[0 as libc::c_int as usize] + ru >> 1 as libc::c_int) < ru)
                        as libc::c_int))
            || nbempty > 14 as libc::c_int) as libc::c_int;
        if rdone == 0 && (bdone != 0 || rv < bu) {
            x0 = rx + drxj >> (*_aff).res + 2 as libc::c_int;
            y0 = ry + dryj >> (*_aff).res + 2 as libc::c_int;
            x1 = rx - drxj >> (*_aff).res + 2 as libc::c_int;
            y1 = ry - dryj >> (*_aff).res + 2 as libc::c_int;
            if nr >= cr {
                cr = cr << 1 as libc::c_int | 1 as libc::c_int;
                r = realloc(
                    r as *mut libc::c_void,
                    (cr as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<qr_point>() as libc::c_ulong),
                ) as *mut qr_point
            }
            ret = qr_finder_quick_crossing_check(
                _img,
                _width,
                _height,
                x0,
                y0,
                x1,
                y1,
                1 as libc::c_int,
            );
            if ret == 0 {
                ret = qr_finder_locate_crossing(
                    _img,
                    _width,
                    _height,
                    x0,
                    y0,
                    x1,
                    y1,
                    1 as libc::c_int,
                    (*r.offset(nr as isize)).as_mut_ptr(),
                )
            }
            if ret >= 0 as libc::c_int {
                if ret == 0 {
                    qr_aff_unproject(
                        q.as_mut_ptr(),
                        _aff,
                        (*r.offset(nr as isize))[0 as libc::c_int as usize],
                        (*r.offset(nr as isize))[1 as libc::c_int as usize],
                    );
                    /*Move the current point halfway towards the crossing.
                    We don't move the whole way to give us some robustness to noise.*/
                    ru = ru + q[0 as libc::c_int as usize] >> 1 as libc::c_int;
                    /* But ensure that rv monotonically increases. */
                    if q[1 as libc::c_int as usize] + drv > rv {
                        rv = rv + q[1 as libc::c_int as usize] >> 1 as libc::c_int
                    }
                    rx = (*_aff).fwd[0 as libc::c_int as usize][0 as libc::c_int as usize] * ru
                        + (*_aff).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize] * rv
                        + ox;
                    ry = (*_aff).fwd[1 as libc::c_int as usize][0 as libc::c_int as usize] * ru
                        + (*_aff).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize] * rv
                        + oy;
                    nr += 1;
                    /* Re-fit the line to update the step direction periodically. */
                    if nr
                        > 1 as libc::c_int
                            - (1 as libc::c_int - (rlastfit + (rlastfit >> 2 as libc::c_int))
                                & -((rlastfit + (rlastfit >> 2 as libc::c_int) > 1 as libc::c_int)
                                    as libc::c_int))
                    {
                        qr_line_fit_points(
                            l[1 as libc::c_int as usize].as_mut_ptr(),
                            r,
                            nr,
                            (*_aff).res,
                        );
                        if qr_aff_line_step(
                            _aff,
                            l[1 as libc::c_int as usize].as_mut_ptr(),
                            1 as libc::c_int,
                            drv,
                            &mut dru,
                        ) >= 0 as libc::c_int
                        {
                            drxi = (*_aff).fwd[0 as libc::c_int as usize]
                                [0 as libc::c_int as usize]
                                * dru
                                + (*_aff).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize]
                                    * drv;
                            dryi = (*_aff).fwd[1 as libc::c_int as usize][0 as libc::c_int as usize]
                                * dru
                                + (*_aff).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize]
                                    * drv
                        }
                        rlastfit = nr
                    }
                }
                nrempty = 0 as libc::c_int
            } else {
                nrempty += 1
            }
            ru += dru;
            /* Our final defense: if we overflow, stop. */
            if rv + drv > rv {
                rv += drv
            } else {
                nrempty = 2147483647 as libc::c_int
            }
            rx += drxi;
            ry += dryi
        } else {
            if !(bdone == 0) {
                break;
            }
            x0 = bx + dbxj >> (*_aff).res + 2 as libc::c_int;
            y0 = by + dbyj >> (*_aff).res + 2 as libc::c_int;
            x1 = bx - dbxj >> (*_aff).res + 2 as libc::c_int;
            y1 = by - dbyj >> (*_aff).res + 2 as libc::c_int;
            if nb >= cb {
                cb = cb << 1 as libc::c_int | 1 as libc::c_int;
                b = realloc(
                    b as *mut libc::c_void,
                    (cb as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<qr_point>() as libc::c_ulong),
                ) as *mut qr_point
            }
            ret = qr_finder_quick_crossing_check(
                _img,
                _width,
                _height,
                x0,
                y0,
                x1,
                y1,
                1 as libc::c_int,
            );
            if ret == 0 {
                ret = qr_finder_locate_crossing(
                    _img,
                    _width,
                    _height,
                    x0,
                    y0,
                    x1,
                    y1,
                    1 as libc::c_int,
                    (*b.offset(nb as isize)).as_mut_ptr(),
                )
            }
            if ret >= 0 as libc::c_int {
                if ret == 0 {
                    qr_aff_unproject(
                        q.as_mut_ptr(),
                        _aff,
                        (*b.offset(nb as isize))[0 as libc::c_int as usize],
                        (*b.offset(nb as isize))[1 as libc::c_int as usize],
                    );
                    /*Move the current point halfway towards the crossing.
                    We don't move the whole way to give us some robustness to noise.*/
                    /* But ensure that bu monotonically increases. */
                    if q[0 as libc::c_int as usize] + dbu > bu {
                        bu = bu + q[0 as libc::c_int as usize] >> 1 as libc::c_int
                    }
                    bv = bv + q[1 as libc::c_int as usize] >> 1 as libc::c_int;
                    bx = (*_aff).fwd[0 as libc::c_int as usize][0 as libc::c_int as usize] * bu
                        + (*_aff).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize] * bv
                        + ox;
                    by = (*_aff).fwd[1 as libc::c_int as usize][0 as libc::c_int as usize] * bu
                        + (*_aff).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize] * bv
                        + oy;
                    nb += 1;
                    /* Re-fit the line to update the step direction periodically. */
                    if nb
                        > 1 as libc::c_int
                            - (1 as libc::c_int - (blastfit + (blastfit >> 2 as libc::c_int))
                                & -((blastfit + (blastfit >> 2 as libc::c_int) > 1 as libc::c_int)
                                    as libc::c_int))
                    {
                        qr_line_fit_points(
                            l[3 as libc::c_int as usize].as_mut_ptr(),
                            b,
                            nb,
                            (*_aff).res,
                        );
                        if qr_aff_line_step(
                            _aff,
                            l[3 as libc::c_int as usize].as_mut_ptr(),
                            0 as libc::c_int,
                            dbu,
                            &mut dbv,
                        ) >= 0 as libc::c_int
                        {
                            dbxi = (*_aff).fwd[0 as libc::c_int as usize]
                                [0 as libc::c_int as usize]
                                * dbu
                                + (*_aff).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize]
                                    * dbv;
                            dbyi = (*_aff).fwd[1 as libc::c_int as usize][0 as libc::c_int as usize]
                                * dbu
                                + (*_aff).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize]
                                    * dbv
                        }
                        blastfit = nb
                    }
                }
                nbempty = 0 as libc::c_int
            } else {
                nbempty += 1
            }
            /* Our final defense: if we overflow, stop. */
            if bu + dbu > bu {
                bu += dbu
            } else {
                nbempty = 2147483647 as libc::c_int
            }
            bv += dbv;
            bx += dbxi;
            by += dbyi
        }
    }
    /*Fit the new lines.
    If we _still_ don't have enough sample points, then just use an
     axis-aligned line from the affine coordinate system (e.g., one parallel
     to the opposite edge in the image).*/
    if nr > 1 as libc::c_int {
        qr_line_fit_points(l[1 as libc::c_int as usize].as_mut_ptr(), r, nr, (*_aff).res);
    } else {
        qr_aff_project(
            p.as_mut_ptr(),
            _aff,
            (*_ur).o[0 as libc::c_int as usize]
                + 3 as libc::c_int * (*_ur).size[0 as libc::c_int as usize],
            (*_ur).o[1 as libc::c_int as usize],
        );
        shift = 0 as libc::c_int
            - (0 as libc::c_int
                - (qr_ilog(
                    (abs((*_aff).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize])
                        - (abs((*_aff).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize])
                            - abs(
                                (*_aff).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize]
                            )
                            & -((abs(
                                (*_aff).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize]
                            ) > abs(
                                (*_aff).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize]
                            )) as libc::c_int))) as libc::c_uint,
                ) - ((*_aff).res + 1 as libc::c_int >> 1 as libc::c_int))
                & -((qr_ilog(
                    (abs((*_aff).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize])
                        - (abs((*_aff).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize])
                            - abs(
                                (*_aff).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize]
                            )
                            & -((abs(
                                (*_aff).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize]
                            ) > abs(
                                (*_aff).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize]
                            )) as libc::c_int))) as libc::c_uint,
                ) - ((*_aff).res + 1 as libc::c_int >> 1 as libc::c_int)
                    > 0 as libc::c_int) as libc::c_int));
        round = (1 as libc::c_int) << shift >> 1 as libc::c_int;
        l[1 as libc::c_int as usize][0 as libc::c_int as usize] =
            (*_aff).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize] + round >> shift;
        l[1 as libc::c_int as usize][1 as libc::c_int as usize] =
            -(*_aff).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize] + round >> shift;
        l[1 as libc::c_int as usize][2 as libc::c_int as usize] = -(l[1 as libc::c_int as usize]
            [0 as libc::c_int as usize]
            * p[0 as libc::c_int as usize]
            + l[1 as libc::c_int as usize][1 as libc::c_int as usize]
                * p[1 as libc::c_int as usize])
    }
    free(r as *mut libc::c_void);
    if nb > 1 as libc::c_int {
        qr_line_fit_points(l[3 as libc::c_int as usize].as_mut_ptr(), b, nb, (*_aff).res);
    } else {
        qr_aff_project(
            p.as_mut_ptr(),
            _aff,
            (*_dl).o[0 as libc::c_int as usize],
            (*_dl).o[1 as libc::c_int as usize]
                + 3 as libc::c_int * (*_dl).size[1 as libc::c_int as usize],
        );
        shift = 0 as libc::c_int
            - (0 as libc::c_int
                - (qr_ilog(
                    (abs((*_aff).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize])
                        - (abs((*_aff).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize])
                            - abs(
                                (*_aff).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize]
                            )
                            & -((abs(
                                (*_aff).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize]
                            ) > abs(
                                (*_aff).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize]
                            )) as libc::c_int))) as libc::c_uint,
                ) - ((*_aff).res + 1 as libc::c_int >> 1 as libc::c_int))
                & -((qr_ilog(
                    (abs((*_aff).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize])
                        - (abs((*_aff).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize])
                            - abs(
                                (*_aff).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize]
                            )
                            & -((abs(
                                (*_aff).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize]
                            ) > abs(
                                (*_aff).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize]
                            )) as libc::c_int))) as libc::c_uint,
                ) - ((*_aff).res + 1 as libc::c_int >> 1 as libc::c_int)
                    > 0 as libc::c_int) as libc::c_int));
        round = (1 as libc::c_int) << shift >> 1 as libc::c_int;
        l[3 as libc::c_int as usize][0 as libc::c_int as usize] =
            (*_aff).fwd[1 as libc::c_int as usize][0 as libc::c_int as usize] + round >> shift;
        l[3 as libc::c_int as usize][1 as libc::c_int as usize] =
            -(*_aff).fwd[0 as libc::c_int as usize][0 as libc::c_int as usize] + round >> shift;
        l[3 as libc::c_int as usize][2 as libc::c_int as usize] = -(l[1 as libc::c_int as usize]
            [0 as libc::c_int as usize]
            * p[0 as libc::c_int as usize]
            + l[1 as libc::c_int as usize][1 as libc::c_int as usize]
                * p[1 as libc::c_int as usize])
    }
    free(b as *mut libc::c_void);
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if qr_line_isect(
            (*_p.offset(i as isize)).as_mut_ptr(),
            l[(i & 1 as libc::c_int) as usize].as_mut_ptr() as *const libc::c_int,
            l[(2 as libc::c_int + (i >> 1 as libc::c_int)) as usize].as_mut_ptr()
                as *const libc::c_int,
        ) < 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        /*It's plausible for points to be somewhat outside the image, but too far
        and too much of the pattern will be gone for it to be decodable.*/
        if (*_p.offset(i as isize))[0 as libc::c_int as usize] < -_width << 2 as libc::c_int
            || (*_p.offset(i as isize))[0 as libc::c_int as usize]
                >= _width << 2 as libc::c_int + 1 as libc::c_int
            || (*_p.offset(i as isize))[1 as libc::c_int as usize] < -_height << 2 as libc::c_int
            || (*_p.offset(i as isize))[1 as libc::c_int as usize]
                >= _height << 2 as libc::c_int + 1 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        i += 1
    }
    /* By default, use the edge intersection point for the bottom-right corner. */
    brx = (*_p.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize];
    bry = (*_p.offset(3 as libc::c_int as isize))[1 as libc::c_int as usize];
    /*However, if our average version estimate is greater than 1, NOW we try to
     search for an alignment pattern.
    We get a much better success rate by doing this after our initial attempt
     to promote the transform to a homography than before.
    You might also think it would be more reliable to use the interior finder
     pattern edges, since the outer ones may be obscured or damaged, and it
     would save us a reprojection below, since they would form a nice square
     with the location of the alignment pattern, but this turns out to be a bad
     idea.
    Non-linear distortion is usually maximal on the outside edge, and thus
     estimating the grid position from points on the interior means we might
     get mis-aligned by the time we reach the edge.*/
    version4 = (*_ul).eversion[0 as libc::c_int as usize]
        + (*_ul).eversion[1 as libc::c_int as usize]
        + (*_ur).eversion[0 as libc::c_int as usize]
        + (*_dl).eversion[1 as libc::c_int as usize];
    if version4 > 4 as libc::c_int {
        let mut cell: qr_hom_cell = qr_hom_cell {
            fwd: [[0; 3]; 3],
            x0: 0,
            y0: 0,
            u0: 0,
            v0: 0,
        };
        let mut p3: qr_point = [0; 2];
        let mut dim: libc::c_int = 0;
        dim = 17 as libc::c_int + version4;
        qr_hom_cell_init(
            &mut cell,
            0 as libc::c_int,
            0 as libc::c_int,
            dim - 1 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            dim - 1 as libc::c_int,
            dim - 1 as libc::c_int,
            dim - 1 as libc::c_int,
            (*_p.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize],
            (*_p.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize],
            (*_p.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize],
            (*_p.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize],
            (*_p.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize],
            (*_p.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize],
            (*_p.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize],
            (*_p.offset(3 as libc::c_int as isize))[1 as libc::c_int as usize],
        );
        if qr_alignment_pattern_search(
            p3.as_mut_ptr(),
            &mut cell,
            dim - 7 as libc::c_int,
            dim - 7 as libc::c_int,
            4 as libc::c_int,
            _img,
            _width,
            _height,
        ) >= 0 as libc::c_int
        {
            let mut w: libc::c_longlong = 0;
            let mut mask: libc::c_longlong = 0;
            let mut c21: libc::c_int = 0;
            let mut dx21: libc::c_int = 0;
            let mut dy21: libc::c_int = 0;
            /*There's no real need to update the bounding box corner, and in fact we
             actively perform worse if we do.
            Clearly it was good enough for us to find this alignment pattern, so
             it should be good enough to use for grid initialization.
            The point of doing the search was to get more accurate version
             estimates and a better chance of decoding the version and format info.
            This is particularly important for small versions that have no encoded
             version info, since any mismatch in version renders the code
             undecodable.*/
            /*We do, however, need four points in a square to initialize our
            homography, so project the point from the alignment center to the
            corner of the code area.*/
            c21 = (*_p.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize]
                * (*_p.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
                - (*_p.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize]
                    * (*_p.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize];
            dx21 = (*_p.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize]
                - (*_p.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize];
            dy21 = (*_p.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize]
                - (*_p.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize];
            w = (dim - 7 as libc::c_int) as libc::c_longlong * c21 as libc::c_longlong
                + ((dim - 13 as libc::c_int) as libc::c_longlong
                    * ((*_p.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] * dy21
                        - (*_p.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] * dx21)
                        as libc::c_longlong
                    + (6 as libc::c_int as libc::c_longlong
                        * (p3[0 as libc::c_int as usize] * dy21
                            - p3[1 as libc::c_int as usize] * dx21)
                            as libc::c_longlong
                        + 0 as libc::c_int as libc::c_longlong));
            /* The projection failed: invalid geometry. */
            if w == 0 as libc::c_int as libc::c_longlong {
                return -(1 as libc::c_int);
            }
            mask = -((w < 0 as libc::c_int as libc::c_longlong) as libc::c_int) as libc::c_longlong;
            w = w + mask ^ mask;
            brx = (((((dim - 7 as libc::c_int)
                * (*_p.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize])
                as libc::c_longlong
                * (p3[0 as libc::c_int as usize] * dy21) as libc::c_longlong
                + (((dim - 13 as libc::c_int) * p3[0 as libc::c_int as usize])
                    as libc::c_longlong
                    * (c21
                        - (*_p.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] * dx21)
                        as libc::c_longlong
                    + ((6 as libc::c_int
                        * (*_p.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize])
                        as libc::c_longlong
                        * (c21 - p3[1 as libc::c_int as usize] * dx21) as libc::c_longlong
                        + 0 as libc::c_int as libc::c_longlong))
                + mask
                ^ mask)
                + ((w >> 1 as libc::c_int)
                    + -((((dim - 7 as libc::c_int)
                        * (*_p.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize])
                        as libc::c_longlong
                        * (p3[0 as libc::c_int as usize] * dy21) as libc::c_longlong
                        + (((dim - 13 as libc::c_int) * p3[0 as libc::c_int as usize])
                            as libc::c_longlong
                            * (c21
                                - (*_p.offset(0 as libc::c_int as isize))
                                    [1 as libc::c_int as usize]
                                    * dx21) as libc::c_longlong
                            + ((6 as libc::c_int
                                * (*_p.offset(0 as libc::c_int as isize))
                                    [0 as libc::c_int as usize])
                                as libc::c_longlong
                                * (c21 - p3[1 as libc::c_int as usize] * dx21)
                                    as libc::c_longlong
                                + 0 as libc::c_int as libc::c_longlong))
                        + mask
                        ^ mask
                        < 0 as libc::c_int as libc::c_longlong)
                        as libc::c_int) as libc::c_longlong
                    ^ -((((dim - 7 as libc::c_int)
                        * (*_p.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize])
                        as libc::c_longlong
                        * (p3[0 as libc::c_int as usize] * dy21) as libc::c_longlong
                        + (((dim - 13 as libc::c_int) * p3[0 as libc::c_int as usize])
                            as libc::c_longlong
                            * (c21
                                - (*_p.offset(0 as libc::c_int as isize))
                                    [1 as libc::c_int as usize]
                                    * dx21) as libc::c_longlong
                            + ((6 as libc::c_int
                                * (*_p.offset(0 as libc::c_int as isize))
                                    [0 as libc::c_int as usize])
                                as libc::c_longlong
                                * (c21 - p3[1 as libc::c_int as usize] * dx21)
                                    as libc::c_longlong
                                + 0 as libc::c_int as libc::c_longlong))
                        + mask
                        ^ mask
                        < 0 as libc::c_int as libc::c_longlong)
                        as libc::c_int) as libc::c_longlong))
                / w) as libc::c_int;
            bry = (((((dim - 7 as libc::c_int)
                * (*_p.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize])
                as libc::c_longlong
                * (-p3[1 as libc::c_int as usize] * dx21) as libc::c_longlong
                + (((dim - 13 as libc::c_int) * p3[1 as libc::c_int as usize])
                    as libc::c_longlong
                    * (c21
                        + (*_p.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] * dy21)
                        as libc::c_longlong
                    + ((6 as libc::c_int
                        * (*_p.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize])
                        as libc::c_longlong
                        * (c21 + p3[0 as libc::c_int as usize] * dy21) as libc::c_longlong
                        + 0 as libc::c_int as libc::c_longlong))
                + mask
                ^ mask)
                + ((w >> 1 as libc::c_int)
                    + -((((dim - 7 as libc::c_int)
                        * (*_p.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize])
                        as libc::c_longlong
                        * (-p3[1 as libc::c_int as usize] * dx21) as libc::c_longlong
                        + (((dim - 13 as libc::c_int) * p3[1 as libc::c_int as usize])
                            as libc::c_longlong
                            * (c21
                                + (*_p.offset(0 as libc::c_int as isize))
                                    [0 as libc::c_int as usize]
                                    * dy21) as libc::c_longlong
                            + ((6 as libc::c_int
                                * (*_p.offset(0 as libc::c_int as isize))
                                    [1 as libc::c_int as usize])
                                as libc::c_longlong
                                * (c21 + p3[0 as libc::c_int as usize] * dy21)
                                    as libc::c_longlong
                                + 0 as libc::c_int as libc::c_longlong))
                        + mask
                        ^ mask
                        < 0 as libc::c_int as libc::c_longlong)
                        as libc::c_int) as libc::c_longlong
                    ^ -((((dim - 7 as libc::c_int)
                        * (*_p.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize])
                        as libc::c_longlong
                        * (-p3[1 as libc::c_int as usize] * dx21) as libc::c_longlong
                        + (((dim - 13 as libc::c_int) * p3[1 as libc::c_int as usize])
                            as libc::c_longlong
                            * (c21
                                + (*_p.offset(0 as libc::c_int as isize))
                                    [0 as libc::c_int as usize]
                                    * dy21) as libc::c_longlong
                            + ((6 as libc::c_int
                                * (*_p.offset(0 as libc::c_int as isize))
                                    [1 as libc::c_int as usize])
                                as libc::c_longlong
                                * (c21 + p3[0 as libc::c_int as usize] * dy21)
                                    as libc::c_longlong
                                + 0 as libc::c_int as libc::c_longlong))
                        + mask
                        ^ mask
                        < 0 as libc::c_int as libc::c_longlong)
                        as libc::c_int) as libc::c_longlong))
                / w) as libc::c_int
        }
    }
    /* Now we have four points that map to a square: initialize the projection. */
    qr_hom_init(
        _hom,
        (*_p.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize],
        (*_p.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize],
        (*_p.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize],
        (*_p.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize],
        (*_p.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize],
        (*_p.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize],
        brx,
        bry,
        14 as libc::c_int,
    );
    return 0 as libc::c_int;
}
/*The BCH(18,6,3) codes are only used for version information, which must lie
between 7 and 40 (inclusive).*/
static mut BCH18_6_CODES: [libc::c_uint; 34] = [
    0x7c94 as libc::c_int as libc::c_uint,
    0x85bc as libc::c_int as libc::c_uint,
    0x9a99 as libc::c_int as libc::c_uint,
    0xa4d3 as libc::c_int as libc::c_uint,
    0xbbf6 as libc::c_int as libc::c_uint,
    0xc762 as libc::c_int as libc::c_uint,
    0xd847 as libc::c_int as libc::c_uint,
    0xe60d as libc::c_int as libc::c_uint,
    0xf928 as libc::c_int as libc::c_uint,
    0x10b78 as libc::c_int as libc::c_uint,
    0x1145d as libc::c_int as libc::c_uint,
    0x12a17 as libc::c_int as libc::c_uint,
    0x13532 as libc::c_int as libc::c_uint,
    0x149a6 as libc::c_int as libc::c_uint,
    0x15683 as libc::c_int as libc::c_uint,
    0x168c9 as libc::c_int as libc::c_uint,
    0x177ec as libc::c_int as libc::c_uint,
    0x18ec4 as libc::c_int as libc::c_uint,
    0x191e1 as libc::c_int as libc::c_uint,
    0x1afab as libc::c_int as libc::c_uint,
    0x1b08e as libc::c_int as libc::c_uint,
    0x1cc1a as libc::c_int as libc::c_uint,
    0x1d33f as libc::c_int as libc::c_uint,
    0x1ed75 as libc::c_int as libc::c_uint,
    0x1f250 as libc::c_int as libc::c_uint,
    0x209d5 as libc::c_int as libc::c_uint,
    0x216f0 as libc::c_int as libc::c_uint,
    0x228ba as libc::c_int as libc::c_uint,
    0x2379f as libc::c_int as libc::c_uint,
    0x24b0b as libc::c_int as libc::c_uint,
    0x2542e as libc::c_int as libc::c_uint,
    0x26a64 as libc::c_int as libc::c_uint,
    0x27541 as libc::c_int as libc::c_uint,
    0x28c69 as libc::c_int as libc::c_uint,
];
/*Corrects a BCH(18,6,3) code word.
_y: Contains the code word to be checked on input, and the corrected value on
     output.
Return: The number of errors.
        If more than 3 errors are detected, returns a negative value and
         performs no correction.*/
unsafe extern fn bch18_6_correct(mut _y: *mut libc::c_uint) -> libc::c_int {
    let mut x: libc::c_uint = 0;
    let mut y: libc::c_uint = 0;
    let mut nerrs: libc::c_int = 0;
    y = *_y;
    /* Check the easy case first: see if the data bits were uncorrupted. */
    x = y >> 12 as libc::c_int;
    if x >= 7 as libc::c_int as libc::c_uint && x <= 40 as libc::c_int as libc::c_uint {
        nerrs = qr_hamming_dist(
            y,
            BCH18_6_CODES[x.wrapping_sub(7 as libc::c_int as libc::c_uint) as usize],
            4 as libc::c_int,
        );
        if nerrs < 4 as libc::c_int {
            *_y = BCH18_6_CODES[x.wrapping_sub(7 as libc::c_int as libc::c_uint) as usize];
            return nerrs;
        }
    }
    /* Exhaustive search is faster than field operations in GF(19). */
    x = 0 as libc::c_int as libc::c_uint;
    while x < 34 as libc::c_int as libc::c_uint {
        if x.wrapping_add(7 as libc::c_int as libc::c_uint) != y >> 12 as libc::c_int {
            nerrs = qr_hamming_dist(y, BCH18_6_CODES[x as usize], 4 as libc::c_int);
            if nerrs < 4 as libc::c_int {
                *_y = BCH18_6_CODES[x as usize];
                return nerrs;
            }
        }
        x = x.wrapping_add(1)
    }
    return -(1 as libc::c_int);
}
/* Reads the version bits near a finder module and decodes the version number. */
unsafe extern fn qr_finder_version_decode(
    mut _f: *mut qr_finder,
    mut _hom: *const qr_hom,
    mut _img: *const libc::c_uchar,
    mut _width: libc::c_int,
    mut _height: libc::c_int,
    mut _dir: libc::c_int,
) -> libc::c_int {
    let mut q: qr_point = [0; 2];
    let mut v: libc::c_uint = 0;
    let mut x0: libc::c_int = 0;
    let mut y0: libc::c_int = 0;
    let mut w0: libc::c_int = 0;
    let mut dxi: libc::c_int = 0;
    let mut dyi: libc::c_int = 0;
    let mut dwi: libc::c_int = 0;
    let mut dxj: libc::c_int = 0;
    let mut dyj: libc::c_int = 0;
    let mut dwj: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    v = 0 as libc::c_int as libc::c_uint;
    q[_dir as usize] = (*_f).o[_dir as usize] - 7 as libc::c_int * (*_f).size[_dir as usize];
    q[(1 as libc::c_int - _dir) as usize] = (*_f).o[(1 as libc::c_int - _dir) as usize]
        - 3 as libc::c_int * (*_f).size[(1 as libc::c_int - _dir) as usize];
    x0 = (*_hom).fwd[0 as libc::c_int as usize][0 as libc::c_int as usize]
        * q[0 as libc::c_int as usize]
        + (*_hom).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize]
            * q[1 as libc::c_int as usize];
    y0 = (*_hom).fwd[1 as libc::c_int as usize][0 as libc::c_int as usize]
        * q[0 as libc::c_int as usize]
        + (*_hom).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize]
            * q[1 as libc::c_int as usize];
    w0 = (*_hom).fwd[2 as libc::c_int as usize][0 as libc::c_int as usize]
        * q[0 as libc::c_int as usize]
        + (*_hom).fwd[2 as libc::c_int as usize][1 as libc::c_int as usize]
            * q[1 as libc::c_int as usize]
        + (*_hom).fwd22;
    dxi = (*_hom).fwd[0 as libc::c_int as usize][(1 as libc::c_int - _dir) as usize]
        * (*_f).size[(1 as libc::c_int - _dir) as usize];
    dyi = (*_hom).fwd[1 as libc::c_int as usize][(1 as libc::c_int - _dir) as usize]
        * (*_f).size[(1 as libc::c_int - _dir) as usize];
    dwi = (*_hom).fwd[2 as libc::c_int as usize][(1 as libc::c_int - _dir) as usize]
        * (*_f).size[(1 as libc::c_int - _dir) as usize];
    dxj = (*_hom).fwd[0 as libc::c_int as usize][_dir as usize] * (*_f).size[_dir as usize];
    dyj = (*_hom).fwd[1 as libc::c_int as usize][_dir as usize] * (*_f).size[_dir as usize];
    dwj = (*_hom).fwd[2 as libc::c_int as usize][_dir as usize] * (*_f).size[_dir as usize];
    i = 0 as libc::c_int;
    k = i;
    while i < 6 as libc::c_int {
        let mut x: libc::c_int = 0;
        let mut y: libc::c_int = 0;
        let mut w: libc::c_int = 0;
        x = x0;
        y = y0;
        w = w0;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            let mut p: qr_point = [0; 2];
            qr_hom_fproject(p.as_mut_ptr(), _hom, x, y, w);
            v |= (qr_img_get_bit(
                _img,
                _width,
                _height,
                p[0 as libc::c_int as usize],
                p[1 as libc::c_int as usize],
            ) << k) as libc::c_uint;
            x += dxj;
            y += dyj;
            w += dwj;
            j += 1;
            k += 1
        }
        x0 += dxi;
        y0 += dyi;
        w0 += dwi;
        i += 1
    }
    ret = bch18_6_correct(&mut v);
    /*TODO: I seem to have an image with the version bits in a different order
     (the transpose of the standard order).
    Even if I change the order here so I can parse the version on this image,
     I can't decode the rest of the code.
    If this is really needed, we should just re-order the bits.*/
    return if ret >= 0 as libc::c_int {
        (v >> 12 as libc::c_int) as libc::c_int
    } else {
        ret
    };
}
/* Reads the format info bits near the finder modules and decodes them. */
unsafe extern fn qr_finder_fmt_info_decode(
    mut _ul: *mut qr_finder,
    mut _ur: *mut qr_finder,
    mut _dl: *mut qr_finder,
    mut _hom: *const qr_hom,
    mut _img: *const libc::c_uchar,
    mut _width: libc::c_int,
    mut _height: libc::c_int,
) -> libc::c_int {
    let mut p: qr_point = [0; 2];
    let mut lo: [libc::c_uint; 2] = [0; 2];
    let mut hi: [libc::c_uint; 2] = [0; 2];
    let mut u: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut dx: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    let mut dw: libc::c_int = 0;
    let mut fmt_info: [libc::c_int; 4] = [0; 4];
    let mut count: [libc::c_int; 4] = [0; 4];
    let mut nerrs: [libc::c_int; 4] = [0; 4];
    let mut nfmt_info: libc::c_int = 0;
    let mut besti: libc::c_int = 0;
    let mut imax: libc::c_int = 0;
    let mut di: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    /* Read the bits around the UL corner. */
    lo[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uint;
    u = (*_ul).o[0 as libc::c_int as usize]
        + 5 as libc::c_int * (*_ul).size[0 as libc::c_int as usize];
    v = (*_ul).o[1 as libc::c_int as usize]
        - 3 as libc::c_int * (*_ul).size[1 as libc::c_int as usize];
    x = (*_hom).fwd[0 as libc::c_int as usize][0 as libc::c_int as usize] * u
        + (*_hom).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize] * v;
    y = (*_hom).fwd[1 as libc::c_int as usize][0 as libc::c_int as usize] * u
        + (*_hom).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize] * v;
    w = (*_hom).fwd[2 as libc::c_int as usize][0 as libc::c_int as usize] * u
        + (*_hom).fwd[2 as libc::c_int as usize][1 as libc::c_int as usize] * v
        + (*_hom).fwd22;
    dx = (*_hom).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize]
        * (*_ul).size[1 as libc::c_int as usize];
    dy = (*_hom).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize]
        * (*_ul).size[1 as libc::c_int as usize];
    dw = (*_hom).fwd[2 as libc::c_int as usize][1 as libc::c_int as usize]
        * (*_ul).size[1 as libc::c_int as usize];
    i = 0 as libc::c_int;
    k = i;
    loop
    /* Skip the timing pattern row. */
    {
        if i != 6 as libc::c_int {
            qr_hom_fproject(p.as_mut_ptr(), _hom, x, y, w);
            let fresh10 = k;
            k = k + 1;
            lo[0 as libc::c_int as usize] |= (qr_img_get_bit(
                _img,
                _width,
                _height,
                p[0 as libc::c_int as usize],
                p[1 as libc::c_int as usize],
            ) << fresh10) as libc::c_uint;
            /*Don't advance q in the last iteration... we'll start the next loop from
            the current position.*/
            if i >= 8 as libc::c_int {
                break;
            }
        }
        x += dx;
        y += dy;
        w += dw;
        i += 1
    }
    hi[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uint;
    dx = -(*_hom).fwd[0 as libc::c_int as usize][0 as libc::c_int as usize]
        * (*_ul).size[0 as libc::c_int as usize];
    dy = -(*_hom).fwd[1 as libc::c_int as usize][0 as libc::c_int as usize]
        * (*_ul).size[0 as libc::c_int as usize];
    dw = -(*_hom).fwd[2 as libc::c_int as usize][0 as libc::c_int as usize]
        * (*_ul).size[0 as libc::c_int as usize];
    loop {
        let fresh11 = i;
        i = i - 1;
        if !(fresh11 > 0 as libc::c_int) {
            break;
        }
        x += dx;
        y += dy;
        w += dw;
        /* Skip the timing pattern column. */
        if i != 6 as libc::c_int {
            qr_hom_fproject(p.as_mut_ptr(), _hom, x, y, w);
            let fresh12 = k;
            k = k + 1;
            hi[0 as libc::c_int as usize] |= (qr_img_get_bit(
                _img,
                _width,
                _height,
                p[0 as libc::c_int as usize],
                p[1 as libc::c_int as usize],
            ) << fresh12) as libc::c_uint
        }
    }
    /* Read the bits next to the UR corner. */
    lo[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uint;
    u = (*_ur).o[0 as libc::c_int as usize]
        + 3 as libc::c_int * (*_ur).size[0 as libc::c_int as usize];
    v = (*_ur).o[1 as libc::c_int as usize]
        + 5 as libc::c_int * (*_ur).size[1 as libc::c_int as usize];
    x = (*_hom).fwd[0 as libc::c_int as usize][0 as libc::c_int as usize] * u
        + (*_hom).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize] * v;
    y = (*_hom).fwd[1 as libc::c_int as usize][0 as libc::c_int as usize] * u
        + (*_hom).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize] * v;
    w = (*_hom).fwd[2 as libc::c_int as usize][0 as libc::c_int as usize] * u
        + (*_hom).fwd[2 as libc::c_int as usize][1 as libc::c_int as usize] * v
        + (*_hom).fwd22;
    dx = -(*_hom).fwd[0 as libc::c_int as usize][0 as libc::c_int as usize]
        * (*_ur).size[0 as libc::c_int as usize];
    dy = -(*_hom).fwd[1 as libc::c_int as usize][0 as libc::c_int as usize]
        * (*_ur).size[0 as libc::c_int as usize];
    dw = -(*_hom).fwd[2 as libc::c_int as usize][0 as libc::c_int as usize]
        * (*_ur).size[0 as libc::c_int as usize];
    k = 0 as libc::c_int;
    while k < 8 as libc::c_int {
        qr_hom_fproject(p.as_mut_ptr(), _hom, x, y, w);
        lo[1 as libc::c_int as usize] |= (qr_img_get_bit(
            _img,
            _width,
            _height,
            p[0 as libc::c_int as usize],
            p[1 as libc::c_int as usize],
        ) << k) as libc::c_uint;
        x += dx;
        y += dy;
        w += dw;
        k += 1
    }
    /* Read the bits next to the DL corner. */
    hi[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uint;
    u = (*_dl).o[0 as libc::c_int as usize]
        + 5 as libc::c_int * (*_dl).size[0 as libc::c_int as usize];
    v = (*_dl).o[1 as libc::c_int as usize]
        - 3 as libc::c_int * (*_dl).size[1 as libc::c_int as usize];
    x = (*_hom).fwd[0 as libc::c_int as usize][0 as libc::c_int as usize] * u
        + (*_hom).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize] * v;
    y = (*_hom).fwd[1 as libc::c_int as usize][0 as libc::c_int as usize] * u
        + (*_hom).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize] * v;
    w = (*_hom).fwd[2 as libc::c_int as usize][0 as libc::c_int as usize] * u
        + (*_hom).fwd[2 as libc::c_int as usize][1 as libc::c_int as usize] * v
        + (*_hom).fwd22;
    dx = (*_hom).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize]
        * (*_dl).size[1 as libc::c_int as usize];
    dy = (*_hom).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize]
        * (*_dl).size[1 as libc::c_int as usize];
    dw = (*_hom).fwd[2 as libc::c_int as usize][1 as libc::c_int as usize]
        * (*_dl).size[1 as libc::c_int as usize];
    k = 8 as libc::c_int;
    while k < 15 as libc::c_int {
        qr_hom_fproject(p.as_mut_ptr(), _hom, x, y, w);
        hi[1 as libc::c_int as usize] |= (qr_img_get_bit(
            _img,
            _width,
            _height,
            p[0 as libc::c_int as usize],
            p[1 as libc::c_int as usize],
        ) << k) as libc::c_uint;
        x += dx;
        y += dy;
        w += dw;
        k += 1
    }
    /*For each group of bits we have two samples... try them in all combinations
    and pick the most popular valid code, breaking ties using the number of
    bit errors.*/
    imax = (2 as libc::c_int)
        << (hi[0 as libc::c_int as usize] != hi[1 as libc::c_int as usize]) as libc::c_int;
    di = 1 as libc::c_int
        + (lo[0 as libc::c_int as usize] == lo[1 as libc::c_int as usize]) as libc::c_int;
    nfmt_info = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < imax {
        let mut v_0: libc::c_uint = 0;
        let mut ret: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        v_0 = (lo[(i & 1 as libc::c_int) as usize] | hi[(i >> 1 as libc::c_int) as usize])
            ^ 0x5412 as libc::c_int as libc::c_uint;
        ret = bch15_5_correct(&mut v_0);
        v_0 >>= 10 as libc::c_int;
        if ret < 0 as libc::c_int {
            ret = 4 as libc::c_int
        }
        j = 0 as libc::c_int;
        loop {
            if j >= nfmt_info {
                fmt_info[j as usize] = v_0 as libc::c_int;
                count[j as usize] = 1 as libc::c_int;
                nerrs[j as usize] = ret;
                nfmt_info += 1;
                break;
            } else if fmt_info[j as usize] == v_0 as libc::c_int {
                count[j as usize] += 1;
                if ret < nerrs[j as usize] {
                    nerrs[j as usize] = ret
                }
                break;
            } else {
                j += 1
            }
        }
        i += di
    }
    besti = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < nfmt_info {
        if nerrs[besti as usize] > 3 as libc::c_int && nerrs[i as usize] <= 3 as libc::c_int
            || count[i as usize] > count[besti as usize]
            || count[i as usize] == count[besti as usize]
                && nerrs[i as usize] < nerrs[besti as usize]
        {
            besti = i
        }
        i += 1
    }
    return if nerrs[besti as usize] < 4 as libc::c_int {
        fmt_info[besti as usize]
    } else {
        -(1 as libc::c_int)
    };
}
/* Mark a given region as belonging to the function pattern. */
unsafe extern fn qr_sampling_grid_fp_mask_rect(
    mut _grid: *mut qr_sampling_grid,
    mut _dim: libc::c_int,
    mut _u: libc::c_int,
    mut _v: libc::c_int,
    mut _w: libc::c_int,
    mut _h: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut stride: libc::c_int = 0;
    stride = _dim
        + ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int * 8 as libc::c_int
        - 1 as libc::c_int
        >> (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
            * 8 as libc::c_int) as libc::c_uint
            & 0xffff0000 as libc::c_uint
            != 0
        {
            (16 as libc::c_int)
                + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                    * 8 as libc::c_int) as libc::c_uint
                    >> 16 as libc::c_int
                    & 0xff00 as libc::c_int as libc::c_uint
                    != 0
                {
                    (8 as libc::c_int)
                        + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int) as libc::c_uint
                            >> 16 as libc::c_int
                            >> 8 as libc::c_int
                            & 0xf0 as libc::c_int as libc::c_uint
                            != 0
                        {
                            (4 as libc::c_int)
                                + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 16 as libc::c_int
                                    >> 8 as libc::c_int
                                    >> 4 as libc::c_int
                                    & 0xc as libc::c_int as libc::c_uint
                                    != 0
                                {
                                    (2 as libc::c_int)
                                        + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 16 as libc::c_int
                                            >> 8 as libc::c_int
                                            >> 4 as libc::c_int
                                            >> 2 as libc::c_int
                                            & 0x2 as libc::c_int as libc::c_uint
                                            != 0)
                                            as libc::c_int
                                } else {
                                    ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 16 as libc::c_int
                                        >> 8 as libc::c_int
                                        >> 4 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                                })
                        } else {
                            (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int) as libc::c_uint
                                >> 16 as libc::c_int
                                >> 8 as libc::c_int
                                & 0xc as libc::c_int as libc::c_uint
                                != 0
                            {
                                (2 as libc::c_int)
                                    + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 16 as libc::c_int
                                        >> 8 as libc::c_int
                                        >> 2 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                            } else {
                                ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 16 as libc::c_int
                                    >> 8 as libc::c_int
                                    & 0x2 as libc::c_int as libc::c_uint
                                    != 0) as libc::c_int
                            })
                        })
                } else {
                    (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                        * 8 as libc::c_int) as libc::c_uint
                        >> 16 as libc::c_int
                        & 0xf0 as libc::c_int as libc::c_uint
                        != 0
                    {
                        (4 as libc::c_int)
                            + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int)
                                as libc::c_uint
                                >> 16 as libc::c_int
                                >> 4 as libc::c_int
                                & 0xc as libc::c_int as libc::c_uint
                                != 0
                            {
                                (2 as libc::c_int)
                                    + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 16 as libc::c_int
                                        >> 4 as libc::c_int
                                        >> 2 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                            } else {
                                ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 16 as libc::c_int
                                    >> 4 as libc::c_int
                                    & 0x2 as libc::c_int as libc::c_uint
                                    != 0) as libc::c_int
                            })
                    } else {
                        (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int) as libc::c_uint
                            >> 16 as libc::c_int
                            & 0xc as libc::c_int as libc::c_uint
                            != 0
                        {
                            (2 as libc::c_int)
                                + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 16 as libc::c_int
                                    >> 2 as libc::c_int
                                    & 0x2 as libc::c_int as libc::c_uint
                                    != 0) as libc::c_int
                        } else {
                            ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                                * 8 as libc::c_int) as libc::c_uint
                                >> 16 as libc::c_int
                                & 0x2 as libc::c_int as libc::c_uint
                                != 0) as libc::c_int
                        })
                    })
                })
        } else {
            (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                * 8 as libc::c_int) as libc::c_uint
                & 0xff00 as libc::c_int as libc::c_uint
                != 0
            {
                (8 as libc::c_int)
                    + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                        * 8 as libc::c_int) as libc::c_uint
                        >> 8 as libc::c_int
                        & 0xf0 as libc::c_int as libc::c_uint
                        != 0
                    {
                        (4 as libc::c_int)
                            + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int)
                                as libc::c_uint
                                >> 8 as libc::c_int
                                >> 4 as libc::c_int
                                & 0xc as libc::c_int as libc::c_uint
                                != 0
                            {
                                (2 as libc::c_int)
                                    + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 8 as libc::c_int
                                        >> 4 as libc::c_int
                                        >> 2 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                            } else {
                                ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 8 as libc::c_int
                                    >> 4 as libc::c_int
                                    & 0x2 as libc::c_int as libc::c_uint
                                    != 0) as libc::c_int
                            })
                    } else {
                        (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int) as libc::c_uint
                            >> 8 as libc::c_int
                            & 0xc as libc::c_int as libc::c_uint
                            != 0
                        {
                            (2 as libc::c_int)
                                + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 8 as libc::c_int
                                    >> 2 as libc::c_int
                                    & 0x2 as libc::c_int as libc::c_uint
                                    != 0) as libc::c_int
                        } else {
                            ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                                * 8 as libc::c_int) as libc::c_uint
                                >> 8 as libc::c_int
                                & 0x2 as libc::c_int as libc::c_uint
                                != 0) as libc::c_int
                        })
                    })
            } else {
                (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                    * 8 as libc::c_int) as libc::c_uint
                    & 0xf0 as libc::c_int as libc::c_uint
                    != 0
                {
                    (4 as libc::c_int)
                        + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int) as libc::c_uint
                            >> 4 as libc::c_int
                            & 0xc as libc::c_int as libc::c_uint
                            != 0
                        {
                            (2 as libc::c_int)
                                + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 4 as libc::c_int
                                    >> 2 as libc::c_int
                                    & 0x2 as libc::c_int as libc::c_uint
                                    != 0) as libc::c_int
                        } else {
                            ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                                * 8 as libc::c_int) as libc::c_uint
                                >> 4 as libc::c_int
                                & 0x2 as libc::c_int as libc::c_uint
                                != 0) as libc::c_int
                        })
                } else {
                    (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                        * 8 as libc::c_int) as libc::c_uint
                        & 0xc as libc::c_int as libc::c_uint
                        != 0
                    {
                        (2 as libc::c_int)
                            + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int) as libc::c_uint
                                >> 2 as libc::c_int
                                & 0x2 as libc::c_int as libc::c_uint
                                != 0) as libc::c_int
                    } else {
                        ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int) as libc::c_uint
                            & 0x2 as libc::c_int as libc::c_uint
                            != 0) as libc::c_int
                    })
                })
            })
        });
    /*Note that we store bits column-wise, since that's how they're read out of
    the grid.*/
    j = _u;
    while j < _u + _w {
        i = _v;
        while i < _v + _h {
            *(*_grid).fpmask.offset(
                (j * stride
                    + (i >> (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                        as libc::c_int
                        * 8 as libc::c_int) as libc::c_uint
                        & 0xffff0000 as libc::c_uint
                        != 0
                    {
                        (16 as libc::c_int)
                            + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int)
                                as libc::c_uint
                                >> 16 as libc::c_int
                                & 0xff00 as libc::c_int as libc::c_uint
                                != 0
                            {
                                (8 as libc::c_int)
                                    + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 16 as libc::c_int
                                        >> 8 as libc::c_int
                                        & 0xf0 as libc::c_int as libc::c_uint
                                        != 0
                                    {
                                        (4 as libc::c_int)
                                            + (if (::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong
                                                as libc::c_int
                                                * 8 as libc::c_int)
                                                as libc::c_uint
                                                >> 16 as libc::c_int
                                                >> 8 as libc::c_int
                                                >> 4 as libc::c_int
                                                & 0xc as libc::c_int as libc::c_uint
                                                != 0
                                            {
                                                (2 as libc::c_int)
                                                    + ((::std::mem::size_of::<libc::c_int>()
                                                        as libc::c_ulong
                                                        as libc::c_int
                                                        * 8 as libc::c_int)
                                                        as libc::c_uint
                                                        >> 16 as libc::c_int
                                                        >> 8 as libc::c_int
                                                        >> 4 as libc::c_int
                                                        >> 2 as libc::c_int
                                                        & 0x2 as libc::c_int as libc::c_uint
                                                        != 0)
                                                        as libc::c_int
                                            } else {
                                                ((::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong
                                                    as libc::c_int
                                                    * 8 as libc::c_int)
                                                    as libc::c_uint
                                                    >> 16 as libc::c_int
                                                    >> 8 as libc::c_int
                                                    >> 4 as libc::c_int
                                                    & 0x2 as libc::c_int as libc::c_uint
                                                    != 0)
                                                    as libc::c_int
                                            })
                                    } else {
                                        (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 16 as libc::c_int
                                            >> 8 as libc::c_int
                                            & 0xc as libc::c_int as libc::c_uint
                                            != 0
                                        {
                                            (2 as libc::c_int)
                                                + ((::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong
                                                    as libc::c_int
                                                    * 8 as libc::c_int)
                                                    as libc::c_uint
                                                    >> 16 as libc::c_int
                                                    >> 8 as libc::c_int
                                                    >> 2 as libc::c_int
                                                    & 0x2 as libc::c_int as libc::c_uint
                                                    != 0)
                                                    as libc::c_int
                                        } else {
                                            ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                                as libc::c_int
                                                * 8 as libc::c_int)
                                                as libc::c_uint
                                                >> 16 as libc::c_int
                                                >> 8 as libc::c_int
                                                & 0x2 as libc::c_int as libc::c_uint
                                                != 0)
                                                as libc::c_int
                                        })
                                    })
                            } else {
                                (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 16 as libc::c_int
                                    & 0xf0 as libc::c_int as libc::c_uint
                                    != 0
                                {
                                    (4 as libc::c_int)
                                        + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 16 as libc::c_int
                                            >> 4 as libc::c_int
                                            & 0xc as libc::c_int as libc::c_uint
                                            != 0
                                        {
                                            (2 as libc::c_int)
                                                + ((::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong
                                                    as libc::c_int
                                                    * 8 as libc::c_int)
                                                    as libc::c_uint
                                                    >> 16 as libc::c_int
                                                    >> 4 as libc::c_int
                                                    >> 2 as libc::c_int
                                                    & 0x2 as libc::c_int as libc::c_uint
                                                    != 0)
                                                    as libc::c_int
                                        } else {
                                            ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                                as libc::c_int
                                                * 8 as libc::c_int)
                                                as libc::c_uint
                                                >> 16 as libc::c_int
                                                >> 4 as libc::c_int
                                                & 0x2 as libc::c_int as libc::c_uint
                                                != 0)
                                                as libc::c_int
                                        })
                                } else {
                                    (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 16 as libc::c_int
                                        & 0xc as libc::c_int as libc::c_uint
                                        != 0
                                    {
                                        (2 as libc::c_int)
                                            + ((::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong
                                                as libc::c_int
                                                * 8 as libc::c_int)
                                                as libc::c_uint
                                                >> 16 as libc::c_int
                                                >> 2 as libc::c_int
                                                & 0x2 as libc::c_int as libc::c_uint
                                                != 0)
                                                as libc::c_int
                                    } else {
                                        ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 16 as libc::c_int
                                            & 0x2 as libc::c_int as libc::c_uint
                                            != 0)
                                            as libc::c_int
                                    })
                                })
                            })
                    } else {
                        (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int) as libc::c_uint
                            & 0xff00 as libc::c_int as libc::c_uint
                            != 0
                        {
                            (8 as libc::c_int)
                                + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 8 as libc::c_int
                                    & 0xf0 as libc::c_int as libc::c_uint
                                    != 0
                                {
                                    (4 as libc::c_int)
                                        + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 8 as libc::c_int
                                            >> 4 as libc::c_int
                                            & 0xc as libc::c_int as libc::c_uint
                                            != 0
                                        {
                                            (2 as libc::c_int)
                                                + ((::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong
                                                    as libc::c_int
                                                    * 8 as libc::c_int)
                                                    as libc::c_uint
                                                    >> 8 as libc::c_int
                                                    >> 4 as libc::c_int
                                                    >> 2 as libc::c_int
                                                    & 0x2 as libc::c_int as libc::c_uint
                                                    != 0)
                                                    as libc::c_int
                                        } else {
                                            ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                                as libc::c_int
                                                * 8 as libc::c_int)
                                                as libc::c_uint
                                                >> 8 as libc::c_int
                                                >> 4 as libc::c_int
                                                & 0x2 as libc::c_int as libc::c_uint
                                                != 0)
                                                as libc::c_int
                                        })
                                } else {
                                    (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 8 as libc::c_int
                                        & 0xc as libc::c_int as libc::c_uint
                                        != 0
                                    {
                                        (2 as libc::c_int)
                                            + ((::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong
                                                as libc::c_int
                                                * 8 as libc::c_int)
                                                as libc::c_uint
                                                >> 8 as libc::c_int
                                                >> 2 as libc::c_int
                                                & 0x2 as libc::c_int as libc::c_uint
                                                != 0)
                                                as libc::c_int
                                    } else {
                                        ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 8 as libc::c_int
                                            & 0x2 as libc::c_int as libc::c_uint
                                            != 0)
                                            as libc::c_int
                                    })
                                })
                        } else {
                            (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int) as libc::c_uint
                                & 0xf0 as libc::c_int as libc::c_uint
                                != 0
                            {
                                (4 as libc::c_int)
                                    + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 4 as libc::c_int
                                        & 0xc as libc::c_int as libc::c_uint
                                        != 0
                                    {
                                        (2 as libc::c_int)
                                            + ((::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong
                                                as libc::c_int
                                                * 8 as libc::c_int)
                                                as libc::c_uint
                                                >> 4 as libc::c_int
                                                >> 2 as libc::c_int
                                                & 0x2 as libc::c_int as libc::c_uint
                                                != 0)
                                                as libc::c_int
                                    } else {
                                        ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 4 as libc::c_int
                                            & 0x2 as libc::c_int as libc::c_uint
                                            != 0)
                                            as libc::c_int
                                    })
                            } else {
                                (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    & 0xc as libc::c_int as libc::c_uint
                                    != 0
                                {
                                    (2 as libc::c_int)
                                        + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 2 as libc::c_int
                                            & 0x2 as libc::c_int as libc::c_uint
                                            != 0)
                                            as libc::c_int
                                } else {
                                    ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                                })
                            })
                        })
                    }))) as isize,
            ) |= ((1 as libc::c_int)
                << (i & ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                    * 8 as libc::c_int
                    - 1 as libc::c_int)) as libc::c_uint;
            i += 1
        }
        j += 1
    }
}
/* Determine if a given grid location is inside the function pattern. */
unsafe extern fn qr_sampling_grid_is_in_fp(
    mut _grid: *const qr_sampling_grid,
    mut _dim: libc::c_int,
    mut _u: libc::c_int,
    mut _v: libc::c_int,
) -> libc::c_int {
    return (*(*_grid).fpmask.offset(
        (_u * (_dim
            + ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                * 8 as libc::c_int
            - 1 as libc::c_int
            >> (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                * 8 as libc::c_int) as libc::c_uint
                & 0xffff0000 as libc::c_uint
                != 0
            {
                (16 as libc::c_int)
                    + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                        * 8 as libc::c_int) as libc::c_uint
                        >> 16 as libc::c_int
                        & 0xff00 as libc::c_int as libc::c_uint
                        != 0
                    {
                        (8 as libc::c_int)
                            + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int)
                                as libc::c_uint
                                >> 16 as libc::c_int
                                >> 8 as libc::c_int
                                & 0xf0 as libc::c_int as libc::c_uint
                                != 0
                            {
                                (4 as libc::c_int)
                                    + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 16 as libc::c_int
                                        >> 8 as libc::c_int
                                        >> 4 as libc::c_int
                                        & 0xc as libc::c_int as libc::c_uint
                                        != 0
                                    {
                                        (2 as libc::c_int)
                                            + ((::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong
                                                as libc::c_int
                                                * 8 as libc::c_int)
                                                as libc::c_uint
                                                >> 16 as libc::c_int
                                                >> 8 as libc::c_int
                                                >> 4 as libc::c_int
                                                >> 2 as libc::c_int
                                                & 0x2 as libc::c_int as libc::c_uint
                                                != 0)
                                                as libc::c_int
                                    } else {
                                        ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 16 as libc::c_int
                                            >> 8 as libc::c_int
                                            >> 4 as libc::c_int
                                            & 0x2 as libc::c_int as libc::c_uint
                                            != 0)
                                            as libc::c_int
                                    })
                            } else {
                                (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 16 as libc::c_int
                                    >> 8 as libc::c_int
                                    & 0xc as libc::c_int as libc::c_uint
                                    != 0
                                {
                                    (2 as libc::c_int)
                                        + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 16 as libc::c_int
                                            >> 8 as libc::c_int
                                            >> 2 as libc::c_int
                                            & 0x2 as libc::c_int as libc::c_uint
                                            != 0)
                                            as libc::c_int
                                } else {
                                    ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 16 as libc::c_int
                                        >> 8 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                                })
                            })
                    } else {
                        (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int) as libc::c_uint
                            >> 16 as libc::c_int
                            & 0xf0 as libc::c_int as libc::c_uint
                            != 0
                        {
                            (4 as libc::c_int)
                                + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 16 as libc::c_int
                                    >> 4 as libc::c_int
                                    & 0xc as libc::c_int as libc::c_uint
                                    != 0
                                {
                                    (2 as libc::c_int)
                                        + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 16 as libc::c_int
                                            >> 4 as libc::c_int
                                            >> 2 as libc::c_int
                                            & 0x2 as libc::c_int as libc::c_uint
                                            != 0)
                                            as libc::c_int
                                } else {
                                    ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 16 as libc::c_int
                                        >> 4 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                                })
                        } else {
                            (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int) as libc::c_uint
                                >> 16 as libc::c_int
                                & 0xc as libc::c_int as libc::c_uint
                                != 0
                            {
                                (2 as libc::c_int)
                                    + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 16 as libc::c_int
                                        >> 2 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                            } else {
                                ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 16 as libc::c_int
                                    & 0x2 as libc::c_int as libc::c_uint
                                    != 0) as libc::c_int
                            })
                        })
                    })
            } else {
                (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                    * 8 as libc::c_int) as libc::c_uint
                    & 0xff00 as libc::c_int as libc::c_uint
                    != 0
                {
                    (8 as libc::c_int)
                        + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int) as libc::c_uint
                            >> 8 as libc::c_int
                            & 0xf0 as libc::c_int as libc::c_uint
                            != 0
                        {
                            (4 as libc::c_int)
                                + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 8 as libc::c_int
                                    >> 4 as libc::c_int
                                    & 0xc as libc::c_int as libc::c_uint
                                    != 0
                                {
                                    (2 as libc::c_int)
                                        + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 8 as libc::c_int
                                            >> 4 as libc::c_int
                                            >> 2 as libc::c_int
                                            & 0x2 as libc::c_int as libc::c_uint
                                            != 0)
                                            as libc::c_int
                                } else {
                                    ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 8 as libc::c_int
                                        >> 4 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                                })
                        } else {
                            (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int) as libc::c_uint
                                >> 8 as libc::c_int
                                & 0xc as libc::c_int as libc::c_uint
                                != 0
                            {
                                (2 as libc::c_int)
                                    + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 8 as libc::c_int
                                        >> 2 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                            } else {
                                ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 8 as libc::c_int
                                    & 0x2 as libc::c_int as libc::c_uint
                                    != 0) as libc::c_int
                            })
                        })
                } else {
                    (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                        * 8 as libc::c_int) as libc::c_uint
                        & 0xf0 as libc::c_int as libc::c_uint
                        != 0
                    {
                        (4 as libc::c_int)
                            + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int)
                                as libc::c_uint
                                >> 4 as libc::c_int
                                & 0xc as libc::c_int as libc::c_uint
                                != 0
                            {
                                (2 as libc::c_int)
                                    + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 4 as libc::c_int
                                        >> 2 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                            } else {
                                ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 4 as libc::c_int
                                    & 0x2 as libc::c_int as libc::c_uint
                                    != 0) as libc::c_int
                            })
                    } else {
                        (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int) as libc::c_uint
                            & 0xc as libc::c_int as libc::c_uint
                            != 0
                        {
                            (2 as libc::c_int)
                                + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 2 as libc::c_int
                                    & 0x2 as libc::c_int as libc::c_uint
                                    != 0) as libc::c_int
                        } else {
                            ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                                * 8 as libc::c_int) as libc::c_uint
                                & 0x2 as libc::c_int as libc::c_uint
                                != 0) as libc::c_int
                        })
                    })
                })
            }))
            + (_v
                >> (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                    * 8 as libc::c_int) as libc::c_uint
                    & 0xffff0000 as libc::c_uint
                    != 0
                {
                    (16 as libc::c_int)
                        + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int) as libc::c_uint
                            >> 16 as libc::c_int
                            & 0xff00 as libc::c_int as libc::c_uint
                            != 0
                        {
                            (8 as libc::c_int)
                                + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 16 as libc::c_int
                                    >> 8 as libc::c_int
                                    & 0xf0 as libc::c_int as libc::c_uint
                                    != 0
                                {
                                    (4 as libc::c_int)
                                        + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 16 as libc::c_int
                                            >> 8 as libc::c_int
                                            >> 4 as libc::c_int
                                            & 0xc as libc::c_int as libc::c_uint
                                            != 0
                                        {
                                            (2 as libc::c_int)
                                                + ((::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong
                                                    as libc::c_int
                                                    * 8 as libc::c_int)
                                                    as libc::c_uint
                                                    >> 16 as libc::c_int
                                                    >> 8 as libc::c_int
                                                    >> 4 as libc::c_int
                                                    >> 2 as libc::c_int
                                                    & 0x2 as libc::c_int as libc::c_uint
                                                    != 0)
                                                    as libc::c_int
                                        } else {
                                            ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                                as libc::c_int
                                                * 8 as libc::c_int)
                                                as libc::c_uint
                                                >> 16 as libc::c_int
                                                >> 8 as libc::c_int
                                                >> 4 as libc::c_int
                                                & 0x2 as libc::c_int as libc::c_uint
                                                != 0)
                                                as libc::c_int
                                        })
                                } else {
                                    (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 16 as libc::c_int
                                        >> 8 as libc::c_int
                                        & 0xc as libc::c_int as libc::c_uint
                                        != 0
                                    {
                                        (2 as libc::c_int)
                                            + ((::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong
                                                as libc::c_int
                                                * 8 as libc::c_int)
                                                as libc::c_uint
                                                >> 16 as libc::c_int
                                                >> 8 as libc::c_int
                                                >> 2 as libc::c_int
                                                & 0x2 as libc::c_int as libc::c_uint
                                                != 0)
                                                as libc::c_int
                                    } else {
                                        ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 16 as libc::c_int
                                            >> 8 as libc::c_int
                                            & 0x2 as libc::c_int as libc::c_uint
                                            != 0)
                                            as libc::c_int
                                    })
                                })
                        } else {
                            (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int) as libc::c_uint
                                >> 16 as libc::c_int
                                & 0xf0 as libc::c_int as libc::c_uint
                                != 0
                            {
                                (4 as libc::c_int)
                                    + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 16 as libc::c_int
                                        >> 4 as libc::c_int
                                        & 0xc as libc::c_int as libc::c_uint
                                        != 0
                                    {
                                        (2 as libc::c_int)
                                            + ((::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong
                                                as libc::c_int
                                                * 8 as libc::c_int)
                                                as libc::c_uint
                                                >> 16 as libc::c_int
                                                >> 4 as libc::c_int
                                                >> 2 as libc::c_int
                                                & 0x2 as libc::c_int as libc::c_uint
                                                != 0)
                                                as libc::c_int
                                    } else {
                                        ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 16 as libc::c_int
                                            >> 4 as libc::c_int
                                            & 0x2 as libc::c_int as libc::c_uint
                                            != 0)
                                            as libc::c_int
                                    })
                            } else {
                                (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 16 as libc::c_int
                                    & 0xc as libc::c_int as libc::c_uint
                                    != 0
                                {
                                    (2 as libc::c_int)
                                        + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 16 as libc::c_int
                                            >> 2 as libc::c_int
                                            & 0x2 as libc::c_int as libc::c_uint
                                            != 0)
                                            as libc::c_int
                                } else {
                                    ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 16 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                                })
                            })
                        })
                } else {
                    (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                        * 8 as libc::c_int) as libc::c_uint
                        & 0xff00 as libc::c_int as libc::c_uint
                        != 0
                    {
                        (8 as libc::c_int)
                            + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int)
                                as libc::c_uint
                                >> 8 as libc::c_int
                                & 0xf0 as libc::c_int as libc::c_uint
                                != 0
                            {
                                (4 as libc::c_int)
                                    + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 8 as libc::c_int
                                        >> 4 as libc::c_int
                                        & 0xc as libc::c_int as libc::c_uint
                                        != 0
                                    {
                                        (2 as libc::c_int)
                                            + ((::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong
                                                as libc::c_int
                                                * 8 as libc::c_int)
                                                as libc::c_uint
                                                >> 8 as libc::c_int
                                                >> 4 as libc::c_int
                                                >> 2 as libc::c_int
                                                & 0x2 as libc::c_int as libc::c_uint
                                                != 0)
                                                as libc::c_int
                                    } else {
                                        ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 8 as libc::c_int
                                            >> 4 as libc::c_int
                                            & 0x2 as libc::c_int as libc::c_uint
                                            != 0)
                                            as libc::c_int
                                    })
                            } else {
                                (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 8 as libc::c_int
                                    & 0xc as libc::c_int as libc::c_uint
                                    != 0
                                {
                                    (2 as libc::c_int)
                                        + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 8 as libc::c_int
                                            >> 2 as libc::c_int
                                            & 0x2 as libc::c_int as libc::c_uint
                                            != 0)
                                            as libc::c_int
                                } else {
                                    ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 8 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                                })
                            })
                    } else {
                        (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int) as libc::c_uint
                            & 0xf0 as libc::c_int as libc::c_uint
                            != 0
                        {
                            (4 as libc::c_int)
                                + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 4 as libc::c_int
                                    & 0xc as libc::c_int as libc::c_uint
                                    != 0
                                {
                                    (2 as libc::c_int)
                                        + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 4 as libc::c_int
                                            >> 2 as libc::c_int
                                            & 0x2 as libc::c_int as libc::c_uint
                                            != 0)
                                            as libc::c_int
                                } else {
                                    ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 4 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                                })
                        } else {
                            (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int) as libc::c_uint
                                & 0xc as libc::c_int as libc::c_uint
                                != 0
                            {
                                (2 as libc::c_int)
                                    + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 2 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                            } else {
                                ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    & 0x2 as libc::c_int as libc::c_uint
                                    != 0) as libc::c_int
                            })
                        })
                    })
                }))) as isize,
    ) >> (_v
        & ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int * 8 as libc::c_int
            - 1 as libc::c_int)
        & 1 as libc::c_int as libc::c_uint) as libc::c_int;
}
/*The spacing between alignment patterns after the second for versions >= 7.
We could compact this more, but the code to access it would eliminate the
 gains.*/
static mut QR_ALIGNMENT_SPACING: [libc::c_uchar; 34] = [
    16 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    20 as libc::c_int as libc::c_uchar,
    22 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    20 as libc::c_int as libc::c_uchar,
    22 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    22 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
];
#[inline]
unsafe extern fn qr_svg_points(
    mut cls: *const libc::c_char,
    mut p: *mut qr_point,
    mut n: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        i += 1;
        p = p.offset(1)
    }
}
/*Initialize the sampling grid for each region of the code.
_version:  The (decoded) version number.
_ul_pos:   The location of the UL finder pattern.
_ur_pos:   The location of the UR finder pattern.
_dl_pos:   The location of the DL finder pattern.
_p:        On input, contains estimated positions of the four corner modules.
           On output, contains a bounding quadrilateral for the code.
_img:      The binary input image.
_width:    The width of the input image.
_height:   The height of the input image.
Return: 0 on success, or a negative value on error.*/
unsafe extern fn qr_sampling_grid_init(
    mut _grid: *mut qr_sampling_grid,
    mut _version: libc::c_int,
    mut _ul_pos: *const libc::c_int,
    mut _ur_pos: *const libc::c_int,
    mut _dl_pos: *const libc::c_int,
    mut _p: *mut qr_point,
    mut _img: *const libc::c_uchar,
    mut _width: libc::c_int,
    mut _height: libc::c_int,
) {
    let mut base_cell: qr_hom_cell = qr_hom_cell {
        fwd: [[0; 3]; 3],
        x0: 0,
        y0: 0,
        u0: 0,
        v0: 0,
    };
    let mut align_pos: [libc::c_int; 7] = [0; 7];
    let mut dim: libc::c_int = 0;
    let mut nalign: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    dim = 17 as libc::c_int + (_version << 2 as libc::c_int);
    nalign = _version / 7 as libc::c_int + 2 as libc::c_int;
    /* Create a base cell to bootstrap the alignment pattern search. */
    qr_hom_cell_init(
        &mut base_cell,
        0 as libc::c_int,
        0 as libc::c_int,
        dim - 1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        dim - 1 as libc::c_int,
        dim - 1 as libc::c_int,
        dim - 1 as libc::c_int,
        (*_p.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize],
        (*_p.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize],
        (*_p.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize],
        (*_p.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize],
        (*_p.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize],
        (*_p.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize],
        (*_p.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize],
        (*_p.offset(3 as libc::c_int as isize))[1 as libc::c_int as usize],
    );
    /* Allocate the array of cells. */
    (*_grid).ncells = nalign - 1 as libc::c_int;
    (*_grid).cells[0 as libc::c_int as usize] = malloc(
        (((nalign - 1 as libc::c_int) * (nalign - 1 as libc::c_int)) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<qr_hom_cell>() as libc::c_ulong),
    ) as *mut qr_hom_cell;
    i = 1 as libc::c_int;
    while i < (*_grid).ncells {
        (*_grid).cells[i as usize] =
            (*_grid).cells[(i - 1 as libc::c_int) as usize].offset((*_grid).ncells as isize);
        i += 1
    }
    /* Initialize the function pattern mask. */
    (*_grid).fpmask = calloc(
        dim as libc::c_ulong,
        ((dim
            + ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                * 8 as libc::c_int
            - 1 as libc::c_int
            >> (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                * 8 as libc::c_int) as libc::c_uint
                & 0xffff0000 as libc::c_uint
                != 0
            {
                (16 as libc::c_int)
                    + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                        * 8 as libc::c_int) as libc::c_uint
                        >> 16 as libc::c_int
                        & 0xff00 as libc::c_int as libc::c_uint
                        != 0
                    {
                        (8 as libc::c_int)
                            + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int)
                                as libc::c_uint
                                >> 16 as libc::c_int
                                >> 8 as libc::c_int
                                & 0xf0 as libc::c_int as libc::c_uint
                                != 0
                            {
                                (4 as libc::c_int)
                                    + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 16 as libc::c_int
                                        >> 8 as libc::c_int
                                        >> 4 as libc::c_int
                                        & 0xc as libc::c_int as libc::c_uint
                                        != 0
                                    {
                                        (2 as libc::c_int)
                                            + ((::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong
                                                as libc::c_int
                                                * 8 as libc::c_int)
                                                as libc::c_uint
                                                >> 16 as libc::c_int
                                                >> 8 as libc::c_int
                                                >> 4 as libc::c_int
                                                >> 2 as libc::c_int
                                                & 0x2 as libc::c_int as libc::c_uint
                                                != 0)
                                                as libc::c_int
                                    } else {
                                        ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 16 as libc::c_int
                                            >> 8 as libc::c_int
                                            >> 4 as libc::c_int
                                            & 0x2 as libc::c_int as libc::c_uint
                                            != 0)
                                            as libc::c_int
                                    })
                            } else {
                                (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 16 as libc::c_int
                                    >> 8 as libc::c_int
                                    & 0xc as libc::c_int as libc::c_uint
                                    != 0
                                {
                                    (2 as libc::c_int)
                                        + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 16 as libc::c_int
                                            >> 8 as libc::c_int
                                            >> 2 as libc::c_int
                                            & 0x2 as libc::c_int as libc::c_uint
                                            != 0)
                                            as libc::c_int
                                } else {
                                    ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 16 as libc::c_int
                                        >> 8 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                                })
                            })
                    } else {
                        (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int) as libc::c_uint
                            >> 16 as libc::c_int
                            & 0xf0 as libc::c_int as libc::c_uint
                            != 0
                        {
                            (4 as libc::c_int)
                                + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 16 as libc::c_int
                                    >> 4 as libc::c_int
                                    & 0xc as libc::c_int as libc::c_uint
                                    != 0
                                {
                                    (2 as libc::c_int)
                                        + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 16 as libc::c_int
                                            >> 4 as libc::c_int
                                            >> 2 as libc::c_int
                                            & 0x2 as libc::c_int as libc::c_uint
                                            != 0)
                                            as libc::c_int
                                } else {
                                    ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 16 as libc::c_int
                                        >> 4 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                                })
                        } else {
                            (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int) as libc::c_uint
                                >> 16 as libc::c_int
                                & 0xc as libc::c_int as libc::c_uint
                                != 0
                            {
                                (2 as libc::c_int)
                                    + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 16 as libc::c_int
                                        >> 2 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                            } else {
                                ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 16 as libc::c_int
                                    & 0x2 as libc::c_int as libc::c_uint
                                    != 0) as libc::c_int
                            })
                        })
                    })
            } else {
                (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                    * 8 as libc::c_int) as libc::c_uint
                    & 0xff00 as libc::c_int as libc::c_uint
                    != 0
                {
                    (8 as libc::c_int)
                        + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int) as libc::c_uint
                            >> 8 as libc::c_int
                            & 0xf0 as libc::c_int as libc::c_uint
                            != 0
                        {
                            (4 as libc::c_int)
                                + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 8 as libc::c_int
                                    >> 4 as libc::c_int
                                    & 0xc as libc::c_int as libc::c_uint
                                    != 0
                                {
                                    (2 as libc::c_int)
                                        + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 8 as libc::c_int
                                            >> 4 as libc::c_int
                                            >> 2 as libc::c_int
                                            & 0x2 as libc::c_int as libc::c_uint
                                            != 0)
                                            as libc::c_int
                                } else {
                                    ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 8 as libc::c_int
                                        >> 4 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                                })
                        } else {
                            (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int) as libc::c_uint
                                >> 8 as libc::c_int
                                & 0xc as libc::c_int as libc::c_uint
                                != 0
                            {
                                (2 as libc::c_int)
                                    + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 8 as libc::c_int
                                        >> 2 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                            } else {
                                ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 8 as libc::c_int
                                    & 0x2 as libc::c_int as libc::c_uint
                                    != 0) as libc::c_int
                            })
                        })
                } else {
                    (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                        * 8 as libc::c_int) as libc::c_uint
                        & 0xf0 as libc::c_int as libc::c_uint
                        != 0
                    {
                        (4 as libc::c_int)
                            + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int)
                                as libc::c_uint
                                >> 4 as libc::c_int
                                & 0xc as libc::c_int as libc::c_uint
                                != 0
                            {
                                (2 as libc::c_int)
                                    + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 4 as libc::c_int
                                        >> 2 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                            } else {
                                ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 4 as libc::c_int
                                    & 0x2 as libc::c_int as libc::c_uint
                                    != 0) as libc::c_int
                            })
                    } else {
                        (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int) as libc::c_uint
                            & 0xc as libc::c_int as libc::c_uint
                            != 0
                        {
                            (2 as libc::c_int)
                                + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 2 as libc::c_int
                                    & 0x2 as libc::c_int as libc::c_uint
                                    != 0) as libc::c_int
                        } else {
                            ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                                * 8 as libc::c_int) as libc::c_uint
                                & 0x2 as libc::c_int as libc::c_uint
                                != 0) as libc::c_int
                        })
                    })
                })
            })) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong),
    ) as *mut libc::c_uint;
    /* Mask out the finder patterns (and separators and format info bits). */
    qr_sampling_grid_fp_mask_rect(
        _grid,
        dim,
        0 as libc::c_int,
        0 as libc::c_int,
        9 as libc::c_int,
        9 as libc::c_int,
    );
    qr_sampling_grid_fp_mask_rect(
        _grid,
        dim,
        0 as libc::c_int,
        dim - 8 as libc::c_int,
        9 as libc::c_int,
        8 as libc::c_int,
    );
    qr_sampling_grid_fp_mask_rect(
        _grid,
        dim,
        dim - 8 as libc::c_int,
        0 as libc::c_int,
        8 as libc::c_int,
        9 as libc::c_int,
    );
    /* Mask out the version number bits. */
    if _version > 6 as libc::c_int {
        qr_sampling_grid_fp_mask_rect(
            _grid,
            dim,
            0 as libc::c_int,
            dim - 11 as libc::c_int,
            6 as libc::c_int,
            3 as libc::c_int,
        );
        qr_sampling_grid_fp_mask_rect(
            _grid,
            dim,
            dim - 11 as libc::c_int,
            0 as libc::c_int,
            3 as libc::c_int,
            6 as libc::c_int,
        );
    }
    /* Mask out the timing patterns. */
    qr_sampling_grid_fp_mask_rect(
        _grid,
        dim,
        9 as libc::c_int,
        6 as libc::c_int,
        dim - 17 as libc::c_int,
        1 as libc::c_int,
    );
    qr_sampling_grid_fp_mask_rect(
        _grid,
        dim,
        6 as libc::c_int,
        9 as libc::c_int,
        1 as libc::c_int,
        dim - 17 as libc::c_int,
    );
    /*If we have no alignment patterns (e.g., this is a version 1 code), just use
    the base cell and hope it's good enough.*/
    if _version < 2 as libc::c_int {
        memcpy(
            (*_grid).cells[0 as libc::c_int as usize] as *mut libc::c_void,
            &mut base_cell as *mut qr_hom_cell as *const libc::c_void,
            ::std::mem::size_of::<qr_hom_cell>() as libc::c_ulong,
        );
    } else {
        let mut q: *mut qr_point = 0 as *mut qr_point;
        let mut p: *mut qr_point = 0 as *mut qr_point;
        let mut j: libc::c_int = 0;
        let mut k: libc::c_int = 0;
        q = malloc(
            ((nalign * nalign) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<qr_point>() as libc::c_ulong),
        ) as *mut qr_point;
        p = malloc(
            ((nalign * nalign) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<qr_point>() as libc::c_ulong),
        ) as *mut qr_point;
        /* Initialize the alignment pattern position list. */
        align_pos[0 as libc::c_int as usize] = 6 as libc::c_int;
        align_pos[(nalign - 1 as libc::c_int) as usize] = dim - 7 as libc::c_int;
        if _version > 6 as libc::c_int {
            let mut d: libc::c_int = 0;
            d = QR_ALIGNMENT_SPACING[(_version - 7 as libc::c_int) as usize] as libc::c_int;
            i = nalign - 1 as libc::c_int;
            loop {
                let fresh13 = i;
                i = i - 1;
                if !(fresh13 > 1 as libc::c_int) {
                    break;
                }
                align_pos[i as usize] = align_pos[(i + 1 as libc::c_int) as usize] - d
            }
        }
        /*Three of the corners use a finder pattern instead of a separate
        alignment pattern.*/
        (*q.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] = 3 as libc::c_int;
        (*q.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] = 3 as libc::c_int;
        (*p.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
            *_ul_pos.offset(0 as libc::c_int as isize);
        (*p.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] =
            *_ul_pos.offset(1 as libc::c_int as isize);
        (*q.offset((nalign - 1 as libc::c_int) as isize))[0 as libc::c_int as usize] =
            dim - 4 as libc::c_int;
        (*q.offset((nalign - 1 as libc::c_int) as isize))[1 as libc::c_int as usize] =
            3 as libc::c_int;
        (*p.offset((nalign - 1 as libc::c_int) as isize))[0 as libc::c_int as usize] =
            *_ur_pos.offset(0 as libc::c_int as isize);
        (*p.offset((nalign - 1 as libc::c_int) as isize))[1 as libc::c_int as usize] =
            *_ur_pos.offset(1 as libc::c_int as isize);
        (*q.offset(((nalign - 1 as libc::c_int) * nalign) as isize))[0 as libc::c_int as usize] =
            3 as libc::c_int;
        (*q.offset(((nalign - 1 as libc::c_int) * nalign) as isize))[1 as libc::c_int as usize] =
            dim - 4 as libc::c_int;
        (*p.offset(((nalign - 1 as libc::c_int) * nalign) as isize))[0 as libc::c_int as usize] =
            *_dl_pos.offset(0 as libc::c_int as isize);
        (*p.offset(((nalign - 1 as libc::c_int) * nalign) as isize))[1 as libc::c_int as usize] =
            *_dl_pos.offset(1 as libc::c_int as isize);
        /* Scan for alignment patterns using a diagonal sweep. */
        k = 1 as libc::c_int;
        while k < 2 as libc::c_int * nalign - 1 as libc::c_int {
            let mut jmin: libc::c_int = 0;
            let mut jmax: libc::c_int = 0;
            jmax = k
                + (nalign - 1 as libc::c_int - k
                    & -(((nalign - 1 as libc::c_int) < k) as libc::c_int))
                - (k == nalign - 1 as libc::c_int) as libc::c_int;
            jmin = 0 as libc::c_int
                - (0 as libc::c_int - (k - (nalign - 1 as libc::c_int))
                    & -((k - (nalign - 1 as libc::c_int) > 0 as libc::c_int) as libc::c_int))
                + (k == nalign - 1 as libc::c_int) as libc::c_int;
            j = jmin;
            while j <= jmax {
                let mut cell: *mut qr_hom_cell = 0 as *mut qr_hom_cell;
                let mut u: libc::c_int = 0;
                let mut v: libc::c_int = 0;
                let mut k_0: libc::c_int = 0;
                i = jmax - (j - jmin);
                k_0 = i * nalign + j;
                u = align_pos[j as usize];
                v = align_pos[i as usize];
                (*q.offset(k_0 as isize))[0 as libc::c_int as usize] = u;
                (*q.offset(k_0 as isize))[1 as libc::c_int as usize] = v;
                /* Mask out the alignment pattern. */
                qr_sampling_grid_fp_mask_rect(
                    _grid,
                    dim,
                    u - 2 as libc::c_int,
                    v - 2 as libc::c_int,
                    5 as libc::c_int,
                    5 as libc::c_int,
                );
                /* Pick a cell to use to govern the alignment pattern search. */
                if i > 1 as libc::c_int && j > 1 as libc::c_int {
                    let mut p0: qr_point = [0; 2];
                    let mut p1: qr_point = [0; 2];
                    let mut p2: qr_point = [0; 2];
                    /*Each predictor is basically a straight-line extrapolation from two
                    neighboring alignment patterns (except possibly near the opposing
                    finder patterns).*/
                    qr_hom_cell_project(
                        p0.as_mut_ptr(),
                        (*_grid).cells[(i - 2 as libc::c_int) as usize]
                            .offset(j as isize)
                            .offset(-(1 as libc::c_int as isize)),
                        u,
                        v,
                        0 as libc::c_int,
                    );
                    qr_hom_cell_project(
                        p1.as_mut_ptr(),
                        (*_grid).cells[(i - 2 as libc::c_int) as usize]
                            .offset(j as isize)
                            .offset(-(2 as libc::c_int as isize)),
                        u,
                        v,
                        0 as libc::c_int,
                    );
                    qr_hom_cell_project(
                        p2.as_mut_ptr(),
                        (*_grid).cells[(i - 1 as libc::c_int) as usize]
                            .offset(j as isize)
                            .offset(-(2 as libc::c_int as isize)),
                        u,
                        v,
                        0 as libc::c_int,
                    );
                    /* Take the median of the predictions as the search center. */
                    let mut t__: libc::c_int = 0;
                    t__ = p0[0 as libc::c_int as usize]
                        + (p1[0 as libc::c_int as usize] - p0[0 as libc::c_int as usize]
                            & -((p1[0 as libc::c_int as usize] < p0[0 as libc::c_int as usize])
                                as libc::c_int))
                        ^ p0[0 as libc::c_int as usize];
                    p0[0 as libc::c_int as usize] ^= t__;
                    p1[0 as libc::c_int as usize] ^= t__;
                    let mut t___0: libc::c_int = 0;
                    t___0 = p0[1 as libc::c_int as usize]
                        + (p1[1 as libc::c_int as usize] - p0[1 as libc::c_int as usize]
                            & -((p1[1 as libc::c_int as usize] < p0[1 as libc::c_int as usize])
                                as libc::c_int))
                        ^ p0[1 as libc::c_int as usize];
                    p0[1 as libc::c_int as usize] ^= t___0;
                    p1[1 as libc::c_int as usize] ^= t___0;
                    let mut t___1: libc::c_int = 0;
                    t___1 = p1[0 as libc::c_int as usize]
                        + (p2[0 as libc::c_int as usize] - p1[0 as libc::c_int as usize]
                            & -((p2[0 as libc::c_int as usize] < p1[0 as libc::c_int as usize])
                                as libc::c_int))
                        ^ p1[0 as libc::c_int as usize];
                    p1[0 as libc::c_int as usize] ^= t___1;
                    p2[0 as libc::c_int as usize] ^= t___1;
                    let mut t___2: libc::c_int = 0;
                    t___2 = p1[1 as libc::c_int as usize]
                        + (p2[1 as libc::c_int as usize] - p1[1 as libc::c_int as usize]
                            & -((p2[1 as libc::c_int as usize] < p1[1 as libc::c_int as usize])
                                as libc::c_int))
                        ^ p1[1 as libc::c_int as usize];
                    p1[1 as libc::c_int as usize] ^= t___2;
                    p2[1 as libc::c_int as usize] ^= t___2;
                    let mut t___3: libc::c_int = 0;
                    t___3 = p0[0 as libc::c_int as usize]
                        + (p1[0 as libc::c_int as usize] - p0[0 as libc::c_int as usize]
                            & -((p1[0 as libc::c_int as usize] < p0[0 as libc::c_int as usize])
                                as libc::c_int))
                        ^ p0[0 as libc::c_int as usize];
                    p0[0 as libc::c_int as usize] ^= t___3;
                    p1[0 as libc::c_int as usize] ^= t___3;
                    let mut t___4: libc::c_int = 0;
                    t___4 = p0[1 as libc::c_int as usize]
                        + (p1[1 as libc::c_int as usize] - p0[1 as libc::c_int as usize]
                            & -((p1[1 as libc::c_int as usize] < p0[1 as libc::c_int as usize])
                                as libc::c_int))
                        ^ p0[1 as libc::c_int as usize];
                    p0[1 as libc::c_int as usize] ^= t___4;
                    p1[1 as libc::c_int as usize] ^= t___4;
                    /*We need a cell that has the target point at a known (u,v) location.
                    Since our cells don't have inverses, just construct one from our
                     neighboring points.*/
                    cell = (*_grid).cells[(i - 1 as libc::c_int) as usize]
                        .offset(j as isize)
                        .offset(-(1 as libc::c_int as isize));
                    qr_hom_cell_init(
                        cell,
                        (*q.offset((k_0 - nalign - 1 as libc::c_int) as isize))
                            [0 as libc::c_int as usize],
                        (*q.offset((k_0 - nalign - 1 as libc::c_int) as isize))
                            [1 as libc::c_int as usize],
                        (*q.offset((k_0 - nalign) as isize))[0 as libc::c_int as usize],
                        (*q.offset((k_0 - nalign) as isize))[1 as libc::c_int as usize],
                        (*q.offset((k_0 - 1 as libc::c_int) as isize))[0 as libc::c_int as usize],
                        (*q.offset((k_0 - 1 as libc::c_int) as isize))[1 as libc::c_int as usize],
                        (*q.offset(k_0 as isize))[0 as libc::c_int as usize],
                        (*q.offset(k_0 as isize))[1 as libc::c_int as usize],
                        (*p.offset((k_0 - nalign - 1 as libc::c_int) as isize))
                            [0 as libc::c_int as usize],
                        (*p.offset((k_0 - nalign - 1 as libc::c_int) as isize))
                            [1 as libc::c_int as usize],
                        (*p.offset((k_0 - nalign) as isize))[0 as libc::c_int as usize],
                        (*p.offset((k_0 - nalign) as isize))[1 as libc::c_int as usize],
                        (*p.offset((k_0 - 1 as libc::c_int) as isize))[0 as libc::c_int as usize],
                        (*p.offset((k_0 - 1 as libc::c_int) as isize))[1 as libc::c_int as usize],
                        p1[0 as libc::c_int as usize],
                        p1[1 as libc::c_int as usize],
                    );
                } else if i > 1 as libc::c_int && j > 0 as libc::c_int {
                    cell = (*_grid).cells[(i - 2 as libc::c_int) as usize]
                        .offset(j as isize)
                        .offset(-(1 as libc::c_int as isize))
                } else if i > 0 as libc::c_int && j > 1 as libc::c_int {
                    cell = (*_grid).cells[(i - 1 as libc::c_int) as usize]
                        .offset(j as isize)
                        .offset(-(2 as libc::c_int as isize))
                } else {
                    cell = &mut base_cell
                }
                /*Use a very small search radius.
                A large displacement here usually means a false positive (e.g., when
                 the real alignment pattern is damaged or missing), which can
                 severely distort the projection.*/
                qr_alignment_pattern_search(
                    (*p.offset(k_0 as isize)).as_mut_ptr(),
                    cell,
                    u,
                    v,
                    2 as libc::c_int,
                    _img,
                    _width,
                    _height,
                );
                if i > 0 as libc::c_int && j > 0 as libc::c_int {
                    qr_hom_cell_init(
                        (*_grid).cells[(i - 1 as libc::c_int) as usize]
                            .offset(j as isize)
                            .offset(-(1 as libc::c_int as isize)),
                        (*q.offset((k_0 - nalign - 1 as libc::c_int) as isize))
                            [0 as libc::c_int as usize],
                        (*q.offset((k_0 - nalign - 1 as libc::c_int) as isize))
                            [1 as libc::c_int as usize],
                        (*q.offset((k_0 - nalign) as isize))[0 as libc::c_int as usize],
                        (*q.offset((k_0 - nalign) as isize))[1 as libc::c_int as usize],
                        (*q.offset((k_0 - 1 as libc::c_int) as isize))[0 as libc::c_int as usize],
                        (*q.offset((k_0 - 1 as libc::c_int) as isize))[1 as libc::c_int as usize],
                        (*q.offset(k_0 as isize))[0 as libc::c_int as usize],
                        (*q.offset(k_0 as isize))[1 as libc::c_int as usize],
                        (*p.offset((k_0 - nalign - 1 as libc::c_int) as isize))
                            [0 as libc::c_int as usize],
                        (*p.offset((k_0 - nalign - 1 as libc::c_int) as isize))
                            [1 as libc::c_int as usize],
                        (*p.offset((k_0 - nalign) as isize))[0 as libc::c_int as usize],
                        (*p.offset((k_0 - nalign) as isize))[1 as libc::c_int as usize],
                        (*p.offset((k_0 - 1 as libc::c_int) as isize))[0 as libc::c_int as usize],
                        (*p.offset((k_0 - 1 as libc::c_int) as isize))[1 as libc::c_int as usize],
                        (*p.offset(k_0 as isize))[0 as libc::c_int as usize],
                        (*p.offset(k_0 as isize))[1 as libc::c_int as usize],
                    );
                }
                j += 1
            }
            k += 1
        }
        qr_svg_points(b"align\x00" as *const u8 as *const libc::c_char, p, nalign * nalign);
        free(q as *mut libc::c_void);
        free(p as *mut libc::c_void);
    }
    /* Set the limits over which each cell is used. */
    memcpy(
        (*_grid).cell_limits.as_mut_ptr() as *mut libc::c_void,
        align_pos.as_mut_ptr().offset(1 as libc::c_int as isize) as *const libc::c_void,
        (((*_grid).ncells - 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    (*_grid).cell_limits[((*_grid).ncells - 1 as libc::c_int) as usize] = dim;
    /*Produce a bounding square for the code (to mark finder centers with).
    Because of non-linear distortion, this might not actually bound the code,
     but it should be good enough.
    I don't think it's worth computing a convex hull or anything silly like
     that.*/
    qr_hom_cell_project(
        (*_p.offset(0 as libc::c_int as isize)).as_mut_ptr(),
        (*_grid).cells[0 as libc::c_int as usize].offset(0 as libc::c_int as isize),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        1 as libc::c_int,
    );
    qr_hom_cell_project(
        (*_p.offset(1 as libc::c_int as isize)).as_mut_ptr(),
        (*_grid).cells[0 as libc::c_int as usize]
            .offset((*_grid).ncells as isize)
            .offset(-(1 as libc::c_int as isize)),
        (dim << 1 as libc::c_int) - 1 as libc::c_int,
        -(1 as libc::c_int),
        1 as libc::c_int,
    );
    qr_hom_cell_project(
        (*_p.offset(2 as libc::c_int as isize)).as_mut_ptr(),
        (*_grid).cells[((*_grid).ncells - 1 as libc::c_int) as usize]
            .offset(0 as libc::c_int as isize),
        -(1 as libc::c_int),
        (dim << 1 as libc::c_int) - 1 as libc::c_int,
        1 as libc::c_int,
    );
    qr_hom_cell_project(
        (*_p.offset(3 as libc::c_int as isize)).as_mut_ptr(),
        (*_grid).cells[((*_grid).ncells - 1 as libc::c_int) as usize]
            .offset((*_grid).ncells as isize)
            .offset(-(1 as libc::c_int as isize)),
        (dim << 1 as libc::c_int) - 1 as libc::c_int,
        (dim << 1 as libc::c_int) - 1 as libc::c_int,
        1 as libc::c_int,
    );
    /*Clamp the points somewhere near the image (this is really just in case a
    corner is near the plane at infinity).*/
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        (*_p.offset(i as isize))[0 as libc::c_int as usize] = (-_width << 2 as libc::c_int)
            - ((-_width << 2 as libc::c_int)
                - ((*_p.offset(i as isize))[0 as libc::c_int as usize]
                    + ((_width << 2 as libc::c_int + 1 as libc::c_int)
                        - (*_p.offset(i as isize))[0 as libc::c_int as usize]
                        & -(((_width << 2 as libc::c_int + 1 as libc::c_int)
                            < (*_p.offset(i as isize))[0 as libc::c_int as usize])
                            as libc::c_int)))
                & -(((*_p.offset(i as isize))[0 as libc::c_int as usize]
                    + ((_width << 2 as libc::c_int + 1 as libc::c_int)
                        - (*_p.offset(i as isize))[0 as libc::c_int as usize]
                        & -(((_width << 2 as libc::c_int + 1 as libc::c_int)
                            < (*_p.offset(i as isize))[0 as libc::c_int as usize])
                            as libc::c_int))
                    > -_width << 2 as libc::c_int) as libc::c_int));
        (*_p.offset(i as isize))[1 as libc::c_int as usize] = (-_height << 2 as libc::c_int)
            - ((-_height << 2 as libc::c_int)
                - ((*_p.offset(i as isize))[1 as libc::c_int as usize]
                    + ((_height << 2 as libc::c_int + 1 as libc::c_int)
                        - (*_p.offset(i as isize))[1 as libc::c_int as usize]
                        & -(((_height << 2 as libc::c_int + 1 as libc::c_int)
                            < (*_p.offset(i as isize))[1 as libc::c_int as usize])
                            as libc::c_int)))
                & -(((*_p.offset(i as isize))[1 as libc::c_int as usize]
                    + ((_height << 2 as libc::c_int + 1 as libc::c_int)
                        - (*_p.offset(i as isize))[1 as libc::c_int as usize]
                        & -(((_height << 2 as libc::c_int + 1 as libc::c_int)
                            < (*_p.offset(i as isize))[1 as libc::c_int as usize])
                            as libc::c_int))
                    > -_height << 2 as libc::c_int) as libc::c_int));
        i += 1
    }
    /*TODO: Make fine adjustments using the timing patterns.
    Possible strategy: scan the timing pattern at QR_ALIGN_SUBPREC (or finer)
     resolution, use dynamic programming to match midpoints between
     transitions to the ideal grid locations.*/
}
unsafe extern fn qr_sampling_grid_clear(mut _grid: *mut qr_sampling_grid) {
    free((*_grid).fpmask as *mut libc::c_void);
    free((*_grid).cells[0 as libc::c_int as usize] as *mut libc::c_void);
}
/* Generate the data mask corresponding to the given mask pattern. */
unsafe extern fn qr_data_mask_fill(
    mut _mask: *mut libc::c_uint,
    mut _dim: libc::c_int,
    mut _pattern: libc::c_int,
) {
    let mut stride: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    stride = _dim
        + ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int * 8 as libc::c_int
        - 1 as libc::c_int
        >> (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
            * 8 as libc::c_int) as libc::c_uint
            & 0xffff0000 as libc::c_uint
            != 0
        {
            (16 as libc::c_int)
                + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                    * 8 as libc::c_int) as libc::c_uint
                    >> 16 as libc::c_int
                    & 0xff00 as libc::c_int as libc::c_uint
                    != 0
                {
                    (8 as libc::c_int)
                        + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int) as libc::c_uint
                            >> 16 as libc::c_int
                            >> 8 as libc::c_int
                            & 0xf0 as libc::c_int as libc::c_uint
                            != 0
                        {
                            (4 as libc::c_int)
                                + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 16 as libc::c_int
                                    >> 8 as libc::c_int
                                    >> 4 as libc::c_int
                                    & 0xc as libc::c_int as libc::c_uint
                                    != 0
                                {
                                    (2 as libc::c_int)
                                        + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 16 as libc::c_int
                                            >> 8 as libc::c_int
                                            >> 4 as libc::c_int
                                            >> 2 as libc::c_int
                                            & 0x2 as libc::c_int as libc::c_uint
                                            != 0)
                                            as libc::c_int
                                } else {
                                    ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 16 as libc::c_int
                                        >> 8 as libc::c_int
                                        >> 4 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                                })
                        } else {
                            (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int) as libc::c_uint
                                >> 16 as libc::c_int
                                >> 8 as libc::c_int
                                & 0xc as libc::c_int as libc::c_uint
                                != 0
                            {
                                (2 as libc::c_int)
                                    + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 16 as libc::c_int
                                        >> 8 as libc::c_int
                                        >> 2 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                            } else {
                                ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 16 as libc::c_int
                                    >> 8 as libc::c_int
                                    & 0x2 as libc::c_int as libc::c_uint
                                    != 0) as libc::c_int
                            })
                        })
                } else {
                    (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                        * 8 as libc::c_int) as libc::c_uint
                        >> 16 as libc::c_int
                        & 0xf0 as libc::c_int as libc::c_uint
                        != 0
                    {
                        (4 as libc::c_int)
                            + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int)
                                as libc::c_uint
                                >> 16 as libc::c_int
                                >> 4 as libc::c_int
                                & 0xc as libc::c_int as libc::c_uint
                                != 0
                            {
                                (2 as libc::c_int)
                                    + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 16 as libc::c_int
                                        >> 4 as libc::c_int
                                        >> 2 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                            } else {
                                ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 16 as libc::c_int
                                    >> 4 as libc::c_int
                                    & 0x2 as libc::c_int as libc::c_uint
                                    != 0) as libc::c_int
                            })
                    } else {
                        (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int) as libc::c_uint
                            >> 16 as libc::c_int
                            & 0xc as libc::c_int as libc::c_uint
                            != 0
                        {
                            (2 as libc::c_int)
                                + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 16 as libc::c_int
                                    >> 2 as libc::c_int
                                    & 0x2 as libc::c_int as libc::c_uint
                                    != 0) as libc::c_int
                        } else {
                            ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                                * 8 as libc::c_int) as libc::c_uint
                                >> 16 as libc::c_int
                                & 0x2 as libc::c_int as libc::c_uint
                                != 0) as libc::c_int
                        })
                    })
                })
        } else {
            (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                * 8 as libc::c_int) as libc::c_uint
                & 0xff00 as libc::c_int as libc::c_uint
                != 0
            {
                (8 as libc::c_int)
                    + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                        * 8 as libc::c_int) as libc::c_uint
                        >> 8 as libc::c_int
                        & 0xf0 as libc::c_int as libc::c_uint
                        != 0
                    {
                        (4 as libc::c_int)
                            + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int)
                                as libc::c_uint
                                >> 8 as libc::c_int
                                >> 4 as libc::c_int
                                & 0xc as libc::c_int as libc::c_uint
                                != 0
                            {
                                (2 as libc::c_int)
                                    + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 8 as libc::c_int
                                        >> 4 as libc::c_int
                                        >> 2 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                            } else {
                                ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 8 as libc::c_int
                                    >> 4 as libc::c_int
                                    & 0x2 as libc::c_int as libc::c_uint
                                    != 0) as libc::c_int
                            })
                    } else {
                        (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int) as libc::c_uint
                            >> 8 as libc::c_int
                            & 0xc as libc::c_int as libc::c_uint
                            != 0
                        {
                            (2 as libc::c_int)
                                + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 8 as libc::c_int
                                    >> 2 as libc::c_int
                                    & 0x2 as libc::c_int as libc::c_uint
                                    != 0) as libc::c_int
                        } else {
                            ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                                * 8 as libc::c_int) as libc::c_uint
                                >> 8 as libc::c_int
                                & 0x2 as libc::c_int as libc::c_uint
                                != 0) as libc::c_int
                        })
                    })
            } else {
                (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                    * 8 as libc::c_int) as libc::c_uint
                    & 0xf0 as libc::c_int as libc::c_uint
                    != 0
                {
                    (4 as libc::c_int)
                        + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int) as libc::c_uint
                            >> 4 as libc::c_int
                            & 0xc as libc::c_int as libc::c_uint
                            != 0
                        {
                            (2 as libc::c_int)
                                + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 4 as libc::c_int
                                    >> 2 as libc::c_int
                                    & 0x2 as libc::c_int as libc::c_uint
                                    != 0) as libc::c_int
                        } else {
                            ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                                * 8 as libc::c_int) as libc::c_uint
                                >> 4 as libc::c_int
                                & 0x2 as libc::c_int as libc::c_uint
                                != 0) as libc::c_int
                        })
                } else {
                    (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                        * 8 as libc::c_int) as libc::c_uint
                        & 0xc as libc::c_int as libc::c_uint
                        != 0
                    {
                        (2 as libc::c_int)
                            + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int) as libc::c_uint
                                >> 2 as libc::c_int
                                & 0x2 as libc::c_int as libc::c_uint
                                != 0) as libc::c_int
                    } else {
                        ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int) as libc::c_uint
                            & 0x2 as libc::c_int as libc::c_uint
                            != 0) as libc::c_int
                    })
                })
            })
        });
    /*Note that we store bits column-wise, since that's how they're read out of
    the grid.*/
    match _pattern {
        0 => {
            /*10101010 i+j+1&1
            01010101
            10101010
            01010101*/
            let mut m: libc::c_int = 0;
            m = 0x55 as libc::c_int;
            j = 0 as libc::c_int;
            while j < _dim {
                memset(
                    _mask.offset((j * stride) as isize) as *mut libc::c_void,
                    m,
                    (stride as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong),
                );
                m ^= 0xff as libc::c_int;
                j += 1
            }
        }
        1 => {
            /*11111111 i+1&1
            00000000
            11111111
            00000000*/
            memset(
                _mask as *mut libc::c_void,
                0x55 as libc::c_int,
                ((_dim * stride) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong),
            );
        }
        2 => {
            /*10010010 (j+1)%3&1
            10010010
            10010010
            10010010*/
            let mut m_0: libc::c_uint = 0;
            m_0 = 0xff as libc::c_int as libc::c_uint;
            j = 0 as libc::c_int;
            while j < _dim {
                memset(
                    _mask.offset((j * stride) as isize) as *mut libc::c_void,
                    (m_0 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
                    (stride as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong),
                );
                m_0 = m_0 << 8 as libc::c_int | m_0 >> 16 as libc::c_int;
                j += 1
            }
        }
        3 => {
            /*10010010 (i+j+1)%3&1
            00100100
            01001001
            10010010*/
            let mut mi: libc::c_uint = 0;
            let mut mj: libc::c_uint = 0;
            mj = 0 as libc::c_int as libc::c_uint;
            i = 0 as libc::c_int;
            while i
                < (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                    * 8 as libc::c_int
                    + 2 as libc::c_int)
                    / 3 as libc::c_int
            {
                mj |= ((1 as libc::c_int) << 3 as libc::c_int * i) as libc::c_uint;
                i += 1
            }
            j = 0 as libc::c_int;
            while j < _dim {
                mi = mj;
                i = 0 as libc::c_int;
                while i < stride {
                    *_mask.offset((j * stride + i) as isize) = mi;
                    mi = mi
                        >> ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int
                            % 3 as libc::c_int
                        | mi << 3 as libc::c_int
                            - ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                                * 8 as libc::c_int
                                % 3 as libc::c_int;
                    i += 1
                }
                mj = mj >> 1 as libc::c_int | mj << 2 as libc::c_int;
                j += 1
            }
        }
        4 => {
            /*11100011 (i>>1)+(j/3)+1&1
            11100011
            00011100
            00011100*/
            let mut m_1: libc::c_uint = 0;
            m_1 = 7 as libc::c_int as libc::c_uint;
            j = 0 as libc::c_int;
            while j < _dim {
                memset(
                    _mask.offset((j * stride) as isize) as *mut libc::c_void,
                    ((0xcc as libc::c_int as libc::c_uint
                        ^ (m_1 & 1 as libc::c_int as libc::c_uint).wrapping_neg())
                        & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
                    (stride as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong),
                );
                m_1 = m_1 >> 1 as libc::c_int | m_1 << 5 as libc::c_int;
                j += 1
            }
        }
        5 => {
            /*11111111 !((i*j)%6)
            10000010
            10010010
            10101010*/
            j = 0 as libc::c_int;
            while j < _dim {
                let mut m_2: libc::c_uint = 0;
                m_2 = 0 as libc::c_int as libc::c_uint;
                i = 0 as libc::c_int;
                while i < 6 as libc::c_int {
                    m_2 |= (((i * j % 6 as libc::c_int == 0) as libc::c_int) << i) as libc::c_uint;
                    i += 1
                }
                i = 6 as libc::c_int;
                while i < ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                    * 8 as libc::c_int
                {
                    m_2 |= m_2 << i;
                    i <<= 1 as libc::c_int
                }
                i = 0 as libc::c_int;
                while i < stride {
                    *_mask.offset((j * stride + i) as isize) = m_2;
                    m_2 = m_2
                        >> ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int
                            % 6 as libc::c_int
                        | m_2
                            << 6 as libc::c_int
                                - ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int
                                    % 6 as libc::c_int;
                    i += 1
                }
                j += 1
            }
        }
        6 => {
            /*11111111 (i*j)%3+i*j+1&1
            11100011
            11011011
            10101010*/
            j = 0 as libc::c_int;
            while j < _dim {
                let mut m_3: libc::c_uint = 0;
                m_3 = 0 as libc::c_int as libc::c_uint;
                i = 0 as libc::c_int;
                while i < 6 as libc::c_int {
                    m_3 |= ((i * j % 3 as libc::c_int + i * j + 1 as libc::c_int
                        & 1 as libc::c_int)
                        << i) as libc::c_uint;
                    i += 1
                }
                i = 6 as libc::c_int;
                while i < ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                    * 8 as libc::c_int
                {
                    m_3 |= m_3 << i;
                    i <<= 1 as libc::c_int
                }
                i = 0 as libc::c_int;
                while i < stride {
                    *_mask.offset((j * stride + i) as isize) = m_3;
                    m_3 = m_3
                        >> ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int
                            % 6 as libc::c_int
                        | m_3
                            << 6 as libc::c_int
                                - ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int
                                    % 6 as libc::c_int;
                    i += 1
                }
                j += 1
            }
        }
        _ => {
            /*10101010 (i*j)%3+i+j+1&1
            00011100
            10001110
            01010101*/
            j = 0 as libc::c_int;
            while j < _dim {
                let mut m_4: libc::c_uint = 0;
                m_4 = 0 as libc::c_int as libc::c_uint;
                i = 0 as libc::c_int;
                while i < 6 as libc::c_int {
                    m_4 |= ((i * j % 3 as libc::c_int + i + j + 1 as libc::c_int
                        & 1 as libc::c_int)
                        << i) as libc::c_uint;
                    i += 1
                }
                i = 6 as libc::c_int;
                while i < ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                    * 8 as libc::c_int
                {
                    m_4 |= m_4 << i;
                    i <<= 1 as libc::c_int
                }
                i = 0 as libc::c_int;
                while i < stride {
                    *_mask.offset((j * stride + i) as isize) = m_4;
                    m_4 = m_4
                        >> ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int
                            % 6 as libc::c_int
                        | m_4
                            << 6 as libc::c_int
                                - ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int
                                    % 6 as libc::c_int;
                    i += 1
                }
                j += 1
            }
        }
    };
}
unsafe extern fn qr_sampling_grid_sample(
    mut _grid: *const qr_sampling_grid,
    mut _data_bits: *mut libc::c_uint,
    mut _dim: libc::c_int,
    mut _fmt_info: libc::c_int,
    mut _img: *const libc::c_uchar,
    mut _width: libc::c_int,
    mut _height: libc::c_int,
) {
    let mut stride: libc::c_int = 0;
    let mut u0: libc::c_int = 0;
    let mut u1: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    /*We initialize the buffer with the data mask and XOR bits into it as we read
    them out of the image instead of unmasking in a separate step.*/
    qr_data_mask_fill(_data_bits, _dim, _fmt_info & 7 as libc::c_int);
    stride = _dim
        + ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int * 8 as libc::c_int
        - 1 as libc::c_int
        >> (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
            * 8 as libc::c_int) as libc::c_uint
            & 0xffff0000 as libc::c_uint
            != 0
        {
            (16 as libc::c_int)
                + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                    * 8 as libc::c_int) as libc::c_uint
                    >> 16 as libc::c_int
                    & 0xff00 as libc::c_int as libc::c_uint
                    != 0
                {
                    (8 as libc::c_int)
                        + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int) as libc::c_uint
                            >> 16 as libc::c_int
                            >> 8 as libc::c_int
                            & 0xf0 as libc::c_int as libc::c_uint
                            != 0
                        {
                            (4 as libc::c_int)
                                + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 16 as libc::c_int
                                    >> 8 as libc::c_int
                                    >> 4 as libc::c_int
                                    & 0xc as libc::c_int as libc::c_uint
                                    != 0
                                {
                                    (2 as libc::c_int)
                                        + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 16 as libc::c_int
                                            >> 8 as libc::c_int
                                            >> 4 as libc::c_int
                                            >> 2 as libc::c_int
                                            & 0x2 as libc::c_int as libc::c_uint
                                            != 0)
                                            as libc::c_int
                                } else {
                                    ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 16 as libc::c_int
                                        >> 8 as libc::c_int
                                        >> 4 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                                })
                        } else {
                            (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int) as libc::c_uint
                                >> 16 as libc::c_int
                                >> 8 as libc::c_int
                                & 0xc as libc::c_int as libc::c_uint
                                != 0
                            {
                                (2 as libc::c_int)
                                    + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 16 as libc::c_int
                                        >> 8 as libc::c_int
                                        >> 2 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                            } else {
                                ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 16 as libc::c_int
                                    >> 8 as libc::c_int
                                    & 0x2 as libc::c_int as libc::c_uint
                                    != 0) as libc::c_int
                            })
                        })
                } else {
                    (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                        * 8 as libc::c_int) as libc::c_uint
                        >> 16 as libc::c_int
                        & 0xf0 as libc::c_int as libc::c_uint
                        != 0
                    {
                        (4 as libc::c_int)
                            + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int)
                                as libc::c_uint
                                >> 16 as libc::c_int
                                >> 4 as libc::c_int
                                & 0xc as libc::c_int as libc::c_uint
                                != 0
                            {
                                (2 as libc::c_int)
                                    + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 16 as libc::c_int
                                        >> 4 as libc::c_int
                                        >> 2 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                            } else {
                                ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 16 as libc::c_int
                                    >> 4 as libc::c_int
                                    & 0x2 as libc::c_int as libc::c_uint
                                    != 0) as libc::c_int
                            })
                    } else {
                        (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int) as libc::c_uint
                            >> 16 as libc::c_int
                            & 0xc as libc::c_int as libc::c_uint
                            != 0
                        {
                            (2 as libc::c_int)
                                + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 16 as libc::c_int
                                    >> 2 as libc::c_int
                                    & 0x2 as libc::c_int as libc::c_uint
                                    != 0) as libc::c_int
                        } else {
                            ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                                * 8 as libc::c_int) as libc::c_uint
                                >> 16 as libc::c_int
                                & 0x2 as libc::c_int as libc::c_uint
                                != 0) as libc::c_int
                        })
                    })
                })
        } else {
            (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                * 8 as libc::c_int) as libc::c_uint
                & 0xff00 as libc::c_int as libc::c_uint
                != 0
            {
                (8 as libc::c_int)
                    + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                        * 8 as libc::c_int) as libc::c_uint
                        >> 8 as libc::c_int
                        & 0xf0 as libc::c_int as libc::c_uint
                        != 0
                    {
                        (4 as libc::c_int)
                            + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int)
                                as libc::c_uint
                                >> 8 as libc::c_int
                                >> 4 as libc::c_int
                                & 0xc as libc::c_int as libc::c_uint
                                != 0
                            {
                                (2 as libc::c_int)
                                    + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 8 as libc::c_int
                                        >> 4 as libc::c_int
                                        >> 2 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                            } else {
                                ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 8 as libc::c_int
                                    >> 4 as libc::c_int
                                    & 0x2 as libc::c_int as libc::c_uint
                                    != 0) as libc::c_int
                            })
                    } else {
                        (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int) as libc::c_uint
                            >> 8 as libc::c_int
                            & 0xc as libc::c_int as libc::c_uint
                            != 0
                        {
                            (2 as libc::c_int)
                                + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 8 as libc::c_int
                                    >> 2 as libc::c_int
                                    & 0x2 as libc::c_int as libc::c_uint
                                    != 0) as libc::c_int
                        } else {
                            ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                                * 8 as libc::c_int) as libc::c_uint
                                >> 8 as libc::c_int
                                & 0x2 as libc::c_int as libc::c_uint
                                != 0) as libc::c_int
                        })
                    })
            } else {
                (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                    * 8 as libc::c_int) as libc::c_uint
                    & 0xf0 as libc::c_int as libc::c_uint
                    != 0
                {
                    (4 as libc::c_int)
                        + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int) as libc::c_uint
                            >> 4 as libc::c_int
                            & 0xc as libc::c_int as libc::c_uint
                            != 0
                        {
                            (2 as libc::c_int)
                                + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 4 as libc::c_int
                                    >> 2 as libc::c_int
                                    & 0x2 as libc::c_int as libc::c_uint
                                    != 0) as libc::c_int
                        } else {
                            ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                                * 8 as libc::c_int) as libc::c_uint
                                >> 4 as libc::c_int
                                & 0x2 as libc::c_int as libc::c_uint
                                != 0) as libc::c_int
                        })
                } else {
                    (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                        * 8 as libc::c_int) as libc::c_uint
                        & 0xc as libc::c_int as libc::c_uint
                        != 0
                    {
                        (2 as libc::c_int)
                            + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int) as libc::c_uint
                                >> 2 as libc::c_int
                                & 0x2 as libc::c_int as libc::c_uint
                                != 0) as libc::c_int
                    } else {
                        ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int) as libc::c_uint
                            & 0x2 as libc::c_int as libc::c_uint
                            != 0) as libc::c_int
                    })
                })
            })
        });
    u0 = 0 as libc::c_int;
    /*We read data cell-by-cell to avoid having to constantly change which
     projection we're using as we read each bit.
    This (and the position-dependent data mask) is the reason we buffer the
     bits we read instead of converting them directly to codewords here.
    Note that bits are stored column-wise, since that's how we'll scan them.*/
    j = 0 as libc::c_int;
    while j < (*_grid).ncells {
        let mut i: libc::c_int = 0;
        let mut v0: libc::c_int = 0;
        let mut v1: libc::c_int = 0;
        u1 = (*_grid).cell_limits[j as usize];
        v0 = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < (*_grid).ncells {
            let mut cell: *mut qr_hom_cell = 0 as *mut qr_hom_cell;
            let mut x0: libc::c_int = 0;
            let mut y0: libc::c_int = 0;
            let mut w0: libc::c_int = 0;
            let mut u: libc::c_int = 0;
            let mut du: libc::c_int = 0;
            let mut dv: libc::c_int = 0;
            v1 = (*_grid).cell_limits[i as usize];
            cell = (*_grid).cells[i as usize].offset(j as isize);
            du = u0 - (*cell).u0;
            dv = v0 - (*cell).v0;
            x0 = (*cell).fwd[0 as libc::c_int as usize][0 as libc::c_int as usize] * du
                + (*cell).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize] * dv
                + (*cell).fwd[0 as libc::c_int as usize][2 as libc::c_int as usize];
            y0 = (*cell).fwd[1 as libc::c_int as usize][0 as libc::c_int as usize] * du
                + (*cell).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize] * dv
                + (*cell).fwd[1 as libc::c_int as usize][2 as libc::c_int as usize];
            w0 = (*cell).fwd[2 as libc::c_int as usize][0 as libc::c_int as usize] * du
                + (*cell).fwd[2 as libc::c_int as usize][1 as libc::c_int as usize] * dv
                + (*cell).fwd[2 as libc::c_int as usize][2 as libc::c_int as usize];
            u = u0;
            while u < u1 {
                let mut x: libc::c_int = 0;
                let mut y: libc::c_int = 0;
                let mut w: libc::c_int = 0;
                let mut v: libc::c_int = 0;
                x = x0;
                y = y0;
                w = w0;
                v = v0;
                while v < v1 {
                    /*Skip doing all the divisions and bounds checks if the bit is in the
                    function pattern.*/
                    if qr_sampling_grid_is_in_fp(_grid, _dim, u, v) == 0 {
                        let mut p: qr_point = [0; 2];
                        qr_hom_cell_fproject(p.as_mut_ptr(), cell, x, y, w);
                        *_data_bits.offset((u * stride +
                                                (v >>
                                                     (if (::std::mem::size_of::<libc::c_int>()
                                                              as libc::c_ulong
                                                              as libc::c_int *
                                                              8 as
                                                                  libc::c_int)
                                                             as libc::c_uint &
                                                             0xffff0000 as
                                                                 libc::c_uint
                                                             != 0 {
                                                          (16 as libc::c_int)
                                                              +
                                                              (if (::std::mem::size_of::<libc::c_int>()
                                                                       as
                                                                       libc::c_ulong
                                                                       as
                                                                       libc::c_int
                                                                       *
                                                                       8 as
                                                                           libc::c_int)
                                                                      as
                                                                      libc::c_uint
                                                                      >>
                                                                      16 as
                                                                          libc::c_int
                                                                      &
                                                                      0xff00
                                                                          as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint
                                                                      != 0 {
                                                                   (8 as
                                                                        libc::c_int)
                                                                       +
                                                                       (if (::std::mem::size_of::<libc::c_int>()
                                                                                as
                                                                                libc::c_ulong
                                                                                as
                                                                                libc::c_int
                                                                                *
                                                                                8
                                                                                    as
                                                                                    libc::c_int)
                                                                               as
                                                                               libc::c_uint
                                                                               >>
                                                                               16
                                                                                   as
                                                                                   libc::c_int
                                                                               >>
                                                                               8
                                                                                   as
                                                                                   libc::c_int
                                                                               &
                                                                               0xf0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint
                                                                               !=
                                                                               0
                                                                           {
                                                                            (4
                                                                                 as
                                                                                 libc::c_int)
                                                                                +
                                                                                (if (::std::mem::size_of::<libc::c_int>()
                                                                                         as
                                                                                         libc::c_ulong
                                                                                         as
                                                                                         libc::c_int
                                                                                         *
                                                                                         8
                                                                                             as
                                                                                             libc::c_int)
                                                                                        as
                                                                                        libc::c_uint
                                                                                        >>
                                                                                        16
                                                                                            as
                                                                                            libc::c_int
                                                                                        >>
                                                                                        8
                                                                                            as
                                                                                            libc::c_int
                                                                                        >>
                                                                                        4
                                                                                            as
                                                                                            libc::c_int
                                                                                        &
                                                                                        0xc
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_uint
                                                                                        !=
                                                                                        0
                                                                                    {
                                                                                     (2
                                                                                          as
                                                                                          libc::c_int)
                                                                                         +
                                                                                         ((::std::mem::size_of::<libc::c_int>()
                                                                                               as
                                                                                               libc::c_ulong
                                                                                               as
                                                                                               libc::c_int
                                                                                               *
                                                                                               8
                                                                                                   as
                                                                                                   libc::c_int)
                                                                                              as
                                                                                              libc::c_uint
                                                                                              >>
                                                                                              16
                                                                                                  as
                                                                                                  libc::c_int
                                                                                              >>
                                                                                              8
                                                                                                  as
                                                                                                  libc::c_int
                                                                                              >>
                                                                                              4
                                                                                                  as
                                                                                                  libc::c_int
                                                                                              >>
                                                                                              2
                                                                                                  as
                                                                                                  libc::c_int
                                                                                              &
                                                                                              0x2
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  libc::c_uint
                                                                                              !=
                                                                                              0)
                                                                                             as
                                                                                             libc::c_int
                                                                                 } else {
                                                                                     ((::std::mem::size_of::<libc::c_int>()
                                                                                           as
                                                                                           libc::c_ulong
                                                                                           as
                                                                                           libc::c_int
                                                                                           *
                                                                                           8
                                                                                               as
                                                                                               libc::c_int)
                                                                                          as
                                                                                          libc::c_uint
                                                                                          >>
                                                                                          16
                                                                                              as
                                                                                              libc::c_int
                                                                                          >>
                                                                                          8
                                                                                              as
                                                                                              libc::c_int
                                                                                          >>
                                                                                          4
                                                                                              as
                                                                                              libc::c_int
                                                                                          &
                                                                                          0x2
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_uint
                                                                                          !=
                                                                                          0)
                                                                                         as
                                                                                         libc::c_int
                                                                                 })
                                                                        } else {
                                                                            (if (::std::mem::size_of::<libc::c_int>()
                                                                                     as
                                                                                     libc::c_ulong
                                                                                     as
                                                                                     libc::c_int
                                                                                     *
                                                                                     8
                                                                                         as
                                                                                         libc::c_int)
                                                                                    as
                                                                                    libc::c_uint
                                                                                    >>
                                                                                    16
                                                                                        as
                                                                                        libc::c_int
                                                                                    >>
                                                                                    8
                                                                                        as
                                                                                        libc::c_int
                                                                                    &
                                                                                    0xc
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint
                                                                                    !=
                                                                                    0
                                                                                {
                                                                                 (2
                                                                                      as
                                                                                      libc::c_int)
                                                                                     +
                                                                                     ((::std::mem::size_of::<libc::c_int>()
                                                                                           as
                                                                                           libc::c_ulong
                                                                                           as
                                                                                           libc::c_int
                                                                                           *
                                                                                           8
                                                                                               as
                                                                                               libc::c_int)
                                                                                          as
                                                                                          libc::c_uint
                                                                                          >>
                                                                                          16
                                                                                              as
                                                                                              libc::c_int
                                                                                          >>
                                                                                          8
                                                                                              as
                                                                                              libc::c_int
                                                                                          >>
                                                                                          2
                                                                                              as
                                                                                              libc::c_int
                                                                                          &
                                                                                          0x2
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_uint
                                                                                          !=
                                                                                          0)
                                                                                         as
                                                                                         libc::c_int
                                                                             } else {
                                                                                 ((::std::mem::size_of::<libc::c_int>()
                                                                                       as
                                                                                       libc::c_ulong
                                                                                       as
                                                                                       libc::c_int
                                                                                       *
                                                                                       8
                                                                                           as
                                                                                           libc::c_int)
                                                                                      as
                                                                                      libc::c_uint
                                                                                      >>
                                                                                      16
                                                                                          as
                                                                                          libc::c_int
                                                                                      >>
                                                                                      8
                                                                                          as
                                                                                          libc::c_int
                                                                                      &
                                                                                      0x2
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_uint
                                                                                      !=
                                                                                      0)
                                                                                     as
                                                                                     libc::c_int
                                                                             })
                                                                        })
                                                               } else {
                                                                   (if (::std::mem::size_of::<libc::c_int>()
                                                                            as
                                                                            libc::c_ulong
                                                                            as
                                                                            libc::c_int
                                                                            *
                                                                            8
                                                                                as
                                                                                libc::c_int)
                                                                           as
                                                                           libc::c_uint
                                                                           >>
                                                                           16
                                                                               as
                                                                               libc::c_int
                                                                           &
                                                                           0xf0
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint
                                                                           !=
                                                                           0 {
                                                                        (4 as
                                                                             libc::c_int)
                                                                            +
                                                                            (if (::std::mem::size_of::<libc::c_int>()
                                                                                     as
                                                                                     libc::c_ulong
                                                                                     as
                                                                                     libc::c_int
                                                                                     *
                                                                                     8
                                                                                         as
                                                                                         libc::c_int)
                                                                                    as
                                                                                    libc::c_uint
                                                                                    >>
                                                                                    16
                                                                                        as
                                                                                        libc::c_int
                                                                                    >>
                                                                                    4
                                                                                        as
                                                                                        libc::c_int
                                                                                    &
                                                                                    0xc
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint
                                                                                    !=
                                                                                    0
                                                                                {
                                                                                 (2
                                                                                      as
                                                                                      libc::c_int)
                                                                                     +
                                                                                     ((::std::mem::size_of::<libc::c_int>()
                                                                                           as
                                                                                           libc::c_ulong
                                                                                           as
                                                                                           libc::c_int
                                                                                           *
                                                                                           8
                                                                                               as
                                                                                               libc::c_int)
                                                                                          as
                                                                                          libc::c_uint
                                                                                          >>
                                                                                          16
                                                                                              as
                                                                                              libc::c_int
                                                                                          >>
                                                                                          4
                                                                                              as
                                                                                              libc::c_int
                                                                                          >>
                                                                                          2
                                                                                              as
                                                                                              libc::c_int
                                                                                          &
                                                                                          0x2
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_uint
                                                                                          !=
                                                                                          0)
                                                                                         as
                                                                                         libc::c_int
                                                                             } else {
                                                                                 ((::std::mem::size_of::<libc::c_int>()
                                                                                       as
                                                                                       libc::c_ulong
                                                                                       as
                                                                                       libc::c_int
                                                                                       *
                                                                                       8
                                                                                           as
                                                                                           libc::c_int)
                                                                                      as
                                                                                      libc::c_uint
                                                                                      >>
                                                                                      16
                                                                                          as
                                                                                          libc::c_int
                                                                                      >>
                                                                                      4
                                                                                          as
                                                                                          libc::c_int
                                                                                      &
                                                                                      0x2
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_uint
                                                                                      !=
                                                                                      0)
                                                                                     as
                                                                                     libc::c_int
                                                                             })
                                                                    } else {
                                                                        (if (::std::mem::size_of::<libc::c_int>()
                                                                                 as
                                                                                 libc::c_ulong
                                                                                 as
                                                                                 libc::c_int
                                                                                 *
                                                                                 8
                                                                                     as
                                                                                     libc::c_int)
                                                                                as
                                                                                libc::c_uint
                                                                                >>
                                                                                16
                                                                                    as
                                                                                    libc::c_int
                                                                                &
                                                                                0xc
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint
                                                                                !=
                                                                                0
                                                                            {
                                                                             (2
                                                                                  as
                                                                                  libc::c_int)
                                                                                 +
                                                                                 ((::std::mem::size_of::<libc::c_int>()
                                                                                       as
                                                                                       libc::c_ulong
                                                                                       as
                                                                                       libc::c_int
                                                                                       *
                                                                                       8
                                                                                           as
                                                                                           libc::c_int)
                                                                                      as
                                                                                      libc::c_uint
                                                                                      >>
                                                                                      16
                                                                                          as
                                                                                          libc::c_int
                                                                                      >>
                                                                                      2
                                                                                          as
                                                                                          libc::c_int
                                                                                      &
                                                                                      0x2
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_uint
                                                                                      !=
                                                                                      0)
                                                                                     as
                                                                                     libc::c_int
                                                                         } else {
                                                                             ((::std::mem::size_of::<libc::c_int>()
                                                                                   as
                                                                                   libc::c_ulong
                                                                                   as
                                                                                   libc::c_int
                                                                                   *
                                                                                   8
                                                                                       as
                                                                                       libc::c_int)
                                                                                  as
                                                                                  libc::c_uint
                                                                                  >>
                                                                                  16
                                                                                      as
                                                                                      libc::c_int
                                                                                  &
                                                                                  0x2
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_uint
                                                                                  !=
                                                                                  0)
                                                                                 as
                                                                                 libc::c_int
                                                                         })
                                                                    })
                                                               })
                                                      } else {
                                                          (if (::std::mem::size_of::<libc::c_int>()
                                                                   as
                                                                   libc::c_ulong
                                                                   as
                                                                   libc::c_int
                                                                   *
                                                                   8 as
                                                                       libc::c_int)
                                                                  as
                                                                  libc::c_uint
                                                                  &
                                                                  0xff00 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint
                                                                  != 0 {
                                                               (8 as
                                                                    libc::c_int)
                                                                   +
                                                                   (if (::std::mem::size_of::<libc::c_int>()
                                                                            as
                                                                            libc::c_ulong
                                                                            as
                                                                            libc::c_int
                                                                            *
                                                                            8
                                                                                as
                                                                                libc::c_int)
                                                                           as
                                                                           libc::c_uint
                                                                           >>
                                                                           8
                                                                               as
                                                                               libc::c_int
                                                                           &
                                                                           0xf0
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint
                                                                           !=
                                                                           0 {
                                                                        (4 as
                                                                             libc::c_int)
                                                                            +
                                                                            (if (::std::mem::size_of::<libc::c_int>()
                                                                                     as
                                                                                     libc::c_ulong
                                                                                     as
                                                                                     libc::c_int
                                                                                     *
                                                                                     8
                                                                                         as
                                                                                         libc::c_int)
                                                                                    as
                                                                                    libc::c_uint
                                                                                    >>
                                                                                    8
                                                                                        as
                                                                                        libc::c_int
                                                                                    >>
                                                                                    4
                                                                                        as
                                                                                        libc::c_int
                                                                                    &
                                                                                    0xc
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint
                                                                                    !=
                                                                                    0
                                                                                {
                                                                                 (2
                                                                                      as
                                                                                      libc::c_int)
                                                                                     +
                                                                                     ((::std::mem::size_of::<libc::c_int>()
                                                                                           as
                                                                                           libc::c_ulong
                                                                                           as
                                                                                           libc::c_int
                                                                                           *
                                                                                           8
                                                                                               as
                                                                                               libc::c_int)
                                                                                          as
                                                                                          libc::c_uint
                                                                                          >>
                                                                                          8
                                                                                              as
                                                                                              libc::c_int
                                                                                          >>
                                                                                          4
                                                                                              as
                                                                                              libc::c_int
                                                                                          >>
                                                                                          2
                                                                                              as
                                                                                              libc::c_int
                                                                                          &
                                                                                          0x2
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_uint
                                                                                          !=
                                                                                          0)
                                                                                         as
                                                                                         libc::c_int
                                                                             } else {
                                                                                 ((::std::mem::size_of::<libc::c_int>()
                                                                                       as
                                                                                       libc::c_ulong
                                                                                       as
                                                                                       libc::c_int
                                                                                       *
                                                                                       8
                                                                                           as
                                                                                           libc::c_int)
                                                                                      as
                                                                                      libc::c_uint
                                                                                      >>
                                                                                      8
                                                                                          as
                                                                                          libc::c_int
                                                                                      >>
                                                                                      4
                                                                                          as
                                                                                          libc::c_int
                                                                                      &
                                                                                      0x2
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_uint
                                                                                      !=
                                                                                      0)
                                                                                     as
                                                                                     libc::c_int
                                                                             })
                                                                    } else {
                                                                        (if (::std::mem::size_of::<libc::c_int>()
                                                                                 as
                                                                                 libc::c_ulong
                                                                                 as
                                                                                 libc::c_int
                                                                                 *
                                                                                 8
                                                                                     as
                                                                                     libc::c_int)
                                                                                as
                                                                                libc::c_uint
                                                                                >>
                                                                                8
                                                                                    as
                                                                                    libc::c_int
                                                                                &
                                                                                0xc
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint
                                                                                !=
                                                                                0
                                                                            {
                                                                             (2
                                                                                  as
                                                                                  libc::c_int)
                                                                                 +
                                                                                 ((::std::mem::size_of::<libc::c_int>()
                                                                                       as
                                                                                       libc::c_ulong
                                                                                       as
                                                                                       libc::c_int
                                                                                       *
                                                                                       8
                                                                                           as
                                                                                           libc::c_int)
                                                                                      as
                                                                                      libc::c_uint
                                                                                      >>
                                                                                      8
                                                                                          as
                                                                                          libc::c_int
                                                                                      >>
                                                                                      2
                                                                                          as
                                                                                          libc::c_int
                                                                                      &
                                                                                      0x2
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_uint
                                                                                      !=
                                                                                      0)
                                                                                     as
                                                                                     libc::c_int
                                                                         } else {
                                                                             ((::std::mem::size_of::<libc::c_int>()
                                                                                   as
                                                                                   libc::c_ulong
                                                                                   as
                                                                                   libc::c_int
                                                                                   *
                                                                                   8
                                                                                       as
                                                                                       libc::c_int)
                                                                                  as
                                                                                  libc::c_uint
                                                                                  >>
                                                                                  8
                                                                                      as
                                                                                      libc::c_int
                                                                                  &
                                                                                  0x2
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_uint
                                                                                  !=
                                                                                  0)
                                                                                 as
                                                                                 libc::c_int
                                                                         })
                                                                    })
                                                           } else {
                                                               (if (::std::mem::size_of::<libc::c_int>()
                                                                        as
                                                                        libc::c_ulong
                                                                        as
                                                                        libc::c_int
                                                                        *
                                                                        8 as
                                                                            libc::c_int)
                                                                       as
                                                                       libc::c_uint
                                                                       &
                                                                       0xf0 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint
                                                                       != 0 {
                                                                    (4 as
                                                                         libc::c_int)
                                                                        +
                                                                        (if (::std::mem::size_of::<libc::c_int>()
                                                                                 as
                                                                                 libc::c_ulong
                                                                                 as
                                                                                 libc::c_int
                                                                                 *
                                                                                 8
                                                                                     as
                                                                                     libc::c_int)
                                                                                as
                                                                                libc::c_uint
                                                                                >>
                                                                                4
                                                                                    as
                                                                                    libc::c_int
                                                                                &
                                                                                0xc
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint
                                                                                !=
                                                                                0
                                                                            {
                                                                             (2
                                                                                  as
                                                                                  libc::c_int)
                                                                                 +
                                                                                 ((::std::mem::size_of::<libc::c_int>()
                                                                                       as
                                                                                       libc::c_ulong
                                                                                       as
                                                                                       libc::c_int
                                                                                       *
                                                                                       8
                                                                                           as
                                                                                           libc::c_int)
                                                                                      as
                                                                                      libc::c_uint
                                                                                      >>
                                                                                      4
                                                                                          as
                                                                                          libc::c_int
                                                                                      >>
                                                                                      2
                                                                                          as
                                                                                          libc::c_int
                                                                                      &
                                                                                      0x2
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_uint
                                                                                      !=
                                                                                      0)
                                                                                     as
                                                                                     libc::c_int
                                                                         } else {
                                                                             ((::std::mem::size_of::<libc::c_int>()
                                                                                   as
                                                                                   libc::c_ulong
                                                                                   as
                                                                                   libc::c_int
                                                                                   *
                                                                                   8
                                                                                       as
                                                                                       libc::c_int)
                                                                                  as
                                                                                  libc::c_uint
                                                                                  >>
                                                                                  4
                                                                                      as
                                                                                      libc::c_int
                                                                                  &
                                                                                  0x2
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_uint
                                                                                  !=
                                                                                  0)
                                                                                 as
                                                                                 libc::c_int
                                                                         })
                                                                } else {
                                                                    (if (::std::mem::size_of::<libc::c_int>()
                                                                             as
                                                                             libc::c_ulong
                                                                             as
                                                                             libc::c_int
                                                                             *
                                                                             8
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            libc::c_uint
                                                                            &
                                                                            0xc
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint
                                                                            !=
                                                                            0
                                                                        {
                                                                         (2 as
                                                                              libc::c_int)
                                                                             +
                                                                             ((::std::mem::size_of::<libc::c_int>()
                                                                                   as
                                                                                   libc::c_ulong
                                                                                   as
                                                                                   libc::c_int
                                                                                   *
                                                                                   8
                                                                                       as
                                                                                       libc::c_int)
                                                                                  as
                                                                                  libc::c_uint
                                                                                  >>
                                                                                  2
                                                                                      as
                                                                                      libc::c_int
                                                                                  &
                                                                                  0x2
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_uint
                                                                                  !=
                                                                                  0)
                                                                                 as
                                                                                 libc::c_int
                                                                     } else {
                                                                         ((::std::mem::size_of::<libc::c_int>()
                                                                               as
                                                                               libc::c_ulong
                                                                               as
                                                                               libc::c_int
                                                                               *
                                                                               8
                                                                                   as
                                                                                   libc::c_int)
                                                                              as
                                                                              libc::c_uint
                                                                              &
                                                                              0x2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_uint
                                                                              !=
                                                                              0)
                                                                             as
                                                                             libc::c_int
                                                                     })
                                                                })
                                                           })
                                                      }))) as isize) ^=
                            (qr_img_get_bit(_img, _width, _height,
                                            p[0 as libc::c_int as usize],
                                            p[1 as libc::c_int as usize]) <<
                                 (v &
                                      ::std::mem::size_of::<libc::c_int>() as
                                          libc::c_ulong as libc::c_int *
                                          8 as libc::c_int -
                                          1 as libc::c_int)) as libc::c_uint
                    }
                    x += (*cell).fwd[0 as libc::c_int as usize][1 as libc::c_int as usize];
                    y += (*cell).fwd[1 as libc::c_int as usize][1 as libc::c_int as usize];
                    w += (*cell).fwd[2 as libc::c_int as usize][1 as libc::c_int as usize];
                    v += 1
                }
                x0 += (*cell).fwd[0 as libc::c_int as usize][0 as libc::c_int as usize];
                y0 += (*cell).fwd[1 as libc::c_int as usize][0 as libc::c_int as usize];
                w0 += (*cell).fwd[2 as libc::c_int as usize][0 as libc::c_int as usize];
                u += 1
            }
            v0 = v1;
            i += 1
        }
        u0 = u1;
        j += 1
    }
}
/*Arranges the sample bits read by qr_sampling_grid_sample() into bytes and
 groups those bytes into Reed-Solomon blocks.
The individual block pointers are destroyed by this routine.*/
unsafe extern fn qr_samples_unpack(
    mut _blocks: *mut *mut libc::c_uchar,
    mut _nblocks: libc::c_int,
    mut _nshort_data: libc::c_int,
    mut _nshort_blocks: libc::c_int,
    mut _data_bits: *const libc::c_uint,
    mut _fp_mask: *const libc::c_uint,
    mut _dim: libc::c_int,
) {
    let mut bits: libc::c_uint = 0;
    let mut biti: libc::c_int = 0;
    let mut stride: libc::c_int = 0;
    let mut blocki: libc::c_int = 0;
    let mut blockj: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    stride = _dim
        + ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int * 8 as libc::c_int
        - 1 as libc::c_int
        >> (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
            * 8 as libc::c_int) as libc::c_uint
            & 0xffff0000 as libc::c_uint
            != 0
        {
            (16 as libc::c_int)
                + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                    * 8 as libc::c_int) as libc::c_uint
                    >> 16 as libc::c_int
                    & 0xff00 as libc::c_int as libc::c_uint
                    != 0
                {
                    (8 as libc::c_int)
                        + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int) as libc::c_uint
                            >> 16 as libc::c_int
                            >> 8 as libc::c_int
                            & 0xf0 as libc::c_int as libc::c_uint
                            != 0
                        {
                            (4 as libc::c_int)
                                + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 16 as libc::c_int
                                    >> 8 as libc::c_int
                                    >> 4 as libc::c_int
                                    & 0xc as libc::c_int as libc::c_uint
                                    != 0
                                {
                                    (2 as libc::c_int)
                                        + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 16 as libc::c_int
                                            >> 8 as libc::c_int
                                            >> 4 as libc::c_int
                                            >> 2 as libc::c_int
                                            & 0x2 as libc::c_int as libc::c_uint
                                            != 0)
                                            as libc::c_int
                                } else {
                                    ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 16 as libc::c_int
                                        >> 8 as libc::c_int
                                        >> 4 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                                })
                        } else {
                            (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int) as libc::c_uint
                                >> 16 as libc::c_int
                                >> 8 as libc::c_int
                                & 0xc as libc::c_int as libc::c_uint
                                != 0
                            {
                                (2 as libc::c_int)
                                    + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 16 as libc::c_int
                                        >> 8 as libc::c_int
                                        >> 2 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                            } else {
                                ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 16 as libc::c_int
                                    >> 8 as libc::c_int
                                    & 0x2 as libc::c_int as libc::c_uint
                                    != 0) as libc::c_int
                            })
                        })
                } else {
                    (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                        * 8 as libc::c_int) as libc::c_uint
                        >> 16 as libc::c_int
                        & 0xf0 as libc::c_int as libc::c_uint
                        != 0
                    {
                        (4 as libc::c_int)
                            + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int)
                                as libc::c_uint
                                >> 16 as libc::c_int
                                >> 4 as libc::c_int
                                & 0xc as libc::c_int as libc::c_uint
                                != 0
                            {
                                (2 as libc::c_int)
                                    + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 16 as libc::c_int
                                        >> 4 as libc::c_int
                                        >> 2 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                            } else {
                                ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 16 as libc::c_int
                                    >> 4 as libc::c_int
                                    & 0x2 as libc::c_int as libc::c_uint
                                    != 0) as libc::c_int
                            })
                    } else {
                        (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int) as libc::c_uint
                            >> 16 as libc::c_int
                            & 0xc as libc::c_int as libc::c_uint
                            != 0
                        {
                            (2 as libc::c_int)
                                + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 16 as libc::c_int
                                    >> 2 as libc::c_int
                                    & 0x2 as libc::c_int as libc::c_uint
                                    != 0) as libc::c_int
                        } else {
                            ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                                * 8 as libc::c_int) as libc::c_uint
                                >> 16 as libc::c_int
                                & 0x2 as libc::c_int as libc::c_uint
                                != 0) as libc::c_int
                        })
                    })
                })
        } else {
            (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                * 8 as libc::c_int) as libc::c_uint
                & 0xff00 as libc::c_int as libc::c_uint
                != 0
            {
                (8 as libc::c_int)
                    + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                        * 8 as libc::c_int) as libc::c_uint
                        >> 8 as libc::c_int
                        & 0xf0 as libc::c_int as libc::c_uint
                        != 0
                    {
                        (4 as libc::c_int)
                            + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int)
                                as libc::c_uint
                                >> 8 as libc::c_int
                                >> 4 as libc::c_int
                                & 0xc as libc::c_int as libc::c_uint
                                != 0
                            {
                                (2 as libc::c_int)
                                    + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 8 as libc::c_int
                                        >> 4 as libc::c_int
                                        >> 2 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                            } else {
                                ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 8 as libc::c_int
                                    >> 4 as libc::c_int
                                    & 0x2 as libc::c_int as libc::c_uint
                                    != 0) as libc::c_int
                            })
                    } else {
                        (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int) as libc::c_uint
                            >> 8 as libc::c_int
                            & 0xc as libc::c_int as libc::c_uint
                            != 0
                        {
                            (2 as libc::c_int)
                                + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 8 as libc::c_int
                                    >> 2 as libc::c_int
                                    & 0x2 as libc::c_int as libc::c_uint
                                    != 0) as libc::c_int
                        } else {
                            ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                                * 8 as libc::c_int) as libc::c_uint
                                >> 8 as libc::c_int
                                & 0x2 as libc::c_int as libc::c_uint
                                != 0) as libc::c_int
                        })
                    })
            } else {
                (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                    * 8 as libc::c_int) as libc::c_uint
                    & 0xf0 as libc::c_int as libc::c_uint
                    != 0
                {
                    (4 as libc::c_int)
                        + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int) as libc::c_uint
                            >> 4 as libc::c_int
                            & 0xc as libc::c_int as libc::c_uint
                            != 0
                        {
                            (2 as libc::c_int)
                                + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 4 as libc::c_int
                                    >> 2 as libc::c_int
                                    & 0x2 as libc::c_int as libc::c_uint
                                    != 0) as libc::c_int
                        } else {
                            ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                                * 8 as libc::c_int) as libc::c_uint
                                >> 4 as libc::c_int
                                & 0x2 as libc::c_int as libc::c_uint
                                != 0) as libc::c_int
                        })
                } else {
                    (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                        * 8 as libc::c_int) as libc::c_uint
                        & 0xc as libc::c_int as libc::c_uint
                        != 0
                    {
                        (2 as libc::c_int)
                            + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int) as libc::c_uint
                                >> 2 as libc::c_int
                                & 0x2 as libc::c_int as libc::c_uint
                                != 0) as libc::c_int
                    } else {
                        ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int) as libc::c_uint
                            & 0x2 as libc::c_int as libc::c_uint
                            != 0) as libc::c_int
                    })
                })
            })
        });
    /* If _all_ the blocks are short, don't skip anything (see below). */
    if _nshort_blocks >= _nblocks {
        _nshort_blocks = 0 as libc::c_int
    }
    /* Scan columns in pairs from right to left. */
    bits = 0 as libc::c_int as libc::c_uint;
    biti = 0 as libc::c_int;
    blockj = biti;
    blocki = blockj;
    j = _dim - 1 as libc::c_int;
    while j > 0 as libc::c_int {
        let mut data1: libc::c_uint = 0;
        let mut data2: libc::c_uint = 0;
        let mut fp_mask1: libc::c_uint = 0;
        let mut fp_mask2: libc::c_uint = 0;
        let mut nbits: libc::c_int = 0;
        let mut l: libc::c_int = 0;
        /* Scan up a pair of columns. */
        nbits = (_dim - 1 as libc::c_int
            & ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                * 8 as libc::c_int
                - 1 as libc::c_int)
            + 1 as libc::c_int;
        l = j * stride;
        i = stride;
        loop {
            let fresh14 = i;
            i = i - 1;
            if !(fresh14 > 0 as libc::c_int) {
                break;
            }
            data1 = *_data_bits.offset((l + i) as isize);
            fp_mask1 = *_fp_mask.offset((l + i) as isize);
            data2 = *_data_bits.offset((l + i - stride) as isize);
            fp_mask2 = *_fp_mask.offset((l + i - stride) as isize);
            loop {
                let fresh15 = nbits;
                nbits = nbits - 1;
                if !(fresh15 > 0 as libc::c_int) {
                    break;
                }
                /* Pull a bit from the right column. */
                if fp_mask1 >> nbits & 1 as libc::c_int as libc::c_uint == 0 {
                    bits = bits << 1 as libc::c_int
                        | data1 >> nbits & 1 as libc::c_int as libc::c_uint;
                    biti += 1
                }
                /* Pull a bit from the left column. */
                if fp_mask2 >> nbits & 1 as libc::c_int as libc::c_uint == 0 {
                    bits = bits << 1 as libc::c_int
                        | data2 >> nbits & 1 as libc::c_int as libc::c_uint;
                    biti += 1
                }
                /* If we finished a byte, drop it in a block. */
                if biti >= 8 as libc::c_int {
                    biti -= 8 as libc::c_int;
                    let fresh16 = blocki;
                    blocki = blocki + 1;
                    let ref mut fresh17 = *_blocks.offset(fresh16 as isize);
                    let fresh18 = *fresh17;
                    *fresh17 = (*fresh17).offset(1);
                    *fresh18 = (bits >> biti) as libc::c_uchar;
                    /*For whatever reason, the long blocks are at the _end_ of the list,
                     instead of the beginning.
                    Even worse, the extra bytes they get come at the end of the data
                     bytes, before the parity bytes.
                    Hence the logic here: when we've filled up the data portion of the
                     short blocks, skip directly to the long blocks for the next byte.
                    It's also the reason we increment _blocks[blocki] on each store,
                     instead of just indexing with blockj (after this iteration the
                     number of bytes in each block differs).*/
                    if blocki >= _nblocks {
                        blockj += 1;
                        blocki = if blockj == _nshort_data {
                            _nshort_blocks
                        } else {
                            0 as libc::c_int
                        }
                    }
                }
            }
            nbits = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                * 8 as libc::c_int
        }
        j -= 2 as libc::c_int;
        /* Skip the column with the vertical timing pattern. */
        if j == 6 as libc::c_int {
            j -= 1
        }
        /* Scan down a pair of columns. */
        l = j * stride;
        i = 0 as libc::c_int;
        while i < stride {
            data1 = *_data_bits.offset((l + i) as isize);
            fp_mask1 = *_fp_mask.offset((l + i) as isize);
            data2 = *_data_bits.offset((l + i - stride) as isize);
            fp_mask2 = *_fp_mask.offset((l + i - stride) as isize);
            nbits = _dim
                - (i << (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                    * 8 as libc::c_int) as libc::c_uint
                    & 0xffff0000 as libc::c_uint
                    != 0
                {
                    (16 as libc::c_int)
                        + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int) as libc::c_uint
                            >> 16 as libc::c_int
                            & 0xff00 as libc::c_int as libc::c_uint
                            != 0
                        {
                            (8 as libc::c_int)
                                + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 16 as libc::c_int
                                    >> 8 as libc::c_int
                                    & 0xf0 as libc::c_int as libc::c_uint
                                    != 0
                                {
                                    (4 as libc::c_int)
                                        + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 16 as libc::c_int
                                            >> 8 as libc::c_int
                                            >> 4 as libc::c_int
                                            & 0xc as libc::c_int as libc::c_uint
                                            != 0
                                        {
                                            (2 as libc::c_int)
                                                + ((::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong
                                                    as libc::c_int
                                                    * 8 as libc::c_int)
                                                    as libc::c_uint
                                                    >> 16 as libc::c_int
                                                    >> 8 as libc::c_int
                                                    >> 4 as libc::c_int
                                                    >> 2 as libc::c_int
                                                    & 0x2 as libc::c_int as libc::c_uint
                                                    != 0)
                                                    as libc::c_int
                                        } else {
                                            ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                                as libc::c_int
                                                * 8 as libc::c_int)
                                                as libc::c_uint
                                                >> 16 as libc::c_int
                                                >> 8 as libc::c_int
                                                >> 4 as libc::c_int
                                                & 0x2 as libc::c_int as libc::c_uint
                                                != 0)
                                                as libc::c_int
                                        })
                                } else {
                                    (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 16 as libc::c_int
                                        >> 8 as libc::c_int
                                        & 0xc as libc::c_int as libc::c_uint
                                        != 0
                                    {
                                        (2 as libc::c_int)
                                            + ((::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong
                                                as libc::c_int
                                                * 8 as libc::c_int)
                                                as libc::c_uint
                                                >> 16 as libc::c_int
                                                >> 8 as libc::c_int
                                                >> 2 as libc::c_int
                                                & 0x2 as libc::c_int as libc::c_uint
                                                != 0)
                                                as libc::c_int
                                    } else {
                                        ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 16 as libc::c_int
                                            >> 8 as libc::c_int
                                            & 0x2 as libc::c_int as libc::c_uint
                                            != 0)
                                            as libc::c_int
                                    })
                                })
                        } else {
                            (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int) as libc::c_uint
                                >> 16 as libc::c_int
                                & 0xf0 as libc::c_int as libc::c_uint
                                != 0
                            {
                                (4 as libc::c_int)
                                    + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 16 as libc::c_int
                                        >> 4 as libc::c_int
                                        & 0xc as libc::c_int as libc::c_uint
                                        != 0
                                    {
                                        (2 as libc::c_int)
                                            + ((::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong
                                                as libc::c_int
                                                * 8 as libc::c_int)
                                                as libc::c_uint
                                                >> 16 as libc::c_int
                                                >> 4 as libc::c_int
                                                >> 2 as libc::c_int
                                                & 0x2 as libc::c_int as libc::c_uint
                                                != 0)
                                                as libc::c_int
                                    } else {
                                        ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 16 as libc::c_int
                                            >> 4 as libc::c_int
                                            & 0x2 as libc::c_int as libc::c_uint
                                            != 0)
                                            as libc::c_int
                                    })
                            } else {
                                (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 16 as libc::c_int
                                    & 0xc as libc::c_int as libc::c_uint
                                    != 0
                                {
                                    (2 as libc::c_int)
                                        + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 16 as libc::c_int
                                            >> 2 as libc::c_int
                                            & 0x2 as libc::c_int as libc::c_uint
                                            != 0)
                                            as libc::c_int
                                } else {
                                    ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 16 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                                })
                            })
                        })
                } else {
                    (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                        * 8 as libc::c_int) as libc::c_uint
                        & 0xff00 as libc::c_int as libc::c_uint
                        != 0
                    {
                        (8 as libc::c_int)
                            + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int)
                                as libc::c_uint
                                >> 8 as libc::c_int
                                & 0xf0 as libc::c_int as libc::c_uint
                                != 0
                            {
                                (4 as libc::c_int)
                                    + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 8 as libc::c_int
                                        >> 4 as libc::c_int
                                        & 0xc as libc::c_int as libc::c_uint
                                        != 0
                                    {
                                        (2 as libc::c_int)
                                            + ((::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong
                                                as libc::c_int
                                                * 8 as libc::c_int)
                                                as libc::c_uint
                                                >> 8 as libc::c_int
                                                >> 4 as libc::c_int
                                                >> 2 as libc::c_int
                                                & 0x2 as libc::c_int as libc::c_uint
                                                != 0)
                                                as libc::c_int
                                    } else {
                                        ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 8 as libc::c_int
                                            >> 4 as libc::c_int
                                            & 0x2 as libc::c_int as libc::c_uint
                                            != 0)
                                            as libc::c_int
                                    })
                            } else {
                                (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 8 as libc::c_int
                                    & 0xc as libc::c_int as libc::c_uint
                                    != 0
                                {
                                    (2 as libc::c_int)
                                        + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 8 as libc::c_int
                                            >> 2 as libc::c_int
                                            & 0x2 as libc::c_int as libc::c_uint
                                            != 0)
                                            as libc::c_int
                                } else {
                                    ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 8 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                                })
                            })
                    } else {
                        (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int) as libc::c_uint
                            & 0xf0 as libc::c_int as libc::c_uint
                            != 0
                        {
                            (4 as libc::c_int)
                                + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 4 as libc::c_int
                                    & 0xc as libc::c_int as libc::c_uint
                                    != 0
                                {
                                    (2 as libc::c_int)
                                        + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 4 as libc::c_int
                                            >> 2 as libc::c_int
                                            & 0x2 as libc::c_int as libc::c_uint
                                            != 0)
                                            as libc::c_int
                                } else {
                                    ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 4 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                                })
                        } else {
                            (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int) as libc::c_uint
                                & 0xc as libc::c_int as libc::c_uint
                                != 0
                            {
                                (2 as libc::c_int)
                                    + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 2 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                            } else {
                                ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    & 0x2 as libc::c_int as libc::c_uint
                                    != 0) as libc::c_int
                            })
                        })
                    })
                }))
                + (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                    * 8 as libc::c_int
                    - (_dim
                        - (i << (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                            as libc::c_int
                            * 8 as libc::c_int) as libc::c_uint
                            & 0xffff0000 as libc::c_uint
                            != 0
                        {
                            (16 as libc::c_int)
                                + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 16 as libc::c_int
                                    & 0xff00 as libc::c_int as libc::c_uint
                                    != 0
                                {
                                    (8 as libc::c_int)
                                        + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 16 as libc::c_int
                                            >> 8 as libc::c_int
                                            & 0xf0 as libc::c_int as libc::c_uint
                                            != 0
                                        {
                                            (4 as libc::c_int)
                                                + (if (::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong
                                                    as libc::c_int
                                                    * 8 as libc::c_int)
                                                    as libc::c_uint
                                                    >> 16 as libc::c_int
                                                    >> 8 as libc::c_int
                                                    >> 4 as libc::c_int
                                                    & 0xc as libc::c_int as libc::c_uint
                                                    != 0
                                                {
                                                    (2 as libc::c_int)
                                                        + ((::std::mem::size_of::<libc::c_int>()
                                                            as libc::c_ulong
                                                            as libc::c_int
                                                            * 8 as libc::c_int)
                                                            as libc::c_uint
                                                            >> 16 as libc::c_int
                                                            >> 8 as libc::c_int
                                                            >> 4 as libc::c_int
                                                            >> 2 as libc::c_int
                                                            & 0x2 as libc::c_int as libc::c_uint
                                                            != 0)
                                                            as libc::c_int
                                                } else {
                                                    ((::std::mem::size_of::<libc::c_int>()
                                                        as libc::c_ulong
                                                        as libc::c_int
                                                        * 8 as libc::c_int)
                                                        as libc::c_uint
                                                        >> 16 as libc::c_int
                                                        >> 8 as libc::c_int
                                                        >> 4 as libc::c_int
                                                        & 0x2 as libc::c_int as libc::c_uint
                                                        != 0)
                                                        as libc::c_int
                                                })
                                        } else {
                                            (if (::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong
                                                as libc::c_int
                                                * 8 as libc::c_int)
                                                as libc::c_uint
                                                >> 16 as libc::c_int
                                                >> 8 as libc::c_int
                                                & 0xc as libc::c_int as libc::c_uint
                                                != 0
                                            {
                                                (2 as libc::c_int)
                                                    + ((::std::mem::size_of::<libc::c_int>()
                                                        as libc::c_ulong
                                                        as libc::c_int
                                                        * 8 as libc::c_int)
                                                        as libc::c_uint
                                                        >> 16 as libc::c_int
                                                        >> 8 as libc::c_int
                                                        >> 2 as libc::c_int
                                                        & 0x2 as libc::c_int as libc::c_uint
                                                        != 0)
                                                        as libc::c_int
                                            } else {
                                                ((::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong
                                                    as libc::c_int
                                                    * 8 as libc::c_int)
                                                    as libc::c_uint
                                                    >> 16 as libc::c_int
                                                    >> 8 as libc::c_int
                                                    & 0x2 as libc::c_int as libc::c_uint
                                                    != 0)
                                                    as libc::c_int
                                            })
                                        })
                                } else {
                                    (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 16 as libc::c_int
                                        & 0xf0 as libc::c_int as libc::c_uint
                                        != 0
                                    {
                                        (4 as libc::c_int)
                                            + (if (::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong
                                                as libc::c_int
                                                * 8 as libc::c_int)
                                                as libc::c_uint
                                                >> 16 as libc::c_int
                                                >> 4 as libc::c_int
                                                & 0xc as libc::c_int as libc::c_uint
                                                != 0
                                            {
                                                (2 as libc::c_int)
                                                    + ((::std::mem::size_of::<libc::c_int>()
                                                        as libc::c_ulong
                                                        as libc::c_int
                                                        * 8 as libc::c_int)
                                                        as libc::c_uint
                                                        >> 16 as libc::c_int
                                                        >> 4 as libc::c_int
                                                        >> 2 as libc::c_int
                                                        & 0x2 as libc::c_int as libc::c_uint
                                                        != 0)
                                                        as libc::c_int
                                            } else {
                                                ((::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong
                                                    as libc::c_int
                                                    * 8 as libc::c_int)
                                                    as libc::c_uint
                                                    >> 16 as libc::c_int
                                                    >> 4 as libc::c_int
                                                    & 0x2 as libc::c_int as libc::c_uint
                                                    != 0)
                                                    as libc::c_int
                                            })
                                    } else {
                                        (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 16 as libc::c_int
                                            & 0xc as libc::c_int as libc::c_uint
                                            != 0
                                        {
                                            (2 as libc::c_int)
                                                + ((::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong
                                                    as libc::c_int
                                                    * 8 as libc::c_int)
                                                    as libc::c_uint
                                                    >> 16 as libc::c_int
                                                    >> 2 as libc::c_int
                                                    & 0x2 as libc::c_int as libc::c_uint
                                                    != 0)
                                                    as libc::c_int
                                        } else {
                                            ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                                as libc::c_int
                                                * 8 as libc::c_int)
                                                as libc::c_uint
                                                >> 16 as libc::c_int
                                                & 0x2 as libc::c_int as libc::c_uint
                                                != 0)
                                                as libc::c_int
                                        })
                                    })
                                })
                        } else {
                            (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int) as libc::c_uint
                                & 0xff00 as libc::c_int as libc::c_uint
                                != 0
                            {
                                (8 as libc::c_int)
                                    + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 8 as libc::c_int
                                        & 0xf0 as libc::c_int as libc::c_uint
                                        != 0
                                    {
                                        (4 as libc::c_int)
                                            + (if (::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong
                                                as libc::c_int
                                                * 8 as libc::c_int)
                                                as libc::c_uint
                                                >> 8 as libc::c_int
                                                >> 4 as libc::c_int
                                                & 0xc as libc::c_int as libc::c_uint
                                                != 0
                                            {
                                                (2 as libc::c_int)
                                                    + ((::std::mem::size_of::<libc::c_int>()
                                                        as libc::c_ulong
                                                        as libc::c_int
                                                        * 8 as libc::c_int)
                                                        as libc::c_uint
                                                        >> 8 as libc::c_int
                                                        >> 4 as libc::c_int
                                                        >> 2 as libc::c_int
                                                        & 0x2 as libc::c_int as libc::c_uint
                                                        != 0)
                                                        as libc::c_int
                                            } else {
                                                ((::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong
                                                    as libc::c_int
                                                    * 8 as libc::c_int)
                                                    as libc::c_uint
                                                    >> 8 as libc::c_int
                                                    >> 4 as libc::c_int
                                                    & 0x2 as libc::c_int as libc::c_uint
                                                    != 0)
                                                    as libc::c_int
                                            })
                                    } else {
                                        (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 8 as libc::c_int
                                            & 0xc as libc::c_int as libc::c_uint
                                            != 0
                                        {
                                            (2 as libc::c_int)
                                                + ((::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong
                                                    as libc::c_int
                                                    * 8 as libc::c_int)
                                                    as libc::c_uint
                                                    >> 8 as libc::c_int
                                                    >> 2 as libc::c_int
                                                    & 0x2 as libc::c_int as libc::c_uint
                                                    != 0)
                                                    as libc::c_int
                                        } else {
                                            ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                                as libc::c_int
                                                * 8 as libc::c_int)
                                                as libc::c_uint
                                                >> 8 as libc::c_int
                                                & 0x2 as libc::c_int as libc::c_uint
                                                != 0)
                                                as libc::c_int
                                        })
                                    })
                            } else {
                                (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    & 0xf0 as libc::c_int as libc::c_uint
                                    != 0
                                {
                                    (4 as libc::c_int)
                                        + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 4 as libc::c_int
                                            & 0xc as libc::c_int as libc::c_uint
                                            != 0
                                        {
                                            (2 as libc::c_int)
                                                + ((::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong
                                                    as libc::c_int
                                                    * 8 as libc::c_int)
                                                    as libc::c_uint
                                                    >> 4 as libc::c_int
                                                    >> 2 as libc::c_int
                                                    & 0x2 as libc::c_int as libc::c_uint
                                                    != 0)
                                                    as libc::c_int
                                        } else {
                                            ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                                as libc::c_int
                                                * 8 as libc::c_int)
                                                as libc::c_uint
                                                >> 4 as libc::c_int
                                                & 0x2 as libc::c_int as libc::c_uint
                                                != 0)
                                                as libc::c_int
                                        })
                                } else {
                                    (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        & 0xc as libc::c_int as libc::c_uint
                                        != 0
                                    {
                                        (2 as libc::c_int)
                                            + ((::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong
                                                as libc::c_int
                                                * 8 as libc::c_int)
                                                as libc::c_uint
                                                >> 2 as libc::c_int
                                                & 0x2 as libc::c_int as libc::c_uint
                                                != 0)
                                                as libc::c_int
                                    } else {
                                        ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            & 0x2 as libc::c_int as libc::c_uint
                                            != 0)
                                            as libc::c_int
                                    })
                                })
                            })
                        })))
                    & -(((::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                        * 8 as libc::c_int)
                        < _dim
                            - (i << (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int)
                                as libc::c_uint
                                & 0xffff0000 as libc::c_uint
                                != 0
                            {
                                (16 as libc::c_int)
                                    + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 16 as libc::c_int
                                        & 0xff00 as libc::c_int as libc::c_uint
                                        != 0
                                    {
                                        (8 as libc::c_int)
                                            + (if (::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong
                                                as libc::c_int
                                                * 8 as libc::c_int)
                                                as libc::c_uint
                                                >> 16 as libc::c_int
                                                >> 8 as libc::c_int
                                                & 0xf0 as libc::c_int as libc::c_uint
                                                != 0
                                            {
                                                (4 as libc::c_int)
                                                    + (if (::std::mem::size_of::<libc::c_int>()
                                                        as libc::c_ulong
                                                        as libc::c_int
                                                        * 8 as libc::c_int)
                                                        as libc::c_uint
                                                        >> 16 as libc::c_int
                                                        >> 8 as libc::c_int
                                                        >> 4 as libc::c_int
                                                        & 0xc as libc::c_int as libc::c_uint
                                                        != 0
                                                    {
                                                        (2 as libc::c_int)
                                                            + ((::std::mem::size_of::<libc::c_int>()
                                                                as libc::c_ulong
                                                                as libc::c_int
                                                                * 8 as libc::c_int)
                                                                as libc::c_uint
                                                                >> 16 as libc::c_int
                                                                >> 8 as libc::c_int
                                                                >> 4 as libc::c_int
                                                                >> 2 as libc::c_int
                                                                & 0x2 as libc::c_int
                                                                    as libc::c_uint
                                                                != 0)
                                                                as libc::c_int
                                                    } else {
                                                        ((::std::mem::size_of::<libc::c_int>()
                                                            as libc::c_ulong
                                                            as libc::c_int
                                                            * 8 as libc::c_int)
                                                            as libc::c_uint
                                                            >> 16 as libc::c_int
                                                            >> 8 as libc::c_int
                                                            >> 4 as libc::c_int
                                                            & 0x2 as libc::c_int as libc::c_uint
                                                            != 0)
                                                            as libc::c_int
                                                    })
                                            } else {
                                                (if (::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong
                                                    as libc::c_int
                                                    * 8 as libc::c_int)
                                                    as libc::c_uint
                                                    >> 16 as libc::c_int
                                                    >> 8 as libc::c_int
                                                    & 0xc as libc::c_int as libc::c_uint
                                                    != 0
                                                {
                                                    (2 as libc::c_int)
                                                        + ((::std::mem::size_of::<libc::c_int>()
                                                            as libc::c_ulong
                                                            as libc::c_int
                                                            * 8 as libc::c_int)
                                                            as libc::c_uint
                                                            >> 16 as libc::c_int
                                                            >> 8 as libc::c_int
                                                            >> 2 as libc::c_int
                                                            & 0x2 as libc::c_int as libc::c_uint
                                                            != 0)
                                                            as libc::c_int
                                                } else {
                                                    ((::std::mem::size_of::<libc::c_int>()
                                                        as libc::c_ulong
                                                        as libc::c_int
                                                        * 8 as libc::c_int)
                                                        as libc::c_uint
                                                        >> 16 as libc::c_int
                                                        >> 8 as libc::c_int
                                                        & 0x2 as libc::c_int as libc::c_uint
                                                        != 0)
                                                        as libc::c_int
                                                })
                                            })
                                    } else {
                                        (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 16 as libc::c_int
                                            & 0xf0 as libc::c_int as libc::c_uint
                                            != 0
                                        {
                                            (4 as libc::c_int)
                                                + (if (::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong
                                                    as libc::c_int
                                                    * 8 as libc::c_int)
                                                    as libc::c_uint
                                                    >> 16 as libc::c_int
                                                    >> 4 as libc::c_int
                                                    & 0xc as libc::c_int as libc::c_uint
                                                    != 0
                                                {
                                                    (2 as libc::c_int)
                                                        + ((::std::mem::size_of::<libc::c_int>()
                                                            as libc::c_ulong
                                                            as libc::c_int
                                                            * 8 as libc::c_int)
                                                            as libc::c_uint
                                                            >> 16 as libc::c_int
                                                            >> 4 as libc::c_int
                                                            >> 2 as libc::c_int
                                                            & 0x2 as libc::c_int as libc::c_uint
                                                            != 0)
                                                            as libc::c_int
                                                } else {
                                                    ((::std::mem::size_of::<libc::c_int>()
                                                        as libc::c_ulong
                                                        as libc::c_int
                                                        * 8 as libc::c_int)
                                                        as libc::c_uint
                                                        >> 16 as libc::c_int
                                                        >> 4 as libc::c_int
                                                        & 0x2 as libc::c_int as libc::c_uint
                                                        != 0)
                                                        as libc::c_int
                                                })
                                        } else {
                                            (if (::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong
                                                as libc::c_int
                                                * 8 as libc::c_int)
                                                as libc::c_uint
                                                >> 16 as libc::c_int
                                                & 0xc as libc::c_int as libc::c_uint
                                                != 0
                                            {
                                                (2 as libc::c_int)
                                                    + ((::std::mem::size_of::<libc::c_int>()
                                                        as libc::c_ulong
                                                        as libc::c_int
                                                        * 8 as libc::c_int)
                                                        as libc::c_uint
                                                        >> 16 as libc::c_int
                                                        >> 2 as libc::c_int
                                                        & 0x2 as libc::c_int as libc::c_uint
                                                        != 0)
                                                        as libc::c_int
                                            } else {
                                                ((::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong
                                                    as libc::c_int
                                                    * 8 as libc::c_int)
                                                    as libc::c_uint
                                                    >> 16 as libc::c_int
                                                    & 0x2 as libc::c_int as libc::c_uint
                                                    != 0)
                                                    as libc::c_int
                                            })
                                        })
                                    })
                            } else {
                                (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    & 0xff00 as libc::c_int as libc::c_uint
                                    != 0
                                {
                                    (8 as libc::c_int)
                                        + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 8 as libc::c_int
                                            & 0xf0 as libc::c_int as libc::c_uint
                                            != 0
                                        {
                                            (4 as libc::c_int)
                                                + (if (::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong
                                                    as libc::c_int
                                                    * 8 as libc::c_int)
                                                    as libc::c_uint
                                                    >> 8 as libc::c_int
                                                    >> 4 as libc::c_int
                                                    & 0xc as libc::c_int as libc::c_uint
                                                    != 0
                                                {
                                                    (2 as libc::c_int)
                                                        + ((::std::mem::size_of::<libc::c_int>()
                                                            as libc::c_ulong
                                                            as libc::c_int
                                                            * 8 as libc::c_int)
                                                            as libc::c_uint
                                                            >> 8 as libc::c_int
                                                            >> 4 as libc::c_int
                                                            >> 2 as libc::c_int
                                                            & 0x2 as libc::c_int as libc::c_uint
                                                            != 0)
                                                            as libc::c_int
                                                } else {
                                                    ((::std::mem::size_of::<libc::c_int>()
                                                        as libc::c_ulong
                                                        as libc::c_int
                                                        * 8 as libc::c_int)
                                                        as libc::c_uint
                                                        >> 8 as libc::c_int
                                                        >> 4 as libc::c_int
                                                        & 0x2 as libc::c_int as libc::c_uint
                                                        != 0)
                                                        as libc::c_int
                                                })
                                        } else {
                                            (if (::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong
                                                as libc::c_int
                                                * 8 as libc::c_int)
                                                as libc::c_uint
                                                >> 8 as libc::c_int
                                                & 0xc as libc::c_int as libc::c_uint
                                                != 0
                                            {
                                                (2 as libc::c_int)
                                                    + ((::std::mem::size_of::<libc::c_int>()
                                                        as libc::c_ulong
                                                        as libc::c_int
                                                        * 8 as libc::c_int)
                                                        as libc::c_uint
                                                        >> 8 as libc::c_int
                                                        >> 2 as libc::c_int
                                                        & 0x2 as libc::c_int as libc::c_uint
                                                        != 0)
                                                        as libc::c_int
                                            } else {
                                                ((::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong
                                                    as libc::c_int
                                                    * 8 as libc::c_int)
                                                    as libc::c_uint
                                                    >> 8 as libc::c_int
                                                    & 0x2 as libc::c_int as libc::c_uint
                                                    != 0)
                                                    as libc::c_int
                                            })
                                        })
                                } else {
                                    (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        & 0xf0 as libc::c_int as libc::c_uint
                                        != 0
                                    {
                                        (4 as libc::c_int)
                                            + (if (::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong
                                                as libc::c_int
                                                * 8 as libc::c_int)
                                                as libc::c_uint
                                                >> 4 as libc::c_int
                                                & 0xc as libc::c_int as libc::c_uint
                                                != 0
                                            {
                                                (2 as libc::c_int)
                                                    + ((::std::mem::size_of::<libc::c_int>()
                                                        as libc::c_ulong
                                                        as libc::c_int
                                                        * 8 as libc::c_int)
                                                        as libc::c_uint
                                                        >> 4 as libc::c_int
                                                        >> 2 as libc::c_int
                                                        & 0x2 as libc::c_int as libc::c_uint
                                                        != 0)
                                                        as libc::c_int
                                            } else {
                                                ((::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong
                                                    as libc::c_int
                                                    * 8 as libc::c_int)
                                                    as libc::c_uint
                                                    >> 4 as libc::c_int
                                                    & 0x2 as libc::c_int as libc::c_uint
                                                    != 0)
                                                    as libc::c_int
                                            })
                                    } else {
                                        (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            & 0xc as libc::c_int as libc::c_uint
                                            != 0
                                        {
                                            (2 as libc::c_int)
                                                + ((::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong
                                                    as libc::c_int
                                                    * 8 as libc::c_int)
                                                    as libc::c_uint
                                                    >> 2 as libc::c_int
                                                    & 0x2 as libc::c_int as libc::c_uint
                                                    != 0)
                                                    as libc::c_int
                                        } else {
                                            ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                                as libc::c_int
                                                * 8 as libc::c_int)
                                                as libc::c_uint
                                                & 0x2 as libc::c_int as libc::c_uint
                                                != 0)
                                                as libc::c_int
                                        })
                                    })
                                })
                            }))) as libc::c_int));
            loop {
                let fresh19 = nbits;
                nbits = nbits - 1;
                if !(fresh19 > 0 as libc::c_int) {
                    break;
                }
                /* Pull a bit from the right column. */
                if fp_mask1 & 1 as libc::c_int as libc::c_uint == 0 {
                    bits = bits << 1 as libc::c_int | data1 & 1 as libc::c_int as libc::c_uint;
                    biti += 1
                }
                data1 >>= 1 as libc::c_int;
                fp_mask1 >>= 1 as libc::c_int;
                /* Pull a bit from the left column. */
                if fp_mask2 & 1 as libc::c_int as libc::c_uint == 0 {
                    bits = bits << 1 as libc::c_int | data2 & 1 as libc::c_int as libc::c_uint;
                    biti += 1
                }
                data2 >>= 1 as libc::c_int;
                fp_mask2 >>= 1 as libc::c_int;
                /* If we finished a byte, drop it in a block. */
                if biti >= 8 as libc::c_int {
                    biti -= 8 as libc::c_int;
                    let fresh20 = blocki;
                    blocki = blocki + 1;
                    let ref mut fresh21 = *_blocks.offset(fresh20 as isize);
                    let fresh22 = *fresh21;
                    *fresh21 = (*fresh21).offset(1);
                    *fresh22 = (bits >> biti) as libc::c_uchar;
                    /* See comments on the "up" loop for the reason behind this mess. */
                    if blocki >= _nblocks {
                        blockj += 1;
                        blocki = if blockj == _nshort_data {
                            _nshort_blocks
                        } else {
                            0 as libc::c_int
                        }
                    }
                }
            }
            i += 1
        }
        j -= 2 as libc::c_int
    }
}
unsafe extern fn qr_pack_buf_init(
    mut _b: *mut qr_pack_buf,
    mut _data: *const libc::c_uchar,
    mut _ndata: libc::c_int,
) {
    (*_b).buf = _data;
    (*_b).storage = _ndata;
    (*_b).endbit = 0 as libc::c_int;
    (*_b).endbyte = (*_b).endbit;
}
/* Assumes 0<=_bits<=16. */
unsafe extern fn qr_pack_buf_read(mut _b: *mut qr_pack_buf, mut _bits: libc::c_int) -> libc::c_int {
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut ret: libc::c_uint = 0;
    let mut m: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    m = 16 as libc::c_int - _bits;
    _bits += (*_b).endbit;
    d = (*_b).storage - (*_b).endbyte;
    if d <= 2 as libc::c_int {
        /* Not the main path. */
        if (d * 8 as libc::c_int) < _bits {
            (*_b).endbyte += _bits >> 3 as libc::c_int;
            (*_b).endbit = _bits & 7 as libc::c_int;
            return -(1 as libc::c_int);
        } else {
            /*Special case to avoid reading p[0] below, which might be past the end of
            the buffer; also skips some useless accounting.*/
            if _bits == 0 {
                return 0 as libc::c_int;
            }
        }
    }
    p = (*_b).buf.offset((*_b).endbyte as isize);
    ret = ((*p.offset(0 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int + (*_b).endbit)
        as libc::c_uint;
    if _bits > 8 as libc::c_int {
        ret |=
            ((*p.offset(1 as libc::c_int as isize) as libc::c_int) << (*_b).endbit) as libc::c_uint;
        if _bits > 16 as libc::c_int {
            ret |= (*p.offset(2 as libc::c_int as isize) as libc::c_int
                >> 8 as libc::c_int - (*_b).endbit) as libc::c_uint
        }
    }
    (*_b).endbyte += _bits >> 3 as libc::c_int;
    (*_b).endbit = _bits & 7 as libc::c_int;
    return ((ret & 0xffff as libc::c_int as libc::c_uint) >> m) as libc::c_int;
}
unsafe extern fn qr_pack_buf_avail(mut _b: *const qr_pack_buf) -> libc::c_int {
    return ((*_b).storage - (*_b).endbyte << 3 as libc::c_int) - (*_b).endbit;
}
/* The characters available in QR_MODE_ALNUM. */
static mut QR_ALNUM_TABLE: [libc::c_uchar; 45] = [
    '0' as i32 as libc::c_uchar,
    '1' as i32 as libc::c_uchar,
    '2' as i32 as libc::c_uchar,
    '3' as i32 as libc::c_uchar,
    '4' as i32 as libc::c_uchar,
    '5' as i32 as libc::c_uchar,
    '6' as i32 as libc::c_uchar,
    '7' as i32 as libc::c_uchar,
    '8' as i32 as libc::c_uchar,
    '9' as i32 as libc::c_uchar,
    'A' as i32 as libc::c_uchar,
    'B' as i32 as libc::c_uchar,
    'C' as i32 as libc::c_uchar,
    'D' as i32 as libc::c_uchar,
    'E' as i32 as libc::c_uchar,
    'F' as i32 as libc::c_uchar,
    'G' as i32 as libc::c_uchar,
    'H' as i32 as libc::c_uchar,
    'I' as i32 as libc::c_uchar,
    'J' as i32 as libc::c_uchar,
    'K' as i32 as libc::c_uchar,
    'L' as i32 as libc::c_uchar,
    'M' as i32 as libc::c_uchar,
    'N' as i32 as libc::c_uchar,
    'O' as i32 as libc::c_uchar,
    'P' as i32 as libc::c_uchar,
    'Q' as i32 as libc::c_uchar,
    'R' as i32 as libc::c_uchar,
    'S' as i32 as libc::c_uchar,
    'T' as i32 as libc::c_uchar,
    'U' as i32 as libc::c_uchar,
    'V' as i32 as libc::c_uchar,
    'W' as i32 as libc::c_uchar,
    'X' as i32 as libc::c_uchar,
    'Y' as i32 as libc::c_uchar,
    'Z' as i32 as libc::c_uchar,
    ' ' as i32 as libc::c_uchar,
    '$' as i32 as libc::c_uchar,
    '%' as i32 as libc::c_uchar,
    '*' as i32 as libc::c_uchar,
    '+' as i32 as libc::c_uchar,
    '-' as i32 as libc::c_uchar,
    '.' as i32 as libc::c_uchar,
    '/' as i32 as libc::c_uchar,
    ':' as i32 as libc::c_uchar,
];
unsafe extern fn qr_code_data_parse(
    mut _qrdata: *mut qr_code_data,
    mut _version: libc::c_int,
    mut _data: *const libc::c_uchar,
    mut _ndata: libc::c_int,
) -> libc::c_int {
    let mut qpb: qr_pack_buf = qr_pack_buf {
        buf: 0 as *const libc::c_uchar,
        endbyte: 0,
        endbit: 0,
        storage: 0,
    };
    let mut self_parity: libc::c_uint = 0;
    let mut centries: libc::c_int = 0;
    let mut len_bits_idx: libc::c_int = 0;
    /*Entries are stored directly in the struct during parsing.
    Caller cleans up any allocated data on failure.*/
    (*_qrdata).entries = 0 as *mut qr_code_data_entry;
    (*_qrdata).nentries = 0 as libc::c_int;
    (*_qrdata).sa_size = 0 as libc::c_int as libc::c_uchar;
    self_parity = 0 as libc::c_int as libc::c_uint;
    centries = 0 as libc::c_int;
    /*The versions are divided into 3 ranges that each use a different number of
    bits for length fields.*/
    len_bits_idx = (_version > 9 as libc::c_int) as libc::c_int
        + (_version > 26 as libc::c_int) as libc::c_int;
    qr_pack_buf_init(&mut qpb, _data, _ndata);
    /* While we have enough bits to read a mode... */
    while qr_pack_buf_avail(&mut qpb) >= 4 as libc::c_int {
        let mut entry: *mut qr_code_data_entry = 0 as *mut qr_code_data_entry;
        let mut mode: libc::c_int = 0;
        mode = qr_pack_buf_read(&mut qpb, 4 as libc::c_int);
        /* Mode 0 is a terminator. */
        if mode == 0 {
            break;
        }
        if (*_qrdata).nentries >= centries {
            centries = centries << 1 as libc::c_int | 1 as libc::c_int;
            (*_qrdata).entries = realloc(
                (*_qrdata).entries as *mut libc::c_void,
                (centries as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<qr_code_data_entry>() as libc::c_ulong),
            ) as *mut qr_code_data_entry
        }
        let fresh23 = (*_qrdata).nentries;
        (*_qrdata).nentries = (*_qrdata).nentries + 1;
        entry = (*_qrdata).entries.offset(fresh23 as isize);
        (*entry).mode = mode as qr_mode;
        /*Set the buffer to NULL, because if parsing fails, we might try to free it
        on clean-up.*/
        (*entry).payload.data.buf = 0 as *mut libc::c_uchar;
        static mut LEN_BITS: [[libc::c_uchar; 4]; 3] = [
            [
                10 as libc::c_int as libc::c_uchar,
                9 as libc::c_int as libc::c_uchar,
                8 as libc::c_int as libc::c_uchar,
                8 as libc::c_int as libc::c_uchar,
            ],
            [
                12 as libc::c_int as libc::c_uchar,
                11 as libc::c_int as libc::c_uchar,
                16 as libc::c_int as libc::c_uchar,
                10 as libc::c_int as libc::c_uchar,
            ],
            [
                14 as libc::c_int as libc::c_uchar,
                13 as libc::c_int as libc::c_uchar,
                16 as libc::c_int as libc::c_uchar,
                12 as libc::c_int as libc::c_uchar,
            ],
        ];
        match mode {
            1 => {
                let mut buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                let mut bits: libc::c_uint = 0;
                let mut c: libc::c_uint = 0;
                let mut len: libc::c_int = 0;
                let mut count: libc::c_int = 0;
                let mut rem: libc::c_int = 0;
                len = qr_pack_buf_read(
                    &mut qpb,
                    LEN_BITS[len_bits_idx as usize][0 as libc::c_int as usize] as libc::c_int,
                );
                if len < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                /*Check to see if there are enough bits left now, so we don't have to
                in the decode loop.*/
                count = len / 3 as libc::c_int;
                rem = len % 3 as libc::c_int;
                if qr_pack_buf_avail(&mut qpb)
                    < 10 as libc::c_int * count
                        + 7 as libc::c_int * (rem >> 1 as libc::c_int & 1 as libc::c_int)
                        + 4 as libc::c_int * (rem & 1 as libc::c_int)
                {
                    return -(1 as libc::c_int);
                }
                buf = malloc(
                    (len as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
                ) as *mut libc::c_uchar;
                (*entry).payload.data.buf = buf;
                (*entry).payload.data.len = len;
                loop
                /* Read groups of 3 digits encoded in 10 bits. */
                {
                    let fresh24 = count;
                    count = count - 1;
                    if !(fresh24 > 0 as libc::c_int) {
                        break;
                    }
                    bits = qr_pack_buf_read(&mut qpb, 10 as libc::c_int) as libc::c_uint;
                    if bits >= 1000 as libc::c_int as libc::c_uint {
                        return -(1 as libc::c_int);
                    }
                    c = ('0' as i32 as libc::c_uint)
                        .wrapping_add(bits.wrapping_div(100 as libc::c_int as libc::c_uint));
                    self_parity ^= c;
                    let fresh25 = buf;
                    buf = buf.offset(1);
                    *fresh25 = c as libc::c_uchar;
                    bits = bits.wrapping_rem(100 as libc::c_int as libc::c_uint);
                    c = ('0' as i32 as libc::c_uint)
                        .wrapping_add(bits.wrapping_div(10 as libc::c_int as libc::c_uint));
                    self_parity ^= c;
                    let fresh26 = buf;
                    buf = buf.offset(1);
                    *fresh26 = c as libc::c_uchar;
                    c = ('0' as i32 as libc::c_uint)
                        .wrapping_add(bits.wrapping_rem(10 as libc::c_int as libc::c_uint));
                    self_parity ^= c;
                    let fresh27 = buf;
                    buf = buf.offset(1);
                    *fresh27 = c as libc::c_uchar
                }
                /* Read the last two digits encoded in 7 bits. */
                if rem > 1 as libc::c_int {
                    bits = qr_pack_buf_read(&mut qpb, 7 as libc::c_int) as libc::c_uint;
                    if bits >= 100 as libc::c_int as libc::c_uint {
                        return -(1 as libc::c_int);
                    }
                    c = ('0' as i32 as libc::c_uint)
                        .wrapping_add(bits.wrapping_div(10 as libc::c_int as libc::c_uint));
                    self_parity ^= c;
                    let fresh28 = buf;
                    buf = buf.offset(1);
                    *fresh28 = c as libc::c_uchar;
                    c = ('0' as i32 as libc::c_uint)
                        .wrapping_add(bits.wrapping_rem(10 as libc::c_int as libc::c_uint));
                    self_parity ^= c;
                    let fresh29 = buf;
                    buf = buf.offset(1);
                    *fresh29 = c as libc::c_uchar
                } else if rem != 0 {
                    bits = qr_pack_buf_read(&mut qpb, 4 as libc::c_int) as libc::c_uint;
                    if bits >= 10 as libc::c_int as libc::c_uint {
                        return -(1 as libc::c_int);
                    }
                    c = ('0' as i32 as libc::c_uint).wrapping_add(bits);
                    self_parity ^= c;
                    let fresh30 = buf;
                    buf = buf.offset(1);
                    *fresh30 = c as libc::c_uchar
                }
            }
            2 => {
                let mut buf_0: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                let mut bits_0: libc::c_uint = 0;
                let mut c_0: libc::c_uint = 0;
                let mut len_0: libc::c_int = 0;
                let mut count_0: libc::c_int = 0;
                let mut rem_0: libc::c_int = 0;
                len_0 = qr_pack_buf_read(
                    &mut qpb,
                    LEN_BITS[len_bits_idx as usize][1 as libc::c_int as usize] as libc::c_int,
                );
                if len_0 < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                /* Or the last one digit encoded in 4 bits. */
                /*Check to see if there are enough bits left now, so we don't have to
                in the decode loop.*/
                count_0 = len_0 >> 1 as libc::c_int;
                rem_0 = len_0 & 1 as libc::c_int;
                if qr_pack_buf_avail(&mut qpb)
                    < 11 as libc::c_int * count_0 + 6 as libc::c_int * rem_0
                {
                    return -(1 as libc::c_int);
                }
                buf_0 = malloc(
                    (len_0 as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
                ) as *mut libc::c_uchar;
                (*entry).payload.data.buf = buf_0;
                (*entry).payload.data.len = len_0;
                loop
                /* Read groups of two characters encoded in 11 bits. */
                {
                    let fresh31 = count_0;
                    count_0 = count_0 - 1;
                    if !(fresh31 > 0 as libc::c_int) {
                        break;
                    }
                    bits_0 = qr_pack_buf_read(&mut qpb, 11 as libc::c_int) as libc::c_uint;
                    if bits_0 >= 2025 as libc::c_int as libc::c_uint {
                        return -(1 as libc::c_int);
                    }
                    c_0 = QR_ALNUM_TABLE
                        [bits_0.wrapping_div(45 as libc::c_int as libc::c_uint) as usize]
                        as libc::c_uint;
                    self_parity ^= c_0;
                    let fresh32 = buf_0;
                    buf_0 = buf_0.offset(1);
                    *fresh32 = c_0 as libc::c_uchar;
                    c_0 = QR_ALNUM_TABLE
                        [bits_0.wrapping_rem(45 as libc::c_int as libc::c_uint) as usize]
                        as libc::c_uint;
                    self_parity ^= c_0;
                    let fresh33 = buf_0;
                    buf_0 = buf_0.offset(1);
                    *fresh33 = c_0 as libc::c_uchar;
                    len_0 -= 2 as libc::c_int
                }
                /* Read the last character encoded in 6 bits. */
                if rem_0 != 0 {
                    bits_0 = qr_pack_buf_read(&mut qpb, 6 as libc::c_int) as libc::c_uint;
                    if bits_0 >= 45 as libc::c_int as libc::c_uint {
                        return -(1 as libc::c_int);
                    }
                    c_0 = QR_ALNUM_TABLE[bits_0 as usize] as libc::c_uint;
                    self_parity ^= c_0;
                    let fresh34 = buf_0;
                    buf_0 = buf_0.offset(1);
                    *fresh34 = c_0 as libc::c_uchar
                }
            }
            3 => {
                /* Structured-append header. */
                let mut bits_1: libc::c_int = 0;
                bits_1 = qr_pack_buf_read(&mut qpb, 16 as libc::c_int);
                if bits_1 < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                /*We save a copy of the data in _qrdata for easy reference when
                 grouping structured-append codes.
                If for some reason the code has multiple S-A headers, first one wins,
                 since it is supposed to come before everything else (TODO: should we
                 return an error instead?).*/
                if (*_qrdata).sa_size as libc::c_int == 0 as libc::c_int {
                    (*entry).payload.sa.sa_index =
                        (bits_1 >> 12 as libc::c_int & 0xf as libc::c_int) as libc::c_uchar;
                    (*_qrdata).sa_index = (*entry).payload.sa.sa_index;
                    (*entry).payload.sa.sa_size =
                        ((bits_1 >> 8 as libc::c_int & 0xf as libc::c_int) + 1 as libc::c_int)
                            as libc::c_uchar;
                    (*_qrdata).sa_size = (*entry).payload.sa.sa_size;
                    (*entry).payload.sa.sa_parity = (bits_1 & 0xff as libc::c_int) as libc::c_uchar;
                    (*_qrdata).sa_parity = (*entry).payload.sa.sa_parity
                }
            }
            4 => {
                let mut buf_1: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                let mut c_1: libc::c_uint = 0;
                let mut len_1: libc::c_int = 0;
                len_1 = qr_pack_buf_read(
                    &mut qpb,
                    LEN_BITS[len_bits_idx as usize][2 as libc::c_int as usize] as libc::c_int,
                );
                if len_1 < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                /*Check to see if there are enough bits left now, so we don't have to
                in the decode loop.*/
                if qr_pack_buf_avail(&mut qpb) < len_1 << 3 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                buf_1 = malloc(
                    (len_1 as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
                ) as *mut libc::c_uchar;
                (*entry).payload.data.buf = buf_1;
                (*entry).payload.data.len = len_1;
                loop {
                    let fresh35 = len_1;
                    len_1 = len_1 - 1;
                    if !(fresh35 > 0 as libc::c_int) {
                        break;
                    }
                    c_1 = qr_pack_buf_read(&mut qpb, 8 as libc::c_int) as libc::c_uint;
                    self_parity ^= c_1;
                    let fresh36 = buf_1;
                    buf_1 = buf_1.offset(1);
                    *fresh36 = c_1 as libc::c_uchar
                }
            }
            5 => {}
            7 => {
                /* Extended Channel Interpretation data. */
                let mut val: libc::c_uint = 0;
                let mut bits_2: libc::c_int = 0;
                /* ECI uses a variable-width encoding similar to UTF-8 */
                bits_2 = qr_pack_buf_read(&mut qpb, 8 as libc::c_int);
                if bits_2 < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                /* One byte: */
                if bits_2 & 0x80 as libc::c_int == 0 {
                    val = bits_2 as libc::c_uint
                } else if bits_2 & 0x40 as libc::c_int == 0 {
                    val = (bits_2 & (0x3f as libc::c_int) << 8 as libc::c_int) as libc::c_uint;
                    bits_2 = qr_pack_buf_read(&mut qpb, 8 as libc::c_int);
                    if bits_2 < 0 as libc::c_int {
                        return -(1 as libc::c_int);
                    }
                    val |= bits_2 as libc::c_uint
                } else if bits_2 & 0x20 as libc::c_int == 0 {
                    val = (bits_2 & (0x1f as libc::c_int) << 16 as libc::c_int) as libc::c_uint;
                    bits_2 = qr_pack_buf_read(&mut qpb, 16 as libc::c_int);
                    if bits_2 < 0 as libc::c_int {
                        return -(1 as libc::c_int);
                    }
                    val |= bits_2 as libc::c_uint;
                    /* Two bytes: */
                    /* Three bytes: */
                    /* Valid ECI values are 0...999999. */
                    if val >= 1000000 as libc::c_int as libc::c_uint {
                        return -(1 as libc::c_int);
                    }
                } else {
                    /* Invalid lead byte. */
                    return -(1 as libc::c_int);
                }
                (*entry).payload.eci = val
            }
            8 => {
                let mut buf_2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                let mut bits_3: libc::c_uint = 0;
                let mut len_2: libc::c_int = 0;
                len_2 = qr_pack_buf_read(
                    &mut qpb,
                    LEN_BITS[len_bits_idx as usize][3 as libc::c_int as usize] as libc::c_int,
                );
                if len_2 < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                /*Check to see if there are enough bits left now, so we don't have to
                in the decode loop.*/
                if qr_pack_buf_avail(&mut qpb) < 13 as libc::c_int * len_2 {
                    return -(1 as libc::c_int);
                }
                buf_2 = malloc(
                    ((2 as libc::c_int * len_2) as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
                ) as *mut libc::c_uchar;
                (*entry).payload.data.buf = buf_2;
                (*entry).payload.data.len = 2 as libc::c_int * len_2;
                loop
                /* Decode 2-byte SJIS characters encoded in 13 bits. */
                {
                    let fresh37 = len_2;
                    len_2 = len_2 - 1;
                    if !(fresh37 > 0 as libc::c_int) {
                        break;
                    }
                    bits_3 = qr_pack_buf_read(&mut qpb, 13 as libc::c_int) as libc::c_uint;
                    bits_3 = (bits_3.wrapping_div(0xc0 as libc::c_int as libc::c_uint)
                        << 8 as libc::c_int
                        | bits_3.wrapping_rem(0xc0 as libc::c_int as libc::c_uint))
                    .wrapping_add(0x8140 as libc::c_int as libc::c_uint);
                    if bits_3 >= 0xa000 as libc::c_int as libc::c_uint {
                        bits_3 = bits_3.wrapping_add(0x4000 as libc::c_int as libc::c_uint)
                    }
                    /*TODO: Are values 0xXX7F, 0xXXFD...0xXXFF always invalid?
                    Should we reject them here?*/
                    self_parity ^= bits_3;
                    let fresh38 = buf_2;
                    buf_2 = buf_2.offset(1);
                    *fresh38 = (bits_3 >> 8 as libc::c_int) as libc::c_uchar;
                    let fresh39 = buf_2;
                    buf_2 = buf_2.offset(1);
                    *fresh39 = (bits_3 & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar
                }
            }
            9 => {
                /* FNC1 second position marker. */
                let mut bits_4: libc::c_int = 0;
                /*FNC1 in the 2nd position encodes an Application Indicator in one
                 byte, which is either a letter (A...Z or a...z) or a 2-digit number.
                The letters are encoded with their ASCII value plus 100, the numbers
                 are encoded directly with their numeric value.
                Values 100...164, 191...196, and 223...255 are invalid, so we reject
                 them here.*/
                bits_4 = qr_pack_buf_read(&mut qpb, 8 as libc::c_int);
                if !(bits_4 >= 0 as libc::c_int && bits_4 < 100 as libc::c_int
                    || bits_4 >= 165 as libc::c_int && bits_4 < 191 as libc::c_int
                    || bits_4 >= 197 as libc::c_int && bits_4 < 223 as libc::c_int)
                {
                    return -(1 as libc::c_int);
                }
                (*entry).payload.ai = bits_4
            }
            _ => {
                /* Unknown mode number: */
                /*Unfortunately, because we have to understand the format of a mode to
                 know how many bits it occupies, we can't skip unknown modes.
                Therefore we have to fail.*/
                return -(1 as libc::c_int);
            }
        }
    }
    /*Store the parity of the data from this code, for S-A.
    The final parity is the 8-bit XOR of all the decoded bytes of literal data.
    We don't combine the 2-byte kanji codes into one byte in the loops above,
     because we can just do it here instead.*/
    (*_qrdata).self_parity = ((self_parity >> 8 as libc::c_int ^ self_parity)
        & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    /* Success. */
    (*_qrdata).entries = realloc(
        (*_qrdata).entries as *mut libc::c_void,
        ((*_qrdata).nentries as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<qr_code_data_entry>() as libc::c_ulong),
    ) as *mut qr_code_data_entry;
    return 0 as libc::c_int;
}
unsafe extern fn qr_code_data_clear(mut _qrdata: *mut qr_code_data) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*_qrdata).nentries {
        if (*(*_qrdata).entries.offset(i as isize)).mode as libc::c_uint
            & ((*(*_qrdata).entries.offset(i as isize)).mode as libc::c_uint)
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
            == 0
        {
            free((*(*_qrdata).entries.offset(i as isize)).payload.data.buf as *mut libc::c_void);
        }
        i += 1
    }
    free((*_qrdata).entries as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern fn qr_code_data_list_init(mut _qrlist: *mut qr_code_data_list) {
    (*_qrlist).qrdata = 0 as *mut qr_code_data;
    (*_qrlist).cqrdata = 0 as libc::c_int;
    (*_qrlist).nqrdata = (*_qrlist).cqrdata;
}
#[no_mangle]
pub unsafe extern fn qr_code_data_list_clear(mut _qrlist: *mut qr_code_data_list) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*_qrlist).nqrdata {
        qr_code_data_clear((*_qrlist).qrdata.offset(i as isize));
        i += 1
    }
    free((*_qrlist).qrdata as *mut libc::c_void);
    qr_code_data_list_init(_qrlist);
}
unsafe extern fn qr_code_data_list_add(
    mut _qrlist: *mut qr_code_data_list,
    mut _qrdata: *mut qr_code_data,
) {
    if (*_qrlist).nqrdata >= (*_qrlist).cqrdata {
        (*_qrlist).cqrdata = (*_qrlist).cqrdata << 1 as libc::c_int | 1 as libc::c_int;
        (*_qrlist).qrdata = realloc(
            (*_qrlist).qrdata as *mut libc::c_void,
            ((*_qrlist).cqrdata as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<qr_code_data>() as libc::c_ulong),
        ) as *mut qr_code_data
    }
    let fresh40 = (*_qrlist).nqrdata;
    (*_qrlist).nqrdata = (*_qrlist).nqrdata + 1;
    memcpy(
        (*_qrlist).qrdata.offset(fresh40 as isize) as *mut libc::c_void,
        _qrdata as *const libc::c_void,
        ::std::mem::size_of::<qr_code_data>() as libc::c_ulong,
    );
}
/* The total number of codewords in a QR code. */
unsafe extern fn qr_code_ncodewords(mut _version: libc::c_uint) -> libc::c_int {
    let mut nalign: libc::c_uint = 0;
    /*This is 24-27 instructions on ARM in thumb mode, or a 26-32 byte savings
    over just using a table (not counting the instructions that would be
    needed to do the table lookup).*/
    if _version == 1 as libc::c_int as libc::c_uint {
        return 26 as libc::c_int;
    }
    nalign = _version
        .wrapping_div(7 as libc::c_int as libc::c_uint)
        .wrapping_add(2 as libc::c_int as libc::c_uint);
    return ((_version << 4 as libc::c_int)
        .wrapping_mul(_version.wrapping_add(8 as libc::c_int as libc::c_uint))
        .wrapping_sub(
            (5 as libc::c_int as libc::c_uint).wrapping_mul(nalign).wrapping_mul(
                (5 as libc::c_int as libc::c_uint)
                    .wrapping_mul(nalign)
                    .wrapping_sub(2 as libc::c_int as libc::c_uint),
            ),
        )
        .wrapping_add(
            (36 as libc::c_int * (_version < 7 as libc::c_int as libc::c_uint) as libc::c_int)
                as libc::c_uint,
        )
        .wrapping_add(83 as libc::c_int as libc::c_uint)
        >> 3 as libc::c_int) as libc::c_int;
}
/* Bulk data for the number of parity bytes per Reed-Solomon block. */
static mut QR_RS_NPAR_VALS: [libc::c_uchar; 71] = [
    7 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    17 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    22 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    22 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    22 as libc::c_int as libc::c_uchar,
    22 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    22 as libc::c_int as libc::c_uchar,
    15 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    22 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    30 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    20 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    30 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    20 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    30 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    30 as libc::c_int as libc::c_uchar,
    30 as libc::c_int as libc::c_uchar,
    22 as libc::c_int as libc::c_uchar,
    20 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    20 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    20 as libc::c_int as libc::c_uchar,
    30 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    22 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    30 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    30 as libc::c_int as libc::c_uchar,
    30 as libc::c_int as libc::c_uchar,
];
/*An offset into QR_RS_NPAR_DATA for each version that gives the number of
parity bytes per Reed-Solomon block for each error correction level.*/
static mut QR_RS_NPAR_OFFS: [libc::c_uchar; 40] = [
    0 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    19 as libc::c_int as libc::c_uchar,
    55 as libc::c_int as libc::c_uchar,
    15 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    37 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    51 as libc::c_int as libc::c_uchar,
    39 as libc::c_int as libc::c_uchar,
    59 as libc::c_int as libc::c_uchar,
    62 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    22 as libc::c_int as libc::c_uchar,
    41 as libc::c_int as libc::c_uchar,
    31 as libc::c_int as libc::c_uchar,
    44 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    65 as libc::c_int as libc::c_uchar,
    47 as libc::c_int as libc::c_uchar,
    33 as libc::c_int as libc::c_uchar,
    67 as libc::c_int as libc::c_uchar,
    67 as libc::c_int as libc::c_uchar,
    48 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    67 as libc::c_int as libc::c_uchar,
    67 as libc::c_int as libc::c_uchar,
    67 as libc::c_int as libc::c_uchar,
    67 as libc::c_int as libc::c_uchar,
    67 as libc::c_int as libc::c_uchar,
    67 as libc::c_int as libc::c_uchar,
    67 as libc::c_int as libc::c_uchar,
    67 as libc::c_int as libc::c_uchar,
    67 as libc::c_int as libc::c_uchar,
    67 as libc::c_int as libc::c_uchar,
    67 as libc::c_int as libc::c_uchar,
    67 as libc::c_int as libc::c_uchar,
    67 as libc::c_int as libc::c_uchar,
    67 as libc::c_int as libc::c_uchar,
];
/*The number of Reed-Solomon blocks for each version and error correction
level.*/
static mut QR_RS_NBLOCKS: [[libc::c_uchar; 4]; 40] = [
    [
        1 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
    ],
    [
        1 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
    ],
    [
        1 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
    ],
    [
        1 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
    ],
    [
        1 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
    ],
    [
        2 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
    ],
    [
        2 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
    ],
    [
        2 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
    ],
    [
        2 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
    ],
    [
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
    ],
    [
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        11 as libc::c_int as libc::c_uchar,
    ],
    [
        4 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        10 as libc::c_int as libc::c_uchar,
        11 as libc::c_int as libc::c_uchar,
    ],
    [
        4 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        12 as libc::c_int as libc::c_uchar,
        16 as libc::c_int as libc::c_uchar,
    ],
    [
        4 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        16 as libc::c_int as libc::c_uchar,
        16 as libc::c_int as libc::c_uchar,
    ],
    [
        6 as libc::c_int as libc::c_uchar,
        10 as libc::c_int as libc::c_uchar,
        12 as libc::c_int as libc::c_uchar,
        18 as libc::c_int as libc::c_uchar,
    ],
    [
        6 as libc::c_int as libc::c_uchar,
        10 as libc::c_int as libc::c_uchar,
        17 as libc::c_int as libc::c_uchar,
        16 as libc::c_int as libc::c_uchar,
    ],
    [
        6 as libc::c_int as libc::c_uchar,
        11 as libc::c_int as libc::c_uchar,
        16 as libc::c_int as libc::c_uchar,
        19 as libc::c_int as libc::c_uchar,
    ],
    [
        6 as libc::c_int as libc::c_uchar,
        13 as libc::c_int as libc::c_uchar,
        18 as libc::c_int as libc::c_uchar,
        21 as libc::c_int as libc::c_uchar,
    ],
    [
        7 as libc::c_int as libc::c_uchar,
        14 as libc::c_int as libc::c_uchar,
        21 as libc::c_int as libc::c_uchar,
        25 as libc::c_int as libc::c_uchar,
    ],
    [
        8 as libc::c_int as libc::c_uchar,
        16 as libc::c_int as libc::c_uchar,
        20 as libc::c_int as libc::c_uchar,
        25 as libc::c_int as libc::c_uchar,
    ],
    [
        8 as libc::c_int as libc::c_uchar,
        17 as libc::c_int as libc::c_uchar,
        23 as libc::c_int as libc::c_uchar,
        25 as libc::c_int as libc::c_uchar,
    ],
    [
        9 as libc::c_int as libc::c_uchar,
        17 as libc::c_int as libc::c_uchar,
        23 as libc::c_int as libc::c_uchar,
        34 as libc::c_int as libc::c_uchar,
    ],
    [
        9 as libc::c_int as libc::c_uchar,
        18 as libc::c_int as libc::c_uchar,
        25 as libc::c_int as libc::c_uchar,
        30 as libc::c_int as libc::c_uchar,
    ],
    [
        10 as libc::c_int as libc::c_uchar,
        20 as libc::c_int as libc::c_uchar,
        27 as libc::c_int as libc::c_uchar,
        32 as libc::c_int as libc::c_uchar,
    ],
    [
        12 as libc::c_int as libc::c_uchar,
        21 as libc::c_int as libc::c_uchar,
        29 as libc::c_int as libc::c_uchar,
        35 as libc::c_int as libc::c_uchar,
    ],
    [
        12 as libc::c_int as libc::c_uchar,
        23 as libc::c_int as libc::c_uchar,
        34 as libc::c_int as libc::c_uchar,
        37 as libc::c_int as libc::c_uchar,
    ],
    [
        12 as libc::c_int as libc::c_uchar,
        25 as libc::c_int as libc::c_uchar,
        34 as libc::c_int as libc::c_uchar,
        40 as libc::c_int as libc::c_uchar,
    ],
    [
        13 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        35 as libc::c_int as libc::c_uchar,
        42 as libc::c_int as libc::c_uchar,
    ],
    [
        14 as libc::c_int as libc::c_uchar,
        28 as libc::c_int as libc::c_uchar,
        38 as libc::c_int as libc::c_uchar,
        45 as libc::c_int as libc::c_uchar,
    ],
    [
        15 as libc::c_int as libc::c_uchar,
        29 as libc::c_int as libc::c_uchar,
        40 as libc::c_int as libc::c_uchar,
        48 as libc::c_int as libc::c_uchar,
    ],
    [
        16 as libc::c_int as libc::c_uchar,
        31 as libc::c_int as libc::c_uchar,
        43 as libc::c_int as libc::c_uchar,
        51 as libc::c_int as libc::c_uchar,
    ],
    [
        17 as libc::c_int as libc::c_uchar,
        33 as libc::c_int as libc::c_uchar,
        45 as libc::c_int as libc::c_uchar,
        54 as libc::c_int as libc::c_uchar,
    ],
    [
        18 as libc::c_int as libc::c_uchar,
        35 as libc::c_int as libc::c_uchar,
        48 as libc::c_int as libc::c_uchar,
        57 as libc::c_int as libc::c_uchar,
    ],
    [
        19 as libc::c_int as libc::c_uchar,
        37 as libc::c_int as libc::c_uchar,
        51 as libc::c_int as libc::c_uchar,
        60 as libc::c_int as libc::c_uchar,
    ],
    [
        19 as libc::c_int as libc::c_uchar,
        38 as libc::c_int as libc::c_uchar,
        53 as libc::c_int as libc::c_uchar,
        63 as libc::c_int as libc::c_uchar,
    ],
    [
        20 as libc::c_int as libc::c_uchar,
        40 as libc::c_int as libc::c_uchar,
        56 as libc::c_int as libc::c_uchar,
        66 as libc::c_int as libc::c_uchar,
    ],
    [
        21 as libc::c_int as libc::c_uchar,
        43 as libc::c_int as libc::c_uchar,
        59 as libc::c_int as libc::c_uchar,
        70 as libc::c_int as libc::c_uchar,
    ],
    [
        22 as libc::c_int as libc::c_uchar,
        45 as libc::c_int as libc::c_uchar,
        62 as libc::c_int as libc::c_uchar,
        74 as libc::c_int as libc::c_uchar,
    ],
    [
        24 as libc::c_int as libc::c_uchar,
        47 as libc::c_int as libc::c_uchar,
        65 as libc::c_int as libc::c_uchar,
        77 as libc::c_int as libc::c_uchar,
    ],
    [
        25 as libc::c_int as libc::c_uchar,
        49 as libc::c_int as libc::c_uchar,
        68 as libc::c_int as libc::c_uchar,
        81 as libc::c_int as libc::c_uchar,
    ],
];
/*Attempts to fully decode a QR code.
_qrdata:   Returns the parsed code data.
_gf:       Used for Reed-Solomon error correction.
_ul_pos:   The location of the UL finder pattern.
_ur_pos:   The location of the UR finder pattern.
_dl_pos:   The location of the DL finder pattern.
_version:  The (decoded) version number.
_fmt_info: The decoded format info.
_img:      The binary input image.
_width:    The width of the input image.
_height:   The height of the input image.
Return: 0 on success, or a negative value on error.*/
unsafe extern fn qr_code_decode(
    mut _qrdata: *mut qr_code_data,
    mut _gf: *const rs_gf256,
    mut _ul_pos: *const libc::c_int,
    mut _ur_pos: *const libc::c_int,
    mut _dl_pos: *const libc::c_int,
    mut _version: libc::c_int,
    mut _fmt_info: libc::c_int,
    mut _img: *const libc::c_uchar,
    mut _width: libc::c_int,
    mut _height: libc::c_int,
) -> libc::c_int {
    let mut grid: qr_sampling_grid = qr_sampling_grid {
        cells: [0 as *mut qr_hom_cell; 6],
        fpmask: 0 as *mut libc::c_uint,
        cell_limits: [0; 6],
        ncells: 0,
    };
    let mut data_bits: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut blocks: *mut *mut libc::c_uchar = 0 as *mut *mut libc::c_uchar;
    let mut block_data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut nblocks: libc::c_int = 0;
    let mut nshort_blocks: libc::c_int = 0;
    let mut ncodewords: libc::c_int = 0;
    let mut block_sz: libc::c_int = 0;
    let mut ecc_level: libc::c_int = 0;
    let mut ndata: libc::c_int = 0;
    let mut npar: libc::c_int = 0;
    let mut dim: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    /* Read the bits out of the image. */
    qr_sampling_grid_init(
        &mut grid,
        _version,
        _ul_pos,
        _ur_pos,
        _dl_pos,
        (*_qrdata).bbox.as_mut_ptr(),
        _img,
        _width,
        _height,
    );
    dim = 17 as libc::c_int + (_version << 2 as libc::c_int);
    data_bits = malloc(
        ((dim
            * (dim
                + ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                    * 8 as libc::c_int
                - 1 as libc::c_int
                >> (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                    * 8 as libc::c_int) as libc::c_uint
                    & 0xffff0000 as libc::c_uint
                    != 0
                {
                    (16 as libc::c_int)
                        + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int) as libc::c_uint
                            >> 16 as libc::c_int
                            & 0xff00 as libc::c_int as libc::c_uint
                            != 0
                        {
                            (8 as libc::c_int)
                                + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 16 as libc::c_int
                                    >> 8 as libc::c_int
                                    & 0xf0 as libc::c_int as libc::c_uint
                                    != 0
                                {
                                    (4 as libc::c_int)
                                        + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 16 as libc::c_int
                                            >> 8 as libc::c_int
                                            >> 4 as libc::c_int
                                            & 0xc as libc::c_int as libc::c_uint
                                            != 0
                                        {
                                            (2 as libc::c_int)
                                                + ((::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong
                                                    as libc::c_int
                                                    * 8 as libc::c_int)
                                                    as libc::c_uint
                                                    >> 16 as libc::c_int
                                                    >> 8 as libc::c_int
                                                    >> 4 as libc::c_int
                                                    >> 2 as libc::c_int
                                                    & 0x2 as libc::c_int as libc::c_uint
                                                    != 0)
                                                    as libc::c_int
                                        } else {
                                            ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                                as libc::c_int
                                                * 8 as libc::c_int)
                                                as libc::c_uint
                                                >> 16 as libc::c_int
                                                >> 8 as libc::c_int
                                                >> 4 as libc::c_int
                                                & 0x2 as libc::c_int as libc::c_uint
                                                != 0)
                                                as libc::c_int
                                        })
                                } else {
                                    (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 16 as libc::c_int
                                        >> 8 as libc::c_int
                                        & 0xc as libc::c_int as libc::c_uint
                                        != 0
                                    {
                                        (2 as libc::c_int)
                                            + ((::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong
                                                as libc::c_int
                                                * 8 as libc::c_int)
                                                as libc::c_uint
                                                >> 16 as libc::c_int
                                                >> 8 as libc::c_int
                                                >> 2 as libc::c_int
                                                & 0x2 as libc::c_int as libc::c_uint
                                                != 0)
                                                as libc::c_int
                                    } else {
                                        ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 16 as libc::c_int
                                            >> 8 as libc::c_int
                                            & 0x2 as libc::c_int as libc::c_uint
                                            != 0)
                                            as libc::c_int
                                    })
                                })
                        } else {
                            (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int) as libc::c_uint
                                >> 16 as libc::c_int
                                & 0xf0 as libc::c_int as libc::c_uint
                                != 0
                            {
                                (4 as libc::c_int)
                                    + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 16 as libc::c_int
                                        >> 4 as libc::c_int
                                        & 0xc as libc::c_int as libc::c_uint
                                        != 0
                                    {
                                        (2 as libc::c_int)
                                            + ((::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong
                                                as libc::c_int
                                                * 8 as libc::c_int)
                                                as libc::c_uint
                                                >> 16 as libc::c_int
                                                >> 4 as libc::c_int
                                                >> 2 as libc::c_int
                                                & 0x2 as libc::c_int as libc::c_uint
                                                != 0)
                                                as libc::c_int
                                    } else {
                                        ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 16 as libc::c_int
                                            >> 4 as libc::c_int
                                            & 0x2 as libc::c_int as libc::c_uint
                                            != 0)
                                            as libc::c_int
                                    })
                            } else {
                                (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 16 as libc::c_int
                                    & 0xc as libc::c_int as libc::c_uint
                                    != 0
                                {
                                    (2 as libc::c_int)
                                        + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 16 as libc::c_int
                                            >> 2 as libc::c_int
                                            & 0x2 as libc::c_int as libc::c_uint
                                            != 0)
                                            as libc::c_int
                                } else {
                                    ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 16 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                                })
                            })
                        })
                } else {
                    (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                        * 8 as libc::c_int) as libc::c_uint
                        & 0xff00 as libc::c_int as libc::c_uint
                        != 0
                    {
                        (8 as libc::c_int)
                            + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int)
                                as libc::c_uint
                                >> 8 as libc::c_int
                                & 0xf0 as libc::c_int as libc::c_uint
                                != 0
                            {
                                (4 as libc::c_int)
                                    + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 8 as libc::c_int
                                        >> 4 as libc::c_int
                                        & 0xc as libc::c_int as libc::c_uint
                                        != 0
                                    {
                                        (2 as libc::c_int)
                                            + ((::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong
                                                as libc::c_int
                                                * 8 as libc::c_int)
                                                as libc::c_uint
                                                >> 8 as libc::c_int
                                                >> 4 as libc::c_int
                                                >> 2 as libc::c_int
                                                & 0x2 as libc::c_int as libc::c_uint
                                                != 0)
                                                as libc::c_int
                                    } else {
                                        ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 8 as libc::c_int
                                            >> 4 as libc::c_int
                                            & 0x2 as libc::c_int as libc::c_uint
                                            != 0)
                                            as libc::c_int
                                    })
                            } else {
                                (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 8 as libc::c_int
                                    & 0xc as libc::c_int as libc::c_uint
                                    != 0
                                {
                                    (2 as libc::c_int)
                                        + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 8 as libc::c_int
                                            >> 2 as libc::c_int
                                            & 0x2 as libc::c_int as libc::c_uint
                                            != 0)
                                            as libc::c_int
                                } else {
                                    ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 8 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                                })
                            })
                    } else {
                        (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
                            * 8 as libc::c_int) as libc::c_uint
                            & 0xf0 as libc::c_int as libc::c_uint
                            != 0
                        {
                            (4 as libc::c_int)
                                + (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    >> 4 as libc::c_int
                                    & 0xc as libc::c_int as libc::c_uint
                                    != 0
                                {
                                    (2 as libc::c_int)
                                        + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as libc::c_int
                                            * 8 as libc::c_int)
                                            as libc::c_uint
                                            >> 4 as libc::c_int
                                            >> 2 as libc::c_int
                                            & 0x2 as libc::c_int as libc::c_uint
                                            != 0)
                                            as libc::c_int
                                } else {
                                    ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 4 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                                })
                        } else {
                            (if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                as libc::c_int
                                * 8 as libc::c_int) as libc::c_uint
                                & 0xc as libc::c_int as libc::c_uint
                                != 0
                            {
                                (2 as libc::c_int)
                                    + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        as libc::c_int
                                        * 8 as libc::c_int)
                                        as libc::c_uint
                                        >> 2 as libc::c_int
                                        & 0x2 as libc::c_int as libc::c_uint
                                        != 0) as libc::c_int
                            } else {
                                ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int
                                    * 8 as libc::c_int)
                                    as libc::c_uint
                                    & 0x2 as libc::c_int as libc::c_uint
                                    != 0) as libc::c_int
                            })
                        })
                    })
                }))) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong),
    ) as *mut libc::c_uint;
    qr_sampling_grid_sample(&mut grid, data_bits, dim, _fmt_info, _img, _width, _height);
    /* Group those bits into Reed-Solomon codewords. */
    ecc_level = _fmt_info >> 3 as libc::c_int ^ 1 as libc::c_int;
    nblocks =
        QR_RS_NBLOCKS[(_version - 1 as libc::c_int) as usize][ecc_level as usize] as libc::c_int;
    npar = *QR_RS_NPAR_VALS
        .as_ptr()
        .offset(QR_RS_NPAR_OFFS[(_version - 1 as libc::c_int) as usize] as libc::c_int as isize)
        .offset(ecc_level as isize) as libc::c_int;
    ncodewords = qr_code_ncodewords(_version as libc::c_uint);
    block_sz = ncodewords / nblocks;
    nshort_blocks = nblocks - ncodewords % nblocks;
    blocks = malloc(
        (nblocks as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_uchar>() as libc::c_ulong),
    ) as *mut *mut libc::c_uchar;
    block_data = malloc(
        (ncodewords as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
    ) as *mut libc::c_uchar;
    let ref mut fresh41 = *blocks.offset(0 as libc::c_int as isize);
    *fresh41 = block_data;
    i = 1 as libc::c_int;
    while i < nblocks {
        let ref mut fresh42 = *blocks.offset(i as isize);
        *fresh42 = (*blocks.offset((i - 1 as libc::c_int) as isize))
            .offset(block_sz as isize)
            .offset((i > nshort_blocks) as libc::c_int as isize);
        i += 1
    }
    qr_samples_unpack(blocks, nblocks, block_sz - npar, nshort_blocks, data_bits, grid.fpmask, dim);
    qr_sampling_grid_clear(&mut grid);
    free(blocks as *mut libc::c_void);
    free(data_bits as *mut libc::c_void);
    /* Perform the error correction. */
    ndata = 0 as libc::c_int;
    ncodewords = 0 as libc::c_int;
    ret = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nblocks {
        let mut block_szi: libc::c_int = 0;
        let mut ndatai: libc::c_int = 0;
        block_szi = block_sz + (i >= nshort_blocks) as libc::c_int;
        ret = rs_correct(
            _gf,
            0 as libc::c_int,
            block_data.offset(ncodewords as isize),
            block_szi,
            npar,
            0 as *const libc::c_uchar,
            0 as libc::c_int,
        );
        /*For version 1 symbols and version 2-L and 3-L symbols, we aren't allowed
         to use all the parity bytes for correction.
        They are instead used to improve detection.
        Version 1-L reserves 3 parity bytes for detection.
        Versions 1-M and 2-L reserve 2 parity bytes for detection.
        Versions 1-Q, 1-H, and 3-L reserve 1 parity byte for detection.
        We can ignore the version 3-L restriction because it has an odd number of
         parity bytes, and we don't support erasure detection.*/
        if ret < 0 as libc::c_int
            || _version == 1 as libc::c_int
                && ret > (ecc_level + 1 as libc::c_int) << 1 as libc::c_int
            || _version == 2 as libc::c_int
                && ecc_level == 0 as libc::c_int
                && ret > 4 as libc::c_int
        {
            ret = -(1 as libc::c_int);
            break;
        } else {
            ndatai = block_szi - npar;
            memmove(
                block_data.offset(ndata as isize) as *mut libc::c_void,
                block_data.offset(ncodewords as isize) as *const libc::c_void,
                (ndatai as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
            );
            ncodewords += block_szi;
            ndata += ndatai;
            i += 1
        }
    }
    /* Parse the corrected bitstream. */
    if ret >= 0 as libc::c_int {
        ret = qr_code_data_parse(_qrdata, _version, block_data, ndata);
        /*We could return any partially decoded data, but then we'd have to have
        API support for that; a mode ignoring ECC errors might also be useful.*/
        if ret < 0 as libc::c_int {
            qr_code_data_clear(_qrdata);
        }
        (*_qrdata).version = _version as libc::c_uchar;
        (*_qrdata).ecc_level = ecc_level as libc::c_uchar
    }
    free(block_data as *mut libc::c_void);
    return ret;
}
/*Searches for an arrangement of these three finder centers that yields a valid
 configuration.
_c: On input, the three finder centers to consider in any order.
Return: The detected version number, or a negative value on error.*/
unsafe extern fn qr_reader_try_configuration(
    mut _reader: *mut qr_reader,
    mut _qrdata: *mut qr_code_data,
    mut _img: *const libc::c_uchar,
    mut _width: libc::c_int,
    mut _height: libc::c_int,
    mut _c: *mut *mut qr_finder_center,
) -> libc::c_int {
    let mut ci: [libc::c_int; 7] = [0; 7];
    let mut maxd: libc::c_uint = 0;
    let mut ccw: libc::c_int = 0;
    let mut i0: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    /* Sort the points in counter-clockwise order. */
    ccw = qr_point_ccw(
        (**_c.offset(0 as libc::c_int as isize)).pos.as_mut_ptr() as *const libc::c_int,
        (**_c.offset(1 as libc::c_int as isize)).pos.as_mut_ptr() as *const libc::c_int,
        (**_c.offset(2 as libc::c_int as isize)).pos.as_mut_ptr() as *const libc::c_int,
    );
    /* Colinear points can't be the corners of a quadrilateral. */
    if ccw == 0 {
        return -(1 as libc::c_int);
    }
    /* Include a few extra copies of the cyclical list to avoid mods. */
    ci[0 as libc::c_int as usize] = 0 as libc::c_int;
    ci[3 as libc::c_int as usize] = ci[0 as libc::c_int as usize];
    ci[6 as libc::c_int as usize] = ci[3 as libc::c_int as usize];
    ci[1 as libc::c_int as usize] = 1 as libc::c_int + (ccw < 0 as libc::c_int) as libc::c_int;
    ci[4 as libc::c_int as usize] = ci[1 as libc::c_int as usize];
    ci[2 as libc::c_int as usize] = 2 as libc::c_int - (ccw < 0 as libc::c_int) as libc::c_int;
    ci[5 as libc::c_int as usize] = ci[2 as libc::c_int as usize];
    /*Assume the points farthest from each other are the opposite corners, and
    find the top-left point.*/
    maxd = qr_point_distance2(
        (**_c.offset(1 as libc::c_int as isize)).pos.as_mut_ptr() as *const libc::c_int,
        (**_c.offset(2 as libc::c_int as isize)).pos.as_mut_ptr() as *const libc::c_int,
    );
    i0 = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < 3 as libc::c_int {
        let mut d: libc::c_uint = 0;
        d = qr_point_distance2(
            (**_c.offset(ci[(i + 1 as libc::c_int) as usize] as isize)).pos.as_mut_ptr()
                as *const libc::c_int,
            (**_c.offset(ci[(i + 2 as libc::c_int) as usize] as isize)).pos.as_mut_ptr()
                as *const libc::c_int,
        );
        if d > maxd {
            i0 = i;
            maxd = d
        }
        i += 1
    }
    let mut current_block_118: u64;
    /*However, try all three possible orderings, just to be sure (a severely
    skewed projection could move opposite corners closer than adjacent).*/
    i = i0;
    while i < i0 + 3 as libc::c_int {
        let mut aff: qr_aff = qr_aff {
            fwd: [[0; 2]; 2],
            inv: [[0; 2]; 2],
            x0: 0,
            y0: 0,
            res: 0,
            ires: 0,
        };
        let mut hom: qr_hom = qr_hom {
            fwd: [[0; 2]; 3],
            inv: [[0; 2]; 3],
            fwd22: 0,
            inv22: 0,
            x0: 0,
            y0: 0,
            res: 0,
        };
        let mut ul: qr_finder = qr_finder {
            size: [0; 2],
            eversion: [0; 2],
            edge_pts: [0 as *mut qr_finder_edge_pt; 4],
            nedge_pts: [0; 4],
            ninliers: [0; 4],
            o: [0; 2],
            c: 0 as *mut qr_finder_center,
        };
        let mut ur: qr_finder = qr_finder {
            size: [0; 2],
            eversion: [0; 2],
            edge_pts: [0 as *mut qr_finder_edge_pt; 4],
            nedge_pts: [0; 4],
            ninliers: [0; 4],
            o: [0; 2],
            c: 0 as *mut qr_finder_center,
        };
        let mut dl: qr_finder = qr_finder {
            size: [0; 2],
            eversion: [0; 2],
            edge_pts: [0 as *mut qr_finder_edge_pt; 4],
            nedge_pts: [0; 4],
            ninliers: [0; 4],
            o: [0; 2],
            c: 0 as *mut qr_finder_center,
        };
        let mut bbox: [qr_point; 4] = [[0; 2]; 4];
        let mut res: libc::c_int = 0;
        let mut ur_version: libc::c_int = 0;
        let mut dl_version: libc::c_int = 0;
        let mut fmt_info: libc::c_int = 0;
        ul.c = *_c.offset(ci[i as usize] as isize);
        ur.c = *_c.offset(ci[(i + 1 as libc::c_int) as usize] as isize);
        dl.c = *_c.offset(ci[(i + 2 as libc::c_int) as usize] as isize);
        /*Estimate the module size and version number from the two opposite corners.
        The module size is not constant in the image, so we compute an affine
         projection from the three points we have to a square domain, and
         estimate it there.
        Although it should be the same along both axes, we keep separate
         estimates to account for any remaining projective distortion.*/
        res = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
            * 8 as libc::c_int
            - 2 as libc::c_int
            - 2 as libc::c_int
            - qr_ilog(
                (_width
                    - (_width - _height & -((_height > _width) as libc::c_int))
                    - 1 as libc::c_int) as libc::c_uint,
            );
        qr_aff_init(
            &mut aff,
            (*ul.c).pos.as_mut_ptr() as *const libc::c_int,
            (*ur.c).pos.as_mut_ptr() as *const libc::c_int,
            (*dl.c).pos.as_mut_ptr() as *const libc::c_int,
            res,
        );
        qr_aff_unproject(
            ur.o.as_mut_ptr(),
            &mut aff,
            (*ur.c).pos[0 as libc::c_int as usize],
            (*ur.c).pos[1 as libc::c_int as usize],
        );
        qr_finder_edge_pts_aff_classify(&mut ur, &mut aff);
        if !(qr_finder_estimate_module_size_and_version(
            &mut ur,
            (1 as libc::c_int) << res,
            (1 as libc::c_int) << res,
        ) < 0 as libc::c_int)
        {
            qr_aff_unproject(
                dl.o.as_mut_ptr(),
                &mut aff,
                (*dl.c).pos[0 as libc::c_int as usize],
                (*dl.c).pos[1 as libc::c_int as usize],
            );
            qr_finder_edge_pts_aff_classify(&mut dl, &mut aff);
            if !(qr_finder_estimate_module_size_and_version(
                &mut dl,
                (1 as libc::c_int) << res,
                (1 as libc::c_int) << res,
            ) < 0 as libc::c_int)
            {
                /*If the estimated versions are significantly different, reject the
                configuration.*/
                if !(abs(
                    ur.eversion[1 as libc::c_int as usize] - dl.eversion[0 as libc::c_int as usize]
                ) > 3 as libc::c_int)
                {
                    qr_aff_unproject(
                        ul.o.as_mut_ptr(),
                        &mut aff,
                        (*ul.c).pos[0 as libc::c_int as usize],
                        (*ul.c).pos[1 as libc::c_int as usize],
                    );
                    qr_finder_edge_pts_aff_classify(&mut ul, &mut aff);
                    if !(qr_finder_estimate_module_size_and_version(
                        &mut ul,
                        (1 as libc::c_int) << res,
                        (1 as libc::c_int) << res,
                    ) < 0 as libc::c_int
                        || abs(ul.eversion[1 as libc::c_int as usize]
                            - ur.eversion[1 as libc::c_int as usize])
                            > 3 as libc::c_int
                        || abs(ul.eversion[0 as libc::c_int as usize]
                            - dl.eversion[0 as libc::c_int as usize])
                            > 3 as libc::c_int)
                    {
                        /*If we made it this far, upgrade the affine homography to a full
                        homography.*/
                        if !(qr_hom_fit(
                            &mut hom,
                            &mut ul,
                            &mut ur,
                            &mut dl,
                            bbox.as_mut_ptr(),
                            &mut aff,
                            &mut (*_reader).isaac,
                            _img,
                            _width,
                            _height,
                        ) < 0 as libc::c_int)
                        {
                            memcpy(
                                (*_qrdata).bbox.as_mut_ptr() as *mut libc::c_void,
                                bbox.as_mut_ptr() as *const libc::c_void,
                                ::std::mem::size_of::<[qr_point; 4]>() as libc::c_ulong,
                            );
                            qr_hom_unproject(
                                ul.o.as_mut_ptr(),
                                &mut hom,
                                (*ul.c).pos[0 as libc::c_int as usize],
                                (*ul.c).pos[1 as libc::c_int as usize],
                            );
                            qr_hom_unproject(
                                ur.o.as_mut_ptr(),
                                &mut hom,
                                (*ur.c).pos[0 as libc::c_int as usize],
                                (*ur.c).pos[1 as libc::c_int as usize],
                            );
                            qr_hom_unproject(
                                dl.o.as_mut_ptr(),
                                &mut hom,
                                (*dl.c).pos[0 as libc::c_int as usize],
                                (*dl.c).pos[1 as libc::c_int as usize],
                            );
                            qr_finder_edge_pts_hom_classify(&mut ur, &mut hom);
                            if !(qr_finder_estimate_module_size_and_version(
                                &mut ur,
                                ur.o[0 as libc::c_int as usize] - ul.o[0 as libc::c_int as usize],
                                ur.o[0 as libc::c_int as usize] - ul.o[0 as libc::c_int as usize],
                            ) < 0 as libc::c_int)
                            {
                                qr_finder_edge_pts_hom_classify(&mut dl, &mut hom);
                                if !(qr_finder_estimate_module_size_and_version(
                                    &mut dl,
                                    dl.o[1 as libc::c_int as usize]
                                        - ul.o[1 as libc::c_int as usize],
                                    dl.o[1 as libc::c_int as usize]
                                        - ul.o[1 as libc::c_int as usize],
                                ) < 0 as libc::c_int)
                                {
                                    /*If we have a small version (less than 7), there's no encoded version
                                     information.
                                    If the estimated version on the two corners matches and is sufficiently
                                     small, we assume this is the case.*/
                                    if ur.eversion[1 as libc::c_int as usize]
                                        == dl.eversion[0 as libc::c_int as usize]
                                        && ur.eversion[1 as libc::c_int as usize] < 7 as libc::c_int
                                    {
                                        /*We used to do a whole bunch of extra geometric checks for small
                                         versions, because with just an affine correction, it was fairly easy
                                         to estimate two consistent module sizes given a random configuration.
                                        However, now that we're estimating a full homography, these appear to
                                         be unnecessary.*/
                                        ur_version = ur.eversion[1 as libc::c_int as usize];
                                        current_block_118 = 129780949503461575;
                                    } else if abs(ur.eversion[1 as libc::c_int as usize]
                                        - dl.eversion[0 as libc::c_int as usize])
                                        > 3 as libc::c_int
                                    {
                                        current_block_118 = 5948590327928692120;
                                    } else {
                                        /*If the estimated versions are significantly different, reject the
                                        configuration.*/
                                        /*Otherwise we try to read the actual version data from the image.
                                        If the real version is not sufficiently close to our estimated version,
                                         then we assume there was an unrecoverable decoding error (so many bit
                                         errors we were within 3 errors of another valid code), and throw that
                                         value away.
                                        If no decoded version could be sufficiently close, we don't even try.*/
                                        if ur.eversion[1 as libc::c_int as usize]
                                            >= 7 as libc::c_int - 3 as libc::c_int
                                        {
                                            ur_version = qr_finder_version_decode(
                                                &mut ur,
                                                &mut hom,
                                                _img,
                                                _width,
                                                _height,
                                                0 as libc::c_int,
                                            );
                                            if abs(
                                                ur_version - ur.eversion[1 as libc::c_int as usize]
                                            ) > 3 as libc::c_int
                                            {
                                                ur_version = -(1 as libc::c_int)
                                            }
                                        } else {
                                            ur_version = -(1 as libc::c_int)
                                        }
                                        if dl.eversion[0 as libc::c_int as usize]
                                            >= 7 as libc::c_int - 3 as libc::c_int
                                        {
                                            dl_version = qr_finder_version_decode(
                                                &mut dl,
                                                &mut hom,
                                                _img,
                                                _width,
                                                _height,
                                                1 as libc::c_int,
                                            );
                                            if abs(
                                                dl_version - dl.eversion[0 as libc::c_int as usize]
                                            ) > 3 as libc::c_int
                                            {
                                                dl_version = -(1 as libc::c_int)
                                            }
                                        } else {
                                            dl_version = -(1 as libc::c_int)
                                        }
                                        /*If we got at least one valid version, or we got two and they match,
                                        then we found a valid configuration.*/
                                        if ur_version >= 0 as libc::c_int {
                                            if dl_version >= 0 as libc::c_int
                                                && dl_version != ur_version
                                            {
                                                current_block_118 = 5948590327928692120;
                                            } else {
                                                current_block_118 = 129780949503461575;
                                            }
                                        } else if dl_version < 0 as libc::c_int {
                                            current_block_118 = 5948590327928692120;
                                        } else {
                                            ur_version = dl_version;
                                            current_block_118 = 129780949503461575;
                                        }
                                    }
                                    match current_block_118 {
                                        5948590327928692120 => {}
                                        _ => {
                                            qr_finder_edge_pts_hom_classify(&mut ul, &mut hom);
                                            if !(qr_finder_estimate_module_size_and_version(
                                                &mut ul,
                                                ur.o[0 as libc::c_int as usize]
                                                    - dl.o[0 as libc::c_int as usize],
                                                dl.o[1 as libc::c_int as usize]
                                                    - ul.o[1 as libc::c_int as usize],
                                            ) < 0 as libc::c_int
                                                || abs(ul.eversion[1 as libc::c_int as usize]
                                                    - ur.eversion[1 as libc::c_int as usize])
                                                    > 1 as libc::c_int
                                                || abs(ul.eversion[0 as libc::c_int as usize]
                                                    - dl.eversion[0 as libc::c_int as usize])
                                                    > 1 as libc::c_int)
                                            {
                                                fmt_info = qr_finder_fmt_info_decode(
                                                    &mut ul, &mut ur, &mut dl, &mut hom, _img,
                                                    _width, _height,
                                                );
                                                if fmt_info < 0 as libc::c_int
                                                    || qr_code_decode(
                                                        _qrdata,
                                                        &mut (*_reader).gf,
                                                        (*ul.c).pos.as_mut_ptr()
                                                            as *const libc::c_int,
                                                        (*ur.c).pos.as_mut_ptr()
                                                            as *const libc::c_int,
                                                        (*dl.c).pos.as_mut_ptr()
                                                            as *const libc::c_int,
                                                        ur_version,
                                                        fmt_info,
                                                        _img,
                                                        _width,
                                                        _height,
                                                    ) < 0 as libc::c_int
                                                {
                                                    /*The code may be flipped.
                                                    Try again, swapping the UR and DL centers.
                                                    We should get a valid version either way, so it's relatively cheap to
                                                     check this, as we've already filtered out a lot of invalid
                                                     configurations.*/
                                                    let mut t__: libc::c_int = 0;
                                                    t__ = hom.inv[0 as libc::c_int as usize]
                                                        [0 as libc::c_int as usize];
                                                    hom.inv[0 as libc::c_int as usize]
                                                        [0 as libc::c_int as usize] = hom.inv
                                                        [1 as libc::c_int as usize]
                                                        [0 as libc::c_int as usize];
                                                    hom.inv[1 as libc::c_int as usize]
                                                        [0 as libc::c_int as usize] = t__;
                                                    let mut t___0: libc::c_int = 0;
                                                    t___0 = hom.inv[0 as libc::c_int as usize]
                                                        [1 as libc::c_int as usize];
                                                    hom.inv[0 as libc::c_int as usize]
                                                        [1 as libc::c_int as usize] = hom.inv
                                                        [1 as libc::c_int as usize]
                                                        [1 as libc::c_int as usize];
                                                    hom.inv[1 as libc::c_int as usize]
                                                        [1 as libc::c_int as usize] = t___0;
                                                    let mut t___1: libc::c_int = 0;
                                                    t___1 = hom.fwd[0 as libc::c_int as usize]
                                                        [0 as libc::c_int as usize];
                                                    hom.fwd[0 as libc::c_int as usize]
                                                        [0 as libc::c_int as usize] = hom.fwd
                                                        [0 as libc::c_int as usize]
                                                        [1 as libc::c_int as usize];
                                                    hom.fwd[0 as libc::c_int as usize]
                                                        [1 as libc::c_int as usize] = t___1;
                                                    let mut t___2: libc::c_int = 0;
                                                    t___2 = hom.fwd[1 as libc::c_int as usize]
                                                        [0 as libc::c_int as usize];
                                                    hom.fwd[1 as libc::c_int as usize]
                                                        [0 as libc::c_int as usize] = hom.fwd
                                                        [1 as libc::c_int as usize]
                                                        [1 as libc::c_int as usize];
                                                    hom.fwd[1 as libc::c_int as usize]
                                                        [1 as libc::c_int as usize] = t___2;
                                                    let mut t___3: libc::c_int = 0;
                                                    t___3 = hom.fwd[2 as libc::c_int as usize]
                                                        [0 as libc::c_int as usize];
                                                    hom.fwd[2 as libc::c_int as usize]
                                                        [0 as libc::c_int as usize] = hom.fwd
                                                        [2 as libc::c_int as usize]
                                                        [1 as libc::c_int as usize];
                                                    hom.fwd[2 as libc::c_int as usize]
                                                        [1 as libc::c_int as usize] = t___3;
                                                    let mut t___4: libc::c_int = 0;
                                                    t___4 = ul.o[0 as libc::c_int as usize];
                                                    ul.o[0 as libc::c_int as usize] =
                                                        ul.o[1 as libc::c_int as usize];
                                                    ul.o[1 as libc::c_int as usize] = t___4;
                                                    let mut t___5: libc::c_int = 0;
                                                    t___5 = ul.size[0 as libc::c_int as usize];
                                                    ul.size[0 as libc::c_int as usize] =
                                                        ul.size[1 as libc::c_int as usize];
                                                    ul.size[1 as libc::c_int as usize] = t___5;
                                                    let mut t___6: libc::c_int = 0;
                                                    t___6 = ur.o[0 as libc::c_int as usize];
                                                    ur.o[0 as libc::c_int as usize] =
                                                        ur.o[1 as libc::c_int as usize];
                                                    ur.o[1 as libc::c_int as usize] = t___6;
                                                    let mut t___7: libc::c_int = 0;
                                                    t___7 = ur.size[0 as libc::c_int as usize];
                                                    ur.size[0 as libc::c_int as usize] =
                                                        ur.size[1 as libc::c_int as usize];
                                                    ur.size[1 as libc::c_int as usize] = t___7;
                                                    let mut t___8: libc::c_int = 0;
                                                    t___8 = dl.o[0 as libc::c_int as usize];
                                                    dl.o[0 as libc::c_int as usize] =
                                                        dl.o[1 as libc::c_int as usize];
                                                    dl.o[1 as libc::c_int as usize] = t___8;
                                                    let mut t___9: libc::c_int = 0;
                                                    t___9 = dl.size[0 as libc::c_int as usize];
                                                    dl.size[0 as libc::c_int as usize] =
                                                        dl.size[1 as libc::c_int as usize];
                                                    dl.size[1 as libc::c_int as usize] = t___9;
                                                    fmt_info = qr_finder_fmt_info_decode(
                                                        &mut ul, &mut dl, &mut ur, &mut hom, _img,
                                                        _width, _height,
                                                    );
                                                    if fmt_info < 0 as libc::c_int {
                                                        current_block_118 = 5948590327928692120;
                                                    } else {
                                                        let mut t___10: libc::c_int = 0;
                                                        t___10 = bbox[1 as libc::c_int as usize]
                                                            [0 as libc::c_int as usize];
                                                        bbox[1 as libc::c_int as usize]
                                                            [0 as libc::c_int as usize] = bbox
                                                            [2 as libc::c_int as usize]
                                                            [0 as libc::c_int as usize];
                                                        bbox[2 as libc::c_int as usize]
                                                            [0 as libc::c_int as usize] = t___10;
                                                        let mut t___11: libc::c_int = 0;
                                                        t___11 = bbox[1 as libc::c_int as usize]
                                                            [1 as libc::c_int as usize];
                                                        bbox[1 as libc::c_int as usize]
                                                            [1 as libc::c_int as usize] = bbox
                                                            [2 as libc::c_int as usize]
                                                            [1 as libc::c_int as usize];
                                                        bbox[2 as libc::c_int as usize]
                                                            [1 as libc::c_int as usize] = t___11;
                                                        memcpy(
                                                            (*_qrdata).bbox.as_mut_ptr()
                                                                as *mut libc::c_void,
                                                            bbox.as_mut_ptr()
                                                                as *const libc::c_void,
                                                            ::std::mem::size_of::<[qr_point; 4]>()
                                                                as libc::c_ulong,
                                                        );
                                                        if qr_code_decode(
                                                            _qrdata,
                                                            &mut (*_reader).gf,
                                                            (*ul.c).pos.as_mut_ptr()
                                                                as *const libc::c_int,
                                                            (*dl.c).pos.as_mut_ptr()
                                                                as *const libc::c_int,
                                                            (*ur.c).pos.as_mut_ptr()
                                                                as *const libc::c_int,
                                                            ur_version,
                                                            fmt_info,
                                                            _img,
                                                            _width,
                                                            _height,
                                                        ) < 0 as libc::c_int
                                                        {
                                                            current_block_118 = 5948590327928692120;
                                                        } else {
                                                            current_block_118 =
                                                                10601179871800211547;
                                                        }
                                                    }
                                                } else {
                                                    current_block_118 = 10601179871800211547;
                                                }
                                                match current_block_118 {
                                                    5948590327928692120 => {}
                                                    _ => return ur_version,
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern fn qr_reader_match_centers(
    mut _reader: *mut qr_reader,
    mut _qrlist: *mut qr_code_data_list,
    mut _centers: *mut qr_finder_center,
    mut _ncenters: libc::c_int,
    mut _img: *const libc::c_uchar,
    mut _width: libc::c_int,
    mut _height: libc::c_int,
) {
    /*The number of centers should be small, so an O(n^3) exhaustive search of
    which ones go together should be reasonable.*/
    let mut mark: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut nfailures_max: libc::c_int = 0;
    let mut nfailures: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    mark =
        calloc(_ncenters as libc::c_ulong, ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            as *mut libc::c_uchar;
    nfailures_max = 8192 as libc::c_int
        - (8192 as libc::c_int - (_width * _height >> 9 as libc::c_int)
            & -((_width * _height >> 9 as libc::c_int > 8192 as libc::c_int) as libc::c_int));
    nfailures = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < _ncenters {
        /*TODO: We might be able to accelerate this step significantly by
        considering the remaining finder centers in a more intelligent order,
        based on the first finder center we just chose.*/
        j = i + 1 as libc::c_int;
        while *mark.offset(i as isize) == 0 && j < _ncenters {
            k = j + 1 as libc::c_int;
            while *mark.offset(j as isize) == 0 && k < _ncenters {
                if *mark.offset(k as isize) == 0 {
                    let mut c: [*mut qr_finder_center; 3] = [0 as *mut qr_finder_center; 3];
                    let mut qrdata: qr_code_data = qr_code_data {
                        entries: 0 as *mut qr_code_data_entry,
                        nentries: 0,
                        version: 0,
                        ecc_level: 0,
                        sa_index: 0,
                        sa_size: 0,
                        sa_parity: 0,
                        self_parity: 0,
                        bbox: [[0; 2]; 4],
                    };
                    let mut version: libc::c_int = 0;
                    c[0 as libc::c_int as usize] = _centers.offset(i as isize);
                    c[1 as libc::c_int as usize] = _centers.offset(j as isize);
                    c[2 as libc::c_int as usize] = _centers.offset(k as isize);
                    version = qr_reader_try_configuration(
                        _reader,
                        &mut qrdata,
                        _img,
                        _width,
                        _height,
                        c.as_mut_ptr(),
                    );
                    if version >= 0 as libc::c_int {
                        let mut ninside: libc::c_int = 0;
                        let mut l: libc::c_int = 0;
                        /* Add the data to the list. */
                        qr_code_data_list_add(_qrlist, &mut qrdata);
                        /*Convert the bounding box we're returning to the user to normal
                        image coordinates.*/
                        l = 0 as libc::c_int;
                        while l < 4 as libc::c_int {
                            (*(*_qrlist)
                                .qrdata
                                .offset(((*_qrlist).nqrdata - 1 as libc::c_int) as isize))
                            .bbox[l as usize][0 as libc::c_int as usize] >>= 2 as libc::c_int;
                            (*(*_qrlist)
                                .qrdata
                                .offset(((*_qrlist).nqrdata - 1 as libc::c_int) as isize))
                            .bbox[l as usize][1 as libc::c_int as usize] >>= 2 as libc::c_int;
                            l += 1
                        }
                        /* Mark these centers as used. */
                        let ref mut fresh43 = *mark.offset(k as isize);
                        *fresh43 = 1 as libc::c_int as libc::c_uchar;
                        let ref mut fresh44 = *mark.offset(j as isize);
                        *fresh44 = *fresh43;
                        *mark.offset(i as isize) = *fresh44;
                        /* Find any other finder centers located inside this code. */
                        ninside = 0 as libc::c_int;
                        l = ninside;
                        while l < _ncenters {
                            if *mark.offset(l as isize) == 0 {
                                if qr_point_ccw(
                                    qrdata.bbox[0 as libc::c_int as usize].as_mut_ptr()
                                        as *const libc::c_int,
                                    qrdata.bbox[1 as libc::c_int as usize].as_mut_ptr()
                                        as *const libc::c_int,
                                    (*_centers.offset(l as isize)).pos.as_mut_ptr()
                                        as *const libc::c_int,
                                ) >= 0 as libc::c_int
                                    && qr_point_ccw(
                                        qrdata.bbox[1 as libc::c_int as usize].as_mut_ptr()
                                            as *const libc::c_int,
                                        qrdata.bbox[3 as libc::c_int as usize].as_mut_ptr()
                                            as *const libc::c_int,
                                        (*_centers.offset(l as isize)).pos.as_mut_ptr()
                                            as *const libc::c_int,
                                    ) >= 0 as libc::c_int
                                    && qr_point_ccw(
                                        qrdata.bbox[3 as libc::c_int as usize].as_mut_ptr()
                                            as *const libc::c_int,
                                        qrdata.bbox[2 as libc::c_int as usize].as_mut_ptr()
                                            as *const libc::c_int,
                                        (*_centers.offset(l as isize)).pos.as_mut_ptr()
                                            as *const libc::c_int,
                                    ) >= 0 as libc::c_int
                                    && qr_point_ccw(
                                        qrdata.bbox[2 as libc::c_int as usize].as_mut_ptr()
                                            as *const libc::c_int,
                                        qrdata.bbox[0 as libc::c_int as usize].as_mut_ptr()
                                            as *const libc::c_int,
                                        (*_centers.offset(l as isize)).pos.as_mut_ptr()
                                            as *const libc::c_int,
                                    ) >= 0 as libc::c_int
                                {
                                    *mark.offset(l as isize) = 2 as libc::c_int as libc::c_uchar;
                                    ninside += 1
                                }
                            }
                            l += 1
                        }
                        if ninside >= 3 as libc::c_int {
                            /*We might have a "Double QR": a code inside a code.
                            Copy the relevant centers to a new array and do a search confined
                             to that subset.*/
                            let mut inside: *mut qr_finder_center = 0 as *mut qr_finder_center;
                            inside = malloc((ninside as libc::c_ulong).wrapping_mul(
                                ::std::mem::size_of::<qr_finder_center>() as libc::c_ulong,
                            )) as *mut qr_finder_center;
                            ninside = 0 as libc::c_int;
                            l = ninside;
                            while l < _ncenters {
                                if *mark.offset(l as isize) as libc::c_int == 2 as libc::c_int {
                                    let fresh45 = ninside;
                                    ninside = ninside + 1;
                                    *inside.offset(fresh45 as isize) = *_centers.offset(l as isize)
                                }
                                l += 1
                            }
                            qr_reader_match_centers(
                                _reader, _qrlist, inside, ninside, _img, _width, _height,
                            );
                            free(inside as *mut libc::c_void);
                        }
                        /* Mark _all_ such centers used: codes cannot partially overlap. */
                        l = 0 as libc::c_int;
                        while l < _ncenters {
                            if *mark.offset(l as isize) as libc::c_int == 2 as libc::c_int {
                                *mark.offset(l as isize) = 1 as libc::c_int as libc::c_uchar
                            }
                            l += 1
                        }
                        nfailures = 0 as libc::c_int
                    } else {
                        nfailures += 1;
                        if nfailures > nfailures_max {
                            /*Give up.
                            We're unlikely to find a valid code in all this clutter, and we
                             could spent quite a lot of time trying.*/
                            k = _ncenters;
                            j = k;
                            i = j
                        }
                    }
                }
                k += 1
            }
            j += 1
        }
        i += 1
    }
    free(mark as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern fn _zbar_qr_found_line(
    mut reader: *mut qr_reader,
    mut dir: libc::c_int,
    mut line: *const qr_finder_line,
) -> libc::c_int {
    /* minimally intrusive brute force version */
    let mut lines: *mut qr_finder_lines =
        &mut *(*reader).finder_lines.as_mut_ptr().offset(dir as isize) as *mut qr_finder_lines;
    if (*lines).nlines >= (*lines).clines {
        (*lines).clines *= 2 as libc::c_int;
        (*lines).clines += 1;
        (*lines).lines = realloc(
            (*lines).lines as *mut libc::c_void,
            ((*lines).clines as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<qr_finder_line>() as libc::c_ulong),
        ) as *mut qr_finder_line
    }
    let fresh46 = (*lines).nlines;
    (*lines).nlines = (*lines).nlines + 1;
    memcpy(
        (*lines).lines.offset(fresh46 as isize) as *mut libc::c_void,
        line as *const libc::c_void,
        ::std::mem::size_of::<qr_finder_line>() as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
#[inline]
unsafe extern fn qr_svg_centers(mut centers: *const qr_finder_center, mut ncenters: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < ncenters {
        i += 1
    }
    i = 0 as libc::c_int;
    while i < ncenters {
        let mut cen: *const qr_finder_center = centers.offset(i as isize);
        j = 0 as libc::c_int;
        while j < (*cen).nedge_pts {
            j += 1
        }
        i += 1
    }
}
/*The location of the upper/left endpoint of the line.
The left/upper edge of the center section is used, since other lines must
 cross in this region.*/
/*The length of the center section.
This extends to the right/bottom of the center section, since other lines
 must cross in this region.*/
/*The offset to the midpoint of the upper/left section (part of the outside
 ring), or 0 if we couldn't identify the edge of the beginning section.
We use the midpoint instead of the edge because it can be located more
 reliably.*/
/*The offset to the midpoint of the end section (part of the outside ring),
 or 0 if we couldn't identify the edge of the end section.
We use the midpoint instead of the edge because it can be located more
 reliably.*/
#[no_mangle]
pub unsafe extern fn _zbar_qr_decode(
    mut reader: *mut qr_reader,
    mut iscn: *mut zbar_image_scanner_t,
    mut img: *mut zbar_image_t,
) -> libc::c_int {
    let mut nqrdata: libc::c_int = 0 as libc::c_int;
    let mut ncenters: libc::c_int = 0;
    let mut edge_pts: *mut qr_finder_edge_pt = 0 as *mut qr_finder_edge_pt;
    let mut centers: *mut qr_finder_center = 0 as *mut qr_finder_center;
    if (*reader).finder_lines[0 as libc::c_int as usize].nlines < 9 as libc::c_int
        || (*reader).finder_lines[1 as libc::c_int as usize].nlines < 9 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    ncenters = qr_finder_centers_locate(
        &mut centers,
        &mut edge_pts,
        reader,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    if _zbar_verbosity >= 14 as libc::c_int {
        fprintf(
            stderr,
            b"%s: %dx%d finders, %d centers:\n\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"_zbar_qr_decode\x00"))
                .as_ptr(),
            (*reader).finder_lines[0 as libc::c_int as usize].nlines,
            (*reader).finder_lines[1 as libc::c_int as usize].nlines,
            ncenters,
        );
    }
    qr_svg_centers(centers, ncenters);
    if ncenters >= 3 as libc::c_int {
        let mut bin: *mut libc::c_void = qr_binarize(
            (*img).data as *const libc::c_uchar,
            (*img).width as libc::c_int,
            (*img).height as libc::c_int,
        ) as *mut libc::c_void;
        let mut qrlist: qr_code_data_list = qr_code_data_list {
            qrdata: 0 as *mut qr_code_data,
            nqrdata: 0,
            cqrdata: 0,
        };
        qr_code_data_list_init(&mut qrlist);
        qr_reader_match_centers(
            reader,
            &mut qrlist,
            centers,
            ncenters,
            bin as *const libc::c_uchar,
            (*img).width as libc::c_int,
            (*img).height as libc::c_int,
        );
        if qrlist.nqrdata > 0 as libc::c_int {
            nqrdata = qr_code_data_list_extract_text(&mut qrlist, iscn, img)
        }
        qr_code_data_list_clear(&mut qrlist);
        free(bin);
    }
    if !centers.is_null() {
        free(centers as *mut libc::c_void);
    }
    if !edge_pts.is_null() {
        free(edge_pts as *mut libc::c_void);
    }
    return nqrdata;
}
