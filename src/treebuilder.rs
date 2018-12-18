use std::marker;
use std::ptr;
use libc::{c_int, c_void};
use {panic, raw, tree, Error, Oid, Repository, TreeEntry};
use util::{Binding, IntoCString};
pub struct TreeBuilder<'repo> {
    raw: *mut raw::git_treebuilder,
    _marker: marker::PhantomData<&'repo Repository>,
}
impl<'repo> TreeBuilder<'repo> {
    pub fn len(&self) -> usize {
        unsafe { raw::git_treebuilder_entrycount(self.raw) as usize }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get<P>(&self, filename: P) -> Result<Option<TreeEntry>, Error>
        where P: IntoCString
    {
        let filename = try!(filename.into_c_string());
        unsafe {
            let ret = raw::git_treebuilder_get(self.raw, filename.as_ptr());
            if ret.is_null() {
                Ok(None)
            } else {
                Ok(Some(tree::entry_from_raw_const(ret)))
            }
        }
    }
    pub fn insert<P: IntoCString>(&mut self, filename: P, oid: Oid,
                                  filemode: i32) -> Result<TreeEntry, Error> {
        let mut ret = ptr::null();
        unsafe {
            Ok(tree::entry_from_raw_const(ret))
        }
    }
    pub fn remove<P: IntoCString>(&mut self, filename: P) -> Result<(), Error> {
        Ok(())
    }
    pub fn filter<F>(&mut self, mut filter: F)
        where F: FnMut(&TreeEntry) -> bool
    {
        let mut cb: &mut FilterCb = &mut filter;
        let ptr = &mut cb as *mut _;
        unsafe {
            raw::git_treebuilder_filter(self.raw, filter_cb, ptr as *mut _);
        }
    }
    pub fn write(&self) -> Result<Oid, Error> {
        let mut raw = raw::git_oid { id: [0; raw::GIT_OID_RAWSZ] };
        unsafe {
            Ok(Binding::from_raw(&raw as *const _))
        }
    }
}
type FilterCb<'a> = FnMut(&TreeEntry) -> bool + 'a;
extern fn filter_cb(entry: *const raw::git_tree_entry,
                    payload: *mut c_void) -> c_int {
    let ret = panic::wrap(|| unsafe {
        if panic::panicked() {
            true
        } else {
            let entry = tree::entry_from_raw_const(entry);
            let payload = payload as *mut &mut FilterCb;
            (*payload)(&entry)
        }
    });
    if ret == Some(false) {1} else {0}
}
impl<'repo> Binding for TreeBuilder<'repo> {
    type Raw = *mut raw::git_treebuilder;
    unsafe fn from_raw(raw: *mut raw::git_treebuilder) -> TreeBuilder<'repo> {
        TreeBuilder { raw: raw, _marker: marker::PhantomData }
    }
    fn raw(&self) -> *mut raw::git_treebuilder { self.raw }
}
impl<'repo> Drop for TreeBuilder<'repo> {
    fn drop(&mut self) {
    }
}
