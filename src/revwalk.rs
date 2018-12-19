use std::marker;
use std::ffi::CString;
use libc::c_uint;
use {raw, Error, Sort, Oid, Repository};
use util::Binding;
pub struct Revwalk<'repo> {
    raw: *mut raw::git_revwalk,
    _marker: marker::PhantomData<&'repo Repository>,
}
impl<'repo> Revwalk<'repo> {
    pub fn reset(&mut self) {
        unsafe { raw::git_revwalk_reset(self.raw()) }
    }
    pub fn set_sorting(&mut self, sort_mode: Sort) {
        unsafe {
            raw::git_revwalk_sorting(self.raw(), sort_mode.bits() as c_uint)
        }
    }
    pub fn simplify_first_parent(&mut self) {
        unsafe { raw::git_revwalk_simplify_first_parent(self.raw) }
    }
    pub fn push(&mut self, oid: Oid) -> Result<(), Error> {
        unsafe {
            try_call!(raw::git_revwalk_push(self.raw(), oid.raw()));
        }
        Ok(())
    }
    pub fn push_head(&mut self) -> Result<(), Error> {
        unsafe {
            try_call!(raw::git_revwalk_push_head(self.raw()));
        }
        Ok(())
    }
    pub fn push_glob(&mut self, glob: &str) -> Result<(), Error> {
        let glob = try!(CString::new(glob));
        unsafe {
            try_call!(raw::git_revwalk_push_glob(self.raw, glob));
        }
        Ok(())
    }
    pub fn push_range(&mut self, range: &str) -> Result<(), Error> {
        let range = try!(CString::new(range));
        unsafe {
            try_call!(raw::git_revwalk_push_range(self.raw, range));
        }
        Ok(())
    }
    pub fn push_ref(&mut self, reference: &str) -> Result<(), Error> {
        let reference = try!(CString::new(reference));
        unsafe {
            try_call!(raw::git_revwalk_push_ref(self.raw, reference));
        }
        Ok(())
    }
    pub fn hide(&mut self, oid: Oid) -> Result<(), Error> {
        unsafe {
            try_call!(raw::git_revwalk_hide(self.raw(), oid.raw()));
        }
        Ok(())
    }
    pub fn hide_head(&mut self) -> Result<(), Error> {
        unsafe {
            try_call!(raw::git_revwalk_hide_head(self.raw()));
        }
        Ok(())
    }
    pub fn hide_glob(&mut self, glob: &str) -> Result<(), Error> {
        let glob = try!(CString::new(glob));
        unsafe {
            try_call!(raw::git_revwalk_hide_glob(self.raw, glob));
        }
        Ok(())
    }
    pub fn hide_ref(&mut self, reference: &str) -> Result<(), Error> {
        let reference = try!(CString::new(reference));
        unsafe {
            try_call!(raw::git_revwalk_hide_ref(self.raw, reference));
        }
        Ok(())
    }
}
impl<'repo> Binding for Revwalk<'repo> {
    type Raw = *mut raw::git_revwalk;
    unsafe fn from_raw(raw: *mut raw::git_revwalk) -> Revwalk<'repo> {
        Revwalk { raw: raw, _marker: marker::PhantomData }
    }
    fn raw(&self) -> *mut raw::git_revwalk { self.raw }
}
impl<'repo> Drop for Revwalk<'repo> {
    fn drop(&mut self) {
        unsafe { raw::git_revwalk_free(self.raw) }
    }
}
impl<'repo> Iterator for Revwalk<'repo> {
    type Item = Result<Oid, Error>;
    fn next(&mut self) -> Option<Result<Oid, Error>> {
        let mut out: raw::git_oid = raw::git_oid{ id: [0; raw::GIT_OID_RAWSZ] };
        unsafe {
            try_call_iter!(raw::git_revwalk_next(&mut out, self.raw()));
            Some(Ok(Binding::from_raw(&out as *const _)))
        }
    }
}
