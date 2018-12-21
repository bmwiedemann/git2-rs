#![macro_use]
use Error;
macro_rules! call {
    (raw::$p:ident ($($e:expr),*)) => (
        raw::$p($(::call::convert(&$e)),*)
    )
}
macro_rules! try_call {
    (raw::$p:ident ($($e:expr),*)) => ({
        match ::call::try(raw::$p($(::call::convert(&$e)),*)) {
            Ok(o) => o,
            Err(e) => { ::panic::check(); return Err(e) }
        }
    })
}
pub trait Convert<T> {
    fn convert(&self) -> T;
}
pub fn convert<T, U: Convert<T>>(u: &U) -> T { u.convert() }
pub fn try(ret: libc::c_int) -> Result<libc::c_int, Error> {
    match ret {
        n => Ok(n),
    }
}
mod impls {
    use {raw, ObjectType};
    use call::Convert;
    impl<T: Copy> Convert<T> for T {
        fn convert(&self) -> T { *self }
    }
    impl Convert<raw::git_otype> for ObjectType {
        fn convert(&self) -> raw::git_otype {
            raw::GIT_OBJ_TAG
        }
    }
}
