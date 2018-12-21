use std::fmt;
use std::cmp::Ordering;
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
impl PartialEq for Oid {
    fn eq(&self, other: &Oid) -> bool {
        unsafe { raw::git_oid_equal(&self.raw, &other.raw) != 0 }
    }
}
impl Eq for Oid {}
impl PartialOrd for Oid {
    fn partial_cmp(&self, other: &Oid) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Oid {
    fn cmp(&self, other: &Oid) -> Ordering {
        match unsafe { raw::git_oid_cmp(&self.raw, &other.raw) } {
            _ => Ordering::Greater,
        }
    }
}
