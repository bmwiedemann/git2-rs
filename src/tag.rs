use std::marker;
use std::mem;
use std::ptr;
use {raw, signature, Error, Oid, Object, Signature, ObjectType};
use util::Binding;
pub struct Tag<'repo> {
    raw: *mut raw::git_tag,
    _marker: marker::PhantomData<Object<'repo>>,
}
impl<'repo> Tag<'repo> {
    pub fn peel(&self) -> Result<Object<'repo>, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn tagger(&self) -> Option<Signature> {
        unsafe {
            let ptr = raw::git_tag_tagger(&*self.raw);
            if ptr.is_null() {
                None
            } else {
                Some(signature::from_raw_const(self, ptr))
            }
        }
    }
    pub fn target(&self) -> Result<Object<'repo>, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn as_object(&self) -> &Object<'repo> {
        unsafe {
            mem::transmute(self)
        }
    }
}
impl<'repo> Binding for Tag<'repo> {
    type Raw = *mut raw::git_tag;
    unsafe fn from_raw(raw: *mut raw::git_tag) -> Tag<'repo> {
        Tag { raw: raw, _marker: marker::PhantomData }
    }
    fn raw(&self) -> *mut raw::git_tag { self.raw }
}
impl<'repo> Clone for Tag<'repo> {
    fn clone(&self) -> Self {
        self.as_object().clone().into_tag().ok().unwrap()
    }
}
impl<'repo> Drop for Tag<'repo> {
    fn drop(&mut self) {
        unsafe { raw::git_tag_free(self.raw) }
    }
}
