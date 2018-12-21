use std::marker;
use std::ptr;
use {raw, Error, ConfigLevel, IntoCString};
pub struct Config {
}
pub struct ConfigEntry<'cfg> {
    raw: *mut raw::git_config_entry,
    _marker: marker::PhantomData<&'cfg Config>,
    owned: bool,
}
pub struct ConfigEntries<'cfg> {
    _marker: marker::PhantomData<&'cfg Config>,
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
