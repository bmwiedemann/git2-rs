use std::marker;
use std::mem;
use {raw, signature, Error, Oid, Object, Signature, ObjectType};
pub struct Tag<'repo> {
    raw: *mut raw::git_tag,
    _marker: marker::PhantomData<Object<'repo>>,
}
impl<'repo> Tag<'repo> {
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
    pub fn as_object(&self) -> &Object<'repo> {
        unsafe {
            mem::transmute(self)
        }
    }
}
