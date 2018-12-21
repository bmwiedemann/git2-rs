extern crate libc;
extern crate libgit2_sys as raw;
use std::ffi::{CStr};
use std::str;
pub use error::Error;
pub use index::{Index};
pub use oid::Oid;
pub use time::{Time, IndexTime};
pub use util::IntoCString;
pub enum ObjectType { }
#[macro_use] mod panic;
mod call;
mod util;
mod error;
mod index;
mod oid;
mod time;
impl ObjectType {
    pub fn str(&self) -> &'static str {
        unsafe {
            let ptr = call!(raw::git_object_type2string(*self)) as *const _;
            let data = CStr::from_ptr(ptr).to_bytes();
            str::from_utf8(data).unwrap()
        }
    }
}
pub enum FetchPrune { }
