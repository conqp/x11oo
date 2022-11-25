use std::ffi::{c_char, CString};

#[cfg(feature = "xfixes")]
use x11::xfixes::XFixesHideCursor;

use x11::xlib::{
    self, Window, XActivateScreenSaver, XAddExtension, XAddHost, XAddHosts, XAddPixel,
    XAddToExtensionList, XDefaultRootWindow, XExtCodes, XExtData, XHostAddress, XImage,
    XOpenDisplay, XSync,
};

use crate::util::discard_const_1;

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
        discard_const_1(
            unsafe { XActivateScreenSaver(self.display) },
            "XActivateScreenSaver",
        )
    }

    pub fn add_extension(&mut self) -> XExtCodes {
        unsafe { *XAddExtension(self.display) }
    }

    pub fn add_host(&mut self, address: &mut XHostAddress) {
        discard_const_1(unsafe { XAddHost(self.display, address) }, "XAddHost")
    }

    pub fn add_hosts(&mut self, address: &mut XHostAddress, n: i32) {
        discard_const_1(unsafe { XAddHosts(self.display, address, n) }, "XAddHosts")
    }

    pub fn add_pixel(&mut self, image: &mut XImage, value: i64) {
        discard_const_1(unsafe { XAddPixel(image, value) }, "XAddPixel")
    }

    pub fn default_root_window(&mut self) -> Window {
        unsafe { XDefaultRootWindow(self.display) }
    }

    pub fn add_to_extension_list(
        &mut self,
        structure: &mut *mut XExtData,
        ext_data: &mut XExtData,
    ) {
        discard_const_1(
            unsafe { XAddToExtensionList(structure, ext_data) },
            "XAddToExtensionList",
        )
    }

    pub fn sync(&mut self, discard: bool) {
        discard_const_1(unsafe { XSync(self.display, discard as i32) }, "XSync")
    }
}

#[cfg(feature = "xfixes")]
impl<'a> Display<'a> {
    pub fn hide_cursor(&mut self, window: Window) {
        unsafe { XFixesHideCursor(self.display, window) }
    }
}
