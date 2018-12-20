use std::marker;
use {raw, signature, Oid, Error, Signature, Tree, Time, Object};
use util::Binding;
pub struct Commit<'repo> {
    raw: *mut raw::git_commit,
    _marker: marker::PhantomData<Object<'repo>>,
}
pub struct Parents<'commit, 'repo: 'commit> {
    commit: &'commit Commit<'repo>,
}
impl<'repo> Commit<'repo> {
    pub fn amend(&self,
                 update_ref: Option<&str>,
                 author: Option<&Signature>,
                 committer: Option<&Signature>,
                 message_encoding: Option<&str>,
                 message: Option<&str>,
                 tree: Option<&Tree<'repo>>) -> Result<Oid, Error> {
        let mut raw = raw::git_oid { id: [0; raw::GIT_OID_RAWSZ] };
        let update_ref = try!(::opt_cstr(update_ref));
        let encoding = try!(::opt_cstr(message_encoding));
        let message = try!(::opt_cstr(message));
        unsafe {
            try_call!(raw::git_commit_amend(&mut raw,
                                            self.raw(),
                                            update_ref,
                                            author.map(|s| s.raw()),
                                            committer.map(|s| s.raw()),
                                            encoding,
                                            message,
                                            tree.map(|t| t.raw())));
            Ok(Binding::from_raw(&raw as *const _))
        }
    }
    pub fn parent_id(&self, i: usize) -> Result<Oid, Error> {
        unsafe {
            let id = raw::git_commit_parent_id(self.raw, i as libc::c_uint);
            if id.is_null() {
                Err(Error::from_str("parent index out of bounds"))
            } else {
                Ok(Binding::from_raw(id))
            }
        }
    }
}
impl<'repo> Binding for Commit<'repo> {
    type Raw = *mut raw::git_commit;
    unsafe fn from_raw(raw: *mut raw::git_commit) -> Commit<'repo> {
        Commit {
            raw: raw,
            _marker: marker::PhantomData,
        }
    }
    fn raw(&self) -> *mut raw::git_commit { self.raw }
}
