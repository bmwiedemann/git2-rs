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
        let name = try!(CString::new(name));
        let regexp = try!(CString::new(regexp));
        unsafe {
            try_call!(raw::git_config_delete_multivar(self.raw, name, regexp));
        }
        Ok(())
    }

    /// Get the value of a boolean config variable.
    ///
    /// All config files will be looked into, in the order of their defined
    /// level. A higher level means a higher priority. The first occurrence of
    /// the variable will be returned here.
    pub fn get_bool(&self, name: &str) -> Result<bool, Error> {
        let mut out = 0 as libc::c_int;
        let name = try!(CString::new(name));
        unsafe {
            try_call!(raw::git_config_get_bool(&mut out, &*self.raw, name));

        }
        Ok(!(out == 0))
    }

    /// Get the value of an integer config variable.
    ///
    /// All config files will be looked into, in the order of their defined
    /// level. A higher level means a higher priority. The first occurrence of
    /// the variable will be returned here.
    pub fn get_i32(&self, name: &str) -> Result<i32, Error> {
        let mut out = 0i32;
        let name = try!(CString::new(name));
        unsafe {
            try_call!(raw::git_config_get_int32(&mut out, &*self.raw, name));

        }
        Ok(out)
    }

    /// Get the value of an integer config variable.
    ///
    /// All config files will be looked into, in the order of their defined
    /// level. A higher level means a higher priority. The first occurrence of
    /// the variable will be returned here.
    pub fn get_i64(&self, name: &str) -> Result<i64, Error> {
        let mut out = 0i64;
        let name = try!(CString::new(name));
        unsafe {
            try_call!(raw::git_config_get_int64(&mut out, &*self.raw, name));
        }
        Ok(out)
    }

    /// Get the value of a string config variable.
    ///
    /// This is the same as `get_bytes` except that it may return `Err` if
    /// the bytes are not valid utf-8.
    pub fn get_str(&self, name: &str) -> Result<&str, Error> {
        str::from_utf8(try!(self.get_bytes(name))).map_err(|_| {
            Error::from_str("configuration value is not valid utf8")
        })
    }

    /// Get the value of a string config variable as a byte slice.
    ///
    /// This method will return an error if this `Config` is not a snapshot.
    pub fn get_bytes(&self, name: &str) -> Result<&[u8], Error> {
        let mut ret = ptr::null();
        let name = try!(CString::new(name));
        unsafe {
            try_call!(raw::git_config_get_string(&mut ret, &*self.raw, name));
            Ok(::opt_bytes(self, ret).unwrap())
        }
    }

    /// Get the value of a string config variable as an owned string.
    ///
    /// An error will be returned if the config value is not valid utf-8.
    pub fn get_string(&self, name: &str) -> Result<String, Error> {
        let ret = Buf::new();
        let name = try!(CString::new(name));
        unsafe {
            try_call!(raw::git_config_get_string_buf(ret.raw(), self.raw, name));
        }
        str::from_utf8(&ret).map(|s| s.to_string()).map_err(|_| {
            Error::from_str("configuration value is not valid utf8")
        })
    }

    /// Get the value of a path config variable as an owned .
    pub fn get_path(&self, name: &str) -> Result<PathBuf, Error> {
        let ret = Buf::new();
        let name = try!(CString::new(name));
        unsafe {
            try_call!(raw::git_config_get_path(ret.raw(), self.raw, name));
        }
        Ok(::util::bytes2path(&ret).to_path_buf())
    }

    /// Get the ConfigEntry for a config variable.
    pub fn get_entry(&self, name: &str) -> Result<ConfigEntry, Error> {
        let mut ret = ptr::null_mut();
        let name = try!(CString::new(name));
        unsafe {
            try_call!(raw::git_config_get_entry(&mut ret, self.raw, name));
            Ok(Binding::from_raw(ret))
        }
    }

    /// Iterate over all the config variables
    ///
    /// If `glob` is `Some`, then the iterator will only iterate over all
    /// variables whose name matches the pattern.
    ///
    /// # Example
    ///
    /// ```
    /// # #![allow(unstable)]
    /// use git2::Config;
    ///
    /// let cfg = Config::new().unwrap();
    ///
    /// for entry in &cfg.entries(None).unwrap() {
    ///     let entry = entry.unwrap();
    ///     println!("{} => {}", entry.name().unwrap(), entry.value().unwrap());
    /// }
    /// ```
    pub fn entries(&self, glob: Option<&str>) -> Result<ConfigEntries, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            match glob {
                Some(s) => {
                    let s = try!(CString::new(s));
                    try_call!(raw::git_config_iterator_glob_new(&mut ret,
                                                                &*self.raw,
                                                                s));
                }
                None => {
                    try_call!(raw::git_config_iterator_new(&mut ret, &*self.raw));
                }
            }
            Ok(Binding::from_raw(ret))
        }
    }

    /// Open the global/XDG configuration file according to git's rules
    ///
    /// Git allows you to store your global configuration at `$HOME/.config` or
    /// `$XDG_CONFIG_HOME/git/config`. For backwards compatability, the XDG file
    /// shouldn't be used unless the use has created it explicitly. With this
    /// function you'll open the correct one to write to.
    pub fn open_global(&mut self) -> Result<Config, Error> {
        let mut raw = ptr::null_mut();
        unsafe {
            try_call!(raw::git_config_open_global(&mut raw, self.raw));
            Ok(Binding::from_raw(raw))
        }
    }

    /// Build a single-level focused config object from a multi-level one.
    ///
    /// The returned config object can be used to perform get/set/delete
    /// operations on a single specific level.
    pub fn open_level(&self, level: ConfigLevel) -> Result<Config, Error> {
        let mut raw = ptr::null_mut();
        unsafe {
            try_call!(raw::git_config_open_level(&mut raw, &*self.raw, level));
            Ok(Binding::from_raw(raw))
        }
    }

    /// Set the value of a boolean config variable in the config file with the
    /// highest level (usually the local one).
    pub fn set_bool(&mut self, name: &str, value: bool) -> Result<(), Error> {
        let name = try!(CString::new(name));
        unsafe {
            try_call!(raw::git_config_set_bool(self.raw, name, value));
        }
        Ok(())
    }

    /// Set the value of an integer config variable in the config file with the
    /// highest level (usually the local one).
    pub fn set_i32(&mut self, name: &str, value: i32) -> Result<(), Error> {
        let name = try!(CString::new(name));
        unsafe {
            try_call!(raw::git_config_set_int32(self.raw, name, value));
        }
        Ok(())
    }

    /// Set the value of an integer config variable in the config file with the
    /// highest level (usually the local one).
    pub fn set_i64(&mut self, name: &str, value: i64) -> Result<(), Error> {
        let name = try!(CString::new(name));
        unsafe {
            try_call!(raw::git_config_set_int64(self.raw, name, value));
        }
        Ok(())
    }

    /// Set the value of an multivar config variable in the config file with the
    /// highest level (usually the local one).
    pub fn set_multivar(&mut self, name: &str, regexp: &str, value: &str) -> Result<(), Error> {
        let name = try!(CString::new(name));
        let regexp = try!(CString::new(regexp));
        let value = try!(CString::new(value));
        unsafe {
            try_call!(raw::git_config_set_multivar(self.raw, name, regexp, value));
        }
        Ok(())
    }

    /// Set the value of a string config variable in the config file with the
    /// highest level (usually the local one).
    pub fn set_str(&mut self, name: &str, value: &str) -> Result<(), Error> {
        let name = try!(CString::new(name));
        let value = try!(CString::new(value));
        unsafe {
            try_call!(raw::git_config_set_string(self.raw, name, value));
        }
        Ok(())
    }

    /// Create a snapshot of the configuration
    ///
    /// Create a snapshot of the current state of a configuration, which allows
    /// you to look into a consistent view of the configuration for looking up
    /// complex values (e.g. a remote, submodule).
    pub fn snapshot(&mut self) -> Result<Config, Error> {
        let mut ret = ptr::null_mut();
        unsafe {
            try_call!(raw::git_config_snapshot(&mut ret, self.raw));
            Ok(Binding::from_raw(ret))
        }
    }

    /// Parse a string as a bool.
    /// Interprets "true", "yes", "on", 1, or any non-zero number as true.
    /// Interprets "false", "no", "off", 0, or an empty string as false.
    pub fn parse_bool<S: IntoCString>(s: S) -> Result<bool, Error> {
        let s = try!(s.into_c_string());
        let mut out = 0;
        ::init();
        unsafe {
            try_call!(raw::git_config_parse_bool(&mut out, s));
        }
        Ok(out != 0)
    }

    /// Parse a string as an i32; handles suffixes like k, M, or G, and
    /// multiplies by the appropriate power of 1024.
    pub fn parse_i32<S: IntoCString>(s: S) -> Result<i32, Error> {
        let s = try!(s.into_c_string());
        let mut out = 0;
        ::init();
        unsafe {
            try_call!(raw::git_config_parse_int32(&mut out, s));
        }
        Ok(out)
    }

    /// Parse a string as an i64; handles suffixes like k, M, or G, and
    /// multiplies by the appropriate power of 1024.
    pub fn parse_i64<S: IntoCString>(s: S) -> Result<i64, Error> {
        let s = try!(s.into_c_string());
        let mut out = 0;
        ::init();
        unsafe {
            try_call!(raw::git_config_parse_int64(&mut out, s));
        }
        Ok(out)
    }
}

