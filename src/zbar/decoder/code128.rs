use ::c2rust_bitfields;
use ::libc;
extern {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
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
pub type zbar_modifier_e = libc::c_uint;
pub const ZBAR_MOD_NUM: zbar_modifier_e = 2;
pub const ZBAR_MOD_AIM: zbar_modifier_e = 1;
pub const ZBAR_MOD_GS1: zbar_modifier_e = 0;
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
 *------------------------------------------------------------------------ */
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
 *------------------------------------------------------------------------ */
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
 *------------------------------------------------------------------------ */
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
 *------------------------------------------------------------------------ */
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
 *------------------------------------------------------------------------ */
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
 *------------------------------------------------------------------------ */
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
 *------------------------------------------------------------------------ */
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
 *------------------------------------------------------------------------ */
/* state of each parallel decode attempt */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ean_pass_s {
    pub state: libc::c_schar,
    pub width: libc::c_uint,
    pub raw: [libc::c_uchar; 7],
}
pub type zbar_decoder_handler_t = unsafe extern fn(_: *mut zbar_decoder_t) -> ();
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
pub const CODE_A: code128_char_e = 101;
pub const CODE_C: code128_char_e = 99;
pub const START_A: code128_char_e = 103;
pub const FNC1: code128_char_e = 102;
pub const FNC3: code128_char_e = 96;
pub const FNC2: code128_char_e = 97;
pub const SHIFT: code128_char_e = 98;
pub const START_C: code128_char_e = 105;
pub const STOP_FWD: code128_char_e = 106;
pub const STOP_REV: code128_char_e = 107;
pub type code128_char_e = libc::c_uint;
pub const FNC4: code128_char_e = 108;
pub const START_B: code128_char_e = 104;
pub const CODE_B: code128_char_e = 100;
#[inline]
unsafe extern fn decode_e(
    mut e: libc::c_uint,
    mut s: libc::c_uint,
    mut n: libc::c_uint,
) -> libc::c_int {
    let mut E: libc::c_uchar =
        e.wrapping_mul(n)
            .wrapping_mul(2 as libc::c_int as libc::c_uint)
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            .wrapping_div(s)
            .wrapping_sub(3 as libc::c_int as libc::c_uint)
            .wrapping_div(2 as libc::c_int as libc::c_uint) as libc::c_uchar;
    return if E as libc::c_uint >= n.wrapping_sub(3 as libc::c_int as libc::c_uint) {
        -(1 as libc::c_int)
    } else {
        E as libc::c_int
    };
}
#[inline]
unsafe extern fn get_color(mut dcode: *const zbar_decoder_t) -> libc::c_char {
    return ((*dcode).idx as libc::c_int & 1 as libc::c_int) as libc::c_char;
}
#[inline]
unsafe extern fn get_width(
    mut dcode: *const zbar_decoder_t,
    mut offset: libc::c_uchar,
) -> libc::c_uint {
    return (*dcode).w[((*dcode).idx as libc::c_int - offset as libc::c_int
        & 16 as libc::c_int - 1 as libc::c_int) as usize];
}
/* acquire shared state lock */
#[inline]
unsafe extern fn acquire_lock(
    mut dcode: *mut zbar_decoder_t,
    mut req: zbar_symbol_type_t,
) -> libc::c_char {
    if (*dcode).lock as u64 != 0 {
        return 1 as libc::c_int as libc::c_char;
    }
    (*dcode).lock = req;
    return 0 as libc::c_int as libc::c_char;
}
/* check and release shared state lock */
/* ensure output buffer has sufficient allocation for request */
#[inline]
unsafe extern fn size_buf(mut dcode: *mut zbar_decoder_t, mut len: libc::c_uint) -> libc::c_char {
    let mut buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if len <= 0x20 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as libc::c_char;
    }
    if len < (*dcode).buf_alloc {
        /* FIXME size reduction heuristic? */
        return 0 as libc::c_int as libc::c_char;
    }
    if len > 0x100 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int as libc::c_char;
    }
    if len < (*dcode).buf_alloc.wrapping_add(0x10 as libc::c_int as libc::c_uint) {
        len = (*dcode).buf_alloc.wrapping_add(0x10 as libc::c_int as libc::c_uint);
        if len > 0x100 as libc::c_int as libc::c_uint {
            len = 0x100 as libc::c_int as libc::c_uint
        }
    }
    buf = realloc((*dcode).buf as *mut libc::c_void, len as libc::c_ulong) as *mut libc::c_uchar;
    if buf.is_null() {
        return 1 as libc::c_int as libc::c_char;
    }
    (*dcode).buf = buf;
    (*dcode).buf_alloc = len;
    return 0 as libc::c_int as libc::c_char;
}
#[inline]
unsafe extern fn release_lock(
    mut dcode: *mut zbar_decoder_t,
    mut req: zbar_symbol_type_t,
) -> libc::c_char {
    if !((*dcode).lock as libc::c_uint == req as libc::c_uint) {
        fprintf(
            stderr,
            b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\tlock=%d req=%d\n\x00" as *const u8
                as *const libc::c_char,
            b"./zbar/decoder.h\x00" as *const u8 as *const libc::c_char,
            263 as libc::c_int,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"release_lock\x00"))
                .as_ptr(),
            b"dcode->lock == req\x00" as *const u8 as *const libc::c_char,
            (*dcode).lock as libc::c_uint,
            req as libc::c_uint,
        );
        return 1 as libc::c_int as libc::c_char;
    }
    (*dcode).lock = ZBAR_NONE;
    return 0 as libc::c_int as libc::c_char;
}
static mut characters: [libc::c_uchar; 108] = [
    0x5c as libc::c_int as libc::c_uchar,
    0xbf as libc::c_int as libc::c_uchar,
    0xa1 as libc::c_int as libc::c_uchar,
    0x2a as libc::c_int as libc::c_uchar,
    0xc5 as libc::c_int as libc::c_uchar,
    0xc as libc::c_int as libc::c_uchar,
    0xa4 as libc::c_int as libc::c_uchar,
    0x2d as libc::c_int as libc::c_uchar,
    0xe3 as libc::c_int as libc::c_uchar,
    0xf as libc::c_int as libc::c_uchar,
    0x5f as libc::c_int as libc::c_uchar,
    0xe4 as libc::c_int as libc::c_uchar,
    0x6b as libc::c_int as libc::c_uchar,
    0xe8 as libc::c_int as libc::c_uchar,
    0x69 as libc::c_int as libc::c_uchar,
    0xa7 as libc::c_int as libc::c_uchar,
    0xe7 as libc::c_int as libc::c_uchar,
    0xc1 as libc::c_int as libc::c_uchar,
    0x51 as libc::c_int as libc::c_uchar,
    0x1e as libc::c_int as libc::c_uchar,
    0x83 as libc::c_int as libc::c_uchar,
    0xd9 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x84 as libc::c_int as libc::c_uchar,
    0x1f as libc::c_int as libc::c_uchar,
    0xc7 as libc::c_int as libc::c_uchar,
    0xd as libc::c_int as libc::c_uchar,
    0x33 as libc::c_int as libc::c_uchar,
    0x86 as libc::c_int as libc::c_uchar,
    0xb5 as libc::c_int as libc::c_uchar,
    0xe as libc::c_int as libc::c_uchar,
    0x15 as libc::c_int as libc::c_uchar,
    0x87 as libc::c_int as libc::c_uchar,
    0x10 as libc::c_int as libc::c_uchar,
    0xda as libc::c_int as libc::c_uchar,
    0x11 as libc::c_int as libc::c_uchar,
    0x36 as libc::c_int as libc::c_uchar,
    0xe5 as libc::c_int as libc::c_uchar,
    0x18 as libc::c_int as libc::c_uchar,
    0x37 as libc::c_int as libc::c_uchar,
    0xcc as libc::c_int as libc::c_uchar,
    0x13 as libc::c_int as libc::c_uchar,
    0x39 as libc::c_int as libc::c_uchar,
    0x89 as libc::c_int as libc::c_uchar,
    0x97 as libc::c_int as libc::c_uchar,
    0x14 as libc::c_int as libc::c_uchar,
    0x1b as libc::c_int as libc::c_uchar,
    0x8a as libc::c_int as libc::c_uchar,
    0x3a as libc::c_int as libc::c_uchar,
    0xbd as libc::c_int as libc::c_uchar,
    0xa2 as libc::c_int as libc::c_uchar,
    0x5e as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x85 as libc::c_int as libc::c_uchar,
    0xb0 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0xa3 as libc::c_int as libc::c_uchar,
    0xa5 as libc::c_int as libc::c_uchar,
    0x2c as libc::c_int as libc::c_uchar,
    0x16 as libc::c_int as libc::c_uchar,
    0x88 as libc::c_int as libc::c_uchar,
    0xbc as libc::c_int as libc::c_uchar,
    0x12 as libc::c_int as libc::c_uchar,
    0xa6 as libc::c_int as libc::c_uchar,
    0x61 as libc::c_int as libc::c_uchar,
    0xe6 as libc::c_int as libc::c_uchar,
    0x56 as libc::c_int as libc::c_uchar,
    0x62 as libc::c_int as libc::c_uchar,
    0x19 as libc::c_int as libc::c_uchar,
    0xdb as libc::c_int as libc::c_uchar,
    0x1a as libc::c_int as libc::c_uchar,
    0xa8 as libc::c_int as libc::c_uchar,
    0x32 as libc::c_int as libc::c_uchar,
    0x1c as libc::c_int as libc::c_uchar,
    0x8b as libc::c_int as libc::c_uchar,
    0xcd as libc::c_int as libc::c_uchar,
    0x1d as libc::c_int as libc::c_uchar,
    0xa9 as libc::c_int as libc::c_uchar,
    0xc3 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0xc4 as libc::c_int as libc::c_uchar,
    0x50 as libc::c_int as libc::c_uchar,
    0x5d as libc::c_int as libc::c_uchar,
    0xc0 as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0xc6 as libc::c_int as libc::c_uchar,
    0x2e as libc::c_int as libc::c_uchar,
    0x53 as libc::c_int as libc::c_uchar,
    0x60 as libc::c_int as libc::c_uchar,
    0x31 as libc::c_int as libc::c_uchar,
    0x52 as libc::c_int as libc::c_uchar,
    0xc2 as libc::c_int as libc::c_uchar,
    0x34 as libc::c_int as libc::c_uchar,
    0xc8 as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x57 as libc::c_int as libc::c_uchar,
    0x3e as libc::c_int as libc::c_uchar,
    0xce as libc::c_int as libc::c_uchar,
    0x3b as libc::c_int as libc::c_uchar,
    0xc9 as libc::c_int as libc::c_uchar,
    0x6a as libc::c_int as libc::c_uchar,
    0x54 as libc::c_int as libc::c_uchar,
    0x4f as libc::c_int as libc::c_uchar,
    0x38 as libc::c_int as libc::c_uchar,
    0x58 as libc::c_int as libc::c_uchar,
    0xcb as libc::c_int as libc::c_uchar,
    0x2f as libc::c_int as libc::c_uchar,
    0xca as libc::c_int as libc::c_uchar,
];
static mut lo_base: [libc::c_uchar; 8] = [
    0 as libc::c_int as libc::c_uchar,
    0x7 as libc::c_int as libc::c_uchar,
    0xc as libc::c_int as libc::c_uchar,
    0x19 as libc::c_int as libc::c_uchar,
    0x24 as libc::c_int as libc::c_uchar,
    0x32 as libc::c_int as libc::c_uchar,
    0x40 as libc::c_int as libc::c_uchar,
    0x47 as libc::c_int as libc::c_uchar,
];
static mut lo_offset: [libc::c_uchar; 128] = [
    0xff as libc::c_int as libc::c_uchar,
    0xf0 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x1f as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xf2 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x3f as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0xf5 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x6f as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xf0 as libc::c_int as libc::c_uchar,
    0xf1 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x2f as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x3f as libc::c_int as libc::c_uchar,
    0x4f as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xf as libc::c_int as libc::c_uchar,
    0xf1 as libc::c_int as libc::c_uchar,
    0xf2 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x3f as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0xf5 as libc::c_int as libc::c_uchar,
    0xf6 as libc::c_int as libc::c_uchar,
    0xf7 as libc::c_int as libc::c_uchar,
    0x89 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xab as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xfc as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xf as libc::c_int as libc::c_uchar,
    0x1f as libc::c_int as libc::c_uchar,
    0x23 as libc::c_int as libc::c_uchar,
    0x45 as libc::c_int as libc::c_uchar,
    0xf6 as libc::c_int as libc::c_uchar,
    0x7f as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xf8 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xf9 as libc::c_int as libc::c_uchar,
    0xaf as libc::c_int as libc::c_uchar,
    0xf0 as libc::c_int as libc::c_uchar,
    0xf1 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x2f as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xf3 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x4f as libc::c_int as libc::c_uchar,
    0x5f as libc::c_int as libc::c_uchar,
    0x67 as libc::c_int as libc::c_uchar,
    0x89 as libc::c_int as libc::c_uchar,
    0xfa as libc::c_int as libc::c_uchar,
    0xbf as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xcd as libc::c_int as libc::c_uchar,
    0xf0 as libc::c_int as libc::c_uchar,
    0xf1 as libc::c_int as libc::c_uchar,
    0xf2 as libc::c_int as libc::c_uchar,
    0x3f as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0x56 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x7f as libc::c_int as libc::c_uchar,
    0x8f as libc::c_int as libc::c_uchar,
    0x9a as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xbc as libc::c_int as libc::c_uchar,
    0xdf as libc::c_int as libc::c_uchar,
    0xf as libc::c_int as libc::c_uchar,
    0x1f as libc::c_int as libc::c_uchar,
    0xf2 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x3f as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xf5 as libc::c_int as libc::c_uchar,
    0x6f as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xf as libc::c_int as libc::c_uchar,
    0x1f as libc::c_int as libc::c_uchar,
    0x23 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x45 as libc::c_int as libc::c_uchar,
    0x6f as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xf7 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xf8 as libc::c_int as libc::c_uchar,
    0x9f as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
];
#[inline]
unsafe extern fn decode_lo(mut sig: libc::c_int) -> libc::c_schar {
    let mut offset: libc::c_uchar = (sig >> 1 as libc::c_int & 0x1 as libc::c_int
        | sig >> 3 as libc::c_int & 0x6 as libc::c_int
        | sig >> 5 as libc::c_int & 0x18 as libc::c_int
        | sig >> 7 as libc::c_int & 0x60 as libc::c_int)
        as libc::c_uchar;
    let mut idx: libc::c_uchar = lo_offset[offset as usize];
    let mut base: libc::c_uchar = 0;
    let mut c: libc::c_uchar = 0;
    if sig & 1 as libc::c_int != 0 {
        idx = (idx as libc::c_int & 0xf as libc::c_int) as libc::c_uchar
    } else {
        idx = (idx as libc::c_int >> 4 as libc::c_int) as libc::c_uchar
    }
    if idx as libc::c_int == 0xf as libc::c_int {
        return -(1 as libc::c_int) as libc::c_schar;
    }
    base = (sig >> 11 as libc::c_int | sig >> 9 as libc::c_int & 1 as libc::c_int) as libc::c_uchar;
    if !((base as libc::c_int) < 8 as libc::c_int) {
        fprintf(
            stderr,
            b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\tsig=%x offset=%x idx=%x base=%x\n\x00"
                as *const u8 as *const libc::c_char,
            b"zbar/decoder/code128.c\x00" as *const u8 as *const libc::c_char,
            134 as libc::c_int,
            (*::std::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"decode_lo\x00")).as_ptr(),
            b"base < 8\x00" as *const u8 as *const libc::c_char,
            sig,
            offset as libc::c_int,
            idx as libc::c_int,
            base as libc::c_int,
        );
        return -(1 as libc::c_int) as libc::c_schar;
    }
    idx = (idx as libc::c_int + lo_base[base as usize] as libc::c_int) as libc::c_uchar;
    if !(idx as libc::c_int <= 0x50 as libc::c_int) {
        fprintf(
            stderr,
            b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\tsig=%x offset=%x base=%x idx=%x\n\x00"
                as *const u8 as *const libc::c_char,
            b"zbar/decoder/code128.c\x00" as *const u8 as *const libc::c_char,
            138 as libc::c_int,
            (*::std::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"decode_lo\x00")).as_ptr(),
            b"idx <= 0x50\x00" as *const u8 as *const libc::c_char,
            sig,
            offset as libc::c_int,
            base as libc::c_int,
            idx as libc::c_int,
        );
        return -(1 as libc::c_int) as libc::c_schar;
    }
    c = characters[idx as usize];
    return c as libc::c_schar;
}
#[inline]
unsafe extern fn decode_hi(mut sig: libc::c_int) -> libc::c_schar {
    let mut rev: libc::c_uchar =
        (sig & 0x4400 as libc::c_int != 0 as libc::c_int) as libc::c_int as libc::c_uchar;
    let mut idx: libc::c_uchar = 0;
    let mut c: libc::c_uchar = 0;
    if rev != 0 {
        sig = sig >> 12 as libc::c_int & 0xf as libc::c_int
            | sig >> 4 as libc::c_int & 0xf0 as libc::c_int
            | sig << 4 as libc::c_int & 0xf00 as libc::c_int
            | sig << 12 as libc::c_int & 0xf000 as libc::c_int
    }
    match sig {
        20 => idx = 0 as libc::c_int as libc::c_uchar,
        37 => idx = 0x1 as libc::c_int as libc::c_uchar,
        52 => idx = 0x2 as libc::c_int as libc::c_uchar,
        308 => idx = 0x3 as libc::c_int as libc::c_uchar,
        323 => idx = 0x4 as libc::c_int as libc::c_uchar,
        579 => idx = 0x5 as libc::c_int as libc::c_uchar,
        833 => idx = 0x6 as libc::c_int as libc::c_uchar,
        850 => idx = 0x7 as libc::c_int as libc::c_uchar,
        4132 => idx = 0x8 as libc::c_int as libc::c_uchar,
        4372 => idx = 0x9 as libc::c_int as libc::c_uchar,
        4404 => idx = 0xa as libc::c_int as libc::c_uchar,
        4674 => idx = 0xb as libc::c_int as libc::c_uchar,
        4675 => idx = 0xc as libc::c_int as libc::c_uchar,
        5185 => {
            idx = 0xd as libc::c_int as libc::c_uchar;
            rev = 0 as libc::c_int as libc::c_uchar
        }
        _ => return -(1 as libc::c_int) as libc::c_schar,
    }
    if rev != 0 {
        idx = (idx as libc::c_int + 0xe as libc::c_int) as libc::c_uchar
    }
    c = characters[(0x51 as libc::c_int + idx as libc::c_int) as usize];
    return c as libc::c_schar;
}
#[inline]
unsafe extern fn calc_check(mut c: libc::c_uchar) -> libc::c_uchar {
    if c as libc::c_int & 0x80 as libc::c_int == 0 {
        return 0x18 as libc::c_int as libc::c_uchar;
    }
    c = (c as libc::c_int & 0x7f as libc::c_int) as libc::c_uchar;
    if (c as libc::c_int) < 0x3d as libc::c_int {
        return if (c as libc::c_int) < 0x30 as libc::c_int
            && c as libc::c_int != 0x17 as libc::c_int
        {
            0x10 as libc::c_int
        } else {
            0x20 as libc::c_int
        } as libc::c_uchar;
    }
    if (c as libc::c_int) < 0x50 as libc::c_int {
        return if c as libc::c_int == 0x4d as libc::c_int {
            0x20 as libc::c_int
        } else {
            0x10 as libc::c_int
        } as libc::c_uchar;
    }
    return if (c as libc::c_int) < 0x67 as libc::c_int {
        0x20 as libc::c_int
    } else {
        0x10 as libc::c_int
    } as libc::c_uchar;
}
#[inline]
unsafe extern fn decode6(mut dcode: *mut zbar_decoder_t) -> libc::c_schar {
    let mut sig: libc::c_int = 0;
    let mut c: libc::c_schar = 0;
    let mut chk: libc::c_schar = 0;
    let mut bars: libc::c_uint = 0;
    /* build edge signature of character */
    let mut s: libc::c_uint = (*dcode).code128.s6;
    if s < 5 as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int) as libc::c_schar;
    }
    /* calculate similar edge measurements */
    sig = if get_color(dcode) as libc::c_int == ZBAR_BAR as libc::c_int {
        (decode_e(
            get_width(dcode, 0 as libc::c_int as libc::c_uchar)
                .wrapping_add(get_width(dcode, 1 as libc::c_int as libc::c_uchar)),
            s,
            11 as libc::c_int as libc::c_uint,
        ) << 12 as libc::c_int
            | decode_e(
                get_width(dcode, 1 as libc::c_int as libc::c_uchar)
                    .wrapping_add(get_width(dcode, 2 as libc::c_int as libc::c_uchar)),
                s,
                11 as libc::c_int as libc::c_uint,
            ) << 8 as libc::c_int
            | decode_e(
                get_width(dcode, 2 as libc::c_int as libc::c_uchar)
                    .wrapping_add(get_width(dcode, 3 as libc::c_int as libc::c_uchar)),
                s,
                11 as libc::c_int as libc::c_uint,
            ) << 4 as libc::c_int)
            | decode_e(
                get_width(dcode, 3 as libc::c_int as libc::c_uchar)
                    .wrapping_add(get_width(dcode, 4 as libc::c_int as libc::c_uchar)),
                s,
                11 as libc::c_int as libc::c_uint,
            )
    } else {
        (decode_e(
            get_width(dcode, 5 as libc::c_int as libc::c_uchar)
                .wrapping_add(get_width(dcode, 4 as libc::c_int as libc::c_uchar)),
            s,
            11 as libc::c_int as libc::c_uint,
        ) << 12 as libc::c_int
            | decode_e(
                get_width(dcode, 4 as libc::c_int as libc::c_uchar)
                    .wrapping_add(get_width(dcode, 3 as libc::c_int as libc::c_uchar)),
                s,
                11 as libc::c_int as libc::c_uint,
            ) << 8 as libc::c_int
            | decode_e(
                get_width(dcode, 3 as libc::c_int as libc::c_uchar)
                    .wrapping_add(get_width(dcode, 2 as libc::c_int as libc::c_uchar)),
                s,
                11 as libc::c_int as libc::c_uint,
            ) << 4 as libc::c_int)
            | decode_e(
                get_width(dcode, 2 as libc::c_int as libc::c_uchar)
                    .wrapping_add(get_width(dcode, 1 as libc::c_int as libc::c_uchar)),
                s,
                11 as libc::c_int as libc::c_uint,
            )
    };
    if sig < 0 as libc::c_int {
        return -(1 as libc::c_int) as libc::c_schar;
    }
    /* lookup edge signature */
    c = if sig & 0x4444 as libc::c_int != 0 {
        decode_hi(sig) as libc::c_int
    } else {
        decode_lo(sig) as libc::c_int
    } as libc::c_schar;
    if c as libc::c_int == -(1 as libc::c_int) {
        return -(1 as libc::c_int) as libc::c_schar;
    }
    /* character validation */
    bars = if get_color(dcode) as libc::c_int == ZBAR_BAR as libc::c_int {
        get_width(dcode, 0 as libc::c_int as libc::c_uchar)
            .wrapping_add(get_width(dcode, 2 as libc::c_int as libc::c_uchar))
            .wrapping_add(get_width(dcode, 4 as libc::c_int as libc::c_uchar))
    } else {
        get_width(dcode, 1 as libc::c_int as libc::c_uchar)
            .wrapping_add(get_width(dcode, 3 as libc::c_int as libc::c_uchar))
            .wrapping_add(get_width(dcode, 5 as libc::c_int as libc::c_uchar))
    };
    bars = bars
        .wrapping_mul(11 as libc::c_int as libc::c_uint)
        .wrapping_mul(4 as libc::c_int as libc::c_uint)
        .wrapping_div(s);
    chk = calc_check(c as libc::c_uchar) as libc::c_schar;
    if (chk as libc::c_int - 7 as libc::c_int) as libc::c_uint > bars
        || bars > (chk as libc::c_int + 7 as libc::c_int) as libc::c_uint
    {
        return -(1 as libc::c_int) as libc::c_schar;
    }
    return (c as libc::c_int & 0x7f as libc::c_int) as libc::c_schar;
}
#[inline]
unsafe extern fn validate_checksum(mut dcode: *mut zbar_decoder_t) -> libc::c_uchar {
    let mut idx: libc::c_uint = 0;
    let mut sum: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    let mut acc: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut check: libc::c_uchar = 0;
    let mut err: libc::c_uchar = 0;
    let mut dcode128: *mut code128_decoder_t = &mut (*dcode).code128;
    if (*dcode128).character() < 3 as libc::c_int {
        return 1 as libc::c_int as libc::c_uchar;
    }
    /* add in irregularly weighted start character */
    idx = if (*dcode128).direction() as libc::c_int != 0 {
        ((*dcode128).character()) - 1 as libc::c_int
    } else {
        0 as libc::c_int
    } as libc::c_uint;
    sum = *(*dcode).buf.offset(idx as isize) as libc::c_uint;
    if sum >= 103 as libc::c_int as libc::c_uint {
        sum = sum.wrapping_sub(103 as libc::c_int as libc::c_uint)
    }
    /* calculate sum in reverse to avoid multiply operations */
    i = ((*dcode128).character() - 3 as libc::c_int) as libc::c_uint;
    while i != 0 {
        if !(sum < 103 as libc::c_int as libc::c_uint) {
            fprintf(stderr,
                    b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\tdir=%x i=%x sum=%x acc=%x %s\n\x00"
                        as *const u8 as *const libc::c_char,
                    b"zbar/decoder/code128.c\x00" as *const u8 as
                        *const libc::c_char, 255 as libc::c_int,
                    (*::std::mem::transmute::<&[u8; 18],
                                              &[libc::c_char; 18]>(b"validate_checksum\x00")).as_ptr(),
                    b"sum < 103\x00" as *const u8 as *const libc::c_char,
                    (*dcode128).direction() as libc::c_int, i, sum, acc,
                    _zbar_decoder_buf_dump((*dcode).buf,
                                           (*dcode128).character() as
                                               libc::c_uint));
            return -(1 as libc::c_int) as libc::c_uchar;
        }
        idx = if (*dcode128).direction() as libc::c_int != 0 {
            (((*dcode128).character() - 1 as libc::c_int) as libc::c_uint).wrapping_sub(i)
        } else {
            i
        };
        acc = acc.wrapping_add(*(*dcode).buf.offset(idx as isize) as libc::c_uint);
        if acc >= 103 as libc::c_int as libc::c_uint {
            acc = acc.wrapping_sub(103 as libc::c_int as libc::c_uint)
        }
        if !(acc < 103 as libc::c_int as libc::c_uint) {
            fprintf(stderr,
                    b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\tdir=%x i=%x sum=%x acc=%x %s\n\x00"
                        as *const u8 as *const libc::c_char,
                    b"zbar/decoder/code128.c\x00" as *const u8 as
                        *const libc::c_char, 262 as libc::c_int,
                    (*::std::mem::transmute::<&[u8; 18],
                                              &[libc::c_char; 18]>(b"validate_checksum\x00")).as_ptr(),
                    b"acc < 103\x00" as *const u8 as *const libc::c_char,
                    (*dcode128).direction() as libc::c_int, i, sum, acc,
                    _zbar_decoder_buf_dump((*dcode).buf,
                                           (*dcode128).character() as
                                               libc::c_uint));
            return -(1 as libc::c_int) as libc::c_uchar;
        }
        sum = sum.wrapping_add(acc);
        if sum >= 103 as libc::c_int as libc::c_uint {
            sum = sum.wrapping_sub(103 as libc::c_int as libc::c_uint)
        }
        i = i.wrapping_sub(1)
    }
    /* and compare to check character */
    idx = if (*dcode128).direction() as libc::c_int != 0 {
        1 as libc::c_int
    } else {
        ((*dcode128).character()) - 2 as libc::c_int
    } as libc::c_uint;
    check = *(*dcode).buf.offset(idx as isize);
    err = (sum != check as libc::c_uint) as libc::c_int as libc::c_uchar;
    (err) != 0;
    return err;
}
/* expand and decode character set C */
#[inline]
unsafe extern fn postprocess_c(
    mut dcode: *mut zbar_decoder_t,
    mut start: libc::c_uint,
    mut end: libc::c_uint,
    mut dst: libc::c_uint,
) -> libc::c_uint {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    /* expand buffer to accomodate 2x set C characters (2 digits per-char) */
    let mut delta: libc::c_uint = end.wrapping_sub(start);
    let mut newlen: libc::c_uint =
        ((*dcode).code128.character() as libc::c_uint).wrapping_add(delta);
    size_buf(dcode, newlen);
    /* relocate unprocessed data to end of buffer */
    memmove(
        (*dcode).buf.offset(start as isize).offset(delta as isize) as *mut libc::c_void,
        (*dcode).buf.offset(start as isize) as *const libc::c_void,
        ((*dcode).code128.character() as libc::c_uint).wrapping_sub(start) as libc::c_ulong,
    );
    (*dcode).code128.set_character(newlen as libc::c_int);
    i = 0 as libc::c_int as libc::c_uint;
    j = dst;
    while i < delta {
        /* convert each set C character into two ASCII digits */
        let mut code: libc::c_uchar =
            *(*dcode).buf.offset(start.wrapping_add(delta).wrapping_add(i) as isize);
        *(*dcode).buf.offset(j as isize) = '0' as i32 as libc::c_uchar;
        if code as libc::c_int >= 50 as libc::c_int {
            code = (code as libc::c_int - 50 as libc::c_int) as libc::c_uchar;
            let ref mut fresh0 = *(*dcode).buf.offset(j as isize);
            *fresh0 = (*fresh0 as libc::c_int + 5 as libc::c_int) as libc::c_uchar
        }
        if code as libc::c_int >= 30 as libc::c_int {
            code = (code as libc::c_int - 30 as libc::c_int) as libc::c_uchar;
            let ref mut fresh1 = *(*dcode).buf.offset(j as isize);
            *fresh1 = (*fresh1 as libc::c_int + 3 as libc::c_int) as libc::c_uchar
        }
        if code as libc::c_int >= 20 as libc::c_int {
            code = (code as libc::c_int - 20 as libc::c_int) as libc::c_uchar;
            let ref mut fresh2 = *(*dcode).buf.offset(j as isize);
            *fresh2 = (*fresh2 as libc::c_int + 2 as libc::c_int) as libc::c_uchar
        }
        if code as libc::c_int >= 10 as libc::c_int {
            code = (code as libc::c_int - 10 as libc::c_int) as libc::c_uchar;
            let ref mut fresh3 = *(*dcode).buf.offset(j as isize);
            *fresh3 = (*fresh3 as libc::c_int + 1 as libc::c_int) as libc::c_uchar
        }
        if !(*(*dcode).buf.offset(j as isize) as libc::c_int <= '9' as i32) {
            fprintf(stderr,
                    b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\tstart=%x end=%x i=%x j=%x %s\n\x00"
                        as *const u8 as *const libc::c_char,
                    b"zbar/decoder/code128.c\x00" as *const u8 as
                        *const libc::c_char, 318 as libc::c_int,
                    (*::std::mem::transmute::<&[u8; 14],
                                              &[libc::c_char; 14]>(b"postprocess_c\x00")).as_ptr(),
                    b"dcode->buf[j] <= \'9\'\x00" as *const u8 as
                        *const libc::c_char, start, end, i, j,
                    _zbar_decoder_buf_dump((*dcode).buf,
                                           (*dcode).code128.character() as
                                               libc::c_uint));
            return delta;
        }
        if !(code as libc::c_int <= 9 as libc::c_int) {
            fprintf(stderr,
                    b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\tstart=%x end=%x i=%x j=%x %s\n\x00"
                        as *const u8 as *const libc::c_char,
                    b"zbar/decoder/code128.c\x00" as *const u8 as
                        *const libc::c_char, 321 as libc::c_int,
                    (*::std::mem::transmute::<&[u8; 14],
                                              &[libc::c_char; 14]>(b"postprocess_c\x00")).as_ptr(),
                    b"code <= 9\x00" as *const u8 as *const libc::c_char,
                    start, end, i, j,
                    _zbar_decoder_buf_dump((*dcode).buf,
                                           (*dcode).code128.character() as
                                               libc::c_uint));
            return delta;
        }
        *(*dcode).buf.offset(j.wrapping_add(1 as libc::c_int as libc::c_uint) as isize) =
            ('0' as i32 + code as libc::c_int) as libc::c_uchar;
        i = i.wrapping_add(1);
        j = j.wrapping_add(2 as libc::c_int as libc::c_uint)
    }
    return delta;
}
/* resolve scan direction and convert to ASCII */
#[inline]
unsafe extern fn postprocess(mut dcode: *mut zbar_decoder_t) -> libc::c_uchar {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut cexp: libc::c_uint = 0;
    let mut code: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    let mut charset: libc::c_uchar = 0;
    let mut dcode128: *mut code128_decoder_t = &mut (*dcode).code128;
    (*dcode).modifiers = 0 as libc::c_int as libc::c_uint;
    (*dcode).direction =
        1 as libc::c_int - 2 as libc::c_int * (*dcode128).direction() as libc::c_int;
    if (*dcode128).direction() != 0 {
        /* reverse buffer */
        i = 0 as libc::c_int as libc::c_uint;
        while i < ((*dcode128).character() / 2 as libc::c_int) as libc::c_uint {
            let mut j_0: libc::c_uint =
                (((*dcode128).character() - 1 as libc::c_int) as libc::c_uint).wrapping_sub(i);
            code = *(*dcode).buf.offset(i as isize);
            *(*dcode).buf.offset(i as isize) = *(*dcode).buf.offset(j_0 as isize);
            *(*dcode).buf.offset(j_0 as isize) = code;
            i = i.wrapping_add(1)
        }
        if !(*(*dcode).buf.offset(((*dcode128).character() - 1 as libc::c_int) as isize)
            as libc::c_int
            == STOP_REV as libc::c_int)
        {
            fprintf(
                stderr,
                b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\tdir=%x %s\n\x00" as *const u8
                    as *const libc::c_char,
                b"zbar/decoder/code128.c\x00" as *const u8 as *const libc::c_char,
                347 as libc::c_int,
                (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"postprocess\x00"))
                    .as_ptr(),
                b"dcode->buf[dcode128->character - 1] == STOP_REV\x00" as *const u8
                    as *const libc::c_char,
                (*dcode128).direction() as libc::c_int,
                _zbar_decoder_buf_dump((*dcode).buf, (*dcode).code128.character() as libc::c_uint),
            );
            return 1 as libc::c_int as libc::c_uchar;
        }
    } else if !(*(*dcode).buf.offset(((*dcode128).character() - 1 as libc::c_int) as isize)
        as libc::c_int
        == STOP_FWD as libc::c_int)
    {
        fprintf(
            stderr,
            b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\tdir=%x %s\n\x00" as *const u8
                as *const libc::c_char,
            b"zbar/decoder/code128.c\x00" as *const u8 as *const libc::c_char,
            352 as libc::c_int,
            (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"postprocess\x00")).as_ptr(),
            b"dcode->buf[dcode128->character - 1] == STOP_FWD\x00" as *const u8
                as *const libc::c_char,
            (*dcode128).direction() as libc::c_int,
            _zbar_decoder_buf_dump((*dcode).buf, (*dcode).code128.character() as libc::c_uint),
        );
        return 1 as libc::c_int as libc::c_uchar;
    }
    code = *(*dcode).buf.offset(0 as libc::c_int as isize);
    if !(code as libc::c_int >= START_A as libc::c_int
        && code as libc::c_int <= START_C as libc::c_int)
    {
        fprintf(
            stderr,
            b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\t%s\n\x00" as *const u8
                as *const libc::c_char,
            b"zbar/decoder/code128.c\x00" as *const u8 as *const libc::c_char,
            356 as libc::c_int,
            (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"postprocess\x00")).as_ptr(),
            b"code >= START_A && code <= START_C\x00" as *const u8 as *const libc::c_char,
            _zbar_decoder_buf_dump((*dcode).buf, (*dcode).code128.character() as libc::c_uint),
        );
        return 1 as libc::c_int as libc::c_uchar;
    }
    charset = (code as libc::c_int - START_A as libc::c_int) as libc::c_uchar;
    cexp = if code as libc::c_int == START_C as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } as libc::c_uint;
    i = 1 as libc::c_int as libc::c_uint;
    j = 0 as libc::c_int as libc::c_uint;
    while i < ((*dcode128).character() - 2 as libc::c_int) as libc::c_uint {
        let mut code_0: libc::c_uchar = *(*dcode).buf.offset(i as isize);
        if code_0 as libc::c_int & 0x80 as libc::c_int != 0 {
            fprintf(stderr,
                    b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\ti=%x j=%x code=%02x charset=%x cexp=%x %s\n\x00"
                        as *const u8 as *const libc::c_char,
                    b"zbar/decoder/code128.c\x00" as *const u8 as
                        *const libc::c_char, 367 as libc::c_int,
                    (*::std::mem::transmute::<&[u8; 12],
                                              &[libc::c_char; 12]>(b"postprocess\x00")).as_ptr(),
                    b"!(code & 0x80)\x00" as *const u8 as *const libc::c_char,
                    i, j, code_0 as libc::c_int, charset as libc::c_int, cexp,
                    _zbar_decoder_buf_dump((*dcode).buf,
                                           (*dcode).code128.character() as
                                               libc::c_uint));
            return 1 as libc::c_int as libc::c_uchar;
        }
        if !(charset as libc::c_int & 0x2 as libc::c_int != 0
            && (code_0 as libc::c_int) < 100 as libc::c_int)
        {
            if (code_0 as libc::c_int) < 0x60 as libc::c_int {
                /* convert character set B to ASCII */
                code_0 = (code_0 as libc::c_int + 0x20 as libc::c_int) as libc::c_uchar;
                if (charset == 0 || charset as libc::c_int == 0x81 as libc::c_int)
                    && code_0 as libc::c_int >= 0x60 as libc::c_int
                {
                    /* convert character set A to ASCII */
                    code_0 = (code_0 as libc::c_int - 0x60 as libc::c_int) as libc::c_uchar
                }
                let fresh4 = j;
                j = j.wrapping_add(1);
                *(*dcode).buf.offset(fresh4 as isize) = code_0;
                if charset as libc::c_int & 0x80 as libc::c_int != 0 {
                    charset = (charset as libc::c_int & 0x7f as libc::c_int) as libc::c_uchar
                }
            } else {
                if charset as libc::c_int & 0x2 as libc::c_int != 0 {
                    let mut delta: libc::c_uint = 0;
                    /* expand character set C to ASCII */
                    if cexp == 0 {
                        fprintf(stderr,
                                b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\ti=%x j=%x code=%02x charset=%x cexp=%x %s\n\x00"
                                    as *const u8 as *const libc::c_char,
                                b"zbar/decoder/code128.c\x00" as *const u8 as
                                    *const libc::c_char, 390 as libc::c_int,
                                (*::std::mem::transmute::<&[u8; 12],
                                                          &[libc::c_char; 12]>(b"postprocess\x00")).as_ptr(),
                                b"cexp\x00" as *const u8 as
                                    *const libc::c_char, i, j,
                                code_0 as libc::c_int, charset as libc::c_int,
                                cexp,
                                _zbar_decoder_buf_dump((*dcode).buf,
                                                       (*dcode).code128.character()
                                                           as libc::c_uint));
                        return 1 as libc::c_int as libc::c_uchar;
                    }
                    delta = postprocess_c(dcode, cexp, i, j);
                    i = i.wrapping_add(delta);
                    j = j.wrapping_add(delta.wrapping_mul(2 as libc::c_int as libc::c_uint));
                    cexp = 0 as libc::c_int as libc::c_uint
                }
                if (code_0 as libc::c_int) < CODE_C as libc::c_int {
                    if code_0 as libc::c_int == SHIFT as libc::c_int {
                        charset = (charset as libc::c_int | 0x80 as libc::c_int) as libc::c_uchar
                    } else if !(code_0 as libc::c_int == FNC2 as libc::c_int) {
                        (code_0 as libc::c_int) == FNC3 as libc::c_int;
                    }
                } else if code_0 as libc::c_int == FNC1 as libc::c_int {
                    /* FNC1 - Code 128 subsets or ASCII 0x1d */
                    if i == 1 as libc::c_int as libc::c_uint {
                        (*dcode).modifiers |=
                            ((1 as libc::c_int) << ZBAR_MOD_GS1 as libc::c_int) as libc::c_uint
                    } else if i == 2 as libc::c_int as libc::c_uint {
                        (*dcode).modifiers |=
                            ((1 as libc::c_int) << ZBAR_MOD_AIM as libc::c_int) as libc::c_uint
                    } else if i < ((*dcode).code128.character() - 3 as libc::c_int) as libc::c_uint
                    {
                        let fresh5 = j;
                        j = j.wrapping_add(1);
                        *(*dcode).buf.offset(fresh5 as isize) = 0x1d as libc::c_int as libc::c_uchar
                    }
                /* else drop trailing FNC1 */
                } else if code_0 as libc::c_int >= START_A as libc::c_int {
                    return 1 as libc::c_int as libc::c_uchar;
                } else {
                    let mut newset: libc::c_uchar =
                        (CODE_A as libc::c_int - code_0 as libc::c_int) as libc::c_uchar;
                    if !(code_0 as libc::c_int >= CODE_C as libc::c_int
                        && code_0 as libc::c_int <= CODE_A as libc::c_int)
                    {
                        fprintf(stderr,
                                b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\ti=%x j=%x code=%02x charset=%x cexp=%x %s\n\x00"
                                    as *const u8 as *const libc::c_char,
                                b"zbar/decoder/code128.c\x00" as *const u8 as
                                    *const libc::c_char, 426 as libc::c_int,
                                (*::std::mem::transmute::<&[u8; 12],
                                                          &[libc::c_char; 12]>(b"postprocess\x00")).as_ptr(),
                                b"code >= CODE_C && code <= CODE_A\x00" as
                                    *const u8 as *const libc::c_char, i, j,
                                code_0 as libc::c_int, charset as libc::c_int,
                                cexp,
                                _zbar_decoder_buf_dump((*dcode).buf,
                                                       (*dcode).code128.character()
                                                           as libc::c_uint));
                        return 1 as libc::c_int as libc::c_uchar;
                    }
                    if newset as libc::c_int != charset as libc::c_int {
                        charset = newset
                    }
                }
                if charset as libc::c_int & 0x2 as libc::c_int != 0 {
                    cexp = i.wrapping_add(1 as libc::c_int as libc::c_uint)
                }
            }
        }
        /* defer character set C for expansion */
        i = i.wrapping_add(1)
    }
    if charset as libc::c_int & 0x2 as libc::c_int != 0 {
        if cexp == 0 {
            fprintf(stderr,
                    b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\ti=%x j=%x code=%02x charset=%x cexp=%x %s\n\x00"
                        as *const u8 as *const libc::c_char,
                    b"zbar/decoder/code128.c\x00" as *const u8 as
                        *const libc::c_char, 441 as libc::c_int,
                    (*::std::mem::transmute::<&[u8; 12],
                                              &[libc::c_char; 12]>(b"postprocess\x00")).as_ptr(),
                    b"cexp\x00" as *const u8 as *const libc::c_char, i, j,
                    code as libc::c_int, charset as libc::c_int, cexp,
                    _zbar_decoder_buf_dump((*dcode).buf,
                                           (*dcode).code128.character() as
                                               libc::c_uint));
            return 1 as libc::c_int as libc::c_uchar;
        }
        j = j.wrapping_add(
            postprocess_c(dcode, cexp, i, j).wrapping_mul(2 as libc::c_int as libc::c_uint),
        )
    }
    if !(j < (*dcode).buf_alloc) {
        fprintf(
            stderr,
            b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\tj=%02x %s\n\x00" as *const u8
                as *const libc::c_char,
            b"zbar/decoder/code128.c\x00" as *const u8 as *const libc::c_char,
            445 as libc::c_int,
            (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"postprocess\x00")).as_ptr(),
            b"j < dcode->buf_alloc\x00" as *const u8 as *const libc::c_char,
            j,
            _zbar_decoder_buf_dump((*dcode).buf, (*dcode).code128.character() as libc::c_uint),
        );
        return 1 as libc::c_int as libc::c_uchar;
    }
    (*dcode).buflen = j;
    *(*dcode).buf.offset(j as isize) = '\u{0}' as i32 as libc::c_uchar;
    (*dcode).code128.set_character(j as libc::c_int);
    return 0 as libc::c_int as libc::c_uchar;
}
/* reset Code 128 specific state */
/* decode Code 128 symbols */
#[no_mangle]
pub unsafe extern fn _zbar_decode_code128(mut dcode: *mut zbar_decoder_t) -> zbar_symbol_type_t {
    let mut dcode128: *mut code128_decoder_t = &mut (*dcode).code128;
    let mut c: libc::c_schar = 0;
    /* update latest character width */
    (*dcode128).s6 =
        (*dcode128).s6.wrapping_sub(get_width(dcode, 6 as libc::c_int as libc::c_uchar));
    (*dcode128).s6 =
        (*dcode128).s6.wrapping_add(get_width(dcode, 0 as libc::c_int as libc::c_uchar));
    if if (*dcode128).character() < 0 as libc::c_int {
        (get_color(dcode) as libc::c_int != ZBAR_SPACE as libc::c_int) as libc::c_int
    } else {
        (*dcode128).set_element((*dcode128).element() + 1);
        ((*dcode128).element() as libc::c_int != 6 as libc::c_int
            || get_color(dcode) as libc::c_int != (*dcode128).direction() as libc::c_int)
            as libc::c_int
    } != 0
    {
        return ZBAR_NONE;
    }
    (*dcode128).set_element(0 as libc::c_int as libc::c_uint);
    c = decode6(dcode);
    if (*dcode128).character() < 0 as libc::c_int {
        let mut qz: libc::c_uint = 0;
        if (c as libc::c_int) < START_A as libc::c_int
            || c as libc::c_int > STOP_REV as libc::c_int
            || c as libc::c_int == STOP_FWD as libc::c_int
        {
            return ZBAR_NONE;
        }
        qz = get_width(dcode, 6 as libc::c_int as libc::c_uchar);
        if qz != 0
            && qz
                < (*dcode128)
                    .s6
                    .wrapping_mul(3 as libc::c_int as libc::c_uint)
                    .wrapping_div(4 as libc::c_int as libc::c_uint)
        {
            return ZBAR_NONE;
        }
        /* decoded valid start/stop */
        /* initialize state */
        (*dcode128).set_character(1 as libc::c_int);
        if c as libc::c_int == STOP_REV as libc::c_int {
            (*dcode128).set_direction(ZBAR_BAR as libc::c_int as libc::c_uint);
            (*dcode128).set_element(7 as libc::c_int as libc::c_uint)
        } else {
            (*dcode128).set_direction(ZBAR_SPACE as libc::c_int as libc::c_uint)
        }
        (*dcode128).start = c as libc::c_uchar;
        (*dcode128).width = (*dcode128).s6;
        return ZBAR_NONE;
    } else {
        if (c as libc::c_int) < 0 as libc::c_int
            || size_buf(dcode, ((*dcode128).character() + 1 as libc::c_int) as libc::c_uint)
                as libc::c_int
                != 0
        {
            if (*dcode128).character() > 1 as libc::c_int {
                release_lock(dcode, ZBAR_CODE128);
            }
            (*dcode128).set_character(-(1 as libc::c_int));
            return ZBAR_NONE;
        } else {
            let mut dw: libc::c_uint = 0;
            if (*dcode128).width > (*dcode128).s6 {
                dw = (*dcode128).width.wrapping_sub((*dcode128).s6)
            } else {
                dw = (*dcode128).s6.wrapping_sub((*dcode128).width)
            }
            dw = dw.wrapping_mul(4 as libc::c_int as libc::c_uint);
            if dw > (*dcode128).width {
                if (*dcode128).character() > 1 as libc::c_int {
                    release_lock(dcode, ZBAR_CODE128);
                }
                (*dcode128).set_character(-(1 as libc::c_int));
                return ZBAR_NONE;
            }
        }
    }
    (*dcode128).width = (*dcode128).s6;
    if !((*dcode).buf_alloc > (*dcode128).character() as libc::c_uint) {
        fprintf(
            stderr,
            b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\talloc=%x idx=%x c=%02x %s\n\x00"
                as *const u8 as *const libc::c_char,
            b"zbar/decoder/code128.c\x00" as *const u8 as *const libc::c_char,
            528 as libc::c_int,
            (*::std::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"_zbar_decode_code128\x00"))
                .as_ptr(),
            b"dcode->buf_alloc > dcode128->character\x00" as *const u8 as *const libc::c_char,
            (*dcode).buf_alloc,
            (*dcode128).character(),
            c as libc::c_int,
            _zbar_decoder_buf_dump((*dcode).buf, (*dcode).buf_alloc),
        );
        return ZBAR_NONE;
    }
    if (*dcode128).character() == 1 as libc::c_int {
        /* lock shared resources */
        if acquire_lock(dcode, ZBAR_CODE128) != 0 {
            (*dcode128).set_character(-(1 as libc::c_int));
            return ZBAR_NONE;
        }
        *(*dcode).buf.offset(0 as libc::c_int as isize) = (*dcode128).start
    }
    let ref mut fresh6 = (*dcode128).character();
    let fresh7 = *fresh6;
    *fresh6 = *fresh6 + 1;
    *(*dcode).buf.offset(fresh7 as isize) = c as libc::c_uchar;
    if (*dcode128).character() > 2 as libc::c_int
        && (if (*dcode128).direction() as libc::c_int != 0 {
            (c as libc::c_int >= START_A as libc::c_int
                && c as libc::c_int <= START_C as libc::c_int) as libc::c_int
        } else {
            (c as libc::c_int == STOP_FWD as libc::c_int) as libc::c_int
        }) != 0
    {
        /* FIXME STOP_FWD should check extra bar (and QZ!) */
        let mut sym: zbar_symbol_type_t = ZBAR_CODE128;
        if validate_checksum(dcode) as libc::c_int != 0 || postprocess(dcode) as libc::c_int != 0 {
            sym = ZBAR_NONE
        } else if (*dcode128).character()
            < (*dcode128).configs
                [(ZBAR_CFG_MIN_LEN as libc::c_int - ZBAR_CFG_MIN_LEN as libc::c_int) as usize]
            || (*dcode128).configs
                [(ZBAR_CFG_MAX_LEN as libc::c_int - ZBAR_CFG_MIN_LEN as libc::c_int) as usize]
                > 0 as libc::c_int
                && (*dcode128).character()
                    > (*dcode128).configs[(ZBAR_CFG_MAX_LEN as libc::c_int
                        - ZBAR_CFG_MIN_LEN as libc::c_int)
                        as usize]
        {
            sym = ZBAR_NONE
        }
        (*dcode128).set_character(-(1 as libc::c_int));
        if sym as u64 == 0 {
            release_lock(dcode, ZBAR_CODE128);
        }
        return sym;
    }
    return ZBAR_NONE;
}
