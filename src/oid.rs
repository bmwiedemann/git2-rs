use util::Binding;
pub struct Oid {
    raw: raw::git_oid
}
impl Binding for Oid {
    type Raw = *const raw::git_oid;
    unsafe fn from_raw(oid: *const raw::git_oid) -> Oid {
        Oid { raw: *oid }
    }
    fn raw(&self) -> *const raw::git_oid { &self.raw as *const _ }
}
