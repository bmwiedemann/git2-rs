use std::mem;
use std::cmp::Ordering;
use std::ffi::{CStr, CString};
use std::ops::Range;
use std::marker;
use std::ptr;
use libc::{self, c_int, c_char, c_void};
use {panic, raw, Oid, Repository, Error, Object, ObjectType};
use util::{Binding, IntoCString};
pub struct Tree<'repo> {
    raw: *mut raw::git_tree,
    _marker: marker::PhantomData<Object<'repo>>,
}
pub struct TreeEntry<'tree> {
    raw: *mut raw::git_tree_entry,
    owned: bool,
    _marker: marker::PhantomData<&'tree raw::git_tree_entry>,
}
pub struct TreeIter<'tree> {
    range: Range<usize>,
    tree: &'tree Tree<'tree>,
}
pub enum TreeWalkMode {
}
pub enum TreeWalkResult {
}
impl<'repo> Tree<'repo> {
    pub fn len(&self) -> usize {
        unsafe { raw::git_tree_entrycount(&*self.raw) as usize }
    }
    pub fn iter(&self) -> TreeIter {
        TreeIter { range: 0..self.len(), tree: self }
    }
    pub fn walk<C, T>(&self, mode: TreeWalkMode, mut callback: C) -> Result<(), Error>
    {
        unsafe {
            Ok(())
        }
    }
    pub fn get_id(&self, id: Oid) -> Option<TreeEntry> {
        unsafe {
            let ptr = raw::git_tree_entry_byid(&*self.raw(), &*id.raw());
            if ptr.is_null() {
                None
            } else {
                Some(entry_from_raw_const(ptr))
            }
        }
    }
    pub fn get(&self, n: usize) -> Option<TreeEntry> {
        unsafe {
            let ptr = raw::git_tree_entry_byindex(&*self.raw(),
                                                  n as libc::size_t);
            if ptr.is_null() {
                None
            } else {
                Some(entry_from_raw_const(ptr))
            }
        }
    }
    pub fn get_name(&self, filename: &str) -> Option<TreeEntry> {
        let filename = CString::new(filename).unwrap();
        unsafe {
            let ptr = call!(raw::git_tree_entry_byname(&*self.raw(), filename));
            if ptr.is_null() {
                None
            } else {
                Some(entry_from_raw_const(ptr))
            }
        }
    }
}
extern fn treewalk_cb<T: Into<i32>>(root: *const c_char, entry: *const raw::git_tree_entry, payload: *mut c_void) -> c_int {
    match panic::wrap(|| unsafe {
        let root = match CStr::from_ptr(root).to_str() {
            _ => return -1,
        };
    }) {
        Some(value) => value,
        None => -1,
    }
}
impl<'repo> Binding for Tree<'repo> {
    type Raw = *mut raw::git_tree;
    unsafe fn from_raw(raw: *mut raw::git_tree) -> Tree<'repo> {
        Tree { raw: raw, _marker: marker::PhantomData }
    }
    fn raw(&self) -> *mut raw::git_tree { self.raw }
}
impl<'repo, 'iter> IntoIterator for &'iter Tree<'repo> {
    type Item = TreeEntry<'iter>;
    type IntoIter = TreeIter<'iter>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
pub unsafe fn entry_from_raw_const<'tree>(raw: *const raw::git_tree_entry)
                                          -> TreeEntry<'tree> {
    TreeEntry {
        raw: raw as *mut raw::git_tree_entry,
        owned: false,
        _marker: marker::PhantomData,
    }
}
impl<'tree> TreeEntry<'tree> {
    pub fn to_object<'a>(&self, repo: &'a Repository)
                         -> Result<Object<'a>, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(ret))
        }
    }
}
impl<'a> Binding for TreeEntry<'a> {
    type Raw = *mut raw::git_tree_entry;
    unsafe fn from_raw(raw: *mut raw::git_tree_entry) -> TreeEntry<'a> {
        TreeEntry {
            raw: raw,
            owned: true,
            _marker: marker::PhantomData,
        }
    }
    fn raw(&self) -> *mut raw::git_tree_entry { self.raw }
}
impl<'a> Clone for TreeEntry<'a> {
    fn clone(&self) -> TreeEntry<'a> {
        let mut ret = ptr::null_mut();
        unsafe {
            Binding::from_raw(ret)
        }
    }
}
impl<'a> PartialOrd for TreeEntry<'a> {
    fn partial_cmp(&self, other: &TreeEntry<'a>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<'a> Ord for TreeEntry<'a> {
    fn cmp(&self, other: &TreeEntry<'a>) -> Ordering {
        match unsafe { raw::git_tree_entry_cmp(&*self.raw(), &*other.raw()) } {
            _ => Ordering::Greater,
        }
    }
}
impl<'a> PartialEq for TreeEntry<'a> {
    fn eq(&self, other: &TreeEntry<'a>) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl<'a> Eq for TreeEntry<'a> {}
impl<'tree> Iterator for TreeIter<'tree> {
    type Item = TreeEntry<'tree>;
    fn next(&mut self) -> Option<TreeEntry<'tree>> {
        self.range.next().and_then(|i| self.tree.get(i))
    }
}
