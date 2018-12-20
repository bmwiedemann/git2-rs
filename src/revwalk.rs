use std::marker;
use {raw, Error, Sort, Oid, Repository};
use util::Binding;
pub struct Revwalk<'repo> {
    raw: *mut raw::git_revwalk,
    _marker: marker::PhantomData<&'repo Repository>,
}
impl<'repo> Revwalk<'repo> {
    pub fn push(&mut self, oid: Oid) -> Result<(), Error> {
        Ok(())
    }
    pub fn push_head(&mut self) -> Result<(), Error> {
        Ok(())
    }
    pub fn push_glob(&mut self, glob: &str) -> Result<(), Error> {
        Ok(())
    }
    pub fn push_range(&mut self, range: &str) -> Result<(), Error> {
        Ok(())
    }
    pub fn push_ref(&mut self, reference: &str) -> Result<(), Error> {
        Ok(())
    }
    pub fn hide(&mut self, oid: Oid) -> Result<(), Error> {
        Ok(())
    }
    pub fn hide_head(&mut self) -> Result<(), Error> {
        Ok(())
    }
    pub fn hide_glob(&mut self, glob: &str) -> Result<(), Error> {
        Ok(())
    }
    pub fn hide_ref(&mut self, reference: &str) -> Result<(), Error> {
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
    }
}
impl<'repo> Iterator for Revwalk<'repo> {
    type Item = Result<Oid, Error>;
    fn next(&mut self) -> Option<Result<Oid, Error>> {
        let mut out: raw::git_oid = raw::git_oid{ id: [0; raw::GIT_OID_RAWSZ] };
        unsafe {
            Some(Ok(Binding::from_raw(&out as *const _)))
        }
    }
}
