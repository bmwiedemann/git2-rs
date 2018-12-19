use std::marker;
use std::mem;
use {raw, Oid, Commit, FileFavor};
use util::Binding;
pub struct AnnotatedCommit<'repo> {
    raw: *mut raw::git_annotated_commit,
    _marker: marker::PhantomData<Commit<'repo>>,
}
pub struct MergeOptions {
    raw: raw::git_merge_options,
}
impl MergeOptions {
    pub fn new() -> MergeOptions {
        let mut opts = MergeOptions {
            raw: unsafe { mem::zeroed() },
        };
        opts
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
