use std::marker;
use std::mem;
use std::os::raw::c_int;
use {raw, Oid, Repository, Error, FetchOptions};
use util::{self, Binding};
pub struct Submodule<'repo> {
    _marker: marker::PhantomData<&'repo Repository>,
}
pub struct SubmoduleUpdateOptions<'cb> {
    fetch_opts: FetchOptions<'cb>,
    allow_fetch: bool,
}
impl<'cb> SubmoduleUpdateOptions<'cb> {
    unsafe fn raw(&mut self) -> raw::git_submodule_update_options {
        let mut checkout_opts: raw::git_checkout_options = mem::zeroed();
        let opts = raw::git_submodule_update_options {
            version: raw::GIT_SUBMODULE_UPDATE_OPTIONS_VERSION,
            checkout_opts,
            fetch_opts: self.fetch_opts.raw(),
            allow_fetch: self.allow_fetch as c_int,
        };
        opts
    }
}
