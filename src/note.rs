use std::marker;
use std::str;
use {raw, signature, Signature, Oid, Repository, Error};
use util::Binding;
pub struct Note<'repo> {
    raw: *mut raw::git_note,
    _marker: marker::PhantomData<&'repo Repository>,
}
pub struct Notes<'repo> {
    raw: *mut raw::git_note_iterator,
    _marker: marker::PhantomData<&'repo Repository>,
}
impl<'repo> Note<'repo> {
    pub fn author(&self) -> Signature {
        unsafe {
            signature::from_raw_const(self, raw::git_note_author(&*self.raw))
        }
    }
    pub fn committer(&self) -> Signature {
        unsafe {
            signature::from_raw_const(self, raw::git_note_committer(&*self.raw))
        }
    }
    pub fn message_bytes(&self) -> &[u8] {
        unsafe { ::opt_bytes(self, raw::git_note_message(&*self.raw)).unwrap() }
    }
    pub fn message(&self) -> Option<&str> {
        str::from_utf8(self.message_bytes()).ok()
    }
    pub fn id(&self) -> Oid {
        unsafe { Binding::from_raw(raw::git_note_id(&*self.raw)) }
    }
}
impl<'repo> Binding for Note<'repo> {
    type Raw = *mut raw::git_note;
    unsafe fn from_raw(raw: *mut raw::git_note) -> Note<'repo> {
        Note { raw: raw, _marker: marker::PhantomData, }
    }
    fn raw(&self) -> *mut raw::git_note { self.raw }
}
impl<'repo> ::std::fmt::Debug for Note<'repo> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        f.debug_struct("Note").field("id", &self.id()).finish()
    }
}
impl<'repo> Drop for Note<'repo> {
    fn drop(&mut self) {
    }
}
impl<'repo> Binding for Notes<'repo> {
    type Raw = *mut raw::git_note_iterator;
    unsafe fn from_raw(raw: *mut raw::git_note_iterator) -> Notes<'repo> {
        Notes { raw: raw, _marker: marker::PhantomData, }
    }
    fn raw(&self) -> *mut raw::git_note_iterator { self.raw }
}
impl<'repo> Iterator for Notes<'repo> {
    type Item = Result<(Oid, Oid), Error>;
    fn next(&mut self) -> Option<Result<(Oid, Oid), Error>> {
        let mut note_id = raw::git_oid { id: [0; raw::GIT_OID_RAWSZ] };
        let mut annotated_id = note_id;
        unsafe {
            try_call_iter!(raw::git_note_next(&mut note_id, &mut annotated_id,
                                              self.raw));
            Some(Ok((Binding::from_raw(&note_id as *const _),
                     Binding::from_raw(&annotated_id as *const _))))
        }
    }
}
impl<'repo> Drop for Notes<'repo> {
    fn drop(&mut self) {
    }
}
