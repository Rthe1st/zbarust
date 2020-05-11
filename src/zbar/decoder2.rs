use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
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
    /* scan direction: 0=fwd/space, 1=rev/bar */
    /* element offset 0-8 */
    /* character position in symbol */
    /* current character width */
    /* last character width */
    /* initial scan buffer */
    /* int valued configurations */
    /* reset interleaved 2 of 5 specific state */
    /* decode interleaved 2 of 5 symbols */
    #[no_mangle]
    fn _zbar_decode_i25(dcode: *mut zbar_decoder_t) -> zbar_symbol_type_t;
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
    /* scan direction: 0=fwd, 1=rev */
    /* element offset 0-7 */
    /* character position in symbol */
    /* current character width */
    /* last character width */
    /* initial scan buffer */
    /* int valued configurations */
    /* reset Codabar specific state */
    /* decode Codabar symbols */
    #[no_mangle]
    fn _zbar_decode_codabar(dcode: *mut zbar_decoder_t) -> zbar_symbol_type_t;
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
    /* decoder configuration flags */
    /* allocated segments */
    /* current scan */
    /* active segment list */
    /* outstanding character indices */
    /* reset DataBar segment decode state */
    /* reset DataBar accumulated segments */
    /* decode DataBar symbols */
    #[no_mangle]
    fn _zbar_decode_databar(dcode: *mut zbar_decoder_t) -> zbar_symbol_type_t;
    /* reset Code 128 specific state */
    /* decode Code 128 symbols */
    #[no_mangle]
    fn _zbar_decode_code128(dcode: *mut zbar_decoder_t) -> zbar_symbol_type_t;
    /* reset Code 93 specific state */
    /* decode Code 93 symbols */
    #[no_mangle]
    fn _zbar_decode_code93(dcode: *mut zbar_decoder_t) -> zbar_symbol_type_t;
    /* reset Code 39 specific state */
    /* decode Code 39 symbols */
    #[no_mangle]
    fn _zbar_decode_code39(dcode: *mut zbar_decoder_t) -> zbar_symbol_type_t;
    /* decode EAN/UPC symbols */
    #[no_mangle]
    fn _zbar_decode_ean(dcode: *mut zbar_decoder_t) -> zbar_symbol_type_t;
    /* reset QR finder specific state */
    /* find QR Code symbols */
    #[no_mangle]
    fn _zbar_find_qr(dcode: *mut zbar_decoder_t) -> zbar_symbol_type_t;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type zbar_color_e = libc::c_uint;
pub const ZBAR_BAR: zbar_color_e = 1;
pub const ZBAR_SPACE: zbar_color_e = 0;
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
/* * @file
 * ZBar Barcode Reader C API definition
 */
/* * @mainpage
 *
 * interface to the barcode reader is available at several levels.
 * most applications will want to use the high-level interfaces:
 *
 * @section high-level High-Level Interfaces
 *
 * these interfaces wrap all library functionality into an easy-to-use
 * package for a specific toolkit:
 * - the "GTK+ 2.x widget" may be used with GTK GUI applications.  a
 *   Python wrapper is included for PyGtk
 * - the @ref zbar::QZBar "Qt4 widget" may be used with Qt GUI
 *   applications
 * - the Processor interface (in @ref c-processor "C" or @ref
 *   zbar::Processor "C++") adds a scanning window to an application
 *   with no GUI.
 *
 * @section mid-level Intermediate Interfaces
 *
 * building blocks used to construct high-level interfaces:
 * - the ImageScanner (in @ref c-imagescanner "C" or @ref
 *   zbar::ImageScanner "C++") looks for barcodes in a library defined
 *   image object
 * - the Window abstraction (in @ref c-window "C" or @ref
 *   zbar::Window "C++") sinks library images, displaying them on the
 *   platform display
 * - the Video abstraction (in @ref c-video "C" or @ref zbar::Video
 *   "C++") sources library images from a video device
 *
 * @section low-level Low-Level Interfaces
 *
 * direct interaction with barcode scanning and decoding:
 * - the Scanner (in @ref c-scanner "C" or @ref zbar::Scanner "C++")
 *   looks for barcodes in a linear intensity sample stream
 * - the Decoder (in @ref c-decoder "C" or @ref zbar::Decoder "C++")
 *   extracts barcodes from a stream of bar and space widths
 */
