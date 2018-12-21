use std::marker;
use {raw, Oid, Commit, FileFavor};
pub struct AnnotatedCommit<'repo> {
    _marker: marker::PhantomData<Commit<'repo>>,
}
pub struct MergeOptions {
}
