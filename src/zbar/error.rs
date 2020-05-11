use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type uint32_t = __uint32_t;
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
pub type zbar_error_e = libc::c_uint;
pub const ZBAR_ERR_NUM: zbar_error_e = 12;
pub const ZBAR_ERR_WINAPI: zbar_error_e = 11;
pub const ZBAR_ERR_CLOSED: zbar_error_e = 10;
pub const ZBAR_ERR_XPROTO: zbar_error_e = 9;
pub const ZBAR_ERR_XDISPLAY: zbar_error_e = 8;
pub const ZBAR_ERR_BUSY: zbar_error_e = 7;
pub const ZBAR_ERR_LOCKING: zbar_error_e = 6;
pub const ZBAR_ERR_SYSTEM: zbar_error_e = 5;
pub const ZBAR_ERR_INVALID: zbar_error_e = 4;
pub const ZBAR_ERR_UNSUPPORTED: zbar_error_e = 3;
pub const ZBAR_ERR_INTERNAL: zbar_error_e = 2;
pub const ZBAR_ERR_NOMEM: zbar_error_e = 1;
pub const ZBAR_OK: zbar_error_e = 0;
pub type zbar_error_t = zbar_error_e;
pub type errsev_t = errsev_e;
pub type errsev_e = libc::c_int;
pub const SEV_NOTE: errsev_e = 2;
pub const SEV_WARNING: errsev_e = 1;
pub const SEV_OK: errsev_e = 0;
pub const SEV_ERROR: errsev_e = -1;
pub const SEV_FATAL: errsev_e = -2;
pub type errinfo_t = errinfo_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct errinfo_s {
    pub magic: uint32_t,
    pub module: errmodule_t,
    pub buf: *mut libc::c_char,
    pub errnum: libc::c_int,
    pub sev: errsev_t,
    pub type_0: zbar_error_t,
    pub func: *const libc::c_char,
    pub detail: *const libc::c_char,
    pub arg_str: *mut libc::c_char,
    pub arg_int: libc::c_int,
}
pub type errmodule_t = errmodule_e;
pub type errmodule_e = libc::c_uint;
pub const ZBAR_MOD_UNKNOWN: errmodule_e = 4;
pub const ZBAR_MOD_IMAGE_SCANNER: errmodule_e = 3;
pub const ZBAR_MOD_WINDOW: errmodule_e = 2;
pub const ZBAR_MOD_VIDEO: errmodule_e = 1;
pub const ZBAR_MOD_PROCESSOR: errmodule_e = 0;
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
pub static mut _zbar_verbosity: libc::c_int = 0 as libc::c_int;
static mut sev_str: [*const libc::c_char; 5] =
    [b"FATAL ERROR\x00" as *const u8 as *const libc::c_char,
     b"ERROR\x00" as *const u8 as *const libc::c_char,
     b"OK\x00" as *const u8 as *const libc::c_char,
     b"WARNING\x00" as *const u8 as *const libc::c_char,
     b"NOTE\x00" as *const u8 as *const libc::c_char];
static mut mod_str: [*const libc::c_char; 5] =
    [b"processor\x00" as *const u8 as *const libc::c_char,
     b"video\x00" as *const u8 as *const libc::c_char,
     b"window\x00" as *const u8 as *const libc::c_char,
     b"image scanner\x00" as *const u8 as *const libc::c_char,
     b"<unknown>\x00" as *const u8 as *const libc::c_char];
static mut err_str: [*const libc::c_char; 13] =
    [b"no error\x00" as *const u8 as *const libc::c_char,
     b"out of memory\x00" as *const u8 as *const libc::c_char,
     b"internal library error\x00" as *const u8 as *const libc::c_char,
     b"unsupported request\x00" as *const u8 as *const libc::c_char,
     b"invalid request\x00" as *const u8 as *const libc::c_char,
     b"system error\x00" as *const u8 as *const libc::c_char,
     b"locking error\x00" as *const u8 as *const libc::c_char,
     b"all resources busy\x00" as *const u8 as *const libc::c_char,
     b"X11 display error\x00" as *const u8 as *const libc::c_char,
     b"X11 protocol error\x00" as *const u8 as *const libc::c_char,
     b"output window is closed\x00" as *const u8 as *const libc::c_char,
     b"windows system error\x00" as *const u8 as *const libc::c_char,
     b"unknown error\x00" as *const u8 as *const libc::c_char];