/* * @name Global library interfaces */
/*@{*/
/* * "color" of element: bar or space. */
pub type zbar_color_t = zbar_color_e;
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
/* *< light area or space between bars */
/* *< dark area or colored bar segment */
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
/* * decoder configuration options.
 * @since 0.4
 */
pub type zbar_config_t = zbar_config_e;
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
/* *< enable symbology/feature */
/* *< enable check digit when optional */
/* *< return check digit when present */
/* *< enable full ASCII character set */
/* *< number of boolean decoder configs */
/* *< minimum data length for valid decode */
/* *< maximum data length for valid decode */
/* *< required video consistency frames */
/* *< enable scanner to collect position data */
/* *< image scanner vertical scan density */
/* *< image scanner horizontal scan density */
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ean_pass_s {
    pub state: libc::c_schar,
    pub width: libc::c_uint,
    pub raw: [libc::c_uchar; 7],
}
/* scan direction: 0=fwd, 1=rev */
/* element offset 0-8 */
/* character position in symbol */
/* current character width */
/* last character width */
/* int valued configurations */
/* * decoder data handler callback function.
 * called by decoder when new data has just been decoded
 */
pub type zbar_decoder_handler_t
    =
    unsafe extern "C" fn(_: *mut zbar_decoder_t) -> ();
/*@}*/
/*------------------------------------------------------------*/
/* * @name Decoder interface
 * @anchor c-decoder
 * low-level bar width stream decoder interface.
 * identifies symbols and extracts encoded data
 */
