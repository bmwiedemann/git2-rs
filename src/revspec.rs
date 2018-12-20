use {Object, RevparseMode};
pub struct Revspec<'repo> {
    from: Option<Object<'repo>>,
    to: Option<Object<'repo>>,
    mode: RevparseMode,
}
impl<'repo> Revspec<'repo> {
    pub fn from_objects(from: Option<Object<'repo>>,
                        to: Option<Object<'repo>>,
                        mode: RevparseMode) -> Revspec<'repo> {
        Revspec { from: from, to: to, mode: mode }
    }
}
