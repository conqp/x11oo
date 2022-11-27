use crate::util::PanicOnError;
use std::ffi::{c_char, CString};
use std::ptr::NonNull;
use x11::xlib::{
    self, Window, XActivateScreenSaver, XAddExtension, XAddHost, XAddHosts, XAddToSaveSet,
    XCloseDisplay, XDefaultRootWindow, XExtCodes, XHostAddress, XOpenDisplay, XSync,
};

#[cfg(feature = "xfixes")]
use x11::xfixes::XFixesHideCursor;

#[derive(Debug)]
pub struct Display {
    display: NonNull<xlib::Display>,
    name: Option<String>,
}

#[derive(Debug)]
pub enum DisplayError {
    CannotOpenDisplay,
    InvalidDisplayName,
}

impl Display {
    pub fn open(name: Option<impl Into<String>>) -> Result<Self, DisplayError> {
        match name {
            Some(name) => Self::open_name(name.into()),
            None => Self::open_raw(&0, None),
        }
    }

    fn open_name(name: String) -> Result<Self, DisplayError> {
        match CString::new(name.clone()) {
            Ok(cstring) => Self::open_raw(cstring.as_ptr(), Some(name)),
            Err(_) => Err(DisplayError::InvalidDisplayName),
        }
    }

    fn open_raw(display: *const c_char, name: Option<String>) -> Result<Self, DisplayError> {
        let display = unsafe { XOpenDisplay(display) };

        if display.is_null() {
            Err(DisplayError::CannotOpenDisplay)
        } else {
            match NonNull::new(display) {
                Some(display) => Ok(Self { display, name }),
                None => Err(DisplayError::CannotOpenDisplay),
            }
        }
    }

    pub fn name(&self) -> Option<&str> {
        match &self.name {
            Some(name) => Some(name),
            None => None,
        }
    }

    pub fn activate_screen_saver(&self) {
        unsafe { XActivateScreenSaver(self.display.as_ptr()) }.panic_if_zero()
    }

    pub fn add_extension(&self) -> XExtCodes {
        unsafe { *XAddExtension(self.display.as_ptr()) }
    }

    pub fn add_host(&self, address: &mut XHostAddress) {
        unsafe { XAddHost(self.display.as_ptr(), address) }.panic_if_zero()
    }

    pub fn add_hosts(&self, address: &mut XHostAddress, n: i32) {
        unsafe { XAddHosts(self.display.as_ptr(), address, n) }.panic_if_zero()
    }

    pub fn add_to_save_set(&self, window: Window) {
        unsafe { XAddToSaveSet(self.display.as_ptr(), window) }.panic_if_zero()
    }

    // TODO: implement all xlib functions that take a display as first argument as methods.

    pub fn default_root_window(&self) -> Window {
        unsafe { XDefaultRootWindow(self.display.as_ptr()) }
    }

    pub fn sync(&self, discard: bool) {
        unsafe { XSync(self.display.as_ptr(), discard as i32) }.panic_if_zero()
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe { XCloseDisplay(self.display.as_ptr()).panic_if_zero() }
    }
}

#[cfg(feature = "xfixes")]
impl Display {
    pub fn hide_cursor(&self, window: Window) {
        unsafe { XFixesHideCursor(self.display.as_ptr(), window) }
    }
}
