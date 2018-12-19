use std::slice;
use std::ptr;
use std::ops::{Deref, DerefMut};
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
impl Deref for Buf {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.raw.ptr as *const u8,
                                  self.raw.size as usize)
        }
    }
}
impl DerefMut for Buf {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self.raw.ptr as *mut u8,
                                      self.raw.size as usize)
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
