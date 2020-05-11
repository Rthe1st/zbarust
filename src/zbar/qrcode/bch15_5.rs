use ::libc;
/*Copyright (C) 2008-2009  Timothy B. Terriberry (tterribe@xiph.org)
  You can redistribute this library and/or modify it under the terms of the
   GNU Lesser General Public License as published by the Free Software
   Foundation; either version 2.1 of the License, or (at your option) any later
   version.*/
/*A cycle in GF(2**4) generated by alpha=(x**4+x+1).
  It is extended an extra 16 entries to avoid some expensive mod operations.*/
static mut gf16_exp: [libc::c_uchar; 31] =
    [1 as libc::c_int as libc::c_uchar, 2 as libc::c_int as libc::c_uchar,
     4 as libc::c_int as libc::c_uchar, 8 as libc::c_int as libc::c_uchar,
     3 as libc::c_int as libc::c_uchar, 6 as libc::c_int as libc::c_uchar,
     12 as libc::c_int as libc::c_uchar, 11 as libc::c_int as libc::c_uchar,
     5 as libc::c_int as libc::c_uchar, 10 as libc::c_int as libc::c_uchar,
     7 as libc::c_int as libc::c_uchar, 14 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 13 as libc::c_int as libc::c_uchar,
     9 as libc::c_int as libc::c_uchar, 1 as libc::c_int as libc::c_uchar,
     2 as libc::c_int as libc::c_uchar, 4 as libc::c_int as libc::c_uchar,
     8 as libc::c_int as libc::c_uchar, 3 as libc::c_int as libc::c_uchar,
     6 as libc::c_int as libc::c_uchar, 12 as libc::c_int as libc::c_uchar,
     11 as libc::c_int as libc::c_uchar, 5 as libc::c_int as libc::c_uchar,
     10 as libc::c_int as libc::c_uchar, 7 as libc::c_int as libc::c_uchar,
     14 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     13 as libc::c_int as libc::c_uchar, 9 as libc::c_int as libc::c_uchar,
     1 as libc::c_int as libc::c_uchar];
/*The location of each integer 1...16 in the cycle.*/
static mut gf16_log: [libc::c_schar; 16] =
    [-(1 as libc::c_int) as libc::c_schar, 0 as libc::c_int as libc::c_schar,
     1 as libc::c_int as libc::c_schar, 4 as libc::c_int as libc::c_schar,
     2 as libc::c_int as libc::c_schar, 8 as libc::c_int as libc::c_schar,
     5 as libc::c_int as libc::c_schar, 10 as libc::c_int as libc::c_schar,
     3 as libc::c_int as libc::c_schar, 14 as libc::c_int as libc::c_schar,
     9 as libc::c_int as libc::c_schar, 7 as libc::c_int as libc::c_schar,
     6 as libc::c_int as libc::c_schar, 13 as libc::c_int as libc::c_schar,
     11 as libc::c_int as libc::c_schar, 12 as libc::c_int as libc::c_schar];
/*Multiplication in GF(2**4) using logarithms.*/
unsafe extern "C" fn gf16_mul(mut _a: libc::c_uint, mut _b: libc::c_uint)
 -> libc::c_uint {
    return if _a == 0 as libc::c_int as libc::c_uint ||
                  _b == 0 as libc::c_int as libc::c_uint {
               0 as libc::c_int
           } else {
               gf16_exp[(gf16_log[_a as usize] as libc::c_int +
                             gf16_log[_b as usize] as libc::c_int) as usize]
                   as libc::c_int
           } as libc::c_uint;
}
/*Division in GF(2**4) using logarithms.
  The result when dividing by zero is undefined.*/
unsafe extern "C" fn gf16_div(mut _a: libc::c_uint, mut _b: libc::c_uint)
 -> libc::c_uint {
    return if _a == 0 as libc::c_int as libc::c_uint {
               0 as libc::c_int
           } else {
               gf16_exp[(gf16_log[_a as usize] as libc::c_int +
                             15 as libc::c_int -
                             gf16_log[_b as usize] as libc::c_int) as usize]
                   as libc::c_int
           } as libc::c_uint;
}
/*Multiplication in GF(2**4) when the second argument is known to be non-zero
   (proven by representing it by its logarithm).*/