/*@{*/
/* * opaque decoder object. */
pub type zbar_decoder_t = zbar_decoder_s;
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
/* module position of w[idx] in symbol */
/*   scan direction reversed */
/*   scanning add-on */
/*   element offset into symbol */
/* width of last character */
/* decode in process */
/* EAN/UPC specific decode state */
/* state of each parallel decode attempt */
/* current holding buffer contents */
/* scan direction */
/* character width */
/* holding buffer */
/* reset EAN/UPC pass specific state */
/* reset all EAN/UPC state */
#[inline]
unsafe extern "C" fn ean_reset(mut ean: *mut ean_decoder_t) {
    ean_new_scan(ean);
    (*ean).right = ZBAR_NONE;
    (*ean).left = (*ean).right;
}
#[inline]
unsafe extern "C" fn ean_new_scan(mut ean: *mut ean_decoder_t) {
    (*ean).pass[1 as libc::c_int as usize].state =
        -(1 as libc::c_int) as libc::c_schar;
    (*ean).pass[0 as libc::c_int as usize].state =
        (*ean).pass[1 as libc::c_int as usize].state;
    (*ean).pass[3 as libc::c_int as usize].state =
        -(1 as libc::c_int) as libc::c_schar;
    (*ean).pass[2 as libc::c_int as usize].state =
        (*ean).pass[3 as libc::c_int as usize].state;
    (*ean).s4 = 0 as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn get_width(mut dcode: *const zbar_decoder_t,
                               mut offset: libc::c_uchar) -> libc::c_uint {
    return (*dcode).w[((*dcode).idx as libc::c_int - offset as libc::c_int &
                           16 as libc::c_int - 1 as libc::c_int) as usize];
}
#[inline]
unsafe extern "C" fn get_color(mut dcode: *const zbar_decoder_t)
 -> libc::c_char {
    return ((*dcode).idx as libc::c_int & 1 as libc::c_int) as libc::c_char;
}
#[inline]
unsafe extern "C" fn databar_new_scan(mut db: *mut databar_decoder_t) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        if (*db).chars[i as usize] as libc::c_int >= 0 as libc::c_int {
            let mut seg: *mut databar_segment_t =
                (*db).segs.offset((*db).chars[i as usize] as libc::c_int as
                                      isize);
            if (*seg).partial() != 0 {
                (*seg).set_finder(-(1 as libc::c_int))
            }
            (*db).chars[i as usize] = -(1 as libc::c_int) as libc::c_schar
        }
        i += 1
    };
}
#[inline]
unsafe extern "C" fn databar_reset(mut db: *mut databar_decoder_t) {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = (*db).csegs() as libc::c_int;
    databar_new_scan(db);
    i = 0 as libc::c_int;
    while i < n {
        let ref mut fresh0 = *(*db).segs.offset(i as isize);
        (*fresh0).set_finder(-(1 as libc::c_int));
        i += 1
    };
}
#[inline]
unsafe extern "C" fn codabar_reset(mut codabar: *mut codabar_decoder_t) {
    (*codabar).set_direction(0 as libc::c_int as libc::c_uint);
    (*codabar).set_element(0 as libc::c_int as libc::c_uint);
    (*codabar).set_character(-(1 as libc::c_int));
    (*codabar).s7 = 0 as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn code39_reset(mut dcode39: *mut code39_decoder_t) {
    (*dcode39).set_direction(0 as libc::c_int as libc::c_uint);
    (*dcode39).set_element(0 as libc::c_int as libc::c_uint);
    (*dcode39).set_character(-(1 as libc::c_int));
    (*dcode39).s9 = 0 as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn code93_reset(mut dcode93: *mut code93_decoder_t) {
    (*dcode93).set_direction(0 as libc::c_int as libc::c_uint);
    (*dcode93).set_element(0 as libc::c_int as libc::c_uint);
    (*dcode93).set_character(-(1 as libc::c_int));
}
#[inline]
unsafe extern "C" fn code128_reset(mut dcode128: *mut code128_decoder_t) {
    (*dcode128).set_direction(0 as libc::c_int as libc::c_uint);
    (*dcode128).set_element(0 as libc::c_int as libc::c_uint);
    (*dcode128).set_character(-(1 as libc::c_int));
    (*dcode128).s6 = 0 as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn qr_finder_reset(mut qrf: *mut qr_finder_t) {
    (*qrf).s5 = 0 as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn i25_reset(mut i25: *mut i25_decoder_t) {
    (*i25).set_direction(0 as libc::c_int as libc::c_uint);
    (*i25).set_element(0 as libc::c_int as libc::c_uint);
    (*i25).set_character(-(1 as libc::c_int));
    (*i25).s10 = 0 as libc::c_int as libc::c_uint;
}
/* check and release shared state lock */
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
/* * constructor. */
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
/* malloc, calloc, free */
/* snprintf */
/* memset, strlen */
#[no_mangle]
pub unsafe extern "C" fn zbar_decoder_create() -> *mut zbar_decoder_t {
    let mut dcode: *mut zbar_decoder_t =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<zbar_decoder_t>() as libc::c_ulong) as
            *mut zbar_decoder_t;
    (*dcode).buf_alloc = 0x20 as libc::c_int as libc::c_uint;
    (*dcode).buf =
        malloc((*dcode).buf_alloc as libc::c_ulong) as *mut libc::c_uchar;
    /* initialize default configs */
    (*dcode).ean.enable = 1 as libc::c_int as libc::c_schar;
    (*dcode).ean.ean13_config =
        ((1 as libc::c_int) << ZBAR_CFG_ENABLE as libc::c_int |
             (1 as libc::c_int) << ZBAR_CFG_EMIT_CHECK as libc::c_int) as
            libc::c_uint;
    (*dcode).ean.ean8_config =
        ((1 as libc::c_int) << ZBAR_CFG_ENABLE as libc::c_int |
             (1 as libc::c_int) << ZBAR_CFG_EMIT_CHECK as libc::c_int) as
            libc::c_uint;
    (*dcode).ean.upca_config =
        ((1 as libc::c_int) << ZBAR_CFG_EMIT_CHECK as libc::c_int) as
            libc::c_uint;
    (*dcode).ean.upce_config =
        ((1 as libc::c_int) << ZBAR_CFG_EMIT_CHECK as libc::c_int) as
            libc::c_uint;
    (*dcode).ean.isbn10_config =
        ((1 as libc::c_int) << ZBAR_CFG_EMIT_CHECK as libc::c_int) as
            libc::c_uint;
    (*dcode).ean.isbn13_config =
        ((1 as libc::c_int) << ZBAR_CFG_EMIT_CHECK as libc::c_int) as
            libc::c_uint;
    (*dcode).i25.config =
        ((1 as libc::c_int) << ZBAR_CFG_ENABLE as libc::c_int) as
            libc::c_uint;
    (*dcode).i25.configs[(ZBAR_CFG_MIN_LEN as libc::c_int -
                              ZBAR_CFG_MIN_LEN as libc::c_int) as usize] =
        6 as libc::c_int;
    (*dcode).databar.config =
        ((1 as libc::c_int) << ZBAR_CFG_ENABLE as libc::c_int |
             (1 as libc::c_int) << ZBAR_CFG_EMIT_CHECK as libc::c_int) as
            libc::c_uint;
    (*dcode).databar.config_exp =
        ((1 as libc::c_int) << ZBAR_CFG_ENABLE as libc::c_int |
             (1 as libc::c_int) << ZBAR_CFG_EMIT_CHECK as libc::c_int) as
            libc::c_uint;
    (*dcode).databar.set_csegs(4 as libc::c_int as libc::c_uint);
    (*dcode).databar.segs =
        calloc(4 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<databar_segment_t>() as libc::c_ulong) as
            *mut databar_segment_t;
    (*dcode).codabar.config =
        ((1 as libc::c_int) << ZBAR_CFG_ENABLE as libc::c_int) as
            libc::c_uint;
    (*dcode).codabar.configs[(ZBAR_CFG_MIN_LEN as libc::c_int -
                                  ZBAR_CFG_MIN_LEN as libc::c_int) as usize] =
        4 as libc::c_int;
    (*dcode).code39.config =
        ((1 as libc::c_int) << ZBAR_CFG_ENABLE as libc::c_int) as
            libc::c_uint;
    (*dcode).code39.configs[(ZBAR_CFG_MIN_LEN as libc::c_int -
                                 ZBAR_CFG_MIN_LEN as libc::c_int) as usize] =
        1 as libc::c_int;
    (*dcode).code93.config =
        ((1 as libc::c_int) << ZBAR_CFG_ENABLE as libc::c_int) as
            libc::c_uint;
    (*dcode).code128.config =
        ((1 as libc::c_int) << ZBAR_CFG_ENABLE as libc::c_int) as
            libc::c_uint;
    (*dcode).qrf.config =
        ((1 as libc::c_int) << ZBAR_CFG_ENABLE as libc::c_int) as
            libc::c_uint;
    zbar_decoder_reset(dcode);
    return dcode;
}
/* * destructor. */
#[no_mangle]
pub unsafe extern "C" fn zbar_decoder_destroy(mut dcode:
                                                  *mut zbar_decoder_t) {
    if !(*dcode).databar.segs.is_null() {
        free((*dcode).databar.segs as *mut libc::c_void);
    }
    if !(*dcode).buf.is_null() { free((*dcode).buf as *mut libc::c_void); }
    free(dcode as *mut libc::c_void);
}
/* * clear all decoder state.
 * any partial symbols are flushed
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_decoder_reset(mut dcode: *mut zbar_decoder_t) {
    memset(dcode as *mut libc::c_void, 0 as libc::c_int,
           (&mut (*dcode).buf_alloc as *mut libc::c_uint as libc::c_long -
                dcode as libc::c_long) as libc::c_ulong);
    ean_reset(&mut (*dcode).ean);
    i25_reset(&mut (*dcode).i25);
    databar_reset(&mut (*dcode).databar);
    codabar_reset(&mut (*dcode).codabar);
    code39_reset(&mut (*dcode).code39);
    code93_reset(&mut (*dcode).code93);
    code128_reset(&mut (*dcode).code128);
    qr_finder_reset(&mut (*dcode).qrf);
}
/* * mark start of a new scan pass.
 * clears any intra-symbol state and resets color to ::ZBAR_SPACE.
 * any partially decoded symbol state is retained
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_decoder_new_scan(mut dcode:
                                                   *mut zbar_decoder_t) {
    /* soft reset decoder */
    memset((*dcode).w.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[libc::c_uint; 16]>() as libc::c_ulong);
    (*dcode).lock = ZBAR_NONE;
    (*dcode).idx = 0 as libc::c_int as libc::c_uchar;
    (*dcode).s6 = 0 as libc::c_int as libc::c_uint;
    ean_new_scan(&mut (*dcode).ean);
    i25_reset(&mut (*dcode).i25);
    databar_new_scan(&mut (*dcode).databar);
    codabar_reset(&mut (*dcode).codabar);
    code39_reset(&mut (*dcode).code39);
    code93_reset(&mut (*dcode).code93);
    code128_reset(&mut (*dcode).code128);
    qr_finder_reset(&mut (*dcode).qrf);
}
/* * retrieve color of @em next element passed to
 * zbar_decode_width(). */
#[no_mangle]
pub unsafe extern "C" fn zbar_decoder_get_color(mut dcode:
                                                    *const zbar_decoder_t)
 -> zbar_color_t {
    return get_color(dcode) as zbar_color_t;
}
/* * retrieve last decoded data.
 * @returns the data string or NULL if no new data available.
 * the returned data buffer is owned by library, contents are only
 * valid between non-0 return from zbar_decode_width and next library
 * call
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_decoder_get_data(mut dcode:
                                                   *const zbar_decoder_t)
 -> *const libc::c_char {
    return (*dcode).buf as *mut libc::c_char;
}
/* * retrieve length of binary data.
 * @returns the length of the decoded data or 0 if no new data
 * available.
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_decoder_get_data_length(mut dcode:
                                                          *const zbar_decoder_t)
 -> libc::c_uint {
    return (*dcode).buflen;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_decoder_get_direction(mut dcode:
                                                        *const zbar_decoder_t)
 -> libc::c_int {
    return (*dcode).direction;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_decoder_set_handler(mut dcode:
                                                      *mut zbar_decoder_t,
                                                  mut handler:
                                                      Option<zbar_decoder_handler_t>)
 -> Option<zbar_decoder_handler_t> {
    let mut result: Option<zbar_decoder_handler_t> = (*dcode).handler;
    (*dcode).handler = handler;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_decoder_set_userdata(mut dcode:
                                                       *mut zbar_decoder_t,
                                                   mut userdata:
                                                       *mut libc::c_void) {
    (*dcode).userdata = userdata;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_decoder_get_userdata(mut dcode:
                                                       *const zbar_decoder_t)
 -> *mut libc::c_void {
    return (*dcode).userdata;
}
/* * retrieve last decoded symbol type.
 * @returns the type or ::ZBAR_NONE if no new data available
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_decoder_get_type(mut dcode:
                                                   *const zbar_decoder_t)
 -> zbar_symbol_type_t {
    return (*dcode).type_0;
}
/* * retrieve modifier flags for the last decoded symbol.
 * @returns a bitmask indicating which characteristics were detected
 * during decoding.
 * @since 0.11
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_decoder_get_modifiers(mut dcode:
                                                        *const zbar_decoder_t)
 -> libc::c_uint {
    return (*dcode).modifiers;
}
/* * process next bar/space width from input stream.
 * the width is in arbitrary relative units.  first value of a scan
 * is ::ZBAR_SPACE width, alternating from there.
 * @returns appropriate symbol type if width completes
 * decode of a symbol (data is available for retrieval)
 * @returns ::ZBAR_PARTIAL as a hint if part of a symbol was decoded
 * @returns ::ZBAR_NONE (0) if no new symbol data is available
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_decode_width(mut dcode: *mut zbar_decoder_t,
                                           mut w: libc::c_uint)
 -> zbar_symbol_type_t {
    let mut tmp: zbar_symbol_type_t = ZBAR_NONE;
    let mut sym: zbar_symbol_type_t = ZBAR_NONE;
    (*dcode).w[((*dcode).idx as libc::c_int &
                    16 as libc::c_int - 1 as libc::c_int) as usize] = w;
    /* update shared character width */
    (*dcode).s6 =
        (*dcode).s6.wrapping_sub(get_width(dcode,
                                           7 as libc::c_int as
                                               libc::c_uchar));
    (*dcode).s6 =
        (*dcode).s6.wrapping_add(get_width(dcode,
                                           1 as libc::c_int as
                                               libc::c_uchar));
    /* each decoder processes width stream in parallel */
    if (*dcode).qrf.config >> ZBAR_CFG_ENABLE as libc::c_int &
           1 as libc::c_int as libc::c_uint != 0 &&
           {
               tmp = _zbar_find_qr(dcode);
               (tmp as libc::c_uint) >
                   ZBAR_PARTIAL as libc::c_int as libc::c_uint
           } {
        sym = tmp
    }
    if (*dcode).ean.enable as libc::c_int != 0 &&
           { tmp = _zbar_decode_ean(dcode); (tmp as libc::c_uint) != 0 } {
        sym = tmp
    }
    if (*dcode).code39.config >> ZBAR_CFG_ENABLE as libc::c_int &
           1 as libc::c_int as libc::c_uint != 0 &&
           {
               tmp = _zbar_decode_code39(dcode);
               (tmp as libc::c_uint) >
                   ZBAR_PARTIAL as libc::c_int as libc::c_uint
           } {
        sym = tmp
    }
    if (*dcode).code93.config >> ZBAR_CFG_ENABLE as libc::c_int &
           1 as libc::c_int as libc::c_uint != 0 &&
           {
               tmp = _zbar_decode_code93(dcode);
               (tmp as libc::c_uint) >
                   ZBAR_PARTIAL as libc::c_int as libc::c_uint
           } {
        sym = tmp
    }
    if (*dcode).code128.config >> ZBAR_CFG_ENABLE as libc::c_int &
           1 as libc::c_int as libc::c_uint != 0 &&
           {
               tmp = _zbar_decode_code128(dcode);
               (tmp as libc::c_uint) >
                   ZBAR_PARTIAL as libc::c_int as libc::c_uint
           } {
        sym = tmp
    }
    if ((*dcode).databar.config | (*dcode).databar.config_exp) >>
           ZBAR_CFG_ENABLE as libc::c_int & 1 as libc::c_int as libc::c_uint
           != 0 &&
           {
               tmp = _zbar_decode_databar(dcode);
               (tmp as libc::c_uint) >
                   ZBAR_PARTIAL as libc::c_int as libc::c_uint
           } {
        sym = tmp
    }
    if (*dcode).codabar.config >> ZBAR_CFG_ENABLE as libc::c_int &
           1 as libc::c_int as libc::c_uint != 0 &&
           {
               tmp = _zbar_decode_codabar(dcode);
               (tmp as libc::c_uint) >
                   ZBAR_PARTIAL as libc::c_int as libc::c_uint
           } {
        sym = tmp
    }
    if (*dcode).i25.config >> ZBAR_CFG_ENABLE as libc::c_int &
           1 as libc::c_int as libc::c_uint != 0 &&
           {
               tmp = _zbar_decode_i25(dcode);
               (tmp as libc::c_uint) >
                   ZBAR_PARTIAL as libc::c_int as libc::c_uint
           } {
        sym = tmp
    }
    (*dcode).idx = (*dcode).idx.wrapping_add(1);
    (*dcode).type_0 = sym;
    if sym as u64 != 0 {
        if (*dcode).lock as libc::c_uint != 0 &&
               sym as libc::c_uint >
                   ZBAR_PARTIAL as libc::c_int as libc::c_uint &&
               sym as libc::c_uint !=
                   ZBAR_QRCODE as libc::c_int as libc::c_uint {
            release_lock(dcode, sym);
        }
        if (*dcode).handler.is_some() {
            (*dcode).handler.expect("non-null function pointer")(dcode);
        }
    }
    return sym;
}
#[inline]
unsafe extern "C" fn decoder_get_configp(mut dcode: *const zbar_decoder_t,
                                         mut sym: zbar_symbol_type_t)
 -> *const libc::c_uint {
    let mut config: *const libc::c_uint = 0 as *const libc::c_uint;
    match sym as libc::c_uint {
        13 => { config = &(*dcode).ean.ean13_config }
        2 => { config = &(*dcode).ean.ean2_config }
        5 => { config = &(*dcode).ean.ean5_config }
        8 => { config = &(*dcode).ean.ean8_config }
        12 => { config = &(*dcode).ean.upca_config }
        9 => { config = &(*dcode).ean.upce_config }
        10 => { config = &(*dcode).ean.isbn10_config }
        14 => { config = &(*dcode).ean.isbn13_config }
        25 => { config = &(*dcode).i25.config }
        34 => { config = &(*dcode).databar.config }
        35 => { config = &(*dcode).databar.config_exp }
        38 => { config = &(*dcode).codabar.config }
        39 => { config = &(*dcode).code39.config }
        93 => { config = &(*dcode).code93.config }
        128 => { config = &(*dcode).code128.config }
        64 => { config = &(*dcode).qrf.config }
        _ => { config = 0 as *const libc::c_uint }
    }
    return config;
}
/* * retrieve symbology boolean config settings.
 * @returns a bitmask indicating which configs are currently set for the
 * specified symbology.
 * @since 0.11
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_decoder_get_configs(mut dcode:
                                                      *const zbar_decoder_t,
                                                  mut sym: zbar_symbol_type_t)
 -> libc::c_uint {
    let mut config: *const libc::c_uint = decoder_get_configp(dcode, sym);
    if config.is_null() { return 0 as libc::c_int as libc::c_uint }
    return *config;
}
#[inline]
unsafe extern "C" fn decoder_set_config_bool(mut dcode: *mut zbar_decoder_t,
                                             mut sym: zbar_symbol_type_t,
                                             mut cfg: zbar_config_t,
                                             mut val: libc::c_int)
 -> libc::c_int {
    let mut config: *mut libc::c_uint =
        decoder_get_configp(dcode, sym) as *mut libc::c_void as
            *mut libc::c_uint;
    if config.is_null() ||
           cfg as libc::c_uint >= ZBAR_CFG_NUM as libc::c_int as libc::c_uint
       {
        return 1 as libc::c_int
    }
    if val == 0 {
        *config &=
            !((1 as libc::c_int) << cfg as libc::c_uint) as libc::c_uint
    } else if val == 1 as libc::c_int {
        *config |= ((1 as libc::c_int) << cfg as libc::c_uint) as libc::c_uint
    } else { return 1 as libc::c_int }
    (*dcode).ean.enable =
        (((*dcode).ean.ean13_config | (*dcode).ean.ean2_config |
              (*dcode).ean.ean5_config | (*dcode).ean.ean8_config |
              (*dcode).ean.upca_config | (*dcode).ean.upce_config |
              (*dcode).ean.isbn10_config | (*dcode).ean.isbn13_config) >>
             ZBAR_CFG_ENABLE as libc::c_int &
             1 as libc::c_int as libc::c_uint) as libc::c_schar;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn decoder_set_config_int(mut dcode: *mut zbar_decoder_t,
                                            mut sym: zbar_symbol_type_t,
                                            mut cfg: zbar_config_t,
                                            mut val: libc::c_int)
 -> libc::c_int {
    match sym as libc::c_uint {
        25 => {
            (*dcode).i25.configs[(cfg as
                                      libc::c_uint).wrapping_sub(ZBAR_CFG_MIN_LEN
                                                                     as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
                                     as usize] = val
        }
        38 => {
            (*dcode).codabar.configs[(cfg as
                                          libc::c_uint).wrapping_sub(ZBAR_CFG_MIN_LEN
                                                                         as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint)
                                         as usize] = val
        }
        39 => {
            (*dcode).code39.configs[(cfg as
                                         libc::c_uint).wrapping_sub(ZBAR_CFG_MIN_LEN
                                                                        as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)
                                        as usize] = val
        }
        93 => {
            (*dcode).code93.configs[(cfg as
                                         libc::c_uint).wrapping_sub(ZBAR_CFG_MIN_LEN
                                                                        as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)
                                        as usize] = val
        }
        128 => {
            (*dcode).code128.configs[(cfg as
                                          libc::c_uint).wrapping_sub(ZBAR_CFG_MIN_LEN
                                                                         as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint)
                                         as usize] = val
        }
        _ => { return 1 as libc::c_int }
    }
    return 0 as libc::c_int;
}
/* * set config for indicated symbology (0 for all) to specified value.
 * @returns 0 for success, non-0 for failure (config does not apply to
 * specified symbology, or value out of range)
 * @since 0.4
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_decoder_set_config(mut dcode:
                                                     *mut zbar_decoder_t,
                                                 mut sym: zbar_symbol_type_t,
                                                 mut cfg: zbar_config_t,
                                                 mut val: libc::c_int)
 -> libc::c_int {
    if sym as libc::c_uint == ZBAR_NONE as libc::c_int as libc::c_uint {
        static mut all: [zbar_symbol_type_t; 18] =
            [ZBAR_EAN13, ZBAR_EAN2, ZBAR_EAN5, ZBAR_EAN8, ZBAR_UPCA,
             ZBAR_UPCE, ZBAR_ISBN10, ZBAR_ISBN13, ZBAR_I25, ZBAR_DATABAR,
             ZBAR_DATABAR_EXP, ZBAR_CODABAR, ZBAR_CODE39, ZBAR_CODE93,
             ZBAR_CODE128, ZBAR_QRCODE, ZBAR_PDF417, ZBAR_NONE];
        let mut symp: *const zbar_symbol_type_t =
            0 as *const zbar_symbol_type_t;
        symp = all.as_ptr();
        while *symp as u64 != 0 {
            zbar_decoder_set_config(dcode, *symp, cfg, val);
            symp = symp.offset(1)
        }
        return 0 as libc::c_int
    }
    if cfg as libc::c_uint >= 0 as libc::c_int as libc::c_uint &&
           (cfg as libc::c_uint) < ZBAR_CFG_NUM as libc::c_int as libc::c_uint
       {
        return decoder_set_config_bool(dcode, sym, cfg, val)
    } else if cfg as libc::c_uint >=
                  ZBAR_CFG_MIN_LEN as libc::c_int as libc::c_uint &&
                  cfg as libc::c_uint <=
                      ZBAR_CFG_MAX_LEN as libc::c_int as libc::c_uint {
        return decoder_set_config_int(dcode, sym, cfg, val)
    } else { return 1 as libc::c_int };
}
static mut decoder_dump: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut decoder_dumplen: libc::c_uint = 0 as libc::c_int as libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn _zbar_decoder_buf_dump(mut buf: *mut libc::c_uchar,
                                                mut buflen: libc::c_uint)
 -> *const libc::c_char {
    let mut dumplen: libc::c_int =
        buflen.wrapping_mul(3 as libc::c_int as
                                libc::c_uint).wrapping_add(12 as libc::c_int
                                                               as
                                                               libc::c_uint)
            as libc::c_int;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    if decoder_dump.is_null() || dumplen as libc::c_uint > decoder_dumplen {
        if !decoder_dump.is_null() {
            free(decoder_dump as *mut libc::c_void);
        }
        decoder_dump = malloc(dumplen as libc::c_ulong) as *mut libc::c_char;
        decoder_dumplen = dumplen as libc::c_uint
    }
    p =
        decoder_dump.offset(snprintf(decoder_dump,
                                     12 as libc::c_int as libc::c_ulong,
                                     b"buf[%04x]=\x00" as *const u8 as
                                         *const libc::c_char,
                                     (if buflen >
                                             0xffff as libc::c_int as
                                                 libc::c_uint {
                                          0xffff as libc::c_int as
                                              libc::c_uint
                                      } else { buflen })) as isize);
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < buflen {
        p =
            p.offset(snprintf(p, 4 as libc::c_int as libc::c_ulong,
                              b"%s%02x\x00" as *const u8 as
                                  *const libc::c_char,
                              if i != 0 {
                                  b" \x00" as *const u8 as *const libc::c_char
                              } else {
                                  b"\x00" as *const u8 as *const libc::c_char
                              }, *buf.offset(i as isize) as libc::c_int) as
                         isize);
        i += 1
    }
    return decoder_dump;
}
