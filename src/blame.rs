use std::marker;
use {raw, Repository, Oid, signature, Signature};
pub struct Blame<'repo> {
    _marker: marker::PhantomData<&'repo Repository>,
}
pub struct BlameHunk<'blame> {
    _marker: marker::PhantomData<&'blame raw::git_blame>,
}
pub struct BlameOptions {
}
pub struct BlameIter<'blame> {
    blame: &'blame Blame<'blame>,
}
