use std::marker;
pub struct Signature<'a> {
    _marker: marker::PhantomData<&'a str>,
}