unsafe extern "C" fn gf16_hmul(mut _a: libc::c_uint, mut _logb: libc::c_uint)
 -> libc::c_uint {
    return if _a == 0 as libc::c_int as libc::c_uint {
               0 as libc::c_int
           } else {
               gf16_exp[(gf16_log[_a as usize] as
                             libc::c_uint).wrapping_add(_logb) as usize] as
                   libc::c_int
           } as libc::c_uint;
}
/*The syndrome normally has five values, S_1 ... S_5.
  We only calculate and store the odd ones in _s, since S_2=S_1**2 and
   S_4=S_2**2.
  Returns zero iff all the syndrome values are zero.*/
unsafe extern "C" fn bch15_5_calc_syndrome(mut _s: *mut libc::c_uint,
                                           mut _y: libc::c_uint)
 -> libc::c_int {
    let mut p: libc::c_uint = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    p = 0 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int;
    while i < 15 as libc::c_int {
        if _y & ((1 as libc::c_int) << i) as libc::c_uint != 0 {
            p ^= gf16_exp[i as usize] as libc::c_uint
        }
        i += 1
    }
    *_s.offset(0 as libc::c_int as isize) = p;
    p = 0 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 5 as libc::c_int {
            if _y &
                   ((1 as libc::c_int) << 5 as libc::c_int * i + j) as
                       libc::c_uint != 0 {
                p ^= gf16_exp[(j * 3 as libc::c_int) as usize] as libc::c_uint
            }
            j += 1
        }
        i += 1
    }
    *_s.offset(1 as libc::c_int as isize) = p;
    p = 0 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            if _y &
                   ((1 as libc::c_int) << 3 as libc::c_int * i + j) as
                       libc::c_uint != 0 {
                p ^= gf16_exp[(j * 5 as libc::c_int) as usize] as libc::c_uint
            }
            j += 1
        }
        i += 1
    }
    *_s.offset(2 as libc::c_int as isize) = p;
    return (*_s.offset(0 as libc::c_int as isize) !=
                0 as libc::c_int as libc::c_uint ||
                *_s.offset(1 as libc::c_int as isize) !=
                    0 as libc::c_int as libc::c_uint ||
                *_s.offset(2 as libc::c_int as isize) !=
                    0 as libc::c_int as libc::c_uint) as libc::c_int;
}
/*Compute the coefficients of the error-locator polynomial.
  Returns the number of errors (the degree of the polynomial).*/
unsafe extern "C" fn bch15_5_calc_omega(mut _o: *mut libc::c_uint,
                                        mut _s: *mut libc::c_uint)
 -> libc::c_int {
    let mut s02: libc::c_uint = 0;
    let mut tt: libc::c_uint = 0;
    let mut dd: libc::c_uint = 0;
    let mut d: libc::c_int = 0;
    *_o.offset(0 as libc::c_int as isize) =
        *_s.offset(0 as libc::c_int as isize);
    s02 =
        gf16_mul(*_s.offset(0 as libc::c_int as isize),
                 *_s.offset(0 as libc::c_int as isize));
    dd =
        *_s.offset(1 as libc::c_int as isize) ^
            gf16_mul(*_s.offset(0 as libc::c_int as isize), s02);
    tt =
        *_s.offset(2 as libc::c_int as isize) ^
            gf16_mul(s02, *_s.offset(1 as libc::c_int as isize));
    *_o.offset(1 as libc::c_int as isize) =
        if dd != 0 {
            gf16_div(tt, dd)
        } else { 0 as libc::c_int as libc::c_uint };
    *_o.offset(2 as libc::c_int as isize) =
        dd ^
            gf16_mul(*_s.offset(0 as libc::c_int as isize),
                     *_o.offset(1 as libc::c_int as isize));
    d = 3 as libc::c_int;
    while d > 0 as libc::c_int &&
              *_o.offset((d - 1 as libc::c_int) as isize) == 0 {
        d -= 1
    }
    return d;
}
/*Find the roots of the error polynomial.
  Returns the number of roots found, or a negative value if the polynomial did
   not have enough roots, indicating a decoding error.*/
