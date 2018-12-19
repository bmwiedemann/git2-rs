use std::marker;
use std::ptr;
use {raw, Oid, ObjectType, Error, Buf, Commit, Tag, Blob, Tree, Repository};
use {Describe, DescribeOptions};
use util::Binding;
pub struct Object<'repo> {
    raw: *mut raw::git_object,
    _marker: marker::PhantomData<&'repo Repository>,
}
impl<'repo> Object<'repo> {
    pub fn id(&self) -> Oid {
        unsafe {
            Binding::from_raw(raw::git_object_id(&*self.raw))
        }
    }
    pub fn kind(&self) -> Option<ObjectType> {
        ObjectType::from_raw(unsafe { raw::git_object_type(&*self.raw) })
    }
    pub fn peel(&self, kind: ObjectType) -> Result<Object<'repo>, Error> {
        let mut raw = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(raw))
        }
    }
    pub fn short_id(&self) -> Result<Buf, Error> {
        unsafe {
            let buf = Buf::new();
            Ok(buf)
        }
    }
    pub fn into_commit(self) -> Result<Commit<'repo>, Object<'repo>> {
        self.cast_into(ObjectType::Commit)
    }
    pub fn into_tag(self) -> Result<Tag<'repo>, Object<'repo>> {
        self.cast_into(ObjectType::Tag)
    }
    pub fn into_tree(self) -> Result<Tree<'repo>, Object<'repo>> {
        self.cast_into(ObjectType::Tree)
    }
    pub fn into_blob(self) -> Result<Blob<'repo>, Object<'repo>> {
        self.cast_into(ObjectType::Blob)
    }
    pub fn describe(&self, opts: &DescribeOptions)
                    -> Result<Describe, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(ret))
        }
    }
    fn cast<T>(&self, kind: ObjectType) -> Option<&T> {
        if self.kind() == Some(kind) {
            unsafe { Some(&*(self as *const _ as *const T)) }
        } else {
            None
        }
    }
    fn cast_into<T>(self, kind: ObjectType) -> Result<T, Object<'repo>> {
        if self.kind() == Some(kind) {
            Ok(unsafe {
                let other = ptr::read(&self as *const _ as *const T);
                other
            })
        } else {
            Err(self)
        }
    }
}
pub trait CastOrPanic {
    fn cast_or_panic<T>(self, kind: ObjectType) -> T;
}
impl<'repo> CastOrPanic for Object<'repo> {
    fn cast_or_panic<T>(self, kind: ObjectType) -> T {
        if self.kind() == Some(kind) {
            unsafe {
                let other = ptr::read(&self as *const _ as *const T);
                other
            }
        } else {
            let buf;
            let akind = match self.kind() {
                Some(akind) => akind.str(),
                None => {
                    buf = format!("unknown ({})", unsafe { raw::git_object_type(&*self.raw) });
                    &buf
                }
            };
            panic!("Expected object {} to be {} but it is {}", self.id(), kind.str(), akind)
        }
    }
}
impl<'repo> Clone for Object<'repo> {
    fn clone(&self) -> Object<'repo> {
        let mut raw = ptr::null_mut();
        unsafe {
            Binding::from_raw(raw)
        }
    }
}
impl<'repo> ::std::fmt::Debug for Object<'repo> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        let mut ds = f.debug_struct("Object");
        ds.finish()
    }
}
impl<'repo> Binding for Object<'repo> {
    type Raw = *mut raw::git_object;
    unsafe fn from_raw(raw: *mut raw::git_object) -> Object<'repo> {
        Object { raw: raw, _marker: marker::PhantomData, }
    }
    fn raw(&self) -> *mut raw::git_object { self.raw }
}
impl<'repo> Drop for Object<'repo> {
    fn drop(&mut self) {
        unsafe { raw::git_object_free(self.raw) }
    }
}
