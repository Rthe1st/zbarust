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
    fn _zbar_decoder_buf_dump(buf: *mut libc::c_uchar, buflen: libc::c_uint)
        -> *const libc::c_char;
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
pub type qr_finder_t = qr_finder_s;
/* QR Code symbol finder state */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qr_finder_s {
    pub s5: libc::c_uint,
    pub line: qr_finder_line,
    pub config: libc::c_uint,
}
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
pub type code128_decoder_t = code128_decoder_s;
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
/* scan direction: 0=fwd/space, 1=rev/bar */
/* element offset 0-5 */
/* character position in symbol */
/* start character */
/* character width */
/* last character width */
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
pub type code93_decoder_t = code93_decoder_s;
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
/* scan direction: 0=fwd/space, 1=rev/bar */
/* element offset 0-5 */
/* character position in symbol */
/* last character width */
/* first character */
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
pub type code39_decoder_t = code39_decoder_s;
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
/* scan direction: 0=fwd, 1=rev */
/* element offset 0-8 */
/* character position in symbol */
/* current character width */
/* last character width */
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
pub type codabar_decoder_t = codabar_decoder_s;
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
/* scan direction: 0=fwd, 1=rev */
/* element offset 0-7 */
/* character position in symbol */
/* current character width */
/* last character width */
/* initial scan buffer */
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
/* active DataBar (partial) segment entry */
/* finder pattern */
/* DataBar expanded finder */
/* finder coloring */
/* data character side of finder */
/* unpaired partial segment */
/* times encountered */
/* age, in characters scanned */
/* bar checksum */
/* decoded character data */
/* measured width of finder (14 modules) */
/* DataBar specific decode state */
pub type databar_decoder_t = databar_decoder_s;
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
/* decoder configuration flags */
/* allocated segments */
/* current scan */
/* active segment list */
/* outstanding character indices */
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
pub type i25_decoder_t = i25_decoder_s;
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
/* scan direction: 0=fwd/space, 1=rev/bar */
/* element offset 0-8 */
/* character position in symbol */
/* current character width */
/* last character width */
/* initial scan buffer */
/* int valued configurations */
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
pub const EAN_RIGHT: symbol_partial_e = 4096;
pub const EAN_LEFT: symbol_partial_e = 0;
pub type symbol_partial_e = libc::c_uint;
#[inline]
unsafe extern fn calc_s(
    mut dcode: *const zbar_decoder_t,
    mut offset: libc::c_uchar,
    mut n: libc::c_uchar,
) -> libc::c_uint {
    let mut s: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    loop {
        let fresh0 = n;
        n = n.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        let fresh1 = offset;
        offset = offset.wrapping_add(1);
        s = s.wrapping_add(get_width(dcode, fresh1))
    }
    return s;
}
#[inline]
unsafe extern fn get_color(mut dcode: *const zbar_decoder_t) -> libc::c_char {
    return ((*dcode).idx as libc::c_int & 1 as libc::c_int) as libc::c_char;
}
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
unsafe extern fn get_width(
    mut dcode: *const zbar_decoder_t,
    mut offset: libc::c_uchar,
) -> libc::c_uint {
    return (*dcode).w[((*dcode).idx as libc::c_int - offset as libc::c_int
        & 16 as libc::c_int - 1 as libc::c_int) as usize];
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
unsafe extern fn ean_get_config(
    mut ean: *mut ean_decoder_t,
    mut sym: zbar_symbol_type_t,
) -> libc::c_uint {
    match sym as libc::c_uint {
        2 => return (*ean).ean2_config,
        5 => return (*ean).ean5_config,
        8 => return (*ean).ean8_config,
        9 => return (*ean).upce_config,
        10 => return (*ean).isbn10_config,
        12 => return (*ean).upca_config,
        13 => return (*ean).ean13_config,
        14 => return (*ean).isbn13_config,
        _ => return 0 as libc::c_int as libc::c_uint,
    };
}
/* convert compact encoded D2E1E2 to character (bit4 is parity) */
static mut digits: [libc::c_uchar; 20] = [
    0x6 as libc::c_int as libc::c_uchar,
    0x10 as libc::c_int as libc::c_uchar,
    0x4 as libc::c_int as libc::c_uchar,
    0x13 as libc::c_int as libc::c_uchar,
    0x19 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0x11 as libc::c_int as libc::c_uchar,
    0x5 as libc::c_int as libc::c_uchar,
    0x9 as libc::c_int as libc::c_uchar,
    0x12 as libc::c_int as libc::c_uchar,
    0x7 as libc::c_int as libc::c_uchar,
    0x15 as libc::c_int as libc::c_uchar,
    0x16 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x14 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0x18 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0x17 as libc::c_int as libc::c_uchar,
];
static mut parity_decode: [libc::c_uchar; 32] = [
    0xf0 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xf as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x1f as libc::c_int as libc::c_uchar,
    0x2f as libc::c_int as libc::c_uchar,
    0xf3 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x4f as libc::c_int as libc::c_uchar,
    0x7f as libc::c_int as libc::c_uchar,
    0xf8 as libc::c_int as libc::c_uchar,
    0x5f as libc::c_int as libc::c_uchar,
    0xf9 as libc::c_int as libc::c_uchar,
    0xf6 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x6f as libc::c_int as libc::c_uchar,
    0x9f as libc::c_int as libc::c_uchar,
    0xf5 as libc::c_int as libc::c_uchar,
    0x8f as libc::c_int as libc::c_uchar,
    0xf7 as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x3f as libc::c_int as libc::c_uchar,
    0xf2 as libc::c_int as libc::c_uchar,
    0xf1 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xf as libc::c_int as libc::c_uchar,
];
#[inline]
unsafe extern fn check_width(mut w0: libc::c_uint, mut w1: libc::c_uint) -> libc::c_int {
    let mut dw0: libc::c_uint = w0;
    w0 = w0.wrapping_mul(8 as libc::c_int as libc::c_uint);
    w1 = w1.wrapping_mul(8 as libc::c_int as libc::c_uint);
    return (w0.wrapping_sub(dw0) <= w1 && w1 <= w0.wrapping_add(dw0)) as libc::c_int;
}
/* evaluate previous N (>= 2) widths as auxiliary pattern,
 * using preceding 4 as character width
 */
#[inline]
unsafe extern fn aux_end(mut dcode: *mut zbar_decoder_t, mut fwd: libc::c_uchar) -> libc::c_schar {
    let mut code: libc::c_schar = 0;
    let mut i: libc::c_schar = 0;
    /* reference width from previous character */
    let mut s: libc::c_uint = calc_s(
        dcode,
        (4 as libc::c_int + fwd as libc::c_int) as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
    );
    /* check quiet zone */
    let mut qz: libc::c_uint = get_width(dcode, 0 as libc::c_int as libc::c_uchar);
    if fwd == 0
        && qz != 0
        && qz
            <= s.wrapping_mul(3 as libc::c_int as libc::c_uint)
                .wrapping_div(4 as libc::c_int as libc::c_uint)
    {
        return -(1 as libc::c_int) as libc::c_schar;
    }
    code = 0 as libc::c_int as libc::c_schar;
    i = (1 as libc::c_int - fwd as libc::c_int) as libc::c_schar;
    while (i as libc::c_int) < 3 as libc::c_int + fwd as libc::c_int {
        let mut e: libc::c_uint = get_width(dcode, i as libc::c_uchar)
            .wrapping_add(get_width(dcode, (i as libc::c_int + 1 as libc::c_int) as libc::c_uchar));
        code = ((code as libc::c_int) << 2 as libc::c_int
            | decode_e(e, s, 7 as libc::c_int as libc::c_uint)) as libc::c_schar;
        if (code as libc::c_int) < 0 as libc::c_int {
            return -(1 as libc::c_int) as libc::c_schar;
        }
        i += 1
    }
    return code;
}
/* determine possible auxiliary pattern
 * using current 4 as possible character
 */
#[inline]
unsafe extern fn aux_start(mut dcode: *mut zbar_decoder_t) -> libc::c_schar {
    /* FIXME NB add-on has no guard in reverse */
    let mut e1: libc::c_uint = 0;
    let mut e2: libc::c_uint = get_width(dcode, 5 as libc::c_int as libc::c_uchar)
        .wrapping_add(get_width(dcode, 6 as libc::c_int as libc::c_uchar));
    let mut E1: libc::c_uchar = 0;
    if (*dcode).ean.s4 < 6 as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int) as libc::c_schar;
    }
    if decode_e(e2, (*dcode).ean.s4, 7 as libc::c_int as libc::c_uint) != 0 {
        return -(1 as libc::c_int) as libc::c_schar;
    }
    e1 = get_width(dcode, 4 as libc::c_int as libc::c_uchar)
        .wrapping_add(get_width(dcode, 5 as libc::c_int as libc::c_uchar));
    E1 = decode_e(e1, (*dcode).ean.s4, 7 as libc::c_int as libc::c_uint) as libc::c_uchar;
    if get_color(dcode) as libc::c_int == ZBAR_BAR as libc::c_int {
        /* check for quiet-zone */
        let mut qz: libc::c_uint = get_width(dcode, 7 as libc::c_int as libc::c_uchar);
        if qz == 0
            || qz
                > (*dcode)
                    .ean
                    .s4
                    .wrapping_mul(3 as libc::c_int as libc::c_uint)
                    .wrapping_div(4 as libc::c_int as libc::c_uint)
        {
            if E1 == 0 {
                return 0 as libc::c_int as libc::c_schar;
            /* normal symbol start */
            } else {
                if E1 as libc::c_int == 1 as libc::c_int {
                    return 0x40 as libc::c_int as libc::c_schar;
                    /* add-on symbol start */
                }
            }
        }
        return -(1 as libc::c_int) as libc::c_schar;
    }
    if E1 == 0 {
        /* attempting decode from SPACE => validate center guard */
        let mut e3: libc::c_uint = get_width(dcode, 6 as libc::c_int as libc::c_uchar)
            .wrapping_add(get_width(dcode, 7 as libc::c_int as libc::c_uchar));
        let mut e4: libc::c_uint = get_width(dcode, 7 as libc::c_int as libc::c_uchar)
            .wrapping_add(get_width(dcode, 8 as libc::c_int as libc::c_uchar));
        if decode_e(e3, (*dcode).ean.s4, 7 as libc::c_int as libc::c_uint) == 0
            && decode_e(e4, (*dcode).ean.s4, 7 as libc::c_int as libc::c_uint) == 0
        {
            return 0 as libc::c_int as libc::c_schar;
            /* start after center guard */
        }
    }
    return -(1 as libc::c_int) as libc::c_schar;
}
/* check addon delimiter using current 4 as character
 */
