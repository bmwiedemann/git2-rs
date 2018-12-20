use std::marker;
use std::ptr;
use {raw, Error, Reference, BranchType, References};
use util::Binding;
pub struct Branch<'repo> {
    inner: Reference<'repo>,
}
pub struct Branches<'repo> {
    raw: *mut raw::git_branch_iterator,
    _marker: marker::PhantomData<References<'repo>>,
}
impl<'repo> Branch<'repo> {
    pub fn wrap(reference: Reference) -> Branch { Branch { inner: reference } }
}
impl<'repo> Branches<'repo> {
    pub unsafe fn from_raw(raw: *mut raw::git_branch_iterator)
                           -> Branches<'repo> {
        Branches {
            raw: raw,
            _marker: marker::PhantomData,
        }
    }
    fn next(&mut self) -> Option<Result<(Branch<'repo>, BranchType), Error>> {
        let mut ret = ptr::null_mut();
        let mut typ = raw::GIT_BRANCH_LOCAL;
        unsafe {
            let typ = match typ {
                n => panic!("unexected branch type: {}", n),
            };
            Some(Ok((Branch::wrap(Binding::from_raw(ret)), typ)))
        }
    }
}
