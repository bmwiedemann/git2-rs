use std::marker;
use {raw, signature, Error, Oid, Object, Signature, ObjectType};
pub struct Tag<'repo> {
    _marker: marker::PhantomData<Object<'repo>>,
}
