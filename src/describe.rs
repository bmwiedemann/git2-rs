use std::marker;
use std::mem;
use std::ffi::CString;
use {raw, Repository, Error, Buf};
pub struct Describe<'repo> {
    _marker: marker::PhantomData<&'repo Repository>,
}
pub struct DescribeOptions {
    raw: raw::git_describe_options,
    pattern: CString,
}
pub struct DescribeFormatOptions {
}
impl DescribeOptions {
    pub fn new() -> DescribeOptions {
        let mut opts = DescribeOptions {
            raw: unsafe { mem::zeroed() },
            pattern: CString::new(Vec::new()).unwrap(),
        };
        opts
    }
}