#[no_mangle]
pub unsafe extern "C" fn zbar_version(mut major: *mut libc::c_uint,
                                      mut minor: *mut libc::c_uint)
 -> libc::c_int {
    if !major.is_null() { *major = 0 as libc::c_int as libc::c_uint }
    if !minor.is_null() { *minor = 20 as libc::c_int as libc::c_uint }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_set_verbosity(mut level: libc::c_int) {
    _zbar_verbosity = level;
}
#[no_mangle]
pub unsafe extern "C" fn zbar_increase_verbosity() {
    if _zbar_verbosity == 0 {
        _zbar_verbosity += 1
    } else { _zbar_verbosity <<= 1 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn _zbar_error_spew(mut container: *const libc::c_void,
                                          mut verbosity: libc::c_int)
 -> libc::c_int {
    let mut err: *const errinfo_t = container as *const errinfo_t;
    if (*err).magic == 0x5252457a as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"err->magic == ERRINFO_MAGIC\x00" as *const u8 as
                          *const libc::c_char,
                      b"zbar/error.c\x00" as *const u8 as *const libc::c_char,
                      83 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 40],
                                                &[libc::c_char; 40]>(b"int _zbar_error_spew(const void *, int)\x00")).as_ptr());
    }
    fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
            _zbar_error_string(err as *const libc::c_void, verbosity));
    return -((*err).sev as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn _zbar_get_error_code(mut container:
                                                  *const libc::c_void)
 -> zbar_error_t {
    let mut err: *const errinfo_t = container as *const errinfo_t;
    if (*err).magic == 0x5252457a as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"err->magic == ERRINFO_MAGIC\x00" as *const u8 as
                          *const libc::c_char,
                      b"zbar/error.c\x00" as *const u8 as *const libc::c_char,
                      91 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 48],
                                                &[libc::c_char; 48]>(b"zbar_error_t _zbar_get_error_code(const void *)\x00")).as_ptr());
    }
    return (*err).type_0;
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
/* *< light area or space between bars */
/* *< dark area or colored bar segment */
/* * decoded symbol type. */
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
/* *< unable to determine orientation */
/* *< upright, read left to right */
/* *< sideways, read top to bottom */
/* *< upside-down, read right to left */
/* *< sideways, read bottom to top */
/* * error codes. */
/* *< no error */
/* *< out of memory */
/* *< internal library error */
/* *< unsupported request */
/* *< invalid request */
/* *< system error */
/* *< locking error */
/* *< all resources busy */
/* *< X11 display error */
/* *< X11 protocol error */
/* *< output window is closed */
/* *< windows system error */
/* *< number of error codes */
/* * decoder configuration options.
 * @since 0.4
 */
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
/* * barcode tagged as GS1 (EAN.UCC) reserved
     * (eg, FNC1 before first data character).
     * data may be parsed as a sequence of GS1 AIs
     */
/* * barcode tagged as AIM reserved
     * (eg, FNC1 after first character or digit pair)
     */
/* * number of modifiers */
/* * store video control menu
 * @param name name of the menu item
 * @param val integer value associated with the item
 */
/* * store video controls
 * @param name name of the control
 * @param group name of the control group/class
 * @param type type of the control
 * @param min minimum value of control (if control is integer)
 * @param max maximum value of control (if control is integer)
 * @param def default value of control (if control is integer)
 * @param step increment steps (if control is integer)
 * @param menu menu array
 * @param menu_size menu size
 * @since 0.20
 */
// video drivers may add extra private data in the end of this struct
/* * retrieve runtime library version information.
 * @param major set to the running major version (unless NULL)
 * @param minor set to the running minor version (unless NULL)
 * @returns 0
 */
