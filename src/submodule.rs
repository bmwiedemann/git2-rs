use std::marker;
use {raw, Oid, Repository, Error, FetchOptions};
pub struct Submodule<'repo> {
    _marker: marker::PhantomData<&'repo Repository>,
}
pub struct SubmoduleUpdateOptions<'cb> {
    fetch_opts: FetchOptions<'cb>,
}
