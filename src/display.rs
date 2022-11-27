use crate::util::IsAlways;
use std::ffi::{c_char, CString};
use x11::xlib::{
    self, Window, XActivateScreenSaver, XAddExtension, XAddHost, XAddHosts, XAddToSaveSet,
    XDefaultRootWindow, XExtCodes, XHostAddress, XOpenDisplay, XSync,
};

#[cfg(feature = "xfixes")]
use x11::xfixes::XFixesHideCursor;

#[derive(Debug)]
pub struct Display {
    display: *mut xlib::Display,
    name: Option<String>,
}

impl Display {
    pub fn open(name: Option<impl Into<String>>) -> Option<Self> {
        match name {
            Some(name) => Self::open_name(name.into()),
            None => Self::open_raw(&0, None),
        }
    }

    fn open_name(name: String) -> Option<Self> {
        match CString::new(name.clone()) {
            Ok(cstring) => Self::open_raw(cstring.as_ptr(), Some(name)),
            Err(_) => None,
        }
    }

    fn open_raw(display: *const c_char, name: Option<String>) -> Option<Self> {
        unsafe { XOpenDisplay(display).as_mut() }.map(|display| Self { display, name })
    }

    pub fn name(&self) -> Option<&str> {
        match &self.name {
            Some(name) => Some(name),
            None => None,
        }
    }

    pub fn activate_screen_saver(&mut self) {
        unsafe { XActivateScreenSaver(self.display) }.is_always(1)
    }

    pub fn add_extension(&mut self) -> XExtCodes {
        unsafe { *XAddExtension(self.display) }
    }

    pub fn add_host(&mut self, address: &mut XHostAddress) {
        unsafe { XAddHost(self.display, address) }.is_always(1)
    }

    pub fn add_hosts(&mut self, address: &mut XHostAddress, n: i32) {
        unsafe { XAddHosts(self.display, address, n) }.is_always(1)
    }

    pub fn add_to_save_set(&mut self, window: Window) {
        unsafe { XAddToSaveSet(self.display, window) }.is_always(1)
    }

    // TODO: implement all xlib functions that take a display as first argument as methods.

    pub fn default_root_window(&mut self) -> Window {
        unsafe { XDefaultRootWindow(self.display) }
    }

    pub fn sync(&mut self, discard: bool) {
        unsafe { XSync(self.display, discard as i32) }.is_always(1)
    }
}

#[cfg(feature = "xfixes")]
impl Display {
    pub fn hide_cursor(&mut self, window: Window) {
        unsafe { XFixesHideCursor(self.display, window) }
    }
}
