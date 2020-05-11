use ::libc;
extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
}
/*ISAAC is the most advanced of a series of Pseudo-Random Number Generators
   designed by Robert J. Jenkins Jr. in 1996.
  http://www.burtleburtle.net/bob/rand/isaac.html
  To quote:
    No efficient method is known for deducing their internal states.
    ISAAC requires an amortized 18.75 instructions to produce a 32-bit value.
    There are no cycles in ISAAC shorter than 2**40 values.
    The expected cycle length is 2**8295 values.*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct isaac_ctx {
    pub n: libc::c_uint,
    pub r: [libc::c_uint; 256],
    pub m: [libc::c_uint; 256],
    pub a: libc::c_uint,
    pub b: libc::c_uint,
    pub c: libc::c_uint,
}
unsafe extern "C" fn isaac_update(mut _ctx: *mut isaac_ctx) {
    let mut m: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut r: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut a: libc::c_uint = 0;
    let mut b: libc::c_uint = 0;
    let mut x: libc::c_uint = 0;
    let mut y: libc::c_uint = 0;
    let mut i: libc::c_int = 0;
    m = (*_ctx).m.as_mut_ptr();
    r = (*_ctx).r.as_mut_ptr();
    a = (*_ctx).a;
    (*_ctx).c = (*_ctx).c.wrapping_add(1);
    b = (*_ctx).b.wrapping_add((*_ctx).c) & 0xffffffff as libc::c_uint;
    i = 0 as libc::c_int;
    while i < ((1 as libc::c_int) << 8 as libc::c_int) / 2 as libc::c_int {
        x = *m.offset(i as isize);
        a =
            (a ^
                 a <<
                     13 as
                         libc::c_int).wrapping_add(*m.offset((i +
                                                                  ((1 as
                                                                        libc::c_int)
                                                                       <<
                                                                       8 as
                                                                           libc::c_int)
                                                                      /
                                                                      2 as
                                                                          libc::c_int)
                                                                 as isize)) &
                0xffffffff as libc::c_uint;
        y =
            (*m.offset(((x &
                             ((((1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) << 2 as libc::c_int) as
                                 libc::c_uint) >> 2 as libc::c_int) as
                           isize)).wrapping_add(a).wrapping_add(b) &
                0xffffffff as libc::c_uint;
        *m.offset(i as isize) = y;
        b =
            (*m.offset((y >> 8 as libc::c_int + 2 as libc::c_int &
                            (((1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) as
                           isize)).wrapping_add(x) &
                0xffffffff as libc::c_uint;
        *r.offset(i as isize) = b;
        i += 1;
        x = *m.offset(i as isize);
        a =
            (a ^
                 a >>
                     6 as
                         libc::c_int).wrapping_add(*m.offset((i +
                                                                  ((1 as
                                                                        libc::c_int)
                                                                       <<
                                                                       8 as
                                                                           libc::c_int)
                                                                      /
                                                                      2 as
                                                                          libc::c_int)
                                                                 as isize)) &
                0xffffffff as libc::c_uint;
        y =
            (*m.offset(((x &
                             ((((1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) << 2 as libc::c_int) as
                                 libc::c_uint) >> 2 as libc::c_int) as
                           isize)).wrapping_add(a).wrapping_add(b) &
                0xffffffff as libc::c_uint;
        *m.offset(i as isize) = y;
        b =
            (*m.offset((y >> 8 as libc::c_int + 2 as libc::c_int &
                            (((1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) as
                           isize)).wrapping_add(x) &
                0xffffffff as libc::c_uint;
        *r.offset(i as isize) = b;
        i += 1;
        x = *m.offset(i as isize);
        a =
            (a ^
                 a <<
                     2 as
                         libc::c_int).wrapping_add(*m.offset((i +
                                                                  ((1 as
                                                                        libc::c_int)
                                                                       <<
                                                                       8 as
                                                                           libc::c_int)
                                                                      /
                                                                      2 as
                                                                          libc::c_int)
                                                                 as isize)) &
                0xffffffff as libc::c_uint;
        y =
            (*m.offset(((x &
                             ((((1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) << 2 as libc::c_int) as
                                 libc::c_uint) >> 2 as libc::c_int) as
                           isize)).wrapping_add(a).wrapping_add(b) &
                0xffffffff as libc::c_uint;
        *m.offset(i as isize) = y;
        b =
            (*m.offset((y >> 8 as libc::c_int + 2 as libc::c_int &
                            (((1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) as
                           isize)).wrapping_add(x) &
                0xffffffff as libc::c_uint;
        *r.offset(i as isize) = b;
        i += 1;
        x = *m.offset(i as isize);
        a =
            (a ^
                 a >>
                     16 as
                         libc::c_int).wrapping_add(*m.offset((i +
                                                                  ((1 as
                                                                        libc::c_int)
                                                                       <<
                                                                       8 as
                                                                           libc::c_int)
                                                                      /
                                                                      2 as
                                                                          libc::c_int)
                                                                 as isize)) &
                0xffffffff as libc::c_uint;
        y =
            (*m.offset(((x &
                             ((((1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) << 2 as libc::c_int) as
                                 libc::c_uint) >> 2 as libc::c_int) as
                           isize)).wrapping_add(a).wrapping_add(b) &
                0xffffffff as libc::c_uint;
        *m.offset(i as isize) = y;
        b =
            (*m.offset((y >> 8 as libc::c_int + 2 as libc::c_int &
                            (((1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) as
                           isize)).wrapping_add(x) &
                0xffffffff as libc::c_uint;
        *r.offset(i as isize) = b;
        i += 1
    }
    i = ((1 as libc::c_int) << 8 as libc::c_int) / 2 as libc::c_int;
    while i < (1 as libc::c_int) << 8 as libc::c_int {
        x = *m.offset(i as isize);
        a =
            (a ^
                 a <<
                     13 as
                         libc::c_int).wrapping_add(*m.offset((i -
                                                                  ((1 as
                                                                        libc::c_int)
                                                                       <<
                                                                       8 as
                                                                           libc::c_int)
                                                                      /
                                                                      2 as
                                                                          libc::c_int)
                                                                 as isize)) &
                0xffffffff as libc::c_uint;
        y =
            (*m.offset(((x &
                             ((((1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) << 2 as libc::c_int) as
                                 libc::c_uint) >> 2 as libc::c_int) as
                           isize)).wrapping_add(a).wrapping_add(b) &
                0xffffffff as libc::c_uint;
        *m.offset(i as isize) = y;
        b =
            (*m.offset((y >> 8 as libc::c_int + 2 as libc::c_int &
                            (((1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) as
                           isize)).wrapping_add(x) &
                0xffffffff as libc::c_uint;
        *r.offset(i as isize) = b;
        i += 1;
        x = *m.offset(i as isize);
        a =
            (a ^
                 a >>
                     6 as
                         libc::c_int).wrapping_add(*m.offset((i -
                                                                  ((1 as
                                                                        libc::c_int)
                                                                       <<
                                                                       8 as
                                                                           libc::c_int)
                                                                      /
                                                                      2 as
                                                                          libc::c_int)
                                                                 as isize)) &
                0xffffffff as libc::c_uint;
        y =
            (*m.offset(((x &
                             ((((1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) << 2 as libc::c_int) as
                                 libc::c_uint) >> 2 as libc::c_int) as
                           isize)).wrapping_add(a).wrapping_add(b) &
                0xffffffff as libc::c_uint;
        *m.offset(i as isize) = y;
        b =
            (*m.offset((y >> 8 as libc::c_int + 2 as libc::c_int &
                            (((1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) as
                           isize)).wrapping_add(x) &
                0xffffffff as libc::c_uint;
        *r.offset(i as isize) = b;
        i += 1;
        x = *m.offset(i as isize);
        a =
            (a ^
                 a <<
                     2 as
                         libc::c_int).wrapping_add(*m.offset((i -
                                                                  ((1 as
                                                                        libc::c_int)
                                                                       <<
                                                                       8 as
                                                                           libc::c_int)
                                                                      /
                                                                      2 as
                                                                          libc::c_int)
                                                                 as isize)) &
                0xffffffff as libc::c_uint;
        y =
            (*m.offset(((x &
                             ((((1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) << 2 as libc::c_int) as
                                 libc::c_uint) >> 2 as libc::c_int) as
                           isize)).wrapping_add(a).wrapping_add(b) &
                0xffffffff as libc::c_uint;
        *m.offset(i as isize) = y;
        b =
            (*m.offset((y >> 8 as libc::c_int + 2 as libc::c_int &
                            (((1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) as
                           isize)).wrapping_add(x) &
                0xffffffff as libc::c_uint;
        *r.offset(i as isize) = b;
        i += 1;
        x = *m.offset(i as isize);
        a =
            (a ^
                 a >>
                     16 as
                         libc::c_int).wrapping_add(*m.offset((i -
                                                                  ((1 as
                                                                        libc::c_int)
                                                                       <<
                                                                       8 as
                                                                           libc::c_int)
                                                                      /
                                                                      2 as
                                                                          libc::c_int)
                                                                 as isize)) &
                0xffffffff as libc::c_uint;
        y =
            (*m.offset(((x &
                             ((((1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) << 2 as libc::c_int) as
                                 libc::c_uint) >> 2 as libc::c_int) as
                           isize)).wrapping_add(a).wrapping_add(b) &
                0xffffffff as libc::c_uint;
        *m.offset(i as isize) = y;
        b =
            (*m.offset((y >> 8 as libc::c_int + 2 as libc::c_int &
                            (((1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) as
                           isize)).wrapping_add(x) &
                0xffffffff as libc::c_uint;
        *r.offset(i as isize) = b;
        i += 1
    }
    (*_ctx).b = b;
    (*_ctx).a = a;
    (*_ctx).n = ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint;
}
unsafe extern "C" fn isaac_mix(mut _x: *mut libc::c_uint) {
    static mut SHIFT: [libc::c_uchar; 8] =
        [11 as libc::c_int as libc::c_uchar,
         2 as libc::c_int as libc::c_uchar, 8 as libc::c_int as libc::c_uchar,
         16 as libc::c_int as libc::c_uchar,
         10 as libc::c_int as libc::c_uchar,
         4 as libc::c_int as libc::c_uchar, 8 as libc::c_int as libc::c_uchar,
         9 as libc::c_int as libc::c_uchar];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        *_x.offset(i as isize) ^=
            *_x.offset((i + 1 as libc::c_int & 7 as libc::c_int) as isize) <<
                SHIFT[i as usize] as libc::c_int;
        let ref mut fresh0 =
            *_x.offset((i + 3 as libc::c_int & 7 as libc::c_int) as isize);
        *fresh0 = (*fresh0).wrapping_add(*_x.offset(i as isize));
        let ref mut fresh1 =
            *_x.offset((i + 1 as libc::c_int & 7 as libc::c_int) as isize);
        *fresh1 =
            (*fresh1).wrapping_add(*_x.offset((i + 2 as libc::c_int &
                                                   7 as libc::c_int) as
                                                  isize));
        i += 1;
        *_x.offset(i as isize) ^=
            *_x.offset((i + 1 as libc::c_int & 7 as libc::c_int) as isize) >>
                SHIFT[i as usize] as libc::c_int;
        let ref mut fresh2 =
            *_x.offset((i + 3 as libc::c_int & 7 as libc::c_int) as isize);
        *fresh2 = (*fresh2).wrapping_add(*_x.offset(i as isize));
        let ref mut fresh3 =
            *_x.offset((i + 1 as libc::c_int & 7 as libc::c_int) as isize);
        *fresh3 =
            (*fresh3).wrapping_add(*_x.offset((i + 2 as libc::c_int &
                                                   7 as libc::c_int) as
                                                  isize));
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn isaac_init(mut _ctx: *mut isaac_ctx,
                                    mut _seed: *const libc::c_void,
                                    mut _nseed: libc::c_int) {
    let mut seed: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut m: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut r: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut x: [libc::c_uint; 8] = [0; 8];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    (*_ctx).c = 0 as libc::c_int as libc::c_uint;
    (*_ctx).b = (*_ctx).c;
    (*_ctx).a = (*_ctx).b;
    m = (*_ctx).m.as_mut_ptr();
    r = (*_ctx).r.as_mut_ptr();
    x[7 as libc::c_int as usize] = 0x9e3779b9 as libc::c_uint;
    x[6 as libc::c_int as usize] = x[7 as libc::c_int as usize];
    x[5 as libc::c_int as usize] = x[6 as libc::c_int as usize];
    x[4 as libc::c_int as usize] = x[5 as libc::c_int as usize];
    x[3 as libc::c_int as usize] = x[4 as libc::c_int as usize];
    x[2 as libc::c_int as usize] = x[3 as libc::c_int as usize];
    x[1 as libc::c_int as usize] = x[2 as libc::c_int as usize];
    x[0 as libc::c_int as usize] = x[1 as libc::c_int as usize];
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int { isaac_mix(x.as_mut_ptr()); i += 1 }
    if _nseed > ((1 as libc::c_int) << 8 as libc::c_int) << 2 as libc::c_int {
        _nseed = ((1 as libc::c_int) << 8 as libc::c_int) << 2 as libc::c_int
    }
    seed = _seed as *const libc::c_uchar;
    i = 0 as libc::c_int;
    while i < _nseed >> 2 as libc::c_int {
        *r.offset(i as isize) =
            ((*seed.offset((i << 2 as libc::c_int | 3 as libc::c_int) as
                               isize) as libc::c_int) << 24 as libc::c_int |
                 (*seed.offset((i << 2 as libc::c_int | 2 as libc::c_int) as
                                   isize) as libc::c_int) << 16 as libc::c_int
                 |
                 (*seed.offset((i << 2 as libc::c_int | 1 as libc::c_int) as
                                   isize) as libc::c_int) << 8 as libc::c_int
                 |
                 *seed.offset((i << 2 as libc::c_int) as isize) as
                     libc::c_int) as libc::c_uint;
        i += 1
    }
    if _nseed & 3 as libc::c_int != 0 {
        *r.offset(i as isize) =
            *seed.offset((i << 2 as libc::c_int) as isize) as libc::c_uint;
        j = 1 as libc::c_int;
        while j < _nseed & 3 as libc::c_int {
            let ref mut fresh4 = *r.offset(i as isize);
            *fresh4 =
                (*fresh4).wrapping_add(((*seed.offset((i << 2 as libc::c_int |
                                                           j) as isize) as
                                             libc::c_int) <<
                                            (j << 3 as libc::c_int)) as
                                           libc::c_uint);
            j += 1
        }
        i += 1
    }
    memset(r.offset(i as isize) as *mut libc::c_void, 0 as libc::c_int,
           ((((1 as libc::c_int) << 8 as libc::c_int) - i) as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_uint>()
                                                as libc::c_ulong));
    i = 0 as libc::c_int;
    while i < (1 as libc::c_int) << 8 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 8 as libc::c_int {
            x[j as usize] =
                x[j as usize].wrapping_add(*r.offset((i + j) as isize));
            j += 1
        }
        isaac_mix(x.as_mut_ptr());
        memcpy(m.offset(i as isize) as *mut libc::c_void,
               x.as_mut_ptr() as *const libc::c_void,
               ::std::mem::size_of::<[libc::c_uint; 8]>() as libc::c_ulong);
        i += 8 as libc::c_int
    }
    i = 0 as libc::c_int;
    while i < (1 as libc::c_int) << 8 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 8 as libc::c_int {
            x[j as usize] =
                x[j as usize].wrapping_add(*m.offset((i + j) as isize));
            j += 1
        }
        isaac_mix(x.as_mut_ptr());
        memcpy(m.offset(i as isize) as *mut libc::c_void,
               x.as_mut_ptr() as *const libc::c_void,
               ::std::mem::size_of::<[libc::c_uint; 8]>() as libc::c_ulong);
        i += 8 as libc::c_int
    }
    isaac_update(_ctx);
}
#[no_mangle]
pub unsafe extern "C" fn isaac_next_uint32(mut _ctx: *mut isaac_ctx)
 -> libc::c_uint {
    if (*_ctx).n == 0 { isaac_update(_ctx); }
    (*_ctx).n = (*_ctx).n.wrapping_sub(1);
    return (*_ctx).r[(*_ctx).n as usize];
}
/*Returns a uniform random integer less than the given maximum value.
  _n: The upper bound on the range of numbers returned (not inclusive).
      This must be strictly less than 2**32.
  Return: An integer uniformly distributed between 0 (inclusive) and _n
           (exclusive).*/
#[no_mangle]
pub unsafe extern "C" fn isaac_next_uint(mut _ctx: *mut isaac_ctx,
                                         mut _n: libc::c_uint)
 -> libc::c_uint {
    let mut r: libc::c_uint = 0;
    let mut v: libc::c_uint = 0;
    let mut d: libc::c_uint = 0;
    loop  {
        r = isaac_next_uint32(_ctx);
        v = r.wrapping_rem(_n);
        d = r.wrapping_sub(v);
        if !((d.wrapping_add(_n).wrapping_sub(1 as libc::c_int as
                                                  libc::c_uint) &
                  0xffffffff as libc::c_uint) < d) {
            break ;
        }
    }
    return v;
}
