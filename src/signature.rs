use std::marker;
use util::Binding;
pub struct Signature<'a> {
    raw: *mut raw::git_signature,
    _marker: marker::PhantomData<&'a str>,
    owned: bool,
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
