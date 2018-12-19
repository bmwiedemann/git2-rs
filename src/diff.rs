use std::ffi::CString;
use std::marker;
use std::mem;
use std::ops::Range;
use std::ptr;
use std::slice;
use libc::{c_char, size_t, c_void, c_int};
use {raw, panic, Buf, Delta, Oid, Repository, Error, DiffFormat};
use util::{self, Binding};
pub struct Diff<'repo> {
    raw: *mut raw::git_diff,
    _marker: marker::PhantomData<&'repo Repository>,
}
pub struct DiffDelta<'a> {
    raw: *mut raw::git_diff_delta,
    _marker: marker::PhantomData<&'a raw::git_diff_delta>,
}
pub struct DiffFile<'a> {
    raw: *const raw::git_diff_file,
    _marker: marker::PhantomData<&'a raw::git_diff_file>,
}
pub struct DiffOptions {
    pathspec: Vec<CString>,
    pathspec_ptrs: Vec<*const c_char>,
    old_prefix: Option<CString>,
    new_prefix: Option<CString>,
    raw: raw::git_diff_options,
}
pub struct DiffFindOptions {
    raw: raw::git_diff_find_options,
}
pub struct Deltas<'diff> {
    range: Range<usize>,
    diff: &'diff Diff<'diff>,
}
pub struct DiffLine<'a> {
    raw: *const raw::git_diff_line,
    _marker: marker::PhantomData<&'a raw::git_diff_line>,
}
pub struct DiffHunk<'a> {
    raw: *const raw::git_diff_hunk,
    _marker: marker::PhantomData<&'a raw::git_diff_hunk>,
}
pub struct DiffStats {
    raw: *mut raw::git_diff_stats,
}
pub struct DiffBinary<'a> {
    raw: *const raw::git_diff_binary,
    _marker: marker::PhantomData<&'a raw::git_diff_binary>,
}
pub struct DiffBinaryFile<'a> {
    raw: *const raw::git_diff_binary_file,
    _marker: marker::PhantomData<&'a raw::git_diff_binary_file>,
}
pub enum DiffBinaryKind {
    None,
    Literal,
    Delta,
}
type PrintCb<'a> = FnMut(DiffDelta, Option<DiffHunk>, DiffLine) -> bool + 'a;
pub type FileCb<'a> = FnMut(DiffDelta, f32) -> bool + 'a;
pub type BinaryCb<'a> = FnMut(DiffDelta, DiffBinary) -> bool + 'a;
pub type HunkCb<'a> = FnMut(DiffDelta, DiffHunk) -> bool + 'a;
pub type LineCb<'a> = FnMut(DiffDelta, Option<DiffHunk>, DiffLine) -> bool + 'a;
struct ForeachCallbacks<'a, 'b: 'a, 'c, 'd: 'c, 'e, 'f: 'e, 'g, 'h: 'g> {
    file: &'a mut FileCb<'b>,
    binary: Option<&'c mut BinaryCb<'d>>,
    hunk: Option<&'e mut HunkCb<'f>>,
    line: Option<&'g mut LineCb<'h>>,
}
impl<'repo> Diff<'repo> {
    pub fn get_delta(&self, i: usize) -> Option<DiffDelta> {
        unsafe {
            let ptr = raw::git_diff_get_delta(&*self.raw, i as size_t);
            Binding::from_raw_opt(ptr as *mut _)
        }
    }
    pub fn print<F>(&self, format: DiffFormat, mut cb: F) -> Result<(), Error>
                    where F: FnMut(DiffDelta,
                                   DiffLine) -> bool {
        unsafe {
            Ok(())
        }
    }
    pub fn foreach(&self,
                   file_cb: &mut FileCb,
                   binary_cb: Option<&mut BinaryCb>,
                   hunk_cb: Option<&mut HunkCb>,
                   line_cb: Option<&mut LineCb>) -> Result<(), Error> {
        let mut cbs = ForeachCallbacks {
            file: file_cb,
            binary: binary_cb,
            hunk: hunk_cb,
            line: line_cb,
        };
        unsafe {
            Ok(())
        }
    }
    pub fn stats(&self) -> Result<DiffStats, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(ret))
        }
    }
}
pub extern fn print_cb(delta: *const raw::git_diff_delta,
                   hunk: *const raw::git_diff_hunk,
                   line: *const raw::git_diff_line,
                   data: *mut c_void) -> c_int {
    unsafe {
        let delta = Binding::from_raw(delta as *mut _);
        let hunk = Binding::from_raw_opt(hunk);
        let line = Binding::from_raw(line);
        let r = panic::wrap(|| {
            let data = data as *mut &mut PrintCb;
            (*data)(delta, hunk, line)
        });
        if r == Some(true) {0} else {-1}
    }
}
extern fn file_cb_c(delta: *const raw::git_diff_delta,
                    progress: f32,
                    data: *mut c_void) -> c_int {
    unsafe {
        let delta = Binding::from_raw(delta as *mut _);
        let r = panic::wrap(|| {
            let cbs = data as *mut ForeachCallbacks;
            ((*cbs).file)(delta, progress)
        });
        if r == Some(true) {0} else {-1}
    }
}
extern fn binary_cb_c(delta: *const raw::git_diff_delta,
                      binary: *const raw::git_diff_binary,
                      data: *mut c_void) -> c_int {
    unsafe {
        let delta = Binding::from_raw(delta as *mut _);
        let binary = Binding::from_raw(binary);
        let r = panic::wrap(|| {
            let cbs = data as *mut ForeachCallbacks;
            match (*cbs).binary {
                Some(ref mut cb) => cb(delta, binary),
                None => false,
            }
        });
        if r == Some(true) {0} else {-1}
    }
}
extern fn hunk_cb_c(delta: *const raw::git_diff_delta,
                    hunk: *const raw::git_diff_hunk,
                    data: *mut c_void) -> c_int {
    unsafe {
        let delta = Binding::from_raw(delta as *mut _);
        let hunk = Binding::from_raw(hunk);
        let r = panic::wrap(|| {
            let cbs = data as *mut ForeachCallbacks;
            match (*cbs).hunk {
                Some(ref mut cb) => cb(delta, hunk),
                None => false,
            }
        });
        if r == Some(true) {0} else {-1}
    }
}
extern fn line_cb_c(delta: *const raw::git_diff_delta,
                    hunk: *const raw::git_diff_hunk,
                    line: *const raw::git_diff_line,
                    data: *mut c_void) -> c_int {
    unsafe {
        let delta = Binding::from_raw(delta as *mut _);
        let hunk = Binding::from_raw_opt(hunk);
        let line = Binding::from_raw(line);
        let r = panic::wrap(|| {
            let cbs = data as *mut ForeachCallbacks;
            match (*cbs).line {
                Some(ref mut cb) => cb(delta, hunk, line),
                None => false,
            }
        });
        if r == Some(true) {0} else {-1}
    }
}
impl<'repo> Binding for Diff<'repo> {
    type Raw = *mut raw::git_diff;
    unsafe fn from_raw(raw: *mut raw::git_diff) -> Diff<'repo> {
        Diff {
          raw: raw,
          _marker: marker::PhantomData,
        }
    }
    fn raw(&self) -> *mut raw::git_diff { self.raw }
}
impl<'a> Binding for DiffDelta<'a> {
    type Raw = *mut raw::git_diff_delta;
    unsafe fn from_raw(raw: *mut raw::git_diff_delta) -> DiffDelta<'a> {
        DiffDelta {
            raw: raw,
            _marker: marker::PhantomData,
        }
    }
    fn raw(&self) -> *mut raw::git_diff_delta { self.raw }
}
impl<'a> Binding for DiffFile<'a> {
    type Raw = *const raw::git_diff_file;
    unsafe fn from_raw(raw: *const raw::git_diff_file) -> DiffFile<'a> {
        DiffFile {
            raw: raw,
            _marker: marker::PhantomData,
        }
    }
    fn raw(&self) -> *const raw::git_diff_file { self.raw }
}
impl DiffOptions {
    pub fn new() -> DiffOptions {
        let mut opts = DiffOptions {
            pathspec: Vec::new(),
            pathspec_ptrs: Vec::new(),
            raw: unsafe { mem::zeroed() },
            old_prefix: None,
            new_prefix: None,
        };
        assert_eq!(unsafe {
            raw::git_diff_init_options(&mut opts.raw, 1)
        }, 0);
        opts
    }
}
impl<'diff> Iterator for Deltas<'diff> {
    type Item = DiffDelta<'diff>;
    fn next(&mut self) -> Option<DiffDelta<'diff>> {
        self.range.next().and_then(|i| self.diff.get_delta(i))
    }
}
impl<'a> DiffLine<'a> {
    pub fn content(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts((*self.raw).content as *const u8,
                                  (*self.raw).content_len as usize)
        }
    }
}
impl<'a> Binding for DiffLine<'a> {
    type Raw = *const raw::git_diff_line;
    unsafe fn from_raw(raw: *const raw::git_diff_line) -> DiffLine<'a> {
        DiffLine {
            raw: raw,
            _marker: marker::PhantomData,
        }
    }
    fn raw(&self) -> *const raw::git_diff_line { self.raw }
}
impl<'a> DiffHunk<'a> {
    pub fn header(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts((*self.raw).header.as_ptr() as *const u8,
                                  (*self.raw).header_len as usize)
        }
    }
}
impl<'a> Binding for DiffHunk<'a> {
    type Raw = *const raw::git_diff_hunk;
    unsafe fn from_raw(raw: *const raw::git_diff_hunk) -> DiffHunk<'a> {
        DiffHunk {
            raw: raw,
            _marker: marker::PhantomData,
        }
    }
    fn raw(&self) -> *const raw::git_diff_hunk { self.raw }
}
impl Binding for DiffStats {
    type Raw = *mut raw::git_diff_stats;
    unsafe fn from_raw(raw: *mut raw::git_diff_stats) -> DiffStats {
        DiffStats { raw: raw }
    }
    fn raw(&self) -> *mut raw::git_diff_stats { self.raw }
}
impl<'a> Binding for DiffBinary<'a> {
    type Raw = *const raw::git_diff_binary;
    unsafe fn from_raw(raw: *const raw::git_diff_binary) -> DiffBinary<'a> {
        DiffBinary {
            raw: raw,
            _marker: marker::PhantomData,
        }
    }
    fn raw(&self) -> *const raw::git_diff_binary { self.raw }
}
impl<'a> DiffBinaryFile<'a> {
    pub fn data(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts((*self.raw).data as *const u8,
                                  (*self.raw).datalen as usize)
        }
    }
    unsafe fn from_raw(raw: *const raw::git_diff_binary_file) -> DiffBinaryFile<'a> {
        DiffBinaryFile {
            raw: raw,
            _marker: marker::PhantomData,
        }
    }
}
impl Binding for DiffBinaryKind {
    type Raw = raw::git_diff_binary_t;
    unsafe fn from_raw(raw: raw::git_diff_binary_t) -> DiffBinaryKind {
        match raw {
            _ => panic!("Unknown git diff binary kind"),
        }
    }
    fn raw(&self) -> raw::git_diff_binary_t {
        match *self {
            DiffBinaryKind::None => raw::GIT_DIFF_BINARY_NONE,
            DiffBinaryKind::Literal => raw::GIT_DIFF_BINARY_LITERAL,
            DiffBinaryKind::Delta => raw::GIT_DIFF_BINARY_DELTA,
        }
    }
}
impl DiffFindOptions {
    pub fn new() -> DiffFindOptions {
        let mut opts = DiffFindOptions {
            raw: unsafe { mem::zeroed() },
        };
        opts
    }
    fn flag(&mut self, opt: u32, val: bool) -> &mut DiffFindOptions {
        self
    }
    pub fn break_rewrites_for_renames_only(&mut self, b: bool)
                                           -> &mut DiffFindOptions {
        self.flag(raw::GIT_DIFF_BREAK_REWRITES_FOR_RENAMES_ONLY, b)
    }
}
