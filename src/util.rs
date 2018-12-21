use std::ffi::CString;
use libc::{c_char, size_t};
use {raw, Error};
pub trait Binding: Sized {
    type Raw;
    unsafe fn from_raw(raw: Self::Raw) -> Self;
    fn raw(&self) -> Self::Raw;
}
pub fn iter2cstrs<T, I>(iter: I) -> Result<(Vec<CString>, Vec<*const c_char>,
                                            raw::git_strarray), Error>
    where T: IntoCString, I: IntoIterator<Item=T>
{
    let cstrs: Vec<_> = try!(iter.into_iter().map(|i| i.into_c_string()).collect());
    let ptrs = cstrs.iter().map(|i| i.as_ptr()).collect::<Vec<_>>();
    let raw = raw::git_strarray {
        strings: ptrs.as_ptr() as *mut _,
        count: ptrs.len() as size_t,
    };
    Ok((cstrs, ptrs, raw))
}
pub trait IntoCString {
    fn into_c_string(self) -> Result<CString, Error>;
}
