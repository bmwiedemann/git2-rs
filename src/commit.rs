use std::marker;
use std::mem;
use std::ops::Range;
use std::ptr;
use std::str;
use {raw, signature, Oid, Error, Signature, Tree, Time, Object};
use util::Binding;
pub struct Commit<'repo> {
    raw: *mut raw::git_commit,
    _marker: marker::PhantomData<Object<'repo>>,
}
pub struct Parents<'commit, 'repo: 'commit> {
    range: Range<usize>,
    commit: &'commit Commit<'repo>,
}
pub struct ParentIds<'commit> {
    range: Range<usize>,
    commit: &'commit Commit<'commit>,
}
impl<'repo> Commit<'repo> {
    pub fn tree(&self) -> Result<Tree<'repo>, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn message_encoding(&self) -> Option<&str> {
        let bytes = unsafe {
            ::opt_bytes(self, raw::git_commit_message_encoding(&*self.raw))
        };
        bytes.and_then(|b| str::from_utf8(b).ok())
    }
    pub fn time(&self) -> Time {
        unsafe {
            Time::new(raw::git_commit_time(&*self.raw) as i64,
                      raw::git_commit_time_offset(&*self.raw) as i32)
        }
    }
    pub fn author(&self) -> Signature {
        unsafe {
            let ptr = raw::git_commit_author(&*self.raw);
            signature::from_raw_const(self, ptr)
        }
    }
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
    pub fn parent(&self, i: usize) -> Result<Commit<'repo>, Error> {
        unsafe {
            let mut raw = ptr::null_mut();
            Ok(Binding::from_raw(raw))
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
    pub fn as_object(&self) -> &Object<'repo> {
        unsafe {
            mem::transmute(self)
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
impl<'repo> ::std::fmt::Debug for Commit<'repo> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
		let mut ds = f.debug_struct("Commit");
		ds.finish()
    }
}
impl<'repo, 'commit> Iterator for Parents<'commit, 'repo> {
    type Item = Commit<'repo>;
    fn next(&mut self) -> Option<Commit<'repo>> {
        self.range.next().and_then(|i| self.commit.parent(i).ok())
    }
}
impl<'commit> Iterator for ParentIds<'commit> {
    type Item = Oid;
    fn next(&mut self) -> Option<Oid> {
        self.range.next().and_then(|i| self.commit.parent_id(i).ok())
    }
}
impl<'repo> Clone for Commit<'repo> {
    fn clone(&self) -> Self {
        self.as_object().clone().into_commit().ok().unwrap()
    }
}
