use ::c2rust_bitfields;
use ::libc;
extern {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
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
#[inline]
unsafe extern fn get_color(mut dcode: *const zbar_decoder_t) -> libc::c_char {
    return ((*dcode).idx as libc::c_int & 1 as libc::c_int) as libc::c_char;
}
#[inline]
unsafe extern fn size_buf(mut dcode: *mut zbar_decoder_t, mut len: libc::c_uint) -> libc::c_char {
    let mut buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if len <= 0x20 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as libc::c_char;
    }
    if len < (*dcode).buf_alloc {
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
unsafe extern fn decode_sort3(mut dcode: *mut zbar_decoder_t, mut i0: libc::c_int) -> libc::c_uint {
    let mut w0: libc::c_uint = get_width(dcode, i0 as libc::c_uchar);
    let mut w2: libc::c_uint = get_width(dcode, (i0 + 2 as libc::c_int) as libc::c_uchar);
    let mut w4: libc::c_uint = get_width(dcode, (i0 + 4 as libc::c_int) as libc::c_uchar);
    if w0 < w2 {
        if w2 < w4 {
            return (i0 << 8 as libc::c_int
                | (i0 + 2 as libc::c_int) << 4 as libc::c_int
                | i0 + 4 as libc::c_int) as libc::c_uint;
        }
        if w0 < w4 {
            return (i0 << 8 as libc::c_int
                | (i0 + 4 as libc::c_int) << 4 as libc::c_int
                | i0 + 2 as libc::c_int) as libc::c_uint;
        }
        return ((i0 + 4 as libc::c_int) << 8 as libc::c_int
            | i0 << 4 as libc::c_int
            | i0 + 2 as libc::c_int) as libc::c_uint;
    }
    if w4 < w2 {
        return ((i0 + 4 as libc::c_int) << 8 as libc::c_int
            | (i0 + 2 as libc::c_int) << 4 as libc::c_int
            | i0) as libc::c_uint;
    }
    if w0 < w4 {
        return ((i0 + 2 as libc::c_int) << 8 as libc::c_int
            | i0 << 4 as libc::c_int
            | i0 + 4 as libc::c_int) as libc::c_uint;
    }
    return ((i0 + 2 as libc::c_int) << 8 as libc::c_int
        | (i0 + 4 as libc::c_int) << 4 as libc::c_int
        | i0) as libc::c_uint;
}
#[inline]
unsafe extern fn decode_sortn(
    mut dcode: *mut zbar_decoder_t,
    mut n: libc::c_int,
    mut i0: libc::c_int,
) -> libc::c_uint {
    let mut mask: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut sort: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i: libc::c_int = 0;
    i = n - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        let mut wmin: libc::c_uint = (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint);
        let mut jmin: libc::c_int = -(1 as libc::c_int);
        let mut j: libc::c_int = 0;
        j = n - 1 as libc::c_int;
        while j >= 0 as libc::c_int {
            if !(mask >> j & 1 as libc::c_int as libc::c_uint != 0) {
                let mut w: libc::c_uint =
                    get_width(dcode, (i0 + j * 2 as libc::c_int) as libc::c_uchar);
                if wmin >= w {
                    wmin = w;
                    jmin = j
                }
            }
            j -= 1
        }
        if !(jmin >= 0 as libc::c_int) {
            fprintf(
                stderr,
                b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\tsortn(%d,%d) jmin=%d\x00"
                    as *const u8 as *const libc::c_char,
                b"./zbar/decoder.h\x00" as *const u8 as *const libc::c_char,
                238 as libc::c_int,
                (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"decode_sortn\x00"))
                    .as_ptr(),
                b"jmin >= 0\x00" as *const u8 as *const libc::c_char,
                n,
                i0,
                jmin,
            );
            return 0 as libc::c_int as libc::c_uint;
        }
        sort <<= 4 as libc::c_int;
        mask |= ((1 as libc::c_int) << jmin) as libc::c_uint;
        sort |= (i0 + jmin * 2 as libc::c_int) as libc::c_uint;
        i -= 1
    }
    return sort;
}
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
#[inline]
unsafe extern fn get_width(
    mut dcode: *const zbar_decoder_t,
    mut offset: libc::c_uchar,
) -> libc::c_uint {
    return (*dcode).w[((*dcode).idx as libc::c_int - offset as libc::c_int
        & 16 as libc::c_int - 1 as libc::c_int) as usize];
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
/* initial scan buffer size */
static mut codabar_lo: [libc::c_schar; 12] = [
    0 as libc::c_int as libc::c_schar,
    0x1 as libc::c_int as libc::c_schar,
    0x4 as libc::c_int as libc::c_schar,
    0x5 as libc::c_int as libc::c_schar,
    0x2 as libc::c_int as libc::c_schar,
    0xa as libc::c_int as libc::c_schar,
    0xb as libc::c_int as libc::c_schar,
    0x9 as libc::c_int as libc::c_schar,
    0x6 as libc::c_int as libc::c_schar,
    0x7 as libc::c_int as libc::c_schar,
    0x8 as libc::c_int as libc::c_schar,
    0x3 as libc::c_int as libc::c_schar,
];
static mut codabar_hi: [libc::c_uchar; 8] = [
    0x1 as libc::c_int as libc::c_uchar,
    0x4 as libc::c_int as libc::c_uchar,
    0x7 as libc::c_int as libc::c_uchar,
    0x6 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x5 as libc::c_int as libc::c_uchar,
];
static mut codabar_characters: [libc::c_uchar; 20] =
    [48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 45, 36, 58, 47, 46, 43, 65, 66, 67, 68];
#[inline]
unsafe extern fn check_width(mut ref_0: libc::c_uint, mut w: libc::c_uint) -> libc::c_int {
    let mut dref: libc::c_uint = ref_0;
    ref_0 = ref_0.wrapping_mul(4 as libc::c_int as libc::c_uint);
    w = w.wrapping_mul(4 as libc::c_int as libc::c_uint);
    return (ref_0.wrapping_sub(dref) <= w && w <= ref_0.wrapping_add(dref)) as libc::c_int;
}
#[inline]
unsafe extern fn codabar_decode7(mut dcode: *mut zbar_decoder_t) -> libc::c_schar {
    let mut codabar: *mut codabar_decoder_t = &mut (*dcode).codabar;
    let mut s: libc::c_uint = (*codabar).s7;
    if s < 7 as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int) as libc::c_schar;
    }
    /* check width */
    if check_width((*codabar).width, s) == 0 {
        return -(1 as libc::c_int) as libc::c_schar;
    }
    /* extract min/max bar */
    let mut ibar: libc::c_uint = decode_sortn(dcode, 4 as libc::c_int, 1 as libc::c_int);
    let mut wbmax: libc::c_uint =
        get_width(dcode, (ibar & 0xf as libc::c_int as libc::c_uint) as libc::c_uchar);
    let mut wbmin: libc::c_uint = get_width(dcode, (ibar >> 12 as libc::c_int) as libc::c_uchar);
    if (8 as libc::c_int as libc::c_uint).wrapping_mul(wbmin) < wbmax
        || (3 as libc::c_int as libc::c_uint).wrapping_mul(wbmin)
            > (2 as libc::c_int as libc::c_uint).wrapping_mul(wbmax)
    {
        return -(1 as libc::c_int) as libc::c_schar;
    }
    let mut wb1: libc::c_uint = get_width(
        dcode,
        (ibar >> 8 as libc::c_int & 0xf as libc::c_int as libc::c_uint) as libc::c_uchar,
    );
    let mut wb2: libc::c_uint = get_width(
        dcode,
        (ibar >> 4 as libc::c_int & 0xf as libc::c_int as libc::c_uint) as libc::c_uchar,
    );
    let mut b0b3: libc::c_ulong = wbmin.wrapping_mul(wbmax) as libc::c_ulong;
    let mut b1b2: libc::c_ulong = wb1.wrapping_mul(wb2) as libc::c_ulong;
    if b1b2.wrapping_add(b1b2.wrapping_div(8 as libc::c_int as libc::c_ulong)) < b0b3 {
        /* single wide bar combinations */
        if (8 as libc::c_int as libc::c_uint).wrapping_mul(wbmin)
            < (5 as libc::c_int as libc::c_uint).wrapping_mul(wb1)
            || (8 as libc::c_int as libc::c_uint).wrapping_mul(wb1)
                < (5 as libc::c_int as libc::c_uint).wrapping_mul(wb2)
            || (4 as libc::c_int as libc::c_uint).wrapping_mul(wb2)
                > (3 as libc::c_int as libc::c_uint).wrapping_mul(wbmax)
            || wb2.wrapping_mul(wb2) >= wb1.wrapping_mul(wbmax)
        {
            return -(1 as libc::c_int) as libc::c_schar;
        }
        ibar = ibar >> 1 as libc::c_int & 0x3 as libc::c_int as libc::c_uint
    } else if b1b2 > b0b3.wrapping_add(b0b3.wrapping_div(8 as libc::c_int as libc::c_ulong)) {
        /* three wide bars, no wide spaces */
        if (4 as libc::c_int as libc::c_uint).wrapping_mul(wbmin)
            > (3 as libc::c_int as libc::c_uint).wrapping_mul(wb1)
            || (8 as libc::c_int as libc::c_uint).wrapping_mul(wb1)
                < (5 as libc::c_int as libc::c_uint).wrapping_mul(wb2)
            || (8 as libc::c_int as libc::c_uint).wrapping_mul(wb2)
                < (5 as libc::c_int as libc::c_uint).wrapping_mul(wbmax)
            || wbmin.wrapping_mul(wb2) >= wb1.wrapping_mul(wb1)
        {
            return -(1 as libc::c_int) as libc::c_schar;
        }
        ibar = (ibar >> 13 as libc::c_int).wrapping_add(4 as libc::c_int as libc::c_uint)
    } else {
        return -(1 as libc::c_int) as libc::c_schar;
    }
    let mut ispc: libc::c_uint = decode_sort3(dcode, 2 as libc::c_int);
    let mut wsmax: libc::c_uint =
        get_width(dcode, (ispc & 0xf as libc::c_int as libc::c_uint) as libc::c_uchar);
    let mut wsmid: libc::c_uint = get_width(
        dcode,
        (ispc >> 4 as libc::c_int & 0xf as libc::c_int as libc::c_uint) as libc::c_uchar,
    );
    let mut wsmin: libc::c_uint = get_width(
        dcode,
        (ispc >> 8 as libc::c_int & 0xf as libc::c_int as libc::c_uint) as libc::c_uchar,
    );
    if ibar >> 2 as libc::c_int != 0 {
        /* verify no wide spaces */
        if (8 as libc::c_int as libc::c_uint).wrapping_mul(wsmin) < wsmax
            || (8 as libc::c_int as libc::c_uint).wrapping_mul(wsmin)
                < (5 as libc::c_int as libc::c_uint).wrapping_mul(wsmid)
            || (8 as libc::c_int as libc::c_uint).wrapping_mul(wsmid)
                < (5 as libc::c_int as libc::c_uint).wrapping_mul(wsmax)
        {
            return -(1 as libc::c_int) as libc::c_schar;
        }
        ibar &= 0x3 as libc::c_int as libc::c_uint;
        if (*codabar).direction() != 0 {
            ibar = (3 as libc::c_int as libc::c_uint).wrapping_sub(ibar)
        }
        let mut c: libc::c_int =
            0xfcde as libc::c_int >> (ibar << 2 as libc::c_int) & 0xf as libc::c_int;
        return c as libc::c_schar;
    } else {
        if (8 as libc::c_int as libc::c_uint).wrapping_mul(wsmin) < wsmax
            || (3 as libc::c_int as libc::c_uint).wrapping_mul(wsmin)
                > (2 as libc::c_int as libc::c_uint).wrapping_mul(wsmax)
        {
            return -(1 as libc::c_int) as libc::c_schar;
        }
    }
    let mut s0s2: libc::c_ulong = wsmin.wrapping_mul(wsmax) as libc::c_ulong;
    let mut s1s1: libc::c_ulong = wsmid.wrapping_mul(wsmid) as libc::c_ulong;
    if s1s1.wrapping_add(s1s1.wrapping_div(8 as libc::c_int as libc::c_ulong)) < s0s2 {
        /* single wide space */
        if (8 as libc::c_int as libc::c_uint).wrapping_mul(wsmin)
            < (5 as libc::c_int as libc::c_uint).wrapping_mul(wsmid)
            || (4 as libc::c_int as libc::c_uint).wrapping_mul(wsmid)
                > (3 as libc::c_int as libc::c_uint).wrapping_mul(wsmax)
        {
            return -(1 as libc::c_int) as libc::c_schar;
        }
        ispc = ((ispc & 0xf as libc::c_int as libc::c_uint) >> 1 as libc::c_int)
            .wrapping_sub(1 as libc::c_int as libc::c_uint);
        let mut ic: libc::c_uint = ispc << 2 as libc::c_int | ibar;
        if (*codabar).direction() != 0 {
            ic = (11 as libc::c_int as libc::c_uint).wrapping_sub(ic)
        }
        let mut c_0: libc::c_int = codabar_lo[ic as usize] as libc::c_int;
        return c_0 as libc::c_schar;
    } else if s1s1 > s0s2.wrapping_add(s0s2.wrapping_div(8 as libc::c_int as libc::c_ulong)) {
        /* two wide spaces, check start/stop */
        if (4 as libc::c_int as libc::c_uint).wrapping_mul(wsmin)
            > (3 as libc::c_int as libc::c_uint).wrapping_mul(wsmid)
            || (8 as libc::c_int as libc::c_uint).wrapping_mul(wsmid)
                < (5 as libc::c_int as libc::c_uint).wrapping_mul(wsmax)
        {
            return -(1 as libc::c_int) as libc::c_schar;
        }
        if ispc >> 8 as libc::c_int == 4 as libc::c_int as libc::c_uint {
            return -(1 as libc::c_int) as libc::c_schar;
        }
        ispc >>= 10 as libc::c_int;
        let mut ic_0: libc::c_uint =
            ispc.wrapping_mul(4 as libc::c_int as libc::c_uint).wrapping_add(ibar);
        if !(ic_0 < 8 as libc::c_int as libc::c_uint) {
            fprintf(
                stderr,
                b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\tic=%d ispc=%d ibar=%d\x00"
                    as *const u8 as *const libc::c_char,
                b"zbar/decoder/codabar.c\x00" as *const u8 as *const libc::c_char,
                182 as libc::c_int,
                (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"codabar_decode7\x00"))
                    .as_ptr(),
                b"ic < 8\x00" as *const u8 as *const libc::c_char,
                ic_0,
                ispc,
                ibar,
            );
            return -(1 as libc::c_int) as libc::c_schar;
        }
        let mut c_1: libc::c_uchar = codabar_hi[ic_0 as usize];
        if c_1 as libc::c_int >> 2 as libc::c_int != (*codabar).direction() as libc::c_int {
            return -(1 as libc::c_int) as libc::c_schar;
        }
        c_1 = (c_1 as libc::c_int & 0x3 as libc::c_int | 0x10 as libc::c_int) as libc::c_uchar;
        return c_1 as libc::c_schar;
    } else {
        return -(1 as libc::c_int) as libc::c_schar;
    };
}
#[inline]
unsafe extern fn codabar_decode_start(mut dcode: *mut zbar_decoder_t) -> libc::c_schar {
    let mut codabar: *mut codabar_decoder_t = &mut (*dcode).codabar;
    let mut s: libc::c_uint = (*codabar).s7;
    if s < 8 as libc::c_int as libc::c_uint {
        return ZBAR_NONE as libc::c_int as libc::c_schar;
    }
    /* check leading quiet zone - spec is 10x */
    let mut qz: libc::c_uint = get_width(dcode, 8 as libc::c_int as libc::c_uchar);
    if qz != 0 && qz.wrapping_mul(2 as libc::c_int as libc::c_uint) < s
        || (4 as libc::c_int as libc::c_uint)
            .wrapping_mul(get_width(dcode, 0 as libc::c_int as libc::c_uchar))
            > (3 as libc::c_int as libc::c_uint).wrapping_mul(s)
    {
        return ZBAR_NONE as libc::c_int as libc::c_schar;
    }
    /* check space ratios first */
    let mut ispc: libc::c_uint = decode_sort3(dcode, 2 as libc::c_int);
    if ispc >> 8 as libc::c_int == 4 as libc::c_int as libc::c_uint {
        return ZBAR_NONE as libc::c_int as libc::c_schar;
    }
    /* require 2 wide and 1 narrow spaces */
    let mut wsmax: libc::c_uint =
        get_width(dcode, (ispc & 0xf as libc::c_int as libc::c_uint) as libc::c_uchar);
    let mut wsmin: libc::c_uint = get_width(dcode, (ispc >> 8 as libc::c_int) as libc::c_uchar);
    let mut wsmid: libc::c_uint = get_width(
        dcode,
        (ispc >> 4 as libc::c_int & 0xf as libc::c_int as libc::c_uint) as libc::c_uchar,
    );
    if (8 as libc::c_int as libc::c_uint).wrapping_mul(wsmin) < wsmax
        || (3 as libc::c_int as libc::c_uint).wrapping_mul(wsmin)
            > (2 as libc::c_int as libc::c_uint).wrapping_mul(wsmax)
        || (4 as libc::c_int as libc::c_uint).wrapping_mul(wsmin)
            > (3 as libc::c_int as libc::c_uint).wrapping_mul(wsmid)
        || (8 as libc::c_int as libc::c_uint).wrapping_mul(wsmid)
            < (5 as libc::c_int as libc::c_uint).wrapping_mul(wsmax)
        || wsmid.wrapping_mul(wsmid) <= wsmax.wrapping_mul(wsmin)
    {
        return ZBAR_NONE as libc::c_int as libc::c_schar;
    }
    ispc >>= 10 as libc::c_int;
    /* check bar ratios */
    let mut ibar: libc::c_uint = decode_sortn(dcode, 4 as libc::c_int, 1 as libc::c_int);
    let mut wbmax: libc::c_uint =
        get_width(dcode, (ibar & 0xf as libc::c_int as libc::c_uint) as libc::c_uchar);
    let mut wbmin: libc::c_uint = get_width(dcode, (ibar >> 12 as libc::c_int) as libc::c_uchar);
    if (8 as libc::c_int as libc::c_uint).wrapping_mul(wbmin) < wbmax
        || (3 as libc::c_int as libc::c_uint).wrapping_mul(wbmin)
            > (2 as libc::c_int as libc::c_uint).wrapping_mul(wbmax)
    {
        return ZBAR_NONE as libc::c_int as libc::c_schar;
    }
    /* require 1 wide & 3 narrow bars */
    let mut wb1: libc::c_uint = get_width(
        dcode,
        (ibar >> 8 as libc::c_int & 0xf as libc::c_int as libc::c_uint) as libc::c_uchar,
    );
    let mut wb2: libc::c_uint = get_width(
        dcode,
        (ibar >> 4 as libc::c_int & 0xf as libc::c_int as libc::c_uint) as libc::c_uchar,
    );
    if (8 as libc::c_int as libc::c_uint).wrapping_mul(wbmin)
        < (5 as libc::c_int as libc::c_uint).wrapping_mul(wb1)
        || (8 as libc::c_int as libc::c_uint).wrapping_mul(wb1)
            < (5 as libc::c_int as libc::c_uint).wrapping_mul(wb2)
        || (4 as libc::c_int as libc::c_uint).wrapping_mul(wb2)
            > (3 as libc::c_int as libc::c_uint).wrapping_mul(wbmax)
        || wb1.wrapping_mul(wb2) >= wbmin.wrapping_mul(wbmax)
        || wb2.wrapping_mul(wb2) >= wb1.wrapping_mul(wbmax)
    {
        return ZBAR_NONE as libc::c_int as libc::c_schar;
    }
    ibar = (ibar & 0xf as libc::c_int as libc::c_uint)
        .wrapping_sub(1 as libc::c_int as libc::c_uint)
        >> 1 as libc::c_int;
    /* decode combination */
    let mut ic: libc::c_int =
        ispc.wrapping_mul(4 as libc::c_int as libc::c_uint).wrapping_add(ibar) as libc::c_int;
    if !(ic < 8 as libc::c_int) {
        fprintf(
            stderr,
            b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\tic=%d ispc=%d ibar=%d\x00"
                as *const u8 as *const libc::c_char,
            b"zbar/decoder/codabar.c\x00" as *const u8 as *const libc::c_char,
            270 as libc::c_int,
            (*::std::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"codabar_decode_start\x00"))
                .as_ptr(),
            b"ic < 8\x00" as *const u8 as *const libc::c_char,
            ic,
            ispc,
            ibar,
        );
        return ZBAR_NONE as libc::c_int as libc::c_schar;
    }
    let mut c: libc::c_int = codabar_hi[ic as usize] as libc::c_int;
    (*codabar).buf[0 as libc::c_int as usize] =
        (c & 0x3 as libc::c_int | 0x10 as libc::c_int) as libc::c_uchar;
    /* set character direction */
    (*codabar).set_direction((c >> 2 as libc::c_int) as libc::c_uint);
    (*codabar).set_element(4 as libc::c_int as libc::c_uint);
    (*codabar).set_character(1 as libc::c_int);
    (*codabar).width = (*codabar).s7;
    return ZBAR_PARTIAL as libc::c_int as libc::c_schar;
}
#[inline]
unsafe extern fn codabar_checksum(
    mut dcode: *mut zbar_decoder_t,
    mut n: libc::c_uint,
) -> libc::c_int {
    let mut chk: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut buf: *mut libc::c_uchar = (*dcode).buf;
    loop {
        let fresh0 = n;
        n = n.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        let fresh1 = buf;
        buf = buf.offset(1);
        chk = chk.wrapping_add(*fresh1 as libc::c_uint)
    }
    return (chk & 0xf as libc::c_int as libc::c_uint != 0) as libc::c_int;
}
#[inline]
unsafe extern fn codabar_postprocess(mut dcode: *mut zbar_decoder_t) -> zbar_symbol_type_t {
    let mut codabar: *mut codabar_decoder_t = &mut (*dcode).codabar;
    let mut dir: libc::c_int = (*codabar).direction() as libc::c_int;
    (*dcode).direction = 1 as libc::c_int - 2 as libc::c_int * dir;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = (*codabar).character();
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        *(*dcode).buf.offset(i as isize) = (*codabar).buf[i as usize];
        i += 1
    }
    if dir != 0 {
        /* reverse buffer */
        i = 0 as libc::c_int;
        while i < n / 2 as libc::c_int {
            let mut j: libc::c_uint = (n - 1 as libc::c_int - i) as libc::c_uint;
            let mut code: libc::c_char = *(*dcode).buf.offset(i as isize) as libc::c_char;
            *(*dcode).buf.offset(i as isize) = *(*dcode).buf.offset(j as isize);
            *(*dcode).buf.offset(j as isize) = code as libc::c_uchar;
            i += 1
        }
    }
    if (*codabar).config >> ZBAR_CFG_ADD_CHECK as libc::c_int & 1 as libc::c_int as libc::c_uint
        != 0
    {
        /* validate checksum */
        if codabar_checksum(dcode, n as libc::c_uint) != 0 {
            return ZBAR_NONE;
        }
        if (*codabar).config >> ZBAR_CFG_EMIT_CHECK as libc::c_int
            & 1 as libc::c_int as libc::c_uint
            == 0
        {
            *(*dcode).buf.offset((n - 2 as libc::c_int) as isize) =
                *(*dcode).buf.offset((n - 1 as libc::c_int) as isize);
            n -= 1
        }
    }
    i = 0 as libc::c_int;
    while i < n {
        let mut c: libc::c_uint = *(*dcode).buf.offset(i as isize) as libc::c_uint;
        *(*dcode).buf.offset(i as isize) = if c < 0x14 as libc::c_int as libc::c_uint {
            codabar_characters[c as usize] as libc::c_int
        } else {
            '?' as i32
        } as libc::c_uchar;
        i += 1
    }
    (*dcode).buflen = i as libc::c_uint;
    *(*dcode).buf.offset(i as isize) = '\u{0}' as i32 as libc::c_uchar;
    (*dcode).modifiers = 0 as libc::c_int as libc::c_uint;
    (*codabar).set_character(-(1 as libc::c_int));
    return ZBAR_CODABAR;
}
/* reset Codabar specific state */
/* decode Codabar symbols */
#[no_mangle]
pub unsafe extern fn _zbar_decode_codabar(mut dcode: *mut zbar_decoder_t) -> zbar_symbol_type_t {
    let mut buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut s: libc::c_uint = 0;
    let mut current_block: u64;
    let mut codabar: *mut codabar_decoder_t = &mut (*dcode).codabar;
    /* update latest character width */
    (*codabar).s7 = (*codabar).s7.wrapping_sub(get_width(dcode, 8 as libc::c_int as libc::c_uchar));
    (*codabar).s7 = (*codabar).s7.wrapping_add(get_width(dcode, 1 as libc::c_int as libc::c_uchar));
    if get_color(dcode) as libc::c_int != ZBAR_SPACE as libc::c_int {
        return ZBAR_NONE;
    }
    if (*codabar).character() < 0 as libc::c_int {
        return codabar_decode_start(dcode) as zbar_symbol_type_t;
    }
    if (*codabar).character() < 2 as libc::c_int && codabar_decode_start(dcode) as libc::c_int != 0
    {
        return ZBAR_PARTIAL;
    }
    (*codabar).set_element((*codabar).element() - 1);
    if (*codabar).element() != 0 {
        return ZBAR_NONE;
    }
    (*codabar).set_element(4 as libc::c_int as libc::c_uint);
    let mut c: libc::c_schar = codabar_decode7(dcode);
    if !((c as libc::c_int) < 0 as libc::c_int) {
        buf = 0 as *mut libc::c_uchar;
        if (*codabar).character() < 6 as libc::c_int {
            buf = (*codabar).buf.as_mut_ptr();
            current_block = 9828876828309294594;
        } else if (*codabar).character() >= 0x20 as libc::c_int
            && size_buf(dcode, ((*codabar).character() + 1 as libc::c_int) as libc::c_uint)
                as libc::c_int
                != 0
        {
            current_block = 9416449825185623772;
        } else {
            buf = (*dcode).buf;
            current_block = 9828876828309294594;
        }
        match current_block {
            9416449825185623772 => {}
            _ => {
                let ref mut fresh2 = (*codabar).character();
                let fresh3 = *fresh2;
                *fresh2 = *fresh2 + 1;
                *buf.offset(fresh3 as isize) = c as libc::c_uchar;
                /* lock shared resources */
                if (*codabar).character() == 6 as libc::c_int
                    && acquire_lock(dcode, ZBAR_CODABAR) as libc::c_int != 0
                {
                    (*codabar).set_character(-(1 as libc::c_int));
                    return ZBAR_PARTIAL;
                }
                s = (*codabar).s7;
                if c as libc::c_int & 0x10 as libc::c_int != 0 {
                    let mut qz: libc::c_uint = get_width(dcode, 0 as libc::c_int as libc::c_uchar);
                    if !(qz != 0 && qz.wrapping_mul(2 as libc::c_int as libc::c_uint) < s) {
                        let mut n: libc::c_uint = (*codabar).character() as libc::c_uint;
                        if !(n
                            < (*codabar).configs[(ZBAR_CFG_MIN_LEN as libc::c_int
                                - ZBAR_CFG_MIN_LEN as libc::c_int)
                                as usize] as libc::c_uint
                            || (*codabar).configs[(ZBAR_CFG_MAX_LEN as libc::c_int
                                - ZBAR_CFG_MIN_LEN as libc::c_int)
                                as usize]
                                > 0 as libc::c_int
                                && n > (*codabar).configs[(ZBAR_CFG_MAX_LEN as libc::c_int
                                    - ZBAR_CFG_MIN_LEN as libc::c_int)
                                    as usize]
                                    as libc::c_uint)
                        {
                            if (*codabar).character() < 6 as libc::c_int
                                && acquire_lock(dcode, ZBAR_CODABAR) as libc::c_int != 0
                            {
                                (*codabar).set_character(-(1 as libc::c_int));
                                return ZBAR_PARTIAL;
                            }
                            let mut sym: zbar_symbol_type_t = codabar_postprocess(dcode);
                            if !(sym as libc::c_uint > ZBAR_PARTIAL as libc::c_int as libc::c_uint)
                            {
                                release_lock(dcode, ZBAR_CODABAR);
                                (*codabar).set_character(-(1 as libc::c_int))
                            }
                            return sym;
                        }
                    }
                } else if !((4 as libc::c_int as libc::c_uint)
                    .wrapping_mul(get_width(dcode, 0 as libc::c_int as libc::c_uchar))
                    > (3 as libc::c_int as libc::c_uint).wrapping_mul(s))
                {
                    return ZBAR_NONE;
                }
            }
        }
    }
    if (*codabar).character() >= 6 as libc::c_int {
        release_lock(dcode, ZBAR_CODABAR);
    }
    (*codabar).set_character(-(1 as libc::c_int));
    return ZBAR_NONE;
}
