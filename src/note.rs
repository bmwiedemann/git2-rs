use std::marker;
use {raw, signature, Signature, Oid, Repository, Error};
use util::Binding;
pub struct Note<'repo> {
    _marker: marker::PhantomData<&'repo Repository>,
}
pub struct Notes<'repo> {
    _marker: marker::PhantomData<&'repo Repository>,
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
