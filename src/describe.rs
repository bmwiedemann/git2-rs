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
impl<'repo> Describe<'repo> {
    pub fn format(&self, opts: Option<&DescribeFormatOptions>)
                  -> Result<String, Error> {
        let buf = Buf::new();
        unsafe {
        }
        Ok(String::from_utf8(buf.to_vec()).unwrap())
    }
}
impl<'repo> Binding for Describe<'repo> {
    type Raw = *mut raw::git_describe_result;
    unsafe fn from_raw(raw: *mut raw::git_describe_result) -> Describe<'repo> {
        Describe { raw: raw, _marker: marker::PhantomData, }
    }
    fn raw(&self) -> *mut raw::git_describe_result { self.raw }
}
impl<'repo> Drop for Describe<'repo> {
    fn drop(&mut self) {
    }
}
impl Default for DescribeFormatOptions {
    fn default() -> Self {
        Self::new()
    }
}
impl DescribeFormatOptions {
    pub fn new() -> DescribeFormatOptions {
        let mut opts = DescribeFormatOptions {
            raw: unsafe { mem::zeroed() },
            dirty_suffix: CString::new(Vec::new()).unwrap(),
        };
        opts
    }
    pub fn abbreviated_size(&mut self, size: u32) -> &mut Self {
        self
    }
    pub fn always_use_long_format(&mut self, long: bool) -> &mut Self {
        self
    }
    pub fn dirty_suffix(&mut self, suffix: &str) -> &mut Self {
        self
    }
    fn default() -> Self {
        Self::new()
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
    pub fn max_candidates_tags(&mut self, max: u32) -> &mut Self {
        self
    }
    pub fn describe_tags(&mut self) -> &mut Self {
        self
    }
    pub fn describe_all(&mut self) -> &mut Self {
        self
    }
    pub fn only_follow_first_parent(&mut self, follow: bool) -> &mut Self {
        self
    }
    pub fn show_commit_oid_as_fallback(&mut self, show: bool) -> &mut Self {
        self
    }
    pub fn pattern(&mut self, pattern: &str) -> &mut Self {
        self
    }
}
impl Binding for DescribeOptions {
    type Raw = *mut raw::git_describe_options;
    unsafe fn from_raw(_raw: *mut raw::git_describe_options)
                       -> DescribeOptions {
        panic!("unimplemened")
    }
    fn raw(&self) -> *mut raw::git_describe_options {
        &self.raw as *const _ as *mut _
    }
}
