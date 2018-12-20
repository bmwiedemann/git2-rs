use std::ptr;
use util::Binding;
pub struct Buf {
    raw: raw::git_buf,
}
impl Buf {
    pub fn new() -> Buf {
        unsafe {
            Binding::from_raw(&mut raw::git_buf {
                ptr: ptr::null_mut(),
                size: 0,
                asize: 0,
            } as *mut _)
        }
    }
}
impl Binding for Buf {
    type Raw = *mut raw::git_buf;
    unsafe fn from_raw(raw: *mut raw::git_buf) -> Buf {
        Buf { raw: *raw }
    }
    fn raw(&self) -> *mut raw::git_buf { &self.raw as *const _ as *mut _ }
}
