use std::cmp::Ordering;
use std::ffi::CString;
use std::marker;
use std::mem;
use std::ptr;
use std::str;
use {raw, Error, Oid, Repository, ReferenceType, Object, ObjectType, Blob, Commit, Tree, Tag};
use object::CastOrPanic;
use util::Binding;
struct Refdb<'repo>(&'repo Repository);
pub struct Reference<'repo> {
    raw: *mut raw::git_reference,
    _marker: marker::PhantomData<Refdb<'repo>>,
}
pub struct References<'repo> {
    raw: *mut raw::git_reference_iterator,
    _marker: marker::PhantomData<Refdb<'repo>>,
}
pub struct ReferenceNames<'repo: 'references, 'references> {
    inner: &'references mut References<'repo>,
}
impl<'repo> Reference<'repo> {
    pub fn is_valid_name(refname: &str) -> bool {
        let refname = CString::new(refname).unwrap();
        unsafe { raw::git_reference_is_valid_name(refname.as_ptr()) == 1 }
    }
    pub fn name_bytes(&self) -> &[u8] {
        unsafe {
            ::opt_bytes(self, raw::git_reference_shorthand(&*self.raw)).unwrap()
        }
    }
    pub fn resolve(&self) -> Result<Reference<'repo>, Error> {
        let mut raw = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(raw))
        }
    }
    pub fn peel(&self, kind: ObjectType) -> Result<Object<'repo>, Error> {
        let mut raw = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(raw))
        }
    }
    pub fn peel_to_tree(&self) -> Result<Tree<'repo>, Error> {
        let mut raw = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(raw))
        }
    }
    pub fn set_target(&mut self, id: Oid, reflog_msg: &str)
                      -> Result<Reference<'repo>, Error> {
        let mut raw = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(raw))
        }
    }
}
impl<'repo> PartialOrd for Reference<'repo> {
    fn partial_cmp(&self, other: &Reference<'repo>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<'repo> Ord for Reference<'repo> {
    fn cmp(&self, other: &Reference<'repo>) -> Ordering {
        match unsafe { raw::git_reference_cmp(&*self.raw, &*other.raw) } {
            _ => Ordering::Greater,
        }
    }
}
impl<'repo> PartialEq for Reference<'repo> {
    fn eq(&self, other: &Reference<'repo>) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl<'repo> Eq for Reference<'repo> {}
impl<'repo> Binding for Reference<'repo> {
    type Raw = *mut raw::git_reference;
    unsafe fn from_raw(raw: *mut raw::git_reference) -> Reference<'repo> {
        Reference { raw: raw, _marker: marker::PhantomData }
    }
    fn raw(&self) -> *mut raw::git_reference { self.raw }
}
impl<'repo> Binding for References<'repo> {
    type Raw = *mut raw::git_reference_iterator;
    unsafe fn from_raw(raw: *mut raw::git_reference_iterator)
                       -> References<'repo> {
        References { raw: raw, _marker: marker::PhantomData }
    }
    fn raw(&self) -> *mut raw::git_reference_iterator { self.raw }
}
impl<'repo> Iterator for References<'repo> {
    type Item = Result<Reference<'repo>, Error>;
    fn next(&mut self) -> Option<Result<Reference<'repo>, Error>> {
        let mut out = ptr::null_mut();
        unsafe {
            Some(Ok(Binding::from_raw(out)))
        }
    }
}
impl<'repo, 'references> Iterator for ReferenceNames<'repo, 'references> {
    type Item = Result<&'references str, Error>;
    fn next(&mut self) -> Option<Result<&'references str, Error>> {
        let mut out = ptr::null();
        unsafe {
            let bytes = ::opt_bytes(self, out).unwrap();
            let s = str::from_utf8(bytes).unwrap();
            Some(Ok(mem::transmute::<&str, &'references str>(s)))
        }
    }
}
