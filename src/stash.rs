use {raw, panic, Oid, StashApplyProgress};
use std::ffi::{CStr};
use util::{Binding};
use libc::{c_int, c_char, size_t, c_void};
use build::{CheckoutBuilder};
use std::mem;
pub type StashApplyProgressCb<'a> = FnMut(StashApplyProgress) -> bool + 'a;
pub type StashCb<'a> = FnMut(usize, &str, &Oid) -> bool + 'a;
pub struct StashApplyOptions<'cb> {
    progress: Option<Box<StashApplyProgressCb<'cb>>>,
    checkout_options: Option<CheckoutBuilder<'cb>>,
    raw_opts: raw::git_stash_apply_options
}
impl<'cb> StashApplyOptions<'cb> {
    pub fn new() -> StashApplyOptions<'cb> {
        let mut opts = StashApplyOptions {
            progress: None,
            checkout_options: None,
            raw_opts: unsafe { mem::zeroed() },
        };
        assert_eq!(unsafe {
            raw::git_stash_apply_init_options(&mut opts.raw_opts, 1)
        }, 0);
        opts
    }
}
pub struct StashCbData<'a> {
    pub callback: &'a mut StashCb<'a>
}
pub extern fn stash_cb(index: size_t,
                        message: *const c_char,
                        stash_id: *const raw::git_oid,
                        payload: *mut c_void)
                        -> c_int
{
    panic::wrap(|| unsafe {
        let mut data = &mut *(payload as *mut StashCbData);
        let res = {
            let mut callback = &mut data.callback;
            callback(index,
                     CStr::from_ptr(message).to_str().unwrap(),
                     &Binding::from_raw(stash_id))
        };
        if res { 0 } else { 1 }
    }).unwrap_or(1)
}
fn convert_progress(progress: raw::git_stash_apply_progress_t) -> StashApplyProgress {
    match progress {
        _ => StashApplyProgress::None
    }
}
extern fn stash_apply_progress_cb(progress: raw::git_stash_apply_progress_t,
                                  payload: *mut c_void)
                                  -> c_int
{
    panic::wrap(|| unsafe {
        let mut options = &mut *(payload as *mut StashApplyOptions);
        let res = {
            let mut callback = options.progress.as_mut().unwrap();
            callback(convert_progress(progress))
        };
        if res { 0 } else { -1 }
    }).unwrap_or(-1)
}
