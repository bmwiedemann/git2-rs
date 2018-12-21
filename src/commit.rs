use std::marker;
use {raw, signature, Oid, Error, Signature, Tree, Time, Object};
pub struct Commit<'repo> {
    _marker: marker::PhantomData<Object<'repo>>,
}
pub struct Parents<'commit, 'repo: 'commit> {
    commit: &'commit Commit<'repo>,
}
