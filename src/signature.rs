use std::ffi::CString;
use std::marker;
use std::mem;
use std::ptr;
use std::str;
use std::fmt;
use {raw, Error, Time};
use util::Binding;
pub struct Signature<'a> {
    raw: *mut raw::git_signature,
    _marker: marker::PhantomData<&'a str>,
    owned: bool,
}
impl<'a> Signature<'a> {
    pub fn now(name: &str, email: &str) -> Result<Signature<'static>, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn new(name: &str, email: &str, time: &Time)
               -> Result<Signature<'static>, Error> {
        let mut ret = ptr::null_mut();
        let name = try!(CString::new(name));
        let email = try!(CString::new(email));
        unsafe {
            try_call!(raw::git_signature_new(&mut ret, name, email,
                                             time.seconds() as raw::git_time_t,
                                             time.offset_minutes() as libc::c_int));
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn name(&self) -> Option<&str> {
        str::from_utf8(self.name_bytes()).ok()
    }
    pub fn name_bytes(&self) -> &[u8] {
        unsafe { ::opt_bytes(self, (*self.raw).name).unwrap() }
    }
    pub fn email(&self) -> Option<&str> {
        str::from_utf8(self.email_bytes()).ok()
    }
    pub fn email_bytes(&self) -> &[u8] {
        unsafe { ::opt_bytes(self, (*self.raw).email).unwrap() }
    }
    pub fn when(&self) -> Time {
        unsafe { Binding::from_raw((*self.raw).when) }
    }
    pub fn to_owned(&self) -> Signature<'static> {
        unsafe {
            let me = mem::transmute::<&Signature<'a>, &Signature<'static>>(self);
            me.clone()
        }
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
impl Clone for Signature<'static> {
    fn clone(&self) -> Signature<'static> {
        let mut raw = ptr::null_mut();
        unsafe { Binding::from_raw(raw) }
    }
}
impl<'a> Drop for Signature<'a> {
    fn drop(&mut self) {
    }
}
impl<'a> fmt::Display for Signature<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} <{}>",
               String::from_utf8_lossy(self.name_bytes()),
               String::from_utf8_lossy(self.email_bytes()))
    }
}
