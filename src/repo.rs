use std::ffi::{CStr, CString, OsStr};
use std::mem;
use std::path::Path;
use std::ptr;
use libc::{c_int, c_char, size_t, c_void, c_uint};
use {raw, Revspec, Error, init, Object, RepositoryOpenFlags, RepositoryState, Remote, Buf, StashFlags};
use {ResetType, Signature, Reference, References, Submodule, Blame, BlameOptions};
use {Branches, BranchType, Index, Config, Oid, Blob, BlobWriter, Branch, Commit, Tree};
use {AnnotatedCommit, MergeOptions, SubmoduleIgnore, SubmoduleStatus, MergeAnalysis, MergePreference};
use {RevparseMode, RepositoryInitMode, Reflog, IntoCString, Describe};
use build::{RepoBuilder, CheckoutBuilder};
use util::{self, Binding};
pub struct Repository {
    raw: *mut raw::git_repository,
}
pub struct RepositoryInitOptions {
    flags: u32,
    mode: u32,
    workdir_path: Option<CString>,
    description: Option<CString>,
    template_path: Option<CString>,
    initial_head: Option<CString>,
    origin_url: Option<CString>,
}
impl Repository {
    pub fn revparse(&self, spec: &str) -> Result<Revspec, Error> {
        let mut raw = raw::git_revspec {
            from: ptr::null_mut(),
            to: ptr::null_mut(),
            flags: 0,
        };
        unsafe {
            let to = Binding::from_raw_opt(raw.to);
            let from = Binding::from_raw_opt(raw.from);
            let mode = RevparseMode::from_bits_truncate(raw.flags as u32);
            Ok(Revspec::from_objects(from, to, mode))
        }
    }
    pub fn state(&self) -> RepositoryState {
        let state = unsafe { raw::git_repository_state(self.raw) };
        macro_rules! check( ($($raw:ident => $real:ident),*) => (
            $(if state == raw::$raw as c_int {
                super::RepositoryState::$real
            }) else *
            else {
                panic!("unknown repository state: {}", state)
            }
        ) );
        check!(
            GIT_REPOSITORY_STATE_APPLY_MAILBOX_OR_REBASE => ApplyMailboxOrRebase
        )
    }
    pub fn workdir(&self) -> Option<&Path> {
        unsafe {
            let ptr = raw::git_repository_workdir(self.raw);
            if ptr.is_null() {
                None
            } else {
                Some(util::bytes2path(CStr::from_ptr(ptr).to_bytes()))
            }
        }
    }
    pub fn branch(&self,
                  update_ref: Option<&str>,
                  author: &Signature,
                  committer: &Signature,
                  message: &str,
                  tree: &Tree,
                  parents: &[&Commit]) -> Result<Oid, Error> {
        let update_ref = try!(::opt_cstr(update_ref));
        let mut parent_ptrs = parents.iter().map(|p| {
            p.raw() as *const raw::git_commit
        }).collect::<Vec<_>>();
        let message = try!(CString::new(message));
        let mut raw = raw::git_oid { id: [0; raw::GIT_OID_RAWSZ] };
        unsafe {
            try_call!(raw::git_commit_create(&mut raw,
                                             self.raw(),
                                             update_ref,
                                             author.raw(),
                                             committer.raw(),
                                             ptr::null(),
                                             message,
                                             tree.raw(),
                                             parents.len() as size_t,
                                             parent_ptrs.as_mut_ptr()));
            Ok(Binding::from_raw(&raw as *const _))
        }
    }
    pub fn commit_signed(&self,
                 annotated_commits: &[&AnnotatedCommit],
                 merge_opts: Option<&mut MergeOptions>,
                 checkout_opts: Option<&mut CheckoutBuilder>)
                 -> Result<(), Error>
    {
        unsafe {
            let mut raw_checkout_opts = mem::zeroed();
            let mut commit_ptrs = annotated_commits.iter().map(|c| {
                c.raw() as *const raw::git_annotated_commit
            }).collect::<Vec<_>>();
            try_call!(raw::git_merge(self.raw,
                                     commit_ptrs.as_mut_ptr(),
                                     annotated_commits.len() as size_t,
                                     merge_opts.map(|o| o.raw())
                                               .unwrap_or(ptr::null()),
                                     &raw_checkout_opts));
        }
        Ok(())
    }
}
impl Binding for Repository {
    type Raw = *mut raw::git_repository;
    unsafe fn from_raw(ptr: *mut raw::git_repository) -> Repository {
        Repository { raw: ptr }
    }
    fn raw(&self) -> *mut raw::git_repository { self.raw }
}
impl RepositoryInitOptions {
    pub fn new() -> RepositoryInitOptions {
        RepositoryInitOptions {
            flags: raw::GIT_REPOSITORY_INIT_MKDIR as u32 |
                   raw::GIT_REPOSITORY_INIT_EXTERNAL_TEMPLATE as u32,
            mode: 0,
            workdir_path: None,
            description: None,
            template_path: None,
            initial_head: None,
            origin_url: None,
        }
    }
    pub unsafe fn raw(&self) -> raw::git_repository_init_options {
        let mut opts = mem::zeroed();
        opts
    }
}