#[inline]
unsafe extern fn aux_mid(mut dcode: *mut zbar_decoder_t) -> libc::c_schar {
    let mut e: libc::c_uint = get_width(dcode, 4 as libc::c_int as libc::c_uchar)
        .wrapping_add(get_width(dcode, 5 as libc::c_int as libc::c_uchar));
    return decode_e(e, (*dcode).ean.s4, 7 as libc::c_int as libc::c_uint) as libc::c_schar;
}
/* attempt to decode previous 4 widths (2 bars and 2 spaces) as a character */
#[inline]
unsafe extern fn decode4(mut dcode: *mut zbar_decoder_t) -> libc::c_schar {
    let mut code: libc::c_schar = 0;
    /* calculate similar edge measurements */
    let mut e1: libc::c_uint = if get_color(dcode) as libc::c_int == ZBAR_BAR as libc::c_int {
        get_width(dcode, 0 as libc::c_int as libc::c_uchar)
            .wrapping_add(get_width(dcode, 1 as libc::c_int as libc::c_uchar))
    } else {
        get_width(dcode, 2 as libc::c_int as libc::c_uchar)
            .wrapping_add(get_width(dcode, 3 as libc::c_int as libc::c_uchar))
    };
    let mut e2: libc::c_uint = get_width(dcode, 1 as libc::c_int as libc::c_uchar)
        .wrapping_add(get_width(dcode, 2 as libc::c_int as libc::c_uchar));
    if (*dcode).ean.s4 < 6 as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int) as libc::c_schar;
    }
    /* create compacted encoding for direct lookup */
    code = (decode_e(e1, (*dcode).ean.s4, 7 as libc::c_int as libc::c_uint) << 2 as libc::c_int
        | decode_e(e2, (*dcode).ean.s4, 7 as libc::c_int as libc::c_uint))
        as libc::c_schar;
    if (code as libc::c_int) < 0 as libc::c_int {
        return -(1 as libc::c_int) as libc::c_schar;
    }
    /* 4 combinations require additional determinant (D2)
      E1E2 == 34 (0110)
      E1E2 == 43 (1001)
      E1E2 == 33 (0101)
      E1E2 == 44 (1010)
    */
    if (1 as libc::c_int) << code as libc::c_int & 0x660 as libc::c_int != 0 {
        let mut mid: libc::c_uchar = 0;
        let mut alt: libc::c_uchar = 0;
        /* use sum of bar widths */
        let mut d2: libc::c_uint = if get_color(dcode) as libc::c_int == ZBAR_BAR as libc::c_int {
            get_width(dcode, 0 as libc::c_int as libc::c_uchar)
                .wrapping_add(get_width(dcode, 2 as libc::c_int as libc::c_uchar))
        } else {
            get_width(dcode, 1 as libc::c_int as libc::c_uchar)
                .wrapping_add(get_width(dcode, 3 as libc::c_int as libc::c_uchar))
        }; /* E1E2 in 34,43 */
        d2 = d2.wrapping_mul(7 as libc::c_int as libc::c_uint); /* compress code space */
        mid = if (1 as libc::c_int) << code as libc::c_int & 0x420 as libc::c_int != 0 {
            3 as libc::c_int
        } else {
            4 as libc::c_int
        } as libc::c_uchar;
        alt = (d2 > (mid as libc::c_uint).wrapping_mul((*dcode).ean.s4)) as libc::c_int
            as libc::c_uchar;
        if alt != 0 {
            code = (code as libc::c_int >> 1 as libc::c_int & 3 as libc::c_int
                | 0x10 as libc::c_int) as libc::c_schar
        }
    }
    if !((code as libc::c_int) < 0x14 as libc::c_int) {
        fprintf(stderr,
                b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\tcode=%02x e1=%x e2=%x s4=%x color=%x\n\x00"
                    as *const u8 as *const libc::c_char,
                b"zbar/decoder/ean.c\x00" as *const u8 as *const libc::c_char,
                257 as libc::c_int,
                (*::std::mem::transmute::<&[u8; 8],
                                          &[libc::c_char; 8]>(b"decode4\x00")).as_ptr(),
                b"code < 0x14\x00" as *const u8 as *const libc::c_char,
                code as libc::c_int, e1, e2, (*dcode).ean.s4,
                get_color(dcode) as libc::c_int);
        return -(1 as libc::c_int) as libc::c_schar;
    }
    return code;
}
#[inline]
unsafe extern fn ean_part_end2(
    mut ean: *mut ean_decoder_t,
    mut pass: *mut ean_pass_t,
) -> libc::c_char {
    if (*ean).ean2_config >> ZBAR_CFG_ENABLE as libc::c_int & 1 as libc::c_int as libc::c_uint == 0
    {
        return ZBAR_NONE as libc::c_int as libc::c_char;
    }
    /* extract parity bits */
    let mut par: libc::c_uchar = (((*pass).raw[1 as libc::c_int as usize] as libc::c_int
        & 0x10 as libc::c_int)
        >> 3 as libc::c_int
        | ((*pass).raw[2 as libc::c_int as usize] as libc::c_int & 0x10 as libc::c_int)
            >> 4 as libc::c_int) as libc::c_uchar;
    /* calculate "checksum" */
    let mut chk: libc::c_uchar = (!(((*pass).raw[1 as libc::c_int as usize] as libc::c_int
        & 0xf as libc::c_int)
        * 10 as libc::c_int
        + ((*pass).raw[2 as libc::c_int as usize] as libc::c_int & 0xf as libc::c_int))
        & 0x3 as libc::c_int) as libc::c_uchar;
    if par as libc::c_int != chk as libc::c_int {
        return ZBAR_NONE as libc::c_int as libc::c_char;
    }
    return ZBAR_EAN2 as libc::c_int as libc::c_char;
}
#[inline]
unsafe extern fn ean_part_end4(
    mut pass: *mut ean_pass_t,
    mut fwd: libc::c_uchar,
) -> zbar_symbol_type_t {
    /* extract parity bits */
    let mut par: libc::c_uchar = (((*pass).raw[1 as libc::c_int as usize] as libc::c_int
        & 0x10 as libc::c_int)
        >> 1 as libc::c_int
        | ((*pass).raw[2 as libc::c_int as usize] as libc::c_int & 0x10 as libc::c_int)
            >> 2 as libc::c_int
        | ((*pass).raw[3 as libc::c_int as usize] as libc::c_int & 0x10 as libc::c_int)
            >> 3 as libc::c_int
        | ((*pass).raw[4 as libc::c_int as usize] as libc::c_int & 0x10 as libc::c_int)
            >> 4 as libc::c_int) as libc::c_uchar;
    if par as libc::c_int != 0 && par as libc::c_int != 0xf as libc::c_int {
        /* invalid parity combination */
        return ZBAR_NONE;
    }
    if (par == 0) as libc::c_int == fwd as libc::c_int {
        /* reverse sampled digits */
        let mut tmp: libc::c_uchar = (*pass).raw[1 as libc::c_int as usize];
        (*pass).state = ((*pass).state as libc::c_int | 0x80 as libc::c_int) as libc::c_schar;
        (*pass).raw[1 as libc::c_int as usize] = (*pass).raw[4 as libc::c_int as usize];
        (*pass).raw[4 as libc::c_int as usize] = tmp;
        tmp = (*pass).raw[2 as libc::c_int as usize];
        (*pass).raw[2 as libc::c_int as usize] = (*pass).raw[3 as libc::c_int as usize];
        (*pass).raw[3 as libc::c_int as usize] = tmp
    }
    if par == 0 {
        return (ZBAR_EAN8 as libc::c_int | EAN_RIGHT as libc::c_int) as zbar_symbol_type_t;
    }
    return (ZBAR_EAN8 as libc::c_int | EAN_LEFT as libc::c_int) as zbar_symbol_type_t;
}
#[inline]
unsafe extern fn ean_part_end5(
    mut ean: *mut ean_decoder_t,
    mut pass: *mut ean_pass_t,
) -> libc::c_char {
    if (*ean).ean5_config >> ZBAR_CFG_ENABLE as libc::c_int & 1 as libc::c_int as libc::c_uint == 0
    {
        return ZBAR_NONE as libc::c_int as libc::c_char;
    }
    /* extract parity bits */
    let mut par: libc::c_uchar = ((*pass).raw[1 as libc::c_int as usize] as libc::c_int
        & 0x10 as libc::c_int
        | ((*pass).raw[2 as libc::c_int as usize] as libc::c_int & 0x10 as libc::c_int)
            >> 1 as libc::c_int
        | ((*pass).raw[3 as libc::c_int as usize] as libc::c_int & 0x10 as libc::c_int)
            >> 2 as libc::c_int
        | ((*pass).raw[4 as libc::c_int as usize] as libc::c_int & 0x10 as libc::c_int)
            >> 3 as libc::c_int
        | ((*pass).raw[5 as libc::c_int as usize] as libc::c_int & 0x10 as libc::c_int)
            >> 4 as libc::c_int) as libc::c_uchar;
    /* calculate checksum */
    let mut chk: libc::c_uchar = ((((*pass).raw[1 as libc::c_int as usize] as libc::c_int
        & 0xf as libc::c_int)
        + ((*pass).raw[2 as libc::c_int as usize] as libc::c_int & 0xf as libc::c_int)
            * 3 as libc::c_int
        + ((*pass).raw[3 as libc::c_int as usize] as libc::c_int & 0xf as libc::c_int)
        + ((*pass).raw[4 as libc::c_int as usize] as libc::c_int & 0xf as libc::c_int)
            * 3 as libc::c_int
        + ((*pass).raw[5 as libc::c_int as usize] as libc::c_int & 0xf as libc::c_int))
        * 3 as libc::c_int
        % 10 as libc::c_int) as libc::c_uchar;
    let mut parchk: libc::c_uchar =
        parity_decode[(par as libc::c_int >> 1 as libc::c_int) as usize];
    if par as libc::c_int & 1 as libc::c_int != 0 {
        parchk = (parchk as libc::c_int >> 4 as libc::c_int) as libc::c_uchar
    }
    parchk = (parchk as libc::c_int & 0xf as libc::c_int) as libc::c_uchar;
    if parchk as libc::c_int != chk as libc::c_int {
        return ZBAR_NONE as libc::c_int as libc::c_char;
    }
    return ZBAR_EAN5 as libc::c_int as libc::c_char;
}
#[inline]
unsafe extern fn ean_part_end7(
    mut ean: *mut ean_decoder_t,
    mut pass: *mut ean_pass_t,
    mut fwd: libc::c_uchar,
) -> zbar_symbol_type_t {
    /* calculate parity index */
    let mut par: libc::c_uchar = if fwd as libc::c_int != 0 {
        (((*pass).raw[1 as libc::c_int as usize] as libc::c_int & 0x10 as libc::c_int)
            << 1 as libc::c_int
            | (*pass).raw[2 as libc::c_int as usize] as libc::c_int & 0x10 as libc::c_int
            | ((*pass).raw[3 as libc::c_int as usize] as libc::c_int & 0x10 as libc::c_int)
                >> 1 as libc::c_int
            | ((*pass).raw[4 as libc::c_int as usize] as libc::c_int & 0x10 as libc::c_int)
                >> 2 as libc::c_int
            | ((*pass).raw[5 as libc::c_int as usize] as libc::c_int & 0x10 as libc::c_int)
                >> 3 as libc::c_int)
            | ((*pass).raw[6 as libc::c_int as usize] as libc::c_int & 0x10 as libc::c_int)
                >> 4 as libc::c_int
    } else {
        (((*pass).raw[1 as libc::c_int as usize] as libc::c_int & 0x10 as libc::c_int)
            >> 4 as libc::c_int
            | ((*pass).raw[2 as libc::c_int as usize] as libc::c_int & 0x10 as libc::c_int)
                >> 3 as libc::c_int
            | ((*pass).raw[3 as libc::c_int as usize] as libc::c_int & 0x10 as libc::c_int)
                >> 2 as libc::c_int
            | ((*pass).raw[4 as libc::c_int as usize] as libc::c_int & 0x10 as libc::c_int)
                >> 1 as libc::c_int
            | (*pass).raw[5 as libc::c_int as usize] as libc::c_int & 0x10 as libc::c_int)
            | ((*pass).raw[6 as libc::c_int as usize] as libc::c_int & 0x10 as libc::c_int)
                << 1 as libc::c_int
    } as libc::c_uchar;
    /* lookup parity combination */
    (*pass).raw[0 as libc::c_int as usize] =
        parity_decode[(par as libc::c_int >> 1 as libc::c_int) as usize];
    if par as libc::c_int & 1 as libc::c_int != 0 {
        (*pass).raw[0 as libc::c_int as usize] = ((*pass).raw[0 as libc::c_int as usize]
            as libc::c_int
            >> 4 as libc::c_int) as libc::c_uchar
    }
    (*pass).raw[0 as libc::c_int as usize] = ((*pass).raw[0 as libc::c_int as usize] as libc::c_int
        & 0xf as libc::c_int) as libc::c_uchar;
    if (*pass).raw[0 as libc::c_int as usize] as libc::c_int == 0xf as libc::c_int {
        /* invalid parity combination */
        return ZBAR_NONE;
    }
    if (par == 0) as libc::c_int == fwd as libc::c_int {
        let mut i: libc::c_uchar = 0;
        (*pass).state = ((*pass).state as libc::c_int | 0x80 as libc::c_int) as libc::c_schar;
        /* reverse sampled digits */
        i = 1 as libc::c_int as libc::c_uchar;
        while (i as libc::c_int) < 4 as libc::c_int {
            let mut tmp: libc::c_uchar = (*pass).raw[i as usize];
            (*pass).raw[i as usize] = (*pass).raw[(7 as libc::c_int - i as libc::c_int) as usize];
            (*pass).raw[(7 as libc::c_int - i as libc::c_int) as usize] = tmp;
            i = i.wrapping_add(1)
        }
    }
    if (*ean).ean13_config >> ZBAR_CFG_ENABLE as libc::c_int & 1 as libc::c_int as libc::c_uint != 0
    {
        if par == 0 {
            return (ZBAR_EAN13 as libc::c_int | EAN_RIGHT as libc::c_int) as zbar_symbol_type_t;
        }
        if par as libc::c_int & 0x20 as libc::c_int != 0 {
            return (ZBAR_EAN13 as libc::c_int | EAN_LEFT as libc::c_int) as zbar_symbol_type_t;
        }
    }
    if par as libc::c_int != 0 && par as libc::c_int & 0x20 as libc::c_int == 0 {
        return ZBAR_UPCE;
    }
    return ZBAR_NONE;
}
/* update state for one of 4 parallel passes */
#[inline]
unsafe extern fn decode_pass(
    mut dcode: *mut zbar_decoder_t,
    mut pass: *mut ean_pass_t,
) -> zbar_symbol_type_t {
    let mut idx: libc::c_uchar = 0;
    let mut fwd: libc::c_uchar = 0;
    (*pass).state += 1;
    idx = ((*pass).state as libc::c_int & 0x3f as libc::c_int) as libc::c_uchar;
    fwd = ((*pass).state as libc::c_int & 1 as libc::c_int) as libc::c_uchar;
    if get_color(dcode) as libc::c_int == ZBAR_SPACE as libc::c_int {
        if (*pass).state as libc::c_int & 0x40 as libc::c_int != 0 {
            if idx as libc::c_int == 0x9 as libc::c_int || idx as libc::c_int == 0x21 as libc::c_int
            {
                let mut qz: libc::c_uint = get_width(dcode, 0 as libc::c_int as libc::c_uchar);
                let mut s: libc::c_uint = calc_s(
                    dcode,
                    1 as libc::c_int as libc::c_uchar,
                    4 as libc::c_int as libc::c_uchar,
                );
                let mut part: zbar_symbol_type_t = (qz == 0
                    || qz
                        >= s.wrapping_mul(3 as libc::c_int as libc::c_uint)
                            .wrapping_div(4 as libc::c_int as libc::c_uint))
                    as libc::c_int
                    as zbar_symbol_type_t;
                if part as libc::c_uint != 0 && idx as libc::c_int == 0x9 as libc::c_int {
                    part = ean_part_end2(&mut (*dcode).ean, pass) as zbar_symbol_type_t
                } else if part as u64 != 0 {
                    part = ean_part_end5(&mut (*dcode).ean, pass) as zbar_symbol_type_t
                }
                if part as libc::c_uint != 0 || idx as libc::c_int == 0x21 as libc::c_int {
                    (*dcode).ean.direction = 0 as libc::c_int;
                    (*pass).state = -(1 as libc::c_int) as libc::c_schar;
                    return part;
                }
            }
            if idx as libc::c_int & 7 as libc::c_int == 1 as libc::c_int {
                (*pass).state = ((*pass).state as libc::c_int + 2 as libc::c_int) as libc::c_schar;
                idx = (idx as libc::c_int + 2 as libc::c_int) as libc::c_uchar
            }
        } else if (idx as libc::c_int == 0x10 as libc::c_int
            || idx as libc::c_int == 0x11 as libc::c_int)
            && (*dcode).ean.ean8_config >> ZBAR_CFG_ENABLE as libc::c_int
                & 1 as libc::c_int as libc::c_uint
                != 0
            && aux_end(dcode, fwd) == 0
        {
            let mut part_0: zbar_symbol_type_t = ean_part_end4(pass, fwd);
            if part_0 as u64 != 0 {
                (*dcode).ean.direction = ((*pass).state as libc::c_int & 0x80 as libc::c_int
                    != 0 as libc::c_int) as libc::c_int
            }
            (*pass).state = -(1 as libc::c_int) as libc::c_schar;
            return part_0;
        } else {
            if idx as libc::c_int == 0x18 as libc::c_int
                || idx as libc::c_int == 0x19 as libc::c_int
            {
                let mut part_1: zbar_symbol_type_t = ZBAR_NONE;
                if aux_end(dcode, fwd) == 0
                    && (*pass).raw[5 as libc::c_int as usize] as libc::c_int != 0xff as libc::c_int
                {
                    part_1 = ean_part_end7(&mut (*dcode).ean, pass, fwd)
                }
                if part_1 as u64 != 0 {
                    (*dcode).ean.direction = ((*pass).state as libc::c_int & 0x80 as libc::c_int
                        != 0 as libc::c_int)
                        as libc::c_int
                }
                (*pass).state = -(1 as libc::c_int) as libc::c_schar;
                return part_1;
            }
        }
    }
    if (*pass).state as libc::c_int & 0x40 as libc::c_int != 0 {
        idx = (idx as libc::c_int >> 1 as libc::c_int) as libc::c_uchar
    }
    if idx as libc::c_int & 0x3 as libc::c_int == 0 && idx as libc::c_int <= 0x14 as libc::c_int {
        let mut code: libc::c_schar = -(1 as libc::c_int) as libc::c_schar;
        let mut w: libc::c_uint = (*pass).width;
        if (*dcode).ean.s4 == 0 {
            return ZBAR_NONE;
        }
        /* validate guard bars before decoding first char of symbol */
        if (*pass).state == 0 {
            (*pass).state = aux_start(dcode);
            (*pass).width = (*dcode).ean.s4;
            if ((*pass).state as libc::c_int) < 0 as libc::c_int {
                return ZBAR_NONE;
            }
            idx = ((*pass).state as libc::c_int & 0x3f as libc::c_int) as libc::c_uchar
        } else {
            w = check_width(w, (*dcode).ean.s4) as libc::c_uint;
            if w != 0 {
                (*pass).width = (*pass)
                    .width
                    .wrapping_add((*dcode).ean.s4.wrapping_mul(3 as libc::c_int as libc::c_uint))
                    .wrapping_div(4 as libc::c_int as libc::c_uint)
            }
        }
        if w != 0 {
            code = decode4(dcode)
        }
        if (code as libc::c_int) < 0 as libc::c_int && idx as libc::c_int != 0x10 as libc::c_int
            || idx as libc::c_int > 0 as libc::c_int
                && (*pass).state as libc::c_int & 0x40 as libc::c_int != 0
                && aux_mid(dcode) as libc::c_int != 0
        {
            (*pass).state = -(1 as libc::c_int) as libc::c_schar
        } else if (code as libc::c_int) < 0 as libc::c_int {
            (*pass).raw[5 as libc::c_int as usize] = 0xff as libc::c_int as libc::c_uchar
        } else {
            (*pass).raw[((idx as libc::c_int >> 2 as libc::c_int) + 1 as libc::c_int) as usize] =
                digits[code as libc::c_uchar as usize]
        }
    }
    return ZBAR_NONE;
}
#[inline]
unsafe extern fn ean_verify_checksum(
    mut ean: *mut ean_decoder_t,
    mut n: libc::c_int,
) -> libc::c_schar {
    let mut chk: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    let mut i: libc::c_uchar = 0;
    let mut d: libc::c_uchar = 0;
    i = 0 as libc::c_int as libc::c_uchar;
    while (i as libc::c_int) < n {
        let mut d_0: libc::c_uchar = (*ean).buf[i as usize] as libc::c_uchar;
        if !((d_0 as libc::c_int) < 10 as libc::c_int) {
            fprintf(
                stderr,
                b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\ti=%x d=%x chk=%x %s\n\x00"
                    as *const u8 as *const libc::c_char,
                b"zbar/decoder/ean.c\x00" as *const u8 as *const libc::c_char,
                522 as libc::c_int,
                (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(
                    b"ean_verify_checksum\x00",
                ))
                .as_ptr(),
                b"d < 10\x00" as *const u8 as *const libc::c_char,
                i as libc::c_int,
                d_0 as libc::c_int,
                chk as libc::c_int,
                _zbar_decoder_buf_dump(
                    (*ean).buf.as_mut_ptr() as *mut libc::c_void as *mut libc::c_uchar,
                    18 as libc::c_int as libc::c_uint,
                ),
            );
            return -(1 as libc::c_int) as libc::c_schar;
        }
        chk = (chk as libc::c_int + d_0 as libc::c_int) as libc::c_uchar;
        if (i as libc::c_int ^ n) & 1 as libc::c_int != 0 {
            chk =
                (chk as libc::c_int + ((d_0 as libc::c_int) << 1 as libc::c_int)) as libc::c_uchar;
            if chk as libc::c_int >= 20 as libc::c_int {
                chk = (chk as libc::c_int - 20 as libc::c_int) as libc::c_uchar
            }
        }
        if chk as libc::c_int >= 10 as libc::c_int {
            chk = (chk as libc::c_int - 10 as libc::c_int) as libc::c_uchar
        }
        i = i.wrapping_add(1)
    }
    if !((chk as libc::c_int) < 10 as libc::c_int) {
        fprintf(
            stderr,
            b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\tchk=%x n=%x %s\x00" as *const u8
                as *const libc::c_char,
            b"zbar/decoder/ean.c\x00" as *const u8 as *const libc::c_char,
            533 as libc::c_int,
            (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"ean_verify_checksum\x00"))
                .as_ptr(),
            b"chk < 10\x00" as *const u8 as *const libc::c_char,
            chk as libc::c_int,
            n,
            _zbar_decoder_buf_dump(
                (*ean).buf.as_mut_ptr() as *mut libc::c_void as *mut libc::c_uchar,
                18 as libc::c_int as libc::c_uint,
            ),
        );
        return -(1 as libc::c_int) as libc::c_schar;
    }
    if chk != 0 {
        chk = (10 as libc::c_int - chk as libc::c_int) as libc::c_uchar
    }
    d = (*ean).buf[n as usize] as libc::c_uchar;
    if !((d as libc::c_int) < 10 as libc::c_int) {
        fprintf(
            stderr,
            b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\tn=%x d=%x chk=%x %s\n\x00"
                as *const u8 as *const libc::c_char,
            b"zbar/decoder/ean.c\x00" as *const u8 as *const libc::c_char,
            538 as libc::c_int,
            (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"ean_verify_checksum\x00"))
                .as_ptr(),
            b"d < 10\x00" as *const u8 as *const libc::c_char,
            n,
            d as libc::c_int,
            chk as libc::c_int,
            _zbar_decoder_buf_dump(
                (*ean).buf.as_mut_ptr() as *mut libc::c_void as *mut libc::c_uchar,
                18 as libc::c_int as libc::c_uint,
            ),
        );
        return -(1 as libc::c_int) as libc::c_schar;
    }
    if chk as libc::c_int != d as libc::c_int {
        return -(1 as libc::c_int) as libc::c_schar;
    }
    return 0 as libc::c_int as libc::c_schar;
}
#[inline]
unsafe extern fn isbn10_calc_checksum(mut ean: *mut ean_decoder_t) -> libc::c_uchar {
    let mut chk: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut w: libc::c_uchar = 0;
    w = 10 as libc::c_int as libc::c_uchar;
    while w as libc::c_int > 1 as libc::c_int {
        let mut d: libc::c_uchar =
            (*ean).buf[(13 as libc::c_int - w as libc::c_int) as usize] as libc::c_uchar;
        if !((d as libc::c_int) < 10 as libc::c_int) {
            fprintf(
                stderr,
                b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\tw=%x d=%x chk=%x %s\n\x00"
                    as *const u8 as *const libc::c_char,
                b"zbar/decoder/ean.c\x00" as *const u8 as *const libc::c_char,
                554 as libc::c_int,
                (*::std::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(
                    b"isbn10_calc_checksum\x00",
                ))
                .as_ptr(),
                b"d < 10\x00" as *const u8 as *const libc::c_char,
                w as libc::c_int,
                d as libc::c_int,
                chk,
                _zbar_decoder_buf_dump(
                    (*ean).buf.as_mut_ptr() as *mut libc::c_void as *mut libc::c_uchar,
                    18 as libc::c_int as libc::c_uint,
                ),
            );
            return '?' as i32 as libc::c_uchar;
        }
        chk = chk.wrapping_add((d as libc::c_int * w as libc::c_int) as libc::c_uint);
        w = w.wrapping_sub(1)
    }
    chk = chk.wrapping_rem(11 as libc::c_int as libc::c_uint);
    if chk == 0 {
        return '0' as i32 as libc::c_uchar;
    }
    chk = (11 as libc::c_int as libc::c_uint).wrapping_sub(chk);
    if chk < 10 as libc::c_int as libc::c_uint {
        return chk.wrapping_add('0' as i32 as libc::c_uint) as libc::c_uchar;
    }
    return 'X' as i32 as libc::c_uchar;
}
#[inline]
unsafe extern fn ean_expand_upce(mut ean: *mut ean_decoder_t, mut pass: *mut ean_pass_t) {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut decode: libc::c_uchar = 0;
    /* parity encoded digit is checksum */
    let fresh2 = i;
    i = i + 1;
    (*ean).buf[12 as libc::c_int as usize] = (*pass).raw[fresh2 as usize] as libc::c_schar;
    decode = ((*pass).raw[6 as libc::c_int as usize] as libc::c_int & 0xf as libc::c_int)
        as libc::c_uchar;
    (*ean).buf[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_schar;
    (*ean).buf[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_schar;
    let fresh3 = i;
    i = i + 1;
    (*ean).buf[2 as libc::c_int as usize] =
        ((*pass).raw[fresh3 as usize] as libc::c_int & 0xf as libc::c_int) as libc::c_schar;
    let fresh4 = i;
    i = i + 1;
    (*ean).buf[3 as libc::c_int as usize] =
        ((*pass).raw[fresh4 as usize] as libc::c_int & 0xf as libc::c_int) as libc::c_schar;
    (*ean).buf[4 as libc::c_int as usize] = if (decode as libc::c_int) < 3 as libc::c_int {
        decode as libc::c_int
    } else {
        let fresh5 = i;
        i = i + 1;
        ((*pass).raw[fresh5 as usize] as libc::c_int) & 0xf as libc::c_int
    } as libc::c_schar;
    (*ean).buf[5 as libc::c_int as usize] = if (decode as libc::c_int) < 4 as libc::c_int {
        0 as libc::c_int
    } else {
        let fresh6 = i;
        i = i + 1;
        ((*pass).raw[fresh6 as usize] as libc::c_int) & 0xf as libc::c_int
    } as libc::c_schar;
    (*ean).buf[6 as libc::c_int as usize] = if (decode as libc::c_int) < 5 as libc::c_int {
        0 as libc::c_int
    } else {
        let fresh7 = i;
        i = i + 1;
        ((*pass).raw[fresh7 as usize] as libc::c_int) & 0xf as libc::c_int
    } as libc::c_schar;
    (*ean).buf[7 as libc::c_int as usize] = 0 as libc::c_int as libc::c_schar;
    (*ean).buf[8 as libc::c_int as usize] = 0 as libc::c_int as libc::c_schar;
    (*ean).buf[9 as libc::c_int as usize] = if (decode as libc::c_int) < 3 as libc::c_int {
        let fresh8 = i;
        i = i + 1;
        ((*pass).raw[fresh8 as usize] as libc::c_int) & 0xf as libc::c_int
    } else {
        0 as libc::c_int
    } as libc::c_schar;
    (*ean).buf[10 as libc::c_int as usize] = if (decode as libc::c_int) < 4 as libc::c_int {
        let fresh9 = i;
        i = i + 1;
        ((*pass).raw[fresh9 as usize] as libc::c_int) & 0xf as libc::c_int
    } else {
        0 as libc::c_int
    } as libc::c_schar;
    (*ean).buf[11 as libc::c_int as usize] = if (decode as libc::c_int) < 5 as libc::c_int {
        ((*pass).raw[i as usize] as libc::c_int) & 0xf as libc::c_int
    } else {
        decode as libc::c_int
    } as libc::c_schar;
}
#[inline]
unsafe extern fn integrate_partial(
    mut ean: *mut ean_decoder_t,
    mut pass: *mut ean_pass_t,
    mut part: zbar_symbol_type_t,
) -> zbar_symbol_type_t {
    /* copy raw data into holding buffer */
    /* if same partial is not consistent, reset others */
    let mut i: libc::c_schar = 0;
    let mut j: libc::c_schar = 0;
    if (*ean).left as libc::c_uint != 0
        && part as libc::c_uint & ZBAR_SYMBOL as libc::c_int as libc::c_uint
            != (*ean).left as libc::c_uint
        || (*ean).right as libc::c_uint != 0
            && part as libc::c_uint & ZBAR_SYMBOL as libc::c_int as libc::c_uint
                != (*ean).right as libc::c_uint
    {
        /* partial mismatch - reset collected parts */
        (*ean).right = ZBAR_NONE;
        (*ean).left = (*ean).right
    }
    if ((*ean).left as libc::c_uint != 0 || (*ean).right as libc::c_uint != 0)
        && check_width((*ean).width, (*pass).width) == 0
    {
        (*ean).right = ZBAR_NONE;
        (*ean).left = (*ean).right
    }
    if part as libc::c_uint & EAN_RIGHT as libc::c_int as libc::c_uint != 0 {
        part = ::std::mem::transmute::<libc::c_uint, zbar_symbol_type_t>(
            part as libc::c_uint & ZBAR_SYMBOL as libc::c_int as libc::c_uint,
        );
        j = (part as libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_schar;
        i = (part as libc::c_uint >> 1 as libc::c_int) as libc::c_schar;
        while i != 0 {
            let mut digit: libc::c_uchar =
                ((*pass).raw[i as usize] as libc::c_int & 0xf as libc::c_int) as libc::c_uchar;
            if (*ean).right as libc::c_uint != 0
                && (*ean).buf[j as usize] as libc::c_int != digit as libc::c_int
            {
                /* FIXME!? */
                /* partial mismatch - reset collected parts */
                (*ean).right = ZBAR_NONE;
                (*ean).left = (*ean).right
            }
            (*ean).buf[j as usize] = digit as libc::c_schar;
            i -= 1;
            j -= 1
        }
        (*ean).right = part;
        part = ::std::mem::transmute::<libc::c_uint, zbar_symbol_type_t>(
            part as libc::c_uint & (*ean).left as libc::c_uint,
        )
    } else if part as libc::c_uint == ZBAR_EAN13 as libc::c_int as libc::c_uint
        || part as libc::c_uint == ZBAR_EAN8 as libc::c_int as libc::c_uint
    {
        /* EAN_LEFT */
        j = ((part as libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint)
            >> 1 as libc::c_int) as libc::c_schar;
        i = (part as libc::c_uint >> 1 as libc::c_int) as libc::c_schar;
        while j as libc::c_int >= 0 as libc::c_int {
            let mut digit_0: libc::c_uchar =
                ((*pass).raw[i as usize] as libc::c_int & 0xf as libc::c_int) as libc::c_uchar;
            if (*ean).left as libc::c_uint != 0
                && (*ean).buf[j as usize] as libc::c_int != digit_0 as libc::c_int
            {
                /* FIXME!? */
                /* partial mismatch - reset collected parts */
                (*ean).right = ZBAR_NONE;
                (*ean).left = (*ean).right
            }
            (*ean).buf[j as usize] = digit_0 as libc::c_schar;
            i -= 1;
            j -= 1
        }
        (*ean).left = part;
        part = ::std::mem::transmute::<libc::c_uint, zbar_symbol_type_t>(
            part as libc::c_uint & (*ean).right as libc::c_uint,
        )
    } else if part as libc::c_uint != ZBAR_UPCE as libc::c_int as libc::c_uint {
        /* add-ons */
        i = part as libc::c_schar;
        while i as libc::c_int > 0 as libc::c_int {
            (*ean).buf[(i as libc::c_int - 1 as libc::c_int) as usize] =
                ((*pass).raw[i as usize] as libc::c_int & 0xf as libc::c_int) as libc::c_schar;
            i -= 1
        }
        (*ean).left = part
    } else {
        ean_expand_upce(ean, pass);
    }
    (*ean).width = (*pass).width;
    if part as u64 == 0 {
        part = ZBAR_PARTIAL
    }
    if (part as libc::c_uint == ZBAR_EAN13 as libc::c_int as libc::c_uint
        || part as libc::c_uint == ZBAR_UPCE as libc::c_int as libc::c_uint)
        && ean_verify_checksum(ean, 12 as libc::c_int) as libc::c_int != 0
        || part as libc::c_uint == ZBAR_EAN8 as libc::c_int as libc::c_uint
            && ean_verify_checksum(ean, 7 as libc::c_int) as libc::c_int != 0
    {
        /* invalid checksum */
        if (*ean).right as u64 != 0 {
            (*ean).left = ZBAR_NONE
        } else {
            (*ean).right = ZBAR_NONE
        }
        part = ZBAR_NONE
    }
    if part as libc::c_uint == ZBAR_EAN13 as libc::c_int as libc::c_uint {
        /* special case EAN-13 subsets */
        if (*ean).buf[0 as libc::c_int as usize] == 0
            && (*ean).upca_config >> ZBAR_CFG_ENABLE as libc::c_int
                & 1 as libc::c_int as libc::c_uint
                != 0
        {
            part = ZBAR_UPCA
        } else if (*ean).buf[0 as libc::c_int as usize] as libc::c_int == 9 as libc::c_int
            && (*ean).buf[1 as libc::c_int as usize] as libc::c_int == 7 as libc::c_int
        {
            /* ISBN-10 has priority over ISBN-13(?) */
            if (*ean).buf[2 as libc::c_int as usize] as libc::c_int == 8 as libc::c_int
                && (*ean).isbn10_config >> ZBAR_CFG_ENABLE as libc::c_int
                    & 1 as libc::c_int as libc::c_uint
                    != 0
            {
                part = ZBAR_ISBN10
            } else if ((*ean).buf[2 as libc::c_int as usize] as libc::c_int == 8 as libc::c_int
                || (*ean).buf[2 as libc::c_int as usize] as libc::c_int == 9 as libc::c_int)
                && (*ean).isbn13_config >> ZBAR_CFG_ENABLE as libc::c_int
                    & 1 as libc::c_int as libc::c_uint
                    != 0
            {
                part = ZBAR_ISBN13
            }
        }
    } else if part as libc::c_uint == ZBAR_UPCE as libc::c_int as libc::c_uint {
        if (*ean).upce_config >> ZBAR_CFG_ENABLE as libc::c_int & 1 as libc::c_int as libc::c_uint
            != 0
        {
            /* UPC-E was decompressed for checksum verification,
             * but user requested compressed result
             */
            (*ean).buf[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_schar;
            (*ean).buf[0 as libc::c_int as usize] = (*ean).buf[1 as libc::c_int as usize];
            i = 2 as libc::c_int as libc::c_schar;
            while (i as libc::c_int) < 8 as libc::c_int {
                (*ean).buf[i as usize] =
                    ((*pass).raw[(i as libc::c_int - 1 as libc::c_int) as usize] as libc::c_int
                        & 0xf as libc::c_int) as libc::c_schar;
                i += 1
            }
            (*ean).buf[i as usize] = ((*pass).raw[0 as libc::c_int as usize] as libc::c_int
                & 0xf as libc::c_int) as libc::c_schar
        } else if (*ean).upca_config >> ZBAR_CFG_ENABLE as libc::c_int
            & 1 as libc::c_int as libc::c_uint
            != 0
        {
            /* UPC-E reported as UPC-A has priority over EAN-13 */
            part = ZBAR_UPCA
        } else if (*ean).ean13_config >> ZBAR_CFG_ENABLE as libc::c_int
            & 1 as libc::c_int as libc::c_uint
            != 0
        {
            part = ZBAR_EAN13
        } else {
            part = ZBAR_NONE
        }
    }
    return part;
}
/* copy result to output buffer */
#[inline]
unsafe extern fn postprocess(mut dcode: *mut zbar_decoder_t, mut sym: zbar_symbol_type_t) {
    let mut ean: *mut ean_decoder_t = &mut (*dcode).ean;
    let mut base: zbar_symbol_type_t = sym;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0 as libc::c_int;
    if base as libc::c_uint > ZBAR_PARTIAL as libc::c_int as libc::c_uint {
        if base as libc::c_uint == ZBAR_UPCA as libc::c_int as libc::c_uint {
            i = 1 as libc::c_int
        } else if base as libc::c_uint == ZBAR_UPCE as libc::c_int as libc::c_uint {
            i = 1 as libc::c_int;
            base -= 1
        } else if base as libc::c_uint == ZBAR_ISBN13 as libc::c_int as libc::c_uint {
            base = ZBAR_EAN13
        } else if base as libc::c_uint == ZBAR_ISBN10 as libc::c_int as libc::c_uint {
            i = 3 as libc::c_int
        }
        if base as libc::c_uint == ZBAR_ISBN10 as libc::c_int as libc::c_uint
            || base as libc::c_uint > ZBAR_EAN5 as libc::c_int as libc::c_uint
                && ean_get_config(ean, sym) >> ZBAR_CFG_EMIT_CHECK as libc::c_int
                    & 1 as libc::c_int as libc::c_uint
                    == 0
        {
            base -= 1
        }
        while (j as libc::c_uint) < base as libc::c_uint
            && (*ean).buf[i as usize] as libc::c_int >= 0 as libc::c_int
        {
            *(*dcode).buf.offset(j as isize) =
                ((*ean).buf[i as usize] as libc::c_int + '0' as i32) as libc::c_uchar;
            i += 1;
            j += 1
        }
        if sym as libc::c_uint == ZBAR_ISBN10 as libc::c_int as libc::c_uint
            && j == 9 as libc::c_int
            && (*ean).isbn10_config >> ZBAR_CFG_EMIT_CHECK as libc::c_int
                & 1 as libc::c_int as libc::c_uint
                != 0
        {
            /* recalculate ISBN-10 check digit */
            let fresh10 = j;
            j = j + 1;
            *(*dcode).buf.offset(fresh10 as isize) = isbn10_calc_checksum(ean)
        }
    }
    (*dcode).buflen = j as libc::c_uint;
    *(*dcode).buf.offset(j as isize) = '\u{0}' as i32 as libc::c_uchar;
    (*dcode).direction = 1 as libc::c_int - 2 as libc::c_int * (*ean).direction;
    (*dcode).modifiers = 0 as libc::c_int as libc::c_uint;
}
/* reset EAN/UPC pass specific state */
/* reset all EAN/UPC state */
/* decode EAN/UPC symbols */
#[no_mangle]
pub unsafe extern fn _zbar_decode_ean(mut dcode: *mut zbar_decoder_t) -> zbar_symbol_type_t {
    /* process upto 4 separate passes */
    let mut sym: zbar_symbol_type_t = ZBAR_NONE;
    let mut pass_idx: libc::c_uchar =
        ((*dcode).idx as libc::c_int & 3 as libc::c_int) as libc::c_uchar;
    let mut i: libc::c_uchar = 0;
    /* update latest character width */
    (*dcode).ean.s4 =
        (*dcode).ean.s4.wrapping_sub(get_width(dcode, 4 as libc::c_int as libc::c_uchar));
    (*dcode).ean.s4 =
        (*dcode).ean.s4.wrapping_add(get_width(dcode, 0 as libc::c_int as libc::c_uchar));
    i = 0 as libc::c_int as libc::c_uchar;
    while (i as libc::c_int) < 4 as libc::c_int {
        let mut pass: *mut ean_pass_t =
            &mut *(*dcode).ean.pass.as_mut_ptr().offset(i as isize) as *mut ean_pass_t;
        if (*pass).state as libc::c_int >= 0 as libc::c_int
            || i as libc::c_int == pass_idx as libc::c_int
        {
            let mut part: zbar_symbol_type_t = ZBAR_NONE;
            part = decode_pass(dcode, pass);
            if part as u64 != 0 {
                /* update accumulated data from new partial decode */
                sym = integrate_partial(&mut (*dcode).ean, pass, part);
                if sym as u64 != 0 {
                    /* this pass valid => _reset_ all passes */
                    (*dcode).ean.pass[1 as libc::c_int as usize].state =
                        -(1 as libc::c_int) as libc::c_schar;
                    (*dcode).ean.pass[0 as libc::c_int as usize].state =
                        (*dcode).ean.pass[1 as libc::c_int as usize].state;
                    (*dcode).ean.pass[3 as libc::c_int as usize].state =
                        -(1 as libc::c_int) as libc::c_schar;
                    (*dcode).ean.pass[2 as libc::c_int as usize].state =
                        (*dcode).ean.pass[3 as libc::c_int as usize].state;
                    if sym as libc::c_uint > ZBAR_PARTIAL as libc::c_int as libc::c_uint {
                        if acquire_lock(dcode, sym) == 0 {
                            postprocess(dcode, sym);
                        } else {
                            sym = ZBAR_PARTIAL
                        }
                    }
                }
            }
        }
        i = i.wrapping_add(1)
    }
    return sym;
}
