use std::cmp::Ordering;
use libc::{c_int, c_char};
use util::Binding;
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Time {
    raw: raw::git_time,
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IndexTime {
    raw: raw::git_index_time,
}
impl Time {
    pub fn new(time: i64, offset: i32) -> Time {
        unsafe {
            Binding::from_raw(raw::git_time {
                time: time as raw::git_time_t,
                offset: offset as c_int,
                sign: if offset < 0 { '-' } else { '+' } as c_char,
            })
        }
    }
}
impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Time) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Time {
    fn cmp(&self, other: &Time) -> Ordering {
        (self.raw.time, self.raw.offset).cmp(&(other.raw.time, other.raw.offset))
    }
}
impl Binding for Time {
    type Raw = raw::git_time;
    unsafe fn from_raw(raw: raw::git_time) -> Time {
        Time { raw: raw }
    }
    fn raw(&self) -> raw::git_time { self.raw }
}
impl IndexTime {
    pub fn new(seconds: i32, nanoseconds: u32) -> IndexTime {
        unsafe {
            Binding::from_raw(raw::git_index_time {
                seconds: seconds,
                nanoseconds: nanoseconds,
            })
        }
    }
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
