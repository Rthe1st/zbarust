use ::libc;
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
}
/*Binarizes a grayscale image.*/
/*Copyright (C) 2008-2009  Timothy B. Terriberry (tterribe@xiph.org)
  You can redistribute this library and/or modify it under the terms of the
   GNU Lesser General Public License as published by the Free Software
   Foundation; either version 2.1 of the License, or (at your option) any later
   version.*/
/*The above algorithms are computationally expensive, and do not work as well
   as the simple algorithm below.
  Sauvola by itself does an excellent job of classifying regions outside the
   QR code as background, which greatly reduces the chance of false alarms.
  However, it also tends to over-shrink isolated black dots inside the code,
   making them easy to miss with even slight mis-alignment.
  Since the Gatos method uses Sauvola as input to its background interpolation
   method, it cannot possibly mark any pixels as foreground which Sauvola
   classified as background, and thus suffers from the same problem.
  The following simple adaptive threshold method does not have this problem,
   though it produces essentially random noise outside the QR code region.
  QR codes are structured well enough that this does not seem to lead to any
   actual false alarms in practice, and it allows many more codes to be
   detected and decoded successfully than the Sauvola or Gatos binarization
   methods.*/
/*A simplified adaptive thresholder.
  This compares the current pixel value to the mean value of a (large) window
   surrounding it.*/
#[no_mangle]
pub unsafe extern "C" fn qr_binarize(mut _img: *const libc::c_uchar,
                                     mut _width: libc::c_int,
                                     mut _height: libc::c_int)
 -> *mut libc::c_uchar {
    let mut mask: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if _width > 0 as libc::c_int && _height > 0 as libc::c_int {
        let mut col_sums: *mut libc::c_uint = 0 as *mut libc::c_uint;
        let mut logwindw: libc::c_int = 0;
        let mut logwindh: libc::c_int = 0;
        let mut windw: libc::c_int = 0;
        let mut windh: libc::c_int = 0;
        let mut y0offs: libc::c_int = 0;
        let mut y1offs: libc::c_int = 0;
        let mut g: libc::c_uint = 0;
        let mut x: libc::c_int = 0;
        let mut y: libc::c_int = 0;
        mask =
            malloc(((_width * _height) as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_uchar>()
                                                        as libc::c_ulong)) as
                *mut libc::c_uchar;
        /*We keep the window size fairly large to ensure it doesn't fit completely
       inside the center of a finder pattern of a version 1 QR code at full
       resolution.*/
        logwindw = 4 as libc::c_int;
        while logwindw < 8 as libc::c_int &&
                  (1 as libc::c_int) << logwindw <
                      _width + 7 as libc::c_int >> 3 as libc::c_int {
            logwindw += 1
        }
        logwindh = 4 as libc::c_int;
        while logwindh < 8 as libc::c_int &&
                  (1 as libc::c_int) << logwindh <
                      _height + 7 as libc::c_int >> 3 as libc::c_int {
            logwindh += 1
        }
        windw = (1 as libc::c_int) << logwindw;
        windh = (1 as libc::c_int) << logwindh;
        col_sums =
            malloc((_width as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_uint>()
                                                        as libc::c_ulong)) as
                *mut libc::c_uint;
        /*Initialize sums down each column.*/
        x = 0 as libc::c_int;
        while x < _width {
            g = *_img.offset(x as isize) as libc::c_uint;
            *col_sums.offset(x as isize) =
                (g << logwindh - 1 as libc::c_int).wrapping_add(g);
            x += 1
        }
        y = 1 as libc::c_int;
        while y < windh >> 1 as libc::c_int {
            y1offs =
                (y +
                     (_height - 1 as libc::c_int - y &
                          -(((_height - 1 as libc::c_int) < y) as
                                libc::c_int))) * _width;
            x = 0 as libc::c_int;
            while x < _width {
                g = *_img.offset((y1offs + x) as isize) as libc::c_uint;
                let ref mut fresh0 = *col_sums.offset(x as isize);
                *fresh0 = (*fresh0).wrapping_add(g);
                x += 1
            }
            y += 1
        }
        y = 0 as libc::c_int;
        while y < _height {
            let mut m: libc::c_uint = 0;
            let mut x0: libc::c_int = 0;
            let mut x1: libc::c_int = 0;
            /*Initialize the sum over the window.*/
            m =
                (*col_sums.offset(0 as libc::c_int as isize) <<
                     logwindw -
                         1 as
                             libc::c_int).wrapping_add(*col_sums.offset(0 as
                                                                            libc::c_int
                                                                            as
                                                                            isize));
            x = 1 as libc::c_int;
            while x < windw >> 1 as libc::c_int {
                x1 =
                    x +
                        (_width - 1 as libc::c_int - x &
                             -(((_width - 1 as libc::c_int) < x) as
                                   libc::c_int));
                m = m.wrapping_add(*col_sums.offset(x1 as isize));
                x += 1
            }
            x = 0 as libc::c_int;
            while x < _width {
                /*Perform the test against the threshold T = (m/n)-D,
           where n=windw*windh and D=3.*/
                g = *_img.offset((y * _width + x) as isize) as libc::c_uint;
                *mask.offset((y * _width + x) as isize) =
                    (-((g.wrapping_add(3 as libc::c_int as libc::c_uint) <<
                            logwindw + logwindh < m) as libc::c_int) &
                         0xff as libc::c_int) as libc::c_uchar;
                /*Update the window sum.*/
                if (x + 1 as libc::c_int) < _width {
                    x0 =
                        0 as libc::c_int -
                            (0 as libc::c_int -
                                 (x - (windw >> 1 as libc::c_int)) &
                                 -((x - (windw >> 1 as libc::c_int) >
                                        0 as libc::c_int) as libc::c_int));
                    x1 =
                        x + (windw >> 1 as libc::c_int) +
                            (_width - 1 as libc::c_int -
                                 (x + (windw >> 1 as libc::c_int)) &
                                 -(((_width - 1 as libc::c_int) <
                                        x + (windw >> 1 as libc::c_int)) as
                                       libc::c_int));
                    m =
                        m.wrapping_add((*col_sums.offset(x1 as
                                                             isize)).wrapping_sub(*col_sums.offset(x0
                                                                                                       as
                                                                                                       isize)))
                }
                x += 1
            }
            /*Update the column sums.*/
            if (y + 1 as libc::c_int) < _height {
                y0offs =
                    (0 as libc::c_int -
                         (0 as libc::c_int - (y - (windh >> 1 as libc::c_int))
                              &
                              -((y - (windh >> 1 as libc::c_int) >
                                     0 as libc::c_int) as libc::c_int))) *
                        _width;
                y1offs =
                    (y + (windh >> 1 as libc::c_int) +
                         (_height - 1 as libc::c_int -
                              (y + (windh >> 1 as libc::c_int)) &
                              -(((_height - 1 as libc::c_int) <
                                     y + (windh >> 1 as libc::c_int)) as
                                    libc::c_int))) * _width;
                x = 0 as libc::c_int;
                while x < _width {
                    let ref mut fresh1 = *col_sums.offset(x as isize);
                    *fresh1 =
                        (*fresh1).wrapping_sub(*_img.offset((y0offs + x) as
                                                                isize) as
                                                   libc::c_uint);
                    let ref mut fresh2 = *col_sums.offset(x as isize);
                    *fresh2 =
                        (*fresh2).wrapping_add(*_img.offset((y1offs + x) as
                                                                isize) as
                                                   libc::c_uint);
                    x += 1
                }
            }
            y += 1
        }
        free(col_sums as *mut libc::c_void);
    }
    return mask;
}
