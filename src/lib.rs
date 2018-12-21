extern crate libc;
extern crate libgit2_sys as raw;
use std::ffi::CStr;
use std::str;
pub use index::Index;
pub use util::IntoCString;
#[macro_use] mod panic;
mod call;
mod util;
mod index;
pub struct Error { }
pub enum ObjectType { }
impl ObjectType {
    pub fn str(&self) -> &'static str {
        unsafe {
            let ptr = call!(raw::git_object_type2string(*self)) as *const _;
            let data = CStr::from_ptr(ptr).to_bytes();
            str::from_utf8(data).unwrap()
        }
    }
}
