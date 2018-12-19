use std::marker;
use util::Binding;
pub struct Refspec<'remote> {
    raw: *const raw::git_refspec,
    _marker: marker::PhantomData<&'remote raw::git_remote>,
}
impl<'remote> Binding for Refspec<'remote> {
    type Raw = *const raw::git_refspec;
    unsafe fn from_raw(raw: *const raw::git_refspec) -> Refspec<'remote> {
        Refspec { raw: raw, _marker: marker::PhantomData }
    }
    fn raw(&self) -> *const raw::git_refspec { self.raw }
}