/* * set global library debug level.
 * @param verbosity desired debug level.  higher values create more spew
 */
/* * increase global library debug level.
 * eg, for -vvvv
 */
/* * retrieve string name for symbol encoding.
 * @param sym symbol type encoding
 * @returns the static string name for the specified symbol type,
 * or "UNKNOWN" if the encoding is not recognized
 */
/* * retrieve string name for addon encoding.
 * @param sym symbol type encoding
 * @returns static string name for any addon, or the empty string
 * if no addons were decoded
 * @deprecated in 0.11
 */
/* * retrieve string name for configuration setting.
 * @param config setting to name
 * @returns static string name for config,
 * or the empty string if value is not a known config
 */
/* * retrieve string name for modifier.
 * @param modifier flag to name
 * @returns static string name for modifier,
 * or the empty string if the value is not a known flag
 */
/* * retrieve string name for orientation.
 * @param orientation orientation encoding
 * @returns the static string name for the specified orientation,
 * or "UNKNOWN" if the orientation is not recognized
 * @since 0.11
 */
/* * parse a configuration string of the form "[symbology.]config[=value]".
 * the config must match one of the recognized names.
 * the symbology, if present, must match one of the recognized names.
 * if symbology is unspecified, it will be set to 0.
 * if value is unspecified it will be set to 1.
 * @returns 0 if the config is parsed successfully, 1 otherwise
 * @since 0.4
 */
/* * consistently compute fourcc values across architectures
 * (adapted from v4l2 specification)
 * @since 0.11
 */
/* * parse a fourcc string into its encoded integer value.
 * @since 0.11
 */
/* * @internal type unsafe error API (don't use) */
/* ERROR: zbar video in v4l1_set_format():
 *     system error: blah[: blah]
 */
