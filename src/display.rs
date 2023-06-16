use crate::util::PanicOr;
use std::ffi::{c_char, CString};
use std::fmt::Formatter;
use std::ptr::NonNull;
use x11::xlib::{
    self, Window, XActivateScreenSaver, XAddExtension, XAddHost, XAddHosts, XAddToSaveSet,
    XCloseDisplay, XDefaultRootWindow, XExtCodes, XHostAddress, XOpenDisplay, XSync,
};

#[cfg(feature = "xfixes")]
use x11::xfixes::XFixesHideCursor;
#[cfg(feature = "xfixes")]
use x11::xfixes::XFixesShowCursor;

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
        Self::open_raw(
            CString::new(name.as_str())
                .map_err(|_| DisplayError::InvalidDisplayName)?
                .as_ptr(),
            Some(name),
        )
    }

    fn open_raw(display: *const c_char, name: Option<String>) -> Result<Self, DisplayError> {
        let display = unsafe { XOpenDisplay(display) };

        if display.is_null() {
            Err(DisplayError::CannotOpenDisplay)
        } else {
            NonNull::new(display)
                .map(|display| Self { display, name })
                .ok_or(DisplayError::CannotOpenDisplay)
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn activate_screen_saver(&self) -> &Self {
        unsafe { XActivateScreenSaver(self.display.as_ptr()) }.panic_or(self)
    }

    pub fn add_extension(&self) -> XExtCodes {
        unsafe { *XAddExtension(self.display.as_ptr()) }
    }

    pub fn add_host(&self, address: NonNull<XHostAddress>) -> &Self {
        unsafe { XAddHost(self.display.as_ptr(), address.as_ptr()) }.panic_or(self)
    }

    pub fn add_hosts(&self, address: NonNull<XHostAddress>, n: i32) -> &Self {
        unsafe { XAddHosts(self.display.as_ptr(), address.as_ptr(), n) }.panic_or(self)
    }

    pub fn add_to_save_set(&self, window: Window) -> &Self {
        unsafe { XAddToSaveSet(self.display.as_ptr(), window) }.panic_or(self)
    }

    // TODO: implement all xlib functions that take a display as first argument as methods.

    pub fn default_root_window(&self) -> Window {
        unsafe { XDefaultRootWindow(self.display.as_ptr()) }
    }

    pub fn sync(&self, discard: bool) -> &Self {
        unsafe { XSync(self.display.as_ptr(), discard as i32) }.panic_or(self)
    }
}

#[cfg(feature = "xfixes")]
impl Display {
    pub fn hide_cursor(&self, window: Window) -> &Self {
        unsafe { XFixesHideCursor(self.display.as_ptr(), window) };
        self
    }

    pub fn show_cursor(&self, window: Window) -> &Self {
        unsafe { XFixesShowCursor(self.display.as_ptr(), window) };
        self
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        match unsafe { XCloseDisplay(self.display.as_ptr()) } {
            0 => (),
            _ => panic!("Could not close display"),
        }
    }
}

impl std::fmt::Display for DisplayError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CannotOpenDisplay => write!(f, "Cannot open display"),
            Self::InvalidDisplayName => write!(f, "Invalid display name"),
        }
    }
}
