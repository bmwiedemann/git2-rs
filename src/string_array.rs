use std::str;
use std::ops::Range;
use util::Binding;
pub struct StringArray {
    raw: raw::git_strarray,
}
pub struct Iter<'a> {
    range: Range<usize>,
    arr: &'a StringArray,
}
impl StringArray {
    pub fn get(&self, i: usize) -> Option<&str> {
        self.get_bytes(i).and_then(|s| str::from_utf8(s).ok())
    }
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
    pub fn iter(&self) -> Iter {
        Iter { range: 0..self.len(), arr: self }
    }
    pub fn len(&self) -> usize { self.raw.count as usize }
}
impl Binding for StringArray {
    type Raw = raw::git_strarray;
    unsafe fn from_raw(raw: raw::git_strarray) -> StringArray {
        StringArray { raw: raw }
    }
    fn raw(&self) -> raw::git_strarray { self.raw }
}
impl<'a> IntoIterator for &'a StringArray {
    type Item = Option<&'a str>;
    type IntoIter = Iter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'a> Iterator for Iter<'a> {
    type Item = Option<&'a str>;
    fn next(&mut self) -> Option<Option<&'a str>> {
        self.range.next().map(|i| self.arr.get(i))
    }
}
