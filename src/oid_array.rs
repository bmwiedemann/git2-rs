use std::ops::Deref;
use oid::Oid;
use util::Binding;
use std::slice;
pub struct OidArray {
    raw: raw::git_oidarray,
}
impl Deref for OidArray {
    type Target = [Oid];
    fn deref(&self) -> &[Oid] {
        unsafe {
            slice::from_raw_parts(self.raw.ids as *const Oid, self.raw.count as usize)
        }
    }
}
impl Binding for OidArray {
    type Raw = raw::git_oidarray;
    unsafe fn from_raw(raw: raw::git_oidarray) -> OidArray {
        OidArray { raw: raw }
    }
    fn raw(&self) -> raw::git_oidarray { self.raw }
}
impl<'repo> ::std::fmt::Debug for OidArray {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
		f.debug_tuple("OidArray").field(&self.deref()).finish()
    }
}
