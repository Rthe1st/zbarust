use ::libc;
extern "C" {
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
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
    #[no_mangle]
    fn _zbar_refcnt(cnt: *mut refcnt_t, delta: libc::c_int) -> libc::c_int;
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
/* *< unable to determine orientation */
/* *< upright, read left to right */
/* *< sideways, read top to bottom */
/* *< upside-down, read right to left */
/* *< sideways, read bottom to top */
/* * decoder configuration options.
 * @since 0.4
 */
pub type zbar_config_t = zbar_config_e;
pub type zbar_modifier_e = libc::c_uint;
pub const ZBAR_MOD_NUM: zbar_modifier_e = 2;
pub const ZBAR_MOD_AIM: zbar_modifier_e = 1;
pub const ZBAR_MOD_GS1: zbar_modifier_e = 0;
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
/* * decoder symbology modifier flags.
 * @since 0.11
 */
pub type zbar_modifier_t = zbar_modifier_e;
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
/* * barcode tagged as GS1 (EAN.UCC) reserved
     * (eg, FNC1 before first data character).
     * data may be parsed as a sequence of GS1 AIs
     */
/* * barcode tagged as AIM reserved
     * (eg, FNC1 after first character or digit pair)
     */
/* * number of modifiers */
/*@}*/
pub type zbar_symbol_t = zbar_symbol_s;
pub type refcnt_t = libc::c_int;
pub type point_t = point_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point_s {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[inline]
unsafe extern "C" fn _zbar_symbol_refcnt(mut sym: *mut zbar_symbol_t,
                                         mut delta: libc::c_int) {
    if _zbar_refcnt(&mut (*sym).refcnt, delta) == 0 &&
           delta <= 0 as libc::c_int {
        _zbar_symbol_free(sym);
    };
}
/* * retrieve string name for symbol encoding.
 * @param sym symbol type encoding
 * @returns the static string name for the specified symbol type,
 * or "UNKNOWN" if the encoding is not recognized
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
#[no_mangle]
pub unsafe extern "C" fn zbar_get_symbol_name(mut sym: zbar_symbol_type_t)
 -> *const libc::c_char {
    match sym as libc::c_uint & ZBAR_SYMBOL as libc::c_int as libc::c_uint {
        2 => { return b"EAN-2\x00" as *const u8 as *const libc::c_char }
        5 => { return b"EAN-5\x00" as *const u8 as *const libc::c_char }
        8 => { return b"EAN-8\x00" as *const u8 as *const libc::c_char }
        9 => { return b"UPC-E\x00" as *const u8 as *const libc::c_char }
        10 => { return b"ISBN-10\x00" as *const u8 as *const libc::c_char }
        12 => { return b"UPC-A\x00" as *const u8 as *const libc::c_char }
        13 => { return b"EAN-13\x00" as *const u8 as *const libc::c_char }
        14 => { return b"ISBN-13\x00" as *const u8 as *const libc::c_char }
        15 => { return b"COMPOSITE\x00" as *const u8 as *const libc::c_char }
        25 => { return b"I2/5\x00" as *const u8 as *const libc::c_char }
        34 => { return b"DataBar\x00" as *const u8 as *const libc::c_char }
        35 => {
            return b"DataBar-Exp\x00" as *const u8 as *const libc::c_char
        }
        38 => { return b"Codabar\x00" as *const u8 as *const libc::c_char }
        39 => { return b"CODE-39\x00" as *const u8 as *const libc::c_char }
        93 => { return b"CODE-93\x00" as *const u8 as *const libc::c_char }
        128 => { return b"CODE-128\x00" as *const u8 as *const libc::c_char }
        57 => { return b"PDF417\x00" as *const u8 as *const libc::c_char }
        64 => { return b"QR-Code\x00" as *const u8 as *const libc::c_char }
        _ => { return b"UNKNOWN\x00" as *const u8 as *const libc::c_char }
    };
}
/* * retrieve string name for addon encoding.
 * @param sym symbol type encoding
 * @returns static string name for any addon, or the empty string
 * if no addons were decoded
 * @deprecated in 0.11
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_get_addon_name(mut sym: zbar_symbol_type_t)
 -> *const libc::c_char {
    return b"\x00" as *const u8 as *const libc::c_char;
}
/* * retrieve string name for configuration setting.
 * @param config setting to name
 * @returns static string name for config,
 * or the empty string if value is not a known config
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_get_config_name(mut cfg: zbar_config_t)
 -> *const libc::c_char {
    match cfg as libc::c_uint {
        0 => { return b"ENABLE\x00" as *const u8 as *const libc::c_char }
        1 => { return b"ADD_CHECK\x00" as *const u8 as *const libc::c_char }
        2 => { return b"EMIT_CHECK\x00" as *const u8 as *const libc::c_char }
        3 => { return b"ASCII\x00" as *const u8 as *const libc::c_char }
        32 => { return b"MIN_LEN\x00" as *const u8 as *const libc::c_char }
        33 => { return b"MAX_LEN\x00" as *const u8 as *const libc::c_char }
        64 => {
            return b"UNCERTAINTY\x00" as *const u8 as *const libc::c_char
        }
        128 => { return b"POSITION\x00" as *const u8 as *const libc::c_char }
        256 => { return b"X_DENSITY\x00" as *const u8 as *const libc::c_char }
        257 => { return b"Y_DENSITY\x00" as *const u8 as *const libc::c_char }
        _ => { return b"\x00" as *const u8 as *const libc::c_char }
    };
}
/* * retrieve string name for modifier.
 * @param modifier flag to name
 * @returns static string name for modifier,
 * or the empty string if the value is not a known flag
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_get_modifier_name(mut mod_0: zbar_modifier_t)
 -> *const libc::c_char {
    match mod_0 as libc::c_uint {
        0 => { return b"GS1\x00" as *const u8 as *const libc::c_char }
        1 => { return b"AIM\x00" as *const u8 as *const libc::c_char }
        _ => { return b"\x00" as *const u8 as *const libc::c_char }
    };
}
/* * retrieve string name for orientation.
 * @param orientation orientation encoding
 * @returns the static string name for the specified orientation,
 * or "UNKNOWN" if the orientation is not recognized
 * @since 0.11
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_get_orientation_name(mut orient:
                                                       zbar_orientation_t)
 -> *const libc::c_char {
    match orient as libc::c_int {
        0 => { return b"UP\x00" as *const u8 as *const libc::c_char }
        1 => { return b"RIGHT\x00" as *const u8 as *const libc::c_char }
        2 => { return b"DOWN\x00" as *const u8 as *const libc::c_char }
        3 => { return b"LEFT\x00" as *const u8 as *const libc::c_char }
        _ => { return b"UNKNOWN\x00" as *const u8 as *const libc::c_char }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _zbar_get_symbol_hash(mut sym: zbar_symbol_type_t)
 -> libc::c_int {
    static mut hash: [libc::c_schar; 32] =
        [0 as libc::c_int as libc::c_schar,
         0x1 as libc::c_int as libc::c_schar,
         0x10 as libc::c_int as libc::c_schar,
         0x11 as libc::c_int as libc::c_schar,
         -(1 as libc::c_int) as libc::c_schar,
         0x11 as libc::c_int as libc::c_schar,
         0x16 as libc::c_int as libc::c_schar,
         0xc as libc::c_int as libc::c_schar,
         0x5 as libc::c_int as libc::c_schar,
         0x6 as libc::c_int as libc::c_schar,
         0x8 as libc::c_int as libc::c_schar,
         -(1 as libc::c_int) as libc::c_schar,
         0x4 as libc::c_int as libc::c_schar,
         0x3 as libc::c_int as libc::c_schar,
         0x7 as libc::c_int as libc::c_schar,
         0x12 as libc::c_int as libc::c_schar,
         -(1 as libc::c_int) as libc::c_schar,
         -(1 as libc::c_int) as libc::c_schar,
         -(1 as libc::c_int) as libc::c_schar,
         -(1 as libc::c_int) as libc::c_schar,
         -(1 as libc::c_int) as libc::c_schar,
         -(1 as libc::c_int) as libc::c_schar,
         -(1 as libc::c_int) as libc::c_schar,
         0x2 as libc::c_int as libc::c_schar,
         -(1 as libc::c_int) as libc::c_schar,
         0 as libc::c_int as libc::c_schar,
         0x12 as libc::c_int as libc::c_schar,
         0xc as libc::c_int as libc::c_schar,
         0xb as libc::c_int as libc::c_schar,
         0x1d as libc::c_int as libc::c_schar,
         0xa as libc::c_int as libc::c_schar,
         0 as libc::c_int as libc::c_schar];
    let mut g0: libc::c_int =
        hash[(sym as libc::c_uint & 0x1f as libc::c_int as libc::c_uint) as
                 usize] as libc::c_int;
    let mut g1: libc::c_int =
        hash[(!(sym as libc::c_uint >> 4 as libc::c_int) &
                  0x1f as libc::c_int as libc::c_uint) as usize] as
            libc::c_int;
    if g0 >= 0 as libc::c_int && g1 >= 0 as libc::c_int {
    } else {
        __assert_fail(b"g0 >= 0 && g1 >= 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"zbar/symbol.c\x00" as *const u8 as
                          *const libc::c_char,
                      109 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 46],
                                                &[libc::c_char; 46]>(b"int _zbar_get_symbol_hash(zbar_symbol_type_t)\x00")).as_ptr());
    }
    if g0 < 0 as libc::c_int || g1 < 0 as libc::c_int {
        return 0 as libc::c_int
    }
    return g0 + g1 & 0x1f as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _zbar_symbol_free(mut sym: *mut zbar_symbol_t) {
    if !(*sym).syms.is_null() {
        zbar_symbol_set_ref((*sym).syms, -(1 as libc::c_int));
        (*sym).syms = 0 as *mut zbar_symbol_set_t
    }
    if !(*sym).pts.is_null() { free((*sym).pts as *mut libc::c_void); }
    if (*sym).data_alloc != 0 && !(*sym).data.is_null() {
        free((*sym).data as *mut libc::c_void);
    }
    free(sym as *mut libc::c_void);
}
/*------------------------------------------------------------*/
/* * @name Symbol interface
 * decoded barcode symbol result object.  stores type, data, and image
 * location of decoded symbol.  all memory is owned by the library
 */
/*@{*/
/* * @typedef zbar_symbol_t
 * opaque decoded symbol object.
 */
/* * symbol reference count manipulation.
 * increment the reference count when you store a new reference to the
 * symbol.  decrement when the reference is no longer used.  do not
 * refer to the symbol once the count is decremented and the
 * containing image has been recycled or destroyed.
 * @note the containing image holds a reference to the symbol, so you
 * only need to use this if you keep a symbol after the image has been
 * destroyed or reused.
 * @since 0.9
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_symbol_ref(mut sym: *const zbar_symbol_t,
                                         mut refs: libc::c_int) {
    let mut ncsym: *mut zbar_symbol_t = sym as *mut zbar_symbol_t;
    _zbar_symbol_refcnt(ncsym, refs);
}
/* * retrieve type of decoded symbol.
 * @returns the symbol type
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_symbol_get_type(mut sym: *const zbar_symbol_t)
 -> zbar_symbol_type_t {
    return (*sym).type_0;
}
/* * retrieve symbology boolean config settings.
 * @returns a bitmask indicating which configs were set for the detected
 * symbology during decoding.
 * @since 0.11
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_symbol_get_configs(mut sym:
                                                     *const zbar_symbol_t)
 -> libc::c_uint {
    return (*sym).configs;
}
/* * retrieve symbology modifier flag settings.
 * @returns a bitmask indicating which characteristics were detected
 * during decoding.
 * @since 0.11
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_symbol_get_modifiers(mut sym:
                                                       *const zbar_symbol_t)
 -> libc::c_uint {
    return (*sym).modifiers;
}
/* * retrieve data decoded from symbol.
 * @returns the data string
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_symbol_get_data(mut sym: *const zbar_symbol_t)
 -> *const libc::c_char {
    return (*sym).data;
}
/* * retrieve length of binary data.
 * @returns the length of the decoded data
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_symbol_get_data_length(mut sym:
                                                         *const zbar_symbol_t)
 -> libc::c_uint {
    return (*sym).datalen;
}
/* * retrieve current cache count.  when the cache is enabled for the
 * image_scanner this provides inter-frame reliability and redundancy
 * information for video streams.
 * @returns < 0 if symbol is still uncertain.
 * @returns 0 if symbol is newly verified.
 * @returns > 0 for duplicate symbols
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_symbol_get_count(mut sym: *const zbar_symbol_t)
 -> libc::c_int {
    return (*sym).cache_count;
}
/* * retrieve a symbol confidence metric.
 * @returns an unscaled, relative quantity: larger values are better
 * than smaller values, where "large" and "small" are application
 * dependent.
 * @note expect the exact definition of this quantity to change as the
 * metric is refined.  currently, only the ordered relationship
 * between two values is defined and will remain stable in the future
 * @since 0.9
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_symbol_get_quality(mut sym:
                                                     *const zbar_symbol_t)
 -> libc::c_int {
    return (*sym).quality;
}
/* * retrieve the number of points in the location polygon.  the
 * location polygon defines the image area that the symbol was
 * extracted from.
 * @returns the number of points in the location polygon
 * @note this is currently not a polygon, but the scan locations
 * where the symbol was decoded
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_symbol_get_loc_size(mut sym:
                                                      *const zbar_symbol_t)
 -> libc::c_uint {
    return (*sym).npts;
}
/* * retrieve location polygon x-coordinates.
 * points are specified by 0-based index.
 * @returns the x-coordinate for a point in the location polygon.
 * @returns -1 if index is out of range
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_symbol_get_loc_x(mut sym: *const zbar_symbol_t,
                                               mut idx: libc::c_uint)
 -> libc::c_int {
    if idx < (*sym).npts {
        return (*(*sym).pts.offset(idx as isize)).x
    } else { return -(1 as libc::c_int) };
}
/* * retrieve location polygon y-coordinates.
 * points are specified by 0-based index.
 * @returns the y-coordinate for a point in the location polygon.
 * @returns -1 if index is out of range
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_symbol_get_loc_y(mut sym: *const zbar_symbol_t,
                                               mut idx: libc::c_uint)
 -> libc::c_int {
    if idx < (*sym).npts {
        return (*(*sym).pts.offset(idx as isize)).y
    } else { return -(1 as libc::c_int) };
}
/* * retrieve general orientation of decoded symbol.
 * @returns a coarse, axis-aligned indication of symbol orientation or
 * ::ZBAR_ORIENT_UNKNOWN if unknown
 * @since 0.11
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_symbol_get_orientation(mut sym:
                                                         *const zbar_symbol_t)
 -> zbar_orientation_t {
    return (*sym).orient;
}
/* * iterate the set to which this symbol belongs (there can be only one).
 * @returns the next symbol in the set, or
 * @returns NULL when no more results are available
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_symbol_next(mut sym: *const zbar_symbol_t)
 -> *const zbar_symbol_t {
    return if !sym.is_null() { (*sym).next } else { 0 as *mut zbar_symbol_t };
}
/* * retrieve components of a composite result.
 * @returns the symbol set containing the components
 * @returns NULL if the symbol is already a physical symbol
 * @since 0.10
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_symbol_get_components(mut sym:
                                                        *const zbar_symbol_t)
 -> *const zbar_symbol_set_t {
    return (*sym).syms;
}
/* * iterate components of a composite result.
 * @returns the first physical component symbol of a composite result
 * @returns NULL if the symbol is already a physical symbol
 * @since 0.10
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_symbol_first_component(mut sym:
                                                         *const zbar_symbol_t)
 -> *const zbar_symbol_t {
    return if !sym.is_null() && !(*sym).syms.is_null() {
               (*(*sym).syms).head
           } else { 0 as *mut zbar_symbol_t };
}
#[no_mangle]
pub unsafe extern "C" fn base64_encode(mut dst: *mut libc::c_char,
                                       mut src: *const libc::c_char,
                                       mut srclen: libc::c_uint)
 -> libc::c_uint {
    static mut alphabet: [libc::c_char; 65] =
        [65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81,
         82, 83, 84, 85, 86, 87, 88, 89, 90, 97, 98, 99, 100, 101, 102, 103,
         104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117,
         118, 119, 120, 121, 122, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 43,
         47, 0];
    let mut start: *mut libc::c_char = dst;
    let mut nline: libc::c_int = 19 as libc::c_int;
    while srclen != 0 {
        let fresh0 = src;
        src = src.offset(1);
        let mut buf: libc::c_uint =
            ((*fresh0 as libc::c_int) << 16 as libc::c_int) as libc::c_uint;
        if srclen > 1 as libc::c_int as libc::c_uint {
            let fresh1 = src;
            src = src.offset(1);
            buf |=
                ((*fresh1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint
        }
        if srclen > 2 as libc::c_int as libc::c_uint {
            let fresh2 = src;
            src = src.offset(1);
            buf |= *fresh2 as libc::c_uint
        }
        let fresh3 = dst;
        dst = dst.offset(1);
        *fresh3 =
            alphabet[(buf >> 18 as libc::c_int &
                          0x3f as libc::c_int as libc::c_uint) as usize];
        let fresh4 = dst;
        dst = dst.offset(1);
        *fresh4 =
            alphabet[(buf >> 12 as libc::c_int &
                          0x3f as libc::c_int as libc::c_uint) as usize];
        let fresh5 = dst;
        dst = dst.offset(1);
        *fresh5 =
            if srclen > 1 as libc::c_int as libc::c_uint {
                alphabet[(buf >> 6 as libc::c_int &
                              0x3f as libc::c_int as libc::c_uint) as usize]
                    as libc::c_int
            } else { '=' as i32 } as libc::c_char;
        let fresh6 = dst;
        dst = dst.offset(1);
        *fresh6 =
            if srclen > 2 as libc::c_int as libc::c_uint {
                alphabet[(buf & 0x3f as libc::c_int as libc::c_uint) as usize]
                    as libc::c_int
            } else { '=' as i32 } as libc::c_char;
        if srclen < 3 as libc::c_int as libc::c_uint { break ; }
        nline -= 1;
        if nline == 0 {
            let fresh7 = dst;
            dst = dst.offset(1);
            *fresh7 = '\n' as i32 as libc::c_char;
            nline = 19 as libc::c_int
        }
        srclen = srclen.wrapping_sub(3 as libc::c_int as libc::c_uint)
    }
    let fresh8 = dst;
    dst = dst.offset(1);
    *fresh8 = '\n' as i32 as libc::c_char;
    let fresh9 = dst;
    dst = dst.offset(1);
    *fresh9 = '\u{0}' as i32 as libc::c_char;
    return (dst.wrapping_offset_from(start) as libc::c_long -
                1 as libc::c_int as libc::c_long) as libc::c_uint;
}
/* * print XML symbol element representation to user result buffer.
 * @see http://zbar.sourceforge.net/2008/barcode.xsd for the schema.
 * @param symbol is the symbol to print
 * @param buffer is the inout result pointer, it will be reallocated
 * with a larger size if necessary.
 * @param buflen is inout length of the result buffer.
 * @returns the buffer pointer
 * @since 0.6
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_symbol_xml(mut sym: *const zbar_symbol_t,
                                         mut buf: *mut *mut libc::c_char,
                                         mut len: *mut libc::c_uint)
 -> *mut libc::c_char {
    let mut datalen: libc::c_uint = 0;
    let mut maxlen: libc::c_uint = 0;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut type_0: *const libc::c_char = zbar_get_symbol_name((*sym).type_0);
    let mut orient: *const libc::c_char =
        zbar_get_orientation_name((*sym).orient);
    /* check for binary data */
    let mut data: *mut libc::c_uchar = (*sym).data as *mut libc::c_uchar;
    let mut binary: libc::c_char =
        (*data.offset(0 as libc::c_int as isize) as libc::c_int ==
             0xff as libc::c_int &&
             *data.offset(1 as libc::c_int as isize) as libc::c_int ==
                 0xfe as libc::c_int ||
             *data.offset(0 as libc::c_int as isize) as libc::c_int ==
                 0xfe as libc::c_int &&
                 *data.offset(1 as libc::c_int as isize) as libc::c_int ==
                     0xff as libc::c_int ||
             strncmp((*sym).data,
                     b"<?xml\x00" as *const u8 as *const libc::c_char,
                     5 as libc::c_int as libc::c_ulong) == 0) as libc::c_int
            as libc::c_char;
    i = 0 as libc::c_int;
    while binary == 0 && (i as libc::c_uint) < (*sym).datalen {
        let mut c: libc::c_uchar =
            *(*sym).data.offset(i as isize) as libc::c_uchar;
        binary =
            ((c as libc::c_int) < 0x20 as libc::c_int &&
                 !(0x2600 as libc::c_int) >> c as libc::c_int &
                     1 as libc::c_int != 0 ||
                 c as libc::c_int >= 0x7f as libc::c_int &&
                     (c as libc::c_int) < 0xa0 as libc::c_int ||
                 c as libc::c_int == ']' as i32 &&
                     ((i + 2 as libc::c_int) as libc::c_uint) < (*sym).datalen
                     &&
                     *(*sym).data.offset((i + 1 as libc::c_int) as isize) as
                         libc::c_int == ']' as i32 &&
                     *(*sym).data.offset((i + 2 as libc::c_int) as isize) as
                         libc::c_int == '>' as i32) as libc::c_int as
                libc::c_char;
        i += 1
    }
    datalen = strlen((*sym).data) as libc::c_uint;
    if binary != 0 {
        datalen =
            (*sym).datalen.wrapping_add(2 as libc::c_int as
                                            libc::c_uint).wrapping_div(3 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint).wrapping_mul(4
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          as
                                                                                                          libc::c_uint).wrapping_add((*sym).datalen.wrapping_div(57
                                                                                                                                                                     as
                                                                                                                                                                     libc::c_int
                                                                                                                                                                     as
                                                                                                                                                                     libc::c_uint)).wrapping_add(3
                                                                                                                                                                                                     as
                                                                                                                                                                                                     libc::c_int
                                                                                                                                                                                                     as
                                                                                                                                                                                                     libc::c_uint)
    }
    maxlen =
        (256 as libc::c_int as
             libc::c_ulong).wrapping_add(strlen(type_0)).wrapping_add(strlen(orient)).wrapping_add(datalen
                                                                                                       as
                                                                                                       libc::c_ulong).wrapping_add(10
                                                                                                                                       as
                                                                                                                                       libc::c_int
                                                                                                                                       as
                                                                                                                                       libc::c_ulong).wrapping_add(1
                                                                                                                                                                       as
                                                                                                                                                                       libc::c_int
                                                                                                                                                                       as
                                                                                                                                                                       libc::c_ulong)
            as libc::c_uint;
    let mut mods: libc::c_uint = (*sym).modifiers;
    if mods != 0 {
        maxlen =
            maxlen.wrapping_add((5 as libc::c_int *
                                     ZBAR_MOD_NUM as libc::c_int) as
                                    libc::c_uint)
    }
    let mut cfgs: libc::c_uint =
        (*sym).configs &
            !((1 as libc::c_int) << ZBAR_CFG_ENABLE as libc::c_int) as
                libc::c_uint;
    if cfgs != 0 {
        maxlen =
            maxlen.wrapping_add((10 as libc::c_int *
                                     ZBAR_CFG_NUM as libc::c_int) as
                                    libc::c_uint)
    }
    if binary != 0 {
        maxlen = maxlen.wrapping_add(10 as libc::c_int as libc::c_uint)
    }
    if (*buf).is_null() || *len < maxlen {
        if !(*buf).is_null() { free(*buf as *mut libc::c_void); }
        *buf = malloc(maxlen as libc::c_ulong) as *mut libc::c_char;
        /* FIXME check OOM */
        *len = maxlen
    }
    static mut _st: *const libc::c_char =
        b"<symbol type=\'%s\' quality=\'%d\' orientation=\'%s\'\x00" as
            *const u8 as *const libc::c_char;
    i =
        snprintf((*buf).offset(n as isize),
                 maxlen.wrapping_sub(n as libc::c_uint) as libc::c_ulong, _st,
                 type_0, (*sym).quality, orient);
    if i > 0 as libc::c_int {
    } else {
        __assert_fail(b"i > 0\x00" as *const u8 as *const libc::c_char,
                      b"zbar/symbol.c\x00" as *const u8 as
                          *const libc::c_char,
                      318 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 70],
                                                &[libc::c_char; 70]>(b"char *zbar_symbol_xml(const zbar_symbol_t *, char **, unsigned int *)\x00")).as_ptr());
    }
    n += i;
    if n as libc::c_uint <= maxlen {
    } else {
        __assert_fail(b"n <= maxlen\x00" as *const u8 as *const libc::c_char,
                      b"zbar/symbol.c\x00" as *const u8 as
                          *const libc::c_char,
                      318 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 70],
                                                &[libc::c_char; 70]>(b"char *zbar_symbol_xml(const zbar_symbol_t *, char **, unsigned int *)\x00")).as_ptr());
    }
    if mods != 0 {
        let mut j: libc::c_int = 0;
        static mut _st_0: *const libc::c_char =
            b" modifiers=\'\x00" as *const u8 as *const libc::c_char;
        i = strlen(_st_0) as libc::c_int;
        memcpy((*buf).offset(n as isize) as *mut libc::c_void,
               _st_0 as *const libc::c_void,
               (i + 1 as libc::c_int) as libc::c_ulong);
        n += i;
        if n as libc::c_uint <= maxlen {
        } else {
            __assert_fail(b"n <= maxlen\x00" as *const u8 as
                              *const libc::c_char,
                          b"zbar/symbol.c\x00" as *const u8 as
                              *const libc::c_char,
                          322 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 70],
                                                    &[libc::c_char; 70]>(b"char *zbar_symbol_xml(const zbar_symbol_t *, char **, unsigned int *)\x00")).as_ptr());
        }
        j = 0 as libc::c_int;
        while mods != 0 && j < ZBAR_MOD_NUM as libc::c_int {
            if mods & 1 as libc::c_int as libc::c_uint != 0 {
                static mut _st_1: *const libc::c_char =
                    b"%s \x00" as *const u8 as *const libc::c_char;
                i =
                    snprintf((*buf).offset(n as isize),
                             maxlen.wrapping_sub(n as libc::c_uint) as
                                 libc::c_ulong, _st_1,
                             zbar_get_modifier_name(j as zbar_modifier_t));
                if i > 0 as libc::c_int {
                } else {
                    __assert_fail(b"i > 0\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"zbar/symbol.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  325 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 70],
                                                            &[libc::c_char; 70]>(b"char *zbar_symbol_xml(const zbar_symbol_t *, char **, unsigned int *)\x00")).as_ptr());
                }
                n += i;
                if n as libc::c_uint <= maxlen {
                } else {
                    __assert_fail(b"n <= maxlen\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"zbar/symbol.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  325 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 70],
                                                            &[libc::c_char; 70]>(b"char *zbar_symbol_xml(const zbar_symbol_t *, char **, unsigned int *)\x00")).as_ptr());
                }
            }
            j += 1;
            mods >>= 1 as libc::c_int
        }
        /* cleanup trailing space */
        n -= 1;
        static mut _st_2: *const libc::c_char =
            b"\'\x00" as *const u8 as *const libc::c_char;
        i = strlen(_st_2) as libc::c_int;
        memcpy((*buf).offset(n as isize) as *mut libc::c_void,
               _st_2 as *const libc::c_void,
               (i + 1 as libc::c_int) as libc::c_ulong);
        n += i;
        if n as libc::c_uint <= maxlen {
        } else {
            __assert_fail(b"n <= maxlen\x00" as *const u8 as
                              *const libc::c_char,
                          b"zbar/symbol.c\x00" as *const u8 as
                              *const libc::c_char,
                          328 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 70],
                                                    &[libc::c_char; 70]>(b"char *zbar_symbol_xml(const zbar_symbol_t *, char **, unsigned int *)\x00")).as_ptr());
        }
    }
    if cfgs != 0 {
        let mut j_0: libc::c_int = 0;
        static mut _st_3: *const libc::c_char =
            b" configs=\'\x00" as *const u8 as *const libc::c_char;
        i = strlen(_st_3) as libc::c_int;
        memcpy((*buf).offset(n as isize) as *mut libc::c_void,
               _st_3 as *const libc::c_void,
               (i + 1 as libc::c_int) as libc::c_ulong);
        n += i;
        if n as libc::c_uint <= maxlen {
        } else {
            __assert_fail(b"n <= maxlen\x00" as *const u8 as
                              *const libc::c_char,
                          b"zbar/symbol.c\x00" as *const u8 as
                              *const libc::c_char,
                          333 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 70],
                                                    &[libc::c_char; 70]>(b"char *zbar_symbol_xml(const zbar_symbol_t *, char **, unsigned int *)\x00")).as_ptr());
        }
        j_0 = 0 as libc::c_int;
        while cfgs != 0 && j_0 < ZBAR_CFG_NUM as libc::c_int {
            if cfgs & 1 as libc::c_int as libc::c_uint != 0 {
                static mut _st_4: *const libc::c_char =
                    b"%s \x00" as *const u8 as *const libc::c_char;
                i =
                    snprintf((*buf).offset(n as isize),
                             maxlen.wrapping_sub(n as libc::c_uint) as
                                 libc::c_ulong, _st_4,
                             zbar_get_config_name(j_0 as zbar_config_t));
                if i > 0 as libc::c_int {
                } else {
                    __assert_fail(b"i > 0\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"zbar/symbol.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  336 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 70],
                                                            &[libc::c_char; 70]>(b"char *zbar_symbol_xml(const zbar_symbol_t *, char **, unsigned int *)\x00")).as_ptr());
                }
                n += i;
                if n as libc::c_uint <= maxlen {
                } else {
                    __assert_fail(b"n <= maxlen\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"zbar/symbol.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  336 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 70],
                                                            &[libc::c_char; 70]>(b"char *zbar_symbol_xml(const zbar_symbol_t *, char **, unsigned int *)\x00")).as_ptr());
                }
            }
            j_0 += 1;
            cfgs >>= 1 as libc::c_int
        }
        /* cleanup trailing space */
        n -= 1;
        static mut _st_5: *const libc::c_char =
            b"\'\x00" as *const u8 as *const libc::c_char;
        i = strlen(_st_5) as libc::c_int;
        memcpy((*buf).offset(n as isize) as *mut libc::c_void,
               _st_5 as *const libc::c_void,
               (i + 1 as libc::c_int) as libc::c_ulong);
        n += i;
        if n as libc::c_uint <= maxlen {
        } else {
            __assert_fail(b"n <= maxlen\x00" as *const u8 as
                              *const libc::c_char,
                          b"zbar/symbol.c\x00" as *const u8 as
                              *const libc::c_char,
                          339 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 70],
                                                    &[libc::c_char; 70]>(b"char *zbar_symbol_xml(const zbar_symbol_t *, char **, unsigned int *)\x00")).as_ptr());
        }
    }
    if (*sym).cache_count != 0 {
        static mut _st_6: *const libc::c_char =
            b" count=\'%d\'\x00" as *const u8 as *const libc::c_char;
        i =
            snprintf((*buf).offset(n as isize),
                     maxlen.wrapping_sub(n as libc::c_uint) as libc::c_ulong,
                     _st_6, (*sym).cache_count);
        if i > 0 as libc::c_int {
        } else {
            __assert_fail(b"i > 0\x00" as *const u8 as *const libc::c_char,
                          b"zbar/symbol.c\x00" as *const u8 as
                              *const libc::c_char,
                          343 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 70],
                                                    &[libc::c_char; 70]>(b"char *zbar_symbol_xml(const zbar_symbol_t *, char **, unsigned int *)\x00")).as_ptr());
        }
        n += i;
        if n as libc::c_uint <= maxlen {
        } else {
            __assert_fail(b"n <= maxlen\x00" as *const u8 as
                              *const libc::c_char,
                          b"zbar/symbol.c\x00" as *const u8 as
                              *const libc::c_char,
                          343 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 70],
                                                    &[libc::c_char; 70]>(b"char *zbar_symbol_xml(const zbar_symbol_t *, char **, unsigned int *)\x00")).as_ptr());
        }
    }
    static mut _st_7: *const libc::c_char =
        b"><data\x00" as *const u8 as *const libc::c_char;
    i = strlen(_st_7) as libc::c_int;
    memcpy((*buf).offset(n as isize) as *mut libc::c_void,
           _st_7 as *const libc::c_void,
           (i + 1 as libc::c_int) as libc::c_ulong);
    n += i;
    if n as libc::c_uint <= maxlen {
    } else {
        __assert_fail(b"n <= maxlen\x00" as *const u8 as *const libc::c_char,
                      b"zbar/symbol.c\x00" as *const u8 as
                          *const libc::c_char,
                      345 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 70],
                                                &[libc::c_char; 70]>(b"char *zbar_symbol_xml(const zbar_symbol_t *, char **, unsigned int *)\x00")).as_ptr());
    }
    if binary != 0 {
        static mut _st_8: *const libc::c_char =
            b" format=\'base64\' length=\'%d\'\x00" as *const u8 as
                *const libc::c_char;
        i =
            snprintf((*buf).offset(n as isize),
                     maxlen.wrapping_sub(n as libc::c_uint) as libc::c_ulong,
                     _st_8, (*sym).datalen);
        if i > 0 as libc::c_int {
        } else {
            __assert_fail(b"i > 0\x00" as *const u8 as *const libc::c_char,
                          b"zbar/symbol.c\x00" as *const u8 as
                              *const libc::c_char,
                          347 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 70],
                                                    &[libc::c_char; 70]>(b"char *zbar_symbol_xml(const zbar_symbol_t *, char **, unsigned int *)\x00")).as_ptr());
        }
        n += i;
        if n as libc::c_uint <= maxlen {
        } else {
            __assert_fail(b"n <= maxlen\x00" as *const u8 as
                              *const libc::c_char,
                          b"zbar/symbol.c\x00" as *const u8 as
                              *const libc::c_char,
                          347 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 70],
                                                    &[libc::c_char; 70]>(b"char *zbar_symbol_xml(const zbar_symbol_t *, char **, unsigned int *)\x00")).as_ptr());
        }
    }
    static mut _st_9: *const libc::c_char =
        b"><![CDATA[\x00" as *const u8 as *const libc::c_char;
    i = strlen(_st_9) as libc::c_int;
    memcpy((*buf).offset(n as isize) as *mut libc::c_void,
           _st_9 as *const libc::c_void,
           (i + 1 as libc::c_int) as libc::c_ulong);
    n += i;
    if n as libc::c_uint <= maxlen {
    } else {
        __assert_fail(b"n <= maxlen\x00" as *const u8 as *const libc::c_char,
                      b"zbar/symbol.c\x00" as *const u8 as
                          *const libc::c_char,
                      348 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 70],
                                                &[libc::c_char; 70]>(b"char *zbar_symbol_xml(const zbar_symbol_t *, char **, unsigned int *)\x00")).as_ptr());
    }
    if binary == 0 {
        memcpy((*buf).offset(n as isize) as *mut libc::c_void,
               (*sym).data as *const libc::c_void,
               (*sym).datalen.wrapping_add(1 as libc::c_int as libc::c_uint)
                   as libc::c_ulong);
        n =
            (n as libc::c_uint).wrapping_add((*sym).datalen) as libc::c_int as
                libc::c_int
    } else {
        static mut _st_10: *const libc::c_char =
            b"\n\x00" as *const u8 as *const libc::c_char;
        i = strlen(_st_10) as libc::c_int;
        memcpy((*buf).offset(n as isize) as *mut libc::c_void,
               _st_10 as *const libc::c_void,
               (i + 1 as libc::c_int) as libc::c_ulong);
        n += i;
        if n as libc::c_uint <= maxlen {
        } else {
            __assert_fail(b"n <= maxlen\x00" as *const u8 as
                              *const libc::c_char,
                          b"zbar/symbol.c\x00" as *const u8 as
                              *const libc::c_char,
                          355 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 70],
                                                    &[libc::c_char; 70]>(b"char *zbar_symbol_xml(const zbar_symbol_t *, char **, unsigned int *)\x00")).as_ptr());
        }
        n =
            (n as
                 libc::c_uint).wrapping_add(base64_encode((*buf).offset(n as
                                                                            isize),
                                                          (*sym).data,
                                                          (*sym).datalen)) as
                libc::c_int as libc::c_int
    }
    if n as libc::c_uint <= maxlen {
    } else {
        __assert_fail(b"n <= maxlen\x00" as *const u8 as *const libc::c_char,
                      b"zbar/symbol.c\x00" as *const u8 as
                          *const libc::c_char,
                      358 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 70],
                                                &[libc::c_char; 70]>(b"char *zbar_symbol_xml(const zbar_symbol_t *, char **, unsigned int *)\x00")).as_ptr());
    }
    static mut _st_11: *const libc::c_char =
        b"]]></data></symbol>\x00" as *const u8 as *const libc::c_char;
    i = strlen(_st_11) as libc::c_int;
    memcpy((*buf).offset(n as isize) as *mut libc::c_void,
           _st_11 as *const libc::c_void,
           (i + 1 as libc::c_int) as libc::c_ulong);
    n += i;
    if n as libc::c_uint <= maxlen {
    } else {
        __assert_fail(b"n <= maxlen\x00" as *const u8 as *const libc::c_char,
                      b"zbar/symbol.c\x00" as *const u8 as
                          *const libc::c_char,
                      360 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 70],
                                                &[libc::c_char; 70]>(b"char *zbar_symbol_xml(const zbar_symbol_t *, char **, unsigned int *)\x00")).as_ptr());
    }
    *len = n as libc::c_uint;
    return *buf;
}
#[no_mangle]
pub unsafe extern "C" fn _zbar_symbol_set_create() -> *mut zbar_symbol_set_t {
    let mut syms: *mut zbar_symbol_set_t =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<zbar_symbol_set_t>() as libc::c_ulong) as
            *mut zbar_symbol_set_t;
    _zbar_refcnt(&mut (*syms).refcnt, 1 as libc::c_int);
    return syms;
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
/* number of filtered symbols */
/* first of decoded symbol results */
/* last of unfiltered symbol results */
/* symbol type */
/* symbology boolean config bitmask */
/* symbology modifier bitmask */
/* allocation size of data */
/* length of binary symbol data */
/* symbol data */
/* allocation size of pts */
/* number of points in location polygon */
/* list of points in location polygon */
/* coarse orientation */
/* reference count */
/* linked list of results (or siblings) */
/* components of composite result */
/* relative symbol capture time */
/* cache state */
/* relative symbol reliability metric */
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn _zbar_symbol_set_free(mut syms:
                                                   *mut zbar_symbol_set_t) {
    let mut sym: *mut zbar_symbol_t = 0 as *mut zbar_symbol_t;
    let mut next: *mut zbar_symbol_t = 0 as *mut zbar_symbol_t;
    sym = (*syms).head;
    while !sym.is_null() {
        next = (*sym).next;
        (*sym).next = 0 as *mut zbar_symbol_t;
        _zbar_symbol_refcnt(sym, -(1 as libc::c_int));
        sym = next
    }
    (*syms).head = 0 as *mut zbar_symbol_t;
    free(syms as *mut libc::c_void);
}
/*@}*/
/*------------------------------------------------------------*/
/* * @name Symbol Set interface
 * container for decoded result symbols associated with an image
 * or a composite symbol.
 * @since 0.10
 */
