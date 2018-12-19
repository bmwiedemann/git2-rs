use std::marker;
use std::mem;
use std::slice;
use std::io;
use {raw, Oid, Object, Error};
use util::Binding;
pub struct Blob<'repo> {
    raw: *mut raw::git_blob,
    _marker: marker::PhantomData<Object<'repo>>,
}
impl<'repo> Blob<'repo> {
    pub fn content(&self) -> &[u8] {
        unsafe {
            let data = raw::git_blob_rawcontent(&*self.raw) as *const u8;
            let len = raw::git_blob_rawsize(&*self.raw) as usize;
            slice::from_raw_parts(data, len)
        }
    }
    pub fn as_object(&self) -> &Object<'repo> {
        unsafe {
            mem::transmute(self)
        }
    }
}
impl<'repo> Binding for Blob<'repo> {
    type Raw = *mut raw::git_blob;
    unsafe fn from_raw(raw: *mut raw::git_blob) -> Blob<'repo> {
        Blob {
            raw: raw,
            _marker: marker::PhantomData,
        }
    }
    fn raw(&self) -> *mut raw::git_blob { self.raw }
}
impl<'repo> Clone for Blob<'repo> {
    fn clone(&self) -> Self {
        self.as_object().clone().into_blob().ok().unwrap()
    }
}
impl<'repo> Drop for Blob<'repo> {
    fn drop(&mut self) {
        unsafe { raw::git_blob_free(self.raw) }
    }
}
pub struct BlobWriter<'repo> {
    raw: *mut raw::git_writestream,
    need_cleanup: bool,
    _marker: marker::PhantomData<Object<'repo>>,
}
impl<'repo> BlobWriter<'repo> {
    pub fn commit(mut self) -> Result<Oid, Error> {
        let mut raw = raw::git_oid { id: [0; raw::GIT_OID_RAWSZ] };
        unsafe {
            Ok(Binding::from_raw(&raw as *const _))
        }
    }
}
impl<'repo> Binding for BlobWriter<'repo> {
    type Raw = *mut raw::git_writestream;
    unsafe fn from_raw(raw: *mut raw::git_writestream) -> BlobWriter<'repo> {
        BlobWriter {
            raw: raw,
            need_cleanup: true,
            _marker: marker::PhantomData,
        }
    }
    fn raw(&self) -> *mut raw::git_writestream { self.raw }
}
impl<'repo> io::Write for BlobWriter<'repo> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        unsafe {
            let res = ((*self.raw).write)(self.raw, buf.as_ptr() as *const _, buf.len());
            if res < 0 {
                Err(io::Error::new(io::ErrorKind::Other, "Write error"))
            } else {
                Ok(buf.len())
            }
        }
    }
    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}
