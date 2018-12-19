use std::ops::Range;
use std::marker;
use libc::size_t;
use {raw, signature, Oid, Error, Signature};
use util::Binding;
pub struct Reflog {
    raw: *mut raw::git_reflog,
}
pub struct ReflogEntry<'reflog> {
    raw: *const raw::git_reflog_entry,
    _marker: marker::PhantomData<&'reflog Reflog>,
}
pub struct ReflogIter<'reflog> {
    range: Range<usize>,
    reflog: &'reflog Reflog,
}
impl Reflog {
    pub fn get(&self, i: usize) -> Option<ReflogEntry> {
        unsafe {
            let ptr = raw::git_reflog_entry_byindex(self.raw, i as size_t);
            Binding::from_raw_opt(ptr)
        }
    }
}
impl Binding for Reflog {
    type Raw = *mut raw::git_reflog;
    unsafe fn from_raw(raw: *mut raw::git_reflog) -> Reflog {
        Reflog { raw: raw }
    }
    fn raw(&self) -> *mut raw::git_reflog { self.raw }
}
impl<'reflog> ReflogEntry<'reflog> {
    pub fn committer(&self) -> Signature {
        unsafe {
            let ptr = raw::git_reflog_entry_committer(self.raw);
            signature::from_raw_const(self, ptr)
        }
    }
}
impl<'reflog> Binding for ReflogEntry<'reflog> {
    type Raw = *const raw::git_reflog_entry;
    unsafe fn from_raw(raw: *const raw::git_reflog_entry) -> ReflogEntry<'reflog> {
        ReflogEntry { raw: raw, _marker: marker::PhantomData }
    }
    fn raw(&self) -> *const raw::git_reflog_entry { self.raw }
}
impl<'reflog> Iterator for ReflogIter<'reflog> {
    type Item = ReflogEntry<'reflog>;
    fn next(&mut self) -> Option<ReflogEntry<'reflog>> {
        self.range.next().and_then(|i| self.reflog.get(i))
    }
}
impl<'reflog> DoubleEndedIterator for ReflogIter<'reflog> {
    fn next_back(&mut self) -> Option<ReflogEntry<'reflog>> {
        self.range.next_back().and_then(|i| self.reflog.get(i))
    }
}
