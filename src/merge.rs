use std::marker;
use std::mem;
use libc::c_uint;
use {raw, Oid, Commit, FileFavor};
use util::Binding;
use call::Convert;
pub struct AnnotatedCommit<'repo> {
    raw: *mut raw::git_annotated_commit,
    _marker: marker::PhantomData<Commit<'repo>>,
}
pub struct MergeOptions {
    raw: raw::git_merge_options,
}
impl<'repo> AnnotatedCommit<'repo> {
    pub fn id(&self) -> Oid {
        unsafe { Binding::from_raw(raw::git_annotated_commit_id(self.raw)) }
    }
}
impl Default for MergeOptions {
    fn default() -> Self {
        Self::new()
    }
}
impl MergeOptions {
    pub fn new() -> MergeOptions {
        let mut opts = MergeOptions {
            raw: unsafe { mem::zeroed() },
        };
        assert_eq!(unsafe {
            raw::git_merge_init_options(&mut opts.raw, 1)
        }, 0);
        opts
    }
    fn flag(&mut self, opt: raw::git_merge_flag_t, val: bool) -> &mut MergeOptions {
        if val {
            self.raw.flags |= opt;
        }
        self
    }
    pub fn find_renames(&mut self, find: bool) -> &mut MergeOptions {
        self.flag(raw::GIT_MERGE_FIND_RENAMES, find)
    }
    pub fn fail_on_conflict(&mut self, fail: bool) -> &mut MergeOptions {
        self.flag(raw::GIT_MERGE_FAIL_ON_CONFLICT, fail)
    }
    pub fn skip_reuc(&mut self, skip: bool) -> &mut MergeOptions {
        self.flag(raw::GIT_MERGE_FAIL_ON_CONFLICT, skip)
    }
    pub fn no_recursive(&mut self, disable: bool) -> &mut MergeOptions {
        self.flag(raw::GIT_MERGE_NO_RECURSIVE, disable)
    }
    pub fn rename_threshold(&mut self, thresh: u32) -> &mut MergeOptions {
        self.raw.rename_threshold = thresh;
        self
    }
    pub fn target_limit(&mut self, limit: u32) -> &mut MergeOptions {
        self.raw.target_limit = limit as c_uint;
        self
    }
    pub fn recursion_limit(&mut self, limit: u32) -> &mut MergeOptions {
        self.raw.recursion_limit = limit as c_uint;
        self
    }
    pub fn file_favor(&mut self, favor: FileFavor) -> &mut MergeOptions {
        self.raw.file_favor = favor.convert();
        self
    }
    fn file_flag(&mut self, opt: raw::git_merge_file_flag_t, val: bool) -> &mut MergeOptions {
        if val {
            self.raw.file_flags |= opt;
        }
        self
    }
    pub fn standard_style(&mut self, standard: bool) -> &mut MergeOptions {
        self.file_flag(raw::GIT_MERGE_FILE_STYLE_MERGE, standard)
    }
    pub fn diff3_style(&mut self, diff3: bool) -> &mut MergeOptions {
        self.file_flag(raw::GIT_MERGE_FILE_STYLE_DIFF3, diff3)
    }
    pub fn simplify_alnum(&mut self, simplify: bool) -> &mut MergeOptions {
        self.file_flag(raw::GIT_MERGE_FILE_SIMPLIFY_ALNUM, simplify)
    }
    pub fn ignore_whitespace(&mut self, ignore: bool) -> &mut MergeOptions {
        self.file_flag(raw::GIT_MERGE_FILE_IGNORE_WHITESPACE, ignore)
    }
    pub fn ignore_whitespace_change(&mut self, ignore: bool) -> &mut MergeOptions {
        self.file_flag(raw::GIT_MERGE_FILE_IGNORE_WHITESPACE_CHANGE, ignore)
    }
    pub fn ignore_whitespace_eol(&mut self, ignore: bool) -> &mut MergeOptions {
        self.file_flag(raw::GIT_MERGE_FILE_IGNORE_WHITESPACE_EOL, ignore)
    }
    pub fn patience(&mut self, patience: bool) -> &mut MergeOptions {
        self.file_flag(raw::GIT_MERGE_FILE_DIFF_PATIENCE, patience)
    }
    pub fn minimal(&mut self, minimal: bool) -> &mut MergeOptions {
        self.file_flag(raw::GIT_MERGE_FILE_DIFF_MINIMAL, minimal)
    }
    pub unsafe fn raw(&self) -> *const raw::git_merge_options {
        &self.raw as *const _
    }
}
impl<'repo> Binding for AnnotatedCommit<'repo> {
    type Raw = *mut raw::git_annotated_commit;
    unsafe fn from_raw(raw: *mut raw::git_annotated_commit)
                       -> AnnotatedCommit<'repo> {
        AnnotatedCommit {
            raw: raw,
            _marker: marker::PhantomData,
        }
    }
    fn raw(&self) -> *mut raw::git_annotated_commit { self.raw }
}
impl<'repo> Drop for AnnotatedCommit<'repo> {
    fn drop(&mut self) {
    }
}
