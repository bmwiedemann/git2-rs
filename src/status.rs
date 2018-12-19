use std::ffi::CString;
use std::ops::Range;
use std::marker;
use std::mem;
use libc::{c_char, size_t, c_uint};
use {raw, Status, DiffDelta, IntoCString, Repository};
use util::Binding;
pub struct StatusOptions {
    raw: raw::git_status_options,
    pathspec: Vec<CString>,
    ptrs: Vec<*const c_char>,
}
pub enum StatusShow {
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
        unsafe {
            let mut raw = mem::zeroed();
            StatusOptions {
                raw: raw,
                pathspec: Vec::new(),
                ptrs: Vec::new(),
            }
        }
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
impl<'a> Iterator for StatusIter<'a> {
    type Item = StatusEntry<'a>;
    fn next(&mut self) -> Option<StatusEntry<'a>> {
        self.range.next().and_then(|i| self.statuses.get(i))
    }
}
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
}
impl<'statuses> Binding for StatusEntry<'statuses> {
    type Raw = *const raw::git_status_entry;
    unsafe fn from_raw(raw: *const raw::git_status_entry)
                           -> StatusEntry<'statuses> {
        StatusEntry { raw: raw, _marker: marker::PhantomData }
    }
    fn raw(&self) -> *const raw::git_status_entry { self.raw }
}
