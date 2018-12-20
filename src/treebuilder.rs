use std::marker;
use libc::{c_int, c_void};
use {panic, raw, tree, Error, Oid, Repository, TreeEntry};
use util::{Binding, IntoCString};
pub struct TreeBuilder<'repo> {
    raw: *mut raw::git_treebuilder,
    _marker: marker::PhantomData<&'repo Repository>,
}
impl<'repo> TreeBuilder<'repo> {
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
