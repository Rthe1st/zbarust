use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    /* acquire shared state lock */
    /* check and release shared state lock */
    /* ensure output buffer has sufficient allocation for request */
    /* FIXME size reduction heuristic? */
    #[no_mangle]
    fn _zbar_decoder_buf_dump(buf: *mut libc::c_uchar, buflen: libc::c_uint)
     -> *const libc::c_char;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type zbar_color_e = libc::c_uint;
pub const ZBAR_BAR: zbar_color_e = 1;
pub const ZBAR_SPACE: zbar_color_e = 0;
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
/* realloc */
/* size of bar width history (implementation assumes power of two) */
/* initial data buffer allocation */
/* maximum data buffer allocation
 * (longer symbols are rejected)
 */
/* buffer allocation increment */
/* symbology independent decoder state */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zbar_decoder_s {
    pub idx: libc::c_uchar,
    pub w: [libc::c_uint; 16],
    pub type_0: zbar_symbol_type_t,
    pub lock: zbar_symbol_type_t,
    pub modifiers: libc::c_uint,
    pub direction: libc::c_int,
    pub s6: libc::c_uint,
    pub buf_alloc: libc::c_uint,
    pub buflen: libc::c_uint,
    pub buf: *mut libc::c_uchar,
    pub userdata: *mut libc::c_void,
    pub handler: Option<zbar_decoder_handler_t>,
    pub ean: ean_decoder_t,
    pub i25: i25_decoder_t,
    pub databar: databar_decoder_t,
    pub codabar: codabar_decoder_t,
    pub code39: code39_decoder_t,
    pub code93: code93_decoder_t,
    pub code128: code128_decoder_t,
    pub qrf: qr_finder_t,
}
/* QR Code symbol finder state */
pub type qr_finder_t = qr_finder_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qr_finder_s {
    pub s5: libc::c_uint,
    pub line: qr_finder_line,
    pub config: libc::c_uint,
}
/* finder pattern width */
/* position info needed by decoder */
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
/*Copyright (C) 2008-2009  Timothy B. Terriberry (tterribe@xiph.org)
  You can redistribute this library and/or modify it under the terms of the
   GNU Lesser General Public License as published by the Free Software
   Foundation; either version 2.1 of the License, or (at your option) any later
   version.*/
pub type qr_point = [libc::c_int; 2];
pub type code128_decoder_t = code128_decoder_s;
/* int valued configurations */
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
/* Code 128 specific decode state */
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct code128_decoder_s {
    #[bitfield(name = "direction", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "element", ty = "libc::c_uint", bits = "1..=3")]
    #[bitfield(name = "character", ty = "libc::c_int", bits = "4..=15")]
    pub direction_element_character: [u8; 2],
    pub start: libc::c_uchar,
    pub s6: libc::c_uint,
    pub width: libc::c_uint,
    pub config: libc::c_uint,
    pub configs: [libc::c_int; 2],
}
pub type code93_decoder_t = code93_decoder_s;
/* int valued configurations */
/*------------------------------------------------------------------------
 *  Copyright 2010 (c) Jeff Brown <spadix@users.sourceforge.net>
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
/* Code 93 specific decode state */
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct code93_decoder_s {
    #[bitfield(name = "direction", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "element", ty = "libc::c_uint", bits = "1..=3")]
    #[bitfield(name = "character", ty = "libc::c_int", bits = "4..=15")]
    pub direction_element_character: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
    pub width: libc::c_uint,
    pub buf: libc::c_uchar,
    pub config: libc::c_uint,
    pub configs: [libc::c_int; 2],
}
pub type code39_decoder_t = code39_decoder_s;
/* int valued configurations */
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
 *------------------------------------------------------------------------*/