#[no_mangle]
pub unsafe extern "C" fn _zbar_error_string(mut container:
                                                *const libc::c_void,
                                            mut verbosity: libc::c_int)
 -> *const libc::c_char {
    static mut basefmt: [libc::c_char; 30] =
        [37, 115, 58, 32, 122, 98, 97, 114, 32, 37, 115, 32, 105, 110, 32, 37,
         115, 40, 41, 58, 10, 32, 32, 32, 32, 37, 115, 58, 32, 0];
    let mut err: *mut errinfo_t = container as *mut errinfo_t;
    let mut sev: *const libc::c_char = 0 as *const libc::c_char;
    let mut mod_0: *const libc::c_char = 0 as *const libc::c_char;
    let mut func: *const libc::c_char = 0 as *const libc::c_char;
    let mut type_0: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: libc::c_int = 0;
    if (*err).magic == 0x5252457a as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"err->magic == ERRINFO_MAGIC\x00" as *const u8 as
                          *const libc::c_char,
                      b"zbar/error.c\x00" as *const u8 as *const libc::c_char,
                      107 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 50],
                                                &[libc::c_char; 50]>(b"const char *_zbar_error_string(const void *, int)\x00")).as_ptr());
    }
    if (*err).sev as libc::c_int >= SEV_FATAL as libc::c_int &&
           (*err).sev as libc::c_int <= SEV_NOTE as libc::c_int {
        sev = sev_str[((*err).sev as libc::c_int + 2 as libc::c_int) as usize]
    } else { sev = sev_str[1 as libc::c_int as usize] }
    if (*err).module as libc::c_uint >=
           ZBAR_MOD_PROCESSOR as libc::c_int as libc::c_uint &&
           ((*err).module as libc::c_uint) <
               ZBAR_MOD_UNKNOWN as libc::c_int as libc::c_uint {
        mod_0 = mod_str[(*err).module as usize]
    } else { mod_0 = mod_str[ZBAR_MOD_UNKNOWN as libc::c_int as usize] }
    func =
        if !(*err).func.is_null() {
            (*err).func
        } else { b"<unknown>\x00" as *const u8 as *const libc::c_char };
    if (*err).type_0 as libc::c_uint >= 0 as libc::c_int as libc::c_uint &&
           ((*err).type_0 as libc::c_uint) <
               ZBAR_ERR_NUM as libc::c_int as libc::c_uint {
        type_0 = err_str[(*err).type_0 as usize]
    } else { type_0 = err_str[ZBAR_ERR_NUM as libc::c_int as usize] }
    len =
        strlen(sev_str[0 as libc::c_int as
                           usize]).wrapping_add(strlen(mod_str[ZBAR_MOD_IMAGE_SCANNER
                                                                   as
                                                                   libc::c_int
                                                                   as
                                                                   usize])).wrapping_add(strlen(err_str[ZBAR_ERR_CLOSED
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            usize])).wrapping_add(strlen(func)).wrapping_add(::std::mem::size_of::<[libc::c_char; 30]>()
                                                                                                                                                                 as
                                                                                                                                                                 libc::c_ulong)
            as libc::c_int;
    (*err).buf =
        realloc((*err).buf as *mut libc::c_void, len as libc::c_ulong) as
            *mut libc::c_char;
    len = sprintf((*err).buf, basefmt.as_ptr(), sev, mod_0, func, type_0);
    if len <= 0 as libc::c_int {
        return b"<unknown>\x00" as *const u8 as *const libc::c_char
    }
    if !(*err).detail.is_null() {
        let mut newlen: libc::c_int =
            (len as
                 libc::c_ulong).wrapping_add(strlen((*err).detail)).wrapping_add(1
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong)
                as libc::c_int;
        if !strstr((*err).detail,
                   b"%s\x00" as *const u8 as *const libc::c_char).is_null() {
            if (*err).arg_str.is_null() {
                (*err).arg_str =
                    strdup(b"<?>\x00" as *const u8 as *const libc::c_char)
            }
            (*err).buf =
                realloc((*err).buf as *mut libc::c_void,
                        (newlen as
                             libc::c_ulong).wrapping_add(strlen((*err).arg_str)))
                    as *mut libc::c_char;
            len +=
                sprintf((*err).buf.offset(len as isize), (*err).detail,
                        (*err).arg_str)
        } else if !strstr((*err).detail,
                          b"%d\x00" as *const u8 as
                              *const libc::c_char).is_null() ||
                      !strstr((*err).detail,
                              b"%x\x00" as *const u8 as
                                  *const libc::c_char).is_null() {
            (*err).buf =
                realloc((*err).buf as *mut libc::c_void,
                        (newlen + 32 as libc::c_int) as libc::c_ulong) as
                    *mut libc::c_char;
            len +=
                sprintf((*err).buf.offset(len as isize), (*err).detail,
                        (*err).arg_int)
        } else {
            (*err).buf =
                realloc((*err).buf as *mut libc::c_void,
                        newlen as libc::c_ulong) as *mut libc::c_char;
            len +=
                sprintf((*err).buf.offset(len as isize),
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        (*err).detail)
        }
        if len <= 0 as libc::c_int {
            return b"<unknown>\x00" as *const u8 as *const libc::c_char
        }
    }
    if (*err).type_0 as libc::c_uint ==
           ZBAR_ERR_SYSTEM as libc::c_int as libc::c_uint {
        static mut sysfmt: [libc::c_char; 11] =
            [58, 32, 37, 115, 32, 40, 37, 100, 41, 10, 0];
        let mut syserr: *const libc::c_char = strerror((*err).errnum);
        (*err).buf =
            realloc((*err).buf as *mut libc::c_void,
                    (len as
                         libc::c_ulong).wrapping_add(strlen(sysfmt.as_ptr())).wrapping_add(strlen(syserr)))
                as *mut libc::c_char;
        len +=
            sprintf((*err).buf.offset(len as isize), sysfmt.as_ptr(), syserr,
                    (*err).errnum)
    } else {
        (*err).buf =
            realloc((*err).buf as *mut libc::c_void,
                    (len + 2 as libc::c_int) as libc::c_ulong) as
                *mut libc::c_char;
        len +=
            sprintf((*err).buf.offset(len as isize),
                    b"\n\x00" as *const u8 as *const libc::c_char)
    }
    return (*err).buf;
}
