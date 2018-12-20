use std::ffi::CString;
use std::io::Write;
use std::mem;
use std::path::Path;
use std::process::{Command, Stdio};
use std::ptr;
use url;
use {raw, Error, Config, IntoCString};
use util::Binding;
pub struct Cred {
    raw: *mut raw::git_cred,
}
pub struct CredentialHelper {
    pub username: Option<String>,
    protocol: Option<String>,
    host: Option<String>,
    url: String,
    commands: Vec<String>,
}
impl Cred {
    pub fn default() -> Result<Cred, Error> {
        ::init();
        let mut out = ptr::null_mut();
        unsafe {
            try_call!(raw::git_cred_default_new(&mut out));
            Ok(Binding::from_raw(out))
        }
    }
    pub fn ssh_key_from_agent(username: &str) -> Result<Cred, Error> {
        ::init();
        let mut out = ptr::null_mut();
        let username = try!(CString::new(username));
        unsafe {
            try_call!(raw::git_cred_ssh_key_from_agent(&mut out, username));
            Ok(Binding::from_raw(out))
        }
    }
    pub fn ssh_key(username: &str,
                   publickey: Option<&Path>,
                   privatekey: &Path,
                   passphrase: Option<&str>) -> Result<Cred, Error> {
        ::init();
        let username = try!(CString::new(username));
        let publickey = try!(::opt_cstr(publickey));
        let privatekey = try!(privatekey.into_c_string());
        let passphrase = try!(::opt_cstr(passphrase));
        let mut out = ptr::null_mut();
        unsafe {
            try_call!(raw::git_cred_ssh_key_new(&mut out, username, publickey,
                                                privatekey, passphrase));
            Ok(Binding::from_raw(out))
        }
    }
    pub fn ssh_key_from_memory(username: &str,
                               publickey: Option<&str>,
                               privatekey: &str,
                               passphrase: Option<&str>) -> Result<Cred, Error> {
        ::init();
        let username = try!(CString::new(username));
        let publickey = try!(::opt_cstr(publickey));
        let privatekey = try!(CString::new(privatekey));
        let passphrase = try!(::opt_cstr(passphrase));
        let mut out = ptr::null_mut();
        unsafe {
            try_call!(raw::git_cred_ssh_key_memory_new(&mut out, username, publickey,
                                                       privatekey, passphrase));
            Ok(Binding::from_raw(out))
        }
    }
    pub fn userpass_plaintext(username: &str,
                              password: &str) -> Result<Cred, Error> {
        ::init();
        let username = try!(CString::new(username));
        let password = try!(CString::new(password));
        let mut out = ptr::null_mut();
        unsafe {
            try_call!(raw::git_cred_userpass_plaintext_new(&mut out, username,
                                                           password));
            Ok(Binding::from_raw(out))
        }
    }
    pub fn credential_helper(config: &Config,
                             url: &str,
                             username: Option<&str>)
                             -> Result<Cred, Error> {
        match CredentialHelper::new(url).config(config).username(username)
                               .execute() {
            Some((username, password)) => {
                Cred::userpass_plaintext(&username, &password)
            }
            None => Err(Error::from_str("failed to acquire username/password \
                                         from local configuration"))
        }
    }
    pub fn credtype(&self) -> raw::git_credtype_t {
        unsafe { (*self.raw).credtype }
    }
    pub unsafe fn unwrap(mut self) -> *mut raw::git_cred {
        mem::replace(&mut self.raw, ptr::null_mut())
    }
}
impl Binding for Cred {
    type Raw = *mut raw::git_cred;
    unsafe fn from_raw(raw: *mut raw::git_cred) -> Cred {
        Cred { raw: raw }
    }
    fn raw(&self) -> *mut raw::git_cred { self.raw }
}
impl CredentialHelper {
    pub fn new(url: &str) -> CredentialHelper {
        let mut ret = CredentialHelper {
            protocol: None,
            host: None,
            username: None,
            url: url.to_string(),
            commands: Vec::new(),
        };
        ret
    }
    pub fn username(&mut self, username: Option<&str>) -> &mut CredentialHelper {
        self
    }
    pub fn config(&mut self, config: &Config) -> &mut CredentialHelper {
        self.config_helper(config);
        self
    }
    fn config_helper(&mut self, config: &Config) {
        let exact = config.get_string(&self.exact_key("helper"));
        let global = config.get_string("credential.helper");
    }
    fn exact_key(&self, name: &str) -> String {
        format!("credential.{}.{}", self.url, name)
    }
    fn url_key(&self, name: &str) -> Option<String> {
        match (&self.host, &self.protocol) {
            _ => None
        }
    }
    pub fn execute(&self) -> Option<(String, String)> {
        let mut username = self.username.clone();
        let mut password = None;
        for cmd in &self.commands {
            let (u, p) = self.execute_cmd(cmd, &username);
            if u.is_some() && username.is_none() {
                password = p;
            }
            if username.is_some() && password.is_some() { break }
        }
        match (username, password) {
            _ => None,
        }
    }
    fn execute_cmd(&self, cmd: &str, username: &Option<String>)
                   -> (Option<String>, Option<String>) {
        macro_rules! my_try( ($e:expr) => (
            match $e {
                Ok(e) => e,
                Err(e) => {
                    return (None, None)
                }
            }
        ) );
        let mut c = Command::new("sh");
        let mut p = match c.spawn() {
            Ok(p) => p,
            Err(e) => {
                match c.spawn() {
                    Ok(p) => p,
                    Err(e) => {
                        debug!("fallback of {:?} failed with {}", cmd, e);
                        return (None, None);
                    }
                }
            }
        };
        {
            let stdin = p.stdin.as_mut().unwrap();
            if let Some(ref p)  = self.protocol {
                let _ = writeln!(stdin, "protocol={}", p);
            }
        }
        let output = my_try!(p.wait_with_output());
        self.parse_output(output.stdout)
    }
    fn parse_output(&self, output: Vec<u8>) -> (Option<String>, Option<String>) {
        let mut username = None;
        let mut password = None;
        for line in output.split(|t| *t == b'\n') {
            let mut parts = line.splitn(2, |t| *t == b'=');
            let value = match parts.next() {
                Some(s) => s,
                None => {
                    continue
                }
            };
        }
        (username, password)
    }
}
