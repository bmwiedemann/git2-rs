use std::ffi::{CString, OsStr, OsString};
use std::path::{Path, PathBuf};
use libc::{c_char, size_t};
use {raw, Error};
pub trait IsNull {
    fn is_ptr_null(&self) -> bool;
}
impl<T> IsNull for *mut T {
    fn is_ptr_null(&self) -> bool {
        self.is_null()
    }
}
pub trait Binding: Sized {
    type Raw;
    unsafe fn from_raw(raw: Self::Raw) -> Self;
    fn raw(&self) -> Self::Raw;
    unsafe fn from_raw_opt<T>(raw: T) -> Option<Self>
        where T: Copy + IsNull, Self: Binding<Raw=T>
    {
        if raw.is_ptr_null() {
            None
        } else {
            Some(Binding::from_raw(raw))
        }
    }
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
pub fn bytes2path(b: &[u8]) -> &Path {
    use std::os::unix::prelude::*;
    Path::new(OsStr::from_bytes(b))
}
pub trait IntoCString {
    fn into_c_string(self) -> Result<CString, Error>;
}
