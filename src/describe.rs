use std::marker;
use std::ffi::CString;
use {raw, Repository, Error};
pub struct Describe<'repo> {
    _marker: marker::PhantomData<&'repo Repository>,
}
pub struct DescribeOptions {
}
pub struct DescribeFormatOptions {
}
