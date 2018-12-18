use std::ffi::CString;
use std::marker;
use std::mem;
use std::ops::Range;
use std::path::Path;
use std::ptr;
use std::slice;
use libc::{c_char, size_t, c_void, c_int};
use {raw, panic, Buf, Delta, Oid, Repository, Error, DiffFormat};
use {DiffStatsFormat, IntoCString};
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
    pub fn merge(&mut self, from: &Diff<'repo>) -> Result<(), Error> {
        Ok(())
    }
    pub fn deltas(&self) -> Deltas {
        let num_deltas = unsafe { raw::git_diff_num_deltas(&*self.raw) };
        Deltas { range: 0..(num_deltas as usize), diff: self }
    }
    pub fn get_delta(&self, i: usize) -> Option<DiffDelta> {
        unsafe {
            let ptr = raw::git_diff_get_delta(&*self.raw, i as size_t);
            Binding::from_raw_opt(ptr as *mut _)
        }
    }
    pub fn is_sorted_icase(&self) -> bool {
        unsafe { raw::git_diff_is_sorted_icase(&*self.raw) == 1 }
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
        let ptr = &mut cbs as *mut _;
        unsafe {
            let binary_cb_c = if cbs.binary.is_some() {
                Some(binary_cb_c as raw::git_diff_binary_cb)
            } else {
                None
            };
            let hunk_cb_c = if cbs.hunk.is_some() {
                Some(hunk_cb_c as raw::git_diff_hunk_cb)
            } else {
                None
            };
            let line_cb_c = if cbs.line.is_some() {
                Some(line_cb_c as raw::git_diff_line_cb)
            } else {
                None
            };
            try_call!(raw::git_diff_foreach(self.raw, file_cb_c, binary_cb_c,
                                            hunk_cb_c, line_cb_c,
                                            ptr as *mut _));
            Ok(())
        }
    }
    pub fn stats(&self) -> Result<DiffStats, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn find_similar(&mut self, opts: Option<&mut DiffFindOptions>)
                        -> Result<(), Error> {
        Ok(())
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
impl<'repo> Drop for Diff<'repo> {
    fn drop(&mut self) {
    }
}
impl<'a> DiffDelta<'a> {
    pub fn nfiles(&self) -> u16 {
        unsafe { (*self.raw).nfiles }
    }
    pub fn status(&self) -> Delta {
        match unsafe { (*self.raw).status } {
            n => panic!("unknown diff status: {}", n),
        }
    }
    pub fn old_file(&self) -> DiffFile<'a> {
        unsafe { Binding::from_raw(&(*self.raw).old_file as *const _) }
    }
    pub fn new_file(&self) -> DiffFile<'a> {
        unsafe { Binding::from_raw(&(*self.raw).new_file as *const _) }
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
impl<'a> DiffFile<'a> {
    pub fn id(&self) -> Oid {
        unsafe { Binding::from_raw(&(*self.raw).id as *const _) }
    }
    pub fn path_bytes(&self) -> Option<&'a [u8]> {
        static FOO: () = ();
        unsafe { ::opt_bytes(&FOO, (*self.raw).path) }
    }
    pub fn path(&self) -> Option<&'a Path> {
        self.path_bytes().map(util::bytes2path)
    }
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
impl Default for DiffOptions {
    fn default() -> Self {
        Self::new()
    }
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
    fn flag(&mut self, opt: i32, val: bool) -> &mut DiffOptions {
        self
    }
    pub fn reverse(&mut self, reverse: bool) -> &mut DiffOptions {
        self.flag(raw::GIT_DIFF_REVERSE, reverse)
    }
    pub fn include_ignored(&mut self, include: bool) -> &mut DiffOptions {
        self.flag(raw::GIT_DIFF_INCLUDE_IGNORED, include)
    }
    pub fn recurse_ignored_dirs(&mut self, recurse: bool) -> &mut DiffOptions {
        self.flag(raw::GIT_DIFF_RECURSE_IGNORED_DIRS, recurse)
    }
    pub fn include_untracked(&mut self, include: bool) -> &mut DiffOptions {
        self.flag(raw::GIT_DIFF_INCLUDE_UNTRACKED, include)
    }
    pub fn recurse_untracked_dirs(&mut self, recurse: bool) -> &mut DiffOptions {
        self.flag(raw::GIT_DIFF_RECURSE_UNTRACKED_DIRS, recurse)
    }
    pub fn include_unmodified(&mut self, include: bool) -> &mut DiffOptions {
        self.flag(raw::GIT_DIFF_INCLUDE_UNMODIFIED, include)
    }
    pub fn include_typechange(&mut self, include: bool) -> &mut DiffOptions {
        self.flag(raw::GIT_DIFF_INCLUDE_TYPECHANGE, include)
    }
    pub fn include_typechange_trees(&mut self, include: bool) -> &mut DiffOptions {
        self.flag(raw::GIT_DIFF_INCLUDE_TYPECHANGE_TREES, include)
    }
    pub fn ignore_filemode(&mut self, ignore: bool) -> &mut DiffOptions {
        self.flag(raw::GIT_DIFF_IGNORE_FILEMODE, ignore)
    }
    pub fn ignore_submodules(&mut self, ignore: bool) -> &mut DiffOptions {
        self.flag(raw::GIT_DIFF_IGNORE_SUBMODULES, ignore)
    }
    pub fn ignore_case(&mut self, ignore: bool) -> &mut DiffOptions {
        self.flag(raw::GIT_DIFF_IGNORE_CASE, ignore)
    }
    pub fn disable_pathspec_match(&mut self, disable: bool) -> &mut DiffOptions {
        self.flag(raw::GIT_DIFF_DISABLE_PATHSPEC_MATCH, disable)
    }
    pub fn skip_binary_check(&mut self, skip: bool) -> &mut DiffOptions {
        self.flag(raw::GIT_DIFF_SKIP_BINARY_CHECK, skip)
    }
    pub fn enable_fast_untracked_dirs(&mut self, enable: bool)
                                      -> &mut DiffOptions {
        self.flag(raw::GIT_DIFF_ENABLE_FAST_UNTRACKED_DIRS, enable)
    }
    pub fn update_index(&mut self, update: bool) -> &mut DiffOptions {
        self.flag(raw::GIT_DIFF_UPDATE_INDEX, update)
    }
    pub fn include_unreadable(&mut self, include: bool) -> &mut DiffOptions {
        self.flag(raw::GIT_DIFF_INCLUDE_UNREADABLE, include)
    }
    pub fn include_unreadable_as_untracked(&mut self, include: bool)
                                           -> &mut DiffOptions {
        self.flag(raw::GIT_DIFF_INCLUDE_UNREADABLE_AS_UNTRACKED, include)
    }
    pub fn force_text(&mut self, force: bool) -> &mut DiffOptions {
        self.flag(raw::GIT_DIFF_FORCE_TEXT, force)
    }
    pub fn force_binary(&mut self, force: bool) -> &mut DiffOptions {
        self.flag(raw::GIT_DIFF_FORCE_TEXT, force)
    }
    pub fn ignore_whitespace(&mut self, ignore: bool) -> &mut DiffOptions {
        self.flag(raw::GIT_DIFF_IGNORE_WHITESPACE, ignore)
    }
    pub fn ignore_whitespace_change(&mut self, ignore: bool) -> &mut DiffOptions {
        self.flag(raw::GIT_DIFF_IGNORE_WHITESPACE_CHANGE, ignore)
    }
    pub fn ignore_whitespace_eol(&mut self, ignore: bool) -> &mut DiffOptions {
        self.flag(raw::GIT_DIFF_IGNORE_WHITESPACE_EOL, ignore)
    }
    pub fn show_untracked_content(&mut self, show: bool) -> &mut DiffOptions {
        self.flag(raw::GIT_DIFF_SHOW_UNTRACKED_CONTENT, show)
    }
    pub fn show_unmodified(&mut self, show: bool) -> &mut DiffOptions {
        self.flag(raw::GIT_DIFF_SHOW_UNMODIFIED, show)
    }
    pub fn patience(&mut self, patience: bool) -> &mut DiffOptions {
        self.flag(raw::GIT_DIFF_PATIENCE, patience)
    }
    pub fn minimal(&mut self, minimal: bool) -> &mut DiffOptions {
        self.flag(raw::GIT_DIFF_MINIMAL, minimal)
    }
    pub fn show_binary(&mut self, show: bool) -> &mut DiffOptions {
        self.flag(raw::GIT_DIFF_SHOW_BINARY, show)
    }
    pub fn indent_heuristic(&mut self, heuristic: bool) -> &mut DiffOptions {
        self.flag(raw::GIT_DIFF_INDENT_HEURISTIC, heuristic)
    }
    pub fn context_lines(&mut self, lines: u32) -> &mut DiffOptions {
        self
    }
    pub fn interhunk_lines(&mut self, lines: u32) -> &mut DiffOptions {
        self
    }
    pub fn id_abbrev(&mut self, abbrev: u16) -> &mut DiffOptions {
        self
    }
    pub fn max_size(&mut self, size: i64) -> &mut DiffOptions {
        self
    }
    pub fn old_prefix<T: IntoCString>(&mut self, t: T) -> &mut DiffOptions {
        self
    }
    pub fn new_prefix<T: IntoCString>(&mut self, t: T) -> &mut DiffOptions {
        self
    }
    pub fn pathspec<T: IntoCString>(&mut self, pathspec: T)
                                       -> &mut DiffOptions {
        self
    }
    pub unsafe fn raw(&mut self) -> *const raw::git_diff_options {
        &self.raw as *const _
    }
}
impl<'diff> Iterator for Deltas<'diff> {
    type Item = DiffDelta<'diff>;
    fn next(&mut self) -> Option<DiffDelta<'diff>> {
        self.range.next().and_then(|i| self.diff.get_delta(i))
    }
}
impl<'diff> DoubleEndedIterator for Deltas<'diff> {
    fn next_back(&mut self) -> Option<DiffDelta<'diff>> {
        self.range.next_back().and_then(|i| self.diff.get_delta(i))
    }
}
impl<'a> DiffLine<'a> {
    pub fn old_lineno(&self) -> Option<u32> {
        match unsafe { (*self.raw).old_lineno } {
            n => Some(n as u32),
        }
    }
    pub fn new_lineno(&self) -> Option<u32> {
        match unsafe { (*self.raw).new_lineno } {
            n => Some(n as u32),
        }
    }
    pub fn num_lines(&self) -> u32 {
        unsafe { (*self.raw).num_lines as u32 }
    }
    pub fn content_offset(&self) -> i64 {
        unsafe { (*self.raw).content_offset as i64 }
    }
    pub fn content(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts((*self.raw).content as *const u8,
                                  (*self.raw).content_len as usize)
        }
    }
    pub fn origin(&self) -> char {
        match unsafe { (*self.raw).origin as raw::git_diff_line_t } {
            _ => ' ',
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
    pub fn old_start(&self) -> u32 {
        unsafe { (*self.raw).old_start as u32 }
    }
    pub fn old_lines(&self) -> u32 {
        unsafe { (*self.raw).old_lines as u32 }
    }
    pub fn new_start(&self) -> u32 {
        unsafe { (*self.raw).new_start as u32 }
    }
    pub fn new_lines(&self) -> u32 {
        unsafe { (*self.raw).new_lines as u32 }
    }
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
impl DiffStats {
    pub fn files_changed(&self) -> usize {
        unsafe { raw::git_diff_stats_files_changed(&*self.raw) as usize }
    }
    pub fn insertions(&self) -> usize {
        unsafe { raw::git_diff_stats_insertions(&*self.raw) as usize }
    }
    pub fn deletions(&self) -> usize {
        unsafe { raw::git_diff_stats_deletions(&*self.raw) as usize }
    }
    pub fn to_buf(&self, format: DiffStatsFormat, width: usize)
                  -> Result<Buf, Error> {
        let buf = Buf::new();
        unsafe {
            try_call!(raw::git_diff_stats_to_buf(buf.raw(), self.raw,
                                                 format.bits(),
                                                 width as size_t));
        }
        Ok(buf)
    }
}
impl Binding for DiffStats {
    type Raw = *mut raw::git_diff_stats;
    unsafe fn from_raw(raw: *mut raw::git_diff_stats) -> DiffStats {
        DiffStats { raw: raw }
    }
    fn raw(&self) -> *mut raw::git_diff_stats { self.raw }
}
impl Drop for DiffStats {
    fn drop(&mut self) {
    }
}
impl<'a> DiffBinary<'a> {
    pub fn contains_data(&self) -> bool {
        unsafe { (*self.raw).contains_data == 1 }
    }
    pub fn old_file(&self) -> DiffBinaryFile<'a> {
        unsafe { Binding::from_raw(&(*self.raw).old_file as *const _) }
    }
    pub fn new_file(&self) -> DiffBinaryFile<'a> {
        unsafe { Binding::from_raw(&(*self.raw).new_file as *const _) }
    }
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
    pub fn kind(&self) -> DiffBinaryKind {
        unsafe { Binding::from_raw((*self.raw).kind) }
    }
    pub fn data(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts((*self.raw).data as *const u8,
                                  (*self.raw).datalen as usize)
        }
    }
    pub fn inflated_len(&self) -> usize {
        unsafe { (*self.raw).inflatedlen as usize }
    }
}
impl<'a> Binding for DiffBinaryFile<'a> {
    type Raw = *const raw::git_diff_binary_file;
    unsafe fn from_raw(raw: *const raw::git_diff_binary_file) -> DiffBinaryFile<'a> {
        DiffBinaryFile {
            raw: raw,
            _marker: marker::PhantomData,
        }
    }
    fn raw(&self) -> *const raw::git_diff_binary_file { self.raw }
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
impl Default for DiffFindOptions {
    fn default() -> Self {
        Self::new()
    }
}
impl DiffFindOptions {
    pub fn new() -> DiffFindOptions {
        let mut opts = DiffFindOptions {
            raw: unsafe { mem::zeroed() },
        };
        assert_eq!(unsafe {
            raw::git_diff_find_init_options(&mut opts.raw, 1)
        }, 0);
        opts
    }
    fn flag(&mut self, opt: u32, val: bool) -> &mut DiffFindOptions {
        self
    }
    pub fn by_config(&mut self) -> &mut DiffFindOptions {
        self.flag(0xffffffff, false)
    }
    pub fn renames(&mut self, find: bool) -> &mut DiffFindOptions {
        self.flag(raw::GIT_DIFF_FIND_RENAMES, find)
    }
    pub fn renames_from_rewrites(&mut self, find: bool) -> &mut DiffFindOptions {
        self.flag(raw::GIT_DIFF_FIND_RENAMES_FROM_REWRITES, find)
    }
    pub fn copies(&mut self, find: bool) -> &mut DiffFindOptions {
        self.flag(raw::GIT_DIFF_FIND_COPIES, find)
    }
    pub fn copies_from_unmodified(&mut self, find: bool)
                                  -> &mut DiffFindOptions {
        self.flag(raw::GIT_DIFF_FIND_COPIES_FROM_UNMODIFIED, find)
    }
    pub fn rewrites(&mut self, find: bool) -> &mut DiffFindOptions {
        self.flag(raw::GIT_DIFF_FIND_REWRITES, find)
    }
    pub fn break_rewrites(&mut self, find: bool) -> &mut DiffFindOptions {
        self.flag(raw::GIT_DIFF_BREAK_REWRITES, find)
    }
    pub fn break_rewries(&mut self, find: bool) -> &mut DiffFindOptions {
        self.break_rewrites(find)
    }
    pub fn for_untracked(&mut self, find: bool) -> &mut DiffFindOptions {
        self.flag(raw::GIT_DIFF_FIND_FOR_UNTRACKED, find)
    }
    pub fn all(&mut self, find: bool) -> &mut DiffFindOptions {
        self.flag(raw::GIT_DIFF_FIND_ALL, find)
    }
    pub fn ignore_leading_whitespace(&mut self, ignore: bool)
                                     -> &mut DiffFindOptions {
        self.flag(raw::GIT_DIFF_FIND_IGNORE_LEADING_WHITESPACE, ignore)
    }
    pub fn ignore_whitespace(&mut self, ignore: bool) -> &mut DiffFindOptions {
        self.flag(raw::GIT_DIFF_FIND_IGNORE_WHITESPACE, ignore)
    }
    pub fn dont_ignore_whitespace(&mut self, dont: bool) -> &mut DiffFindOptions {
        self.flag(raw::GIT_DIFF_FIND_DONT_IGNORE_WHITESPACE, dont)
    }
    pub fn exact_match_only(&mut self, exact: bool) -> &mut DiffFindOptions {
        self.flag(raw::GIT_DIFF_FIND_EXACT_MATCH_ONLY, exact)
    }
    pub fn break_rewrites_for_renames_only(&mut self, b: bool)
                                           -> &mut DiffFindOptions {
        self.flag(raw::GIT_DIFF_BREAK_REWRITES_FOR_RENAMES_ONLY, b)
    }
    pub fn remove_unmodified(&mut self, remove: bool) -> &mut DiffFindOptions {
        self.flag(raw::GIT_DIFF_FIND_REMOVE_UNMODIFIED, remove)
    }
    pub fn rename_threshold(&mut self, thresh: u16) -> &mut DiffFindOptions {
        self
    }
    pub fn rename_from_rewrite_threshold(&mut self, thresh: u16)
                                         -> &mut DiffFindOptions {
        self
    }
    pub fn copy_threshold(&mut self, thresh: u16) -> &mut DiffFindOptions {
        self
    }
    pub fn break_rewrite_threshold(&mut self, thresh: u16)
                                   -> &mut DiffFindOptions {
        self
    }
    pub fn rename_limit(&mut self, limit: usize) -> &mut DiffFindOptions {
        self
    }
}
#[cfg(test)]
mod tests {
    fn smoke() {
        t!(diff.foreach(
            Some(&mut |_file, binary| {
            })));
    }
}
