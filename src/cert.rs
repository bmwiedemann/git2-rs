use std::marker;
use util::Binding;
pub struct Cert<'a> {
    raw: *mut raw::git_cert,
    _marker: marker::PhantomData<&'a raw::git_cert>,
}
pub struct CertHostkey<'a> {
    raw: *mut raw::git_cert_hostkey,
    _marker: marker::PhantomData<&'a raw::git_cert>,
}
impl<'a> Cert<'a> {
    fn cast<T>(&self, kind: raw::git_cert_t) -> Option<&T> {
        unsafe {
            if kind == (*self.raw).cert_type {
                Some(&*(self as *const Cert<'a> as *const T))
            } else {
                None
            }
        }
    }
}
impl<'a> CertHostkey<'a> {
    pub fn hash_md5(&self) -> Option<&[u8; 16]> {
        unsafe {
            if (*self.raw).kind as u32 & raw::GIT_CERT_SSH_MD5 as u32 == 0 {
                None
            } else {
                Some(&(*self.raw).hash_md5)
            }
        }
    }
    pub fn hash_sha1(&self) -> Option<&[u8; 20]> {
        unsafe {
            if (*self.raw).kind as u32 & raw::GIT_CERT_SSH_SHA1 as u32 == 0 {
                None
            } else {
                Some(&(*self.raw).hash_sha1)
            }
        }
    }
}
impl<'a> Binding for Cert<'a> {
    type Raw = *mut raw::git_cert;
    unsafe fn from_raw(raw: *mut raw::git_cert) -> Cert<'a> {
        Cert { raw: raw, _marker: marker::PhantomData }
    }
    fn raw(&self) -> *mut raw::git_cert { self.raw }
}
