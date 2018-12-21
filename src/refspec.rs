use std::marker;
pub struct Refspec<'remote> {
    _marker: marker::PhantomData<&'remote raw::git_remote>,
}
