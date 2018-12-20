use std::ffi::{CStr, CString};
use std::marker;
use std::mem;
use std::slice;
use std::str;
use libc::{c_void, c_int, c_char, c_uint};
use {raw, panic, Error, Cred, CredentialType, Oid};
use cert::Cert;
use util::Binding;
pub struct RemoteCallbacks<'a> {
    progress: Option<Box<TransferProgress<'a>>>,
    credentials: Option<Box<Credentials<'a>>>,
    sideband_progress: Option<Box<TransportMessage<'a>>>,
    update_tips: Option<Box<UpdateTips<'a>>>,
    certificate_check: Option<Box<CertificateCheck<'a>>>,
    push_update_reference: Option<Box<PushUpdateReference<'a>>>,
}
pub struct Progress<'a> {
    raw: ProgressState,
    _marker: marker::PhantomData<&'a raw::git_transfer_progress>,
}
enum ProgressState {
    Borrowed(*const raw::git_transfer_progress),
    Owned(raw::git_transfer_progress),
}
pub type Credentials<'a> = FnMut(&str, Option<&str>, CredentialType)
                                 -> Result<Cred, Error> + 'a;
pub type TransferProgress<'a> = FnMut(Progress) -> bool + 'a;
pub type TransportMessage<'a> = FnMut(&[u8]) -> bool + 'a;
pub type UpdateTips<'a> = FnMut(&str, Oid, Oid) -> bool + 'a;
pub type CertificateCheck<'a> = FnMut(&Cert, &str) -> bool + 'a;
pub type PushUpdateReference<'a> = FnMut(&str, Option<&str>) -> Result<(), Error> + 'a;
impl<'a> RemoteCallbacks<'a> {
    pub fn new() -> RemoteCallbacks<'a> {
        RemoteCallbacks {
            credentials: None,
            progress: None,
            sideband_progress: None,
            update_tips: None,
            certificate_check: None,
            push_update_reference: None,
        }
    }
}
impl<'a> Binding for RemoteCallbacks<'a> {
    type Raw = raw::git_remote_callbacks;
    unsafe fn from_raw(_raw: raw::git_remote_callbacks) -> RemoteCallbacks<'a> {
        panic!("unimplemented");
    }
    fn raw(&self) -> raw::git_remote_callbacks {
        unsafe {
            let mut callbacks: raw::git_remote_callbacks = mem::zeroed();
            assert_eq!(raw::git_remote_init_callbacks(&mut callbacks,
                                        raw::GIT_REMOTE_CALLBACKS_VERSION), 0);
            callbacks
        }
    }
}
impl<'a> Binding for Progress<'a> {
    type Raw = *const raw::git_transfer_progress;
    unsafe fn from_raw(raw: *const raw::git_transfer_progress)
                           -> Progress<'a> {
        Progress {
            raw: ProgressState::Borrowed(raw),
            _marker: marker::PhantomData,
        }
    }
    fn raw(&self) -> *const raw::git_transfer_progress {
        match self.raw {
            ProgressState::Borrowed(raw) => raw,
            ProgressState::Owned(ref raw) => raw as *const _,
        }
    }
}
extern fn credentials_cb(ret: *mut *mut raw::git_cred,
                         url: *const c_char,
                         username_from_url: *const c_char,
                         allowed_types: c_uint,
                         payload: *mut c_void) -> c_int {
    unsafe {
        let ok = panic::wrap(|| {
            let payload = &mut *(payload as *mut RemoteCallbacks);
            let callback = try!(payload.credentials.as_mut()
                                       .ok_or(raw::GIT_PASSTHROUGH as c_int));
            let url = try!(str::from_utf8(CStr::from_ptr(url).to_bytes())
                              .map_err(|_| raw::GIT_PASSTHROUGH as c_int));
            let username_from_url = match ::opt_bytes(&url, username_from_url) {
                Some(username) => {
                    Some(try!(str::from_utf8(username)
                                 .map_err(|_| raw::GIT_PASSTHROUGH as c_int)))
                }
                None => None,
            };
            let cred_type = CredentialType::from_bits_truncate(allowed_types as u32);
            callback(url, username_from_url, cred_type).map_err(|e| {
                e.raw_code() as c_int
            })
        });
        match ok {
            Some(Ok(cred)) => {
                if allowed_types & (cred.credtype() as c_uint) != 0 {
                    0
                } else {
                    raw::GIT_PASSTHROUGH as c_int
                }
            }
            Some(Err(e)) => e,
            None => -1,
        }
    }
}
extern fn transfer_progress_cb(stats: *const raw::git_transfer_progress,
                               payload: *mut c_void) -> c_int {
    let ok = panic::wrap(|| unsafe {
        let payload = &mut *(payload as *mut RemoteCallbacks);
        let callback = match payload.progress {
            Some(ref mut c) => c,
            None => return true,
        };
        let progress = Binding::from_raw(stats);
        callback(progress)
    });
    if ok == Some(true) {0} else {-1}
}
extern fn sideband_progress_cb(str: *const c_char,
                               len: c_int,
                               payload: *mut c_void) -> c_int {
    let ok = panic::wrap(|| unsafe {
        let payload = &mut *(payload as *mut RemoteCallbacks);
        let callback = match payload.sideband_progress {
            Some(ref mut c) => c,
            None => return true,
        };
        let buf = slice::from_raw_parts(str as *const u8, len as usize);
        callback(buf)
    });
    if ok == Some(true) {0} else {-1}
}
extern fn update_tips_cb(refname: *const c_char,
                         a: *const raw::git_oid,
                         b: *const raw::git_oid,
                         data: *mut c_void) -> c_int {
    let ok = panic::wrap(|| unsafe {
        let payload = &mut *(data as *mut RemoteCallbacks);
        let callback = match payload.update_tips {
            Some(ref mut c) => c,
            None => return true,
        };
        let refname = str::from_utf8(CStr::from_ptr(refname).to_bytes())
                          .unwrap();
        let a = Binding::from_raw(a);
        let b = Binding::from_raw(b);
        callback(refname, a, b)
    });
    if ok == Some(true) {0} else {-1}
}
extern fn certificate_check_cb(cert: *mut raw::git_cert,
                               hostname: *const c_char,
                               data: *mut c_void) -> c_int {
    let ok = panic::wrap(|| unsafe {
        let payload = &mut *(data as *mut RemoteCallbacks);
        let callback = match payload.certificate_check {
            Some(ref mut c) => c,
            None => return true,
        };
        let cert = Binding::from_raw(cert);
        let hostname = str::from_utf8(CStr::from_ptr(hostname).to_bytes())
                           .unwrap();
        callback(&cert, hostname)
    });
    if ok == Some(true) {0} else {-1}
}
extern fn push_update_reference_cb(refname: *const c_char,
                                   status: *const c_char,
                                   data: *mut c_void) -> c_int {
    panic::wrap(|| unsafe {
        let payload = &mut *(data as *mut RemoteCallbacks);
        let callback = match payload.push_update_reference {
            Some(ref mut c) => c,
            None => return 0,
        };
        let refname = str::from_utf8(CStr::from_ptr(refname).to_bytes())
                           .unwrap();
        let status = if status.is_null() {
            None
        } else {
            Some(str::from_utf8(CStr::from_ptr(status).to_bytes()).unwrap())
        };
        match callback(refname, status) {
            Ok(()) => 0,
            Err(e) => e.raw_code(),
        }
    }).unwrap_or(-1)
}
