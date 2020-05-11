use ::libc;
extern {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rs_gf256 {
    pub log: [libc::c_uchar; 256],
    pub exp: [libc::c_uchar; 511],
}
/*Initialize discrete logarithm tables for GF(2**8) using a given primitive
irreducible polynomial.*/
/*Copyright (C) 1991-1995  Henry Minsky (hqm@ua.com, hqm@ai.mit.edu)
Copyright (C) 2008-2009  Timothy B. Terriberry (tterribe@xiph.org)
You can redistribute this library and/or modify it under the terms of the
 GNU Lesser General Public License as published by the Free Software
 Foundation; either version 2.1 of the License, or (at your option) any later
 version.*/
/*Reed-Solomon encoder and decoder.
Original implementation (C) Henry Minsky (hqm@ua.com, hqm@ai.mit.edu),
 Universal Access 1991-1995.
Updates by Timothy B. Terriberry (C) 2008-2009:
 - Properly reject codes when error-locator polynomial has repeated roots or
    non-trivial irreducible factors.
 - Removed the hard-coded parity size and performed general API cleanup.
 - Allow multiple representations of GF(2**8), since different standards use
    different irreducible polynomials.
 - Allow different starting indices for the generator polynomial, since
    different standards use different values.
 - Greatly reduced the computation by eliminating unnecessary operations.
 - Explicitly solve for the roots of low-degree polynomials instead of using
    an exhaustive search.
   This is another major speed boost when there are few errors.*/
/* Galois Field arithmetic in GF(2**8). */
#[no_mangle]
pub unsafe extern fn rs_gf256_init(mut _gf: *mut rs_gf256, mut _ppoly: libc::c_uint) {
    let mut p: libc::c_uint = 0;
    let mut i: libc::c_int = 0;
    /* Initialize the table of powers of a primtive root, alpha=0x02. */
    p = 1 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        (*_gf).exp[(i + 255 as libc::c_int) as usize] = p as libc::c_uchar;
        (*_gf).exp[i as usize] = (*_gf).exp[(i + 255 as libc::c_int) as usize];
        p = (p << 1 as libc::c_int ^ (p >> 7 as libc::c_int).wrapping_neg() & _ppoly)
            & 0xff as libc::c_int as libc::c_uint;
        i += 1
    }
    /* Invert the table to recover the logs. */
    i = 0 as libc::c_int;
    while i < 255 as libc::c_int {
        (*_gf).log[(*_gf).exp[i as usize] as usize] = i as libc::c_uchar;
        i += 1
    }
    /* Note that we rely on the fact that _gf->log[0]=0 below. */
    (*_gf).log[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
}
/* Multiplication in GF(2**8) using logarithms. */
unsafe extern fn rs_gmul(
    mut _gf: *const rs_gf256,
    mut _a: libc::c_uint,
    mut _b: libc::c_uint,
) -> libc::c_uint {
    return if _a == 0 as libc::c_int as libc::c_uint || _b == 0 as libc::c_int as libc::c_uint {
        0 as libc::c_int
    } else {
        (*_gf).exp[((*_gf).log[_a as usize] as libc::c_int + (*_gf).log[_b as usize] as libc::c_int)
            as usize] as libc::c_int
    } as libc::c_uint;
}
/*Division in GF(2**8) using logarithms.
The result of division by zero is undefined.*/
unsafe extern fn rs_gdiv(
    mut _gf: *const rs_gf256,
    mut _a: libc::c_uint,
    mut _b: libc::c_uint,
) -> libc::c_uint {
    return if _a == 0 as libc::c_int as libc::c_uint {
        0 as libc::c_int
    } else {
        (*_gf).exp[((*_gf).log[_a as usize] as libc::c_int + 255 as libc::c_int
            - (*_gf).log[_b as usize] as libc::c_int) as usize] as libc::c_int
    } as libc::c_uint;
}
/*Multiplication in GF(2**8) when one of the numbers is known to be non-zero
(proven by representing it by its logarithm).*/
unsafe extern fn rs_hgmul(
    mut _gf: *const rs_gf256,
    mut _a: libc::c_uint,
    mut _logb: libc::c_uint,
) -> libc::c_uint {
    return if _a == 0 as libc::c_int as libc::c_uint {
        0 as libc::c_int
    } else {
        (*_gf).exp[((*_gf).log[_a as usize] as libc::c_uint).wrapping_add(_logb) as usize]
            as libc::c_int
    } as libc::c_uint;
}
/* Square root in GF(2**8) using logarithms. */
unsafe extern fn rs_gsqrt(mut _gf: *const rs_gf256, mut _a: libc::c_uint) -> libc::c_uint {
    let mut loga: libc::c_uint = 0;
    if _a == 0 {
        return 0 as libc::c_int as libc::c_uint;
    }
    loga = (*_gf).log[_a as usize] as libc::c_uint;
    return (*_gf).exp[(loga.wrapping_add(
        255 as libc::c_int as libc::c_uint
            & (loga & 1 as libc::c_int as libc::c_uint).wrapping_neg(),
    ) >> 1 as libc::c_int) as usize] as libc::c_uint;
}
/*Polynomial root finding in GF(2**8).
Each routine returns a list of the distinct roots (i.e., with duplicates
 removed).*/
/*Solve a quadratic equation x**2 + _b*x + _c in GF(2**8) using the method
 of~\cite{Wal99}.
Returns the number of distinct roots.
ARTICLE{Wal99,
  author="C. Wayne Walker",
  title="New Formulas for Solving Quadratic Equations over Certain Finite
   Fields",
  journal="{IEEE} Transactions on Information Theory",
  volume=45,
  number=1,
  pages="283--284",
  month=Jan,
  year=1999
}*/
unsafe extern fn rs_quadratic_solve(
    mut _gf: *const rs_gf256,
    mut _b: libc::c_uint,
    mut _c: libc::c_uint,
    mut _x: *mut libc::c_uchar,
) -> libc::c_int {
    let mut b: libc::c_uint = 0;
    let mut logb: libc::c_uint = 0;
    let mut logb2: libc::c_uint = 0;
    let mut logb4: libc::c_uint = 0;
    let mut logb8: libc::c_uint = 0;
    let mut logb12: libc::c_uint = 0;
    let mut logb14: libc::c_uint = 0;
    let mut logc: libc::c_uint = 0;
    let mut logc2: libc::c_uint = 0;
    let mut logc4: libc::c_uint = 0;
    let mut c8: libc::c_uint = 0;
    let mut g3: libc::c_uint = 0;
    let mut z3: libc::c_uint = 0;
    let mut l3: libc::c_uint = 0;
    let mut c0: libc::c_uint = 0;
    let mut g2: libc::c_uint = 0;
    let mut l2: libc::c_uint = 0;
    let mut z2: libc::c_uint = 0;
    let mut inc: libc::c_int = 0;
    /* If _b is zero, all we need is a square root. */
    if _b == 0 {
        *_x.offset(0 as libc::c_int as isize) = rs_gsqrt(_gf, _c) as libc::c_uchar;
        return 1 as libc::c_int;
    }
    /* If _c is zero, 0 and _b are the roots. */
    if _c == 0 {
        *_x.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
        *_x.offset(1 as libc::c_int as isize) = _b as libc::c_uchar;
        return 2 as libc::c_int;
    }
    logb = (*_gf).log[_b as usize] as libc::c_uint;
    logc = (*_gf).log[_c as usize] as libc::c_uint;
    /* If _b lies in GF(2**4), scale x to move it out. */
    inc = (logb.wrapping_rem((255 as libc::c_int / 15 as libc::c_int) as libc::c_uint)
        == 0 as libc::c_int as libc::c_uint) as libc::c_int;
    if inc != 0 {
        b = (*_gf).exp[logb.wrapping_add(254 as libc::c_int as libc::c_uint) as usize]
            as libc::c_uint;
        logb = (*_gf).log[b as usize] as libc::c_uint;
        _c = (*_gf).exp[logc.wrapping_add(253 as libc::c_int as libc::c_uint) as usize]
            as libc::c_uint;
        logc = (*_gf).log[_c as usize] as libc::c_uint
    } else {
        b = _b
    }
    logb2 = (*_gf).log[(*_gf).exp[(logb << 1 as libc::c_int) as usize] as usize] as libc::c_uint;
    logb4 = (*_gf).log[(*_gf).exp[(logb2 << 1 as libc::c_int) as usize] as usize] as libc::c_uint;
    logb8 = (*_gf).log[(*_gf).exp[(logb4 << 1 as libc::c_int) as usize] as usize] as libc::c_uint;
    logb12 = (*_gf).log[(*_gf).exp[logb4.wrapping_add(logb8) as usize] as usize] as libc::c_uint;
    logb14 = (*_gf).log[(*_gf).exp[logb2.wrapping_add(logb12) as usize] as usize] as libc::c_uint;
    logc2 = (*_gf).log[(*_gf).exp[(logc << 1 as libc::c_int) as usize] as usize] as libc::c_uint;
    logc4 = (*_gf).log[(*_gf).exp[(logc2 << 1 as libc::c_int) as usize] as usize] as libc::c_uint;
    c8 = (*_gf).exp[(logc4 << 1 as libc::c_int) as usize] as libc::c_uint;
    g3 = rs_hgmul(
        _gf,
        ((*_gf).exp[logb14.wrapping_add(logc) as usize] as libc::c_int
            ^ (*_gf).exp[logb12.wrapping_add(logc2) as usize] as libc::c_int
            ^ (*_gf).exp[logb8.wrapping_add(logc4) as usize] as libc::c_int)
            as libc::c_uint
            ^ c8,
        logb,
    );
    /*If g3 doesn't lie in GF(2**4), then our roots lie in an extension field.
    Note that we rely on the fact that _gf->log[0]==0 here.*/
    if (*_gf).log[g3 as usize] as libc::c_int % (255 as libc::c_int / 15 as libc::c_int)
        != 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    /*Construct the corresponding quadratic in GF(2**4):
    x**2 + x/alpha**(255/15) + l3/alpha**(2*(255/15))*/
    z3 = rs_gdiv(_gf, g3, (*_gf).exp[(logb8 << 1 as libc::c_int) as usize] as libc::c_uint ^ b);
    l3 = rs_hgmul(
        _gf,
        rs_gmul(_gf, z3, z3) ^ rs_hgmul(_gf, z3, logb) ^ _c,
        (255 as libc::c_int as libc::c_uint).wrapping_sub(logb2),
    );
    c0 = rs_hgmul(
        _gf,
        l3,
        (255 as libc::c_int - 2 as libc::c_int * (255 as libc::c_int / 15 as libc::c_int))
            as libc::c_uint,
    );
    /*Construct the corresponding quadratic in GF(2**2):
    x**2 + x/alpha**(255/3) + l2/alpha**(2*(255/3))*/
    g2 = rs_hgmul(
        _gf,
        rs_hgmul(
            _gf,
            c0,
            (255 as libc::c_int - 2 as libc::c_int * (255 as libc::c_int / 15 as libc::c_int))
                as libc::c_uint,
        ) ^ rs_gmul(_gf, c0, c0),
        (255 as libc::c_int - 255 as libc::c_int / 15 as libc::c_int) as libc::c_uint,
    );
    z2 = rs_gdiv(
        _gf,
        g2,
        ((*_gf).exp[(255 as libc::c_int - 255 as libc::c_int / 15 as libc::c_int * 4 as libc::c_int)
            as usize] as libc::c_int
            ^ (*_gf).exp[(255 as libc::c_int - 255 as libc::c_int / 15 as libc::c_int) as usize]
                as libc::c_int) as libc::c_uint,
    );
    l2 = rs_hgmul(
        _gf,
        rs_gmul(_gf, z2, z2)
            ^ rs_hgmul(
                _gf,
                z2,
                (255 as libc::c_int - 255 as libc::c_int / 15 as libc::c_int) as libc::c_uint,
            )
            ^ c0,
        (2 as libc::c_int * (255 as libc::c_int / 15 as libc::c_int)) as libc::c_uint,
    );
    /* Back substitute to the solution in the original field. */
    *_x.offset(0 as libc::c_int as isize) = (*_gf).exp[((*_gf).log[(z3
        ^ rs_hgmul(
            _gf,
            rs_hgmul(_gf, l2, (255 as libc::c_int / 3 as libc::c_int) as libc::c_uint)
                ^ rs_hgmul(_gf, z2, (255 as libc::c_int / 15 as libc::c_int) as libc::c_uint),
            logb,
        )) as usize] as libc::c_int
        + inc) as usize];
    *_x.offset(1 as libc::c_int as isize) =
        (*_x.offset(0 as libc::c_int as isize) as libc::c_uint ^ _b) as libc::c_uchar;
    return 2 as libc::c_int;
}
/*Solve a cubic equation x**3 + _a*x**2 + _b*x + _c in GF(2**8).
Returns the number of distinct roots.*/
unsafe extern fn rs_cubic_solve(
    mut _gf: *const rs_gf256,
    mut _a: libc::c_uint,
    mut _b: libc::c_uint,
    mut _c: libc::c_uint,
    mut _x: *mut libc::c_uchar,
) -> libc::c_int {
    let mut k: libc::c_uint = 0;
    let mut logd: libc::c_uint = 0;
    let mut d2: libc::c_uint = 0;
    let mut logd2: libc::c_uint = 0;
    let mut logw: libc::c_uint = 0;
    let mut nroots: libc::c_int = 0;
    /* If _c is zero, factor out the 0 root. */
    if _c == 0 {
        nroots = rs_quadratic_solve(_gf, _a, _b, _x);
        if _b != 0 {
            let fresh0 = nroots;
            nroots = nroots + 1;
            *_x.offset(fresh0 as isize) = 0 as libc::c_int as libc::c_uchar
        }
        return nroots;
    }
    /*Substitute x=_a+y*sqrt(_a**2+_b) to get y**3 + y + k == 0,
    k = (_a*_b+c)/(_a**2+b)**(3/2).*/
    k = rs_gmul(_gf, _a, _b) ^ _c;
    d2 = rs_gmul(_gf, _a, _a) ^ _b;
    if d2 == 0 {
        let mut logx: libc::c_int = 0;
        if k == 0 {
            /* We have a triple root. */
            *_x.offset(0 as libc::c_int as isize) = _a as libc::c_uchar;
            return 1 as libc::c_int;
        }
        logx = (*_gf).log[k as usize] as libc::c_int;
        if logx % 3 as libc::c_int != 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        logx /= 3 as libc::c_int;
        *_x.offset(0 as libc::c_int as isize) =
            (_a ^ (*_gf).exp[logx as usize] as libc::c_uint) as libc::c_uchar;
        *_x.offset(1 as libc::c_int as isize) = (_a
            ^ (*_gf).exp[(logx + 255 as libc::c_int / 3 as libc::c_int) as usize] as libc::c_uint)
            as libc::c_uchar;
        *_x.offset(2 as libc::c_int as isize) = (_a
            ^ *_x.offset(0 as libc::c_int as isize) as libc::c_uint
            ^ *_x.offset(1 as libc::c_int as isize) as libc::c_uint)
            as libc::c_uchar;
        return 3 as libc::c_int;
    }
    logd2 = (*_gf).log[d2 as usize] as libc::c_uint;
    logd = logd2.wrapping_add(
        255 as libc::c_int as libc::c_uint
            & (logd2 & 1 as libc::c_int as libc::c_uint).wrapping_neg(),
    ) >> 1 as libc::c_int;
    k = rs_gdiv(_gf, k, (*_gf).exp[logd.wrapping_add(logd2) as usize] as libc::c_uint);
    /* Substitute y=w+1/w and z=w**3 to get z**2 + k*z + 1 == 0. */
    nroots = rs_quadratic_solve(_gf, k, 1 as libc::c_int as libc::c_uint, _x);
    if nroots < 1 as libc::c_int {
        /*The Reed-Solomon code is only valid if we can find 3 distinct roots in
         GF(2**8), so if we know there's only one, we don't actually need to find
         it.
        Note that we're also called by the quartic solver, but if we contain a
         non-trivial irreducible factor, than so does the original
         quartic~\cite{LW72}, and failing to return a root here actually saves us
         some work there, also.*/
        return 0 as libc::c_int;
    }
    /* Recover w from z. */
    logw = (*_gf).log[*_x.offset(0 as libc::c_int as isize) as usize] as libc::c_uint;
    if logw != 0 {
        if logw.wrapping_rem(3 as libc::c_int as libc::c_uint) != 0 as libc::c_int as libc::c_uint {
            return 0 as libc::c_int;
        }
        logw = logw.wrapping_div(3 as libc::c_int as libc::c_uint);
        /* Recover x from w. */
        *_x.offset(0 as libc::c_int as isize) =
            ((*_gf).exp[((*_gf).log[((*_gf).exp[logw as usize] as libc::c_int
                ^ (*_gf).exp[(255 as libc::c_int as libc::c_uint).wrapping_sub(logw) as usize]
                    as libc::c_int) as usize] as libc::c_uint)
                .wrapping_add(logd) as usize] as libc::c_uint
                ^ _a) as libc::c_uchar;
        logw = logw.wrapping_add((255 as libc::c_int / 3 as libc::c_int) as libc::c_uint);
        *_x.offset(1 as libc::c_int as isize) =
            ((*_gf).exp[((*_gf).log[((*_gf).exp[logw as usize] as libc::c_int
                ^ (*_gf).exp[(255 as libc::c_int as libc::c_uint).wrapping_sub(logw) as usize]
                    as libc::c_int) as usize] as libc::c_uint)
                .wrapping_add(logd) as usize] as libc::c_uint
                ^ _a) as libc::c_uchar;
        *_x.offset(2 as libc::c_int as isize) = ((*_x.offset(0 as libc::c_int as isize)
            as libc::c_int
            ^ *_x.offset(1 as libc::c_int as isize) as libc::c_int)
            as libc::c_uint
            ^ _a) as libc::c_uchar;
        return 3 as libc::c_int;
    } else {
        *_x.offset(0 as libc::c_int as isize) = _a as libc::c_uchar;
        /*In this case _x[1] is a double root, so we know the Reed-Solomon code is
         invalid.
        Note that we still have to return at least one root, because if we're
         being called by the quartic solver, the quartic might still have 4
         distinct roots.
        But we don't need more than one root, so we can avoid computing the
         expensive one.*/
        /* _x[1]=_gf->exp[_gf->log[_gf->exp[255/3]^_gf->exp[2*(255/3)]]+logd]^_a; */
        return 1 as libc::c_int;
    };
}
/*Solve a quartic equation x**4 + _a*x**3 + _b*x**2 + _c*x + _d in GF(2**8) by
 decomposing it into the cases given by~\cite{LW72}.
Returns the number of distinct roots.
@ARTICLE{LW72,
  author="Philip A. Leonard and Kenneth S. Williams",
  title="Quartics over $GF(2^n)$",
  journal="Proceedings of the American Mathematical Society",
  volume=36,
  number=2,
  pages="347--450",
  month=Dec,
  year=1972
}*/
unsafe extern fn rs_quartic_solve(
    mut _gf: *const rs_gf256,
    mut _a: libc::c_uint,
    mut _b: libc::c_uint,
    mut _c: libc::c_uint,
    mut _d: libc::c_uint,
    mut _x: *mut libc::c_uchar,
) -> libc::c_int {
    let mut r: libc::c_uint = 0;
    let mut s: libc::c_uint = 0;
    let mut t: libc::c_uint = 0;
    let mut b: libc::c_uint = 0;
    let mut nroots: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    /* If _d is zero, factor out the 0 root. */
    if _d == 0 {
        nroots = rs_cubic_solve(_gf, _a, _b, _c, _x);
        if _c != 0 {
            let fresh1 = nroots;
            nroots = nroots + 1;
            *_x.offset(fresh1 as isize) = 0 as libc::c_int as libc::c_uchar
        }
        return nroots;
    }
    if _a != 0 {
        let mut loga: libc::c_uint = 0;
        /* Substitute x=(1/y) + sqrt(_c/_a) to eliminate the cubic term. */
        loga = (*_gf).log[_a as usize] as libc::c_uint;
        r = rs_hgmul(_gf, _c, (255 as libc::c_int as libc::c_uint).wrapping_sub(loga));
        s = rs_gsqrt(_gf, r);
        t = _d ^ rs_gmul(_gf, _b, r) ^ rs_gmul(_gf, r, r);
        if t != 0 {
            let mut logti: libc::c_uint = 0;
            logti = (255 as libc::c_int - (*_gf).log[t as usize] as libc::c_int) as libc::c_uint;
            /* The result is still quartic, but with no cubic term. */
            nroots = rs_quartic_solve(
                _gf,
                0 as libc::c_int as libc::c_uint,
                rs_hgmul(_gf, _b ^ rs_hgmul(_gf, s, loga), logti),
                (*_gf).exp[loga.wrapping_add(logti) as usize] as libc::c_uint,
                (*_gf).exp[logti as usize] as libc::c_uint,
                _x,
            );
            i = 0 as libc::c_int;
            while i < nroots {
                *_x.offset(i as isize) = ((*_gf).exp[(255 as libc::c_int
                    - (*_gf).log[*_x.offset(i as isize) as usize] as libc::c_int)
                    as usize] as libc::c_uint
                    ^ s) as libc::c_uchar;
                i += 1
            }
        } else {
            /*s must be a root~\cite{LW72}, and is in fact a double-root~\cite{CCO69}.
            Thus we're left with only a quadratic to solve.
            @ARTICLE{CCO69,
              author="Robert T. Chien and B. D. Cunningham and I. B. Oldham",
              title="Hybrid Methods for Finding Roots of a Polynomial---With
               Applications to {BCH} Decoding",
              journal="{IEEE} Transactions on Information Theory",
              volume=15,
              number=2,
              pages="329--335",
              month=Mar,
              year=1969
            }*/
            nroots = rs_quadratic_solve(_gf, _a, _b ^ r, _x);
            /* s may be a triple root if s=_b/_a, but not quadruple, since _a!=0. */
            if nroots != 2 as libc::c_int
                || *_x.offset(0 as libc::c_int as isize) as libc::c_uint != s
                    && *_x.offset(1 as libc::c_int as isize) as libc::c_uint != s
            {
                let fresh2 = nroots;
                nroots = nroots + 1;
                *_x.offset(fresh2 as isize) = s as libc::c_uchar
            }
        }
        return nroots;
    }
    /* If there are no odd powers, it's really just a quadratic in disguise. */
    if _c == 0 {
        return rs_quadratic_solve(_gf, rs_gsqrt(_gf, _b), rs_gsqrt(_gf, _d), _x);
    }
    /*Factor into (x**2 + r*x + s)*(x**2 + r*x + t) by solving for r, which can
    be shown to satisfy r**3 + _b*r + _c == 0.*/
    nroots = rs_cubic_solve(_gf, 0 as libc::c_int as libc::c_uint, _b, _c, _x);
    if nroots < 1 as libc::c_int {
        /*The Reed-Solomon code is only valid if we can find 4 distinct roots in
         GF(2**8).
        If the cubic does not factor into 3 (possibly duplicate) roots, then we
         know that the quartic must have a non-trivial irreducible factor.*/
        return 0 as libc::c_int;
    }
    r = *_x.offset(0 as libc::c_int as isize) as libc::c_uint;
    /* Now solve for s and t. */
    b = rs_gdiv(_gf, _c, r);
    nroots = rs_quadratic_solve(_gf, b, _d, _x);
    if nroots < 2 as libc::c_int {
        return 0 as libc::c_int;
    }
    s = *_x.offset(0 as libc::c_int as isize) as libc::c_uint;
    t = *_x.offset(1 as libc::c_int as isize) as libc::c_uint;
    /*_c=r*(s^t) was non-zero, so s and t must be distinct.
    But if z is a root of z**2 ^ r*z ^ s, then so is (z^r), and s = z*(z^r).
    Hence if z is also a root of z**2 ^ r*z ^ t, then t = s, a contradiction.
    Thus all four roots are distinct, if they exist.*/
    nroots = rs_quadratic_solve(_gf, r, s, _x);
    return nroots + rs_quadratic_solve(_gf, r, t, _x.offset(nroots as isize));
}
/* Polynomial arithmetic with coefficients in GF(2**8). */
unsafe extern fn rs_poly_zero(mut _p: *mut libc::c_uchar, mut _dp1: libc::c_int) {
    memset(
        _p as *mut libc::c_void,
        0 as libc::c_int,
        (_dp1 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
    );
}
unsafe extern fn rs_poly_copy(
    mut _p: *mut libc::c_uchar,
    mut _q: *const libc::c_uchar,
    mut _dp1: libc::c_int,
) {
    memcpy(
        _p as *mut libc::c_void,
        _q as *const libc::c_void,
        (_dp1 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
    );
}
/*Multiply the polynomial by the free variable, x (shift the coefficients).
The number of coefficients, _dp1, must be non-zero.*/
unsafe extern fn rs_poly_mul_x(
    mut _p: *mut libc::c_uchar,
    mut _q: *const libc::c_uchar,
    mut _dp1: libc::c_int,
) {
    memmove(
        _p.offset(1 as libc::c_int as isize) as *mut libc::c_void,
        _q as *const libc::c_void,
        ((_dp1 - 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
    );
    *_p.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
}
/*Divide the polynomial by the free variable, x (shift the coefficients).
The number of coefficients, _dp1, must be non-zero.*/
unsafe extern fn rs_poly_div_x(
    mut _p: *mut libc::c_uchar,
    mut _q: *const libc::c_uchar,
    mut _dp1: libc::c_int,
) {
    memmove(
        _p as *mut libc::c_void,
        _q.offset(1 as libc::c_int as isize) as *const libc::c_void,
        ((_dp1 - 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
    );
    *_p.offset((_dp1 - 1 as libc::c_int) as isize) = 0 as libc::c_int as libc::c_uchar;
}
/*Compute the first (d+1) coefficients of the product of a degree e and a
degree f polynomial.*/
unsafe extern fn rs_poly_mult(
    mut _gf: *const rs_gf256,
    mut _p: *mut libc::c_uchar,
    mut _dp1: libc::c_int,
    mut _q: *const libc::c_uchar,
    mut _ep1: libc::c_int,
    mut _r: *const libc::c_uchar,
    mut _fp1: libc::c_int,
) {
    let mut m: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    rs_poly_zero(_p, _dp1);
    m = if _ep1 < _dp1 {
        _ep1
    } else {
        _dp1
    };
    i = 0 as libc::c_int;
    while i < m {
        if *_q.offset(i as isize) as libc::c_int != 0 as libc::c_int {
            let mut logqi: libc::c_uint = 0;
            let mut n: libc::c_int = 0;
            let mut j: libc::c_int = 0;
            n = if _dp1 - i < _fp1 {
                (_dp1) - i
            } else {
                _fp1
            };
            logqi = (*_gf).log[*_q.offset(i as isize) as usize] as libc::c_uint;
            j = 0 as libc::c_int;
            while j < n {
                let ref mut fresh3 = *_p.offset((i + j) as isize);
                *fresh3 = (*fresh3 as libc::c_uint
                    ^ rs_hgmul(_gf, *_r.offset(j as isize) as libc::c_uint, logqi))
                    as libc::c_uchar;
                j += 1
            }
        }
        i += 1
    }
}
/* Decoding. */
/* Computes the syndrome of a codeword. */
unsafe extern fn rs_calc_syndrome(
    mut _gf: *const rs_gf256,
    mut _m0: libc::c_int,
    mut _s: *mut libc::c_uchar,
    mut _npar: libc::c_int,
    mut _data: *const libc::c_uchar,
    mut _ndata: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < _npar {
        let mut alphaj: libc::c_uint = 0;
        let mut sj: libc::c_uint = 0;
        sj = 0 as libc::c_int as libc::c_uint;
        alphaj = (*_gf).log[(*_gf).exp[(j + _m0) as usize] as usize] as libc::c_uint;
        i = 0 as libc::c_int;
        while i < _ndata {
            sj = *_data.offset(i as isize) as libc::c_uint ^ rs_hgmul(_gf, sj, alphaj);
            i += 1
        }
        *_s.offset(j as isize) = sj as libc::c_uchar;
        j += 1
    }
}
/*Berlekamp-Peterson and Berlekamp-Massey Algorithms for error-location,
 modified to handle known erasures, from \cite{CC81}, p. 205.
This finds the coefficients of the error locator polynomial.
The roots are then found by looking for the values of alpha**n where
 evaluating the polynomial yields zero.
Error correction is done using the error-evaluator equation on p. 207.
@BOOK{CC81,
  author="George C. Clark, Jr and J. Bibb Cain",
  title="Error-Correction Coding for Digitial Communications",
  series="Applications of Communications Theory",
  publisher="Springer",
  address="New York, NY",
  month=Jun,
  year=1981
}*/
/*Initialize lambda to the product of (1-x*alpha**e[i]) for erasure locations
 e[i].
Note that the user passes in array indices counting from the beginning of the
 data, while our polynomial indexes starting from the end, so
 e[i]=(_ndata-1)-_erasures[i].*/
unsafe extern fn rs_init_lambda(
    mut _gf: *const rs_gf256,
    mut _lambda: *mut libc::c_uchar,
    mut _npar: libc::c_int,
    mut _erasures: *const libc::c_uchar,
    mut _nerasures: libc::c_int,
    mut _ndata: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    rs_poly_zero(
        _lambda,
        (if _npar < 4 as libc::c_int {
            4 as libc::c_int
        } else {
            _npar
        }) + 1 as libc::c_int,
    );
    *_lambda.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_uchar;
    i = 0 as libc::c_int;
    while i < _nerasures {
        j = i + 1 as libc::c_int;
        while j > 0 as libc::c_int {
            let ref mut fresh4 = *_lambda.offset(j as isize);
            *fresh4 = (*fresh4 as libc::c_uint
                ^ rs_hgmul(
                    _gf,
                    *_lambda.offset((j - 1 as libc::c_int) as isize) as libc::c_uint,
                    (_ndata - 1 as libc::c_int - *_erasures.offset(i as isize) as libc::c_int)
                        as libc::c_uint,
                )) as libc::c_uchar;
            j -= 1
        }
        i += 1
    }
}
/*From \cite{CC81}, p. 216.
Returns the number of errors detected (degree of _lambda).*/
unsafe extern fn rs_modified_berlekamp_massey(
    mut _gf: *const rs_gf256,
    mut _lambda: *mut libc::c_uchar,
    mut _s: *const libc::c_uchar,
    mut _omega: *mut libc::c_uchar,
    mut _npar: libc::c_int,
    mut _erasures: *const libc::c_uchar,
    mut _nerasures: libc::c_int,
    mut _ndata: libc::c_int,
) -> libc::c_int {
    let mut tt: [libc::c_uchar; 256] = [0; 256];
    let mut n: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    /*Initialize _lambda, the error locator-polynomial, with the location of
    known erasures.*/
    rs_init_lambda(_gf, _lambda, _npar, _erasures, _nerasures, _ndata);
    rs_poly_copy(tt.as_mut_ptr(), _lambda, _npar + 1 as libc::c_int);
    l = _nerasures;
    k = 0 as libc::c_int;
    n = _nerasures + 1 as libc::c_int;
    while n <= _npar {
        let mut d: libc::c_uint = 0;
        rs_poly_mul_x(tt.as_mut_ptr(), tt.as_mut_ptr(), n - k + 1 as libc::c_int);
        d = 0 as libc::c_int as libc::c_uint;
        i = 0 as libc::c_int;
        while i <= l {
            d ^= rs_gmul(
                _gf,
                *_lambda.offset(i as isize) as libc::c_uint,
                *_s.offset((n - 1 as libc::c_int - i) as isize) as libc::c_uint,
            );
            i += 1
        }
        if d != 0 as libc::c_int as libc::c_uint {
            let mut logd: libc::c_uint = 0;
            logd = (*_gf).log[d as usize] as libc::c_uint;
            if l < n - k {
                let mut t: libc::c_int = 0;
                i = 0 as libc::c_int;
                while i <= n - k {
                    let mut tti: libc::c_uint = 0;
                    tti = tt[i as usize] as libc::c_uint;
                    tt[i as usize] = rs_hgmul(
                        _gf,
                        *_lambda.offset(i as isize) as libc::c_uint,
                        (255 as libc::c_int as libc::c_uint).wrapping_sub(logd),
                    ) as libc::c_uchar;
                    *_lambda.offset(i as isize) = (*_lambda.offset(i as isize) as libc::c_uint
                        ^ rs_hgmul(_gf, tti, logd))
                        as libc::c_uchar;
                    i += 1
                }
                t = n - k;
                k = n - l;
                l = t
            } else {
                i = 0 as libc::c_int;
                while i <= l {
                    *_lambda.offset(i as isize) = (*_lambda.offset(i as isize) as libc::c_uint
                        ^ rs_hgmul(_gf, tt[i as usize] as libc::c_uint, logd))
                        as libc::c_uchar;
                    i += 1
                }
            }
        }
        n += 1
    }
    rs_poly_mult(_gf, _omega, _npar, _lambda, l + 1 as libc::c_int, _s, _npar);
    return l;
}
/*Finds all the roots of an error-locator polynomial _lambda by evaluating it
 at successive values of alpha, and returns the positions of the associated
 errors in _epos.
Returns the number of valid roots identified.*/
unsafe extern fn rs_find_roots(
    mut _gf: *const rs_gf256,
    mut _epos: *mut libc::c_uchar,
    mut _lambda: *const libc::c_uchar,
    mut _nerrors: libc::c_int,
    mut _ndata: libc::c_int,
) -> libc::c_int {
    let mut alpha: libc::c_uint = 0;
    let mut nroots: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    nroots = 0 as libc::c_int;
    if _nerrors <= 4 as libc::c_int {
        /*Explicit solutions for higher degrees are possible.
        Chien uses large lookup tables to solve quintics, and Truong et al. give
         special algorithms for degree up through 11, though they use exhaustive
         search (with reduced complexity) for some portions.
        Quartics are good enough for reading CDs, and represent a reasonable code
         complexity trade-off without requiring any extra tables.
        Note that _lambda[0] is always 1.*/
        _nerrors = rs_quartic_solve(
            _gf,
            *_lambda.offset(1 as libc::c_int as isize) as libc::c_uint,
            *_lambda.offset(2 as libc::c_int as isize) as libc::c_uint,
            *_lambda.offset(3 as libc::c_int as isize) as libc::c_uint,
            *_lambda.offset(4 as libc::c_int as isize) as libc::c_uint,
            _epos,
        );
        i = 0 as libc::c_int;
        while i < _nerrors {
            if *_epos.offset(i as isize) != 0 {
                alpha = (*_gf).log[*_epos.offset(i as isize) as usize] as libc::c_uint;
                if (alpha as libc::c_int) < _ndata {
                    let fresh5 = nroots;
                    nroots = nroots + 1;
                    *_epos.offset(fresh5 as isize) = alpha as libc::c_uchar
                }
            }
            i += 1
        }
        return nroots;
    } else {
        alpha = 0 as libc::c_int as libc::c_uint;
        while (alpha as libc::c_int) < _ndata {
            let mut alphai: libc::c_uint = 0;
            let mut sum: libc::c_uint = 0;
            sum = 0 as libc::c_int as libc::c_uint;
            alphai = 0 as libc::c_int as libc::c_uint;
            i = 0 as libc::c_int;
            while i <= _nerrors {
                sum ^=
                    rs_hgmul(_gf, *_lambda.offset((_nerrors - i) as isize) as libc::c_uint, alphai);
                alphai = (*_gf).log[(*_gf).exp[alphai.wrapping_add(alpha) as usize] as usize]
                    as libc::c_uint;
                i += 1
            }
            if sum == 0 {
                let fresh6 = nroots;
                nroots = nroots + 1;
                *_epos.offset(fresh6 as isize) = alpha as libc::c_uchar
            }
            alpha = alpha.wrapping_add(1)
        }
    }
    return nroots;
}
/*Corrects a codeword with _ndata<256 bytes, of which the last _npar are parity
 bytes.
Known locations of errors can be passed in the _erasures array.
Twice as many (up to _npar) errors with a known location can be corrected
 compared to errors with an unknown location.
Returns the number of errors corrected if successful, or a negative number if
 the message could not be corrected because too many errors were detected.*/
/*Corrects a codeword with _ndata<256 bytes, of which the last _npar are parity
 bytes.
Known locations of errors can be passed in the _erasures array.
Twice as many (up to _npar) errors with a known location can be corrected
 compared to errors with an unknown location.
Returns the number of errors corrected if successful, or a negative number if
 the message could not be corrected because too many errors were detected.*/
#[no_mangle]
pub unsafe extern fn rs_correct(
    mut _gf: *const rs_gf256,
    mut _m0: libc::c_int,
    mut _data: *mut libc::c_uchar,
    mut _ndata: libc::c_int,
    mut _npar: libc::c_int,
    mut _erasures: *const libc::c_uchar,
    mut _nerasures: libc::c_int,
) -> libc::c_int {
    /*lambda must have storage for at least five entries to avoid special cases
    in the low-degree polynomial solver.*/
    let mut lambda: [libc::c_uchar; 256] = [0; 256];
    let mut omega: [libc::c_uchar; 256] = [0; 256];
    let mut epos: [libc::c_uchar; 256] = [0; 256];
    let mut s: [libc::c_uchar; 256] = [0; 256];
    let mut i: libc::c_int = 0;
    /* If we already have too many erasures, we can't possibly succeed. */
    if _nerasures > _npar {
        return -(1 as libc::c_int);
    }
    /* Compute the syndrome values. */
    rs_calc_syndrome(_gf, _m0, s.as_mut_ptr(), _npar, _data, _ndata);
    /* Check for a non-zero value. */
    i = 0 as libc::c_int;
    while i < _npar {
        if s[i as usize] != 0 {
            let mut nerrors: libc::c_int = 0;
            let mut j: libc::c_int = 0;
            /* Construct the error locator polynomial. */
            nerrors = rs_modified_berlekamp_massey(
                _gf,
                lambda.as_mut_ptr(),
                s.as_mut_ptr(),
                omega.as_mut_ptr(),
                _npar,
                _erasures,
                _nerasures,
                _ndata,
            );
            /*If we can't locate any errors, we can't force the syndrome values to
             zero, and must have a decoding error.
            Conversely, if we have too many errors, there's no reason to even attempt
             the root search.*/
            if nerrors <= 0 as libc::c_int
                || nerrors - _nerasures > _npar - _nerasures >> 1 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
            /*Compute the locations of the errors.
            If they are not all distinct, or some of them were outside the valid
             range for our block size, we have a decoding error.*/
            if rs_find_roots(_gf, epos.as_mut_ptr(), lambda.as_mut_ptr(), nerrors, _ndata) < nerrors
            {
                return -(1 as libc::c_int);
            }
            /* Now compute the error magnitudes. */
            i = 0 as libc::c_int;
            while i < nerrors {
                let mut a: libc::c_uint = 0;
                let mut b: libc::c_uint = 0;
                let mut alpha: libc::c_uint = 0;
                let mut alphan1: libc::c_uint = 0;
                let mut alphan2: libc::c_uint = 0;
                let mut alphanj: libc::c_uint = 0;
                alpha = epos[i as usize] as libc::c_uint;
                /* Evaluate omega at alpha**-1. */
                a = 0 as libc::c_int as libc::c_uint;
                alphan1 = (255 as libc::c_int as libc::c_uint).wrapping_sub(alpha);
                alphanj = 0 as libc::c_int as libc::c_uint;
                j = 0 as libc::c_int;
                while j < _npar {
                    a ^= rs_hgmul(_gf, omega[j as usize] as libc::c_uint, alphanj);
                    alphanj = (*_gf).log
                        [(*_gf).exp[alphanj.wrapping_add(alphan1) as usize] as usize]
                        as libc::c_uint;
                    j += 1
                }
                /*Evaluate the derivative of lambda at alpha**-1
                All the odd powers vanish.*/
                b = 0 as libc::c_int as libc::c_uint;
                alphan2 = (*_gf).log[(*_gf).exp[(alphan1 << 1 as libc::c_int) as usize] as usize]
                    as libc::c_uint;
                alphanj = alphan1.wrapping_add(
                    (_m0 as libc::c_uint)
                        .wrapping_mul(alpha)
                        .wrapping_rem(255 as libc::c_int as libc::c_uint),
                );
                j = 1 as libc::c_int;
                while j <= _npar {
                    b ^= rs_hgmul(_gf, lambda[j as usize] as libc::c_uint, alphanj);
                    alphanj = (*_gf).log
                        [(*_gf).exp[alphanj.wrapping_add(alphan2) as usize] as usize]
                        as libc::c_uint;
                    j += 2 as libc::c_int
                }
                /* Apply the correction. */
                let ref mut fresh7 =
                    *_data
                        .offset(((_ndata - 1 as libc::c_int) as libc::c_uint).wrapping_sub(alpha)
                            as isize);
                *fresh7 = (*fresh7 as libc::c_uint ^ rs_gdiv(_gf, a, b)) as libc::c_uchar;
                i += 1
            }
            return nerrors;
        }
        i += 1
    }
    return 0 as libc::c_int;
}
/*Create an _npar-coefficient generator polynomial for a Reed-Solomon code with
_npar<256 parity bytes.*/
/* Encoding. */
/*Create an _npar-coefficient generator polynomial for a Reed-Solomon code
with _npar<256 parity bytes.*/
#[no_mangle]
pub unsafe extern fn rs_compute_genpoly(
    mut _gf: *const rs_gf256,
    mut _m0: libc::c_int,
    mut _genpoly: *mut libc::c_uchar,
    mut _npar: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    if _npar <= 0 as libc::c_int {
        return;
    }
    rs_poly_zero(_genpoly, _npar);
    *_genpoly.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_uchar;
    /* Multiply by (x+alpha^i) for i = 1 ... _ndata. */
    i = 0 as libc::c_int;
    while i < _npar {
        let mut alphai: libc::c_uint = 0;
        let mut n: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        n = if (i + 1 as libc::c_int) < _npar - 1 as libc::c_int {
            (i) + 1 as libc::c_int
        } else {
            (_npar) - 1 as libc::c_int
        };
        alphai = (*_gf).log[(*_gf).exp[(_m0 + i) as usize] as usize] as libc::c_uint;
        j = n;
        while j > 0 as libc::c_int {
            *_genpoly.offset(j as isize) = (*_genpoly.offset((j - 1 as libc::c_int) as isize)
                as libc::c_uint
                ^ rs_hgmul(_gf, *_genpoly.offset(j as isize) as libc::c_uint, alphai))
                as libc::c_uchar;
            j -= 1
        }
        *_genpoly.offset(0 as libc::c_int as isize) =
            rs_hgmul(_gf, *_genpoly.offset(0 as libc::c_int as isize) as libc::c_uint, alphai)
                as libc::c_uchar;
        i += 1
    }
}
/*Adds _npar<=_ndata parity bytes to an _ndata-_npar byte message.
_data must contain room for _ndata<256 bytes.*/
/*Adds _npar<=_ndata parity bytes to an _ndata-_npar byte message.
_data must contain room for _ndata<256 bytes.*/
#[no_mangle]
pub unsafe extern fn rs_encode(
    mut _gf: *const rs_gf256,
    mut _data: *mut libc::c_uchar,
    mut _ndata: libc::c_int,
    mut _genpoly: *const libc::c_uchar,
    mut _npar: libc::c_int,
) {
    let mut lfsr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut d: libc::c_uint = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if _npar <= 0 as libc::c_int {
        return;
    }
    lfsr = _data.offset(_ndata as isize).offset(-(_npar as isize));
    rs_poly_zero(lfsr, _npar);
    i = 0 as libc::c_int;
    while i < _ndata - _npar {
        d = (*_data.offset(i as isize) as libc::c_int
            ^ *lfsr.offset(0 as libc::c_int as isize) as libc::c_int) as libc::c_uint;
        if d != 0 {
            let mut logd: libc::c_uint = 0;
            logd = (*_gf).log[d as usize] as libc::c_uint;
            j = 0 as libc::c_int;
            while j < _npar - 1 as libc::c_int {
                *lfsr.offset(j as isize) = (*lfsr.offset((j + 1 as libc::c_int) as isize)
                    as libc::c_uint
                    ^ rs_hgmul(
                        _gf,
                        *_genpoly.offset((_npar - 1 as libc::c_int - j) as isize) as libc::c_uint,
                        logd,
                    )) as libc::c_uchar;
                j += 1
            }
            *lfsr.offset((_npar - 1 as libc::c_int) as isize) =
                rs_hgmul(_gf, *_genpoly.offset(0 as libc::c_int as isize) as libc::c_uint, logd)
                    as libc::c_uchar
        } else {
            rs_poly_div_x(lfsr, lfsr, _npar);
        }
        i += 1
    }
}
