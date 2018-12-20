use std::marker;
use std::fmt;
use util::Binding;
pub struct Signature<'a> {
    raw: *mut raw::git_signature,
    _marker: marker::PhantomData<&'a str>,
    owned: bool,
}
impl<'a> Signature<'a> {
    pub fn name_bytes(&self) -> &[u8] {
        unsafe { ::opt_bytes(self, (*self.raw).name).unwrap() }
    }
    pub fn email_bytes(&self) -> &[u8] {
        unsafe { ::opt_bytes(self, (*self.raw).email).unwrap() }
    }
}
impl<'a> Binding for Signature<'a> {
    type Raw = *mut raw::git_signature;
    unsafe fn from_raw(raw: *mut raw::git_signature) -> Signature<'a> {
        Signature {
            raw: raw,
            _marker: marker::PhantomData,
            owned: true,
        }
    }
    fn raw(&self) -> *mut raw::git_signature { self.raw }
}
pub unsafe fn from_raw_const<'b, T>(_lt: &'b T,
                                    raw: *const raw::git_signature)
                                    -> Signature<'b> {
    Signature {
        raw: raw as *mut raw::git_signature,
        _marker: marker::PhantomData,
        owned: false,
    }
}
impl<'a> fmt::Display for Signature<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} <{}>",
               String::from_utf8_lossy(self.name_bytes()),
               String::from_utf8_lossy(self.email_bytes()))
    }
}
