use {Object};
pub struct Revspec<'repo> {
    from: Option<Object<'repo>>,
    to: Option<Object<'repo>>,
}
impl<'repo> Revspec<'repo> {
    pub fn from_objects(from: Option<Object<'repo>>,
                        to: Option<Object<'repo>>
                        ) -> Revspec<'repo> {
        Revspec { from: from, to: to }
    }
}
