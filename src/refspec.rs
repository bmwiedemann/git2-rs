use std::ffi::CString;
use std::marker;
use std::str;
use {raw, Direction};
use util::Binding;
pub struct Refspec<'remote> {
    raw: *const raw::git_refspec,
    _marker: marker::PhantomData<&'remote raw::git_remote>,
}
impl<'remote> Refspec<'remote> {
    pub fn direction(&self) -> Direction {
        match unsafe { raw::git_refspec_direction(self.raw) } {
            raw::GIT_DIRECTION_FETCH => Direction::Fetch,
            raw::GIT_DIRECTION_PUSH => Direction::Push,
            n => panic!("unknown refspec direction: {}", n),
        }
    }
    pub fn dst(&self) -> Option<&str> {
        str::from_utf8(self.dst_bytes()).ok()
    }
    pub fn dst_bytes(&self) -> &[u8] {
        unsafe { ::opt_bytes(self, raw::git_refspec_dst(self.raw)).unwrap() }
    }
    pub fn dst_matches(&self, refname: &str) -> bool {
        let refname = CString::new(refname).unwrap();
        unsafe { raw::git_refspec_dst_matches(self.raw, refname.as_ptr()) == 1 }
    }
    pub fn src(&self) -> Option<&str> {
        str::from_utf8(self.src_bytes()).ok()
    }
    pub fn src_bytes(&self) -> &[u8] {
        unsafe { ::opt_bytes(self, raw::git_refspec_src(self.raw)).unwrap() }
    }
    pub fn src_matches(&self, refname: &str) -> bool {
        let refname = CString::new(refname).unwrap();
        unsafe { raw::git_refspec_src_matches(self.raw, refname.as_ptr()) == 1 }
    }
    pub fn is_force(&self) -> bool {
        unsafe { raw::git_refspec_force(self.raw) == 1 }
    }
    pub fn str(&self) -> Option<&str> {
        str::from_utf8(self.bytes()).ok()
    }
    pub fn bytes(&self) -> &[u8] {
        unsafe { ::opt_bytes(self, raw::git_refspec_string(self.raw)).unwrap() }
    }
}
impl<'remote> Binding for Refspec<'remote> {
    type Raw = *const raw::git_refspec;
    unsafe fn from_raw(raw: *const raw::git_refspec) -> Refspec<'remote> {
        Refspec { raw: raw, _marker: marker::PhantomData }
    }
    fn raw(&self) -> *const raw::git_refspec { self.raw }
}
