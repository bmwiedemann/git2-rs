use std::marker;
pub struct Signature<'a> {
    raw: *mut raw::git_signature,
    _marker: marker::PhantomData<&'a str>,
    owned: bool,
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
