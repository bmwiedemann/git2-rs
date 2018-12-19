use std::{marker, ptr, mem, str};
use {raw, Oid, Error, Signature, MergeOptions, Index};
use build::CheckoutBuilder;
use util::Binding;
pub struct RebaseOptions<'cb> {
    raw: raw::git_rebase_options,
    merge_options: Option<MergeOptions>,
    checkout_options: Option<CheckoutBuilder<'cb>>,
}
impl<'cb> RebaseOptions<'cb> {
    pub fn new() -> RebaseOptions<'cb> {
        let mut opts = RebaseOptions {
            raw: unsafe { mem::zeroed() },
            merge_options: None,
            checkout_options: None,
        };
        opts
    }
    pub fn raw(&mut self) -> *const raw::git_rebase_options {
        &self.raw
    }
}
pub struct Rebase<'repo> {
    raw: *mut raw::git_rebase,
    _marker: marker::PhantomData<&'repo raw::git_rebase>,
}
impl <'repo> Rebase<'repo> {
    pub fn nth(&mut self, n: usize) -> Option<RebaseOperation> {
        unsafe {
            let op = raw::git_rebase_operation_byindex(self.raw, n);
            if op.is_null() {
                None
            } else {
                Some(RebaseOperation::from_raw(op))
            }
        }
    }
    pub fn operation_current(&mut self) -> Option<usize> {
        let cur = unsafe { raw::git_rebase_operation_current(self.raw) };
        if cur == raw::GIT_REBASE_NO_OPERATION {
            None
        } else {
            Some(cur)
        }
    }
    pub fn inmemory_index(&mut self) -> Result<Index, Error> {
        let mut idx = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(idx))
        }
    }
    pub fn commit(&mut self, author: &Signature, committer: &Signature, message: &str) -> Result<Oid, Error> {
        let mut id: raw::git_oid = unsafe { mem::zeroed() };
        unsafe {
            Ok(Binding::from_raw(&id as *const _))
        }
    }
}
impl <'rebase> Iterator for Rebase<'rebase> {
    type Item = Result<RebaseOperation<'rebase>, Error>;
    fn next(&mut self) -> Option<Result<RebaseOperation<'rebase>, Error>> {
        let mut out = ptr::null_mut();
        unsafe {
            Some(Ok(RebaseOperation::from_raw(out)))
        }
    }
}
impl<'repo> Binding for Rebase<'repo> {
    type Raw = *mut raw::git_rebase;
    unsafe fn from_raw(raw: *mut raw::git_rebase)
                       -> Rebase<'repo> {
        Rebase {
            raw: raw,
            _marker: marker::PhantomData,
        }
    }
    fn raw(&self) -> *mut raw::git_rebase { self.raw }
}
pub enum RebaseOperationType {
}
#[derive(Debug)]
pub struct RebaseOperation<'rebase> {
    raw: *const raw::git_rebase_operation,
    _marker: marker::PhantomData<Rebase<'rebase>>,
}
impl<'rebase> RebaseOperation<'rebase> {
    unsafe fn from_raw(raw: *const raw::git_rebase_operation) -> RebaseOperation<'rebase> {
        RebaseOperation {
            raw: raw,
            _marker: marker::PhantomData,
        }
    }
}
