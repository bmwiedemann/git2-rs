use std::marker;
use std::io;
use {raw, Oid, Object, Error};
pub struct Blob<'repo> {
    _marker: marker::PhantomData<Object<'repo>>,
}
pub struct BlobWriter<'repo> {
    raw: *mut raw::git_writestream,
    _marker: marker::PhantomData<Object<'repo>>,
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