/* Code 39 specific decode state */
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct code39_decoder_s {
    #[bitfield(name = "direction", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "element", ty = "libc::c_uint", bits = "1..=4")]
    #[bitfield(name = "character", ty = "libc::c_int", bits = "5..=16")]
    pub direction_element_character: [u8; 3],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub s9: libc::c_uint,
    pub width: libc::c_uint,
    pub config: libc::c_uint,
    pub configs: [libc::c_int; 2],
}
pub type codabar_decoder_t = codabar_decoder_s;
/* int valued configurations */
/*------------------------------------------------------------------------
 *  Copyright 2011 (c) Jeff Brown <spadix@users.sourceforge.net>
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
/* Codabar specific decode state */
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct codabar_decoder_s {
    #[bitfield(name = "direction", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "element", ty = "libc::c_uint", bits = "1..=4")]
    #[bitfield(name = "character", ty = "libc::c_int", bits = "5..=16")]
    pub direction_element_character: [u8; 3],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub s7: libc::c_uint,
    pub width: libc::c_uint,
    pub buf: [libc::c_uchar; 6],
    pub config: libc::c_uint,
    pub configs: [libc::c_int; 2],
}
pub type databar_decoder_t = databar_decoder_s;
/* outstanding character indices */
/* DataBar specific decode state */
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct databar_decoder_s {
    pub config: libc::c_uint,
    pub config_exp: libc::c_uint,
    #[bitfield(name = "csegs", ty = "libc::c_uint", bits = "0..=7")]
    #[bitfield(name = "epoch", ty = "libc::c_uint", bits = "8..=15")]
    pub csegs_epoch: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 6],
    pub segs: *mut databar_segment_t,
    pub chars: [libc::c_schar; 16],
}
pub type databar_segment_t = databar_segment_s;
/* measured width of finder (14 modules) */
/*------------------------------------------------------------------------
 *  Copyright 2010 (c) Jeff Brown <spadix@users.sourceforge.net>
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
/* active DataBar (partial) segment entry */
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct databar_segment_s {
    #[bitfield(name = "finder", ty = "libc::c_int", bits = "0..=4")]
    #[bitfield(name = "exp", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "color", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "side", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "partial", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "count", ty = "libc::c_uint", bits = "9..=15")]
    #[bitfield(name = "epoch", ty = "libc::c_uint", bits = "16..=23")]
    #[bitfield(name = "check", ty = "libc::c_uint", bits = "24..=31")]
    pub finder_exp_color_side_partial_count_epoch_check: [u8; 4],
    pub data: libc::c_short,
    pub width: libc::c_ushort,
}
pub type i25_decoder_t = i25_decoder_s;
/* int valued configurations */
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
 *------------------------------------------------------------------------*/
/* interleaved 2 of 5 specific decode state */
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct i25_decoder_s {
    #[bitfield(name = "direction", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "element", ty = "libc::c_uint", bits = "1..=4")]
    #[bitfield(name = "character", ty = "libc::c_int", bits = "5..=16")]
    pub direction_element_character: [u8; 3],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub s10: libc::c_uint,
    pub width: libc::c_uint,
    pub buf: [libc::c_uchar; 4],
    pub config: libc::c_uint,
    pub configs: [libc::c_int; 2],
}
pub type ean_decoder_t = ean_decoder_s;
/* EAN/UPC specific decode state */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ean_decoder_s {
    pub pass: [ean_pass_t; 4],
    pub left: zbar_symbol_type_t,
    pub right: zbar_symbol_type_t,
    pub direction: libc::c_int,
    pub s4: libc::c_uint,
    pub width: libc::c_uint,
    pub buf: [libc::c_schar; 18],
    pub enable: libc::c_schar,
    pub ean13_config: libc::c_uint,
    pub ean8_config: libc::c_uint,
    pub upca_config: libc::c_uint,
    pub upce_config: libc::c_uint,
    pub isbn10_config: libc::c_uint,
    pub isbn13_config: libc::c_uint,
    pub ean5_config: libc::c_uint,
    pub ean2_config: libc::c_uint,
}
pub type ean_pass_t = ean_pass_s;
/* decode in process */
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
/* state of each parallel decode attempt */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ean_pass_s {
    pub state: libc::c_schar,
    pub width: libc::c_uint,
    pub raw: [libc::c_uchar; 7],
}
pub type zbar_decoder_handler_t
    =
    unsafe extern "C" fn(_: *mut zbar_decoder_t) -> ();