unsafe extern "C" fn bch15_5_calc_epos(mut _epos: *mut libc::c_uint,
                                       mut _s: *mut libc::c_uint)
 -> libc::c_int {
    let mut o: [libc::c_uint; 3] = [0; 3];
    let mut nerrors: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    d = bch15_5_calc_omega(o.as_mut_ptr(), _s);
    nerrors = 0 as libc::c_int;
    if d == 1 as libc::c_int {
        let fresh0 = nerrors;
        nerrors = nerrors + 1;
        *_epos.offset(fresh0 as isize) =
            gf16_log[o[0 as libc::c_int as usize] as usize] as libc::c_uint
    } else if d > 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 15 as libc::c_int {
            let mut i2: libc::c_int = 0;
            i2 =
                gf16_log[gf16_exp[(i << 1 as libc::c_int) as usize] as usize]
                    as libc::c_int;
            if gf16_exp[(i + i2) as usize] as libc::c_uint ^
                   gf16_hmul(o[0 as libc::c_int as usize], i2 as libc::c_uint)
                   ^
                   gf16_hmul(o[1 as libc::c_int as usize], i as libc::c_uint)
                   ^ o[2 as libc::c_int as usize] == 0 {
                let fresh1 = nerrors;
                nerrors = nerrors + 1;
                *_epos.offset(fresh1 as isize) = i as libc::c_uint
            }
            i += 1
        }
        if nerrors < d { return -(1 as libc::c_int) }
    }
    return nerrors;
}
/*Corrects the received code *_y, if possible.
  The original data is located in the top five bits.
  Returns the number of errors corrected, or a negative value if decoding
   failed due to too many bit errors, in which case *_y is left unchanged.*/
#[no_mangle]
pub unsafe extern "C" fn bch15_5_correct(mut _y: *mut libc::c_uint)
 -> libc::c_int {
    let mut s: [libc::c_uint; 3] = [0; 3];
    let mut epos: [libc::c_uint; 3] = [0; 3];
    let mut y: libc::c_uint = 0;
    let mut nerrors: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    y = *_y;
    if bch15_5_calc_syndrome(s.as_mut_ptr(), y) == 0 {
        return 0 as libc::c_int
    }
    nerrors = bch15_5_calc_epos(epos.as_mut_ptr(), s.as_mut_ptr());
    if nerrors > 0 as libc::c_int {
        /*If we had a non-zero syndrome value, we should always find at least one
       error location, or we've got a decoding error.*/
        i = 0 as libc::c_int;
        while i < nerrors {
            y ^= ((1 as libc::c_int) << epos[i as usize]) as libc::c_uint;
            i += 1
        }
        /*If there were too many errors, we may not find enough roots to reduce the
       syndrome to zero.
      We could recompute it to check, but it's much faster just to check that
       we have a valid codeword.*/
        if bch15_5_encode(y >> 10 as libc::c_int) == y {
            /*Decoding succeeded.*/
            *_y = y;
            return nerrors
        }
    }
    /*Decoding failed due to too many bit errors.*/
    return -(1 as libc::c_int);
}
/*Copyright (C) 2008-2009  Timothy B. Terriberry (tterribe@xiph.org)
  You can redistribute this library and/or modify it under the terms of the
   GNU Lesser General Public License as published by the Free Software
   Foundation; either version 2.1 of the License, or (at your option) any later
   version.*/
/*Encodes a raw 5-bit value _x into a 15-bit BCH(15,5) code.
  This is capable of correcting up to 3 bit errors, and detecting as many as
   5 bit errors in some cases.*/
#[no_mangle]
pub unsafe extern "C" fn bch15_5_encode(mut _x: libc::c_uint)
 -> libc::c_uint {
    return (_x & 1 as libc::c_int as libc::c_uint).wrapping_neg() &
               0x537 as libc::c_int as libc::c_uint ^
               (_x >> 1 as libc::c_int &
                    1 as libc::c_int as libc::c_uint).wrapping_neg() &
                   0xa6e as libc::c_int as libc::c_uint ^
               (_x >> 2 as libc::c_int &
                    1 as libc::c_int as libc::c_uint).wrapping_neg() &
                   0x11eb as libc::c_int as libc::c_uint ^
               (_x >> 3 as libc::c_int &
                    1 as libc::c_int as libc::c_uint).wrapping_neg() &
                   0x23d6 as libc::c_int as libc::c_uint ^
               (_x >> 4 as libc::c_int &
                    1 as libc::c_int as libc::c_uint).wrapping_neg() &
                   0x429b as libc::c_int as libc::c_uint;
}
