use std::marker;
use std::io;
use std::ptr;
use std::slice;
use libc::{c_char, c_int, c_void, size_t};
use {raw, Oid, Object, ObjectType, Error};
use panic;
use util::Binding;
pub struct Odb<'repo> {
    raw: *mut raw::git_odb,
    _marker: marker::PhantomData<Object<'repo>>,
}
impl<'repo> Binding for Odb<'repo> {
    type Raw = *mut raw::git_odb;
    unsafe fn from_raw(raw: *mut raw::git_odb) -> Odb<'repo> {
        Odb {
            raw: raw,
            _marker: marker::PhantomData,
        }
    }
    fn raw(&self) -> *mut raw::git_odb { self.raw }
}
impl<'repo> Odb<'repo> {
    pub fn reader(&self, oid: Oid) -> Result<(OdbReader, usize, ObjectType), Error> {
        let mut out = ptr::null_mut();
        let mut size = 0usize;
        let mut otype: raw::git_otype = ObjectType::Any.raw();
        unsafe {
            Ok((OdbReader::from_raw(out), size, ObjectType::from_raw(otype).unwrap()))
        }
    }
    pub fn read_header(&self, oid: Oid) -> Result<(usize, ObjectType), Error> {
        let mut size: usize = 0;
        let mut kind_id: i32 = ObjectType::Any.raw();
        unsafe {
            Ok((size, ObjectType::from_raw(kind_id).unwrap()))
        }
    }
}
pub struct OdbObject<'a> {
    raw: *mut raw::git_odb_object,
    _marker: marker::PhantomData<Object<'a>>,
}
impl<'a> Binding for OdbObject<'a> {
    type Raw = *mut raw::git_odb_object;
    unsafe fn from_raw(raw: *mut raw::git_odb_object) -> OdbObject<'a> {
        OdbObject {
            raw: raw,
            _marker: marker::PhantomData,
        }
    }
    fn raw(&self) -> *mut raw::git_odb_object { self.raw }
}
impl<'a> OdbObject<'a> {
    pub fn len(&self) -> usize {
        unsafe { raw::git_odb_object_size(self.raw) }
    }
    pub fn data(&self) -> &[u8] {
        unsafe {
            let size = self.len();
            let ptr : *const u8 = raw::git_odb_object_data(self.raw) as *const u8;
            let buffer = slice::from_raw_parts(ptr, size);
            return buffer;
        }
    }
}
pub struct OdbReader<'repo> {
    raw: *mut raw::git_odb_stream,
    _marker: marker::PhantomData<Object<'repo>>,
}
impl<'repo> Binding for OdbReader<'repo> {
    type Raw = *mut raw::git_odb_stream;
    unsafe fn from_raw(raw: *mut raw::git_odb_stream) -> OdbReader<'repo> {
        OdbReader {
            raw: raw,
            _marker: marker::PhantomData,
        }
    }
    fn raw(&self) -> *mut raw::git_odb_stream { self.raw }
}
impl<'repo> io::Read for OdbReader<'repo> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        unsafe {
            let ptr = buf.as_ptr() as *mut c_char;
            let len = buf.len();
            let res = raw::git_odb_stream_read(self.raw, ptr, len);
            if res < 0 {
                Err(io::Error::new(io::ErrorKind::Other, "Read error"))
            } else {
                Ok(len)
            }
        }
    }
}
pub struct OdbWriter<'repo> {
    raw: *mut raw::git_odb_stream,
    _marker: marker::PhantomData<Object<'repo>>,
}
impl<'repo> OdbWriter<'repo> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        unsafe {
            let ptr = buf.as_ptr() as *const c_char;
            let len = buf.len();
            let res = raw::git_odb_stream_write(self.raw, ptr, len);
            if res < 0 {
                Err(io::Error::new(io::ErrorKind::Other, "Write error"))
            } else {
                Ok(buf.len())
            }
        }
    }
}
pub type ForeachCb<'a> = FnMut(&Oid) -> bool + 'a;
struct ForeachCbData<'a> {
    pub callback: &'a mut ForeachCb<'a>
}
extern fn foreach_cb(id: *const raw::git_oid,
                        payload: *mut c_void)
                        -> c_int
{
    panic::wrap(|| unsafe {
        let data = &mut *(payload as *mut ForeachCbData);
        let res = {
            let callback = &mut data.callback;
            callback(&Binding::from_raw(id))
        };
        if res { 0 } else { 1 }
    }).unwrap_or(1)
}
