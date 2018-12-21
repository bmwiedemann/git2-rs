extern crate libc;
extern crate libgit2_sys as raw;
#[macro_use] extern crate bitflags;
use std::ffi::{CStr};
use std::str;
pub use error::Error;
pub use index::{Index};
pub use object::Object;
pub use oid::Oid;
pub use repo::{Repository};
pub use revspec::Revspec;
pub use time::{Time, IndexTime};
pub use tree::{Tree};
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
bitflags! {
    pub struct CredentialType: u32 {
        const DEFAULT = raw::GIT_CREDTYPE_DEFAULT as u32;
    }
}
bitflags! {
    pub struct IndexAddOption: u32 {
        const DEFAULT = raw::GIT_INDEX_ADD_DEFAULT as u32;
    }
}
bitflags! {
    pub struct RevparseMode: u32 {
        const SINGLE = raw::GIT_REVPARSE_SINGLE as u32;
    }
}
#[macro_use] mod panic;
mod call;
mod util;
mod error;
mod index;
mod object;
mod oid;
mod repo;
mod revspec;
mod time;
mod tree;
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
    Unspecified,
    Auto,
    None,
    All,
}
pub enum FetchPrune {
    Unspecified,
    On,
    Off,
}
bitflags! {
    pub struct StashFlags: u32 {
        const DEFAULT = raw::GIT_STASH_DEFAULT as u32;
    }
}
