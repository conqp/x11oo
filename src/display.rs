use crate::util::Returns;
use std::ffi::{c_char, CString};
use x11::xlib::{
    self, Window, XActivateScreenSaver, XAddExtension, XAddHost, XAddHosts, XAddToSaveSet,
    XDefaultRootWindow, XExtCodes, XHostAddress, XOpenDisplay, XSync,
};

#[cfg(feature = "xfixes")]
use x11::xfixes::XFixesHideCursor;

pub struct Display<'a> {
    display: &'a mut xlib::Display,
}

impl<'a> Display<'a> {
    pub fn open(name: Option<impl Into<String>>) -> Option<Self> {
        match name {
            Some(name) => match CString::new(name.into()) {
                Ok(name) => Self::open_raw(name.as_ptr()),
                Err(_) => None,
            },
            None => Self::open_raw(&0),
        }
    }

    fn open_raw(display: *const c_char) -> Option<Self> {
        unsafe { XOpenDisplay(display).as_mut() }.map(|display| Self { display })
    }

    pub fn activate_screen_saver(&mut self) {
        unsafe { XActivateScreenSaver(self.display) }.returns(1)
    }

    pub fn add_extension(&mut self) -> XExtCodes {
        unsafe { *XAddExtension(self.display) }
    }

    pub fn add_host(&mut self, address: &mut XHostAddress) {
        unsafe { XAddHost(self.display, address) }.returns(1)
    }

    pub fn add_hosts(&mut self, address: &mut XHostAddress, n: i32) {
        unsafe { XAddHosts(self.display, address, n) }.returns(1)
    }

    pub fn add_to_save_set(&mut self, window: Window) {
        unsafe { XAddToSaveSet(self.display, window) }.returns(1)
    }

    // TODO: implement all xlib functions that take a display as first argument as methods.

    pub fn default_root_window(&mut self) -> Window {
        unsafe { XDefaultRootWindow(self.display) }
    }

    pub fn sync(&mut self, discard: bool) {
        unsafe { XSync(self.display, discard as i32) }.returns(1)
    }
}

#[cfg(feature = "xfixes")]
impl<'a> Display<'a> {
    pub fn hide_cursor(&mut self, window: Window) {
        unsafe { XFixesHideCursor(self.display, window) }
    }
}
