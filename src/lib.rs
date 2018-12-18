#![doc(html_root_url = "https://docs.rs/git2/0.6")]
#![allow(trivial_numeric_casts, trivial_casts)]
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
pub use diff::{DiffBinary, DiffBinaryFile, DiffBinaryKind};
pub use diff::{DiffLine, DiffHunk, DiffStats, DiffFindOptions};
pub use error::Error;
pub use index::{Index, IndexEntry, IndexEntries, IndexMatchedPath};
pub use merge::{AnnotatedCommit, MergeOptions};
pub use message::{message_prettify, DEFAULT_COMMENT_CHAR};
pub use note::{Note, Notes};
pub use object::Object;
pub use oid::Oid;
pub use packbuilder::{PackBuilder, PackBuilderStage};
pub use pathspec::{Pathspec, PathspecMatchList, PathspecFailedEntries};
pub use pathspec::{PathspecDiffEntries, PathspecEntries};
pub use patch::Patch;
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
pub use stash::{StashApplyOptions, StashCb, StashApplyProgressCb};
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
#[derive(PartialEq, Eq, Clone, Debug, Copy)]
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
#[derive(Copy, Clone)]
pub enum Direction {
    Fetch,
    Push,
}
#[derive(Copy, Clone)]
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
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum ReferenceType {
    Oid,
    Symbolic,
}
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum BranchType {
    Local,
    Remote,
}
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum ConfigLevel {
    ProgramData,
    System,
    XDG,
    Global,
    Local,
    App,
    Highest,
}
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum FileFavor {
    Normal,
    Ours,
    Theirs,
    Union,
}
bitflags! {
    pub struct Sort: u32 {
        const NONE = raw::GIT_SORT_NONE as u32;
        const TOPOLOGICAL = raw::GIT_SORT_TOPOLOGICAL as u32;
        const TIME = raw::GIT_SORT_TIME as u32;
        const REVERSE = raw::GIT_SORT_REVERSE as u32;
    }
}
impl Sort {
    is_bit_set!(is_none, Sort::NONE);
    is_bit_set!(is_topological, Sort::TOPOLOGICAL);
    is_bit_set!(is_time, Sort::TIME);
    is_bit_set!(is_reverse, Sort::REVERSE);
}
bitflags! {
    pub struct CredentialType: u32 {
        const USER_PASS_PLAINTEXT = raw::GIT_CREDTYPE_USERPASS_PLAINTEXT as u32;
        const SSH_KEY = raw::GIT_CREDTYPE_SSH_KEY as u32;
        const SSH_MEMORY = raw::GIT_CREDTYPE_SSH_MEMORY as u32;
        const SSH_CUSTOM = raw::GIT_CREDTYPE_SSH_CUSTOM as u32;
        const DEFAULT = raw::GIT_CREDTYPE_DEFAULT as u32;
        const SSH_INTERACTIVE = raw::GIT_CREDTYPE_SSH_INTERACTIVE as u32;
        const USERNAME = raw::GIT_CREDTYPE_USERNAME as u32;
    }
}
impl CredentialType {
    is_bit_set!(is_user_pass_plaintext, CredentialType::USER_PASS_PLAINTEXT);
    is_bit_set!(is_ssh_key, CredentialType::SSH_KEY);
    is_bit_set!(is_ssh_memory, CredentialType::SSH_MEMORY);
    is_bit_set!(is_ssh_custom, CredentialType::SSH_CUSTOM);
    is_bit_set!(is_default, CredentialType::DEFAULT);
    is_bit_set!(is_ssh_interactive, CredentialType::SSH_INTERACTIVE);
    is_bit_set!(is_username, CredentialType::USERNAME);
}
impl Default for CredentialType {
    fn default() -> Self {
        CredentialType::DEFAULT
    }
}
bitflags! {
    pub struct IndexEntryFlag: u16 {
        const EXTENDED = raw::GIT_IDXENTRY_EXTENDED as u16;
        const VALID = raw::GIT_IDXENTRY_VALID as u16;
    }
}
impl IndexEntryFlag {
    is_bit_set!(is_extended, IndexEntryFlag::EXTENDED);
    is_bit_set!(is_valid, IndexEntryFlag::VALID);
}
bitflags! {
    pub struct IndexEntryExtendedFlag: u16 {
        const INTENT_TO_ADD = raw::GIT_IDXENTRY_INTENT_TO_ADD as u16;
        const SKIP_WORKTREE = raw::GIT_IDXENTRY_SKIP_WORKTREE as u16;
        const EXTENDED2 = raw::GIT_IDXENTRY_EXTENDED2 as u16;
        const UPDATE = raw::GIT_IDXENTRY_UPDATE as u16;
        const REMOVE = raw::GIT_IDXENTRY_REMOVE as u16;
        const UPTODATE = raw::GIT_IDXENTRY_UPTODATE as u16;
        const ADDED = raw::GIT_IDXENTRY_ADDED as u16;
        const HASHED = raw::GIT_IDXENTRY_HASHED as u16;
        const UNHASHED = raw::GIT_IDXENTRY_UNHASHED as u16;
        const WT_REMOVE = raw::GIT_IDXENTRY_WT_REMOVE as u16;
        const CONFLICTED = raw::GIT_IDXENTRY_CONFLICTED as u16;
        const UNPACKED = raw::GIT_IDXENTRY_UNPACKED as u16;
        const NEW_SKIP_WORKTREE = raw::GIT_IDXENTRY_NEW_SKIP_WORKTREE as u16;
    }
}
impl IndexEntryExtendedFlag {
    is_bit_set!(is_intent_to_add, IndexEntryExtendedFlag::INTENT_TO_ADD);
    is_bit_set!(is_skip_worktree, IndexEntryExtendedFlag::SKIP_WORKTREE);
    is_bit_set!(is_extended2, IndexEntryExtendedFlag::EXTENDED2);
    is_bit_set!(is_update, IndexEntryExtendedFlag::UPDATE);
    is_bit_set!(is_remove, IndexEntryExtendedFlag::REMOVE);
    is_bit_set!(is_up_to_date, IndexEntryExtendedFlag::UPTODATE);
    is_bit_set!(is_added, IndexEntryExtendedFlag::ADDED);
    is_bit_set!(is_hashed, IndexEntryExtendedFlag::HASHED);
    is_bit_set!(is_unhashed, IndexEntryExtendedFlag::UNHASHED);
    is_bit_set!(is_wt_remove, IndexEntryExtendedFlag::WT_REMOVE);
    is_bit_set!(is_conflicted, IndexEntryExtendedFlag::CONFLICTED);
    is_bit_set!(is_unpacked, IndexEntryExtendedFlag::UNPACKED);
    is_bit_set!(is_new_skip_worktree, IndexEntryExtendedFlag::NEW_SKIP_WORKTREE);
}
bitflags! {
    pub struct IndexAddOption: u32 {
        const DEFAULT = raw::GIT_INDEX_ADD_DEFAULT as u32;
        const FORCE = raw::GIT_INDEX_ADD_FORCE as u32;
        const DISABLE_PATHSPEC_MATCH =
                raw::GIT_INDEX_ADD_DISABLE_PATHSPEC_MATCH as u32;
        const CHECK_PATHSPEC = raw::GIT_INDEX_ADD_CHECK_PATHSPEC as u32;
    }
}
impl IndexAddOption {
    is_bit_set!(is_default, IndexAddOption::DEFAULT);
    is_bit_set!(is_force, IndexAddOption::FORCE);
    is_bit_set!(is_disable_pathspec_match, IndexAddOption::DISABLE_PATHSPEC_MATCH);
    is_bit_set!(is_check_pathspec, IndexAddOption::CHECK_PATHSPEC);
}
impl Default for IndexAddOption {
    fn default() -> Self {
        IndexAddOption::DEFAULT
    }
}
bitflags! {
    pub struct RepositoryOpenFlags: u32 {
        const NO_SEARCH = raw::GIT_REPOSITORY_OPEN_NO_SEARCH as u32;
        const CROSS_FS = raw::GIT_REPOSITORY_OPEN_CROSS_FS as u32;
        const BARE = raw::GIT_REPOSITORY_OPEN_BARE as u32;
        const NO_DOTGIT = raw::GIT_REPOSITORY_OPEN_NO_DOTGIT as u32;
        const FROM_ENV = raw::GIT_REPOSITORY_OPEN_FROM_ENV as u32;
    }
}
impl RepositoryOpenFlags {
    is_bit_set!(is_no_search, RepositoryOpenFlags::NO_SEARCH);
    is_bit_set!(is_cross_fs, RepositoryOpenFlags::CROSS_FS);
    is_bit_set!(is_bare, RepositoryOpenFlags::BARE);
    is_bit_set!(is_no_dotgit, RepositoryOpenFlags::NO_DOTGIT);
    is_bit_set!(is_from_env, RepositoryOpenFlags::FROM_ENV);
}
bitflags! {
    pub struct RevparseMode: u32 {
        const SINGLE = raw::GIT_REVPARSE_SINGLE as u32;
        const RANGE = raw::GIT_REVPARSE_RANGE as u32;
        const MERGE_BASE = raw::GIT_REVPARSE_MERGE_BASE as u32;
    }
}
impl RevparseMode {
    is_bit_set!(is_no_single, RevparseMode::SINGLE);
    is_bit_set!(is_range, RevparseMode::RANGE);
    is_bit_set!(is_merge_base, RevparseMode::MERGE_BASE);
}
bitflags! {
    pub struct MergeAnalysis: u32 {
        const ANALYSIS_NONE = raw::GIT_MERGE_ANALYSIS_NONE as u32;
        const ANALYSIS_NORMAL = raw::GIT_MERGE_ANALYSIS_NORMAL as u32;
        const ANALYSIS_UP_TO_DATE = raw::GIT_MERGE_ANALYSIS_UP_TO_DATE as u32;
        const ANALYSIS_FASTFORWARD = raw::GIT_MERGE_ANALYSIS_FASTFORWARD as u32;
        const ANALYSIS_UNBORN = raw::GIT_MERGE_ANALYSIS_UNBORN as u32;
    }
}
impl MergeAnalysis {
    is_bit_set!(is_none, MergeAnalysis::ANALYSIS_NONE);
    is_bit_set!(is_normal, MergeAnalysis::ANALYSIS_NORMAL);
    is_bit_set!(is_up_to_date, MergeAnalysis::ANALYSIS_UP_TO_DATE);
    is_bit_set!(is_fast_forward, MergeAnalysis::ANALYSIS_FASTFORWARD);
    is_bit_set!(is_unborn, MergeAnalysis::ANALYSIS_UNBORN);
}
bitflags! {
    pub struct MergePreference: u32 {
        const NONE = raw::GIT_MERGE_PREFERENCE_NONE as u32;
        const NO_FAST_FORWARD = raw::GIT_MERGE_PREFERENCE_NO_FASTFORWARD as u32;
        const FASTFORWARD_ONLY = raw::GIT_MERGE_PREFERENCE_FASTFORWARD_ONLY as u32;
    }
}
impl MergePreference {
    is_bit_set!(is_none, MergePreference::NONE);
    is_bit_set!(is_no_fast_forward, MergePreference::NO_FAST_FORWARD);
    is_bit_set!(is_fastforward_only, MergePreference::FASTFORWARD_ONLY);
}
#[macro_use] mod panic;
mod call;
mod util;
pub mod build;
pub mod cert;
pub mod string_array;
pub mod oid_array;
pub mod transport;
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
    INIT.call_once(|| {
        openssl_env_init();
    });
    raw::init();
}
#[cfg(all(unix, not(target_os = "macos"), not(target_os = "ios"), feature = "https"))]
fn openssl_env_init() {
    extern crate openssl_probe;
    openssl_probe::init_ssl_cert_env_vars();
}
#[cfg(any(windows, target_os = "macos", target_os = "ios", not(feature = "https")))]
fn openssl_env_init() {}
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
            raw::GIT_OBJ_ANY => Some(ObjectType::Any),
            raw::GIT_OBJ_COMMIT => Some(ObjectType::Commit),
            raw::GIT_OBJ_TREE => Some(ObjectType::Tree),
            raw::GIT_OBJ_BLOB => Some(ObjectType::Blob),
            raw::GIT_OBJ_TAG => Some(ObjectType::Tag),
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
            raw::GIT_REF_OID => Some(ReferenceType::Oid),
            raw::GIT_REF_SYMBOLIC => Some(ReferenceType::Symbolic),
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
            raw::GIT_CONFIG_LEVEL_PROGRAMDATA => ConfigLevel::ProgramData,
            raw::GIT_CONFIG_LEVEL_SYSTEM => ConfigLevel::System,
            raw::GIT_CONFIG_LEVEL_XDG => ConfigLevel::XDG,
            raw::GIT_CONFIG_LEVEL_GLOBAL => ConfigLevel::Global,
            raw::GIT_CONFIG_LEVEL_LOCAL => ConfigLevel::Local,
            raw::GIT_CONFIG_LEVEL_APP => ConfigLevel::App,
            raw::GIT_CONFIG_HIGHEST_LEVEL => ConfigLevel::Highest,
            n => panic!("unknown config level: {}", n),
        }
    }
}
bitflags! {
    pub struct Status: u32 {
        const CURRENT = raw::GIT_STATUS_CURRENT as u32;
        const INDEX_NEW = raw::GIT_STATUS_INDEX_NEW as u32;
        const INDEX_MODIFIED = raw::GIT_STATUS_INDEX_MODIFIED as u32;
        const INDEX_DELETED = raw::GIT_STATUS_INDEX_DELETED as u32;
        const INDEX_RENAMED = raw::GIT_STATUS_INDEX_RENAMED as u32;
        const INDEX_TYPECHANGE = raw::GIT_STATUS_INDEX_TYPECHANGE as u32;
        const WT_NEW = raw::GIT_STATUS_WT_NEW as u32;
        const WT_MODIFIED = raw::GIT_STATUS_WT_MODIFIED as u32;
        const WT_DELETED = raw::GIT_STATUS_WT_DELETED as u32;
        const WT_TYPECHANGE = raw::GIT_STATUS_WT_TYPECHANGE as u32;
        const WT_RENAMED = raw::GIT_STATUS_WT_RENAMED as u32;
        const IGNORED = raw::GIT_STATUS_IGNORED as u32;
        const CONFLICTED = raw::GIT_STATUS_CONFLICTED as u32;
    }
}
impl Status {
    is_bit_set!(is_index_new, Status::INDEX_NEW);
    is_bit_set!(is_index_modified, Status::INDEX_MODIFIED);
    is_bit_set!(is_index_deleted, Status::INDEX_DELETED);
    is_bit_set!(is_index_renamed, Status::INDEX_RENAMED);
    is_bit_set!(is_index_typechange, Status::INDEX_TYPECHANGE);
    is_bit_set!(is_wt_new, Status::WT_NEW);
    is_bit_set!(is_wt_modified, Status::WT_MODIFIED);
}
bitflags! {
    pub struct RepositoryInitMode: u32 {
        const SHARED_UMASK = raw::GIT_REPOSITORY_INIT_SHARED_UMASK as u32;
    }
}
impl RepositoryInitMode {
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
        const WD_UNINITIALIZED =
                raw::GIT_SUBMODULE_STATUS_WD_UNINITIALIZED as u32;
        const WD_INDEX_MODIFIED =
                raw::GIT_SUBMODULE_STATUS_WD_INDEX_MODIFIED as u32;
    }
}
impl SubmoduleStatus {
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
impl PathspecFlags {
}
impl Default for PathspecFlags {
    fn default() -> Self {
        PathspecFlags::DEFAULT
    }
}
bitflags! {
    pub struct CheckoutNotificationType: u32 {
        const IGNORED = raw::GIT_CHECKOUT_NOTIFY_IGNORED as u32;
    }
}
impl CheckoutNotificationType {
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
impl DiffStatsFormat {
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
