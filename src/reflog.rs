use std::marker;
pub struct Reflog {
}
pub struct ReflogEntry<'reflog> {
    _marker: marker::PhantomData<&'reflog Reflog>,
}
pub struct ReflogIter<'reflog> {
    reflog: &'reflog Reflog,
}
