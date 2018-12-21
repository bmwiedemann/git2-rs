use std::marker;
use {raw, Oid, Object, Error};
pub struct Blob<'repo> {
    _marker: marker::PhantomData<Object<'repo>>,
}
pub struct BlobWriter<'repo> {
    _marker: marker::PhantomData<Object<'repo>>,
}
