use std::ffi::{CStr, OsString, CString};
use std::ops::Range;
use std::path::Path;
use std::ptr;
use std::slice;
use libc::{c_int, c_uint, size_t, c_void, c_char};
use {raw, panic, Repository, Error, Tree, Oid, IndexAddOption, IndexTime};
use IntoCString;
use util::{self, Binding};
pub struct Index {
    raw: *mut raw::git_index,
}
pub struct IndexEntries<'index> {
    range: Range<usize>,
    index: &'index Index,
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
    pub fn new() -> Result<Index, Error> {
        let mut raw = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(raw))
        }
    }
    pub fn open(index_path: &Path) -> Result<Index, Error> {
        let mut raw = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(raw))
        }
    }
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
            Ok(())
        }
    }
    pub fn add_frombuffer(&mut self, entry: &IndexEntry, data: &[u8]) -> Result<(), Error> {
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
    pub fn get(&self, n: usize) -> Option<IndexEntry> {
        unsafe {
            let ptr = raw::git_index_get_byindex(self.raw, n as size_t);
            if ptr.is_null() {None} else {Some(Binding::from_raw(*ptr))}
        }
    }
    pub fn get_path(&self, path: &Path, stage: i32) -> Option<IndexEntry> {
        let path = path.into_c_string().unwrap();
        unsafe {
            let ptr = call!(raw::git_index_get_bypath(self.raw, path,
                                                      stage as c_int));
            if ptr.is_null() {None} else {Some(Binding::from_raw(*ptr))}
        }
    }
    pub fn remove_all<T, I>(&mut self,
                            pathspecs: I,
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
            try_call!(raw::git_index_remove_all(self.raw,
                                                &raw_strarray,
                                                callback,
                                                ptr.map(|p| p as *mut _)
                                                   .unwrap_or(ptr::null_mut())
                                                        as *mut c_void));
        }
        Ok(())
    }
    pub fn update_all<T, I>(&mut self,
                            pathspecs: I,
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
            try_call!(raw::git_index_update_all(self.raw,
                                                &raw_strarray,
                                                callback,
                                                ptr.map(|p| p as *mut _)
                                                   .unwrap_or(ptr::null_mut())
                                                        as *mut c_void));
        }
        Ok(())
    }
    pub fn write_tree(&mut self) -> Result<Oid, Error> {
        let mut raw = raw::git_oid { id: [0; raw::GIT_OID_RAWSZ] };
        unsafe {
            Ok(Binding::from_raw(&raw as *const _))
        }
    }
}
impl Binding for Index {
    type Raw = *mut raw::git_index;
    unsafe fn from_raw(raw: *mut raw::git_index) -> Index {
        Index { raw: raw }
    }
    fn raw(&self) -> *mut raw::git_index { self.raw }
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
impl<'index> Iterator for IndexEntries<'index> {
    type Item = IndexEntry;
    fn next(&mut self) -> Option<IndexEntry> {
        self.range.next().map(|i| self.index.get(i).unwrap())
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
