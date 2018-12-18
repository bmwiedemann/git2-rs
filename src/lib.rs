extern crate libc;
extern crate url;
extern crate libgit2_sys as raw;
#[macro_use] extern crate bitflags;
#[macro_use] extern crate log;
use std::ffi::{CStr, CString};
use std::fmt;
use std::str;
use std::sync::{Once, ONCE_INIT};
pub use blame::{Blame, BlameHunk, BlameIter, BlameOptions};
pub use blob::{Blob, BlobWriter};
pub use branch::{Branch, Branches};
pub use buf::Buf;
pub use commit::{Commit, Parents};
pub use config::{Config, ConfigEntry, ConfigEntries};
pub use cred::{Cred, CredentialHelper};
pub use describe::{Describe, DescribeFormatOptions, DescribeOptions};
pub use diff::{Diff, DiffDelta, DiffFile, DiffOptions, Deltas};
pub use diff::{DiffLine, DiffHunk, DiffStats, DiffFindOptions};
pub use error::Error;
pub use index::{Index, IndexEntry, IndexEntries, IndexMatchedPath};
pub use merge::{AnnotatedCommit, MergeOptions};
pub use note::{Note, Notes};
pub use object::Object;
pub use oid::Oid;
pub use packbuilder::{PackBuilder, PackBuilderStage};
pub use proxy_options::ProxyOptions;
pub use rebase::{Rebase, RebaseOptions, RebaseOperation, RebaseOperationType};
pub use reference::{Reference, References, ReferenceNames};
pub use reflog::{Reflog, ReflogEntry, ReflogIter};
pub use refspec::Refspec;
pub use remote::{Remote, RemoteConnection, Refspecs, RemoteHead, FetchOptions, PushOptions};
pub use remote_callbacks::{RemoteCallbacks, Credentials, TransferProgress};
pub use remote_callbacks::{TransportMessage, Progress, UpdateTips};
pub use repo::{Repository, RepositoryInitOptions};
pub use revspec::Revspec;
pub use revwalk::Revwalk;
pub use signature::Signature;
pub use status::{StatusOptions, Statuses, StatusIter, StatusEntry, StatusShow};
pub use submodule::{Submodule, SubmoduleUpdateOptions};
pub use tag::Tag;
pub use time::{Time, IndexTime};
pub use tree::{Tree, TreeEntry, TreeIter, TreeWalkMode, TreeWalkResult};
pub use treebuilder::TreeBuilder;
pub use odb::{Odb, OdbObject, OdbReader, OdbWriter};
pub use util::IntoCString;
macro_rules! is_bit_set {
    ($name:ident, $flag:expr) => (
        pub fn $name(&self) -> bool {
            self.intersects($flag)
        }
    )
}
#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum ErrorCode {
    GenericError,
    NotFound,
    Exists,
    Ambiguous,
    BufSize,
    User,
    BareRepo,
    UnbornBranch,
    Unmerged,
    NotFastForward,
    InvalidSpec,
    Conflict,
    Locked,
    Modified,
    Auth,
    Certificate,
    Applied,
    Peel,
    Eof,
    Invalid,
    Uncommitted,
    Directory,
}
#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum ErrorClass {
    None,
    NoMemory,
    Os,
    Invalid,
    Reference,
    Zlib,
    Repository,
    Config,
    Regex,
    Odb,
    Index,
    Object,
    Net,
    Tag,
    Tree,
    Indexer,
    Ssl,
    Submodule,
    Thread,
    Stash,
    Checkout,
    FetchHead,
    Merge,
    Ssh,
    Filter,
    Revert,
    Callback,
    CherryPick,
    Describe,
    Rebase,
    Filesystem,
}
pub enum RepositoryState {
    Clean,
    Merge,
    Revert,
    RevertSequence,
    CherryPick,
    CherryPickSequence,
    Bisect,
    Rebase,
    RebaseInteractive,
    RebaseMerge,
    ApplyMailbox,
    ApplyMailboxOrRebase,
}
pub enum Direction {
    Fetch,
    Push,
}
pub enum ResetType {
    Soft,
    Mixed,
    Hard,
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
    Oid,
    Symbolic,
}
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum BranchType {
    Local,
    Remote,
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
    pub struct Sort: u32 {
        const NONE = raw::GIT_SORT_NONE as u32;
    }
}
impl Sort {
    is_bit_set!(is_none, Sort::NONE);
}
bitflags! {
    pub struct CredentialType: u32 {
        const DEFAULT = raw::GIT_CREDTYPE_DEFAULT as u32;
    }
}
impl Default for CredentialType {
    fn default() -> Self {
        CredentialType::DEFAULT
    }
}
bitflags! {
    pub struct IndexEntryFlag: u16 {
        const EXTENDED = raw::GIT_IDXENTRY_EXTENDED as u16;
    }
}
bitflags! {
    pub struct IndexEntryExtendedFlag: u16 {
        const INTENT_TO_ADD = raw::GIT_IDXENTRY_INTENT_TO_ADD as u16;
    }
}
bitflags! {
    pub struct IndexAddOption: u32 {
        const DEFAULT = raw::GIT_INDEX_ADD_DEFAULT as u32;
    }
}
impl Default for IndexAddOption {
    fn default() -> Self {
        IndexAddOption::DEFAULT
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
pub mod string_array;
pub mod oid_array;
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
mod message;
mod note;
mod object;
mod odb;
mod oid;
mod packbuilder;
mod pathspec;
mod patch;
mod proxy_options;
mod rebase;
mod reference;
mod reflog;
mod refspec;
mod remote;
mod remote_callbacks;
mod repo;
mod revspec;
mod revwalk;
mod signature;
mod status;
mod submodule;
mod stash;
mod tag;
mod time;
mod tree;
mod treebuilder;
fn init() {
    static INIT: Once = ONCE_INIT;
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
    pub fn is_loose(&self) -> bool {
        unsafe { (call!(raw::git_object_typeisloose(*self)) == 1) }
    }
    pub fn from_raw(raw: raw::git_otype) -> Option<ObjectType> {
        match raw {
            _ => None,
        }
    }
    pub fn raw(&self) -> raw::git_otype {
        call::convert(self)
    }
    pub fn from_str(s: &str) -> Option<ObjectType> {
        let raw = unsafe { call!(raw::git_object_string2type(CString::new(s).unwrap())) };
        ObjectType::from_raw(raw)
    }
}
impl fmt::Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.str().fmt(f)
    }
}
impl ReferenceType {
    pub fn str(&self) -> &'static str {
        match self {
            &ReferenceType::Oid => "oid",
            &ReferenceType::Symbolic => "symbolic",
        }
    }
    pub fn from_raw(raw: raw::git_ref_t) -> Option<ReferenceType> {
        match raw {
            _ => None,
        }
    }
}
impl fmt::Display for ReferenceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.str().fmt(f)
    }
}
impl ConfigLevel {
    pub fn from_raw(raw: raw::git_config_level_t) -> ConfigLevel {
        match raw {
            n => panic!("unknown config level: {}", n),
        }
    }
}
bitflags! {
    pub struct Status: u32 {
        const CURRENT = raw::GIT_STATUS_CURRENT as u32;
    }
}
bitflags! {
    pub struct RepositoryInitMode: u32 {
        const SHARED_UMASK = raw::GIT_REPOSITORY_INIT_SHARED_UMASK as u32;
    }
}
pub enum Delta {
    Unmodified,
    Added,
    Deleted,
    Modified,
    Renamed,
    Copied,
    Ignored,
    Untracked,
    Typechange,
    Unreadable,
    Conflicted,
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
    pub struct PathspecFlags: u32 {
        const DEFAULT = raw::GIT_PATHSPEC_DEFAULT as u32;
    }
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
bitflags! {
    pub struct DiffStatsFormat: raw::git_diff_stats_format_t {
        const INCLUDE_SUMMARY = raw::GIT_DIFF_STATS_INCLUDE_SUMMARY;
    }
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
pub enum StashApplyProgress {
    None,
    LoadingStash,
    AnalyzeIndex,
    AnalyzeModified,
    AnalyzeUntracked,
    CheckoutUntracked,
    CheckoutModified,
    Done,
}
bitflags! {
    pub struct StashApplyFlags: u32 {
        const DEFAULT = raw::GIT_STASH_APPLY_DEFAULT as u32;
    }
}
bitflags! {
    pub struct StashFlags: u32 {
        const DEFAULT = raw::GIT_STASH_DEFAULT as u32;
    }
}
