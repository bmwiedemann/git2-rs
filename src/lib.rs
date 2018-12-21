extern crate libc;
extern crate libgit2_sys as raw;
#[macro_use] extern crate bitflags;
use std::ffi::{CStr, CString};
use std::str;
pub use blame::{Blame, BlameHunk, BlameIter, BlameOptions};
pub use blob::{Blob, BlobWriter};
pub use branch::{Branch, Branches};
pub use buf::Buf;
pub use commit::{Commit, Parents};
pub use config::{Config, ConfigEntry, ConfigEntries};
pub use cred::{Cred, CredentialHelper};
pub use describe::{Describe, DescribeFormatOptions, DescribeOptions};
pub use diff::{Diff, DiffDelta, DiffFile, DiffOptions, Deltas};
pub use error::Error;
pub use index::{Index, IndexEntry, IndexEntries, IndexMatchedPath};
pub use merge::{AnnotatedCommit, MergeOptions};
pub use object::Object;
pub use oid::Oid;
pub use proxy_options::ProxyOptions;
pub use reference::{Reference, References, ReferenceNames};
pub use reflog::{Reflog, ReflogEntry, ReflogIter};
pub use refspec::Refspec;
pub use remote::{Remote, RemoteConnection, Refspecs, RemoteHead, FetchOptions, PushOptions};
pub use remote_callbacks::{RemoteCallbacks, Credentials, TransferProgress};
pub use remote_callbacks::{TransportMessage, Progress, UpdateTips};
pub use repo::{Repository, RepositoryInitOptions};
pub use revspec::Revspec;
pub use signature::Signature;
pub use submodule::{Submodule, SubmoduleUpdateOptions};
pub use tag::Tag;
pub use time::{Time, IndexTime};
pub use tree::{Tree, TreeEntry, TreeIter, TreeWalkMode, TreeWalkResult};
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
pub enum ReferenceType {
}
pub enum BranchType {
}
pub enum ConfigLevel {
    ProgramData,
    System,
    XDG,
    Global,
    Local,
    App,
    Highest,
}
pub enum FileFavor {
    Normal,
    Ours,
    Theirs,
    Union,
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
    pub struct RepositoryOpenFlags: u32 {
        const NO_SEARCH = raw::GIT_REPOSITORY_OPEN_NO_SEARCH as u32;
    }
}
bitflags! {
    pub struct RevparseMode: u32 {
        const SINGLE = raw::GIT_REVPARSE_SINGLE as u32;
    }
}
bitflags! {
    pub struct MergeAnalysis: u32 {
        const ANALYSIS_NONE = raw::GIT_MERGE_ANALYSIS_NONE as u32;
    }
}
bitflags! {
    pub struct MergePreference: u32 {
        const NONE = raw::GIT_MERGE_PREFERENCE_NONE as u32;
    }
}
#[macro_use] mod panic;
mod call;
mod util;
pub mod build;
pub mod cert;
mod blame;
mod blob;
mod branch;
mod buf;
mod commit;
mod config;
mod cred;
mod describe;
mod diff;
mod error;
mod index;
mod merge;
mod object;
mod oid;
mod proxy_options;
mod reference;
mod reflog;
mod refspec;
mod remote;
mod remote_callbacks;
mod repo;
mod revspec;
mod signature;
mod submodule;
mod tag;
mod time;
mod tree;
fn init() {
}
unsafe fn opt_bytes<'a, T>(_anchor: &'a T,
                           c: *const libc::c_char) -> Option<&'a [u8]> {
    if c.is_null() {
        None
    } else {
        Some(CStr::from_ptr(c).to_bytes())
    }
}
fn opt_cstr<T: IntoCString>(o: Option<T>) -> Result<Option<CString>, Error> {
    match o {
        Some(s) => s.into_c_string().map(Some),
        None => Ok(None)
    }
}
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
bitflags! {
    pub struct RepositoryInitMode: u32 {
        const SHARED_UMASK = raw::GIT_REPOSITORY_INIT_SHARED_UMASK as u32;
    }
}
pub enum Delta {
}
bitflags! {
    pub struct SubmoduleStatus: u32 {
        const IN_INDEX = raw::GIT_SUBMODULE_STATUS_IN_INDEX as u32;
    }
}
pub enum SubmoduleIgnore {
    Unspecified,
    None,
    Untracked,
    Dirty,
    All,
}
bitflags! {
    pub struct CheckoutNotificationType: u32 {
        const IGNORED = raw::GIT_CHECKOUT_NOTIFY_IGNORED as u32;
    }
}
pub enum DiffFormat {
    Patch,
    PatchHeader,
    Raw,
    NameOnly,
    NameStatus,
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
