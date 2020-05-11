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
pub type size_t = libc::c_ulong;
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
pub type char39_t = char39_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct char39_s {
    pub chk: libc::c_uchar,
    pub rev: libc::c_uchar,
    pub fwd: libc::c_uchar,
}
#[inline]
unsafe extern "C" fn get_color(mut dcode: *const zbar_decoder_t)
 -> libc::c_char {
    return ((*dcode).idx as libc::c_int & 1 as libc::c_int) as libc::c_char;
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
/*------------------------------------------------------------------------
 *  Copyright 2008-2010 (c) Jeff Brown <spadix@users.sourceforge.net>
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
/* memmove */
static mut code39_hi: [libc::c_uchar; 32] =
    [(0x80 as libc::c_int | 0 as libc::c_int) as libc::c_uchar,
     (0x40 as libc::c_int | 0x2 as libc::c_int) as libc::c_uchar,
     (0x80 as libc::c_int | 0x6 as libc::c_int) as libc::c_uchar,
     (0xc0 as libc::c_int | 0x8 as libc::c_int) as libc::c_uchar,
     (0x40 as libc::c_int | 0xa as libc::c_int) as libc::c_uchar,
     (0x80 as libc::c_int | 0xe as libc::c_int) as libc::c_uchar,
     (0xc0 as libc::c_int | 0x10 as libc::c_int) as libc::c_uchar,
     (0 as libc::c_int | 0x12 as libc::c_int) as libc::c_uchar,
     (0x80 as libc::c_int | 0x13 as libc::c_int) as libc::c_uchar,
     (0xc0 as libc::c_int | 0x15 as libc::c_int) as libc::c_uchar,
     (0x80 as libc::c_int | 0x17 as libc::c_int) as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     (0xc0 as libc::c_int | 0x19 as libc::c_int) as libc::c_uchar,
     (0 as libc::c_int | 0x1b as libc::c_int) as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     (0x40 as libc::c_int | 0x1c as libc::c_int) as libc::c_uchar,
     (0x80 as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar,
     (0xc0 as libc::c_int | 0x22 as libc::c_int) as libc::c_uchar,
     (0 as libc::c_int | 0x24 as libc::c_int) as libc::c_uchar,
     (0x80 as libc::c_int | 0x25 as libc::c_int) as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     (0 as libc::c_int | 0x27 as libc::c_int) as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     (0xc0 as libc::c_int | 0x28 as libc::c_int) as libc::c_uchar,
     (0 as libc::c_int | 0x2a as libc::c_int) as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     (0 as libc::c_int | 0x2b as libc::c_int) as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar];
static mut code39_encodings: [char39_t; 44] =
    [{
         let mut init =
             char39_s{chk: 0x7 as libc::c_int as libc::c_uchar,
                      rev: 0x1a as libc::c_int as libc::c_uchar,
                      fwd: 0x20 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0xd as libc::c_int as libc::c_uchar,
                      rev: 0x10 as libc::c_int as libc::c_uchar,
                      fwd: 0x3 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0x13 as libc::c_int as libc::c_uchar,
                      rev: 0x17 as libc::c_int as libc::c_uchar,
                      fwd: 0x22 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0x16 as libc::c_int as libc::c_uchar,
                      rev: 0x1d as libc::c_int as libc::c_uchar,
                      fwd: 0x23 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0x19 as libc::c_int as libc::c_uchar,
                      rev: 0xd as libc::c_int as libc::c_uchar,
                      fwd: 0x5 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0x1c as libc::c_int as libc::c_uchar,
                      rev: 0x13 as libc::c_int as libc::c_uchar,
                      fwd: 0x6 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0x25 as libc::c_int as libc::c_uchar,
                      rev: 0x7 as libc::c_int as libc::c_uchar,
                      fwd: 0xc as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0x2a as libc::c_int as libc::c_uchar,
                      rev: 0x2a as libc::c_int as libc::c_uchar,
                      fwd: 0x27 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0x31 as libc::c_int as libc::c_uchar,
                      rev: 0x4 as libc::c_int as libc::c_uchar,
                      fwd: 0xe as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0x34 as libc::c_int as libc::c_uchar,
                      rev: 0 as libc::c_int as libc::c_uchar,
                      fwd: 0xf as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0x43 as libc::c_int as libc::c_uchar,
                      rev: 0x15 as libc::c_int as libc::c_uchar,
                      fwd: 0x25 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0x46 as libc::c_int as libc::c_uchar,
                      rev: 0x1c as libc::c_int as libc::c_uchar,
                      fwd: 0x26 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0x49 as libc::c_int as libc::c_uchar,
                      rev: 0xb as libc::c_int as libc::c_uchar,
                      fwd: 0x8 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0x4c as libc::c_int as libc::c_uchar,
                      rev: 0x12 as libc::c_int as libc::c_uchar,
                      fwd: 0x9 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0x52 as libc::c_int as libc::c_uchar,
                      rev: 0x19 as libc::c_int as libc::c_uchar,
                      fwd: 0x2b as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0x58 as libc::c_int as libc::c_uchar,
                      rev: 0xf as libc::c_int as libc::c_uchar,
                      fwd: 0 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0x61 as libc::c_int as libc::c_uchar,
                      rev: 0x2 as libc::c_int as libc::c_uchar,
                      fwd: 0x11 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0x64 as libc::c_int as libc::c_uchar,
                      rev: 0x9 as libc::c_int as libc::c_uchar,
                      fwd: 0x12 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0x70 as libc::c_int as libc::c_uchar,
                      rev: 0x6 as libc::c_int as libc::c_uchar,
                      fwd: 0x13 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0x85 as libc::c_int as libc::c_uchar,
                      rev: 0x24 as libc::c_int as libc::c_uchar,
                      fwd: 0x16 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0x8a as libc::c_int as libc::c_uchar,
                      rev: 0x29 as libc::c_int as libc::c_uchar,
                      fwd: 0x28 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0x91 as libc::c_int as libc::c_uchar,
                      rev: 0x21 as libc::c_int as libc::c_uchar,
                      fwd: 0x18 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0x94 as libc::c_int as libc::c_uchar,
                      rev: 0x2b as libc::c_int as libc::c_uchar,
                      fwd: 0x19 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0xa2 as libc::c_int as libc::c_uchar,
                      rev: 0x28 as libc::c_int as libc::c_uchar,
                      fwd: 0x29 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0xa8 as libc::c_int as libc::c_uchar,
                      rev: 0x27 as libc::c_int as libc::c_uchar,
                      fwd: 0x2a as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0xc1 as libc::c_int as libc::c_uchar,
                      rev: 0x1f as libc::c_int as libc::c_uchar,
                      fwd: 0x1b as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0xc4 as libc::c_int as libc::c_uchar,
                      rev: 0x26 as libc::c_int as libc::c_uchar,
                      fwd: 0x1c as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0xd0 as libc::c_int as libc::c_uchar,
                      rev: 0x23 as libc::c_int as libc::c_uchar,
                      fwd: 0x1d as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0x3 as libc::c_int as libc::c_uchar,
                      rev: 0x14 as libc::c_int as libc::c_uchar,
                      fwd: 0x1e as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0x6 as libc::c_int as libc::c_uchar,
                      rev: 0x1b as libc::c_int as libc::c_uchar,
                      fwd: 0x1f as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0x9 as libc::c_int as libc::c_uchar,
                      rev: 0xa as libc::c_int as libc::c_uchar,
                      fwd: 0x1 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0xc as libc::c_int as libc::c_uchar,
                      rev: 0x11 as libc::c_int as libc::c_uchar,
                      fwd: 0x2 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0x12 as libc::c_int as libc::c_uchar,
                      rev: 0x18 as libc::c_int as libc::c_uchar,
                      fwd: 0x21 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0x18 as libc::c_int as libc::c_uchar,
                      rev: 0xe as libc::c_int as libc::c_uchar,
                      fwd: 0x4 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0x21 as libc::c_int as libc::c_uchar,
                      rev: 0x1 as libc::c_int as libc::c_uchar,
                      fwd: 0xa as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0x24 as libc::c_int as libc::c_uchar,
                      rev: 0x8 as libc::c_int as libc::c_uchar,
                      fwd: 0xb as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0x30 as libc::c_int as libc::c_uchar,
                      rev: 0x5 as libc::c_int as libc::c_uchar,
                      fwd: 0xd as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0x42 as libc::c_int as libc::c_uchar,
                      rev: 0x16 as libc::c_int as libc::c_uchar,
                      fwd: 0x24 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0x48 as libc::c_int as libc::c_uchar,
                      rev: 0xc as libc::c_int as libc::c_uchar,
                      fwd: 0x7 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0x60 as libc::c_int as libc::c_uchar,
                      rev: 0x3 as libc::c_int as libc::c_uchar,
                      fwd: 0x10 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0x81 as libc::c_int as libc::c_uchar,
                      rev: 0x1e as libc::c_int as libc::c_uchar,
                      fwd: 0x14 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0x84 as libc::c_int as libc::c_uchar,
                      rev: 0x25 as libc::c_int as libc::c_uchar,
                      fwd: 0x15 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0x90 as libc::c_int as libc::c_uchar,
                      rev: 0x22 as libc::c_int as libc::c_uchar,
                      fwd: 0x17 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             char39_s{chk: 0xc0 as libc::c_int as libc::c_uchar,
                      rev: 0x20 as libc::c_int as libc::c_uchar,
                      fwd: 0x1a as libc::c_int as libc::c_uchar,};
         init
     }];
static mut code39_characters: [libc::c_uchar; 44] =
    [48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 65, 66, 67, 68, 69, 70, 71, 72,
     73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90,
     45, 46, 32, 36, 47, 43, 37, 42];
#[inline]
unsafe extern "C" fn code39_decode1(mut enc: libc::c_uchar,
                                    mut e: libc::c_uint, mut s: libc::c_uint)
 -> libc::c_uchar {
    let mut E: libc::c_uchar =
        decode_e(e, s, 72 as libc::c_int as libc::c_uint) as libc::c_uchar;
    if E as libc::c_int > 18 as libc::c_int {
        return 0xff as libc::c_int as libc::c_uchar
    }
    enc = ((enc as libc::c_int) << 1 as libc::c_int) as libc::c_uchar;
    if E as libc::c_int > 6 as libc::c_int {
        enc = (enc as libc::c_int | 1 as libc::c_int) as libc::c_uchar
    }
    return enc;
}
#[inline]
unsafe extern "C" fn code39_decode9(mut dcode: *mut zbar_decoder_t)
 -> libc::c_schar {
    let mut dcode39: *mut code39_decoder_t = &mut (*dcode).code39;
    if (*dcode39).s9 < 9 as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int) as libc::c_schar
    }
    /* threshold bar width ratios */
    let mut i: libc::c_uchar = 0;
    let mut enc: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    i = 0 as libc::c_int as libc::c_uchar;
    while (i as libc::c_int) < 5 as libc::c_int {
        enc = code39_decode1(enc, get_width(dcode, i), (*dcode39).s9);
        if enc as libc::c_int == 0xff as libc::c_int {
            return -(1 as libc::c_int) as libc::c_schar
        }
        i = i.wrapping_add(1)
    }
    if !((enc as libc::c_int) < 0x20 as libc::c_int) {
        fprintf(stderr,
                b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\t enc=%x s9=%x\n\x00"
                    as *const u8 as *const libc::c_char,
                b"zbar/decoder/code39.c\x00" as *const u8 as
                    *const libc::c_char, 160 as libc::c_int,
                (*::std::mem::transmute::<&[u8; 15],
                                          &[libc::c_char; 15]>(b"code39_decode9\x00")).as_ptr(),
                b"enc < 0x20\x00" as *const u8 as *const libc::c_char,
                enc as libc::c_int, (*dcode39).s9);
        return -(1 as libc::c_int) as libc::c_schar
    }
    /* lookup first 5 encoded widths for coarse decode */
    let mut idx: libc::c_uchar = code39_hi[enc as usize];
    if idx as libc::c_int == 0xff as libc::c_int {
        return -(1 as libc::c_int) as libc::c_schar
    }
    /* encode remaining widths (NB first encoded width is lost) */
    while (i as libc::c_int) < 9 as libc::c_int {
        enc = code39_decode1(enc, get_width(dcode, i), (*dcode39).s9);
        if enc as libc::c_int == 0xff as libc::c_int {
            return -(1 as libc::c_int) as libc::c_schar
        }
        i = i.wrapping_add(1)
    }
    if idx as libc::c_int & 0xc0 as libc::c_int == 0x80 as libc::c_int {
        idx =
            ((idx as libc::c_int & 0x3f as libc::c_int) +
                 (enc as libc::c_int >> 3 as libc::c_int & 1 as libc::c_int))
                as libc::c_uchar
    } else if idx as libc::c_int & 0xc0 as libc::c_int == 0xc0 as libc::c_int
     {
        idx =
            ((idx as libc::c_int & 0x3f as libc::c_int) +
                 (enc as libc::c_int >> 2 as libc::c_int & 1 as libc::c_int))
                as libc::c_uchar
    } else if idx as libc::c_int & 0xc0 as libc::c_int != 0 {
        idx =
            ((idx as libc::c_int & 0x3f as libc::c_int) +
                 (enc as libc::c_int >> 2 as libc::c_int & 3 as libc::c_int))
                as libc::c_uchar
    }
    if !((idx as libc::c_int) < 0x2c as libc::c_int) {
        fprintf(stderr,
                b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\t idx=%x enc=%x s9=%x\n\x00"
                    as *const u8 as *const libc::c_char,
                b"zbar/decoder/code39.c\x00" as *const u8 as
                    *const libc::c_char, 180 as libc::c_int,
                (*::std::mem::transmute::<&[u8; 15],
                                          &[libc::c_char; 15]>(b"code39_decode9\x00")).as_ptr(),
                b"idx < 0x2c\x00" as *const u8 as *const libc::c_char,
                idx as libc::c_int, enc as libc::c_int, (*dcode39).s9);
        return -(1 as libc::c_int) as libc::c_schar
    }
    let mut c: *const char39_t =
        &*code39_encodings.as_ptr().offset(idx as isize) as *const char39_t;
    if enc as libc::c_int != (*c).chk as libc::c_int {
        return -(1 as libc::c_int) as libc::c_schar
    }
    (*dcode39).width = (*dcode39).s9;
    return if (*dcode39).direction() as libc::c_int != 0 {
               (*c).rev as libc::c_int
           } else { (*c).fwd as libc::c_int } as libc::c_schar;
}
#[inline]
unsafe extern "C" fn code39_decode_start(mut dcode: *mut zbar_decoder_t)
 -> libc::c_schar {
    let mut dcode39: *mut code39_decoder_t = &mut (*dcode).code39;
    let mut c: libc::c_schar = code39_decode9(dcode);
    if c as libc::c_int != 0x19 as libc::c_int &&
           c as libc::c_int != 0x2b as libc::c_int {
        return ZBAR_NONE as libc::c_int as libc::c_schar
    }
    (*dcode39).set_direction((*dcode39).direction() ^
                                 (c as libc::c_int == 0x19 as libc::c_int) as
                                     libc::c_int as libc::c_uint);
    /* check leading quiet zone - spec is 10x */
    let mut quiet: libc::c_uint =
        get_width(dcode, 9 as libc::c_int as libc::c_uchar);
    if quiet != 0 &&
           quiet <
               (*dcode39).s9.wrapping_div(2 as libc::c_int as libc::c_uint) {
        return ZBAR_NONE as libc::c_int as libc::c_schar
    }
    (*dcode39).set_element(9 as libc::c_int as libc::c_uint);
    (*dcode39).set_character(0 as libc::c_int);
    return ZBAR_PARTIAL as libc::c_int as libc::c_schar;
}
#[inline]
unsafe extern "C" fn code39_postprocess(mut dcode: *mut zbar_decoder_t)
 -> libc::c_int {
    let mut dcode39: *mut code39_decoder_t = &mut (*dcode).code39;
    (*dcode).direction =
        1 as libc::c_int -
            2 as libc::c_int * (*dcode39).direction() as libc::c_int;
    let mut i: libc::c_int = 0;
    if (*dcode39).direction() != 0 {
        /* reverse buffer */
        i = 0 as libc::c_int;
        while i < (*dcode39).character() / 2 as libc::c_int {
            let mut j: libc::c_uint =
                ((*dcode39).character() - 1 as libc::c_int - i) as
                    libc::c_uint;
            let mut code: libc::c_char =
                *(*dcode).buf.offset(i as isize) as libc::c_char;
            *(*dcode).buf.offset(i as isize) =
                *(*dcode).buf.offset(j as isize);
            *(*dcode).buf.offset(j as isize) = code as libc::c_uchar;
            i += 1
        }
    }
    i = 0 as libc::c_int;
    while i < (*dcode39).character() {
        *(*dcode).buf.offset(i as isize) =
            if (*(*dcode).buf.offset(i as isize) as libc::c_int) <
                   0x2b as libc::c_int {
                code39_characters[*(*dcode).buf.offset(i as isize) as
                                      libc::c_uint as usize] as libc::c_int
            } else { '?' as i32 } as libc::c_uchar;
        i += 1
    }
    if !((i as libc::c_uint) < (*dcode).buf_alloc) {
        fprintf(stderr,
                b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\ti=%02x %s\n\x00"
                    as *const u8 as *const libc::c_char,
                b"zbar/decoder/code39.c\x00" as *const u8 as
                    *const libc::c_char, 236 as libc::c_int,
                (*::std::mem::transmute::<&[u8; 19],
                                          &[libc::c_char; 19]>(b"code39_postprocess\x00")).as_ptr(),
                b"i < dcode->buf_alloc\x00" as *const u8 as
                    *const libc::c_char, i,
                _zbar_decoder_buf_dump((*dcode).buf,
                                       (*dcode39).character() as
                                           libc::c_uint));
        return -(1 as libc::c_int)
    }
    (*dcode).buflen = i as libc::c_uint;
    *(*dcode).buf.offset(i as isize) = '\u{0}' as i32 as libc::c_uchar;
    (*dcode).modifiers = 0 as libc::c_int as libc::c_uint;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn check_width(mut ref_0: libc::c_uint, mut w: libc::c_uint)
 -> libc::c_int {
    let mut dref: libc::c_uint = ref_0;
    ref_0 = ref_0.wrapping_mul(4 as libc::c_int as libc::c_uint);
    w = w.wrapping_mul(4 as libc::c_int as libc::c_uint);
    return (ref_0.wrapping_sub(dref) <= w && w <= ref_0.wrapping_add(dref)) as
               libc::c_int;
}
/* reset Code 39 specific state */
/* decode Code 39 symbols */
#[no_mangle]
pub unsafe extern "C" fn _zbar_decode_code39(mut dcode: *mut zbar_decoder_t)
 -> zbar_symbol_type_t {
    let mut dcode39: *mut code39_decoder_t = &mut (*dcode).code39;
    /* update latest character width */
    (*dcode39).s9 =
        (*dcode39).s9.wrapping_sub(get_width(dcode,
                                             9 as libc::c_int as
                                                 libc::c_uchar));
    (*dcode39).s9 =
        (*dcode39).s9.wrapping_add(get_width(dcode,
                                             0 as libc::c_int as
                                                 libc::c_uchar));
    if (*dcode39).character() < 0 as libc::c_int {
        if get_color(dcode) as libc::c_int != ZBAR_BAR as libc::c_int {
            return ZBAR_NONE
        }
        return code39_decode_start(dcode) as zbar_symbol_type_t
    }
    (*dcode39).set_element((*dcode39).element() + 1);
    if ((*dcode39).element() as libc::c_int) < 9 as libc::c_int {
        return ZBAR_NONE
    }
    if (*dcode39).element() as libc::c_int == 10 as libc::c_int {
        let mut space: libc::c_uint =
            get_width(dcode, 0 as libc::c_int as libc::c_uchar);
        if (*dcode39).character() != 0 &&
               *(*dcode).buf.offset(((*dcode39).character() -
                                         1 as libc::c_int) as isize) as
                   libc::c_int == 0x2b as libc::c_int {
            /* STOP */
            /* trim STOP character */
            (*dcode39).set_character((*dcode39).character() - 1);
            let mut sym: zbar_symbol_type_t = ZBAR_NONE;
            /* trailing quiet zone check */
            if !(space != 0 &&
                     space <
                         (*dcode39).width.wrapping_div(2 as libc::c_int as
                                                           libc::c_uint)) {
                if !((*dcode39).character() <
                         (*dcode39).configs[(ZBAR_CFG_MIN_LEN as libc::c_int -
                                                 ZBAR_CFG_MIN_LEN as
                                                     libc::c_int) as usize] ||
                         (*dcode39).configs[(ZBAR_CFG_MAX_LEN as libc::c_int -
                                                 ZBAR_CFG_MIN_LEN as
                                                     libc::c_int) as usize] >
                             0 as libc::c_int &&
                             (*dcode39).character() >
                                 (*dcode39).configs[(ZBAR_CFG_MAX_LEN as
                                                         libc::c_int -
                                                         ZBAR_CFG_MIN_LEN as
                                                             libc::c_int) as
                                                        usize]) {
                    if code39_postprocess(dcode) == 0 {
                        /* FIXME checksum */
                        sym = ZBAR_CODE39
                    }
                }
            }
            (*dcode39).set_character(-(1 as libc::c_int));
            if sym as u64 == 0 { release_lock(dcode, ZBAR_CODE39); }
            return sym
        }
        if space >
               (*dcode39).width.wrapping_div(2 as libc::c_int as libc::c_uint)
           {
            /* inter-character space check failure */
            if (*dcode39).character() != 0 {
                release_lock(dcode, ZBAR_CODE39);
            }
            (*dcode39).set_character(-(1 as libc::c_int))
        }
        (*dcode39).set_element(0 as libc::c_int as libc::c_uint);
        return ZBAR_NONE
    }
    if check_width((*dcode39).width, (*dcode39).s9) == 0 {
        if (*dcode39).character() != 0 { release_lock(dcode, ZBAR_CODE39); }
        (*dcode39).set_character(-(1 as libc::c_int));
        return ZBAR_NONE
    }
    let mut c: libc::c_schar = code39_decode9(dcode);
    /* lock shared resources */
    if (*dcode39).character() == 0 &&
           acquire_lock(dcode, ZBAR_CODE39) as libc::c_int != 0 {
        (*dcode39).set_character(-(1 as libc::c_int));
        return ZBAR_PARTIAL
    }
    if (c as libc::c_int) < 0 as libc::c_int ||
           size_buf(dcode,
                    ((*dcode39).character() + 1 as libc::c_int) as
                        libc::c_uint) as libc::c_int != 0 {
        release_lock(dcode, ZBAR_CODE39);
        (*dcode39).set_character(-(1 as libc::c_int));
        return ZBAR_NONE
    } else {
        if !((c as libc::c_int) < 0x2c as libc::c_int) {
            fprintf(stderr,
                    b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\tc=%02x s9=%x\n\x00"
                        as *const u8 as *const libc::c_char,
                    b"zbar/decoder/code39.c\x00" as *const u8 as
                        *const libc::c_char, 337 as libc::c_int,
                    (*::std::mem::transmute::<&[u8; 20],
                                              &[libc::c_char; 20]>(b"_zbar_decode_code39\x00")).as_ptr(),
                    b"c < 0x2c\x00" as *const u8 as *const libc::c_char,
                    c as libc::c_int, (*dcode39).s9);
            return ZBAR_NONE
        }
    }
    let ref mut fresh0 = (*dcode39).character();
    let fresh1 = *fresh0;
    *fresh0 = *fresh0 + 1;
    *(*dcode).buf.offset(fresh1 as isize) = c as libc::c_uchar;
    return ZBAR_NONE;
}
