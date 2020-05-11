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
 *------------------------------------------------------------------------*/
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
 *------------------------------------------------------------------------*/
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
 *------------------------------------------------------------------------*/
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
 *------------------------------------------------------------------------*/
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
 *------------------------------------------------------------------------*/
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
 *------------------------------------------------------------------------*/
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
pub const SCH_ISO646: C2RustUnnamed = 2;
pub const SCH_NUM: C2RustUnnamed = 0;
pub const SCH_ALNUM: C2RustUnnamed = 1;
/* DataBar character encoding groups */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group_s {
    pub sum: libc::c_ushort,
    pub wmax: libc::c_uchar,
    pub todd: libc::c_uchar,
    pub teven: libc::c_uchar,
}
pub type C2RustUnnamed = libc::c_uint;
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
unsafe extern "C" fn pair_width(mut dcode: *const zbar_decoder_t,
                                mut offset: libc::c_uchar) -> libc::c_uint {
    return get_width(dcode,
                     offset).wrapping_add(get_width(dcode,
                                                    (offset as libc::c_int +
                                                         1 as libc::c_int) as
                                                        libc::c_uchar));
}
#[inline]
unsafe extern "C" fn get_width(mut dcode: *const zbar_decoder_t,
                               mut offset: libc::c_uchar) -> libc::c_uint {
    return (*dcode).w[((*dcode).idx as libc::c_int - offset as libc::c_int &
                           16 as libc::c_int - 1 as libc::c_int) as usize];
}
#[inline]
unsafe extern "C" fn calc_s(mut dcode: *const zbar_decoder_t,
                            mut offset: libc::c_uchar, mut n: libc::c_uchar)
 -> libc::c_uint {
    let mut s: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    loop  {
        let fresh0 = n;
        n = n.wrapping_sub(1);
        if !(fresh0 != 0) { break ; }
        let fresh1 = offset;
        offset = offset.wrapping_add(1);
        s = s.wrapping_add(get_width(dcode, fresh1))
    }
    return s;
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
static mut finder_hash: [libc::c_schar; 32] =
    [0x16 as libc::c_int as libc::c_schar,
     0x1f as libc::c_int as libc::c_schar,
     0x2 as libc::c_int as libc::c_schar, 0 as libc::c_int as libc::c_schar,
     0x3 as libc::c_int as libc::c_schar, 0 as libc::c_int as libc::c_schar,
     0x6 as libc::c_int as libc::c_schar, 0xb as libc::c_int as libc::c_schar,
     0x1f as libc::c_int as libc::c_schar,
     0xe as libc::c_int as libc::c_schar,
     0x17 as libc::c_int as libc::c_schar,
     0xc as libc::c_int as libc::c_schar, 0xb as libc::c_int as libc::c_schar,
     0x14 as libc::c_int as libc::c_schar,
     0x11 as libc::c_int as libc::c_schar,
     0xc as libc::c_int as libc::c_schar,
     0x1f as libc::c_int as libc::c_schar,
     0x3 as libc::c_int as libc::c_schar,
     0x13 as libc::c_int as libc::c_schar,
     0x8 as libc::c_int as libc::c_schar, 0 as libc::c_int as libc::c_schar,
     0xa as libc::c_int as libc::c_schar,
     -(1 as libc::c_int) as libc::c_schar,
     0x16 as libc::c_int as libc::c_schar,
     0xc as libc::c_int as libc::c_schar, 0x9 as libc::c_int as libc::c_schar,
     -(1 as libc::c_int) as libc::c_schar,
     0x1a as libc::c_int as libc::c_schar,
     0x1f as libc::c_int as libc::c_schar,
     0x1c as libc::c_int as libc::c_schar, 0 as libc::c_int as libc::c_schar,
     -(1 as libc::c_int) as libc::c_schar];
#[no_mangle]
pub static mut groups: [group_s; 14] =
    [{
         let mut init =
             group_s{sum: 0 as libc::c_int as libc::c_ushort,
                     wmax: 7 as libc::c_int as libc::c_uchar,
                     todd: 87 as libc::c_int as libc::c_uchar,
                     teven: 4 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             group_s{sum: 348 as libc::c_int as libc::c_ushort,
                     wmax: 5 as libc::c_int as libc::c_uchar,
                     todd: 52 as libc::c_int as libc::c_uchar,
                     teven: 20 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             group_s{sum: 1388 as libc::c_int as libc::c_ushort,
                     wmax: 4 as libc::c_int as libc::c_uchar,
                     todd: 30 as libc::c_int as libc::c_uchar,
                     teven: 52 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             group_s{sum: 2948 as libc::c_int as libc::c_ushort,
                     wmax: 3 as libc::c_int as libc::c_uchar,
                     todd: 10 as libc::c_int as libc::c_uchar,
                     teven: 104 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             group_s{sum: 3988 as libc::c_int as libc::c_ushort,
                     wmax: 1 as libc::c_int as libc::c_uchar,
                     todd: 1 as libc::c_int as libc::c_uchar,
                     teven: 204 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             group_s{sum: 0 as libc::c_int as libc::c_ushort,
                     wmax: 8 as libc::c_int as libc::c_uchar,
                     todd: 161 as libc::c_int as libc::c_uchar,
                     teven: 1 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             group_s{sum: 161 as libc::c_int as libc::c_ushort,
                     wmax: 6 as libc::c_int as libc::c_uchar,
                     todd: 80 as libc::c_int as libc::c_uchar,
                     teven: 10 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             group_s{sum: 961 as libc::c_int as libc::c_ushort,
                     wmax: 4 as libc::c_int as libc::c_uchar,
                     todd: 31 as libc::c_int as libc::c_uchar,
                     teven: 34 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             group_s{sum: 2015 as libc::c_int as libc::c_ushort,
                     wmax: 3 as libc::c_int as libc::c_uchar,
                     todd: 10 as libc::c_int as libc::c_uchar,
                     teven: 70 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             group_s{sum: 2715 as libc::c_int as libc::c_ushort,
                     wmax: 1 as libc::c_int as libc::c_uchar,
                     todd: 1 as libc::c_int as libc::c_uchar,
                     teven: 126 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             group_s{sum: 1516 as libc::c_int as libc::c_ushort,
                     wmax: 8 as libc::c_int as libc::c_uchar,
                     todd: 81 as libc::c_int as libc::c_uchar,
                     teven: 1 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             group_s{sum: 1036 as libc::c_int as libc::c_ushort,
                     wmax: 6 as libc::c_int as libc::c_uchar,
                     todd: 48 as libc::c_int as libc::c_uchar,
                     teven: 10 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             group_s{sum: 336 as libc::c_int as libc::c_ushort,
                     wmax: 4 as libc::c_int as libc::c_uchar,
                     todd: 20 as libc::c_int as libc::c_uchar,
                     teven: 35 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             group_s{sum: 0 as libc::c_int as libc::c_ushort,
                     wmax: 2 as libc::c_int as libc::c_uchar,
                     todd: 4 as libc::c_int as libc::c_uchar,
                     teven: 84 as libc::c_int as libc::c_uchar,};
         init
     }];
static mut exp_sequences: [libc::c_uchar; 30] =
    [0x1 as libc::c_int as libc::c_uchar,
     0x23 as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0x7 as libc::c_int as libc::c_uchar,
     0x29 as libc::c_int as libc::c_uchar,
     0x47 as libc::c_int as libc::c_uchar,
     0x29 as libc::c_int as libc::c_uchar,
     0x67 as libc::c_int as libc::c_uchar,
     0xb as libc::c_int as libc::c_uchar,
     0x29 as libc::c_int as libc::c_uchar,
     0x87 as libc::c_int as libc::c_uchar,
     0xab as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0x43 as libc::c_int as libc::c_uchar,
     0x65 as libc::c_int as libc::c_uchar,
     0x7 as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0x43 as libc::c_int as libc::c_uchar,
     0x65 as libc::c_int as libc::c_uchar,
     0x89 as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0x43 as libc::c_int as libc::c_uchar,
     0x65 as libc::c_int as libc::c_uchar,
     0xa9 as libc::c_int as libc::c_uchar,
     0xb as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0x43 as libc::c_int as libc::c_uchar,
     0x67 as libc::c_int as libc::c_uchar,
     0x89 as libc::c_int as libc::c_uchar,
     0xab as libc::c_int as libc::c_uchar];
/* DataBar expanded checksum multipliers */
static mut exp_checksums: [libc::c_uchar; 12] =
    [1 as libc::c_int as libc::c_uchar, 189 as libc::c_int as libc::c_uchar,
     62 as libc::c_int as libc::c_uchar, 113 as libc::c_int as libc::c_uchar,
     46 as libc::c_int as libc::c_uchar, 43 as libc::c_int as libc::c_uchar,
     109 as libc::c_int as libc::c_uchar, 134 as libc::c_int as libc::c_uchar,
     6 as libc::c_int as libc::c_uchar, 79 as libc::c_int as libc::c_uchar,
     161 as libc::c_int as libc::c_uchar, 45 as libc::c_int as libc::c_uchar];
#[inline]
unsafe extern "C" fn append_check14(mut buf: *mut libc::c_uchar) {
    let mut chk: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    let mut d: libc::c_uchar = 0;
    let mut i: libc::c_int = 0;
    i = 13 as libc::c_int;
    loop  {
        i -= 1;
        if !(i >= 0 as libc::c_int) { break ; }
        let fresh2 = buf;
        buf = buf.offset(1);
        d = (*fresh2 as libc::c_int - '0' as i32) as libc::c_uchar;
        chk = (chk as libc::c_int + d as libc::c_int) as libc::c_uchar;
        if i & 1 as libc::c_int == 0 {
            chk =
                (chk as libc::c_int +
                     ((d as libc::c_int) << 1 as libc::c_int)) as
                    libc::c_uchar
        }
    }
    chk = (chk as libc::c_int % 10 as libc::c_int) as libc::c_uchar;
    if chk != 0 {
        chk = (10 as libc::c_int - chk as libc::c_int) as libc::c_uchar
    }
    *buf = (chk as libc::c_int + '0' as i32) as libc::c_uchar;
}
#[inline]
unsafe extern "C" fn decode10(mut buf: *mut libc::c_uchar,
                              mut n: libc::c_ulong, mut i: libc::c_int) {
    buf = buf.offset(i as isize);
    loop  {
        i -= 1;
        if !(i >= 0 as libc::c_int) { break ; }
        let mut d: libc::c_uchar =
            n.wrapping_rem(10 as libc::c_int as libc::c_ulong) as
                libc::c_uchar;
        n = n.wrapping_div(10 as libc::c_int as libc::c_ulong);
        buf = buf.offset(-1);
        *buf = ('0' as i32 + d as libc::c_int) as libc::c_uchar
    };
}
#[inline]
unsafe extern "C" fn databar_postprocess_exp(mut dcode: *mut zbar_decoder_t,
                                             mut data: *mut libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut enc: libc::c_int = 0;
    let mut n: libc::c_uint = 0;
    let mut buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let fresh3 = data;
    data = data.offset(1);
    let mut d: libc::c_ulong = *fresh3 as libc::c_ulong;
    let mut len: libc::c_int =
        d.wrapping_div(211 as libc::c_int as
                           libc::c_ulong).wrapping_add(4 as libc::c_int as
                                                           libc::c_ulong) as
            libc::c_int;
    let mut buflen: libc::c_int = 0;
    /* grok encodation method */
    let fresh4 = data;
    data = data.offset(1);
    d = *fresh4 as libc::c_ulong;
    n =
        (d >> 4 as libc::c_int & 0x7f as libc::c_int as libc::c_ulong) as
            libc::c_uint;
    if n >= 0x40 as libc::c_int as libc::c_uint {
        i = 10 as libc::c_int;
        enc = 1 as libc::c_int;
        buflen =
            2 as libc::c_int + 14 as libc::c_int +
                ((len * 12 as libc::c_int +
                      (10 as libc::c_int - 2 as libc::c_int -
                           44 as libc::c_int + 6 as libc::c_int)) *
                     2 as libc::c_int + 6 as libc::c_int) / 7 as libc::c_int +
                2 as libc::c_int
    } else if n >= 0x38 as libc::c_int as libc::c_uint {
        i = 4 as libc::c_int;
        enc =
            (6 as libc::c_int as
                 libc::c_uint).wrapping_add(n &
                                                7 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int;
        buflen =
            2 as libc::c_int + 14 as libc::c_int + 4 as libc::c_int +
                6 as libc::c_int + 2 as libc::c_int + 6 as libc::c_int +
                2 as libc::c_int
    } else if n >= 0x30 as libc::c_int as libc::c_uint {
        i = 6 as libc::c_int;
        enc =
            (2 as libc::c_int as
                 libc::c_uint).wrapping_add(n >> 2 as libc::c_int &
                                                1 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int;
        buflen =
            2 as libc::c_int + 14 as libc::c_int + 4 as libc::c_int +
                3 as libc::c_int +
                ((len * 12 as libc::c_int +
                      (6 as libc::c_int - 2 as libc::c_int - 44 as libc::c_int
                           - 2 as libc::c_int - 10 as libc::c_int)) *
                     2 as libc::c_int + 6 as libc::c_int) / 7 as libc::c_int +
                2 as libc::c_int
    } else if n >= 0x20 as libc::c_int as libc::c_uint {
        i = 7 as libc::c_int;
        enc =
            (4 as libc::c_int as
                 libc::c_uint).wrapping_add(n >> 3 as libc::c_int &
                                                1 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int;
        buflen =
            2 as libc::c_int + 14 as libc::c_int + 4 as libc::c_int +
                6 as libc::c_int
    } else {
        i = 9 as libc::c_int;
        enc = 0 as libc::c_int;
        buflen =
            ((len * 12 as libc::c_int + (9 as libc::c_int - 2 as libc::c_int))
                 * 2 as libc::c_int + 6 as libc::c_int) / 7 as libc::c_int +
                2 as libc::c_int
    }
    if !(buflen > 2 as libc::c_int) {
        fprintf(stderr,
                b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\tbuflen=%d\n\x00"
                    as *const u8 as *const libc::c_char,
                b"zbar/decoder/databar.c\x00" as *const u8 as
                    *const libc::c_char, 182 as libc::c_int,
                (*::std::mem::transmute::<&[u8; 24],
                                          &[libc::c_char; 24]>(b"databar_postprocess_exp\x00")).as_ptr(),
                b"buflen > 2\x00" as *const u8 as *const libc::c_char,
                buflen);
        return -(1 as libc::c_int)
    }
    if enc < 4 as libc::c_int {
        /* grok variable length symbol bit field */
        i -= 1;
        if (len as libc::c_ulong ^ d >> i) & 1 as libc::c_int as libc::c_ulong
               != 0 {
            /* even/odd length mismatch */
            return -(1 as libc::c_int)
        }
        i -= 1;
        if d >> i & 1 as libc::c_int as libc::c_ulong !=
               (len > 14 as libc::c_int) as libc::c_int as libc::c_ulong {
            /* size group mismatch */
            return -(1 as libc::c_int)
        }
    }
    len -= 2 as libc::c_int;
    if size_buf(dcode, buflen as libc::c_uint) != 0 {
        return -(1 as libc::c_int)
    }
    buf = (*dcode).buf;
    /* handle compressed fields */
    if enc != 0 {
        let fresh5 = buf;
        buf = buf.offset(1);
        *fresh5 = '0' as i32 as libc::c_uchar;
        let fresh6 = buf;
        buf = buf.offset(1);
        *fresh6 = '1' as i32 as libc::c_uchar
    }
    if enc == 1 as libc::c_int {
        i -= 4 as libc::c_int;
        n = (d >> i & 0xf as libc::c_int as libc::c_ulong) as libc::c_uint;
        if i >= 10 as libc::c_int { return -(1 as libc::c_int) }
        let fresh7 = buf;
        buf = buf.offset(1);
        *fresh7 =
            ('0' as i32 as libc::c_uint).wrapping_add(n) as libc::c_uchar
    } else if enc != 0 {
        let fresh8 = buf;
        buf = buf.offset(1);
        *fresh8 = '9' as i32 as libc::c_uchar
    }
    if enc != 0 {
        let mut j: libc::c_int = 0;
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            while i < 10 as libc::c_int && len != 0 {
                let fresh9 = data;
                data = data.offset(1);
                d =
                    d << 12 as libc::c_int |
                        (*fresh9 & 0xfff as libc::c_int) as libc::c_ulong;
                i += 12 as libc::c_int;
                len -= 1
            }
            i -= 10 as libc::c_int;
            n =
                (d >> i & 0x3ff as libc::c_int as libc::c_ulong) as
                    libc::c_uint;
            if n >= 1000 as libc::c_int as libc::c_uint {
                return -(1 as libc::c_int)
            }
            decode10(buf, n as libc::c_ulong, 3 as libc::c_int);
            buf = buf.offset(3 as libc::c_int as isize);
            j += 1
        }
        append_check14(buf.offset(-(13 as libc::c_int as isize)));
        buf = buf.offset(1)
    }
    match enc {
        2 => {
            /* 01100: AI 392x */
            while i < 2 as libc::c_int && len != 0 {
                let fresh10 = data;
                data = data.offset(1);
                d =
                    d << 12 as libc::c_int |
                        (*fresh10 & 0xfff as libc::c_int) as libc::c_ulong;
                i += 12 as libc::c_int;
                len -= 1
            }
            i -= 2 as libc::c_int;
            n =
                (d >> i & 0x3 as libc::c_int as libc::c_ulong) as
                    libc::c_uint;
            let fresh11 = buf;
            buf = buf.offset(1);
            *fresh11 = '3' as i32 as libc::c_uchar;
            let fresh12 = buf;
            buf = buf.offset(1);
            *fresh12 = '9' as i32 as libc::c_uchar;
            let fresh13 = buf;
            buf = buf.offset(1);
            *fresh13 = '2' as i32 as libc::c_uchar;
            let fresh14 = buf;
            buf = buf.offset(1);
            *fresh14 =
                ('0' as i32 as libc::c_uint).wrapping_add(n) as libc::c_uchar
        }
        3 => {
            /* 01101: AI 393x */
            while i < 12 as libc::c_int && len != 0 {
                let fresh15 = data;
                data = data.offset(1);
                d =
                    d << 12 as libc::c_int |
                        (*fresh15 & 0xfff as libc::c_int) as libc::c_ulong;
                i += 12 as libc::c_int;
                len -= 1
            }
            i -= 2 as libc::c_int;
            n =
                (d >> i & 0x3 as libc::c_int as libc::c_ulong) as
                    libc::c_uint;
            let fresh16 = buf;
            buf = buf.offset(1);
            *fresh16 = '3' as i32 as libc::c_uchar;
            let fresh17 = buf;
            buf = buf.offset(1);
            *fresh17 = '9' as i32 as libc::c_uchar;
            let fresh18 = buf;
            buf = buf.offset(1);
            *fresh18 = '3' as i32 as libc::c_uchar;
            let fresh19 = buf;
            buf = buf.offset(1);
            *fresh19 =
                ('0' as i32 as libc::c_uint).wrapping_add(n) as libc::c_uchar;
            i -= 10 as libc::c_int;
            n =
                (d >> i & 0x3ff as libc::c_int as libc::c_ulong) as
                    libc::c_uint;
            if n >= 1000 as libc::c_int as libc::c_uint {
                return -(1 as libc::c_int)
            }
            decode10(buf, n as libc::c_ulong, 3 as libc::c_int);
            buf = buf.offset(3 as libc::c_int as isize)
        }
        4 => {
            /* 0100: AI 3103 */
            while i < 15 as libc::c_int && len != 0 {
                let fresh20 = data;
                data = data.offset(1);
                d =
                    d << 12 as libc::c_int |
                        (*fresh20 & 0xfff as libc::c_int) as libc::c_ulong;
                i += 12 as libc::c_int;
                len -= 1
            }
            i -= 15 as libc::c_int;
            n =
                (d >> i & 0x7fff as libc::c_int as libc::c_ulong) as
                    libc::c_uint;
            let fresh21 = buf;
            buf = buf.offset(1);
            *fresh21 = '3' as i32 as libc::c_uchar;
            let fresh22 = buf;
            buf = buf.offset(1);
            *fresh22 = '1' as i32 as libc::c_uchar;
            let fresh23 = buf;
            buf = buf.offset(1);
            *fresh23 = '0' as i32 as libc::c_uchar;
            let fresh24 = buf;
            buf = buf.offset(1);
            *fresh24 = '3' as i32 as libc::c_uchar;
            decode10(buf, n as libc::c_ulong, 6 as libc::c_int);
            buf = buf.offset(6 as libc::c_int as isize)
        }
        5 => {
            /* 0101: AI 3202/3203 */
            while i < 15 as libc::c_int && len != 0 {
                let fresh25 = data;
                data = data.offset(1);
                d =
                    d << 12 as libc::c_int |
                        (*fresh25 & 0xfff as libc::c_int) as libc::c_ulong;
                i += 12 as libc::c_int;
                len -= 1
            }
            i -= 15 as libc::c_int;
            n =
                (d >> i & 0x7fff as libc::c_int as libc::c_ulong) as
                    libc::c_uint;
            let fresh26 = buf;
            buf = buf.offset(1);
            *fresh26 = '3' as i32 as libc::c_uchar;
            let fresh27 = buf;
            buf = buf.offset(1);
            *fresh27 = '2' as i32 as libc::c_uchar;
            let fresh28 = buf;
            buf = buf.offset(1);
            *fresh28 = '0' as i32 as libc::c_uchar;
            let fresh29 = buf;
            buf = buf.offset(1);
            *fresh29 =
                if n >= 10000 as libc::c_int as libc::c_uint {
                    '3' as i32
                } else { '2' as i32 } as libc::c_uchar;
            if n >= 10000 as libc::c_int as libc::c_uint {
                n = n.wrapping_sub(10000 as libc::c_int as libc::c_uint)
            }
            decode10(buf, n as libc::c_ulong, 6 as libc::c_int);
            buf = buf.offset(6 as libc::c_int as isize)
        }
        _ => { }
    }
    if enc >= 6 as libc::c_int {
        /* 0111000 - 0111111: AI 310x/320x + AI 11/13/15/17 */
        let fresh30 = buf;
        buf = buf.offset(1);
        *fresh30 = '3' as i32 as libc::c_uchar;
        let fresh31 = buf;
        buf = buf.offset(1);
        *fresh31 = ('1' as i32 + (enc & 1 as libc::c_int)) as libc::c_uchar;
        let fresh32 = buf;
        buf = buf.offset(1);
        *fresh32 = '0' as i32 as libc::c_uchar;
        let fresh33 = buf;
        buf = buf.offset(1);
        *fresh33 = 'x' as i32 as libc::c_uchar;
        while i < 20 as libc::c_int && len != 0 {
            let fresh34 = data;
            data = data.offset(1);
            d =
                d << 12 as libc::c_int |
                    (*fresh34 & 0xfff as libc::c_int) as libc::c_ulong;
            i += 12 as libc::c_int;
            len -= 1
        }
        i -= 20 as libc::c_int;
        n =
            (d >> i & 0xfffff as libc::c_int as libc::c_ulong) as
                libc::c_uint;
        if n >= 1000000 as libc::c_int as libc::c_uint {
            return -(1 as libc::c_int)
        }
        decode10(buf, n as libc::c_ulong, 6 as libc::c_int);
        *buf.offset(-(1 as libc::c_int as isize)) = *buf;
        *buf = '0' as i32 as libc::c_uchar;
        buf = buf.offset(6 as libc::c_int as isize);
        while i < 16 as libc::c_int && len != 0 {
            let fresh35 = data;
            data = data.offset(1);
            d =
                d << 12 as libc::c_int |
                    (*fresh35 & 0xfff as libc::c_int) as libc::c_ulong;
            i += 12 as libc::c_int;
            len -= 1
        }
        i -= 16 as libc::c_int;
        n = (d >> i & 0xffff as libc::c_int as libc::c_ulong) as libc::c_uint;
        if n < 38400 as libc::c_int as libc::c_uint {
            let mut dd: libc::c_int = 0;
            let mut mm: libc::c_int = 0;
            let mut yy: libc::c_int = 0;
            dd =
                n.wrapping_rem(32 as libc::c_int as libc::c_uint) as
                    libc::c_int;
            n = n.wrapping_div(32 as libc::c_int as libc::c_uint);
            mm =
                n.wrapping_rem(12 as libc::c_int as
                                   libc::c_uint).wrapping_add(1 as libc::c_int
                                                                  as
                                                                  libc::c_uint)
                    as libc::c_int;
            n = n.wrapping_div(12 as libc::c_int as libc::c_uint);
            yy = n as libc::c_int;
            let fresh36 = buf;
            buf = buf.offset(1);
            *fresh36 = '1' as i32 as libc::c_uchar;
            let fresh37 = buf;
            buf = buf.offset(1);
            *fresh37 =
                ('0' as i32 + (enc - 6 as libc::c_int | 1 as libc::c_int)) as
                    libc::c_uchar;
            decode10(buf, yy as libc::c_ulong, 2 as libc::c_int);
            buf = buf.offset(2 as libc::c_int as isize);
            decode10(buf, mm as libc::c_ulong, 2 as libc::c_int);
            buf = buf.offset(2 as libc::c_int as isize);
            decode10(buf, dd as libc::c_ulong, 2 as libc::c_int);
            buf = buf.offset(2 as libc::c_int as isize)
        } else if n > 38400 as libc::c_int as libc::c_uint {
            return -(1 as libc::c_int)
        }
    }
    if enc < 4 as libc::c_int {
        /* remainder is general-purpose data compaction */
        let mut scheme: libc::c_int = SCH_NUM as libc::c_int;
        while i > 0 as libc::c_int || len > 0 as libc::c_int {
            while i < 8 as libc::c_int && len != 0 {
                let fresh38 = data;
                data = data.offset(1);
                d =
                    d << 12 as libc::c_int |
                        (*fresh38 & 0xfff as libc::c_int) as libc::c_ulong;
                i += 12 as libc::c_int;
                len -= 1
            }
            if scheme == SCH_NUM as libc::c_int {
                let mut n1: libc::c_int = 0;
                i -= 4 as libc::c_int;
                if i < 0 as libc::c_int { break ; }
                if d >> i & 0xf as libc::c_int as libc::c_ulong == 0 {
                    scheme = SCH_ALNUM as libc::c_int
                } else if len == 0 && i < 3 as libc::c_int {
                    /* special case last digit */
                    n =
                        (d >> i &
                             0xf as libc::c_int as
                                 libc::c_ulong).wrapping_sub(1 as libc::c_int
                                                                 as
                                                                 libc::c_ulong)
                            as libc::c_uint;
                    if n > 9 as libc::c_int as libc::c_uint {
                        return -(1 as libc::c_int)
                    }
                    let fresh39 = buf;
                    buf = buf.offset(1);
                    *fresh39 =
                        ('0' as i32 as libc::c_uint).wrapping_add(n) as
                            libc::c_uchar;
                    break ;
                } else {
                    i -= 3 as libc::c_int;
                    if !(i >= 0 as libc::c_int) {
                        fprintf(stderr,
                                b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\t\n\x00"
                                    as *const u8 as *const libc::c_char,
                                b"zbar/decoder/databar.c\x00" as *const u8 as
                                    *const libc::c_char, 337 as libc::c_int,
                                (*::std::mem::transmute::<&[u8; 24],
                                                          &[libc::c_char; 24]>(b"databar_postprocess_exp\x00")).as_ptr(),
                                b"i >= 0\x00" as *const u8 as
                                    *const libc::c_char);
                        return -(1 as libc::c_int)
                    }
                    n =
                        (d >> i &
                             0x7f as libc::c_int as
                                 libc::c_ulong).wrapping_sub(8 as libc::c_int
                                                                 as
                                                                 libc::c_ulong)
                            as libc::c_uint;
                    n1 =
                        n.wrapping_rem(11 as libc::c_int as libc::c_uint) as
                            libc::c_int;
                    n = n.wrapping_div(11 as libc::c_int as libc::c_uint);
                    let fresh40 = buf;
                    buf = buf.offset(1);
                    *fresh40 =
                        if n < 10 as libc::c_int as libc::c_uint {
                            ('0' as i32 as libc::c_uint).wrapping_add(n)
                        } else { '\u{1d}' as i32 as libc::c_uint } as
                            libc::c_uchar;
                    let fresh41 = buf;
                    buf = buf.offset(1);
                    *fresh41 =
                        if n1 < 10 as libc::c_int {
                            ('0' as i32) + n1
                        } else { '\u{1d}' as i32 } as libc::c_uchar
                }
            } else {
                let mut c: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                i -= 3 as libc::c_int;
                if i < 0 as libc::c_int { break ; }
                if d >> i & 0x7 as libc::c_int as libc::c_ulong == 0 {
                    scheme = SCH_NUM as libc::c_int
                } else {
                    i -= 2 as libc::c_int;
                    if i < 0 as libc::c_int { break ; }
                    n =
                        (d >> i & 0x1f as libc::c_int as libc::c_ulong) as
                            libc::c_uint;
                    if n == 0x4 as libc::c_int as libc::c_uint {
                        scheme ^= 0x3 as libc::c_int
                    } else if n == 0xf as libc::c_int as libc::c_uint {
                        c = '\u{1d}' as i32 as libc::c_uint
                    } else if n < 0xf as libc::c_int as libc::c_uint {
                        c =
                            (43 as libc::c_int as
                                 libc::c_uint).wrapping_add(n)
                    } else if scheme == SCH_ALNUM as libc::c_int {
                        i -= 1;
                        if i < 0 as libc::c_int { return -(1 as libc::c_int) }
                        n =
                            (d >> i & 0x1f as libc::c_int as libc::c_ulong) as
                                libc::c_uint;
                        if n < 0x1a as libc::c_int as libc::c_uint {
                            c = ('A' as i32 as libc::c_uint).wrapping_add(n)
                        } else if n == 0x1a as libc::c_int as libc::c_uint {
                            c = '*' as i32 as libc::c_uint
                        } else if n < 0x1f as libc::c_int as libc::c_uint {
                            c =
                                (',' as i32 as
                                     libc::c_uint).wrapping_add(n).wrapping_sub(0x1b
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint)
                        } else { return -(1 as libc::c_int) }
                    } else if scheme == SCH_ISO646 as libc::c_int &&
                                  n < 0x1d as libc::c_int as libc::c_uint {
                        i -= 2 as libc::c_int;
                        if i < 0 as libc::c_int { return -(1 as libc::c_int) }
                        n =
                            (d >> i & 0x3f as libc::c_int as libc::c_ulong) as
                                libc::c_uint;
                        if n < 0x1a as libc::c_int as libc::c_uint {
                            c = ('A' as i32 as libc::c_uint).wrapping_add(n)
                        } else if n < 0x34 as libc::c_int as libc::c_uint {
                            c =
                                ('a' as i32 as
                                     libc::c_uint).wrapping_add(n).wrapping_sub(0x1a
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint)
                        } else { return -(1 as libc::c_int) }
                    } else if scheme == SCH_ISO646 as libc::c_int {
                        i -= 3 as libc::c_int;
                        if i < 0 as libc::c_int { return -(1 as libc::c_int) }
                        n =
                            (d >> i & 0x1f as libc::c_int as libc::c_ulong) as
                                libc::c_uint;
                        if n < 0xa as libc::c_int as libc::c_uint {
                            c =
                                ('!' as i32 as
                                     libc::c_uint).wrapping_add(n).wrapping_sub(8
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint)
                        } else if n < 0x15 as libc::c_int as libc::c_uint {
                            c =
                                ('%' as i32 as
                                     libc::c_uint).wrapping_add(n).wrapping_sub(0xa
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint)
                        } else if n < 0x1b as libc::c_int as libc::c_uint {
                            c =
                                (':' as i32 as
                                     libc::c_uint).wrapping_add(n).wrapping_sub(0x15
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint)
                        } else if n == 0x1b as libc::c_int as libc::c_uint {
                            c = '_' as i32 as libc::c_uint
                        } else if n == 0x1c as libc::c_int as libc::c_uint {
                            c = ' ' as i32 as libc::c_uint
                        } else { return -(1 as libc::c_int) }
                    } else { return -(1 as libc::c_int) }
                    if c != 0 {
                        let fresh42 = buf;
                        buf = buf.offset(1);
                        *fresh42 = c as libc::c_uchar
                    }
                }
            }
        }
    }
    i = buf.wrapping_offset_from((*dcode).buf) as libc::c_long as libc::c_int;
    if !((i as libc::c_uint) < (*dcode).buf_alloc) {
        fprintf(stderr,
                b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\ti=%02x %s\n\x00"
                    as *const u8 as *const libc::c_char,
                b"zbar/decoder/databar.c\x00" as *const u8 as
                    *const libc::c_char, 425 as libc::c_int,
                (*::std::mem::transmute::<&[u8; 24],
                                          &[libc::c_char; 24]>(b"databar_postprocess_exp\x00")).as_ptr(),
                b"i < dcode->buf_alloc\x00" as *const u8 as
                    *const libc::c_char, i,
                _zbar_decoder_buf_dump((*dcode).buf, i as libc::c_uint));
        return -(1 as libc::c_int)
    }
    *buf = 0 as libc::c_int as libc::c_uchar;
    (*dcode).buflen = i as libc::c_uint;
    if i != 0 &&
           { buf = buf.offset(-1); (*buf as libc::c_int) == '\u{1d}' as i32 }
       {
        *buf = 0 as libc::c_int as libc::c_uchar;
        (*dcode).buflen = (*dcode).buflen.wrapping_sub(1)
    }
    return 0 as libc::c_int;
}
/* convert from heterogeneous base {1597,2841}
 * to base 10 character representation
 */
#[inline]
unsafe extern "C" fn databar_postprocess(mut dcode: *mut zbar_decoder_t,
                                         mut d: *mut libc::c_uint) {
    let mut db: *mut databar_decoder_t = &mut (*dcode).databar;
    let mut i: libc::c_int = 0;
    let mut c: libc::c_uint = 0;
    let mut chk: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut buf: *mut libc::c_uchar = (*dcode).buf;
    let fresh43 = buf;
    buf = buf.offset(1);
    *fresh43 = '0' as i32 as libc::c_uchar;
    let fresh44 = buf;
    buf = buf.offset(1);
    *fresh44 = '1' as i32 as libc::c_uchar;
    buf = buf.offset(15 as libc::c_int as isize);
    buf = buf.offset(-1);
    *buf = '\u{0}' as i32 as libc::c_uchar;
    buf = buf.offset(-1);
    *buf = '\u{0}' as i32 as libc::c_uchar;
    let mut r: libc::c_ulong =
        (*d.offset(0 as libc::c_int as
                       isize)).wrapping_mul(1597 as libc::c_int as
                                                libc::c_uint).wrapping_add(*d.offset(1
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         isize))
            as libc::c_ulong;
    *d.offset(1 as libc::c_int as isize) =
        r.wrapping_div(10000 as libc::c_int as libc::c_ulong) as libc::c_uint;
    r = r.wrapping_rem(10000 as libc::c_int as libc::c_ulong);
    r =
        r.wrapping_mul(2841 as libc::c_int as
                           libc::c_ulong).wrapping_add(*d.offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                                           as libc::c_ulong);
    *d.offset(2 as libc::c_int as isize) =
        r.wrapping_div(10000 as libc::c_int as libc::c_ulong) as libc::c_uint;
    r = r.wrapping_rem(10000 as libc::c_int as libc::c_ulong);
    r =
        r.wrapping_mul(1597 as libc::c_int as
                           libc::c_ulong).wrapping_add(*d.offset(3 as
                                                                     libc::c_int
                                                                     as isize)
                                                           as libc::c_ulong);
    *d.offset(3 as libc::c_int as isize) =
        r.wrapping_div(10000 as libc::c_int as libc::c_ulong) as libc::c_uint;
    i = 4 as libc::c_int;
    loop  {
        i -= 1;
        if !(i >= 0 as libc::c_int) { break ; }
        c =
            r.wrapping_rem(10 as libc::c_int as libc::c_ulong) as
                libc::c_uint;
        chk = chk.wrapping_add(c);
        if i & 1 as libc::c_int != 0 {
            chk = chk.wrapping_add(c << 1 as libc::c_int)
        }
        buf = buf.offset(-1);
        *buf = c.wrapping_add('0' as i32 as libc::c_uint) as libc::c_uchar;
        if i != 0 { r = r.wrapping_div(10 as libc::c_int as libc::c_ulong) }
    }
    r =
        (*d.offset(1 as libc::c_int as
                       isize)).wrapping_mul(2841 as libc::c_int as
                                                libc::c_uint).wrapping_add(*d.offset(2
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         isize))
            as libc::c_ulong;
    *d.offset(2 as libc::c_int as isize) =
        r.wrapping_div(10000 as libc::c_int as libc::c_ulong) as libc::c_uint;
    r = r.wrapping_rem(10000 as libc::c_int as libc::c_ulong);
    r =
        r.wrapping_mul(1597 as libc::c_int as
                           libc::c_ulong).wrapping_add(*d.offset(3 as
                                                                     libc::c_int
                                                                     as isize)
                                                           as libc::c_ulong);
    *d.offset(3 as libc::c_int as isize) =
        r.wrapping_div(10000 as libc::c_int as libc::c_ulong) as libc::c_uint;
    i = 4 as libc::c_int;
    loop  {
        i -= 1;
        if !(i >= 0 as libc::c_int) { break ; }
        c =
            r.wrapping_rem(10 as libc::c_int as libc::c_ulong) as
                libc::c_uint;
        chk = chk.wrapping_add(c);
        if i & 1 as libc::c_int != 0 {
            chk = chk.wrapping_add(c << 1 as libc::c_int)
        }
        buf = buf.offset(-1);
        *buf = c.wrapping_add('0' as i32 as libc::c_uint) as libc::c_uchar;
        if i != 0 { r = r.wrapping_div(10 as libc::c_int as libc::c_ulong) }
    }
    r =
        (*d.offset(2 as libc::c_int as
                       isize)).wrapping_mul(1597 as libc::c_int as
                                                libc::c_uint).wrapping_add(*d.offset(3
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         isize))
            as libc::c_ulong;
    i = 5 as libc::c_int;
    loop  {
        i -= 1;
        if !(i >= 0 as libc::c_int) { break ; }
        c =
            r.wrapping_rem(10 as libc::c_int as libc::c_ulong) as
                libc::c_uint;
        chk = chk.wrapping_add(c);
        if i & 1 as libc::c_int == 0 {
            chk = chk.wrapping_add(c << 1 as libc::c_int)
        }
        buf = buf.offset(-1);
        *buf = c.wrapping_add('0' as i32 as libc::c_uint) as libc::c_uchar;
        if i != 0 { r = r.wrapping_div(10 as libc::c_int as libc::c_ulong) }
    }
    /* NB linkage flag not supported */
    if (*db).config >> ZBAR_CFG_EMIT_CHECK as libc::c_int &
           1 as libc::c_int as libc::c_uint != 0 {
        chk = chk.wrapping_rem(10 as libc::c_int as libc::c_uint);
        if chk != 0 {
            chk = (10 as libc::c_int as libc::c_uint).wrapping_sub(chk)
        }
        *buf.offset(13 as libc::c_int as isize) =
            chk.wrapping_add('0' as i32 as libc::c_uint) as libc::c_uchar;
        (*dcode).buflen =
            (buf.wrapping_offset_from((*dcode).buf) as libc::c_long +
                 14 as libc::c_int as libc::c_long) as libc::c_uint
    } else {
        (*dcode).buflen =
            (buf.wrapping_offset_from((*dcode).buf) as libc::c_long +
                 13 as libc::c_int as libc::c_long) as libc::c_uint
    };
}
#[inline]
unsafe extern "C" fn check_width(mut wf: libc::c_uint, mut wd: libc::c_uint,
                                 mut n: libc::c_uint) -> libc::c_int {
    let mut dwf: libc::c_uint =
        wf.wrapping_mul(3 as libc::c_int as libc::c_uint);
    wd = wd.wrapping_mul(14 as libc::c_int as libc::c_uint);
    wf = wf.wrapping_mul(n);
    return (wf.wrapping_sub(dwf) <= wd && wd <= wf.wrapping_add(dwf)) as
               libc::c_int;
}
#[inline]
unsafe extern "C" fn merge_segment(mut db: *mut databar_decoder_t,
                                   mut seg: *mut databar_segment_t) {
    let mut csegs: libc::c_uint = (*db).csegs();
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < csegs {
        let mut s: *mut databar_segment_t = (*db).segs.offset(i as isize);
        if s != seg && (*s).finder() == (*seg).finder() &&
               (*s).exp() as libc::c_int == (*seg).exp() as libc::c_int &&
               (*s).color() as libc::c_int == (*seg).color() as libc::c_int &&
               (*s).side() as libc::c_int == (*seg).side() as libc::c_int &&
               (*s).data as libc::c_int == (*seg).data as libc::c_int &&
               (*s).check() as libc::c_int == (*seg).check() as libc::c_int &&
               check_width((*seg).width as libc::c_uint,
                           (*s).width as libc::c_uint,
                           14 as libc::c_int as libc::c_uint) != 0 {
            /* merge with existing segment */
            let mut cnt: libc::c_uint = (*s).count();
            if cnt < 0x7f as libc::c_int as libc::c_uint {
                cnt = cnt.wrapping_add(1)
            }
            (*seg).set_count(cnt);
            (*seg).set_partial((*seg).partial() &
                                   (*s).partial() as libc::c_int as
                                       libc::c_uint);
            (*seg).width =
                ((3 as libc::c_int * (*seg).width as libc::c_int +
                      (*s).width as libc::c_int + 2 as libc::c_int) /
                     4 as libc::c_int) as libc::c_ushort;
            (*s).set_finder(-(1 as libc::c_int))
        } else if (*s).finder() >= 0 as libc::c_int {
            let mut age: libc::c_uint =
                ((*db).epoch() as libc::c_int - (*s).epoch() as libc::c_int &
                     0xff as libc::c_int) as libc::c_uint;
            if age >= 248 as libc::c_int as libc::c_uint ||
                   age >= 128 as libc::c_int as libc::c_uint &&
                       ((*s).count() as libc::c_int) < 2 as libc::c_int {
                (*s).set_finder(-(1 as libc::c_int))
            }
        }
        i += 1
    };
}
#[inline]
unsafe extern "C" fn match_segment(mut dcode: *mut zbar_decoder_t,
                                   mut seg: *mut databar_segment_t)
 -> zbar_symbol_type_t {
    let mut db: *mut databar_decoder_t = &mut (*dcode).databar;
    let mut csegs: libc::c_uint = (*db).csegs();
    let mut maxage: libc::c_uint = 0xfff as libc::c_int as libc::c_uint;
    let mut i0: libc::c_int = 0;
    let mut i1: libc::c_int = 0;
    let mut i2: libc::c_int = 0;
    let mut maxcnt: libc::c_int = 0 as libc::c_int;
    let mut smax: [*mut databar_segment_t; 3] =
        [0 as *mut databar_segment_t, 0 as *mut databar_segment_t,
         0 as *mut databar_segment_t];
    if (*seg).partial() as libc::c_int != 0 &&
           ((*seg).count() as libc::c_int) < 4 as libc::c_int {
        return ZBAR_PARTIAL
    }
    i0 = 0 as libc::c_int;
    while (i0 as libc::c_uint) < csegs {
        let mut s0: *mut databar_segment_t = (*db).segs.offset(i0 as isize);
        if !(s0 == seg || (*s0).finder() != (*seg).finder() ||
                 (*s0).exp() as libc::c_int != 0 ||
                 (*s0).color() as libc::c_int != (*seg).color() as libc::c_int
                 ||
                 (*s0).side() as libc::c_int == (*seg).side() as libc::c_int
                 ||
                 (*s0).partial() as libc::c_int != 0 &&
                     ((*s0).count() as libc::c_int) < 4 as libc::c_int ||
                 check_width((*seg).width as libc::c_uint,
                             (*s0).width as libc::c_uint,
                             14 as libc::c_int as libc::c_uint) == 0) {
            i1 = 0 as libc::c_int;
            while (i1 as libc::c_uint) < csegs {
                let mut s1: *mut databar_segment_t =
                    (*db).segs.offset(i1 as isize);
                let mut chkf: libc::c_int = 0;
                let mut chks: libc::c_int = 0;
                let mut chk: libc::c_int = 0;
                let mut age1: libc::c_uint = 0;
                if !(i1 == i0 || (*s1).finder() < 0 as libc::c_int ||
                         (*s1).exp() as libc::c_int != 0 ||
                         (*s1).color() as libc::c_int ==
                             (*seg).color() as libc::c_int ||
                         (*s1).partial() as libc::c_int != 0 &&
                             ((*s1).count() as libc::c_int) < 4 as libc::c_int
                         ||
                         check_width((*seg).width as libc::c_uint,
                                     (*s1).width as libc::c_uint,
                                     14 as libc::c_int as libc::c_uint) == 0)
                   {
                    if (*seg).color() != 0 {
                        chkf =
                            (*seg).finder() +
                                (*s1).finder() * 9 as libc::c_int
                    } else {
                        chkf =
                            (*s1).finder() +
                                (*seg).finder() * 9 as libc::c_int
                    }
                    if chkf > 72 as libc::c_int { chkf -= 1 }
                    if chkf > 8 as libc::c_int { chkf -= 1 }
                    chks =
                        ((*seg).check() as libc::c_int +
                             (*s0).check() as libc::c_int +
                             (*s1).check() as libc::c_int) %
                            79 as libc::c_int;
                    if chkf >= chks {
                        chk = chkf - chks
                    } else { chk = 79 as libc::c_int + chkf - chks }
                    age1 =
                        (((*db).epoch() as libc::c_int -
                              (*s0).epoch() as libc::c_int &
                              0xff as libc::c_int) +
                             ((*db).epoch() as libc::c_int -
                                  (*s1).epoch() as libc::c_int &
                                  0xff as libc::c_int)) as libc::c_uint;
                    i2 = i1 + 1 as libc::c_int;
                    while (i2 as libc::c_uint) < csegs {
                        let mut s2: *mut databar_segment_t =
                            (*db).segs.offset(i2 as isize);
                        let mut cnt: libc::c_uint = 0;
                        let mut age2: libc::c_uint = 0;
                        let mut age: libc::c_uint = 0;
                        if !(i2 == i0 || (*s2).finder() != (*s1).finder() ||
                                 (*s2).exp() as libc::c_int != 0 ||
                                 (*s2).color() as libc::c_int !=
                                     (*s1).color() as libc::c_int ||
                                 (*s2).side() as libc::c_int ==
                                     (*s1).side() as libc::c_int ||
                                 (*s2).check() as libc::c_int != chk ||
                                 (*s2).partial() as libc::c_int != 0 &&
                                     ((*s2).count() as libc::c_int) <
                                         4 as libc::c_int ||
                                 check_width((*seg).width as libc::c_uint,
                                             (*s2).width as libc::c_uint,
                                             14 as libc::c_int as
                                                 libc::c_uint) == 0) {
                            age2 =
                                ((*db).epoch() as libc::c_int -
                                     (*s2).epoch() as libc::c_int &
                                     0xff as libc::c_int) as libc::c_uint;
                            age = age1.wrapping_add(age2);
                            cnt =
                                ((*s0).count() as libc::c_int +
                                     (*s1).count() as libc::c_int +
                                     (*s2).count() as libc::c_int) as
                                    libc::c_uint;
                            if (maxcnt as libc::c_uint) < cnt ||
                                   maxcnt as libc::c_uint == cnt &&
                                       maxage > age {
                                maxcnt = cnt as libc::c_int;
                                maxage = age;
                                smax[0 as libc::c_int as usize] = s0;
                                smax[1 as libc::c_int as usize] = s1;
                                smax[2 as libc::c_int as usize] = s2
                            }
                        }
                        i2 += 1
                    }
                }
                i1 += 1
            }
        }
        i0 += 1
    }
    if smax[0 as libc::c_int as usize].is_null() { return ZBAR_PARTIAL }
    let mut d: [libc::c_uint; 4] = [0; 4];
    d[(((*seg).color() as libc::c_int) << 1 as libc::c_int |
           (*seg).side() as libc::c_int) as usize] =
        (*seg).data as libc::c_uint;
    i0 = 0 as libc::c_int;
    while i0 < 3 as libc::c_int {
        d[(((*smax[i0 as usize]).color() as libc::c_int) << 1 as libc::c_int |
               (*smax[i0 as usize]).side() as libc::c_int) as usize] =
            (*smax[i0 as usize]).data as libc::c_uint;
        (*smax[i0 as usize]).set_count((*smax[i0 as usize]).count() - 1);
        if (*smax[i0 as usize]).count() == 0 {
            (*smax[i0 as usize]).set_finder(-(1 as libc::c_int))
        }
        i0 += 1
    }
    (*seg).set_finder(-(1 as libc::c_int));
    if size_buf(dcode, 18 as libc::c_int as libc::c_uint) != 0 {
        return ZBAR_PARTIAL
    }
    if acquire_lock(dcode, ZBAR_DATABAR) != 0 { return ZBAR_PARTIAL }
    databar_postprocess(dcode, d.as_mut_ptr());
    (*dcode).modifiers =
        ((1 as libc::c_int) << ZBAR_MOD_GS1 as libc::c_int) as libc::c_uint;
    (*dcode).direction =
        1 as libc::c_int -
            2 as libc::c_int *
                ((*seg).side() as libc::c_int ^ (*seg).color() as libc::c_int
                     ^ 1 as libc::c_int);
    return ZBAR_DATABAR;
}
#[inline]
unsafe extern "C" fn lookup_sequence(mut seg: *mut databar_segment_t,
                                     mut fixed: libc::c_int,
                                     mut seq: *mut libc::c_int)
 -> libc::c_uint {
    let mut n: libc::c_uint =
        ((*seg).data as libc::c_int / 211 as libc::c_int) as libc::c_uint;
    let mut i: libc::c_uint = 0;
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    i =
        n.wrapping_add(1 as libc::c_int as
                           libc::c_uint).wrapping_div(2 as libc::c_int as
                                                          libc::c_uint).wrapping_add(1
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_uint);
    n = n.wrapping_add(4 as libc::c_int as libc::c_uint);
    i = i.wrapping_mul(i).wrapping_div(4 as libc::c_int as libc::c_uint);
    p = exp_sequences.as_ptr().offset(i as isize);
    fixed >>= 1 as libc::c_int;
    *seq.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    *seq.offset(1 as libc::c_int as isize) = 1 as libc::c_int;
    i = 2 as libc::c_int as libc::c_uint;
    while i < n {
        let mut s: libc::c_int = *p as libc::c_int;
        if i & 2 as libc::c_int as libc::c_uint == 0 {
            p = p.offset(1);
            s >>= 4 as libc::c_int
        } else { s &= 0xf as libc::c_int }
        if s == fixed { fixed = -(1 as libc::c_int) }
        s <<= 1 as libc::c_int;
        let fresh45 = s;
        s = s + 1;
        let fresh46 = i;
        i = i.wrapping_add(1);
        *seq.offset(fresh46 as isize) = fresh45;
        let fresh47 = i;
        i = i.wrapping_add(1);
        *seq.offset(fresh47 as isize) = s
    }
    *seq.offset(n as isize) = -(1 as libc::c_int);
    return (fixed < 1 as libc::c_int) as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn match_segment_exp(mut dcode: *mut zbar_decoder_t,
                                       mut seg: *mut databar_segment_t,
                                       mut dir: libc::c_int)
 -> zbar_symbol_type_t {
    let mut db: *mut databar_decoder_t = &mut (*dcode).databar;
    let mut bestsegs: [libc::c_int; 22] = [0; 22];
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut segs: [libc::c_int; 22] = [0; 22];
    let mut seq: [libc::c_int; 22] = [0; 22];
    let mut ifixed: libc::c_int =
        seg.wrapping_offset_from((*db).segs) as libc::c_long as libc::c_int;
    let mut fixed: libc::c_int =
        (*seg).finder() << 2 as libc::c_int |
            ((*seg).color() as libc::c_int) << 1 as libc::c_int |
            (*seg).color() as libc::c_int ^ (*seg).side() as libc::c_int;
    let mut maxcnt: libc::c_int = 0 as libc::c_int;
    let mut iseg: [libc::c_int; 32] = [0; 32];
    let mut csegs: libc::c_uint = (*db).csegs();
    let mut width: libc::c_uint = (*seg).width as libc::c_uint;
    let mut maxage: libc::c_uint = 0x7fff as libc::c_int as libc::c_uint;
    seq[1 as libc::c_int as usize] = -(1 as libc::c_int);
    segs[0 as libc::c_int as usize] = seq[1 as libc::c_int as usize];
    bestsegs[0 as libc::c_int as usize] = segs[0 as libc::c_int as usize];
    seq[0 as libc::c_int as usize] = 0 as libc::c_int;
    i = csegs as libc::c_int;
    seg =
        (*db).segs.offset(csegs as
                              isize).offset(-(1 as libc::c_int as isize));
    loop  {
        i -= 1;
        if !(i >= 0 as libc::c_int) { break ; }
        if (*seg).exp() as libc::c_int != 0 &&
               (*seg).finder() >= 0 as libc::c_int &&
               ((*seg).partial() == 0 ||
                    (*seg).count() as libc::c_int >= 4 as libc::c_int) {
            iseg[i as usize] =
                (*seg).finder() << 2 as libc::c_int |
                    ((*seg).color() as libc::c_int) << 1 as libc::c_int |
                    (*seg).color() as libc::c_int ^
                        (*seg).side() as libc::c_int
        } else { iseg[i as usize] = -(1 as libc::c_int) }
        seg = seg.offset(-1)
    }
    i = 0 as libc::c_int;
    loop  {
        (i) == 0;
        let mut current_block_36: u64;
        while i >= 0 as libc::c_int && seq[i as usize] >= 0 as libc::c_int {
            let mut j: libc::c_int = 0;
            if seq[i as usize] == fixed {
                seg = (*db).segs.offset(ifixed as isize);
                if segs[i as usize] < 0 as libc::c_int &&
                       check_width(width, (*seg).width as libc::c_uint,
                                   14 as libc::c_int as libc::c_uint) != 0 {
                    j = ifixed;
                    current_block_36 = 5529461102203738653;
                } else { current_block_36 = 4808432441040389987; }
            } else {
                j = segs[i as usize] + 1 as libc::c_int;
                while (j as libc::c_uint) < csegs {
                    if iseg[j as usize] == seq[i as usize] &&
                           (i == 0 ||
                                check_width(width,
                                            (*(*db).segs.offset(j as
                                                                    isize)).width
                                                as libc::c_uint,
                                            14 as libc::c_int as libc::c_uint)
                                    != 0) {
                        seg = (*db).segs.offset(j as isize);
                        break ;
                    } else { j += 1 }
                }
                if j as libc::c_uint == csegs {
                    current_block_36 = 4808432441040389987;
                } else { current_block_36 = 5529461102203738653; }
            }
            match current_block_36 {
                5529461102203738653 => {
                    if i == 0 {
                        if lookup_sequence(seg, fixed, seq.as_mut_ptr()) == 0
                           {
                            current_block_36 = 4808432441040389987;
                        } else {
                            width = (*seg).width as libc::c_uint;
                            current_block_36 = 1423531122933789233;
                        }
                    } else {
                        width =
                            width.wrapping_add((*seg).width as
                                                   libc::c_uint).wrapping_div(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_uint);
                        current_block_36 = 1423531122933789233;
                    }
                    match current_block_36 {
                        4808432441040389987 => { }
                        _ => {
                            let fresh48 = i;
                            i = i + 1;
                            segs[fresh48 as usize] = j;
                            let fresh49 = i;
                            i = i + 1;
                            segs[fresh49 as usize] = -(1 as libc::c_int)
                        }
                    }
                }
                _ => { }
            }
            i -= 1
        }
        if i < 0 as libc::c_int { break ; }
        seg = (*db).segs.offset(segs[0 as libc::c_int as usize] as isize);
        let mut cnt: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        let mut chk: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        let mut age: libc::c_uint =
            ((*db).epoch() as libc::c_int - (*seg).epoch() as libc::c_int &
                 0xff as libc::c_int) as libc::c_uint;
        i = 1 as libc::c_int;
        while segs[i as usize] >= 0 as libc::c_int {
            seg = (*db).segs.offset(segs[i as usize] as isize);
            chk = chk.wrapping_add((*seg).check());
            cnt = cnt.wrapping_add((*seg).count());
            age =
                age.wrapping_add(((*db).epoch() as libc::c_int -
                                      (*seg).epoch() as libc::c_int &
                                      0xff as libc::c_int) as libc::c_uint);
            i += 1
        }
        let mut data0: libc::c_uint =
            (*(*db).segs.offset(segs[0 as libc::c_int as usize] as
                                    isize)).data as libc::c_uint;
        let mut chk0: libc::c_uint =
            data0.wrapping_rem(211 as libc::c_int as libc::c_uint);
        chk = chk.wrapping_rem(211 as libc::c_int as libc::c_uint);
        if !(chk != chk0) {
            if !(maxcnt as libc::c_uint > cnt ||
                     maxcnt as libc::c_uint == cnt && maxage <= age) {
                maxcnt = cnt as libc::c_int;
                maxage = age;
                i = 0 as libc::c_int;
                while segs[i as usize] >= 0 as libc::c_int {
                    bestsegs[i as usize] = segs[i as usize];
                    i += 1
                }
                bestsegs[i as usize] = -(1 as libc::c_int)
            }
        }
        i -= 1
    }
    if bestsegs[0 as libc::c_int as usize] < 0 as libc::c_int {
        return ZBAR_PARTIAL
    }
    if acquire_lock(dcode, ZBAR_DATABAR_EXP) != 0 { return ZBAR_PARTIAL }
    i = 0 as libc::c_int;
    while bestsegs[i as usize] >= 0 as libc::c_int {
        segs[i as usize] =
            (*(*db).segs.offset(bestsegs[i as usize] as isize)).data as
                libc::c_int;
        i += 1
    }
    if databar_postprocess_exp(dcode, segs.as_mut_ptr()) != 0 {
        release_lock(dcode, ZBAR_DATABAR_EXP);
        return ZBAR_PARTIAL
    }
    i = 0 as libc::c_int;
    while bestsegs[i as usize] >= 0 as libc::c_int {
        if bestsegs[i as usize] != ifixed {
            seg = (*db).segs.offset(bestsegs[i as usize] as isize);
            (*seg).set_count((*seg).count() - 1);
            if (*seg).count() == 0 { (*seg).set_finder(-(1 as libc::c_int)) }
        }
        i += 1
    }
    /* FIXME stacked rows are frequently reversed,
     * so direction is impossible to determine at this level
     */
    (*dcode).direction =
        (1 as libc::c_int -
             2 as libc::c_int *
                 ((*seg).side() as libc::c_int ^
                      (*seg).color() as libc::c_int)) * dir;
    (*dcode).modifiers =
        ((1 as libc::c_int) << ZBAR_MOD_GS1 as libc::c_int) as libc::c_uint;
    return ZBAR_DATABAR_EXP;
}
#[inline]
unsafe extern "C" fn calc_check(mut sig0: libc::c_uint,
                                mut sig1: libc::c_uint,
                                mut side: libc::c_uint,
                                mut mod_0: libc::c_uint) -> libc::c_uint {
    let mut chk: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i: libc::c_int = 0;
    i = 4 as libc::c_int;
    loop  {
        i -= 1;
        if !(i >= 0 as libc::c_int) { break ; }
        chk =
            chk.wrapping_mul(3 as libc::c_int as
                                 libc::c_uint).wrapping_add(sig1 &
                                                                0xf as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint).wrapping_add(1
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_uint).wrapping_mul(3
                                                                                                                                  as
                                                                                                                                  libc::c_int
                                                                                                                                  as
                                                                                                                                  libc::c_uint).wrapping_add(sig0
                                                                                                                                                                 &
                                                                                                                                                                 0xf
                                                                                                                                                                     as
                                                                                                                                                                     libc::c_int
                                                                                                                                                                     as
                                                                                                                                                                     libc::c_uint).wrapping_add(1
                                                                                                                                                                                                    as
                                                                                                                                                                                                    libc::c_int
                                                                                                                                                                                                    as
                                                                                                                                                                                                    libc::c_uint);
        sig1 >>= 4 as libc::c_int;
        sig0 >>= 4 as libc::c_int;
        if i & 1 as libc::c_int == 0 { chk = chk.wrapping_rem(mod_0) }
    }
    if side != 0 {
        chk =
            chk.wrapping_mul((6561 as libc::c_int as
                                  libc::c_uint).wrapping_rem(mod_0)).wrapping_rem(mod_0)
    }
    return chk;
}
#[inline]
unsafe extern "C" fn calc_value4(mut sig: libc::c_uint, mut n: libc::c_uint,
                                 mut wmax: libc::c_uint,
                                 mut nonarrow: libc::c_uint) -> libc::c_int {
    let mut v: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    n = n.wrapping_sub(1);
    let mut w0: libc::c_uint =
        sig >> 12 as libc::c_int & 0xf as libc::c_int as libc::c_uint;
    if w0 > 1 as libc::c_int as libc::c_uint {
        if w0 > wmax { return -(1 as libc::c_int) }
        let mut n0: libc::c_uint = n.wrapping_sub(w0);
        let mut sk20: libc::c_uint =
            n.wrapping_sub(1 as libc::c_int as
                               libc::c_uint).wrapping_mul(n).wrapping_mul((2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint).wrapping_mul(n).wrapping_sub(1
                                                                                                                              as
                                                                                                                              libc::c_int
                                                                                                                              as
                                                                                                                              libc::c_uint));
        let mut sk21: libc::c_uint =
            n0.wrapping_mul(n0.wrapping_add(1 as libc::c_int as
                                                libc::c_uint)).wrapping_mul((2
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint).wrapping_mul(n0).wrapping_add(1
                                                                                                                                 as
                                                                                                                                 libc::c_int
                                                                                                                                 as
                                                                                                                                 libc::c_uint));
        v =
            sk20.wrapping_sub(sk21).wrapping_sub((3 as libc::c_int as
                                                      libc::c_uint).wrapping_mul(w0.wrapping_sub(1
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_uint)).wrapping_mul((2
                                                                                                                                      as
                                                                                                                                      libc::c_int
                                                                                                                                      as
                                                                                                                                      libc::c_uint).wrapping_mul(n).wrapping_sub(w0)));
        if nonarrow == 0 && w0 > 2 as libc::c_int as libc::c_uint &&
               n > 4 as libc::c_int as libc::c_uint {
            let mut k: libc::c_uint =
                n.wrapping_sub(2 as libc::c_int as
                                   libc::c_uint).wrapping_mul(n.wrapping_sub(1
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint)).wrapping_mul((2
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  libc::c_uint).wrapping_mul(n).wrapping_sub(3
                                                                                                                                                                 as
                                                                                                                                                                 libc::c_int
                                                                                                                                                                 as
                                                                                                                                                                 libc::c_uint)).wrapping_sub(sk21);
            k =
                k.wrapping_sub((3 as libc::c_int as
                                    libc::c_uint).wrapping_mul(w0.wrapping_sub(2
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint)).wrapping_mul((14
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                                    as
                                                                                                                    libc::c_uint).wrapping_mul(n).wrapping_sub((7
                                                                                                                                                                    as
                                                                                                                                                                    libc::c_int
                                                                                                                                                                    as
                                                                                                                                                                    libc::c_uint).wrapping_mul(w0)).wrapping_sub(31
                                                                                                                                                                                                                     as
                                                                                                                                                                                                                     libc::c_int
                                                                                                                                                                                                                     as
                                                                                                                                                                                                                     libc::c_uint)));
            v = v.wrapping_sub(k)
        }
        if n.wrapping_sub(2 as libc::c_int as libc::c_uint) > wmax {
            let mut wm20: libc::c_uint =
                (2 as libc::c_int as
                     libc::c_uint).wrapping_mul(wmax).wrapping_mul(wmax.wrapping_add(1
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_uint));
            let mut wm21: libc::c_uint =
                (2 as libc::c_int as
                     libc::c_uint).wrapping_mul(wmax).wrapping_add(1 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint);
            let mut k_0: libc::c_uint = sk20;
            if n0 > wmax {
                k_0 = k_0.wrapping_sub(sk21);
                k_0 =
                    k_0.wrapping_add((3 as libc::c_int as
                                          libc::c_uint).wrapping_mul(w0.wrapping_sub(1
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_uint)).wrapping_mul(wm20.wrapping_sub(wm21.wrapping_mul((2
                                                                                                                                                              as
                                                                                                                                                              libc::c_int
                                                                                                                                                              as
                                                                                                                                                              libc::c_uint).wrapping_mul(n).wrapping_sub(w0)))))
            } else {
                k_0 =
                    k_0.wrapping_sub(wmax.wrapping_add(1 as libc::c_int as
                                                           libc::c_uint).wrapping_mul(wmax.wrapping_add(2
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_uint)).wrapping_mul((2
                                                                                                                                             as
                                                                                                                                             libc::c_int
                                                                                                                                             as
                                                                                                                                             libc::c_uint).wrapping_mul(wmax).wrapping_add(3
                                                                                                                                                                                               as
                                                                                                                                                                                               libc::c_int
                                                                                                                                                                                               as
                                                                                                                                                                                               libc::c_uint)));
                k_0 =
                    k_0.wrapping_add((3 as libc::c_int as
                                          libc::c_uint).wrapping_mul(n.wrapping_sub(wmax).wrapping_sub(2
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_uint)).wrapping_mul(wm20.wrapping_sub(wm21.wrapping_mul(n.wrapping_add(wmax).wrapping_add(1
                                                                                                                                                                                                                 as
                                                                                                                                                                                                                 libc::c_int
                                                                                                                                                                                                                 as
                                                                                                                                                                                                                 libc::c_uint)))))
            }
            k_0 = k_0.wrapping_mul(3 as libc::c_int as libc::c_uint);
            v = v.wrapping_sub(k_0)
        }
        v = v.wrapping_div(12 as libc::c_int as libc::c_uint)
    } else { nonarrow = 1 as libc::c_int as libc::c_uint }
    n = n.wrapping_sub(w0);
    let mut w1: libc::c_uint =
        sig >> 8 as libc::c_int & 0xf as libc::c_int as libc::c_uint;
    if w1 > 1 as libc::c_int as libc::c_uint {
        if w1 > wmax { return -(1 as libc::c_int) }
        v =
            v.wrapping_add((2 as libc::c_int as
                                libc::c_uint).wrapping_mul(n).wrapping_sub(w1).wrapping_mul(w1.wrapping_sub(1
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                as
                                                                                                                libc::c_uint)).wrapping_div(2
                                                                                                                                                as
                                                                                                                                                libc::c_int
                                                                                                                                                as
                                                                                                                                                libc::c_uint));
        if nonarrow == 0 && w1 > 2 as libc::c_int as libc::c_uint &&
               n > 3 as libc::c_int as libc::c_uint {
            v =
                v.wrapping_sub((2 as libc::c_int as
                                    libc::c_uint).wrapping_mul(n).wrapping_sub(w1).wrapping_sub(5
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_uint).wrapping_mul(w1.wrapping_sub(2
                                                                                                                                                   as
                                                                                                                                                   libc::c_int
                                                                                                                                                   as
                                                                                                                                                   libc::c_uint)).wrapping_div(2
                                                                                                                                                                                   as
                                                                                                                                                                                   libc::c_int
                                                                                                                                                                                   as
                                                                                                                                                                                   libc::c_uint))
        }
        if n.wrapping_sub(1 as libc::c_int as libc::c_uint) > wmax {
            if n.wrapping_sub(w1) > wmax {
                v =
                    v.wrapping_sub(w1.wrapping_sub(1 as libc::c_int as
                                                       libc::c_uint).wrapping_mul((2
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_uint).wrapping_mul(n).wrapping_sub(w1).wrapping_sub((2
                                                                                                                                                        as
                                                                                                                                                        libc::c_int
                                                                                                                                                        as
                                                                                                                                                        libc::c_uint).wrapping_mul(wmax))))
            } else {
                v =
                    v.wrapping_sub(n.wrapping_sub(wmax).wrapping_mul(n.wrapping_sub(wmax).wrapping_sub(1
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_uint)))
            }
        }
    } else { nonarrow = 1 as libc::c_int as libc::c_uint }
    n = n.wrapping_sub(w1);
    let mut w2: libc::c_uint =
        sig >> 4 as libc::c_int & 0xf as libc::c_int as libc::c_uint;
    if w2 > 1 as libc::c_int as libc::c_uint {
        if w2 > wmax { return -(1 as libc::c_int) }
        v = v.wrapping_add(w2.wrapping_sub(1 as libc::c_int as libc::c_uint));
        if nonarrow == 0 && w2 > 2 as libc::c_int as libc::c_uint &&
               n > 2 as libc::c_int as libc::c_uint {
            v =
                v.wrapping_sub(n.wrapping_sub(2 as libc::c_int as
                                                  libc::c_uint))
        }
        if n > wmax { v = v.wrapping_sub(n.wrapping_sub(wmax)) }
    } else { nonarrow = 1 as libc::c_int as libc::c_uint }
    let mut w3: libc::c_uint = sig & 0xf as libc::c_int as libc::c_uint;
    if w3 == 1 as libc::c_int as libc::c_uint {
        nonarrow = 1 as libc::c_int as libc::c_uint
    } else if w3 > wmax { return -(1 as libc::c_int) }
    if nonarrow == 0 { return -(1 as libc::c_int) }
    return v as libc::c_int;
}
#[inline]
unsafe extern "C" fn decode_char(mut dcode: *mut zbar_decoder_t,
                                 mut seg: *mut databar_segment_t,
                                 mut off: libc::c_int, mut dir: libc::c_int)
 -> zbar_symbol_type_t {
    let mut db: *mut databar_decoder_t = &mut (*dcode).databar;
    let mut s: libc::c_uint =
        calc_s(dcode,
               if dir > 0 as libc::c_int {
                   off
               } else { (off) - 6 as libc::c_int } as libc::c_uchar,
               8 as libc::c_int as libc::c_uchar);
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut emin: [libc::c_int; 2] = [0 as libc::c_int, 0];
    let mut sum: libc::c_int = 0 as libc::c_int;
    let mut sig0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut sig1: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if (*seg).exp() != 0 {
        n = 17 as libc::c_int
    } else if (*seg).side() != 0 {
        n = 15 as libc::c_int
    } else { n = 16 as libc::c_int }
    emin[1 as libc::c_int as usize] = -n;
    if s < 13 as libc::c_int as libc::c_uint ||
           check_width((*seg).width as libc::c_uint, s, n as libc::c_uint) ==
               0 {
        return ZBAR_NONE
    }
    i = 4 as libc::c_int;
    loop  {
        i -= 1;
        if !(i >= 0 as libc::c_int) { break ; }
        let mut e: libc::c_int =
            decode_e(pair_width(dcode, off as libc::c_uchar), s,
                     n as libc::c_uint);
        if e < 0 as libc::c_int { return ZBAR_NONE }
        sum = e - sum;
        off += dir;
        sig1 <<= 4 as libc::c_int;
        if emin[1 as libc::c_int as usize] < -sum {
            emin[1 as libc::c_int as usize] = -sum
        }
        sig1 = sig1.wrapping_add(sum as libc::c_uint);
        if i == 0 { break ; }
        e =
            decode_e(pair_width(dcode, off as libc::c_uchar), s,
                     n as libc::c_uint);
        if e < 0 as libc::c_int { return ZBAR_NONE }
        sum = e - sum;
        off += dir;
        sig0 <<= 4 as libc::c_int;
        if emin[0 as libc::c_int as usize] > sum {
            emin[0 as libc::c_int as usize] = sum
        }
        sig0 = sig0.wrapping_add(sum as libc::c_uint)
    }
    let mut diff: libc::c_int = emin[(!n & 1 as libc::c_int) as usize];
    diff = diff + (diff << 4 as libc::c_int);
    diff = diff + (diff << 8 as libc::c_int);
    sig0 = sig0.wrapping_sub(diff as libc::c_uint);
    sig1 = sig1.wrapping_add(diff as libc::c_uint);
    let mut sum0: libc::c_uint = sig0.wrapping_add(sig0 >> 8 as libc::c_int);
    let mut sum1: libc::c_uint = sig1.wrapping_add(sig1 >> 8 as libc::c_int);
    sum0 = sum0.wrapping_add(sum0 >> 4 as libc::c_int);
    sum1 = sum1.wrapping_add(sum1 >> 4 as libc::c_int);
    sum0 &= 0xf as libc::c_int as libc::c_uint;
    sum1 &= 0xf as libc::c_int as libc::c_uint;
    if sum0.wrapping_add(sum1).wrapping_add(8 as libc::c_int as libc::c_uint)
           != n as libc::c_uint {
        return ZBAR_NONE
    }
    if (sum0 ^ (n >> 1 as libc::c_int) as libc::c_uint |
            sum1 ^ (n >> 1 as libc::c_int) as libc::c_uint ^
                n as libc::c_uint) & 1 as libc::c_int as libc::c_uint != 0 {
        return ZBAR_NONE
    }
    i =
        (((n & 0x3 as libc::c_int ^ 1 as libc::c_int) * 5 as libc::c_int) as
             libc::c_uint).wrapping_add(sum1 >> 1 as libc::c_int) as
            libc::c_int;
    if !((i as libc::c_ulong) <
             (::std::mem::size_of::<[group_s; 14]>() as
                  libc::c_ulong).wrapping_div(::std::mem::size_of::<group_s>()
                                                  as libc::c_ulong)) {
        fprintf(stderr,
                b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\tn=%d sum=%d/%d sig=%04x/%04x g=%d\x00"
                    as *const u8 as *const libc::c_char,
                b"zbar/decoder/databar.c\x00" as *const u8 as
                    *const libc::c_char, 1026 as libc::c_int,
                (*::std::mem::transmute::<&[u8; 12],
                                          &[libc::c_char; 12]>(b"decode_char\x00")).as_ptr(),
                b"i < sizeof(groups) / sizeof(*groups)\x00" as *const u8 as
                    *const libc::c_char, n, sum0, sum1, sig0, sig1, i);
        return 4294967295 as zbar_symbol_type_t
    }
    let mut g: *mut group_s = groups.as_mut_ptr().offset(i as isize);
    let mut vodd: libc::c_int =
        calc_value4(sig0.wrapping_add(0x1111 as libc::c_int as libc::c_uint),
                    sum0.wrapping_add(4 as libc::c_int as libc::c_uint),
                    (*g).wmax as libc::c_uint,
                    (!n & 1 as libc::c_int) as libc::c_uint);
    if vodd < 0 as libc::c_int || vodd > (*g).todd as libc::c_int {
        return ZBAR_NONE
    }
    let mut veven: libc::c_int =
        calc_value4(sig1.wrapping_add(0x1111 as libc::c_int as libc::c_uint),
                    sum1.wrapping_add(4 as libc::c_int as libc::c_uint),
                    (9 as libc::c_int - (*g).wmax as libc::c_int) as
                        libc::c_uint, (n & 1 as libc::c_int) as libc::c_uint);
    if veven < 0 as libc::c_int || veven > (*g).teven as libc::c_int {
        return ZBAR_NONE
    }
    let mut v: libc::c_int = (*g).sum as libc::c_int;
    if n & 2 as libc::c_int != 0 {
        v += vodd + veven * (*g).todd as libc::c_int
    } else { v += veven + vodd * (*g).teven as libc::c_int }
    let mut chk: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if (*seg).exp() != 0 {
        let mut side: libc::c_uint =
            ((*seg).color() as libc::c_int ^ (*seg).side() as libc::c_int ^
                 1 as libc::c_int) as libc::c_uint;
        if v >= 4096 as libc::c_int { return ZBAR_NONE }
        /* skip A1 left */
        chk =
            calc_check(sig0, sig1, side, 211 as libc::c_int as libc::c_uint);
        if (*seg).finder() != 0 || (*seg).color() as libc::c_int != 0 ||
               (*seg).side() as libc::c_int != 0 {
            i =
                (((*seg).finder() << 1 as libc::c_int) as
                     libc::c_uint).wrapping_sub(side).wrapping_add((*seg).color())
                    as libc::c_int;
            if !(i >= 0 as libc::c_int && i < 12 as libc::c_int) {
                fprintf(stderr,
                        b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\tf=%d(%x%x%x) side=%d i=%d\n\x00"
                            as *const u8 as *const libc::c_char,
                        b"zbar/decoder/databar.c\x00" as *const u8 as
                            *const libc::c_char, 1060 as libc::c_int,
                        (*::std::mem::transmute::<&[u8; 12],
                                                  &[libc::c_char; 12]>(b"decode_char\x00")).as_ptr(),
                        b"i >= 0 && i < 12\x00" as *const u8 as
                            *const libc::c_char, (*seg).finder(),
                        (*seg).exp() as libc::c_int,
                        (*seg).color() as libc::c_int,
                        (*seg).side() as libc::c_int, side, i);
                return ZBAR_NONE
            }
            chk =
                chk.wrapping_mul(exp_checksums[i as usize] as
                                     libc::c_uint).wrapping_rem(211 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint)
        } else if v >= 4009 as libc::c_int {
            return ZBAR_NONE
        } else { chk = 0 as libc::c_int as libc::c_uint }
    } else {
        chk =
            calc_check(sig0, sig1, (*seg).side(),
                       79 as libc::c_int as libc::c_uint);
        if (*seg).color() != 0 {
            chk =
                chk.wrapping_mul(16 as libc::c_int as
                                     libc::c_uint).wrapping_rem(79 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint)
        }
    }
    (*seg).set_check(chk);
    (*seg).data = v as libc::c_short;
    merge_segment(db, seg);
    if (*seg).exp() != 0 {
        return match_segment_exp(dcode, seg, dir)
    } else { if dir > 0 as libc::c_int { return match_segment(dcode, seg) } }
    return ZBAR_PARTIAL;
}
#[inline]
unsafe extern "C" fn alloc_segment(mut db: *mut databar_decoder_t)
 -> libc::c_int {
    let mut maxage: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut csegs: libc::c_uint = (*db).csegs();
    let mut i: libc::c_int = 0;
    let mut old: libc::c_int = -(1 as libc::c_int);
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < csegs {
        let mut seg: *mut databar_segment_t = (*db).segs.offset(i as isize);
        let mut age: libc::c_uint = 0;
        if (*seg).finder() < 0 as libc::c_int { return i }
        age =
            ((*db).epoch() as libc::c_int - (*seg).epoch() as libc::c_int &
                 0xff as libc::c_int) as libc::c_uint;
        if age >= 128 as libc::c_int as libc::c_uint &&
               ((*seg).count() as libc::c_int) < 2 as libc::c_int {
            (*seg).set_finder(-(1 as libc::c_int));
            return i
        }
        /* score based on both age and count */
        if age > (*seg).count() {
            age =
                age.wrapping_sub((*seg).count()).wrapping_add(1 as libc::c_int
                                                                  as
                                                                  libc::c_uint)
        } else { age = 1 as libc::c_int as libc::c_uint }
        if maxage < age { maxage = age; old = i }
        i += 1
    }
    if csegs < 32 as libc::c_int as libc::c_uint {
        i = csegs as libc::c_int;
        csegs = csegs.wrapping_mul(2 as libc::c_int as libc::c_uint);
        if csegs > 32 as libc::c_int as libc::c_uint {
            csegs = 32 as libc::c_int as libc::c_uint
        }
        if csegs != (*db).csegs() {
            let mut seg_0: *mut databar_segment_t =
                0 as *mut databar_segment_t;
            (*db).segs =
                realloc((*db).segs as *mut libc::c_void,
                        (csegs as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<databar_segment_t>()
                                                             as
                                                             libc::c_ulong))
                    as *mut databar_segment_t;
            (*db).set_csegs(csegs);
            seg_0 = (*db).segs.offset(csegs as isize);
            loop  {
                seg_0 = seg_0.offset(-1);
                csegs = csegs.wrapping_sub(1);
                if !(csegs >= i as libc::c_uint) { break ; }
                (*seg_0).set_finder(-(1 as libc::c_int));
                (*seg_0).set_exp(0 as libc::c_int as libc::c_uint);
                (*seg_0).set_color(0 as libc::c_int as libc::c_uint);
                (*seg_0).set_side(0 as libc::c_int as libc::c_uint);
                (*seg_0).set_partial(0 as libc::c_int as libc::c_uint);
                (*seg_0).set_count(0 as libc::c_int as libc::c_uint);
                (*seg_0).set_epoch(0 as libc::c_int as libc::c_uint);
                (*seg_0).set_check(0 as libc::c_int as libc::c_uint)
            }
            return i
        }
    }
    if !(old >= 0 as libc::c_int) {
        fprintf(stderr,
                b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\t\n\x00" as
                    *const u8 as *const libc::c_char,
                b"zbar/decoder/databar.c\x00" as *const u8 as
                    *const libc::c_char, 1144 as libc::c_int,
                (*::std::mem::transmute::<&[u8; 14],
                                          &[libc::c_char; 14]>(b"alloc_segment\x00")).as_ptr(),
                b"old >= 0\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    let ref mut fresh50 = *(*db).segs.offset(old as isize);
    (*fresh50).set_finder(-(1 as libc::c_int));
    return old;
}
#[inline]
unsafe extern "C" fn decode_finder(mut dcode: *mut zbar_decoder_t)
 -> zbar_symbol_type_t {
    let mut db: *mut databar_decoder_t = &mut (*dcode).databar;
    let mut seg: *mut databar_segment_t = 0 as *mut databar_segment_t;
    let mut e0: libc::c_uint =
        pair_width(dcode, 1 as libc::c_int as libc::c_uchar);
    let mut e2: libc::c_uint =
        pair_width(dcode, 3 as libc::c_int as libc::c_uchar);
    let mut e1: libc::c_uint = 0;
    let mut e3: libc::c_uint = 0;
    let mut s: libc::c_uint = 0;
    let mut finder: libc::c_uint = 0;
    let mut dir: libc::c_uint = 0;
    let mut sig: libc::c_int = 0;
    let mut iseg: libc::c_int = 0;
    if e0 < e2 {
        let mut e: libc::c_uint =
            e2.wrapping_mul(4 as libc::c_int as libc::c_uint);
        if e < (15 as libc::c_int as libc::c_uint).wrapping_mul(e0) ||
               e > (34 as libc::c_int as libc::c_uint).wrapping_mul(e0) {
            return ZBAR_NONE
        }
        dir = 0 as libc::c_int as libc::c_uint;
        e3 = pair_width(dcode, 4 as libc::c_int as libc::c_uchar)
    } else {
        let mut e_0: libc::c_uint =
            e0.wrapping_mul(4 as libc::c_int as libc::c_uint);
        if e_0 < (15 as libc::c_int as libc::c_uint).wrapping_mul(e2) ||
               e_0 > (34 as libc::c_int as libc::c_uint).wrapping_mul(e2) {
            return ZBAR_NONE
        }
        dir = 1 as libc::c_int as libc::c_uint;
        e2 = e0;
        e3 = pair_width(dcode, 0 as libc::c_int as libc::c_uchar)
    }
    e1 = pair_width(dcode, 2 as libc::c_int as libc::c_uchar);
    s = e1.wrapping_add(e3);
    if s < 12 as libc::c_int as libc::c_uint { return ZBAR_NONE }
    sig =
        decode_e(e3, s, 14 as libc::c_int as libc::c_uint) << 8 as libc::c_int
            |
            decode_e(e2, s, 14 as libc::c_int as libc::c_uint) <<
                4 as libc::c_int |
            decode_e(e1, s, 14 as libc::c_int as libc::c_uint);
    if sig < 0 as libc::c_int ||
           (sig >> 4 as libc::c_int & 0xf as libc::c_int) < 8 as libc::c_int
           || sig >> 4 as libc::c_int & 0xf as libc::c_int > 10 as libc::c_int
           || sig & 0xf as libc::c_int >= 10 as libc::c_int ||
           sig >> 8 as libc::c_int & 0xf as libc::c_int >= 10 as libc::c_int
           ||
           (sig >> 8 as libc::c_int) + sig & 0xf as libc::c_int !=
               10 as libc::c_int {
        return ZBAR_NONE
    }
    finder =
        (finder_hash[(sig - (sig >> 5 as libc::c_int) & 0x1f as libc::c_int)
                         as usize] as libc::c_int +
             finder_hash[(sig >> 1 as libc::c_int & 0x1f as libc::c_int) as
                             usize] as libc::c_int & 0x1f as libc::c_int) as
            libc::c_uint;
    if finder == 0x1f as libc::c_int as libc::c_uint ||
           (if finder < 9 as libc::c_int as libc::c_uint {
                (*db).config
            } else { (*db).config_exp }) >> ZBAR_CFG_ENABLE as libc::c_int &
               1 as libc::c_int as libc::c_uint == 0 {
        return ZBAR_NONE
    }
    if !(finder >= 0 as libc::c_int as libc::c_uint) {
        fprintf(stderr,
                b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\tdir=%d sig=%04x f=%d\n\x00"
                    as *const u8 as *const libc::c_char,
                b"zbar/decoder/databar.c\x00" as *const u8 as
                    *const libc::c_char, 1201 as libc::c_int,
                (*::std::mem::transmute::<&[u8; 14],
                                          &[libc::c_char; 14]>(b"decode_finder\x00")).as_ptr(),
                b"finder >= 0\x00" as *const u8 as *const libc::c_char, dir,
                sig & 0xfff as libc::c_int, finder);
        return ZBAR_NONE
    }
    iseg = alloc_segment(db);
    if iseg < 0 as libc::c_int { return ZBAR_NONE }
    seg = (*db).segs.offset(iseg as isize);
    (*seg).set_finder(if finder >= 9 as libc::c_int as libc::c_uint {
                          finder.wrapping_sub(9 as libc::c_int as
                                                  libc::c_uint)
                      } else { finder } as libc::c_int);
    (*seg).set_exp((finder >= 9 as libc::c_int as libc::c_uint) as libc::c_int
                       as libc::c_uint);
    (*seg).set_color(get_color(dcode) as libc::c_uint ^ dir ^
                         1 as libc::c_int as libc::c_uint);
    (*seg).set_side(dir);
    (*seg).set_partial(0 as libc::c_int as libc::c_uint);
    (*seg).set_count(1 as libc::c_int as libc::c_uint);
    (*seg).width = s as libc::c_ushort;
    (*seg).set_epoch((*db).epoch());
    let mut rc: libc::c_int =
        decode_char(dcode, seg,
                    (12 as libc::c_int as libc::c_uint).wrapping_sub(dir) as
                        libc::c_int, -(1 as libc::c_int)) as libc::c_int;
    if rc == 0 {
        (*seg).set_partial(1 as libc::c_int as libc::c_uint)
    } else { (*db).set_epoch((*db).epoch() + 1) }
    let mut i: libc::c_int =
        ((((*dcode).idx as libc::c_int + 8 as libc::c_int) as
              libc::c_uint).wrapping_add(dir) &
             0xf as libc::c_int as libc::c_uint) as libc::c_int;
    if !((*db).chars[i as usize] as libc::c_int == -(1 as libc::c_int)) {
        fprintf(stderr,
                b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\t\n\x00" as
                    *const u8 as *const libc::c_char,
                b"zbar/decoder/databar.c\x00" as *const u8 as
                    *const libc::c_char, 1224 as libc::c_int,
                (*::std::mem::transmute::<&[u8; 14],
                                          &[libc::c_char; 14]>(b"decode_finder\x00")).as_ptr(),
                b"db->chars[i] == -1\x00" as *const u8 as
                    *const libc::c_char);
        return ZBAR_NONE
    }
    (*db).chars[i as usize] = iseg as libc::c_schar;
    return rc as zbar_symbol_type_t;
}
/* reset DataBar segment decode state */
/* reset DataBar accumulated segments */
/* decode DataBar symbols */
#[no_mangle]
pub unsafe extern "C" fn _zbar_decode_databar(mut dcode: *mut zbar_decoder_t)
 -> zbar_symbol_type_t {
    let mut db: *mut databar_decoder_t = &mut (*dcode).databar;
    let mut seg: *mut databar_segment_t = 0 as *mut databar_segment_t;
    let mut pair: *mut databar_segment_t = 0 as *mut databar_segment_t;
    let mut sym: zbar_symbol_type_t = ZBAR_NONE;
    let mut iseg: libc::c_int = 0;
    let mut i: libc::c_int = (*dcode).idx as libc::c_int & 0xf as libc::c_int;
    sym = decode_finder(dcode);
    iseg = (*db).chars[i as usize] as libc::c_int;
    if iseg < 0 as libc::c_int { return sym }
    (*db).chars[i as usize] = -(1 as libc::c_int) as libc::c_schar;
    seg = (*db).segs.offset(iseg as isize);
    if !((*seg).finder() >= 0 as libc::c_int) {
        fprintf(stderr,
                b"WARNING: %s:%d: %s: Assertion \"%s\" failed.\n\ti=%d f=%d(%x%x%x) part=%x\n\x00"
                    as *const u8 as *const libc::c_char,
                b"zbar/decoder/databar.c\x00" as *const u8 as
                    *const libc::c_char, 1249 as libc::c_int,
                (*::std::mem::transmute::<&[u8; 21],
                                          &[libc::c_char; 21]>(b"_zbar_decode_databar\x00")).as_ptr(),
                b"seg->finder >= 0\x00" as *const u8 as *const libc::c_char,
                iseg, (*seg).finder(), (*seg).exp() as libc::c_int,
                (*seg).color() as libc::c_int, (*seg).side() as libc::c_int,
                (*seg).partial() as libc::c_int);
        return ZBAR_NONE
    }
    if (*seg).partial() != 0 {
        pair = 0 as *mut databar_segment_t;
        (*seg).set_side(((*seg).side() == 0) as libc::c_int as libc::c_uint)
    } else {
        let mut jseg: libc::c_int = alloc_segment(db);
        pair = (*db).segs.offset(iseg as isize);
        seg = (*db).segs.offset(jseg as isize);
        (*seg).set_finder((*pair).finder());
        (*seg).set_exp((*pair).exp());
        (*seg).set_color((*pair).color());
        (*seg).set_side(((*pair).side() == 0) as libc::c_int as libc::c_uint);
        (*seg).set_partial(0 as libc::c_int as libc::c_uint);
        (*seg).set_count(1 as libc::c_int as libc::c_uint);
        (*seg).width = (*pair).width;
        (*seg).set_epoch((*db).epoch())
    }
    sym = decode_char(dcode, seg, 1 as libc::c_int, 1 as libc::c_int);
    if sym as u64 == 0 {
        (*seg).set_finder(-(1 as libc::c_int));
        if !pair.is_null() {
            (*pair).set_partial(1 as libc::c_int as libc::c_uint)
        }
    } else { (*db).set_epoch((*db).epoch() + 1) }
    return sym;
}
