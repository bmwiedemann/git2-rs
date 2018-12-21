use std::fmt;
use std::str;
use util::Binding;
pub struct Oid {
    raw: raw::git_oid
}
impl Binding for Oid {
    type Raw = *const raw::git_oid;
    unsafe fn from_raw(oid: *const raw::git_oid) -> Oid {
        Oid { raw: *oid }
    }
    fn raw(&self) -> *const raw::git_oid { &self.raw as *const _ }
}
impl fmt::Display for Oid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut dst = [0u8; raw::GIT_OID_HEXSZ + 1];
        let s = &dst[..dst.iter().position(|&a| a == 0).unwrap()];
        str::from_utf8(s).unwrap().fmt(f)
    }
}
