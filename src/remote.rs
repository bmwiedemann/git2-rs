use std::marker;
use std::ptr;
use {raw, Direction, Error, Refspec, Oid, FetchPrune, ProxyOptions};
use {RemoteCallbacks, Repository, AutotagOption};
use util::Binding;
pub struct Remote<'repo> {
    _marker: marker::PhantomData<&'repo Repository>,
}
pub struct Refspecs<'remote> {
    remote: &'remote Remote<'remote>,
}
pub struct RemoteHead<'remote> {
    _marker: marker::PhantomData<&'remote str>,
}
pub struct FetchOptions<'cb> {
    callbacks: Option<RemoteCallbacks<'cb>>,
    proxy: Option<ProxyOptions<'cb>>,
    prune: FetchPrune,
    update_fetchhead: bool,
    download_tags: AutotagOption,
}
pub struct PushOptions<'cb> {
    callbacks: Option<RemoteCallbacks<'cb>>,
    proxy: Option<ProxyOptions<'cb>>,
    pb_parallelism: u32,
}
pub struct RemoteConnection<'repo, 'connection, 'cb> where 'repo: 'connection {
    _callbacks: Box<RemoteCallbacks<'cb>>,
    remote: &'connection mut Remote<'repo>,
}
impl<'cb> Binding for FetchOptions<'cb> {
    type Raw = raw::git_fetch_options;
    unsafe fn from_raw(_raw: raw::git_fetch_options) -> FetchOptions<'cb> {
        panic!("unimplemented");
    }
    fn raw(&self) -> raw::git_fetch_options {
        raw::git_fetch_options {
            version: 1,
            callbacks: self.callbacks.as_ref().map(|m| m.raw())
                           .unwrap_or_else(|| RemoteCallbacks::new().raw()),
            proxy_opts: self.proxy.as_ref().map(|m| m.raw())
                            .unwrap_or_else(|| ProxyOptions::new().raw()),
            prune: ::call::convert(&self.prune),
            update_fetchhead: ::call::convert(&self.update_fetchhead),
            download_tags: ::call::convert(&self.download_tags),
            custom_headers: raw::git_strarray {
                count: 0,
                strings: ptr::null_mut(),
            },
        }
    }
}
impl<'cb> PushOptions<'cb> {
    fn raw(&self) -> raw::git_push_options {
        raw::git_push_options {
            version: 1,
            callbacks: self.callbacks.as_ref()
                           .map(|m| m.raw())
                           .unwrap_or_else(|| RemoteCallbacks::new().raw()),
            proxy_opts: self.proxy.as_ref().map(|m| m.raw())
                            .unwrap_or_else(|| ProxyOptions::new().raw()),
            pb_parallelism: self.pb_parallelism as libc::c_uint,
            custom_headers: raw::git_strarray {
                count: 0,
                strings: ptr::null_mut(),
            },
        }
    }
}
