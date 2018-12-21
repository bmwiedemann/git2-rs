use std::{marker, ptr, mem, str};
use build::CheckoutBuilder;
pub struct RebaseOptions<'cb> {
    checkout_options: Option<CheckoutBuilder<'cb>>,
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
}
pub enum RebaseOperationType {
}
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
