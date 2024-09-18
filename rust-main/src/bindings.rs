use std::ffi::c_long;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GoInterface {
    pub t: *mut ::std::os::raw::c_void,
    pub v: *mut ::std::os::raw::c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GoSlice {
    pub data: *mut ::std::os::raw::c_void,
    pub len: c_long,
    pub cap: c_long,
}

#[repr(C)]
pub struct GoString {
    pub p: *const u8,
    pub n: isize,
}

impl From<String> for GoString {
    fn from(s: String) -> Self {
        let s = s.into_bytes();
        let p = s.as_ptr();
        let n = s.len() as isize;
        std::mem::forget(s);
        GoString { p, n }
    }
}

/* -------------------------------------------------------------------------- */
/*                            FUNCTION DECLARATIONS                           */
/* -------------------------------------------------------------------------- */

/* ------------------------ GOLANG RUNTIME FUNCTIONS ------------------------ */
extern "C" {
    pub fn StartGolang();
}
extern "C" {
    pub fn SendToMessageChannel(msg: GoString);
}
extern "C" {
    pub fn QuitGolang();
}

/* -------------------------- RUST BRIDGE FUNCTIONS ------------------------- */
extern "C" {
    pub fn get_receiver() -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn get_sender() -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn send_to_channel(message: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn create_channels();
}