pub type zbar_decoder_t = zbar_decoder_s;
pub type size_t = libc::c_ulong;
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
unsafe extern "C" fn get_color(mut dcode: *const zbar_decoder_t)
 -> libc::c_char {
    return ((*dcode).idx as libc::c_int & 1 as libc::c_int) as libc::c_char;
}
#[inline]
unsafe extern "C" fn decode_e(mut e: libc::c_uint, mut s: libc::c_uint,
                              mut n: libc::c_uint) -> libc::c_int {
    let mut E: libc::c_uchar =
        e.wrapping_mul(n).wrapping_mul(2 as libc::c_int as
                                           libc::c_uint).wrapping_add(1 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint).wrapping_div(s).wrapping_sub(3
                                                                                                                         as
                                                                                                                         libc::c_int
                                                                                                                         as
                                                                                                                         libc::c_uint).wrapping_div(2
                                                                                                                                                        as
                                                                                                                                                        libc::c_int
                                                                                                                                                        as
                                                                                                                                                        libc::c_uint)
            as libc::c_uchar;
    return if E as libc::c_uint >=
                  n.wrapping_sub(3 as libc::c_int as libc::c_uint) {
               -(1 as libc::c_int)
           } else { E as libc::c_int };
}
#[inline]
unsafe extern "C" fn get_width(mut dcode: *const zbar_decoder_t,
                               mut offset: libc::c_uchar) -> libc::c_uint {
    return (*dcode).w[((*dcode).idx as libc::c_int - offset as libc::c_int &
                           16 as libc::c_int - 1 as libc::c_int) as usize];
}
#[inline]
unsafe extern "C" fn pair_width(mut dcode: *const zbar_decoder_t,
                                mut offset: libc::c_uchar) -> libc::c_uint {
    return get_width(dcode,
                     offset).wrapping_add(get_width(dcode,
                                                    (offset as libc::c_int +
                                                         1 as libc::c_int) as
                                                        libc::c_uchar));
}
#[inline]
unsafe extern "C" fn size_buf(mut dcode: *mut zbar_decoder_t,
                              mut len: libc::c_uint) -> libc::c_char {
    let mut buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if len <= 0x20 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as libc::c_char
    }
    if len < (*dcode).buf_alloc { return 0 as libc::c_int as libc::c_char }
    if len > 0x100 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int as libc::c_char
    }
    if len <
           (*dcode).buf_alloc.wrapping_add(0x10 as libc::c_int as
                                               libc::c_uint) {
        len =
            (*dcode).buf_alloc.wrapping_add(0x10 as libc::c_int as
                                                libc::c_uint);
        if len > 0x100 as libc::c_int as libc::c_uint {
            len = 0x100 as libc::c_int as libc::c_uint
        }
    }
    buf =
        realloc((*dcode).buf as *mut libc::c_void, len as libc::c_ulong) as
            *mut libc::c_uchar;
    if buf.is_null() { return 1 as libc::c_int as libc::c_char }
    (*dcode).buf = buf;
    (*dcode).buf_alloc = len;
    return 0 as libc::c_int as libc::c_char;
}
#[inline]
unsafe extern "C" fn acquire_lock(mut dcode: *mut zbar_decoder_t,
                                  mut req: zbar_symbol_type_t)
 -> libc::c_char {
    if (*dcode).lock as u64 != 0 { return 1 as libc::c_int as libc::c_char }
    (*dcode).lock = req;
    return 0 as libc::c_int as libc::c_char;
}
#[inline]
unsafe extern "C" fn release_lock(mut dcode: *mut zbar_decoder_t,
                                  mut req: zbar_symbol_type_t)
 -> libc::c_char {
    if !((*dcode).lock as libc::c_uint == req as libc::c_uint) {
        fprintf(stderr,
                b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\tlock=%d req=%d\n\x00"
                    as *const u8 as *const libc::c_char,
                b"./zbar/decoder.h\x00" as *const u8 as *const libc::c_char,
                263 as libc::c_int,
                (*::std::mem::transmute::<&[u8; 13],
                                          &[libc::c_char; 13]>(b"release_lock\x00")).as_ptr(),
                b"dcode->lock == req\x00" as *const u8 as *const libc::c_char,
                (*dcode).lock as libc::c_uint, req as libc::c_uint);
        return 1 as libc::c_int as libc::c_char
    }
    (*dcode).lock = ZBAR_NONE;
    return 0 as libc::c_int as libc::c_char;
}
/*------------------------------------------------------------------------
 *  Copyright 2010 (c) Jeff Brown <spadix@users.sourceforge.net>
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
static mut code93_hash: [libc::c_schar; 64] =
    [0xf as libc::c_int as libc::c_schar,
     0x2b as libc::c_int as libc::c_schar,
     0x30 as libc::c_int as libc::c_schar,
     0x38 as libc::c_int as libc::c_schar,
     0x13 as libc::c_int as libc::c_schar,
     0x1b as libc::c_int as libc::c_schar,
     0x11 as libc::c_int as libc::c_schar,
     0x2a as libc::c_int as libc::c_schar,
     0xa as libc::c_int as libc::c_schar,
     -(1 as libc::c_int) as libc::c_schar,
     0x2f as libc::c_int as libc::c_schar,
     0xf as libc::c_int as libc::c_schar,
     0x38 as libc::c_int as libc::c_schar,
     0x38 as libc::c_int as libc::c_schar,
     0x2f as libc::c_int as libc::c_schar,
     0x37 as libc::c_int as libc::c_schar,
     0x24 as libc::c_int as libc::c_schar,
     0x3a as libc::c_int as libc::c_schar,
     0x1b as libc::c_int as libc::c_schar,
     0x36 as libc::c_int as libc::c_schar,
     0x18 as libc::c_int as libc::c_schar,
     0x26 as libc::c_int as libc::c_schar,
     0x2 as libc::c_int as libc::c_schar,
     0x2c as libc::c_int as libc::c_schar,
     0x2b as libc::c_int as libc::c_schar,
     0x5 as libc::c_int as libc::c_schar,
     0x21 as libc::c_int as libc::c_schar,
     0x3b as libc::c_int as libc::c_schar,
     0x4 as libc::c_int as libc::c_schar,
     0x15 as libc::c_int as libc::c_schar,
     0x12 as libc::c_int as libc::c_schar,
     0xc as libc::c_int as libc::c_schar, 0 as libc::c_int as libc::c_schar,
     0x26 as libc::c_int as libc::c_schar,
     0x23 as libc::c_int as libc::c_schar, 0 as libc::c_int as libc::c_schar,
     -(1 as libc::c_int) as libc::c_schar,
     0x2e as libc::c_int as libc::c_schar,
     0x3f as libc::c_int as libc::c_schar,
     0x13 as libc::c_int as libc::c_schar,
     0x2e as libc::c_int as libc::c_schar,
     0x36 as libc::c_int as libc::c_schar,
     -(1 as libc::c_int) as libc::c_schar,
     0x8 as libc::c_int as libc::c_schar, 0x9 as libc::c_int as libc::c_schar,
     -(1 as libc::c_int) as libc::c_schar,
     0x15 as libc::c_int as libc::c_schar,
     0x14 as libc::c_int as libc::c_schar,
     -(1 as libc::c_int) as libc::c_schar, 0 as libc::c_int as libc::c_schar,
     0x21 as libc::c_int as libc::c_schar,
     0x3b as libc::c_int as libc::c_schar,
     -(1 as libc::c_int) as libc::c_schar,
     0x33 as libc::c_int as libc::c_schar, 0 as libc::c_int as libc::c_schar,
     -(1 as libc::c_int) as libc::c_schar,
     0x2d as libc::c_int as libc::c_schar,
     0xc as libc::c_int as libc::c_schar,
     0x1b as libc::c_int as libc::c_schar,
     0xa as libc::c_int as libc::c_schar,
     0x3f as libc::c_int as libc::c_schar,
     0x3f as libc::c_int as libc::c_schar,
     0x29 as libc::c_int as libc::c_schar,
     0x1c as libc::c_int as libc::c_schar];
#[inline]
unsafe extern "C" fn check_width(mut cur: libc::c_uint,
                                 mut prev: libc::c_uint) -> libc::c_int {
    let mut dw: libc::c_uint = 0;
    if prev > cur {
        dw = prev.wrapping_sub(cur)
    } else { dw = cur.wrapping_sub(prev) }
    dw = dw.wrapping_mul(4 as libc::c_int as libc::c_uint);
    return (dw > prev) as libc::c_int;
}
#[inline]
unsafe extern "C" fn encode6(mut dcode: *mut zbar_decoder_t) -> libc::c_int {
    /* build edge signature of character */
    let mut s: libc::c_uint = (*dcode).s6;
    let mut sig: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    if s < 9 as libc::c_int as libc::c_uint { return -(1 as libc::c_int) }
    i = 6 as libc::c_int;
    loop  {
        i -= 1;
        if !(i > 0 as libc::c_int) { break ; }
        let mut c: libc::c_uint =
            decode_e(pair_width(dcode, i as libc::c_uchar), s,
                     9 as libc::c_int as libc::c_uint) as libc::c_uint;
        if c > 3 as libc::c_int as libc::c_uint { return -(1 as libc::c_int) }
        sig = ((sig << 2 as libc::c_int) as libc::c_uint | c) as libc::c_int
    }
    return sig;
}
#[inline]
unsafe extern "C" fn validate_sig(mut sig: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut sum: libc::c_int = 0 as libc::c_int;
    let mut emin: libc::c_int = 0 as libc::c_int;
    let mut sig0: libc::c_int = 0 as libc::c_int;
    let mut sig1: libc::c_int = 0 as libc::c_int;
    i = 3 as libc::c_int;
    loop  {
        i -= 1;
        if !(i >= 0 as libc::c_int) { break ; }
        let mut e: libc::c_int = sig & 3 as libc::c_int;
        sig >>= 2 as libc::c_int;
        sum = e - sum;
        sig1 <<= 4 as libc::c_int;
        sig1 += sum;
        if i == 0 { break ; }
        e = sig & 3 as libc::c_int;
        sig >>= 2 as libc::c_int;
        sum = e - sum;
        sig0 <<= 4 as libc::c_int;
        if emin > sum { emin = sum }
        sig0 += sum
    }
    emin = emin + (emin << 4 as libc::c_int) + (emin << 8 as libc::c_int);
    sig0 -= emin;
    sig1 += emin;
    return (sig0 | sig1) & 0x888 as libc::c_int;
}
#[inline]
unsafe extern "C" fn decode6(mut dcode: *mut zbar_decoder_t) -> libc::c_int {
    let mut sig: libc::c_int = encode6(dcode);
    let mut g0: libc::c_int = 0;
    let mut g1: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    if sig < 0 as libc::c_int ||
           (sig & 0x3 as libc::c_int) +
               (sig >> 4 as libc::c_int & 0x3 as libc::c_int) +
               (sig >> 8 as libc::c_int & 0x3 as libc::c_int) !=
               3 as libc::c_int || validate_sig(sig) != 0 {
        return -(1 as libc::c_int)
    }
    if (*dcode).code93.direction() != 0 {
        /* reverse signature */
        let mut tmp: libc::c_uint =
            (sig & 0x30 as libc::c_int) as libc::c_uint;
        sig =
            (sig & 0x3c0 as libc::c_int) >> 6 as libc::c_int |
                (sig & 0xf as libc::c_int) << 6 as libc::c_int;
        sig =
            (((sig & 0x30c as libc::c_int) >> 2 as libc::c_int |
                  (sig & 0xc3 as libc::c_int) << 2 as libc::c_int) as
                 libc::c_uint | tmp) as libc::c_int
    }
    g0 =
        code93_hash[(sig - (sig >> 4 as libc::c_int) & 0x3f as libc::c_int) as
                        usize] as libc::c_int;
    g1 =
        code93_hash[((sig >> 2 as libc::c_int) - (sig >> 7 as libc::c_int) &
                         0x3f as libc::c_int) as usize] as libc::c_int;
    if !(g0 >= 0 as libc::c_int && g1 >= 0 as libc::c_int) {
        fprintf(stderr,
                b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\tdir=%x sig=%03x g0=%03x g1=%03x %s\n\x00"
                    as *const u8 as *const libc::c_char,
                b"zbar/decoder/code93.c\x00" as *const u8 as
                    *const libc::c_char, 137 as libc::c_int,
                (*::std::mem::transmute::<&[u8; 8],
                                          &[libc::c_char; 8]>(b"decode6\x00")).as_ptr(),
                b"g0 >= 0 && g1 >= 0\x00" as *const u8 as *const libc::c_char,
                (*dcode).code93.direction() as libc::c_int, sig, g0, g1,
                _zbar_decoder_buf_dump((*dcode).buf,
                                       (*dcode).code93.character() as
                                           libc::c_uint));
        return -(1 as libc::c_int)
    }
    c = g0 + g1 & 0x3f as libc::c_int;
    return c;
}
#[inline]
unsafe extern "C" fn decode_start(mut dcode: *mut zbar_decoder_t)
 -> zbar_symbol_type_t {
    let mut dcode93: *mut code93_decoder_t = &mut (*dcode).code93;
    let mut dir: libc::c_uint = 0;
    let mut qz: libc::c_uint = 0;
    let mut s: libc::c_uint = (*dcode).s6;
    let mut c: libc::c_int = 0;
    c = encode6(dcode);
    if c < 0 as libc::c_int ||
           c != 0xf as libc::c_int && c != 0xf0 as libc::c_int {
        return ZBAR_NONE
    }
    dir = (c >> 7 as libc::c_int) as libc::c_uint;
    if dir != 0 {
        if decode_e(pair_width(dcode, 0 as libc::c_int as libc::c_uchar), s,
                    9 as libc::c_int as libc::c_uint) != 0 {
            return ZBAR_NONE
        }
        qz = get_width(dcode, 8 as libc::c_int as libc::c_uchar)
    }
    qz = get_width(dcode, 7 as libc::c_int as libc::c_uchar);
    if qz != 0 &&
           qz <
               s.wrapping_mul(3 as libc::c_int as
                                  libc::c_uint).wrapping_div(4 as libc::c_int
                                                                 as
                                                                 libc::c_uint)
       {
        return ZBAR_NONE
    }
    /* decoded valid start/stop - initialize state */
    (*dcode93).set_direction(dir);
    (*dcode93).set_element(if dir == 0 {
                               0 as libc::c_int
                           } else { 7 as libc::c_int } as libc::c_uint);
    (*dcode93).set_character(0 as libc::c_int);
    (*dcode93).width = s;
    return ZBAR_PARTIAL;
}
#[inline]
unsafe extern "C" fn decode_abort(mut dcode: *mut zbar_decoder_t,
                                  mut reason: *const libc::c_char)
 -> zbar_symbol_type_t {
    let mut dcode93: *mut code93_decoder_t = &mut (*dcode).code93;
    if (*dcode93).character() > 1 as libc::c_int {
        release_lock(dcode, ZBAR_CODE93);
    }
    (*dcode93).set_character(-(1 as libc::c_int));
    !reason.is_null();
    return ZBAR_NONE;
}
#[inline]
unsafe extern "C" fn check_stop(mut dcode: *mut zbar_decoder_t)
 -> zbar_symbol_type_t {
    let mut dcode93: *mut code93_decoder_t = &mut (*dcode).code93;
    let mut n: libc::c_uint = (*dcode93).character() as libc::c_uint;
    let mut s: libc::c_uint = (*dcode).s6;
    let mut max_len: libc::c_int =
        (*dcode93).configs[(ZBAR_CFG_MAX_LEN as libc::c_int -
                                ZBAR_CFG_MIN_LEN as libc::c_int) as usize];
    if n < 2 as libc::c_int as libc::c_uint ||
           n <
               (*dcode93).configs[(ZBAR_CFG_MIN_LEN as libc::c_int -
                                       ZBAR_CFG_MIN_LEN as libc::c_int) as
                                      usize] as libc::c_uint ||
           max_len != 0 && n > max_len as libc::c_uint {
        return decode_abort(dcode,
                            b"invalid len\x00" as *const u8 as
                                *const libc::c_char)
    }
    if (*dcode93).direction() != 0 {
        let mut qz: libc::c_uint =
            get_width(dcode, 0 as libc::c_int as libc::c_uchar);
        if qz != 0 &&
               qz <
                   s.wrapping_mul(3 as libc::c_int as
                                      libc::c_uint).wrapping_div(4 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
           {
            return decode_abort(dcode,
                                b"invalid qz\x00" as *const u8 as
                                    *const libc::c_char)
        }
    } else if decode_e(pair_width(dcode, 0 as libc::c_int as libc::c_uchar),
                       s, 9 as libc::c_int as libc::c_uint) != 0 {
        /* FIXME forward-trailing QZ check */
        return decode_abort(dcode,
                            b"invalid stop\x00" as *const u8 as
                                *const libc::c_char)
    }
    return ZBAR_CODE93;
}
#[inline]
unsafe extern "C" fn plusmod47(mut acc: libc::c_int, mut add: libc::c_int)
 -> libc::c_int {
    acc += add;
    if acc >= 47 as libc::c_int { acc -= 47 as libc::c_int }
    return acc;
}
#[inline]
unsafe extern "C" fn validate_checksums(mut dcode: *mut zbar_decoder_t)
 -> libc::c_int {
    let mut dcode93: *mut code93_decoder_t = &mut (*dcode).code93;
    let mut d: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    let mut n: libc::c_uint = (*dcode93).character() as libc::c_uint;
    let mut sum_c: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut acc_c: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i_c: libc::c_uint =
        n.wrapping_sub(2 as libc::c_int as
                           libc::c_uint).wrapping_rem(20 as libc::c_int as
                                                          libc::c_uint);
    let mut sum_k: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut acc_k: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i_k: libc::c_uint =
        n.wrapping_sub(1 as libc::c_int as
                           libc::c_uint).wrapping_rem(15 as libc::c_int as
                                                          libc::c_uint);
    i = 0 as libc::c_int as libc::c_uint;
    while i < n.wrapping_sub(2 as libc::c_int as libc::c_uint) {
        d =
            *(*dcode).buf.offset(if (*dcode93).direction() as libc::c_int != 0
                                    {
                                     n.wrapping_sub(1 as libc::c_int as
                                                        libc::c_uint).wrapping_sub(i)
                                 } else { i } as isize) as libc::c_uint;
        let fresh0 = i_c;
        i_c = i_c.wrapping_sub(1);
        if fresh0 == 0 {
            acc_c = 0 as libc::c_int as libc::c_uint;
            i_c = 19 as libc::c_int as libc::c_uint
        }
        acc_c =
            plusmod47(acc_c as libc::c_int, d as libc::c_int) as libc::c_uint;
        sum_c =
            plusmod47(sum_c as libc::c_int, acc_c as libc::c_int) as
                libc::c_uint;
        let fresh1 = i_k;
        i_k = i_k.wrapping_sub(1);
        if fresh1 == 0 {
            acc_k = 0 as libc::c_int as libc::c_uint;
            i_k = 14 as libc::c_int as libc::c_uint
        }
        acc_k =
            plusmod47(acc_k as libc::c_int, d as libc::c_int) as libc::c_uint;
        sum_k =
            plusmod47(sum_k as libc::c_int, acc_k as libc::c_int) as
                libc::c_uint;
        i = i.wrapping_add(1)
    }
    d =
        *(*dcode).buf.offset(if (*dcode93).direction() as libc::c_int != 0 {
                                 1 as libc::c_int as libc::c_uint
                             } else {
                                 n.wrapping_sub(2 as libc::c_int as
                                                    libc::c_uint)
                             } as isize) as libc::c_uint;
    if d != sum_c { return 1 as libc::c_int }
    acc_k =
        plusmod47(acc_k as libc::c_int, sum_c as libc::c_int) as libc::c_uint;
    sum_k =
        plusmod47(sum_k as libc::c_int, acc_k as libc::c_int) as libc::c_uint;
    d =
        *(*dcode).buf.offset(if (*dcode93).direction() as libc::c_int != 0 {
                                 0 as libc::c_int as libc::c_uint
                             } else {
                                 n.wrapping_sub(1 as libc::c_int as
                                                    libc::c_uint)
                             } as isize) as libc::c_uint;
    if d != sum_k { return 1 as libc::c_int }
    return 0 as libc::c_int;
}
/* resolve scan direction and convert to ASCII */
#[inline]
unsafe extern "C" fn postprocess(mut dcode: *mut zbar_decoder_t)
 -> libc::c_int {
    let mut dcode93: *mut code93_decoder_t = &mut (*dcode).code93;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut n: libc::c_uint = (*dcode93).character() as libc::c_uint;
    static mut code93_graph: [libc::c_uchar; 8] =
        [45, 46, 32, 36, 47, 43, 37, 0];
    static mut code93_s2: [libc::c_uchar; 27] =
        [27, 28, 29, 30, 31, 59, 60, 61, 62, 63, 91, 92, 93, 94, 95, 123, 124,
         125, 126, 127, 0, 64, 96, 127, 127, 127, 0];
    (*dcode).direction =
        1 as libc::c_int -
            2 as libc::c_int * (*dcode93).direction() as libc::c_int;
    if (*dcode93).direction() != 0 {
        /* reverse buffer */
        i = 0 as libc::c_int as libc::c_uint;
        while i < n.wrapping_div(2 as libc::c_int as libc::c_uint) {
            let mut j_0: libc::c_uint =
                n.wrapping_sub(1 as libc::c_int as
                                   libc::c_uint).wrapping_sub(i);
            let mut d: libc::c_uchar = *(*dcode).buf.offset(i as isize);
            *(*dcode).buf.offset(i as isize) =
                *(*dcode).buf.offset(j_0 as isize);
            *(*dcode).buf.offset(j_0 as isize) = d;
            i = i.wrapping_add(1)
        }
    }
    n = n.wrapping_sub(2 as libc::c_int as libc::c_uint);
    i = 0 as libc::c_int as libc::c_uint;
    j = 0 as libc::c_int as libc::c_uint;
    while i < n {
        let fresh2 = i;
        i = i.wrapping_add(1);
        let mut d_0: libc::c_uchar = *(*dcode).buf.offset(fresh2 as isize);
        if (d_0 as libc::c_int) < 0xa as libc::c_int {
            d_0 = ('0' as i32 + d_0 as libc::c_int) as libc::c_uchar
        } else if (d_0 as libc::c_int) < 0x24 as libc::c_int {
            d_0 =
                ('A' as i32 + d_0 as libc::c_int - 0xa as libc::c_int) as
                    libc::c_uchar
        } else if (d_0 as libc::c_int) < 0x2b as libc::c_int {
            d_0 =
                code93_graph[(d_0 as libc::c_int - 0x24 as libc::c_int) as
                                 usize]
        } else {
            let mut shift: libc::c_uint = d_0 as libc::c_uint;
            if !(shift < 0x2f as libc::c_int as libc::c_uint) {
                fprintf(stderr,
                        b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\t%s\n\x00"
                            as *const u8 as *const libc::c_char,
                        b"zbar/decoder/code93.c\x00" as *const u8 as
                            *const libc::c_char, 304 as libc::c_int,
                        (*::std::mem::transmute::<&[u8; 12],
                                                  &[libc::c_char; 12]>(b"postprocess\x00")).as_ptr(),
                        b"shift < 0x2f\x00" as *const u8 as
                            *const libc::c_char,
                        _zbar_decoder_buf_dump((*dcode).buf,
                                               (*dcode93).character() as
                                                   libc::c_uint));
                return -(1 as libc::c_int)
            }
            let fresh3 = i;
            i = i.wrapping_add(1);
            d_0 = *(*dcode).buf.offset(fresh3 as isize);
            if (d_0 as libc::c_int) < 0xa as libc::c_int ||
                   d_0 as libc::c_int >= 0x24 as libc::c_int {
                return 1 as libc::c_int
            }
            d_0 = (d_0 as libc::c_int - 0xa as libc::c_int) as libc::c_uchar;
            match shift {
                43 => { d_0 = d_0.wrapping_add(1) }
                44 => { d_0 = code93_s2[d_0 as usize] }
                45 => {
                    d_0 =
                        (d_0 as libc::c_int + 0x21 as libc::c_int) as
                            libc::c_uchar
                }
                46 => {
                    d_0 =
                        (d_0 as libc::c_int + 0x61 as libc::c_int) as
                            libc::c_uchar
                }
                _ => { return 1 as libc::c_int }
            }
        }
        let fresh4 = j;
        j = j.wrapping_add(1);
        *(*dcode).buf.offset(fresh4 as isize) = d_0
    }
    if !(j < (*dcode).buf_alloc) {
        fprintf(stderr,
                b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\tj=%02x %s\n\x00"
                    as *const u8 as *const libc::c_char,
                b"zbar/decoder/code93.c\x00" as *const u8 as
                    *const libc::c_char, 323 as libc::c_int,
                (*::std::mem::transmute::<&[u8; 12],
                                          &[libc::c_char; 12]>(b"postprocess\x00")).as_ptr(),
                b"j < dcode->buf_alloc\x00" as *const u8 as
                    *const libc::c_char, j,
                _zbar_decoder_buf_dump((*dcode).buf,
                                       (*dcode).code93.character() as
                                           libc::c_uint));
        return 1 as libc::c_int
    }
    (*dcode).buflen = j;
    *(*dcode).buf.offset(j as isize) = '\u{0}' as i32 as libc::c_uchar;
    (*dcode).modifiers = 0 as libc::c_int as libc::c_uint;
    return 0 as libc::c_int;
}
/* reset Code 93 specific state */
/* decode Code 93 symbols */
#[no_mangle]
pub unsafe extern "C" fn _zbar_decode_code93(mut dcode: *mut zbar_decoder_t)
 -> zbar_symbol_type_t {
    let mut dcode93: *mut code93_decoder_t = &mut (*dcode).code93;
    let mut c: libc::c_int = 0;
    if (*dcode93).character() < 0 as libc::c_int {
        let mut sym: zbar_symbol_type_t = ZBAR_NONE;
        if get_color(dcode) as libc::c_int != ZBAR_BAR as libc::c_int {
            return ZBAR_NONE
        }
        sym = decode_start(dcode);
        return sym
    }
    (*dcode93).set_element((*dcode93).element() + 1);
    if (*dcode93).element() as libc::c_int != 6 as libc::c_int ||
           get_color(dcode) as libc::c_int ==
               (*dcode93).direction() as libc::c_int {
        return ZBAR_NONE
    }
    (*dcode93).set_element(0 as libc::c_int as libc::c_uint);
    if check_width((*dcode).s6, (*dcode93).width) != 0 {
        return decode_abort(dcode,
                            b"width var\x00" as *const u8 as
                                *const libc::c_char)
    }
    c = decode6(dcode);
    if c < 0 as libc::c_int {
        return decode_abort(dcode,
                            b"aborted\x00" as *const u8 as
                                *const libc::c_char)
    }
    if c == 0x2f as libc::c_int {
        if check_stop(dcode) as u64 == 0 { return ZBAR_NONE }
        if validate_checksums(dcode) != 0 {
            return decode_abort(dcode,
                                b"checksum error\x00" as *const u8 as
                                    *const libc::c_char)
        }
        if postprocess(dcode) != 0 {
            return decode_abort(dcode,
                                b"invalid encoding\x00" as *const u8 as
                                    *const libc::c_char)
        }
        (*dcode93).set_character(-(1 as libc::c_int));
        return ZBAR_CODE93
    }
    if size_buf(dcode,
                ((*dcode93).character() + 1 as libc::c_int) as libc::c_uint)
           != 0 {
        return decode_abort(dcode,
                            b"overflow\x00" as *const u8 as
                                *const libc::c_char)
    }
    (*dcode93).width = (*dcode).s6;
    if (*dcode93).character() == 1 as libc::c_int {
        /* lock shared resources */
        if acquire_lock(dcode, ZBAR_CODE93) != 0 {
            return decode_abort(dcode, 0 as *const libc::c_char)
        }
        *(*dcode).buf.offset(0 as libc::c_int as isize) = (*dcode93).buf
    }
    if (*dcode93).character() == 0 {
        (*dcode93).buf = c as libc::c_uchar
    } else {
        *(*dcode).buf.offset((*dcode93).character() as isize) =
            c as libc::c_uchar
    }
    (*dcode93).set_character((*dcode93).character() + 1);
    return ZBAR_NONE;
}
