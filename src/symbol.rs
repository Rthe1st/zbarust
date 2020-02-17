use libc::{c_char, c_int, c_uint, c_void};

extern {
    pub fn zbar_symbol_ref(symbol: *const c_void, refs: c_int);
    pub fn zbar_symbol_get_type(symbol: *const c_void) -> c_int;
    pub fn zbar_symbol_get_configs(symbol: *const c_void) -> c_uint;
    pub fn zbar_symbol_get_modifiers(symbol: *const c_void) -> c_uint;
    pub fn zbar_symbol_get_data(symbol: *const c_void) -> *mut c_char;
    pub fn zbar_symbol_get_data_length(symbol: *const c_void) -> c_uint;
    pub fn zbar_symbol_get_quality(symbol: *const c_void) -> c_int;
    pub fn zbar_symbol_get_count(symbol: *const c_void) -> c_int;
    pub fn zbar_symbol_get_loc_size(symbol: *const c_void) -> c_uint;
    pub fn zbar_symbol_get_loc_x(symbol: *const c_void, index: c_uint) -> c_int;
    pub fn zbar_symbol_get_loc_y(symbol: *const c_void, index: c_uint) -> c_int;
    pub fn zbar_symbol_get_orientation(symbol: *const c_void) -> c_int;
    pub fn zbar_symbol_next(symbol: *const c_void) -> *const c_void;
    pub fn zbar_symbol_get_components(symbol: *const c_void) -> *const c_void;
    pub fn zbar_symbol_first_component(symbol: *const c_void) -> *const c_void;
    pub fn zbar_symbol_xml(
        symbol: *const c_void,
        buffer: *mut *mut c_char,
        buflen: *mut c_uint,
    ) -> *mut c_char;
}
