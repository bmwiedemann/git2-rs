use std::ffi::{CStr, NulError};
use libc::c_int;
#[derive(Debug,PartialEq)]
pub struct Error {
    code: c_int,
    klass: c_int,
    message: String,
}
impl Error {
    pub fn last_error(code: c_int) -> Option<Error> {
        unsafe {
            let ptr = raw::giterr_last();
            let err = if ptr.is_null() {
                let mut error = Error::from_str("an unknown git error occurred");
                error
            } else {
                Error::from_raw(code, ptr)
            };
            Some(err)
        }
    }
    unsafe fn from_raw(code: c_int, ptr: *const raw::git_error) -> Error {
        let msg = CStr::from_ptr((*ptr).message as *const _).to_bytes();
        let msg = String::from_utf8_lossy(msg).into_owned();
        Error { code: code, klass: (*ptr).klass, message: msg }
    }
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
