use std::ffi::{CStr, NulError};
use libc::c_int;
pub struct Error {
    code: c_int,
    klass: c_int,
    message: String,
}
impl Error {
    pub fn from_str(s: &str) -> Error {
        Error {
            code: raw::GIT_ERROR as c_int,
            klass: raw::GITERR_NONE as c_int,
            message: s.to_string(),
        }
    }
    pub fn raw_code(&self) -> raw::git_error_code {
        macro_rules! check( ($($e:ident,)*) => (
            $(if self.code == raw::$e as c_int { raw::$e }) else *
            else {
                raw::GIT_ERROR
            }
        ) );
        check!(
            GIT_ENOTFOUND,
        )
    }
    pub fn raw_class(&self) -> raw::git_error_t {
        macro_rules! check( ($($e:ident,)*) => (
            $(if self.klass == raw::$e as c_int { raw::$e }) else *
            else {
                raw::GITERR_NONE
            }
        ) );
        check!(
            GITERR_NONE,
        )
    }
}
impl From<NulError> for Error {
    fn from(_: NulError) -> Error {
        Error::from_str("data contained a nul byte that could not be \
                         represented as a string")
    }
}
