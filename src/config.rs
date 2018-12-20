use std::marker;
use std::ptr;
use std::str;
use {raw, Error, ConfigLevel, Buf, IntoCString};
use util::{self, Binding};
pub struct Config {
    raw: *mut raw::git_config,
}
pub struct ConfigEntry<'cfg> {
    raw: *mut raw::git_config_entry,
    _marker: marker::PhantomData<&'cfg Config>,
    owned: bool,
}
pub struct ConfigEntries<'cfg> {
    raw: *mut raw::git_config_iterator,
    _marker: marker::PhantomData<&'cfg Config>,
}
impl Config {
    pub fn get_string(&self, name: &str) -> Result<String, Error> {
        let ret = Buf::new();
        str::from_utf8(&ret).map(|s| s.to_string()).map_err(|_| {
            Error::from_str("configuration value is not valid utf8")
        })
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
