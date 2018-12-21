use std::marker;
use {raw, Error, Oid, Repository, ReferenceType, Object, ObjectType, Blob, Commit, Tree, Tag};
use util::Binding;
struct Refdb<'repo>(&'repo Repository);
pub struct Reference<'repo> {
    raw: *mut raw::git_reference,
    _marker: marker::PhantomData<Refdb<'repo>>,
}
pub struct References<'repo> {
    _marker: marker::PhantomData<Refdb<'repo>>,
}
pub struct ReferenceNames<'repo: 'references, 'references> {
    inner: &'references mut References<'repo>,
}
impl<'repo> Binding for Reference<'repo> {
    type Raw = *mut raw::git_reference;
    unsafe fn from_raw(raw: *mut raw::git_reference) -> Reference<'repo> {
        Reference { raw: raw, _marker: marker::PhantomData }
    }
    fn raw(&self) -> *mut raw::git_reference { self.raw }
}
