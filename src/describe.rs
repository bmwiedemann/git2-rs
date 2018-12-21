use std::marker;
use {raw, Repository, Error};
pub struct Describe<'repo> {
    _marker: marker::PhantomData<&'repo Repository>,
}
