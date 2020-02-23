//assumes threading is enabled
// zbar supports this differently for each of windows/mac/linux
// so try to wrap up in here and eventually switched for rust threading

// the support is done with C macros, and if threading is disabled a lot of this ends up different and will probs break

use libc::c_int;

type Refcount = c_int;

extern {
    pub fn _zbar_refcnt(cnt: *mut Refcount, delta: c_int) -> c_int;
    pub fn _zbar_refcnt_init();
}
