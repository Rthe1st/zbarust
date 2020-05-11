use ::libc;
extern {
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
}
/*Copyright (C) 2008-2009  Timothy B. Terriberry (tterribe@xiph.org)
You can redistribute this library and/or modify it under the terms of the
 GNU Lesser General Public License as published by the Free Software
 Foundation; either version 2.1 of the License, or (at your option) any later
 version.*/
/* Unlike copysign(), simply inverts the sign of _a if _b is negative. */
/* Divides a signed integer by a positive value with exact rounding. */
/* Swaps two integers _a and _b if _a>_b. */
/* Computes the integer logarithm of a (positive, 32-bit) constant. */
/*Multiplies 32-bit numbers _a and _b, adds (possibly 64-bit) number _r, and
takes bits [_s,_s+31] of the result.*/
/*Multiplies 32-bit numbers _a and _b, adds (possibly 64-bit) number _r, and
gives all 64 bits of the result.*/
/*Copyright (C) 2008-2009  Timothy B. Terriberry (tterribe@xiph.org)
You can redistribute this library and/or modify it under the terms of the
 GNU Lesser General Public License as published by the Free Software
 Foundation; either version 2.1 of the License, or (at your option) any later
 version.*/
/* Computes floor(sqrt(_val)) exactly. */
#[no_mangle]
pub unsafe extern fn qr_isqrt(mut _val: libc::c_uint) -> libc::c_uint {
    let mut g: libc::c_uint = 0;
    let mut b: libc::c_uint = 0;
    let mut bshift: libc::c_int = 0;
    /*Uses the second method from
     http://www.azillionmonkeys.com/qed/sqroot.html
    The main idea is to search for the largest binary digit b such that
     (g+b)*(g+b) <= _val, and add it to the solution g.*/
    g = 0 as libc::c_int as libc::c_uint;
    b = 0x8000 as libc::c_int as libc::c_uint;
    bshift = 16 as libc::c_int;
    loop {
        let fresh0 = bshift;
        bshift = bshift - 1;
        if !(fresh0 > 0 as libc::c_int) {
            break;
        }
        let mut t: libc::c_uint = 0;
        t = (g << 1 as libc::c_int).wrapping_add(b) << bshift;
        if t <= _val {
            g = g.wrapping_add(b);
            _val = _val.wrapping_sub(t)
        }
        b >>= 1 as libc::c_int
    }
    return g;
}
/*Computes sqrt(_x*_x+_y*_y) using CORDIC.
This implementation is valid for all 32-bit inputs and returns a result
 accurate to about 27 bits of precision.
It has been tested for all postiive 16-bit inputs, where it returns correctly
 rounded results in 99.998% of cases and the maximum error is
 0.500137134862598032 (for _x=48140, _y=63018).
Very nearly all results less than (1<<16) are correctly rounded.
All Pythagorean triples with a hypotenuse of less than ((1<<27)-1) evaluate
 correctly, and the total bias over all Pythagorean triples is -0.04579, with
 a relative RMS error of 7.2864E-10 and a relative peak error of 7.4387E-9.*/
#[no_mangle]
pub unsafe extern fn qr_ihypot(mut _x: libc::c_int, mut _y: libc::c_int) -> libc::c_uint {
    let mut x: libc::c_uint = 0;
    let mut y: libc::c_uint = 0;
    let mut mask: libc::c_int = 0;
    let mut shift: libc::c_int = 0;
    let mut u: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    _x = abs(_x);
    x = _x as libc::c_uint;
    _y = abs(_y);
    y = _y as libc::c_uint;
    mask = -((x > y) as libc::c_int) & (_x ^ _y);
    x ^= mask as libc::c_uint;
    y ^= mask as libc::c_uint;
    _y ^= mask;
    shift = 31 as libc::c_int - qr_ilog(y);
    shift = shift - (shift - 0 as libc::c_int & -((0 as libc::c_int > shift) as libc::c_int));
    x = (((x << shift) as libc::c_ulonglong).wrapping_mul(0x9b74edaa as libc::c_ulonglong)
        >> 32 as libc::c_int) as libc::c_uint;
    _y = ((_y << shift) as libc::c_longlong * 0x9b74eda9 as libc::c_longlong >> 32 as libc::c_int)
        as libc::c_int;
    u = x as libc::c_int;
    mask = -((_y < 0 as libc::c_int) as libc::c_int);
    x = x.wrapping_add((_y + mask ^ mask) as libc::c_uint);
    _y -= u + mask ^ mask;
    u = (x.wrapping_add(1 as libc::c_int as libc::c_uint) >> 1 as libc::c_int) as libc::c_int;
    v = _y + 1 as libc::c_int >> 1 as libc::c_int;
    mask = -((_y < 0 as libc::c_int) as libc::c_int);
    x = x.wrapping_add((v + mask ^ mask) as libc::c_uint);
    _y -= u + mask ^ mask;
    i = 1 as libc::c_int;
    while i < 16 as libc::c_int {
        let mut r: libc::c_int = 0;
        u = (x.wrapping_add(1 as libc::c_int as libc::c_uint) >> 2 as libc::c_int) as libc::c_int;
        r = (1 as libc::c_int) << 2 as libc::c_int * i >> 1 as libc::c_int;
        v = _y + r >> 2 as libc::c_int * i;
        mask = -((_y < 0 as libc::c_int) as libc::c_int);
        x = x.wrapping_add((v + mask ^ mask) as libc::c_uint);
        _y = _y - (u + mask ^ mask) << 1 as libc::c_int;
        i += 1
    }
    return x.wrapping_add((1 as libc::c_uint) << shift >> 1 as libc::c_int) >> shift;
}
#[no_mangle]
pub unsafe extern fn qr_ilog(mut _v: libc::c_uint) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    m = ((_v & 0xffff0000 as libc::c_uint != 0) as libc::c_int) << 4 as libc::c_int;
    _v >>= m;
    ret = m;
    m = ((_v & 0xff00 as libc::c_int as libc::c_uint != 0) as libc::c_int) << 3 as libc::c_int;
    _v >>= m;
    ret |= m;
    m = ((_v & 0xf0 as libc::c_int as libc::c_uint != 0) as libc::c_int) << 2 as libc::c_int;
    _v >>= m;
    ret |= m;
    m = ((_v & 0xc as libc::c_int as libc::c_uint != 0) as libc::c_int) << 1 as libc::c_int;
    _v >>= m;
    ret |= m;
    ret |= (_v & 0x2 as libc::c_int as libc::c_uint != 0) as libc::c_int;
    return ret + (_v != 0) as libc::c_int;
}
