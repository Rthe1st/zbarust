use libc;

pub type RefcntT = libc::c_int;
extern {
    pub fn _zbar_refcnt(cnt: *mut RefcntT, delta: libc::c_int) -> libc::c_int;
}
extern {
    pub fn _zbar_refcnt_init();
}
