use ::libc;
extern {
    pub type zbar_video_s;
    pub type zbar_image_scanner_s;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memchr(_: *const libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn iconv_open(__tocode: *const libc::c_char, __fromcode: *const libc::c_char) -> iconv_t;
    #[no_mangle]
    fn iconv(
        __cd: iconv_t,
        __inbuf: *mut *mut libc::c_char,
        __inbytesleft: *mut size_t,
        __outbuf: *mut *mut libc::c_char,
        __outbytesleft: *mut size_t,
    ) -> size_t;
    #[no_mangle]
    fn iconv_close(__cd: iconv_t) -> libc::c_int;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn _zbar_symbol_free(_: *mut zbar_symbol_t);
    #[no_mangle]
    fn _zbar_refcnt(cnt: *mut refcnt_t, delta: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn _zbar_symbol_set_create() -> *mut zbar_symbol_set_t;
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
    /* internal image scanner APIs for 2D readers */
    #[no_mangle]
    fn _zbar_image_scanner_alloc_sym(
        _: *mut zbar_image_scanner_t,
        _: zbar_symbol_type_t,
        _: libc::c_int,
    ) -> *mut zbar_symbol_t;
    #[no_mangle]
    fn _zbar_image_scanner_add_sym(_: *mut zbar_image_scanner_t, _: *mut zbar_symbol_t);
    #[no_mangle]
    fn _zbar_image_scanner_recycle_syms(_: *mut zbar_image_scanner_t, _: *mut zbar_symbol_t);
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type iconv_t = *mut libc::c_void;
pub type uint32_t = __uint32_t;
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
pub type zbar_symbol_type_t = zbar_symbol_type_e;
pub type zbar_orientation_e = libc::c_int;
pub const ZBAR_ORIENT_LEFT: zbar_orientation_e = 3;
pub const ZBAR_ORIENT_DOWN: zbar_orientation_e = 2;
pub const ZBAR_ORIENT_RIGHT: zbar_orientation_e = 1;
pub const ZBAR_ORIENT_UP: zbar_orientation_e = 0;
pub const ZBAR_ORIENT_UNKNOWN: zbar_orientation_e = -1;
pub type zbar_orientation_t = zbar_orientation_e;
pub type zbar_modifier_e = libc::c_uint;
pub const ZBAR_MOD_NUM: zbar_modifier_e = 2;
pub const ZBAR_MOD_AIM: zbar_modifier_e = 1;
pub const ZBAR_MOD_GS1: zbar_modifier_e = 0;
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
/* number of filtered symbols */
/* first of decoded symbol results */
/* last of unfiltered symbol results */
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
pub type zbar_symbol_t = zbar_symbol_s;
pub type refcnt_t = libc::c_int;
pub type point_t = point_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point_s {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zbar_image_s {
    pub format: uint32_t,
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub data: *const libc::c_void,
    pub datalen: libc::c_ulong,
    pub crop_x: libc::c_uint,
    pub crop_y: libc::c_uint,
    pub crop_w: libc::c_uint,
    pub crop_h: libc::c_uint,
    pub userdata: *mut libc::c_void,
    pub cleanup: Option<zbar_image_cleanup_handler_t>,
    pub refcnt: refcnt_t,
    pub src: *mut zbar_video_t,
    pub srcidx: libc::c_int,
    pub next: *mut zbar_image_t,
    pub seq: libc::c_uint,
    pub syms: *mut zbar_symbol_set_t,
}
pub type zbar_image_t = zbar_image_s;
pub type zbar_video_t = zbar_video_s;
pub type zbar_image_cleanup_handler_t = unsafe extern fn(_: *mut zbar_image_t) -> ();
pub type zbar_image_scanner_t = zbar_image_scanner_s;
pub type qr_point = [libc::c_int; 2];
/* A single unit of parsed QR code data. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qr_code_data_entry {
    pub mode: qr_mode,
    pub payload: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub data: C2RustUnnamed_1,
    pub eci: libc::c_uint,
    pub ai: libc::c_int,
    pub sa: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub sa_index: libc::c_uchar,
    pub sa_size: libc::c_uchar,
    pub sa_parity: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub buf: *mut libc::c_uchar,
    pub len: libc::c_int,
}
pub type qr_mode = libc::c_uint;
pub const QR_MODE_FNC1_2ND: qr_mode = 9;
pub const QR_MODE_KANJI: qr_mode = 8;
pub const QR_MODE_ECI: qr_mode = 7;
pub const QR_MODE_FNC1_1ST: qr_mode = 5;
pub const QR_MODE_BYTE: qr_mode = 4;
pub const QR_MODE_STRUCT: qr_mode = 3;
pub const QR_MODE_ALNUM: qr_mode = 2;
pub const QR_MODE_NUM: qr_mode = 1;
/* Low-level QR code data. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qr_code_data {
    pub entries: *mut qr_code_data_entry,
    pub nentries: libc::c_int,
    pub version: libc::c_uchar,
    pub ecc_level: libc::c_uchar,
    pub sa_index: libc::c_uchar,
    pub sa_size: libc::c_uchar,
    pub sa_parity: libc::c_uchar,
    pub self_parity: libc::c_uchar,
    pub bbox: [qr_point; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qr_code_data_list {
    pub qrdata: *mut qr_code_data,
    pub nqrdata: libc::c_int,
    pub cqrdata: libc::c_int,
}
pub type qr_eci_encoding = libc::c_uint;
pub const QR_ECI_UTF8: qr_eci_encoding = 26;
pub const QR_ECI_SJIS: qr_eci_encoding = 20;
pub const QR_ECI_ISO8859_16: qr_eci_encoding = 18;
pub const QR_ECI_ISO8859_15: qr_eci_encoding = 17;
pub const QR_ECI_ISO8859_14: qr_eci_encoding = 16;
pub const QR_ECI_ISO8859_13: qr_eci_encoding = 15;
pub const QR_ECI_ISO8859_11: qr_eci_encoding = 13;
pub const QR_ECI_ISO8859_10: qr_eci_encoding = 12;
pub const QR_ECI_ISO8859_9: qr_eci_encoding = 11;
pub const QR_ECI_ISO8859_8: qr_eci_encoding = 10;
pub const QR_ECI_ISO8859_7: qr_eci_encoding = 9;
pub const QR_ECI_ISO8859_6: qr_eci_encoding = 8;
pub const QR_ECI_ISO8859_5: qr_eci_encoding = 7;
pub const QR_ECI_ISO8859_4: qr_eci_encoding = 6;
pub const QR_ECI_ISO8859_3: qr_eci_encoding = 5;
pub const QR_ECI_ISO8859_2: qr_eci_encoding = 4;
pub const QR_ECI_ISO8859_1: qr_eci_encoding = 3;
pub const QR_ECI_CP437: qr_eci_encoding = 2;
pub const QR_ECI_GLI1: qr_eci_encoding = 1;
pub const QR_ECI_GLI0: qr_eci_encoding = 0;
#[inline]
unsafe extern fn sym_add_point(
    mut sym: *mut zbar_symbol_t,
    mut x: libc::c_int,
    mut y: libc::c_int,
) {
    let mut i: libc::c_int = (*sym).npts as libc::c_int;
    (*sym).npts = (*sym).npts.wrapping_add(1);
    if (*sym).npts >= (*sym).pts_alloc {
        (*sym).pts_alloc = (*sym).pts_alloc.wrapping_add(1);
        (*sym).pts = realloc(
            (*sym).pts as *mut libc::c_void,
            ((*sym).pts_alloc as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<point_t>() as libc::c_ulong),
        ) as *mut point_t
    }
    (*(*sym).pts.offset(i as isize)).x = x;
    (*(*sym).pts.offset(i as isize)).y = y;
}
#[inline]
unsafe extern fn _zbar_symbol_refcnt(mut sym: *mut zbar_symbol_t, mut delta: libc::c_int) {
    if _zbar_refcnt(&mut (*sym).refcnt, delta) == 0 && delta <= 0 as libc::c_int {
        _zbar_symbol_free(sym);
    };
}
/*Copyright (C) 2008-2010  Timothy B. Terriberry (tterribe@xiph.org)
You can redistribute this library and/or modify it under the terms of the
 GNU Lesser General Public License as published by the Free Software
 Foundation; either version 2.1 of the License, or (at your option) any later
 version.*/
unsafe extern fn text_is_ascii(
    mut _text: *const libc::c_uchar,
    mut _len: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < _len {
        if *_text.offset(i as isize) as libc::c_int >= 0x80 as libc::c_int {
            return 0 as libc::c_int;
        }
        i += 1
    }
    return 1 as libc::c_int;
}
unsafe extern fn text_is_latin1(
    mut _text: *const libc::c_uchar,
    mut _len: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < _len {
        /*The following line fails to compile correctly with gcc 3.4.4 on ARM with
        any optimizations enabled.*/
        if *_text.offset(i as isize) as libc::c_int >= 0x80 as libc::c_int
            && (*_text.offset(i as isize) as libc::c_int) < 0xa0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        i += 1
    }
    return 1 as libc::c_int;
}
unsafe extern fn enc_list_mtf(mut _enc_list: *mut iconv_t, mut _enc: iconv_t) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if *_enc_list.offset(i as isize) == _enc {
            let mut j: libc::c_int = 0;
            j = i;
            loop {
                let fresh0 = j;
                j = j - 1;
                if !(fresh0 > 0 as libc::c_int) {
                    break;
                }
                let ref mut fresh1 = *_enc_list.offset((j + 1 as libc::c_int) as isize);
                *fresh1 = *_enc_list.offset(j as isize)
            }
            let ref mut fresh2 = *_enc_list.offset(0 as libc::c_int as isize);
            *fresh2 = _enc;
            break;
        } else {
            i += 1
        }
    }
}
/*Extract symbol data from a list of QR codes and attach to the image.
All text is converted to UTF-8.
Any structured-append group that does not have all of its members is decoded
 as ZBAR_PARTIAL with ZBAR_PARTIAL components for the discontinuities.
Note that isolated members of a structured-append group may be decoded with
 the wrong character set, since the correct setting cannot be propagated
 between codes.
Return: The number of symbols which were successfully extracted from the
 codes; this will be at most the number of codes.*/
#[no_mangle]
pub unsafe extern fn qr_code_data_list_extract_text(
    mut _qrlist: *const qr_code_data_list,
    mut iscn: *mut zbar_image_scanner_t,
    mut img: *mut zbar_image_t,
) -> libc::c_int {
    let mut sjis_cd: iconv_t = 0 as *mut libc::c_void;
    let mut utf8_cd: iconv_t = 0 as *mut libc::c_void;
    let mut latin1_cd: iconv_t = 0 as *mut libc::c_void;
    let mut qrdata: *const qr_code_data = 0 as *const qr_code_data;
    let mut nqrdata: libc::c_int = 0;
    let mut mark: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ntext: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    qrdata = (*_qrlist).qrdata;
    nqrdata = (*_qrlist).nqrdata;
    mark = calloc(nqrdata as libc::c_ulong, ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
        as *mut libc::c_uchar;
    ntext = 0 as libc::c_int;
    /* This is the encoding the standard says is the default. */
    latin1_cd = iconv_open(
        b"UTF-8\x00" as *const u8 as *const libc::c_char,
        b"ISO8859-1\x00" as *const u8 as *const libc::c_char,
    );
    /* But this one is often used, as well. */
    sjis_cd = iconv_open(
        b"UTF-8\x00" as *const u8 as *const libc::c_char,
        b"SJIS\x00" as *const u8 as *const libc::c_char,
    );
    /* This is a trivial conversion just to check validity without extra code. */
    utf8_cd = iconv_open(
        b"UTF-8\x00" as *const u8 as *const libc::c_char,
        b"UTF-8\x00" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < nqrdata {
        if *mark.offset(i as isize) == 0 {
            let mut qrdataj: *const qr_code_data = 0 as *const qr_code_data;
            let mut entry: *const qr_code_data_entry = 0 as *const qr_code_data_entry;
            let mut enc_list: [iconv_t; 3] = [0 as *mut libc::c_void; 3];
            let mut eci_cd: iconv_t = 0 as *mut libc::c_void;
            let mut sa: [libc::c_int; 16] = [0; 16];
            let mut sa_size: libc::c_int = 0;
            let mut sa_text: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut sa_ntext: size_t = 0;
            let mut sa_ctext: size_t = 0;
            let mut fnc1: libc::c_int = 0;
            let mut fnc1_2ai: libc::c_int = 0;
            let mut has_kanji: libc::c_int = 0;
            let mut eci: libc::c_int = 0;
            let mut err: libc::c_int = 0;
            let mut j: libc::c_int = 0;
            let mut k: libc::c_int = 0;
            let mut syms: *mut zbar_symbol_t = 0 as *mut zbar_symbol_t;
            let mut sym: *mut *mut zbar_symbol_t = &mut syms;
            let mut dir: qr_point = [0; 2];
            let mut horiz: libc::c_int = 0;
            /* Step 0: Collect the other QR codes belonging to this S-A group. */
            if (*qrdata.offset(i as isize)).sa_size != 0 {
                let mut sa_parity: libc::c_uint = 0;
                sa_size = (*qrdata.offset(i as isize)).sa_size as libc::c_int;
                sa_parity = (*qrdata.offset(i as isize)).sa_parity as libc::c_uint;
                j = 0 as libc::c_int;
                while j < sa_size {
                    sa[j as usize] = -(1 as libc::c_int);
                    j += 1
                }
                j = i;
                while j < nqrdata {
                    if *mark.offset(j as isize) == 0 {
                        /* TODO: If the S-A group is complete, check the parity. */
                        /*TODO: We could also match version, ECC level, etc. if size and
                        parity alone are too ambiguous.*/
                        if (*qrdata.offset(j as isize)).sa_size as libc::c_int == sa_size
                            && (*qrdata.offset(j as isize)).sa_parity as libc::c_uint == sa_parity
                            && sa[(*qrdata.offset(j as isize)).sa_index as usize] < 0 as libc::c_int
                        {
                            sa[(*qrdata.offset(j as isize)).sa_index as usize] = j;
                            *mark.offset(j as isize) = 1 as libc::c_int as libc::c_uchar
                        }
                    }
                    j += 1
                }
            } else {
                sa[0 as libc::c_int as usize] = i;
                sa_size = 1 as libc::c_int
            }
            sa_ctext = 0 as libc::c_int as size_t;
            fnc1 = 0 as libc::c_int;
            fnc1_2ai = 0 as libc::c_int;
            has_kanji = 0 as libc::c_int;
            /* Step 1: Detect FNC1 markers and estimate the required buffer size. */
            j = 0 as libc::c_int;
            while j < sa_size {
                if sa[j as usize] >= 0 as libc::c_int {
                    qrdataj = qrdata.offset(sa[j as usize] as isize);
                    k = 0 as libc::c_int;
                    while k < (*qrdataj).nentries {
                        let mut shift: libc::c_int = 0;
                        entry = (*qrdataj).entries.offset(k as isize);
                        shift = 0 as libc::c_int;
                        let mut current_block_49: u64;
                        match (*entry).mode as libc::c_uint {
                            5 => {
                                /* FNC1 applies to the entire code and ignores subsequent markers. */
                                if fnc1 == 0 {
                                    fnc1 = (1 as libc::c_int) << ZBAR_MOD_GS1 as libc::c_int
                                }
                                current_block_49 = 7330218953828964527;
                            }
                            9 => {
                                if fnc1 == 0 {
                                    fnc1 = (1 as libc::c_int) << ZBAR_MOD_AIM as libc::c_int;
                                    fnc1_2ai = (*entry).payload.ai;
                                    sa_ctext = (sa_ctext as libc::c_ulong)
                                        .wrapping_add(2 as libc::c_int as libc::c_ulong)
                                        as size_t
                                        as size_t
                                }
                                current_block_49 = 7330218953828964527;
                            }
                            8 => {
                                /*We assume at most 4 UTF-8 bytes per input byte.
                                I believe this is true for all the encodings we actually use.*/
                                has_kanji = 1 as libc::c_int;
                                current_block_49 = 17118267793213255982;
                            }
                            4 => {
                                current_block_49 = 17118267793213255982;
                            }
                            _ => {
                                current_block_49 = 12999864789452897461;
                            }
                        }
                        match current_block_49 {
                            17118267793213255982 => {
                                shift = 2 as libc::c_int;
                                current_block_49 = 12999864789452897461;
                            }
                            _ => {}
                        }
                        match current_block_49 {
                            12999864789452897461 => {
                                /* The remaining two modes are already valid UTF-8. */
                                if (*entry).mode as libc::c_uint
                                    & ((*entry).mode as libc::c_uint)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                    == 0
                                {
                                    sa_ctext = (sa_ctext as libc::c_ulong).wrapping_add(
                                        ((*entry).payload.data.len << shift) as libc::c_ulong,
                                    ) as size_t
                                        as size_t
                                }
                            }
                            _ => {}
                        }
                        k += 1
                    }
                }
                j += 1
            }
            /* Step 2: Convert the entries. */
            sa_text = malloc(
                sa_ctext
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            ) as *mut libc::c_char;
            sa_ntext = 0 as libc::c_int as size_t;
            /* Add the encoded Application Indicator for FNC1 in the second position. */
            if fnc1 == (1 as libc::c_int) << ZBAR_MOD_AIM as libc::c_int {
                if fnc1_2ai < 100 as libc::c_int {
                    /* The Application Indicator is a 2-digit number. */
                    let fresh3 = sa_ntext;
                    sa_ntext = sa_ntext.wrapping_add(1);
                    *sa_text.offset(fresh3 as isize) =
                        ('0' as i32 + fnc1_2ai / 10 as libc::c_int) as libc::c_char;
                    let fresh4 = sa_ntext;
                    sa_ntext = sa_ntext.wrapping_add(1);
                    *sa_text.offset(fresh4 as isize) =
                        ('0' as i32 + fnc1_2ai % 10 as libc::c_int) as libc::c_char
                } else {
                    /*The Application Indicator is a single letter.
                    We already checked that it lies in one of the ranges A...Z, a...z
                     when we decoded it.*/
                    let fresh5 = sa_ntext;
                    sa_ntext = sa_ntext.wrapping_add(1);
                    *sa_text.offset(fresh5 as isize) =
                        (fnc1_2ai - 100 as libc::c_int) as libc::c_char
                }
            }
            eci = -(1 as libc::c_int);
            enc_list[0 as libc::c_int as usize] = sjis_cd;
            enc_list[1 as libc::c_int as usize] = latin1_cd;
            enc_list[2 as libc::c_int as usize] = utf8_cd;
            eci_cd = -(1 as libc::c_int) as iconv_t;
            err = 0 as libc::c_int;
            j = 0 as libc::c_int;
            while j < sa_size && err == 0 {
                *sym = _zbar_image_scanner_alloc_sym(iscn, ZBAR_QRCODE, 0 as libc::c_int);
                (**sym).datalen = sa_ntext as libc::c_uint;
                if sa[j as usize] < 0 as libc::c_int {
                    /* generic placeholder for unfinished results */
                    (**sym).type_0 = ZBAR_PARTIAL;
                    /* Skip all contiguous missing segments. */
                    j += 1;
                    while j < sa_size && sa[j as usize] < 0 as libc::c_int {
                        j += 1
                    }
                    /* If there aren't any more, stop. */
                    if j >= sa_size {
                        break;
                    }
                    /* mark break in data */
                    let fresh6 = sa_ntext;
                    sa_ntext = sa_ntext.wrapping_add(1);
                    *sa_text.offset(fresh6 as isize) = '\u{0}' as i32 as libc::c_char;
                    (**sym).datalen = sa_ntext as libc::c_uint;
                    /* advance to next symbol */
                    sym = &mut (**sym).next;
                    *sym = _zbar_image_scanner_alloc_sym(iscn, ZBAR_QRCODE, 0 as libc::c_int)
                }
                qrdataj = qrdata.offset(sa[j as usize] as isize);
                /* expose bounding box */
                sym_add_point(
                    *sym,
                    (*qrdataj).bbox[0 as libc::c_int as usize][0 as libc::c_int as usize],
                    (*qrdataj).bbox[0 as libc::c_int as usize][1 as libc::c_int as usize],
                );
                sym_add_point(
                    *sym,
                    (*qrdataj).bbox[2 as libc::c_int as usize][0 as libc::c_int as usize],
                    (*qrdataj).bbox[2 as libc::c_int as usize][1 as libc::c_int as usize],
                );
                sym_add_point(
                    *sym,
                    (*qrdataj).bbox[3 as libc::c_int as usize][0 as libc::c_int as usize],
                    (*qrdataj).bbox[3 as libc::c_int as usize][1 as libc::c_int as usize],
                );
                sym_add_point(
                    *sym,
                    (*qrdataj).bbox[1 as libc::c_int as usize][0 as libc::c_int as usize],
                    (*qrdataj).bbox[1 as libc::c_int as usize][1 as libc::c_int as usize],
                );
                /* approx symbol "up" direction */
                dir[0 as libc::c_int as usize] = (*qrdataj).bbox[0 as libc::c_int as usize]
                    [0 as libc::c_int as usize]
                    - (*qrdataj).bbox[2 as libc::c_int as usize][0 as libc::c_int as usize]
                    + (*qrdataj).bbox[1 as libc::c_int as usize][0 as libc::c_int as usize]
                    - (*qrdataj).bbox[3 as libc::c_int as usize][0 as libc::c_int as usize];
                dir[1 as libc::c_int as usize] = (*qrdataj).bbox[2 as libc::c_int as usize]
                    [1 as libc::c_int as usize]
                    - (*qrdataj).bbox[0 as libc::c_int as usize][1 as libc::c_int as usize]
                    + (*qrdataj).bbox[3 as libc::c_int as usize][1 as libc::c_int as usize]
                    - (*qrdataj).bbox[1 as libc::c_int as usize][1 as libc::c_int as usize];
                horiz = (abs(dir[0 as libc::c_int as usize]) > abs(dir[1 as libc::c_int as usize]))
                    as libc::c_int;
                (**sym).orient = (horiz
                    + 2 as libc::c_int
                        * (dir[(1 as libc::c_int - horiz) as usize] < 0 as libc::c_int)
                            as libc::c_int) as zbar_orientation_t;
                let mut current_block_169: u64;
                k = 0 as libc::c_int;
                while k < (*qrdataj).nentries && err == 0 {
                    let mut inleft: size_t = 0;
                    let mut outleft: size_t = 0;
                    let mut in_0: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
                    entry = (*qrdataj).entries.offset(k as isize);
                    match (*entry).mode as libc::c_uint {
                        1 => {
                            if sa_ctext.wrapping_sub(sa_ntext)
                                >= (*entry).payload.data.len as size_t
                            {
                                memcpy(
                                    sa_text.offset(sa_ntext as isize) as *mut libc::c_void,
                                    (*entry).payload.data.buf as *const libc::c_void,
                                    ((*entry).payload.data.len as libc::c_ulong).wrapping_mul(
                                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                    ),
                                );
                                sa_ntext = (sa_ntext as libc::c_ulong)
                                    .wrapping_add((*entry).payload.data.len as libc::c_ulong)
                                    as size_t as size_t
                            } else {
                                err = 1 as libc::c_int
                            }
                        }
                        2 => {
                            let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
                            in_0 = (*entry).payload.data.buf as *mut libc::c_char;
                            inleft = (*entry).payload.data.len as size_t;
                            /* FNC1 uses '%' as an escape character. */
                            if fnc1 != 0 {
                                loop {
                                    let mut plen: size_t = 0;
                                    let mut c: libc::c_char = 0;
                                    p = memchr(
                                        in_0 as *const libc::c_void,
                                        '%' as i32,
                                        inleft
                                            .wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                as libc::c_ulong),
                                    ) as *mut libc::c_char;
                                    if p.is_null() {
                                        break;
                                    }
                                    plen = p.wrapping_offset_from(in_0) as libc::c_long as size_t;
                                    if sa_ctext.wrapping_sub(sa_ntext)
                                        < plen.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    {
                                        break;
                                    }
                                    memcpy(
                                        sa_text.offset(sa_ntext as isize) as *mut libc::c_void,
                                        in_0 as *const libc::c_void,
                                        plen.wrapping_mul(
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        ),
                                    );
                                    sa_ntext = (sa_ntext as libc::c_ulong).wrapping_add(plen)
                                        as size_t
                                        as size_t;
                                    /* Two '%'s is a literal '%' */
                                    if plen.wrapping_add(1 as libc::c_int as libc::c_ulong) < inleft
                                        && *p.offset(1 as libc::c_int as isize) as libc::c_int
                                            == '%' as i32
                                    {
                                        c = '%' as i32 as libc::c_char;
                                        plen = plen.wrapping_add(1);
                                        p = p.offset(1)
                                    } else {
                                        /* One '%' is the ASCII group separator. */
                                        c = 0x1d as libc::c_int as libc::c_char
                                    }
                                    let fresh7 = sa_ntext;
                                    sa_ntext = sa_ntext.wrapping_add(1);
                                    *sa_text.offset(fresh7 as isize) = c;
                                    inleft = (inleft as libc::c_ulong).wrapping_sub(
                                        plen.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                    ) as size_t
                                        as size_t;
                                    in_0 = p.offset(1 as libc::c_int as isize)
                                }
                            } else {
                                p = 0 as *mut libc::c_char
                            }
                            if !p.is_null() || sa_ctext.wrapping_sub(sa_ntext) < inleft {
                                err = 1 as libc::c_int
                            } else {
                                memcpy(
                                    sa_text.offset(sa_ntext as isize) as *mut libc::c_void,
                                    in_0 as *const libc::c_void,
                                    inleft.wrapping_mul(
                                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                    ),
                                );
                                sa_ntext = (sa_ntext as libc::c_ulong).wrapping_add(inleft)
                                    as size_t as size_t
                            }
                        }
                        4 | 8 => {
                            /*TODO: This will not handle a multi-byte sequence split between
                             multiple data blocks.
                            Does such a thing occur?
                            Is it allowed?
                            It requires copying buffers around to handle correctly.*/
                            in_0 = (*entry).payload.data.buf as *mut libc::c_char;
                            inleft = (*entry).payload.data.len as size_t;
                            out = sa_text.offset(sa_ntext as isize);
                            outleft = sa_ctext.wrapping_sub(sa_ntext);
                            /* If we have no specified encoding, attempt to auto-detect it. */
                            if eci < 0 as libc::c_int {
                                let mut ei: libc::c_int = 0;
                                /* If there was data encoded in kanji mode, assume it's SJIS. */
                                if has_kanji != 0 {
                                    enc_list_mtf(enc_list.as_mut_ptr(), sjis_cd);
                                    current_block_169 = 11016644258085151273;
                                } else if inleft >= 3 as libc::c_int as libc::c_ulong
                                    && *in_0.offset(0 as libc::c_int as isize) as libc::c_int
                                        == 0xef as libc::c_int as libc::c_char as libc::c_int
                                    && *in_0.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 0xbb as libc::c_int as libc::c_char as libc::c_int
                                    && *in_0.offset(2 as libc::c_int as isize) as libc::c_int
                                        == 0xbf as libc::c_int as libc::c_char as libc::c_int
                                {
                                    in_0 = in_0.offset(3 as libc::c_int as isize);
                                    inleft = (inleft as libc::c_ulong)
                                        .wrapping_sub(3 as libc::c_int as libc::c_ulong)
                                        as size_t
                                        as size_t;
                                    /*Otherwise check for the UTF-8 BOM.
                                    UTF-8 is rarely specified with ECI, and few decoders
                                     currently support doing so, so this is the best way for
                                     encoders to reliably indicate it.*/
                                    /* Actually try converting (to check validity). */
                                    err = (utf8_cd == -(1 as libc::c_int) as iconv_t
                                        || iconv(
                                            utf8_cd,
                                            &mut in_0,
                                            &mut inleft,
                                            &mut out,
                                            &mut outleft,
                                        ) == -(1 as libc::c_int) as size_t)
                                        as libc::c_int;
                                    if err == 0 {
                                        sa_ntext = out.wrapping_offset_from(sa_text) as libc::c_long
                                            as size_t;
                                        enc_list_mtf(enc_list.as_mut_ptr(), utf8_cd);
                                        current_block_169 = 5697748000427295508;
                                    } else {
                                        in_0 = (*entry).payload.data.buf as *mut libc::c_char;
                                        inleft = (*entry).payload.data.len as size_t;
                                        out = sa_text.offset(sa_ntext as isize);
                                        outleft = sa_ctext.wrapping_sub(sa_ntext);
                                        current_block_169 = 11016644258085151273;
                                    }
                                } else {
                                    /*If the text is 8-bit clean, prefer UTF-8 over SJIS, since
                                    SJIS will corrupt the backslashes used for DoCoMo formats.*/
                                    if text_is_ascii(
                                        in_0 as *mut libc::c_uchar,
                                        inleft as libc::c_int,
                                    ) != 0
                                    {
                                        enc_list_mtf(enc_list.as_mut_ptr(), utf8_cd);
                                    }
                                    current_block_169 = 11016644258085151273;
                                }
                                match current_block_169 {
                                    5697748000427295508 => {}
                                    _ => {
                                        /* Try our list of encodings. */
                                        ei = 0 as libc::c_int;
                                        while ei < 3 as libc::c_int {
                                            if enc_list[ei as usize]
                                                != -(1 as libc::c_int) as iconv_t
                                            {
                                                /*According to the 2005 version of the standard,
                                                 ISO/IEC 8859-1 (one hyphen) is supposed to be used, but
                                                 reality is not always so (and in the 2000 version of the
                                                 standard, it was JIS8/SJIS that was the default).
                                                It's got an invalid range that is used often with SJIS
                                                 and UTF-8, though, which makes detection easier.
                                                However, iconv() does not properly reject characters in
                                                 those ranges, since ISO-8859-1 (two hyphens) defines a
                                                 number of seldom-used control code characters there.
                                                So if we see any of those characters, move this
                                                 conversion to the end of the list.*/
                                                if ei < 2 as libc::c_int
                                                    && enc_list[ei as usize] == latin1_cd
                                                    && text_is_latin1(
                                                        in_0 as *mut libc::c_uchar,
                                                        inleft as libc::c_int,
                                                    ) == 0
                                                {
                                                    let mut ej: libc::c_int = 0;
                                                    ej = ei + 1 as libc::c_int;
                                                    while ej < 3 as libc::c_int {
                                                        enc_list
                                                            [(ej - 1 as libc::c_int) as usize] =
                                                            enc_list[ej as usize];
                                                        ej += 1
                                                    }
                                                    enc_list[2 as libc::c_int as usize] = latin1_cd
                                                }
                                                err = (iconv(
                                                    enc_list[ei as usize],
                                                    &mut in_0,
                                                    &mut inleft,
                                                    &mut out,
                                                    &mut outleft,
                                                ) == -(1 as libc::c_int) as size_t)
                                                    as libc::c_int;
                                                if err == 0 {
                                                    sa_ntext = out.wrapping_offset_from(sa_text)
                                                        as libc::c_long
                                                        as size_t;
                                                    enc_list_mtf(
                                                        enc_list.as_mut_ptr(),
                                                        enc_list[ei as usize],
                                                    );
                                                    break;
                                                } else {
                                                    in_0 = (*entry).payload.data.buf
                                                        as *mut libc::c_char;
                                                    inleft = (*entry).payload.data.len as size_t;
                                                    out = sa_text.offset(sa_ntext as isize);
                                                    outleft = sa_ctext.wrapping_sub(sa_ntext)
                                                }
                                            }
                                            ei += 1
                                        }
                                    }
                                }
                            } else {
                                /*We were actually given a character set; use it.
                                The spec says that in this case, data should be treated as if it
                                 came from the given character set even when encoded in kanji
                                 mode.*/
                                err = (eci_cd == -(1 as libc::c_int) as iconv_t
                                    || iconv(
                                        eci_cd,
                                        &mut in_0,
                                        &mut inleft,
                                        &mut out,
                                        &mut outleft,
                                    ) == -(1 as libc::c_int) as size_t)
                                    as libc::c_int;
                                if err == 0 {
                                    sa_ntext =
                                        out.wrapping_offset_from(sa_text) as libc::c_long as size_t
                                }
                            }
                        }
                        7 => {
                            /* Check to see if a character set was specified. */
                            let mut enc: *const libc::c_char = 0 as *const libc::c_char;
                            let mut buf: [libc::c_char; 16] = [0; 16];
                            let mut cur_eci: libc::c_uint = 0;
                            cur_eci = (*entry).payload.eci;
                            if cur_eci <= QR_ECI_ISO8859_16 as libc::c_int as libc::c_uint
                                && cur_eci != 14 as libc::c_int as libc::c_uint
                            {
                                if cur_eci != QR_ECI_GLI0 as libc::c_int as libc::c_uint
                                    && cur_eci != QR_ECI_CP437 as libc::c_int as libc::c_uint
                                {
                                    sprintf(
                                        buf.as_mut_ptr(),
                                        b"ISO8859-%i\x00" as *const u8 as *const libc::c_char,
                                        cur_eci
                                            .wrapping_sub(
                                                cur_eci
                                                    .wrapping_sub(3 as libc::c_int as libc::c_uint)
                                                    & -((3 as libc::c_int as libc::c_uint > cur_eci)
                                                        as libc::c_int)
                                                        as libc::c_uint,
                                            )
                                            .wrapping_sub(2 as libc::c_int as libc::c_uint),
                                    );
                                    enc = buf.as_mut_ptr()
                                } else {
                                    /*Note that CP437 requires an iconv compiled with
                                    --enable-extra-encodings, and thus may not be available.*/
                                    enc = b"CP437\x00" as *const u8 as *const libc::c_char
                                }
                                current_block_169 = 11099343707781121639;
                            } else if cur_eci == QR_ECI_SJIS as libc::c_int as libc::c_uint {
                                enc = b"SJIS\x00" as *const u8 as *const libc::c_char;
                                current_block_169 = 11099343707781121639;
                            } else if cur_eci == QR_ECI_UTF8 as libc::c_int as libc::c_uint {
                                enc = b"UTF-8\x00" as *const u8 as *const libc::c_char;
                                current_block_169 = 11099343707781121639;
                            } else {
                                /*Don't know what this ECI code specifies, but not an encoding that
                                we recognize.*/
                                current_block_169 = 5697748000427295508;
                            }
                            match current_block_169 {
                                5697748000427295508 => {}
                                _ => {
                                    eci = cur_eci as libc::c_int;
                                    eci_cd = iconv_open(
                                        b"UTF-8\x00" as *const u8 as *const libc::c_char,
                                        enc,
                                    )
                                }
                            }
                        }
                        _ => {}
                    }
                    /* Silence stupid compiler warnings. */
                    k += 1
                }
                /* If eci should be reset between codes, do so. */
                if eci <= QR_ECI_GLI1 as libc::c_int {
                    eci = -(1 as libc::c_int);
                    if eci_cd != -(1 as libc::c_int) as iconv_t {
                        iconv_close(eci_cd);
                    }
                }
                j += 1;
                sym = &mut (**sym).next
            }
            if eci_cd != -(1 as libc::c_int) as iconv_t {
                iconv_close(eci_cd);
            }
            if err == 0 {
                let mut sa_sym: *mut zbar_symbol_t = 0 as *mut zbar_symbol_t;
                let fresh8 = sa_ntext;
                sa_ntext = sa_ntext.wrapping_add(1);
                *sa_text.offset(fresh8 as isize) = '\u{0}' as i32 as libc::c_char;
                if sa_ctext.wrapping_add(1 as libc::c_int as libc::c_ulong) > sa_ntext {
                    sa_text = realloc(
                        sa_text as *mut libc::c_void,
                        sa_ntext
                            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
                    ) as *mut libc::c_char
                }
                if sa_size == 1 as libc::c_int {
                    sa_sym = syms
                } else {
                    /* cheap out w/axis aligned bbox for now */
                    let mut xmin: libc::c_int = (*img).width as libc::c_int;
                    let mut xmax: libc::c_int = -(2 as libc::c_int);
                    let mut ymin: libc::c_int = (*img).height as libc::c_int;
                    let mut ymax: libc::c_int = -(2 as libc::c_int);
                    /* create "virtual" container symbol for composite result */
                    sa_sym = _zbar_image_scanner_alloc_sym(iscn, ZBAR_QRCODE, 0 as libc::c_int);
                    (*sa_sym).syms = _zbar_symbol_set_create();
                    (*(*sa_sym).syms).head = syms;
                    /* fixup data references */
                    while !syms.is_null() {
                        let mut next: libc::c_int = 0;
                        _zbar_symbol_refcnt(syms, 1 as libc::c_int);
                        if (*syms).type_0 as libc::c_uint
                            == ZBAR_PARTIAL as libc::c_int as libc::c_uint
                        {
                            (*sa_sym).type_0 = ZBAR_PARTIAL
                        } else {
                            j = 0 as libc::c_int;
                            while (j as libc::c_uint) < (*syms).npts {
                                let mut u: libc::c_int = (*(*syms).pts.offset(j as isize)).x;
                                if xmin >= u {
                                    xmin = u - 1 as libc::c_int
                                }
                                if xmax <= u {
                                    xmax = u + 1 as libc::c_int
                                }
                                u = (*(*syms).pts.offset(j as isize)).y;
                                if ymin >= u {
                                    ymin = u - 1 as libc::c_int
                                }
                                if ymax <= u {
                                    ymax = u + 1 as libc::c_int
                                }
                                j += 1
                            }
                        }
                        (*syms).data = sa_text.offset((*syms).datalen as isize);
                        next = if !(*syms).next.is_null() {
                            (*(*syms).next).datalen as libc::c_ulong
                        } else {
                            sa_ntext
                        } as libc::c_int;
                        if next as libc::c_uint > (*syms).datalen {
                        } else {
                            __assert_fail(b"next > syms->datalen\x00" as
                                              *const u8 as
                                              *const libc::c_char,
                                          b"zbar/qrcode/qrdectxt.c\x00" as
                                              *const u8 as
                                              *const libc::c_char,
                                          405 as libc::c_int as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 102],
                                                                    &[libc::c_char; 102]>(b"int qr_code_data_list_extract_text(const qr_code_data_list *, zbar_image_scanner_t *, zbar_image_t *)\x00")).as_ptr());
                        }
                        (*syms).datalen = (next as libc::c_uint)
                            .wrapping_sub((*syms).datalen)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint);
                        syms = (*syms).next
                    }
                    if xmax >= -(1 as libc::c_int) {
                        sym_add_point(sa_sym, xmin, ymin);
                        sym_add_point(sa_sym, xmin, ymax);
                        sym_add_point(sa_sym, xmax, ymax);
                        sym_add_point(sa_sym, xmax, ymin);
                    }
                }
                (*sa_sym).data = sa_text;
                (*sa_sym).data_alloc = sa_ntext as libc::c_uint;
                (*sa_sym).datalen =
                    sa_ntext.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uint;
                (*sa_sym).modifiers = fnc1 as libc::c_uint;
                _zbar_image_scanner_add_sym(iscn, sa_sym);
            } else {
                _zbar_image_scanner_recycle_syms(iscn, syms);
                free(sa_text as *mut libc::c_void);
            }
        }
        i += 1
    }
    if utf8_cd != -(1 as libc::c_int) as iconv_t {
        iconv_close(utf8_cd);
    }
    if sjis_cd != -(1 as libc::c_int) as iconv_t {
        iconv_close(sjis_cd);
    }
    if latin1_cd != -(1 as libc::c_int) as iconv_t {
        iconv_close(latin1_cd);
    }
    free(mark as *mut libc::c_void);
    return ntext;
}
