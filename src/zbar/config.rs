use ::libc;
extern "C" {
    #[no_mangle]
    fn strtol(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char,
              __base: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
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
/* * parse a configuration string of the form "[symbology.]config[=value]".
 * the config must match one of the recognized names.
 * the symbology, if present, must match one of the recognized names.
 * if symbology is unspecified, it will be set to 0.
 * if value is unspecified it will be set to 1.
 * @returns 0 if the config is parsed successfully, 1 otherwise
 * @since 0.4
 */
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
/* strtol */
/* strchr, strncmp, strlen */
#[no_mangle]
pub unsafe extern "C" fn zbar_parse_config(mut cfgstr: *const libc::c_char,
                                           mut sym: *mut zbar_symbol_type_t,
                                           mut cfg: *mut zbar_config_t,
                                           mut val: *mut libc::c_int)
 -> libc::c_int {
    let mut dot: *const libc::c_char =
        0 as *const libc::c_char; /* FIXME lame */
    let mut eq: *const libc::c_char =
        0 as
            *const libc::c_char; /* handle this here so we can override later */
    let mut len: libc::c_int = 0;
    let mut negate: libc::c_char = 0;
    if cfgstr.is_null() { return 1 as libc::c_int }
    dot = strchr(cfgstr, '.' as i32);
    if !dot.is_null() {
        let mut len_0: libc::c_int =
            dot.wrapping_offset_from(cfgstr) as libc::c_long as libc::c_int;
        if len_0 == 0 ||
               len_0 == 1 as libc::c_int &&
                   strncmp(cfgstr,
                           b"*\x00" as *const u8 as *const libc::c_char,
                           len_0 as libc::c_ulong) == 0 {
            *sym = ZBAR_NONE
        } else if len_0 < 2 as libc::c_int {
            return 1 as libc::c_int
        } else {
            if strncmp(cfgstr,
                       b"qrcode\x00" as *const u8 as *const libc::c_char,
                       len_0 as libc::c_ulong) == 0 {
                *sym = ZBAR_QRCODE
            } else if strncmp(cfgstr,
                              b"db\x00" as *const u8 as *const libc::c_char,
                              len_0 as libc::c_ulong) == 0 {
                *sym = ZBAR_DATABAR
            } else if len_0 < 3 as libc::c_int {
                return 1 as libc::c_int
            } else {
                if strncmp(cfgstr,
                           b"upca\x00" as *const u8 as *const libc::c_char,
                           len_0 as libc::c_ulong) == 0 {
                    *sym = ZBAR_UPCA
                } else if strncmp(cfgstr,
                                  b"upce\x00" as *const u8 as
                                      *const libc::c_char,
                                  len_0 as libc::c_ulong) == 0 {
                    *sym = ZBAR_UPCE
                } else if strncmp(cfgstr,
                                  b"ean13\x00" as *const u8 as
                                      *const libc::c_char,
                                  len_0 as libc::c_ulong) == 0 {
                    *sym = ZBAR_EAN13
                } else if strncmp(cfgstr,
                                  b"ean8\x00" as *const u8 as
                                      *const libc::c_char,
                                  len_0 as libc::c_ulong) == 0 {
                    *sym = ZBAR_EAN8
                } else if strncmp(cfgstr,
                                  b"ean5\x00" as *const u8 as
                                      *const libc::c_char,
                                  len_0 as libc::c_ulong) == 0 {
                    *sym = ZBAR_EAN5
                } else if strncmp(cfgstr,
                                  b"ean2\x00" as *const u8 as
                                      *const libc::c_char,
                                  len_0 as libc::c_ulong) == 0 {
                    *sym = ZBAR_EAN2
                } else if strncmp(cfgstr,
                                  b"composite\x00" as *const u8 as
                                      *const libc::c_char,
                                  len_0 as libc::c_ulong) == 0 {
                    *sym = ZBAR_COMPOSITE
                } else if strncmp(cfgstr,
                                  b"i25\x00" as *const u8 as
                                      *const libc::c_char,
                                  len_0 as libc::c_ulong) == 0 {
                    *sym = ZBAR_I25
                } else if len_0 < 4 as libc::c_int {
                    return 1 as libc::c_int
                } else {
                    if strncmp(cfgstr,
                               b"scanner\x00" as *const u8 as
                                   *const libc::c_char,
                               len_0 as libc::c_ulong) == 0 {
                        *sym = ZBAR_PARTIAL
                    } else if strncmp(cfgstr,
                                      b"isbn13\x00" as *const u8 as
                                          *const libc::c_char,
                                      len_0 as libc::c_ulong) == 0 {
                        *sym = ZBAR_ISBN13
                    } else if strncmp(cfgstr,
                                      b"isbn10\x00" as *const u8 as
                                          *const libc::c_char,
                                      len_0 as libc::c_ulong) == 0 {
                        *sym = ZBAR_ISBN10
                    } else if strncmp(cfgstr,
                                      b"db-exp\x00" as *const u8 as
                                          *const libc::c_char,
                                      len_0 as libc::c_ulong) == 0 {
                        *sym = ZBAR_DATABAR_EXP
                    } else if strncmp(cfgstr,
                                      b"codabar\x00" as *const u8 as
                                          *const libc::c_char,
                                      len_0 as libc::c_ulong) == 0 {
                        *sym = ZBAR_CODABAR
                    } else if len_0 < 6 as libc::c_int {
                        return 1 as libc::c_int
                    } else {
                        if strncmp(cfgstr,
                                   b"code93\x00" as *const u8 as
                                       *const libc::c_char,
                                   len_0 as libc::c_ulong) == 0 {
                            *sym = ZBAR_CODE93
                        } else if strncmp(cfgstr,
                                          b"code39\x00" as *const u8 as
                                              *const libc::c_char,
                                          len_0 as libc::c_ulong) == 0 {
                            *sym = ZBAR_CODE39
                        } else if strncmp(cfgstr,
                                          b"pdf417\x00" as *const u8 as
                                              *const libc::c_char,
                                          len_0 as libc::c_ulong) == 0 {
                            *sym = ZBAR_PDF417
                        } else if len_0 < 7 as libc::c_int {
                            return 1 as libc::c_int
                        } else {
                            if strncmp(cfgstr,
                                       b"code128\x00" as *const u8 as
                                           *const libc::c_char,
                                       len_0 as libc::c_ulong) == 0 {
                                *sym = ZBAR_CODE128
                            } else if strncmp(cfgstr,
                                              b"databar\x00" as *const u8 as
                                                  *const libc::c_char,
                                              len_0 as libc::c_ulong) == 0 {
                                *sym = ZBAR_DATABAR
                            } else if strncmp(cfgstr,
                                              b"databar-exp\x00" as *const u8
                                                  as *const libc::c_char,
                                              len_0 as libc::c_ulong) == 0 {
                                *sym = ZBAR_DATABAR_EXP
                            } else { return 1 as libc::c_int }
                        }
                    }
                }
            }
        }
        cfgstr = dot.offset(1 as libc::c_int as isize)
    } else { *sym = ZBAR_NONE }
    len = strlen(cfgstr) as libc::c_int;
    eq = strchr(cfgstr, '=' as i32);
    if !eq.is_null() {
        len = eq.wrapping_offset_from(cfgstr) as libc::c_long as libc::c_int
    } else { *val = 1 as libc::c_int }
    negate = 0 as libc::c_int as libc::c_char;
    if len > 3 as libc::c_int &&
           strncmp(cfgstr, b"no-\x00" as *const u8 as *const libc::c_char,
                   3 as libc::c_int as libc::c_ulong) == 0 {
        negate = 1 as libc::c_int as libc::c_char;
        cfgstr = cfgstr.offset(3 as libc::c_int as isize);
        len -= 3 as libc::c_int
    }
    if len < 1 as libc::c_int {
        return 1 as libc::c_int
    } else {
        if strncmp(cfgstr,
                   b"y-density\x00" as *const u8 as *const libc::c_char,
                   len as libc::c_ulong) == 0 {
            *cfg = ZBAR_CFG_Y_DENSITY
        } else if strncmp(cfgstr,
                          b"x-density\x00" as *const u8 as
                              *const libc::c_char, len as libc::c_ulong) == 0
         {
            *cfg = ZBAR_CFG_X_DENSITY
        } else if len < 2 as libc::c_int {
            return 1 as libc::c_int
        } else {
            if strncmp(cfgstr,
                       b"enable\x00" as *const u8 as *const libc::c_char,
                       len as libc::c_ulong) == 0 {
                *cfg = ZBAR_CFG_ENABLE
            } else if len < 3 as libc::c_int {
                return 1 as libc::c_int
            } else {
                if strncmp(cfgstr,
                           b"disable\x00" as *const u8 as *const libc::c_char,
                           len as libc::c_ulong) == 0 {
                    *cfg = ZBAR_CFG_ENABLE;
                    negate = (negate == 0) as libc::c_int as libc::c_char
                    /* no-disable ?!? */
                } else if strncmp(cfgstr,
                                  b"min-length\x00" as *const u8 as
                                      *const libc::c_char,
                                  len as libc::c_ulong) == 0 {
                    *cfg = ZBAR_CFG_MIN_LEN
                } else if strncmp(cfgstr,
                                  b"max-length\x00" as *const u8 as
                                      *const libc::c_char,
                                  len as libc::c_ulong) == 0 {
                    *cfg = ZBAR_CFG_MAX_LEN
                } else if strncmp(cfgstr,
                                  b"ascii\x00" as *const u8 as
                                      *const libc::c_char,
                                  len as libc::c_ulong) == 0 {
                    *cfg = ZBAR_CFG_ASCII
                } else if strncmp(cfgstr,
                                  b"add-check\x00" as *const u8 as
                                      *const libc::c_char,
                                  len as libc::c_ulong) == 0 {
                    *cfg = ZBAR_CFG_ADD_CHECK
                } else if strncmp(cfgstr,
                                  b"emit-check\x00" as *const u8 as
                                      *const libc::c_char,
                                  len as libc::c_ulong) == 0 {
                    *cfg = ZBAR_CFG_EMIT_CHECK
                } else if strncmp(cfgstr,
                                  b"uncertainty\x00" as *const u8 as
                                      *const libc::c_char,
                                  len as libc::c_ulong) == 0 {
                    *cfg = ZBAR_CFG_UNCERTAINTY
                } else if strncmp(cfgstr,
                                  b"position\x00" as *const u8 as
                                      *const libc::c_char,
                                  len as libc::c_ulong) == 0 {
                    *cfg = ZBAR_CFG_POSITION
                } else { return 1 as libc::c_int }
            }
        }
    }
    if !eq.is_null() {
        *__errno_location() = 0 as libc::c_int;
        *val =
            strtol(eq.offset(1 as libc::c_int as isize),
                   0 as *mut *mut libc::c_char, 0 as libc::c_int) as
                libc::c_int;
        if *__errno_location() != 0 { return 1 as libc::c_int }
    }
    if negate != 0 { *val = (*val == 0) as libc::c_int }
    return 0 as libc::c_int;
}