impl Binding for Config {
    type Raw = *mut raw::git_config;
    unsafe fn from_raw(raw: *mut raw::git_config) -> Config {
        Config { raw: raw }
    }
    fn raw(&self) -> *mut raw::git_config { self.raw }
}

impl Drop for Config {
    fn drop(&mut self) {
        unsafe { raw::git_config_free(self.raw) }
    }
}

impl<'cfg> ConfigEntry<'cfg> {
    /// Gets the name of this entry.
    ///
    /// May return `None` if the name is not valid utf-8
    pub fn name(&self) -> Option<&str> { str::from_utf8(self.name_bytes()).ok() }

    /// Gets the name of this entry as a byte slice.
    pub fn name_bytes(&self) -> &[u8] {
        unsafe { ::opt_bytes(self, (*self.raw).name).unwrap() }
    }

    /// Gets the value of this entry.
    ///
    /// May return `None` if the value is not valid utf-8
    pub fn value(&self) -> Option<&str> { str::from_utf8(self.value_bytes()).ok() }

    /// Gets the value of this entry as a byte slice.
    pub fn value_bytes(&self) -> &[u8] {
        unsafe { ::opt_bytes(self, (*self.raw).value).unwrap() }
    }

    /// Gets the configuration level of this entry.
    pub fn level(&self) -> ConfigLevel {
        unsafe { ConfigLevel::from_raw((*self.raw).level) }
    }

	/// Depth of includes where this variable was found
    pub fn include_depth(&self) -> u32 {
        unsafe { (*self.raw).include_depth as u32 }
    }
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
