extern crate libc;
extern crate libgit2_sys as raw;
use std::ffi::{CStr};
use std::str;
pub use error::Error;
pub use index::{Index};
pub use oid::Oid;
pub use repo::{Repository};
pub use time::{Time, IndexTime};
pub use util::IntoCString;
pub enum RepositoryState {
    ApplyMailboxOrRebase,
}
pub enum Direction {
}
pub enum ResetType {
}
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum ObjectType {
    Any,
    Commit,
    Tree,
    Blob,
    Tag,
}
pub enum BranchType {
}
pub enum ConfigLevel {
}
pub enum FileFavor {
}
#[macro_use] mod panic;
mod call;
mod util;
mod error;
mod index;
mod oid;
mod repo;
mod time;
impl ObjectType {
    pub fn str(&self) -> &'static str {
        unsafe {
            let ptr = call!(raw::git_object_type2string(*self)) as *const _;
            let data = CStr::from_ptr(ptr).to_bytes();
            str::from_utf8(data).unwrap()
        }
    }
    pub fn from_raw(raw: raw::git_otype) -> Option<ObjectType> {
        match raw {
            _ => None,
        }
    }
}
pub enum SubmoduleIgnore {
}
pub enum DiffFormat {
}
pub enum AutotagOption {
}
pub enum FetchPrune {
    Unspecified,
    On,
    Off,
}
