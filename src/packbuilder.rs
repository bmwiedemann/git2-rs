use std::marker;
use std::slice;
use libc::{c_int, c_uint, c_void, size_t};
use {raw, panic, Repository, Error, Oid, Revwalk, Buf};
use util::Binding;
pub enum PackBuilderStage {
    AddingObjects,
    Deltafication,
}
pub type ProgressCb<'a> = FnMut(PackBuilderStage, u32, u32) -> bool + 'a;
pub type ForEachCb<'a> = FnMut(&[u8]) -> bool + 'a;
pub struct PackBuilder<'repo> {
    raw: *mut raw::git_packbuilder,
    progress: Option<Box<Box<ProgressCb<'repo>>>>,
    _marker: marker::PhantomData<&'repo Repository>,
}
impl<'repo> PackBuilder<'repo> {
    pub fn object_count(&self) -> usize {
        unsafe { raw::git_packbuilder_object_count(self.raw) }
    }
    pub fn hash(&self) -> Option<Oid> {
        if self.object_count() == 0 {
            unsafe {
                Some(Binding::from_raw(raw::git_packbuilder_hash(self.raw)))
            }
        } else {
            None
        }
    }
}
impl<'repo> Binding for PackBuilder<'repo> {
    type Raw = *mut raw::git_packbuilder;
    unsafe fn from_raw(ptr: *mut raw::git_packbuilder) -> PackBuilder<'repo> {
        PackBuilder {
            raw: ptr,
            progress: None,
            _marker: marker::PhantomData,
        }
    }
    fn raw(&self) -> *mut raw::git_packbuilder {
        self.raw
    }
}
impl Binding for PackBuilderStage {
    type Raw = raw::git_packbuilder_stage_t;
    unsafe fn from_raw(raw: raw::git_packbuilder_stage_t) -> PackBuilderStage {
        match raw {
            _ => panic!("Unknown git diff binary kind"),
        }
    }
    fn raw(&self) -> raw::git_packbuilder_stage_t {
        match *self {
            PackBuilderStage::AddingObjects => raw::GIT_PACKBUILDER_ADDING_OBJECTS,
            PackBuilderStage::Deltafication => raw::GIT_PACKBUILDER_DELTAFICATION,
        }
    }
}
extern fn foreach_c(buf: *const c_void,
                    size: size_t,
                    data: *mut c_void)
                    -> c_int {
    unsafe {
        let buf = slice::from_raw_parts(buf as *const u8, size as usize);
        let r = panic::wrap(|| {
            let data = data as *mut &mut ForEachCb;
            (*data)(buf)
        });
        if r == Some(true) {
            0
        } else {
            -1
        }
    }
}
extern fn progress_c(stage: raw::git_packbuilder_stage_t,
                     current: c_uint,
                     total: c_uint,
                     data: *mut c_void)
                     -> c_int {
    unsafe {
        let stage = Binding::from_raw(stage);
        let r = panic::wrap(|| {
            let data = data as *mut Box<ProgressCb>;
            (*data)(stage, current, total)
        });
        if r == Some(true) {
            0
        } else {
            -1
        }
    }
}
