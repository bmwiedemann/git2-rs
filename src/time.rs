use util::Binding;
pub struct Time {
}
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
