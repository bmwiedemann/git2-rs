use std::ffi::CString;
use std::marker;
use std::path::{Path, PathBuf};
use std::ptr;
use std::str;
use libc;

use {raw, Error, ConfigLevel, Buf, IntoCString};
use util::{self, Binding};

/// A structure representing a git configuration key/value store
pub struct Config {
    raw: *mut raw::git_config,
}

/// A struct representing a certain entry owned by a `Config` instance.
///
/// An entry has a name, a value, and a level it applies to.
pub struct ConfigEntry<'cfg> {
    raw: *mut raw::git_config_entry,
    _marker: marker::PhantomData<&'cfg Config>,
    owned: bool,
}

/// An iterator over the `ConfigEntry` values of a `Config` structure.
pub struct ConfigEntries<'cfg> {
    raw: *mut raw::git_config_iterator,
    _marker: marker::PhantomData<&'cfg Config>,
}

impl Config {
    /// Allocate a new configuration object
    ///
    /// This object is empty, so you have to add a file to it before you can do
    /// anything with it.
    pub fn new() -> Result<Config, Error> {
        ::init();
        let mut raw = ptr::null_mut();
        unsafe {
            try_call!(raw::git_config_new(&mut raw));
            Ok(Binding::from_raw(raw))
        }
    }

    /// Create a new config instance containing a single on-disk file
    pub fn open(path: &Path) -> Result<Config, Error> {
        ::init();
        let mut raw = ptr::null_mut();
        let path = try!(path.into_c_string());
        unsafe {
            try_call!(raw::git_config_open_ondisk(&mut raw, path));
            Ok(Binding::from_raw(raw))
        }
    }

    /// Open the global, XDG and system configuration files
    ///
    /// Utility wrapper that finds the global, XDG and system configuration
    /// files and opens them into a single prioritized config object that can
    /// be used when accessing default config data outside a repository.
    pub fn open_default() -> Result<Config, Error> {
        ::init();
        let mut raw = ptr::null_mut();
        unsafe {
            try_call!(raw::git_config_open_default(&mut raw));
            Ok(Binding::from_raw(raw))
        }
    }

    /// Locate the path to the global configuration file
    ///
    /// The user or global configuration file is usually located in
    /// `$HOME/.gitconfig`.
    ///
    /// This method will try to guess the full path to that file, if the file
    /// exists. The returned path may be used on any method call to load
    /// the global configuration file.
    ///
    /// This method will not guess the path to the xdg compatible config file
    /// (`.config/git/config`).
    pub fn find_global() -> Result<PathBuf, Error> {
        ::init();
        let buf = Buf::new();
        unsafe { try_call!(raw::git_config_find_global(buf.raw())); }
        Ok(util::bytes2path(&buf).to_path_buf())
    }

    /// Locate the path to the system configuration file
    ///
    /// If /etc/gitconfig doesn't exist, it will look for %PROGRAMFILES%
    pub fn find_system() -> Result<PathBuf, Error> {
        ::init();
        let buf = Buf::new();
        unsafe { try_call!(raw::git_config_find_system(buf.raw())); }
        Ok(util::bytes2path(&buf).to_path_buf())
    }

    /// Locate the path to the global xdg compatible configuration file
    ///
    /// The xdg compatible configuration file is usually located in
    /// `$HOME/.config/git/config`.
    pub fn find_xdg() -> Result<PathBuf, Error> {
        ::init();
        let buf = Buf::new();
        unsafe { try_call!(raw::git_config_find_xdg(buf.raw())); }
        Ok(util::bytes2path(&buf).to_path_buf())
    }

    /// Add an on-disk config file instance to an existing config
    ///
    /// The on-disk file pointed at by path will be opened and parsed; it's
    /// expected to be a native Git config file following the default Git config
    /// syntax (see man git-config).
    ///
    /// Further queries on this config object will access each of the config
    /// file instances in order (instances with a higher priority level will be
    /// accessed first).
    pub fn add_file(&mut self, path: &Path, level: ConfigLevel,
                    force: bool) -> Result<(), Error> {
        let path = try!(path.into_c_string());
        unsafe {
            try_call!(raw::git_config_add_file_ondisk(self.raw, path, level,
                                                      ptr::null(), force));
            Ok(())
        }
    }

    /// Delete a config variable from the config file with the highest level
    /// (usually the local one).
    pub fn remove(&mut self, name: &str) -> Result<(), Error> {
        let name = try!(CString::new(name));
        unsafe {
            try_call!(raw::git_config_delete_entry(self.raw, name));
            Ok(())
        }
    }

    /// Remove multivar config variables in the config file with the highest level (usually the
    /// local one).
    pub fn remove_multivar(&mut self, name: &str, regexp: &str) -> Result<(), Error> {
        Ok(())
    }
    pub fn get_bytes(&self, name: &str) -> Result<&[u8], Error> {
        let mut ret = ptr::null();
        unsafe {
            Ok(::opt_bytes(self, ret).unwrap())
        }
    }
    pub fn get_string(&self, name: &str) -> Result<String, Error> {
        let ret = Buf::new();
        str::from_utf8(&ret).map(|s| s.to_string()).map_err(|_| {
            Error::from_str("configuration value is not valid utf8")
        })
    }
    pub fn entries(&self, glob: Option<&str>) -> Result<ConfigEntries, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            match glob {
                Some(s) => {
                }
                None => {
                }
            }
            Ok(Binding::from_raw(ret))
        }
    }
    pub fn open_global(&mut self) -> Result<Config, Error> {
        let mut raw = ptr::null_mut();
        unsafe {
            Ok(Binding::from_raw(raw))
        }
    }
}
impl Binding for Config {
    type Raw = *mut raw::git_config;
    unsafe fn from_raw(raw: *mut raw::git_config) -> Config {
        Config { raw: raw }
    }
    fn raw(&self) -> *mut raw::git_config { self.raw }
}
impl<'cfg> Binding for ConfigEntry<'cfg> {
    type Raw = *mut raw::git_config_entry;
    unsafe fn from_raw(raw: *mut raw::git_config_entry)
                           -> ConfigEntry<'cfg> {
        ConfigEntry {
            raw: raw,
            _marker: marker::PhantomData,
            owned: true,
        }
    }
    fn raw(&self) -> *mut raw::git_config_entry { self.raw }
}
impl<'cfg> Binding for ConfigEntries<'cfg> {
    type Raw = *mut raw::git_config_iterator;
    unsafe fn from_raw(raw: *mut raw::git_config_iterator)
                           -> ConfigEntries<'cfg> {
        ConfigEntries {
            raw: raw,
            _marker: marker::PhantomData,
        }
    }
    fn raw(&self) -> *mut raw::git_config_iterator { self.raw }
}
impl<'cfg, 'b> Iterator for &'b ConfigEntries<'cfg> {
    type Item = Result<ConfigEntry<'b>, Error>;
    fn next(&mut self) -> Option<Result<ConfigEntry<'b>, Error>> {
        let mut raw = ptr::null_mut();
        unsafe {
            Some(Ok(ConfigEntry {
                owned: false,
                raw: raw,
                _marker: marker::PhantomData,
            }))
        }
    }
}
impl<'cfg> Drop for ConfigEntry<'cfg> {
    fn drop(&mut self) {
    }
}
