use std::marker;
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
impl<'repo> Binding for Note<'repo> {
    type Raw = *mut raw::git_note;
    unsafe fn from_raw(raw: *mut raw::git_note) -> Note<'repo> {
        Note { raw: raw, _marker: marker::PhantomData, }
    }
    fn raw(&self) -> *mut raw::git_note { self.raw }
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
            Some(Ok((Binding::from_raw(&note_id as *const _),
                     Binding::from_raw(&annotated_id as *const _))))
        }
    }
}
