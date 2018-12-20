use std::ffi::{CStr, CString};
use std::path::Path;
use libc::{c_char, size_t, c_void, c_uint, c_int};
use {raw, panic, Error, Repository, FetchOptions, IntoCString};
use {CheckoutNotificationType, DiffFile, Remote};
use util::{self, Binding};
pub struct RepoBuilder<'cb> {
    bare: bool,
    branch: Option<CString>,
    local: bool,
    hardlinks: bool,
    checkout: Option<CheckoutBuilder<'cb>>,
    fetch_opts: Option<FetchOptions<'cb>>,
    clone_local: Option<CloneLocal>,
    remote_create: Option<Box<RemoteCreate<'cb>>>,
}
pub type RemoteCreate<'cb> = for<'a> FnMut(&'a Repository, &str, &str)
    -> Result<Remote<'a>, Error> + 'cb;
pub struct CheckoutBuilder<'cb> {
    their_label: Option<CString>,
    our_label: Option<CString>,
    ancestor_label: Option<CString>,
    target_dir: Option<CString>,
    paths: Vec<CString>,
    path_ptrs: Vec<*const c_char>,
    file_perm: Option<i32>,
    dir_perm: Option<i32>,
    disable_filters: bool,
    checkout_opts: u32,
    progress: Option<Box<Progress<'cb>>>,
    notify: Option<Box<Notify<'cb>>>,
    notify_flags: CheckoutNotificationType,
}
pub type Progress<'a> = FnMut(Option<&Path>, usize, usize) + 'a;
pub type Notify<'a> = FnMut(CheckoutNotificationType, Option<&Path>,
                            Option<DiffFile>, Option<DiffFile>,
                            Option<DiffFile>) -> bool + 'a;
pub enum CloneLocal {
}
impl<'cb> RepoBuilder<'cb> {
    pub fn new() -> RepoBuilder<'cb> {
        RepoBuilder {
            bare: false,
            branch: None,
            local: true,
            clone_local: None,
            hardlinks: true,
            checkout: None,
            fetch_opts: None,
            remote_create: None,
        }
    }
}
extern fn remote_create_cb(out: *mut *mut raw::git_remote,
                           repo: *mut raw::git_repository,
                           name: *const c_char,
                           url: *const c_char,
                           payload: *mut c_void) -> c_int {
    unsafe {
        let repo = Repository::from_raw(repo);
        let code = panic::wrap(|| {
            let name = CStr::from_ptr(name).to_str().unwrap();
            let url = CStr::from_ptr(url).to_str().unwrap();
            let f = payload as *mut Box<RemoteCreate>;
            match (*f)(&repo, name, url) {
                Ok(remote) => {
                    0
                }
                Err(e) => e.raw_code(),
            }
        });
        code.unwrap_or(-1)
    }
}
impl<'cb> CheckoutBuilder<'cb> {
    pub fn new() -> CheckoutBuilder<'cb> {
        CheckoutBuilder {
            disable_filters: false,
            dir_perm: None,
            file_perm: None,
            path_ptrs: Vec::new(),
            paths: Vec::new(),
            target_dir: None,
            ancestor_label: None,
            our_label: None,
            their_label: None,
            checkout_opts: raw::GIT_CHECKOUT_SAFE as u32,
            progress: None,
            notify: None,
            notify_flags: CheckoutNotificationType::empty(),
        }
    }
}
extern fn notify_cb(why: raw::git_checkout_notify_t,
                    path: *const c_char,
                    baseline: *const raw::git_diff_file,
                    target: *const raw::git_diff_file,
                    workdir: *const raw::git_diff_file,
                    data: *mut c_void) -> c_int {
    panic::wrap(|| unsafe {
        let payload = &mut *(data as *mut CheckoutBuilder);
        let callback = match payload.notify {
            Some(ref mut c) => c,
            None => return 0,
        };
        let path = if path.is_null() {
            None
        } else {
            Some(util::bytes2path(CStr::from_ptr(path).to_bytes()))
        };
        let baseline = if baseline.is_null() {
            None
        } else {
            Some(DiffFile::from_raw(baseline))
        };
        let target = if target.is_null() {
            None
        } else {
            Some(DiffFile::from_raw(target))
        };
        let workdir = if workdir.is_null() {
            None
        } else {
            Some(DiffFile::from_raw(workdir))
        };
        let why = CheckoutNotificationType::from_bits_truncate(why as u32);
        let keep_going = callback(why, path, baseline, target, workdir);
        if keep_going {0} else {1}
    }).unwrap_or(2)
}
