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
    use std::ffi::CString;
    use std::ptr;
    use {raw, ConfigLevel, ResetType, ObjectType, BranchType, Direction};
    use {DiffFormat, FileFavor, SubmoduleIgnore, AutotagOption, FetchPrune};
    use call::Convert;
    impl<T: Copy> Convert<T> for T {
        fn convert(&self) -> T { *self }
    }
    impl Convert<libc::c_int> for bool {
        fn convert(&self) -> libc::c_int { *self as libc::c_int }
    }
    impl<'a, T> Convert<*mut T> for &'a mut T {
        fn convert(&self) -> *mut T { &**self as *const T as *mut T }
    }
    impl Convert<*const libc::c_char> for CString {
        fn convert(&self) -> *const libc::c_char { self.as_ptr() }
    }
    impl<T, U: Convert<*const T>> Convert<*const T> for Option<U> {
        fn convert(&self) -> *const T {
            self.as_ref().map(|s| s.convert()).unwrap_or(ptr::null())
        }
    }
    impl Convert<raw::git_otype> for ObjectType {
        fn convert(&self) -> raw::git_otype {
            match *self {
                ObjectType::Any => raw::GIT_OBJ_ANY,
                ObjectType::Commit => raw::GIT_OBJ_COMMIT,
                ObjectType::Tree => raw::GIT_OBJ_TREE,
                ObjectType::Blob => raw::GIT_OBJ_BLOB,
                ObjectType::Tag => raw::GIT_OBJ_TAG,
            }
        }
    }
    impl Convert<raw::git_config_level_t> for ConfigLevel {
        fn convert(&self) -> raw::git_config_level_t {
            match *self {
                ConfigLevel::ProgramData => raw::GIT_CONFIG_LEVEL_PROGRAMDATA,
                ConfigLevel::System => raw::GIT_CONFIG_LEVEL_SYSTEM,
                ConfigLevel::XDG => raw::GIT_CONFIG_LEVEL_XDG,
                ConfigLevel::Global => raw::GIT_CONFIG_LEVEL_GLOBAL,
                ConfigLevel::Local => raw::GIT_CONFIG_LEVEL_LOCAL,
                ConfigLevel::App => raw::GIT_CONFIG_LEVEL_APP,
                ConfigLevel::Highest => raw::GIT_CONFIG_HIGHEST_LEVEL,
            }
        }
    }
    impl Convert<raw::git_remote_autotag_option_t> for AutotagOption {
        fn convert(&self) -> raw::git_remote_autotag_option_t {
            match *self {
                AutotagOption::Unspecified =>
                    raw::GIT_REMOTE_DOWNLOAD_TAGS_UNSPECIFIED,
                AutotagOption::None => raw::GIT_REMOTE_DOWNLOAD_TAGS_NONE,
                AutotagOption::Auto => raw::GIT_REMOTE_DOWNLOAD_TAGS_AUTO,
                AutotagOption::All => raw::GIT_REMOTE_DOWNLOAD_TAGS_ALL,
            }
        }
    }
    impl Convert<raw::git_fetch_prune_t> for FetchPrune {
        fn convert(&self) -> raw::git_fetch_prune_t {
            match *self {
                FetchPrune::Unspecified => raw::GIT_FETCH_PRUNE_UNSPECIFIED,
                FetchPrune::On => raw::GIT_FETCH_PRUNE,
                FetchPrune::Off => raw::GIT_FETCH_NO_PRUNE,
            }
        }
    }
}
