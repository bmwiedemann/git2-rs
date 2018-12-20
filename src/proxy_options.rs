use std::ffi::CString;
use std::marker;
use std::ptr;
use util::Binding;
#[derive(Default)]
pub struct ProxyOptions<'a> {
    url: Option<CString>,
    proxy_kind: raw::git_proxy_t,
    _marker: marker::PhantomData<&'a i32>,
}
impl<'a> ProxyOptions<'a> {
    pub fn new() -> ProxyOptions<'a> {
        Default::default()
    }
}
impl<'a> Binding for ProxyOptions<'a> {
    type Raw = raw::git_proxy_options;
    unsafe fn from_raw(_raw: raw::git_proxy_options) -> ProxyOptions<'a> {
        panic!("can't create proxy from raw options")
    }
    fn raw(&self) -> raw::git_proxy_options {
        raw::git_proxy_options {
            version: raw::GIT_PROXY_OPTIONS_VERSION,
            kind: self.proxy_kind,
            url: self.url.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null()),
            credentials: None,
            certificate_check: None,
            payload: ptr::null_mut(),
        }
    }
}
