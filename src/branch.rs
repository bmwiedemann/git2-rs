use std::marker;
use {raw, Error, Reference, BranchType, References};
pub struct Branch<'repo> {
    inner: Reference<'repo>,
}
pub struct Branches<'repo> {
    _marker: marker::PhantomData<References<'repo>>,
}