/*@{*/
/* * @typedef zbar_symbol_set_t
 * opaque symbol iterator object.
 * @since 0.10
 */
/* * reference count manipulation.
 * increment the reference count when you store a new reference.
 * decrement when the reference is no longer used.  do not refer to
 * the object any longer once references have been released.
 * @since 0.10
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_symbol_set_ref(mut syms:
                                                 *const zbar_symbol_set_t,
                                             mut delta: libc::c_int) {
    let mut ncsyms: *mut zbar_symbol_set_t = syms as *mut zbar_symbol_set_t;
    if _zbar_refcnt(&mut (*ncsyms).refcnt, delta) == 0 &&
           delta <= 0 as libc::c_int {
        _zbar_symbol_set_free(ncsyms);
    };
}
/* * retrieve set size.
 * @returns the number of symbols in the set.
 * @since 0.10
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_symbol_set_get_size(mut syms:
                                                      *const zbar_symbol_set_t)
 -> libc::c_int {
    return (*syms).nsyms;
}
/* * set iterator.
 * @returns the first decoded symbol result in a set
 * @returns NULL if the set is empty
 * @since 0.10
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_symbol_set_first_symbol(mut syms:
                                                          *const zbar_symbol_set_t)
 -> *const zbar_symbol_t {
    let mut sym: *mut zbar_symbol_t = (*syms).tail;
    if !sym.is_null() { return (*sym).next }
    return (*syms).head;
}
/* * raw result iterator.
 * @returns the first decoded symbol result in a set, *before* filtering
 * @returns NULL if the set is empty
 * @since 0.11
 */
#[no_mangle]
pub unsafe extern "C" fn zbar_symbol_set_first_unfiltered(mut syms:
                                                              *const zbar_symbol_set_t)
 -> *const zbar_symbol_t {
    return (*syms).head;
}
