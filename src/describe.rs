use std::marker;
use std::mem;
use std::ffi::CString;
use {raw, Repository, Error, Buf};
use util::Binding;
pub struct Describe<'repo> {
    raw: *mut raw::git_describe_result,
    _marker: marker::PhantomData<&'repo Repository>,
}
pub struct DescribeOptions {
    raw: raw::git_describe_options,
    pattern: CString,
}
pub struct DescribeFormatOptions {
    raw: raw::git_describe_format_options,
    dirty_suffix: CString,
}
impl<'repo> Binding for Describe<'repo> {
    type Raw = *mut raw::git_describe_result;
    unsafe fn from_raw(raw: *mut raw::git_describe_result) -> Describe<'repo> {
        Describe { raw: raw, _marker: marker::PhantomData, }
    }
    fn raw(&self) -> *mut raw::git_describe_result { self.raw }
}
impl Default for DescribeFormatOptions {
    fn default() -> Self {
        let mut opts = DescribeFormatOptions {
            raw: unsafe { mem::zeroed() },
            dirty_suffix: CString::new(Vec::new()).unwrap(),
        };
        opts
    }
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
