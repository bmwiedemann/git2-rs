use util::Binding;
pub struct StringArray {
    raw: raw::git_strarray,
}
impl StringArray {
    pub fn get_bytes(&self, i: usize) -> Option<&[u8]> {
        if i < self.raw.count as usize {
            unsafe {
                let ptr = *self.raw.strings.offset(i as isize) as *const _;
                Some(::opt_bytes(self, ptr).unwrap())
            }
        } else {
            None
        }
    }
}
impl Binding for StringArray {
    type Raw = raw::git_strarray;
    unsafe fn from_raw(raw: raw::git_strarray) -> StringArray {
        StringArray { raw: raw }
    }
    fn raw(&self) -> raw::git_strarray { self.raw }
}
