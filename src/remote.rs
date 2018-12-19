use std::ops::Range;
use std::marker;
use std::mem;
use std::ptr;
use std::slice;
use {raw, Direction, Error, Refspec, Oid, FetchPrune, ProxyOptions};
use {RemoteCallbacks, Progress, Repository, AutotagOption};
use string_array::StringArray;
use util::Binding;
pub struct Remote<'repo> {
    raw: *mut raw::git_remote,
    _marker: marker::PhantomData<&'repo Repository>,
}
pub struct Refspecs<'remote> {
    range: Range<usize>,
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
    _proxy: ProxyOptions<'cb>,
    remote: &'connection mut Remote<'repo>,
}
pub fn remote_into_raw(remote: Remote) -> *mut raw::git_remote {
    let ret = remote.raw;
    return ret
}
impl<'repo> Remote<'repo> {
    pub fn connect_auth<'connection, 'cb>(&'connection mut self,
                                          dir: Direction,
                                          cb: Option<RemoteCallbacks<'cb>>,
                                          proxy_options: Option<ProxyOptions<'cb>>)
                    -> Result<RemoteConnection<'repo, 'connection, 'cb>, Error> {
        let cb = Box::new(cb.unwrap_or_else(RemoteCallbacks::new));
        let proxy_options = proxy_options.unwrap_or_else(ProxyOptions::new);
        unsafe {
            try_call!(raw::git_remote_connect(self.raw, dir,
                                              &cb.raw(),
                                              &proxy_options.raw(),
                                              ptr::null()));
        }
        Ok(RemoteConnection {
            _callbacks: cb,
            _proxy: proxy_options,
            remote: self,
        })
    }
    pub fn connected(&mut self) -> bool {
        unsafe { raw::git_remote_connected(self.raw) == 1 }
    }
    pub fn get_refspec(&self, i: usize) -> Option<Refspec<'repo>> {
        unsafe {
            let ptr = raw::git_remote_get_refspec(&*self.raw,
                                                  i as libc::size_t);
            Binding::from_raw_opt(ptr)
        }
    }
    pub fn list(&self) -> Result<&[RemoteHead], Error> {
        let mut size = 0;
        let mut base = ptr::null_mut();
        unsafe {
            let slice = slice::from_raw_parts(base as *const _, size as usize);
            Ok(mem::transmute::<&[*const raw::git_remote_head],
                                &[RemoteHead]>(slice))
        }
    }
    pub fn fetch_refspecs(&self) -> Result<StringArray, Error> {
        unsafe {
            let mut raw: raw::git_strarray = mem::zeroed();
            Ok(StringArray::from_raw(raw))
        }
    }
    fn clone(&self) -> Remote<'repo> {
        let mut ret = ptr::null_mut();
        Remote {
            raw: ret,
            _marker: marker::PhantomData,
        }
    }
}
impl<'repo> Binding for Remote<'repo> {
    type Raw = *mut raw::git_remote;
    unsafe fn from_raw(raw: *mut raw::git_remote) -> Remote<'repo> {
        Remote {
            raw: raw,
            _marker: marker::PhantomData,
        }
    }
    fn raw(&self) -> *mut raw::git_remote { self.raw }
}
impl<'repo> Iterator for Refspecs<'repo> {
    type Item = Refspec<'repo>;
    fn next(&mut self) -> Option<Refspec<'repo>> {
        self.range.next().and_then(|i| self.remote.get_refspec(i))
    }
}
impl<'cb> FetchOptions<'cb> {
    pub fn new() -> FetchOptions<'cb> {
        FetchOptions {
            callbacks: None,
            proxy: None,
            prune: FetchPrune::Unspecified,
            update_fetchhead: true,
            download_tags: AutotagOption::Unspecified,
        }
    }
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
    pub fn new() -> PushOptions<'cb> {
        PushOptions {
            callbacks: None,
            proxy: None,
            pb_parallelism: 1,
        }
    }
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
impl<'repo, 'connection, 'cb> RemoteConnection<'repo, 'connection, 'cb> {
    pub fn connected(&mut self) -> bool {
        self.remote.connected()
    }
}
