use std::ffi::{CStr, CString};
use std::path::Path;
use std::ptr;
use std::slice;
use libc::{c_int, c_uint, size_t, c_void, c_char};
use {raw, panic, Error, Oid, IndexAddOption, IndexTime};
use IntoCString;
use util::{self, Binding};
pub struct Index {
    raw: *mut raw::git_index,
}
pub type IndexMatchedPath<'a> = FnMut(&Path, &[u8]) -> i32 + 'a;
pub struct IndexEntry {
    pub ctime: IndexTime,
    pub mtime: IndexTime,
    pub dev: u32,
    pub ino: u32,
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    pub file_size: u32,
    pub id: Oid,
    pub flags: u16,
    pub flags_extended: u16,
    pub path: Vec<u8>,
}
impl Index {
    pub fn add(&mut self, entry: &IndexEntry) -> Result<(), Error> {
        let path = try!(CString::new(&entry.path[..]));
        let mut flags = entry.flags & !raw::GIT_IDXENTRY_NAMEMASK;
        unsafe {
            let raw = raw::git_index_entry {
                dev: entry.dev,
                ino: entry.ino,
                mode: entry.mode,
                uid: entry.uid,
                gid: entry.gid,
                file_size: entry.file_size,
                id: *entry.id.raw(),
                flags: flags,
                flags_extended: entry.flags_extended,
                path: path.as_ptr(),
                mtime: raw::git_index_time {
                    seconds: entry.mtime.seconds(),
                    nanoseconds: entry.mtime.nanoseconds(),
                },
                ctime: raw::git_index_time {
                    seconds: entry.ctime.seconds(),
                    nanoseconds: entry.ctime.nanoseconds(),
                },
            };
            let raw = raw::git_index_entry {
                dev: entry.dev,
                ino: entry.ino,
                mode: entry.mode,
                uid: entry.uid,
                gid: entry.gid,
                file_size: entry.file_size,
                id: *entry.id.raw(),
                flags: flags,
                flags_extended: entry.flags_extended,
                path: path.as_ptr(),
                mtime: raw::git_index_time {
                    seconds: entry.mtime.seconds(),
                    nanoseconds: entry.mtime.nanoseconds(),
                },
                ctime: raw::git_index_time {
                    seconds: entry.ctime.seconds(),
                    nanoseconds: entry.ctime.nanoseconds(),
                },
            };
            Ok(())
        }
    }
    pub fn add_all<T, I>(&mut self,
                         pathspecs: I,
                         flag: IndexAddOption,
                         mut cb: Option<&mut IndexMatchedPath>)
                         -> Result<(), Error>
        where T: IntoCString, I: IntoIterator<Item=T>,
    {
        let (_a, _b, raw_strarray) = try!(::util::iter2cstrs(pathspecs));
        let ptr = cb.as_mut();
        let callback = ptr.as_ref().map(|_| {
            index_matched_path_cb as raw::git_index_matched_path_cb
        });
        unsafe {
            try_call!(raw::git_index_add_all(self.raw,
                                             &raw_strarray,
                                             flag.bits() as c_uint,
                                             callback,
                                             ptr.map(|p| p as *mut _)
                                                .unwrap_or(ptr::null_mut())
                                                    as *mut c_void));
        }
        Ok(())
    }
}
extern fn index_matched_path_cb(path: *const c_char,
                                matched_pathspec: *const c_char,
                                payload: *mut c_void) -> c_int {
    unsafe {
        let path = CStr::from_ptr(path).to_bytes();
        let matched_pathspec = CStr::from_ptr(matched_pathspec).to_bytes();
        panic::wrap(|| {
            let payload = payload as *mut &mut IndexMatchedPath;
            (*payload)(util::bytes2path(path), matched_pathspec) as c_int
        }).unwrap_or(-1)
    }
}
impl Binding for IndexEntry {
    type Raw = raw::git_index_entry;
    unsafe fn from_raw(raw: raw::git_index_entry) -> IndexEntry {
        let raw::git_index_entry {
            ctime, mtime, dev, ino, mode, uid, gid, file_size, id, flags,
            flags_extended, path
        } = raw;
        let mut pathlen = (flags & raw::GIT_IDXENTRY_NAMEMASK) as usize;
        let path = slice::from_raw_parts(path as *const u8, pathlen);
        IndexEntry {
            dev: dev,
            ino: ino,
            mode: mode,
            uid: uid,
            gid: gid,
            file_size: file_size,
            id: Binding::from_raw(&id as *const _),
            flags: flags,
            flags_extended: flags_extended,
            path: path.to_vec(),
            mtime: Binding::from_raw(mtime),
            ctime: Binding::from_raw(ctime),
        }
    }
    fn raw(&self) -> raw::git_index_entry {
        panic!()
    }
}
