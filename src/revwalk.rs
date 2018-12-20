use std::marker;
use {raw, Error, Sort, Oid, Repository};
use util::Binding;
pub struct Revwalk<'repo> {
    _marker: marker::PhantomData<&'repo Repository>,
}
impl<'repo> Revwalk<'repo> {
    fn next(&mut self) -> Option<Result<Oid, Error>> {
        let mut out: raw::git_oid = raw::git_oid{ id: [0; raw::GIT_OID_RAWSZ] };
        unsafe {
            Some(Ok(Binding::from_raw(&out as *const _)))
        }
    }
}
