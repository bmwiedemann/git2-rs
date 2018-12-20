use std::ffi::{CStr, CString, OsStr};
use std::mem;
use std::path::Path;
use std::ptr;
use libc::{c_int, c_char, size_t, c_void, c_uint};
use {raw, Revspec, Error, init, Object, RepositoryOpenFlags, RepositoryState, Remote, Buf, StashFlags};
use {ResetType, Signature, Reference, References, Submodule, Blame, BlameOptions};
use {Branches, BranchType, Index, Config, Oid, Blob, BlobWriter, Branch, Commit, Tree};
use {AnnotatedCommit, MergeOptions, SubmoduleIgnore, SubmoduleStatus, MergeAnalysis, MergePreference};
use {ObjectType, Tag, Note, Notes, StatusOptions, Statuses, Status, Revwalk};
use {RevparseMode, RepositoryInitMode, Reflog, IntoCString, Describe};
use {DescribeOptions, TreeBuilder, Diff, DiffOptions, PackBuilder, Odb};
use {Rebase, RebaseOptions};
use build::{RepoBuilder, CheckoutBuilder};
use stash::{StashApplyOptions, StashCbData, stash_cb};
use string_array::StringArray;
use oid_array::OidArray;
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
    pub fn open_ext<P, O, I>(path: P,
                             ceiling_dirs: I)
                             -> Result<Repository, Error>
    {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn clone<P: AsRef<Path>>(url: &str, into: P)
                                 -> Result<Repository, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(ret))
        }
    }
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
    pub fn revparse_ext(&self, spec: &str)
                        -> Result<(Object, Option<Reference>), Error> {
        let mut git_obj = ptr::null_mut();
        let mut git_ref = ptr::null_mut();
        unsafe {
            Ok((Binding::from_raw(git_obj), Binding::from_raw_opt(git_ref)))
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
    pub fn set_workdir(&self, path: &Path, update_gitlink: bool)
                       -> Result<(), Error> {
        unsafe {
            let value = raw::git_repository_head_detached(self.raw);
            match value {
                _ => Err(Error::last_error(value).unwrap())
            }
        }
    }
    pub fn statuses(&self, options: Option<&mut StatusOptions>)
                    -> Result<Statuses, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn branches(&self, filter: Option<BranchType>)
                    -> Result<Branches, Error> {
        let mut raw = ptr::null_mut();
        unsafe {
            Ok(Branches::from_raw(raw))
        }
    }
    pub fn branch(&self,
                  force: bool) -> Result<Branch, Error> {
        let mut raw = ptr::null_mut();
        unsafe {
            Ok(Branch::wrap(Binding::from_raw(raw)))
        }
    }
    pub fn find_branch(&self, name: &str, branch_type: BranchType)
                       -> Result<Branch, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(Branch::wrap(Binding::from_raw(ret)))
        }
    }
    pub fn commit(&self,
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
                         signature_field: Option<&str>) -> Result<Oid, Error> {
        let mut raw = raw::git_oid { id: [0; raw::GIT_OID_RAWSZ] };
        unsafe {
            Ok(Binding::from_raw(&raw as *const _))
        }
    }
    pub fn extract_signature(&self,
                             signature_field: Option<&str>)
                             -> Result<(Buf, Buf), Error> {
        let signature = Buf::new();
        let content = Buf::new();
        unsafe {
            Ok((signature, content))
        }
    }
    pub fn find_object(&self, oid: Oid,
                       kind: Option<ObjectType>) -> Result<Object, Error> {
        let mut raw = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(raw))
        }
    }
    pub fn reference(&self, name: &str, id: Oid, force: bool,
                     log_message: &str) -> Result<Reference, Error> {
        let mut raw = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(raw))
        }
    }
    pub fn reference_matching(&self,
                              name: &str,
                              id: Oid,
                              force: bool,
                              current_id: Oid,
                              log_message: &str) -> Result<Reference, Error> {
        let name = try!(CString::new(name));
        let log_message = try!(CString::new(log_message));
        let mut raw = ptr::null_mut();
        unsafe {
            try_call!(raw::git_reference_create_matching(&mut raw,
                                                         self.raw(),
                                                         name,
                                                         id.raw(),
                                                         force,
                                                         current_id.raw(),
                                                         log_message));
            Ok(Binding::from_raw(raw))
        }
    }
    pub fn reference_symbolic(&self, name: &str, target: &str,
                              log_message: &str)
                              -> Result<Reference, Error> {
        let mut raw = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(raw))
        }
    }
    pub fn reference_symbolic_matching(&self,
                                       name: &str,
                                       target: &str,
                                       force: bool,
                                       current_value: &str,
                                       log_message: &str)
                                       -> Result<Reference, Error> {
        let name = try!(CString::new(name));
        let target = try!(CString::new(target));
        let current_value = try!(CString::new(current_value));
        let log_message = try!(CString::new(log_message));
        let mut raw = ptr::null_mut();
        unsafe {
            try_call!(raw::git_reference_symbolic_create_matching(&mut raw,
                                                                  self.raw(),
                                                                  name,
                                                                  target,
                                                                  force,
                                                                  current_value,
                                                                  log_message));
            Ok(Binding::from_raw(raw))
        }
    }
    pub fn reference_to_annotated_commit(&self, reference: &Reference)
                                         -> Result<AnnotatedCommit, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(AnnotatedCommit::from_raw(ret))
        }
    }
    pub fn submodule(&self, url: &str, path: &Path,
                     use_gitlink: bool) -> Result<Submodule, Error> {
        let mut raw = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(raw))
        }
    }
    pub fn tag(&self, name: &str, target: &Object,
               force: bool) -> Result<Oid, Error> {
        let mut raw = raw::git_oid { id: [0; raw::GIT_OID_RAWSZ] };
        unsafe {
            Ok(Binding::from_raw(&raw as *const _))
        }
    }
    pub fn tag_names(&self, pattern: Option<&str>) -> Result<StringArray, Error> {
        let mut arr = raw::git_strarray {
            strings: 0 as *mut *mut c_char,
            count: 0,
        };
        unsafe {
            Ok(Binding::from_raw(arr))
        }
    }
    pub fn merge(&self,
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
    pub fn merge_commits(&self, our_commit: &Commit, their_commit: &Commit,
                         opts: Option<&MergeOptions>) -> Result<Index, Error> {
         let mut raw = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(raw))
        }
    }
    pub fn merge_trees(&self, ancestor_tree: &Tree, our_tree: &Tree,
                       their_tree: &Tree, opts: Option<&MergeOptions>) -> Result<Index, Error> {
        let mut raw = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(raw))
        }
    }
    pub fn merge_analysis(&self,
                          their_heads: &[&AnnotatedCommit])
                          -> Result<(MergeAnalysis, MergePreference), Error> {
        unsafe {
            let mut raw_merge_analysis = 0 as raw::git_merge_analysis_t;
            let mut raw_merge_preference = 0 as raw::git_merge_preference_t;
            Ok((MergeAnalysis::from_bits_truncate(raw_merge_analysis as u32), MergePreference::from_bits_truncate(raw_merge_preference as u32)))
        }
    }
    pub fn rebase(&self,
                  branch: Option<&AnnotatedCommit>,
                  upstream: Option<&AnnotatedCommit>,
                  onto: Option<&AnnotatedCommit>,
                  opts: Option<&mut RebaseOptions>) -> Result<Rebase, Error> {
        let mut rebase: *mut raw::git_rebase = ptr::null_mut();
        unsafe {
            try_call!(raw::git_rebase_init(
                &mut rebase,
                self.raw(),
                branch.map(|c| c.raw()),
                upstream.map(|c| c.raw()),
                onto.map(|c| c.raw()),
                opts.map(|o| o.raw()).unwrap_or(ptr::null())));
                Ok(Rebase::from_raw(rebase))
        }
    }
    pub fn note(&self,
                author: &Signature,
                committer: &Signature,
                notes_ref: Option<&str>,
                oid: Oid,
                note: &str,
                force: bool) -> Result<Oid, Error> {
        let notes_ref = try!(::opt_cstr(notes_ref));
        let note = try!(CString::new(note));
        let mut ret = raw::git_oid { id: [0; raw::GIT_OID_RAWSZ] };
        unsafe {
            try_call!(raw::git_note_create(&mut ret,
                                           self.raw,
                                           notes_ref,
                                           author.raw(),
                                           committer.raw(),
                                           oid.raw(),
                                           note,
                                           force));
            Ok(Binding::from_raw(&ret as *const _))
        }
    }
    pub fn find_note(&self, notes_ref: Option<&str>, id: Oid)
                     -> Result<Note, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn blame_file(&self, path: &Path, opts: Option<&mut BlameOptions>)
                      -> Result<Blame, Error> {
        let mut raw = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(raw))
        }
    }
    pub fn merge_bases(&self, one: Oid, two: Oid) -> Result<OidArray, Error> {
        let mut arr = raw::git_oidarray {
            ids: ptr::null_mut(),
            count: 0,
        };
        unsafe {
            Ok(Binding::from_raw(arr))
        }
    }
    pub fn graph_ahead_behind(&self, local: Oid, upstream: Oid)
                              -> Result<(usize, usize), Error> {
        unsafe {
            let mut ahead: size_t = 0;
            let mut behind: size_t = 0;
            Ok((ahead as usize, behind as usize))
        }
    }
    pub fn graph_descendant_of(&self, commit: Oid, ancestor: Oid)
                               -> Result<bool, Error> {
        unsafe {
            let rv = try_call!(raw::git_graph_descendant_of(self.raw(),
                                                            commit.raw(),
                                                            ancestor.raw()));
            Ok(rv != 0)
        }
    }
    pub fn reference_has_log(&self, name: &str) -> Result<bool, Error> {
        let name = try!(CString::new(name));
        let ret = unsafe {
            try_call!(raw::git_reference_has_log(self.raw, name))
        };
        Ok(ret != 0)
    }
    pub fn diff_tree_to_tree(&self,
                             opts: Option<&mut DiffOptions>)
                             -> Result<Diff, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn diff_tree_to_index(&self,
                              opts: Option<&mut DiffOptions>)
                              -> Result<Diff, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn diff_index_to_index(&self,
                               opts: Option<&mut DiffOptions>)
                               -> Result<Diff, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn diff_index_to_workdir(&self,
                                 opts: Option<&mut DiffOptions>)
                                 -> Result<Diff, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn diff_tree_to_workdir(&self,
                                opts: Option<&mut DiffOptions>)
                                -> Result<Diff, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn diff_tree_to_workdir_with_index(&self,
                                           opts: Option<&mut DiffOptions>)
                                           -> Result<Diff, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn stash_save(&mut self,
                      flags: Option<StashFlags>)
                      -> Result<Oid, Error> {
        unsafe {
            let mut raw_oid = raw::git_oid { id: [0; raw::GIT_OID_RAWSZ] };
            Ok(Binding::from_raw(&raw_oid as *const _))
        }
    }
    pub fn stash_apply(&mut self,
                       opts: Option<&mut StashApplyOptions>)
                       -> Result<(), Error> {
        unsafe {
            Ok(())
        }
    }
    pub fn stash_pop(&mut self,
                     opts: Option<&mut StashApplyOptions>)
                     -> Result<(), Error> {
        unsafe {
            Ok(())
        }
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
