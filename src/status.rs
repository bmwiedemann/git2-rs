use std::ffi::CString;
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
impl<'repo> Binding for Statuses<'repo> {
    type Raw = *mut raw::git_status_list;
    unsafe fn from_raw(raw: *mut raw::git_status_list) -> Statuses<'repo> {
        Statuses { raw: raw, _marker: marker::PhantomData }
    }
    fn raw(&self) -> *mut raw::git_status_list { self.raw }
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
    unsafe fn from_raw(raw: *const raw::git_status_entry)
                           -> StatusEntry<'statuses> {
        StatusEntry { raw: raw, _marker: marker::PhantomData }
    }
}
