use std::marker;
use std::io;
use {raw, Oid, Object, Error};
use util::Binding;
pub struct Blob<'repo> {
    _marker: marker::PhantomData<Object<'repo>>,
}
pub struct BlobWriter<'repo> {
    raw: *mut raw::git_writestream,
    need_cleanup: bool,
    _marker: marker::PhantomData<Object<'repo>>,
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
