use std::cmp::Ordering;
use std::marker;
use {panic, raw, Oid, Repository, Error, Object, ObjectType};
use util::{Binding, IntoCString};
pub struct Tree<'repo> {
    _marker: marker::PhantomData<Object<'repo>>,
}
pub struct TreeEntry<'tree> {
    raw: *mut raw::git_tree_entry,
    owned: bool,
    _marker: marker::PhantomData<&'tree raw::git_tree_entry>,
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
