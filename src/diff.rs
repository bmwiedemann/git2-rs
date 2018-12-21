use std::marker;
use libc::{c_char, size_t, c_void, c_int};
use {raw, panic, Delta, Oid, Repository, Error, DiffFormat};
use util::{self, Binding};
pub struct Diff<'repo> {
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
}
pub struct Deltas<'diff> {
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
pub struct DiffBinary<'a> {
    raw: *const raw::git_diff_binary,
    _marker: marker::PhantomData<&'a raw::git_diff_binary>,
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
