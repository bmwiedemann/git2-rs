use std::ffi::CString;
use std::ops::Range;
use std::marker;
use std::mem;
use std::str;
use libc::{c_char, size_t, c_uint};
use {raw, Status, DiffDelta, IntoCString, Repository};
use util::Binding;
pub struct StatusOptions {
    raw: raw::git_status_options,
    pathspec: Vec<CString>,
    ptrs: Vec<*const c_char>,
}
#[derive(Copy, Clone)]
pub enum StatusShow {
    Index,
    Workdir,
    IndexAndWorkdir,
}
pub struct Statuses<'repo> {
    raw: *mut raw::git_status_list,
    _marker: marker::PhantomData<&'repo Repository>,
}
pub struct StatusIter<'statuses> {
    statuses: &'statuses Statuses<'statuses>,
    range: Range<usize>,
}
pub struct StatusEntry<'statuses> {
    raw: *const raw::git_status_entry,
    _marker: marker::PhantomData<&'statuses DiffDelta<'statuses>>,
}
impl Default for StatusOptions {
    fn default() -> Self {
        Self::new()
    }
}
impl StatusOptions {
    pub fn new() -> StatusOptions {
        unsafe {
            let mut raw = mem::zeroed();
            let r = raw::git_status_init_options(&mut raw,
                                raw::GIT_STATUS_OPTIONS_VERSION);
            assert_eq!(r, 0);
            StatusOptions {
                raw: raw,
                pathspec: Vec::new(),
                ptrs: Vec::new(),
            }
        }
    }
    pub fn show(&mut self, show: StatusShow) -> &mut StatusOptions {
        self.raw.show = match show {
            StatusShow::Index => raw::GIT_STATUS_SHOW_INDEX_ONLY,
            StatusShow::Workdir => raw::GIT_STATUS_SHOW_WORKDIR_ONLY,
            StatusShow::IndexAndWorkdir => raw::GIT_STATUS_SHOW_INDEX_AND_WORKDIR,
        };
        self
    }
    pub fn pathspec<T: IntoCString>(&mut self, pathspec: T)
                                    -> &mut StatusOptions {
        let s = pathspec.into_c_string().unwrap();
        self.ptrs.push(s.as_ptr());
        self.pathspec.push(s);
        self
    }
    fn flag(&mut self, flag: raw::git_status_opt_t, val: bool)
            -> &mut StatusOptions {
        if val {
            self.raw.flags |= flag as c_uint;
        } else {
            self.raw.flags &= !(flag as c_uint);
        }
        self
    }
    pub fn include_untracked(&mut self, include: bool) -> &mut StatusOptions {
        self.flag(raw::GIT_STATUS_OPT_INCLUDE_UNTRACKED, include)
    }
    pub fn include_ignored(&mut self, include: bool) -> &mut StatusOptions {
        self.flag(raw::GIT_STATUS_OPT_INCLUDE_IGNORED, include)
    }
    pub fn include_unmodified(&mut self, include: bool) -> &mut StatusOptions {
        self.flag(raw::GIT_STATUS_OPT_INCLUDE_UNMODIFIED, include)
    }
    pub fn exclude_submodules(&mut self, exclude: bool) -> &mut StatusOptions {
        self.flag(raw::GIT_STATUS_OPT_EXCLUDE_SUBMODULES, exclude)
    }
    pub fn recurse_untracked_dirs(&mut self, include: bool)
                                  -> &mut StatusOptions {
        self.flag(raw::GIT_STATUS_OPT_RECURSE_UNTRACKED_DIRS, include)
    }
    pub fn disable_pathspec_match(&mut self, include: bool)
                                  -> &mut StatusOptions {
        self.flag(raw::GIT_STATUS_OPT_DISABLE_PATHSPEC_MATCH, include)
    }
    pub fn recurse_ignored_dirs(&mut self, include: bool)
                                -> &mut StatusOptions {
        self.flag(raw::GIT_STATUS_OPT_RECURSE_IGNORED_DIRS, include)
    }
    pub fn renames_head_to_index(&mut self, include: bool)
                                 -> &mut StatusOptions {
        self.flag(raw::GIT_STATUS_OPT_RENAMES_HEAD_TO_INDEX, include)
    }
    pub fn renames_index_to_workdir(&mut self, include: bool)
                                    -> &mut StatusOptions {
        self.flag(raw::GIT_STATUS_OPT_RENAMES_INDEX_TO_WORKDIR, include)
    }
    pub fn sort_case_sensitively(&mut self, include: bool)
                                 -> &mut StatusOptions {
        self.flag(raw::GIT_STATUS_OPT_SORT_CASE_SENSITIVELY, include)
    }
    pub fn sort_case_insensitively(&mut self, include: bool)
                                   -> &mut StatusOptions {
        self.flag(raw::GIT_STATUS_OPT_SORT_CASE_INSENSITIVELY, include)
    }
    pub fn renames_from_rewrites(&mut self, include: bool)
                                 -> &mut StatusOptions {
        self.flag(raw::GIT_STATUS_OPT_RENAMES_FROM_REWRITES, include)
    }
    pub fn no_refresh(&mut self, include: bool) -> &mut StatusOptions {
        self.flag(raw::GIT_STATUS_OPT_NO_REFRESH, include)
    }
    pub fn update_index(&mut self, include: bool) -> &mut StatusOptions {
        self.flag(raw::GIT_STATUS_OPT_UPDATE_INDEX, include)
    }
    #[allow(missing_docs)]
    pub fn include_unreadable(&mut self, include: bool) -> &mut StatusOptions {
        self.flag(raw::GIT_STATUS_OPT_INCLUDE_UNREADABLE, include)
    }
    #[allow(missing_docs)]
    pub fn include_unreadable_as_untracked(&mut self, include: bool)
                                           -> &mut StatusOptions {
        self.flag(raw::GIT_STATUS_OPT_INCLUDE_UNREADABLE_AS_UNTRACKED, include)
    }
    pub unsafe fn raw(&mut self) -> *const raw::git_status_options {
        self.raw.pathspec.strings = self.ptrs.as_ptr() as *mut _;
        self.raw.pathspec.count = self.ptrs.len() as size_t;
        &self.raw
    }
}
impl<'repo> Statuses<'repo> {
    pub fn get(&self, index: usize) -> Option<StatusEntry> {
        unsafe {
            let p = raw::git_status_byindex(self.raw, index as size_t);
            Binding::from_raw_opt(p)
        }
    }
    pub fn len(&self) -> usize {
        unsafe { raw::git_status_list_entrycount(self.raw) as usize }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn iter(&self) -> StatusIter {
        StatusIter {
            statuses: self,
            range: 0..self.len(),
        }
    }
}
impl<'repo> Binding for Statuses<'repo> {
    type Raw = *mut raw::git_status_list;
    unsafe fn from_raw(raw: *mut raw::git_status_list) -> Statuses<'repo> {
        Statuses { raw: raw, _marker: marker::PhantomData }
    }
    fn raw(&self) -> *mut raw::git_status_list { self.raw }
}
impl<'repo> Drop for Statuses<'repo> {
    fn drop(&mut self) {
        unsafe { raw::git_status_list_free(self.raw); }
    }
}
impl<'a> Iterator for StatusIter<'a> {
    type Item = StatusEntry<'a>;
    fn next(&mut self) -> Option<StatusEntry<'a>> {
        self.range.next().and_then(|i| self.statuses.get(i))
    }
    fn size_hint(&self) -> (usize, Option<usize>) { self.range.size_hint() }
}
impl<'a> DoubleEndedIterator for StatusIter<'a> {
    fn next_back(&mut self) -> Option<StatusEntry<'a>> {
        self.range.next_back().and_then(|i| self.statuses.get(i))
    }
}
impl<'a> ExactSizeIterator for StatusIter<'a> {}
impl<'statuses> StatusEntry<'statuses> {
    pub fn path_bytes(&self) -> &[u8] {
        unsafe {
            if (*self.raw).head_to_index.is_null() {
                ::opt_bytes(self, (*(*self.raw).index_to_workdir).old_file.path)
            } else {
                ::opt_bytes(self, (*(*self.raw).head_to_index).old_file.path)
            }.unwrap()
        }
    }
    pub fn path(&self) -> Option<&str> { str::from_utf8(self.path_bytes()).ok() }
    pub fn status(&self) -> Status {
        Status::from_bits_truncate(unsafe { (*self.raw).status as u32 })
    }
    pub fn head_to_index(&self) -> Option<DiffDelta<'statuses>> {
        unsafe {
            Binding::from_raw_opt((*self.raw).head_to_index)
        }
    }
    pub fn index_to_workdir(&self) -> Option<DiffDelta<'statuses>> {
        unsafe {
            Binding::from_raw_opt((*self.raw).index_to_workdir)
        }
    }
}
impl<'statuses> Binding for StatusEntry<'statuses> {
    type Raw = *const raw::git_status_entry;
    unsafe fn from_raw(raw: *const raw::git_status_entry)
                           -> StatusEntry<'statuses> {
        StatusEntry { raw: raw, _marker: marker::PhantomData }
    }
    fn raw(&self) -> *const raw::git_status_entry { self.raw }
}
