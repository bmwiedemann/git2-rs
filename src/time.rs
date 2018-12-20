use std::cmp::Ordering;
use util::Binding;
pub struct Time {
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IndexTime {
    raw: raw::git_index_time,
}
impl IndexTime {
    pub fn seconds(&self) -> i32 { self.raw.seconds }
    pub fn nanoseconds(&self) -> u32 { self.raw.nanoseconds }
}
impl Binding for IndexTime {
    type Raw = raw::git_index_time;
    unsafe fn from_raw(raw: raw::git_index_time) -> IndexTime {
        IndexTime { raw: raw }
    }
    fn raw(&self) -> raw::git_index_time { self.raw }
}
impl PartialOrd for IndexTime {
    fn partial_cmp(&self, other: &IndexTime) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for IndexTime {
    fn cmp(&self, other: &IndexTime) -> Ordering {
        let me = (self.raw.seconds, self.raw.nanoseconds);
        let other = (other.raw.seconds, other.raw.nanoseconds);
        me.cmp(&other)
    }
}
