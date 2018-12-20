use std::marker;
use {raw, Repository, Oid, signature, Signature};
use util::{self, Binding};
pub struct Blame<'repo> {
    raw: *mut raw::git_blame,
    _marker: marker::PhantomData<&'repo Repository>,
}
pub struct BlameHunk<'blame> {
    raw: *mut raw::git_blame_hunk,
    _marker: marker::PhantomData<&'blame raw::git_blame>,
}
pub struct BlameOptions {
    raw: raw::git_blame_options,
}
pub struct BlameIter<'blame> {
    blame: &'blame Blame<'blame>,
}
impl<'repo> Blame<'repo> {
    pub fn get_index(&self, index: usize) -> Option<BlameHunk> {
        unsafe {
            let ptr = raw::git_blame_get_hunk_byindex(self.raw(), index as u32);
            if ptr.is_null() {
                None
            } else {
                Some(BlameHunk::from_raw_const(ptr))
            }
        }
    }
    pub fn get_line(&self, lineno: usize) -> Option<BlameHunk> {
        unsafe {
            let ptr = raw::git_blame_get_hunk_byline(self.raw(), lineno);
            if ptr.is_null() {
                None
            } else {
                Some(BlameHunk::from_raw_const(ptr))
            }
        }
    }
}
impl<'blame> BlameHunk<'blame> {
    unsafe fn from_raw_const(raw: *const raw::git_blame_hunk)
                                 -> BlameHunk<'blame> {
        BlameHunk {
            raw: raw as *mut raw::git_blame_hunk,
            _marker: marker::PhantomData,
        }
    }
}
impl<'repo> Binding for Blame<'repo> {
    type Raw = *mut raw::git_blame;
    unsafe fn from_raw(raw: *mut raw::git_blame) -> Blame<'repo> {
        Blame { raw: raw, _marker: marker::PhantomData }
    }
    fn raw(&self) -> *mut raw::git_blame { self.raw }
}
impl Binding for BlameOptions {
    type Raw = *mut raw::git_blame_options;
    unsafe fn from_raw(opts: *mut raw::git_blame_options) -> BlameOptions {
        BlameOptions { raw: *opts }
    }
    fn raw(&self) -> *mut raw::git_blame_options {
        &self.raw as *const _ as *mut _
    }
}
