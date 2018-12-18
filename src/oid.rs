use std::fmt;
use std::cmp::Ordering;
use std::hash::{Hasher, Hash};
use std::str;
use std::path::Path;
use libc;
use {raw, Error, ObjectType, IntoCString};
use util::Binding;
#[derive(Copy, Clone)]
pub struct Oid {
    raw: raw::git_oid
}
impl Oid {
    pub fn from_str(s: &str) -> Result<Oid, Error> {
        ::init();
        let mut raw = raw::git_oid { id: [0; raw::GIT_OID_RAWSZ] };
        unsafe {
            try_call!(raw::git_oid_fromstrn(&mut raw,
                                            s.as_bytes().as_ptr()
                                                as *const libc::c_char,
                                            s.len() as libc::size_t));
        }
        Ok(Oid { raw: raw })
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Oid, Error> {
        ::init();
        let mut raw = raw::git_oid { id: [0; raw::GIT_OID_RAWSZ] };
        if bytes.len() != raw::GIT_OID_RAWSZ {
            Err(Error::from_str("raw byte array must be 20 bytes"))
        } else {
            unsafe { raw::git_oid_fromraw(&mut raw, bytes.as_ptr()) }
            Ok(Oid { raw: raw })
        }
    }
    pub fn zero() -> Oid {
        let out = raw::git_oid { id: [0; raw::GIT_OID_RAWSZ] };
        Oid { raw: out }
    }
    pub fn hash_object(kind: ObjectType, bytes: &[u8]) -> Result<Oid, Error> {
        ::init();
        let mut out = raw::git_oid { id: [0; raw::GIT_OID_RAWSZ] };
        unsafe {
            try_call!(raw::git_odb_hash(&mut out,
                                        bytes.as_ptr()
                                            as *const libc::c_void,
                                        bytes.len(),
                                        kind.raw()));
        }
        Ok(Oid { raw: out })
    }
    pub fn hash_file<P: AsRef<Path>>(kind: ObjectType, path: P) -> Result<Oid, Error> {
        ::init();
        let rpath = try!(path.as_ref().into_c_string());
        let mut out = raw::git_oid { id: [0; raw::GIT_OID_RAWSZ] };
        unsafe {
            try_call!(raw::git_odb_hashfile(&mut out,
                                            rpath,
                                            kind.raw()));
        }
        Ok(Oid { raw: out })
    }
    pub fn as_bytes(&self) -> &[u8] { &self.raw.id }
    pub fn is_zero(&self) -> bool {
        unsafe { raw::git_oid_iszero(&self.raw) == 1 }
    }
}
impl Binding for Oid {
    type Raw = *const raw::git_oid;
    unsafe fn from_raw(oid: *const raw::git_oid) -> Oid {
        Oid { raw: *oid }
    }
    fn raw(&self) -> *const raw::git_oid { &self.raw as *const _ }
}
impl fmt::Debug for Oid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
impl fmt::Display for Oid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut dst = [0u8; raw::GIT_OID_HEXSZ + 1];
        unsafe {
            raw::git_oid_tostr(dst.as_mut_ptr() as *mut libc::c_char,
                               dst.len() as libc::size_t, &self.raw);
        }
        let s = &dst[..dst.iter().position(|&a| a == 0).unwrap()];
        str::from_utf8(s).unwrap().fmt(f)
    }
}
impl str::FromStr for Oid {
    type Err = Error;
    fn from_str(s: &str) -> Result<Oid, Error> {
        Oid::from_str(s)
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
            0 => Ordering::Equal,
            n if n < 0 => Ordering::Less,
            _ => Ordering::Greater,
        }
    }
}
impl Hash for Oid {
    fn hash<H: Hasher>(&self, into: &mut H) {
        self.raw.id.hash(into)
    }
}
impl AsRef<[u8]> for Oid {
    fn as_ref(&self) -> &[u8] { self.as_bytes() }
}
