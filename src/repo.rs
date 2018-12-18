use std::env;
use std::ffi::{CStr, CString, OsStr};
use std::mem;
use std::path::Path;
use std::ptr;
use std::str;
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
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Repository, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn open_bare<P: AsRef<Path>>(path: P) -> Result<Repository, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn open_from_env() -> Result<Repository, Error> {
        let mut ret = ptr::null_mut();
        let flags = raw::GIT_REPOSITORY_OPEN_FROM_ENV;
        unsafe {
            try_call!(raw::git_repository_open_ext(&mut ret,
                                                   ptr::null(),
                                                   flags as c_uint,
                                                   ptr::null()));
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn open_ext<P, O, I>(path: P,
                             flags: RepositoryOpenFlags,
                             ceiling_dirs: I)
                             -> Result<Repository, Error>
            where P: AsRef<Path>, O: AsRef<OsStr>, I: IntoIterator<Item=O>
    {
        let path = try!(path.as_ref().into_c_string());
        let ceiling_dirs_os = try!(env::join_paths(ceiling_dirs));
        let ceiling_dirs = try!(ceiling_dirs_os.into_c_string());
        let mut ret = ptr::null_mut();
        unsafe {
            try_call!(raw::git_repository_open_ext(&mut ret,
                                                   path,
                                                   flags.bits() as c_uint,
                                                   ceiling_dirs));
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn discover<P: AsRef<Path>>(path: P) -> Result<Repository, Error> {
        let buf = Buf::new();
        Repository::open(util::bytes2path(&*buf))
    }
    pub fn init<P: AsRef<Path>>(path: P) -> Result<Repository, Error> {
        Repository::init_opts(path, &RepositoryInitOptions::new())
    }
    pub fn init_bare<P: AsRef<Path>>(path: P) -> Result<Repository, Error> {
        Repository::init_opts(path, RepositoryInitOptions::new().bare(true))
    }
    pub fn init_opts<P: AsRef<Path>>(path: P, opts: &RepositoryInitOptions)
                     -> Result<Repository, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn clone<P: AsRef<Path>>(url: &str, into: P)
                                 -> Result<Repository, Error> {
        RepoBuilder::new().clone(url, into.as_ref())
    }
    pub fn clone_recurse<P: AsRef<Path>>(url: &str, into: P)
                                         -> Result<Repository, Error> {
        let repo = Repository::clone(url, into)?;
        Ok(repo)
    }
    pub fn from_odb(odb: Odb) -> Result<Repository, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(ret))
        }
    }
    fn update_submodules(&self) -> Result<(), Error> {
        fn add_subrepos(repo: &Repository, list: &mut Vec<Repository>)
                        -> Result<(), Error> {
            Ok(())
        }
        let mut repos = Vec::new();
        while let Some(repo) = repos.pop() {
            add_subrepos(&repo, &mut repos)?;
        }
        Ok(())
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
    pub fn revparse_single(&self, spec: &str) -> Result<Object, Error> {
        let mut obj = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(obj))
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
    pub fn is_bare(&self) -> bool {
        unsafe { raw::git_repository_is_bare(self.raw) == 1 }
    }
    pub fn is_shallow(&self) -> bool {
        unsafe { raw::git_repository_is_shallow(self.raw) == 1 }
    }
    pub fn is_worktree(&self) -> bool {
        unsafe { raw::git_repository_is_worktree(self.raw) == 1 }
    }
    pub fn is_empty(&self) -> Result<bool, Error> {
        let empty = unsafe {
            try_call!(raw::git_repository_is_empty(self.raw))
        };
        Ok(empty == 1)
    }
    pub fn path(&self) -> &Path {
        unsafe {
            let ptr = raw::git_repository_path(self.raw);
            util::bytes2path(::opt_bytes(self, ptr).unwrap())
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
        let path = try!(path.into_c_string());
        unsafe {
            try_call!(raw::git_repository_set_workdir(self.raw(), path,
                                                      update_gitlink));
        }
        Ok(())
    }
    pub fn namespace(&self) -> Option<&str> {
        self.namespace_bytes().and_then(|s| str::from_utf8(s).ok())
    }
    pub fn namespace_bytes(&self) -> Option<&[u8]> {
        unsafe { ::opt_bytes(self, raw::git_repository_get_namespace(self.raw)) }
    }
    pub fn set_namespace(&self, namespace: &str) -> Result<(), Error> {
        self.set_namespace_bytes(namespace.as_bytes())
    }
    pub fn set_namespace_bytes(&self, namespace: &[u8]) -> Result<(), Error> {
        unsafe {
            Ok(())
        }
    }
    pub fn remove_namespace(&self) -> Result<(), Error> {
        unsafe {
            Ok(())
        }
    }
    pub fn message(&self) -> Result<String, Error> {
        unsafe {
            let buf = Buf::new();
            Ok(str::from_utf8(&buf).unwrap().to_string())
        }
    }
    pub fn remove_message(&self) -> Result<(), Error> {
        unsafe {
            Ok(())
        }
    }
    pub fn remotes(&self) -> Result<StringArray, Error> {
        let mut arr = raw::git_strarray {
            strings: 0 as *mut *mut c_char,
            count: 0,
        };
        unsafe {
            Ok(Binding::from_raw(arr))
        }
    }
    pub fn find_remote(&self, name: &str) -> Result<Remote, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn remote(&self, name: &str, url: &str) -> Result<Remote, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn remote_anonymous(&self, url: &str) -> Result<Remote, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn remote_rename(&self, name: &str,
                         new_name: &str) -> Result<StringArray, Error> {
        let mut problems = raw::git_strarray {
            count: 0,
            strings: 0 as *mut *mut c_char,
        };
        unsafe {
            Ok(Binding::from_raw(problems))
        }
    }
    pub fn remote_delete(&self, name: &str) -> Result<(), Error> {
        Ok(())
    }
    pub fn remote_add_fetch(&self, name: &str, spec: &str)
                            -> Result<(), Error> {
        Ok(())
    }
    pub fn remote_add_push(&self, name: &str, spec: &str)
                           -> Result<(), Error> {
        Ok(())
    }
    pub fn remote_set_url(&self, name: &str, url: &str) -> Result<(), Error> {
        Ok(())
    }
    pub fn remote_set_pushurl(&self, name: &str, pushurl: Option<&str>)
                              -> Result<(), Error> {
        Ok(())
    }
    pub fn reset(&self,
                 checkout: Option<&mut CheckoutBuilder>)
                 -> Result<(), Error> {
        Ok(())
    }
    pub fn reset_default<T, I>(&self,
                               paths: I) -> Result<(), Error>
    {
        Ok(())
    }
    pub fn head(&self) -> Result<Reference, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn set_head(&self, refname: &str) -> Result<(), Error> {
        Ok(())
    }
    pub fn head_detached(&self) -> Result<bool, Error> {
        unsafe {
            let value = raw::git_repository_head_detached(self.raw);
            match value {
                _ => Err(Error::last_error(value).unwrap())
            }
        }
    }
    pub fn set_head_detached(&self, commitish: Oid) -> Result<(), Error> {
        Ok(())
    }
    pub fn references(&self) -> Result<References, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn references_glob(&self, glob: &str) -> Result<References, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn submodules(&self) -> Result<Vec<Submodule>, Error> {
        struct Data<'a, 'b:'a> {
            repo: &'b Repository,
            ret: &'a mut Vec<Submodule<'b>>,
        }
        let mut ret = Vec::new();
        unsafe {
            let mut data = Data {
                repo: self,
                ret: &mut ret,
            };
            try_call!(raw::git_submodule_foreach(self.raw, append,
                                                 &mut data as *mut _
                                                           as *mut c_void));
        }
        return Ok(ret);
        extern fn append(_repo: *mut raw::git_submodule,
                         name: *const c_char,
                         data: *mut c_void) -> c_int {
            unsafe {
                let data = &mut *(data as *mut Data);
                let mut raw = ptr::null_mut();
                let rc = raw::git_submodule_lookup(&mut raw, data.repo.raw(),
                                                   name);
            }
            0
        }
    }
    pub fn statuses(&self, options: Option<&mut StatusOptions>)
                    -> Result<Statuses, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            try_call!(raw::git_status_list_new(&mut ret, self.raw,
                                               options.map(|s| s.raw())
                                                      .unwrap_or(ptr::null())));
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn status_should_ignore(&self, path: &Path) -> Result<bool, Error> {
        let mut ret = 0 as c_int;
        Ok(ret != 0)
    }
    pub fn status_file(&self, path: &Path) -> Result<Status, Error> {
        let mut ret = 0 as c_uint;
        let path = if cfg!(windows) {
            try!(::std::ffi::CString::new(path.to_string_lossy().replace('\\', "/")))
        } else {
            try!(path.into_c_string())
        };
        Ok(Status::from_bits_truncate(ret as u32))
    }
    pub fn branches(&self, filter: Option<BranchType>)
                    -> Result<Branches, Error> {
        let mut raw = ptr::null_mut();
        unsafe {
            Ok(Branches::from_raw(raw))
        }
    }
    pub fn index(&self) -> Result<Index, Error> {
        let mut raw = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(raw))
        }
    }
    pub fn config(&self) -> Result<Config, Error> {
        let mut raw = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(raw))
        }
    }
    pub fn blob(&self, data: &[u8]) -> Result<Oid, Error> {
        let mut raw = raw::git_oid { id: [0; raw::GIT_OID_RAWSZ] };
        unsafe {
            Ok(Binding::from_raw(&raw as *const _))
        }
    }
    pub fn blob_path(&self, path: &Path) -> Result<Oid, Error> {
        let mut raw = raw::git_oid { id: [0; raw::GIT_OID_RAWSZ] };
        unsafe {
            Ok(Binding::from_raw(&raw as *const _))
        }
    }
    pub fn blob_writer(&self, hintpath: Option<&Path>) -> Result<BlobWriter, Error> {
        let path_str = match hintpath {
            Some(path) => Some(try!(path.into_c_string())),
            None => None,
        };
        let path = match path_str {
            Some(ref path) => path.as_ptr(),
            None => ptr::null(),
        };
        let mut out = ptr::null_mut();
        unsafe {
            Ok(BlobWriter::from_raw(out))
        }
    }
    pub fn find_blob(&self, oid: Oid) -> Result<Blob, Error> {
        let mut raw = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(raw))
        }
    }
    pub fn odb(&self) -> Result<Odb, Error> {
        let mut odb = ptr::null_mut();
        unsafe {
            Ok(Odb::from_raw(odb))
        }
    }
    pub fn branch(&self,
                  branch_name: &str,
                  target: &Commit,
                  force: bool) -> Result<Branch, Error> {
        let branch_name = try!(CString::new(branch_name));
        let mut raw = ptr::null_mut();
        unsafe {
            try_call!(raw::git_branch_create(&mut raw,
                                             self.raw(),
                                             branch_name,
                                             target.raw(),
                                             force));
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
                         commit_content: &str,
                         signature: &str,
                         signature_field: Option<&str>) -> Result<Oid, Error> {
        let commit_content = try!(CString::new(commit_content));
        let signature = try!(CString::new(signature));
        let signature_field = try!(::opt_cstr(signature_field));
        let mut raw = raw::git_oid { id: [0; raw::GIT_OID_RAWSZ] };
        unsafe {
            try_call!(raw::git_commit_create_with_signature(&mut raw,
                                                            self.raw(),
                                                            commit_content,
                                                            signature,
                                                            signature_field));
            Ok(Binding::from_raw(&raw as *const _))
        }
    }
    pub fn extract_signature(&self,
                             commit_id: &Oid,
                             signature_field: Option<&str>)
                             -> Result<(Buf, Buf), Error> {
        let signature_field = try!(::opt_cstr(signature_field));
        let signature = Buf::new();
        let content = Buf::new();
        unsafe {
            try_call!(raw::git_commit_extract_signature(signature.raw(),
                                                        content.raw(),
                                                        self.raw(),
                                                        commit_id.raw() as *mut _,
                                                        signature_field));
            Ok((signature, content))
        }
    }
    pub fn find_commit(&self, oid: Oid) -> Result<Commit, Error> {
        let mut raw = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(raw))
        }
    }
    pub fn find_annotated_commit(&self, id: Oid) -> Result<AnnotatedCommit, Error> {
        unsafe {
            let mut raw = 0 as *mut raw::git_annotated_commit;
            Ok(Binding::from_raw(raw))
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
        let name = try!(CString::new(name));
        let log_message = try!(CString::new(log_message));
        let mut raw = ptr::null_mut();
        unsafe {
            try_call!(raw::git_reference_create(&mut raw, self.raw(), name,
                                                id.raw(), force,
                                                log_message));
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
                              force: bool,
                              log_message: &str)
                              -> Result<Reference, Error> {
        let name = try!(CString::new(name));
        let target = try!(CString::new(target));
        let log_message = try!(CString::new(log_message));
        let mut raw = ptr::null_mut();
        unsafe {
            try_call!(raw::git_reference_symbolic_create(&mut raw, self.raw(),
                                                         name, target, force,
                                                         log_message));
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
    pub fn find_reference(&self, name: &str) -> Result<Reference, Error> {
        let mut raw = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(raw))
        }
    }
    pub fn refname_to_id(&self, name: &str) -> Result<Oid, Error> {
        let mut ret = raw::git_oid { id: [0; raw::GIT_OID_RAWSZ] };
        unsafe {
            Ok(Binding::from_raw(&ret as *const _))
        }
    }
    pub fn reference_to_annotated_commit(&self, reference: &Reference)
                                         -> Result<AnnotatedCommit, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            try_call!(raw::git_annotated_commit_from_ref(&mut ret,
                                                         self.raw(),
                                                         reference.raw()));
            Ok(AnnotatedCommit::from_raw(ret))
        }
    }
    pub fn signature(&self) -> Result<Signature<'static>, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn submodule(&self, url: &str, path: &Path,
                     use_gitlink: bool) -> Result<Submodule, Error> {
        let mut raw = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(raw))
        }
    }
    pub fn find_submodule(&self, name: &str) -> Result<Submodule, Error> {
        let mut raw = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(raw))
        }
    }
    pub fn submodule_status(&self, name: &str, ignore: SubmoduleIgnore)
                            -> Result<SubmoduleStatus, Error> {
        let mut ret = 0;
        Ok(SubmoduleStatus::from_bits_truncate(ret as u32))
    }
    pub fn find_tree(&self, oid: Oid) -> Result<Tree, Error> {
        let mut raw = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(raw))
        }
    }
    pub fn treebuilder(&self, tree: Option<&Tree>) -> Result<TreeBuilder, Error> {
        unsafe {
            let mut ret = ptr::null_mut();
            let tree = match tree {
                Some(tree) => tree.raw(),
                None => ptr::null_mut(),
            };
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn tag(&self, name: &str, target: &Object,
               tagger: &Signature, message: &str,
               force: bool) -> Result<Oid, Error> {
        let name = try!(CString::new(name));
        let message = try!(CString::new(message));
        let mut raw = raw::git_oid { id: [0; raw::GIT_OID_RAWSZ] };
        unsafe {
            try_call!(raw::git_tag_create(&mut raw, self.raw, name,
                                          target.raw(), tagger.raw(),
                                          message, force));
            Ok(Binding::from_raw(&raw as *const _))
        }
    }
    pub fn tag_lightweight(&self,
                           force: bool) -> Result<Oid, Error> {
        let mut raw = raw::git_oid { id: [0; raw::GIT_OID_RAWSZ] };
        unsafe {
            Ok(Binding::from_raw(&raw as *const _))
        }
    }
    pub fn find_tag(&self, id: Oid) -> Result<Tag, Error> {
        let mut raw = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(raw))
        }
    }
    pub fn tag_delete(&self, name: &str) -> Result<(), Error> {
        unsafe {
            Ok(())
        }
    }
    pub fn tag_names(&self, pattern: Option<&str>) -> Result<StringArray, Error> {
        let mut arr = raw::git_strarray {
            strings: 0 as *mut *mut c_char,
            count: 0,
        };
        unsafe {
            match pattern {
                Some(s) => {
                }
                None => { try_call!(raw::git_tag_list(&mut arr, self.raw)); }
            }
            Ok(Binding::from_raw(arr))
        }
    }
    pub fn checkout_head(&self, opts: Option<&mut CheckoutBuilder>)
                         -> Result<(), Error> {
        unsafe {
            let mut raw_opts = mem::zeroed();
            try_call!(raw::git_checkout_init_options(&mut raw_opts,
                                raw::GIT_CHECKOUT_OPTIONS_VERSION));
        }
        Ok(())
    }
    pub fn checkout_index(&self,
                          index: Option<&mut Index>,
                          opts: Option<&mut CheckoutBuilder>) -> Result<(), Error> {
        unsafe {
            let mut raw_opts = mem::zeroed();
            try_call!(raw::git_checkout_index(self.raw,
                                              index.map(|i| &mut *i.raw()),
                                              &raw_opts));
        }
        Ok(())
    }
    pub fn checkout_tree(&self,
                         opts: Option<&mut CheckoutBuilder>) -> Result<(), Error> {
        unsafe {
            let mut raw_opts = mem::zeroed();
            if let Some(c) = opts {
                c.configure(&mut raw_opts);
            }
        }
        Ok(())
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
            try_call!(raw::git_merge_commits(&mut raw, self.raw,
                                             our_commit.raw(),
                                             their_commit.raw(),
                                             opts.map(|o| o.raw())));
            Ok(Binding::from_raw(raw))
        }
    }
    pub fn merge_trees(&self, ancestor_tree: &Tree, our_tree: &Tree,
                       their_tree: &Tree, opts: Option<&MergeOptions>) -> Result<Index, Error> {
        let mut raw = ptr::null_mut();
        unsafe {
            try_call!(raw::git_merge_trees(&mut raw, self.raw, ancestor_tree.raw(),
                                           our_tree.raw(), their_tree.raw(),
                                           opts.map(|o| o.raw())));
            Ok(Binding::from_raw(raw))
        }
    }
    pub fn cleanup_state(&self) -> Result<(), Error> {
        Ok(())
    }
    pub fn merge_analysis(&self,
                          their_heads: &[&AnnotatedCommit])
                          -> Result<(MergeAnalysis, MergePreference), Error> {
        unsafe {
            let mut raw_merge_analysis = 0 as raw::git_merge_analysis_t;
            let mut raw_merge_preference = 0 as raw::git_merge_preference_t;
            let mut their_heads = their_heads
                .iter()
                .collect::<Vec<_>>();
            try_call!(raw::git_merge_analysis(&mut raw_merge_analysis,
                                              &mut raw_merge_preference,
                                              self.raw,
                                              their_heads.as_mut_ptr() as *mut _,
                                              their_heads.len()));
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
    pub fn open_rebase(&self, opts: Option<&mut RebaseOptions>) -> Result<Rebase, Error> {
        let mut rebase: *mut raw::git_rebase = ptr::null_mut();
        unsafe {
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
    pub fn note_default_ref(&self) -> Result<String, Error> {
        let ret = Buf::new();
        Ok(str::from_utf8(&ret).unwrap().to_string())
    }
    pub fn notes(&self, notes_ref: Option<&str>) -> Result<Notes, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn find_note(&self, notes_ref: Option<&str>, id: Oid)
                     -> Result<Note, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn note_delete(&self,
                       committer: &Signature) -> Result<(), Error> {
        unsafe {
            Ok(())
        }
    }
    pub fn revwalk(&self) -> Result<Revwalk, Error> {
        let mut raw = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(raw))
        }
    }
    pub fn blame_file(&self, path: &Path, opts: Option<&mut BlameOptions>)
                      -> Result<Blame, Error> {
        let path = try!(path.into_c_string());
        let mut raw = ptr::null_mut();
        unsafe {
            try_call!(raw::git_blame_file(&mut raw,
                                          self.raw(),
                                          path,
                                          opts.map(|s| s.raw())));
            Ok(Binding::from_raw(raw))
        }
    }
    pub fn merge_base(&self, one: Oid, two: Oid) -> Result<Oid, Error> {
        let mut raw = raw::git_oid { id: [0; raw::GIT_OID_RAWSZ] };
        unsafe {
            Ok(Binding::from_raw(&raw as *const _))
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
            try_call!(raw::git_graph_ahead_behind(&mut ahead, &mut behind,
                                                  self.raw(), local.raw(),
                                                  upstream.raw()));
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
    pub fn reflog(&self, name: &str) -> Result<Reflog, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn reflog_delete(&self, name: &str) -> Result<(), Error> {
        Ok(())
    }
    pub fn reflog_rename(&self, old_name: &str, new_name: &str)
                         -> Result<(), Error> {
        Ok(())
    }
    pub fn reference_has_log(&self, name: &str) -> Result<bool, Error> {
        let name = try!(CString::new(name));
        let ret = unsafe {
            try_call!(raw::git_reference_has_log(self.raw, name))
        };
        Ok(ret != 0)
    }
    pub fn reference_ensure_log(&self, name: &str) -> Result<(), Error> {
        Ok(())
    }
    pub fn describe(&self, opts: &DescribeOptions) -> Result<Describe, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn diff_tree_to_tree(&self,
                             old_tree: Option<&Tree>,
                             new_tree: Option<&Tree>,
                             opts: Option<&mut DiffOptions>)
                             -> Result<Diff, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            try_call!(raw::git_diff_tree_to_tree(&mut ret,
                                                 self.raw(),
                                                 old_tree.map(|s| s.raw()),
                                                 new_tree.map(|s| s.raw()),
                                                 opts.map(|s| s.raw())));
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn diff_tree_to_index(&self,
                              old_tree: Option<&Tree>,
                              index: Option<&Index>,
                              opts: Option<&mut DiffOptions>)
                              -> Result<Diff, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            try_call!(raw::git_diff_tree_to_index(&mut ret,
                                                  self.raw(),
                                                  old_tree.map(|s| s.raw()),
                                                  index.map(|s| s.raw()),
                                                  opts.map(|s| s.raw())));
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn diff_index_to_index(&self,
                               old_index: &Index,
                               new_index: &Index,
                               opts: Option<&mut DiffOptions>)
                               -> Result<Diff, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            try_call!(raw::git_diff_index_to_index(&mut ret,
                                                   self.raw(),
                                                   old_index.raw(),
                                                   new_index.raw(),
                                                   opts.map(|s| s.raw())));
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn diff_index_to_workdir(&self,
                                 index: Option<&Index>,
                                 opts: Option<&mut DiffOptions>)
                                 -> Result<Diff, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            try_call!(raw::git_diff_index_to_workdir(&mut ret,
                                                     self.raw(),
                                                     index.map(|s| s.raw()),
                                                     opts.map(|s| s.raw())));
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn diff_tree_to_workdir(&self,
                                old_tree: Option<&Tree>,
                                opts: Option<&mut DiffOptions>)
                                -> Result<Diff, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            try_call!(raw::git_diff_tree_to_workdir(&mut ret,
                                                    self.raw(),
                                                    old_tree.map(|s| s.raw()),
                                                    opts.map(|s| s.raw())));
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
    pub fn packbuilder(&self) -> Result<PackBuilder, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn stash_save(&mut self,
                      stasher: &Signature,
                      message: &str,
                      flags: Option<StashFlags>)
                      -> Result<Oid, Error> {
        unsafe {
            let mut raw_oid = raw::git_oid { id: [0; raw::GIT_OID_RAWSZ] };
            let message = try!(CString::new(message));
            let flags = flags.unwrap_or_else(StashFlags::empty);
            try_call!(raw::git_stash_save(&mut raw_oid,
                                          self.raw(),
                                          stasher.raw(),
                                          message,
                                          flags.bits() as c_uint));
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
    pub fn stash_foreach<C>(&mut self, mut callback: C) -> Result<(), Error>
        where C: FnMut(usize, &str, &Oid) -> bool
    {
        unsafe {
            let mut data = StashCbData { callback: &mut callback };
            try_call!(raw::git_stash_foreach(self.raw(),
                                             stash_cb,
                                             &mut data as *mut _ as *mut _));
            Ok(())
        }
    }
    pub fn stash_drop(&mut self, index: usize) -> Result<(), Error> {
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
    pub fn add_ignore_rule(&self, rules: &str) -> Result<(), Error> {
        unsafe {
        }
        Ok(())
    }
    pub fn clear_ignore_rules(&self) -> Result<(), Error> {
        Ok(())
    }
    pub fn is_path_ignored<P: AsRef<Path>>(&self, path: P) -> Result<bool, Error> {
        let path = if cfg!(windows) {
            try!(::std::ffi::CString::new(path.as_ref().to_string_lossy().replace('\\', "/")))
        } else {
            try!(path.as_ref().into_c_string())
        };
        let mut ignored: c_int = 0;
        Ok(ignored == 1)
    }
}
impl Binding for Repository {
    type Raw = *mut raw::git_repository;
    unsafe fn from_raw(ptr: *mut raw::git_repository) -> Repository {
        Repository { raw: ptr }
    }
    fn raw(&self) -> *mut raw::git_repository { self.raw }
}
impl Drop for Repository {
    fn drop(&mut self) {
    }
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
    pub fn bare(&mut self, bare: bool) -> &mut RepositoryInitOptions {
        self.flag(raw::GIT_REPOSITORY_INIT_BARE, bare)
    }
    pub fn no_reinit(&mut self, enabled: bool) -> &mut RepositoryInitOptions {
        self.flag(raw::GIT_REPOSITORY_INIT_NO_REINIT, enabled)
    }
    pub fn no_dotgit_dir(&mut self, enabled: bool) -> &mut RepositoryInitOptions {
        self.flag(raw::GIT_REPOSITORY_INIT_NO_DOTGIT_DIR, enabled)
    }
    pub fn mkdir(&mut self, enabled: bool) -> &mut RepositoryInitOptions {
        self.flag(raw::GIT_REPOSITORY_INIT_MKDIR, enabled)
    }
    pub fn mkpath(&mut self, enabled: bool) -> &mut RepositoryInitOptions {
        self.flag(raw::GIT_REPOSITORY_INIT_MKPATH, enabled)
    }
    pub fn mode(&mut self, mode: RepositoryInitMode)
                -> &mut RepositoryInitOptions {
        self
    }
    pub fn external_template(&mut self, enabled: bool)
                             -> &mut RepositoryInitOptions {
        self.flag(raw::GIT_REPOSITORY_INIT_EXTERNAL_TEMPLATE, enabled)
    }
    fn flag(&mut self, flag: raw::git_repository_init_flag_t, on: bool)
            -> &mut RepositoryInitOptions {
        self
    }
    pub fn workdir_path(&mut self, path: &Path) -> &mut RepositoryInitOptions {
        self
    }
    pub fn description(&mut self, desc: &str) -> &mut RepositoryInitOptions {
        self
    }
    pub fn template_path(&mut self, path: &Path) -> &mut RepositoryInitOptions {
        self
    }
    pub fn initial_head(&mut self, head: &str) -> &mut RepositoryInitOptions {
        self
    }
    pub fn origin_url(&mut self, url: &str) -> &mut RepositoryInitOptions {
        self
    }
    pub unsafe fn raw(&self) -> raw::git_repository_init_options {
        let mut opts = mem::zeroed();
        opts
    }
}
