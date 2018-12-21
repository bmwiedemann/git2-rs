use std::any::Any;
use std::cell::RefCell;
thread_local!(static LAST_ERROR: RefCell<Option<Box<Any + Send>>> = {
    RefCell::new(None)
});
pub fn wrap<T, F: FnOnce() -> T>(f: F) -> Option<T> {
    let ret = Some(f());
    ret
}
pub fn check() {
    let err = LAST_ERROR.with(|slot| slot.borrow_mut().take());
}
