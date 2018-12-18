use std::ffi::CString;
use std::ops::Range;
use std::marker;
use std::mem;
use std::ptr;
use std::slice;
use std::str;
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
    raw: *const raw::git_remote_head,
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
    pub fn is_valid_name(remote_name: &str) -> bool {
        let remote_name = CString::new(remote_name).unwrap();
        unsafe { raw::git_remote_is_valid_name(remote_name.as_ptr()) == 1 }
    }
    pub fn name(&self) -> Option<&str> {
        self.name_bytes().and_then(|s| str::from_utf8(s).ok())
    }
    pub fn name_bytes(&self) -> Option<&[u8]> {
        unsafe { ::opt_bytes(self, raw::git_remote_name(&*self.raw)) }
    }
    pub fn url(&self) -> Option<&str> {
        str::from_utf8(self.url_bytes()).ok()
    }
    pub fn url_bytes(&self) -> &[u8] {
        unsafe { ::opt_bytes(self, raw::git_remote_url(&*self.raw)).unwrap() }
    }
    pub fn pushurl(&self) -> Option<&str> {
        self.pushurl_bytes().and_then(|s| str::from_utf8(s).ok())
    }
    pub fn pushurl_bytes(&self) -> Option<&[u8]> {
        unsafe { ::opt_bytes(self, raw::git_remote_pushurl(&*self.raw)) }
    }
    pub fn connect(&mut self, dir: Direction) -> Result<(), Error> {
        unsafe {
            try_call!(raw::git_remote_connect(self.raw, dir,
                                              ptr::null(),
                                              ptr::null(),
                                              ptr::null()));
        }
        Ok(())
    }
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
    pub fn download(&mut self, specs: &[&str], opts: Option<&mut FetchOptions>)
                    -> Result<(), Error> {
        Ok(())
    }
    pub fn refspecs<'a>(&'a self) -> Refspecs<'a> {
        let cnt = unsafe { raw::git_remote_refspec_count(&*self.raw) as usize };
        Refspecs { range: 0..cnt, remote: self }
    }
    pub fn get_refspec(&self, i: usize) -> Option<Refspec<'repo>> {
        unsafe {
            let ptr = raw::git_remote_get_refspec(&*self.raw,
                                                  i as libc::size_t);
            Binding::from_raw_opt(ptr)
        }
    }
    pub fn fetch(&mut self,
                 reflog_msg: Option<&str>) -> Result<(), Error> {
        Ok(())
    }
    pub fn update_tips(&mut self,
                       callbacks: Option<&mut RemoteCallbacks>,
                       update_fetchhead: bool,
                       download_tags: AutotagOption,
                       msg: Option<&str>) -> Result<(), Error> {
        let msg = try!(::opt_cstr(msg));
        let cbs = callbacks.map(|cb| cb.raw());
        unsafe {
            try_call!(raw::git_remote_update_tips(self.raw, cbs.as_ref(),
                                                  update_fetchhead,
                                                  download_tags, msg));
        }
        Ok(())
    }
    pub fn push(&mut self,
                opts: Option<&mut PushOptions>) -> Result<(), Error> {
        Ok(())
    }
    pub fn stats(&self) -> Progress {
        unsafe {
            Binding::from_raw(raw::git_remote_stats(self.raw))
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
    pub fn push_refspecs(&self) -> Result<StringArray, Error> {
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
impl<'repo> Drop for Remote<'repo> {
    fn drop(&mut self) {
    }
}
impl<'repo> Iterator for Refspecs<'repo> {
    type Item = Refspec<'repo>;
    fn next(&mut self) -> Option<Refspec<'repo>> {
        self.range.next().and_then(|i| self.remote.get_refspec(i))
    }
}
impl<'repo> DoubleEndedIterator for Refspecs<'repo> {
    fn next_back(&mut self) -> Option<Refspec<'repo>> {
        self.range.next_back().and_then(|i| self.remote.get_refspec(i))
    }
}
impl<'remote> RemoteHead<'remote> {
    pub fn is_local(&self) -> bool {
        unsafe { (*self.raw).local != 0 }
    }
    pub fn oid(&self) -> Oid {
        unsafe { Binding::from_raw(&(*self.raw).oid as *const _) }
    }
    pub fn loid(&self) -> Oid {
        unsafe { Binding::from_raw(&(*self.raw).loid as *const _) }
    }
    pub fn name(&self) -> &str {
        let b = unsafe { ::opt_bytes(self, (*self.raw).name).unwrap() };
        str::from_utf8(b).unwrap()
    }
    pub fn symref_target(&self) -> Option<&str> {
        let b = unsafe { ::opt_bytes(self, (*self.raw).symref_target) };
        b.map(|b| str::from_utf8(b).unwrap())
    }
}
impl<'cb> Default for FetchOptions<'cb> {
    fn default() -> Self {
        Self::new()
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
    pub fn remote_callbacks(&mut self, cbs: RemoteCallbacks<'cb>) -> &mut Self {
        self
    }
    pub fn proxy_options(&mut self, opts: ProxyOptions<'cb>) -> &mut Self {
        self
    }
    pub fn prune(&mut self, prune: FetchPrune) -> &mut Self {
        self
    }
    pub fn update_fetchhead(&mut self, update: bool) -> &mut Self {
        self
    }
    pub fn download_tags(&mut self, opt: AutotagOption) -> &mut Self {
        self
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
impl<'cb> Default for PushOptions<'cb> {
    fn default() -> Self {
        Self::new()
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
    pub fn remote_callbacks(&mut self, cbs: RemoteCallbacks<'cb>) -> &mut Self {
        self
    }
    pub fn proxy_options(&mut self, opts: ProxyOptions<'cb>) -> &mut Self {
        self
    }
    pub fn packbuilder_parallelism(&mut self, parallel: u32) -> &mut Self {
        self
    }
}
impl<'cb> Binding for PushOptions<'cb> {
    type Raw = raw::git_push_options;
    unsafe fn from_raw(_raw: raw::git_push_options) -> PushOptions<'cb> {
        panic!("unimplemented");
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
    pub fn list(&self) -> Result<&[RemoteHead], Error> {
        self.remote.list()
    }
}
