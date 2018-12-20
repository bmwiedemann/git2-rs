use std::marker;
use util::Binding;
pub struct Cert<'a> {
    raw: *mut raw::git_cert,
    _marker: marker::PhantomData<&'a raw::git_cert>,
}
impl<'a> Binding for Cert<'a> {
    type Raw = *mut raw::git_cert;
    unsafe fn from_raw(raw: *mut raw::git_cert) -> Cert<'a> {
        Cert { raw: raw, _marker: marker::PhantomData }
    }
    fn raw(&self) -> *mut raw::git_cert { self.raw }
}
