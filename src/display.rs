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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Display {
    display: NonNull<xlib::Display>,
    name: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    CannotOpenDisplay,
    InvalidDisplayName,
}

impl Display {
    /// Opens an X display
    /// # Arguments
    /// * `name` - Optional display name to open
    ///
    /// # Examples
    /// ```
    /// use x11oo::Display;
    ///
    /// match Display::open(None::<String>) {
    ///     Ok(display) => println!("{:?}", display),
    ///     Err(err) => {
    ///         panic!("{:?}", err);
    ///     }
    /// }
    ///
    /// match Display::open(Some(":0")) {
    ///     Ok(display) => println!("{:?}", display),
    ///     Err(err) => {
    ///         panic!("{:?}", err);
    ///     }
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns a `x11oo::Error` if the display could not be opened
    pub fn open(name: Option<impl Into<String>>) -> Result<Self, Error> {
        name.map_or_else(
            || Self::open_raw(&0, None),
            |name| Self::open_name(name.into()),
        )
    }

    fn open_name(name: String) -> Result<Self, Error> {
        Self::open_raw(
            CString::new(name.as_str())
                .map_err(|_| Error::InvalidDisplayName)?
                .as_ptr(),
            Some(name),
        )
    }

    fn open_raw(display: *const c_char, name: Option<String>) -> Result<Self, Error> {
        let display = unsafe { XOpenDisplay(display) };

        if display.is_null() {
            Err(Error::CannotOpenDisplay)
        } else {
            NonNull::new(display)
                .map(|display| Self { display, name })
                .ok_or(Error::CannotOpenDisplay)
        }
    }

    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// # Panics
    /// Panics on non-zero xlib return code
    pub fn activate_screen_saver(&self) {
        assert_ne!(unsafe { XActivateScreenSaver(self.display.as_ptr()) }, 0);
    }

    #[must_use]
    pub fn add_extension(&self) -> XExtCodes {
        unsafe { *XAddExtension(self.display.as_ptr()) }
    }

    /// # Panics
    /// Panics on non-zero xlib return code
    pub fn add_host(&self, address: NonNull<XHostAddress>) {
        assert_ne!(
            unsafe { XAddHost(self.display.as_ptr(), address.as_ptr()) },
            0
        );
    }

    /// # Panics
    /// Panics on non-zero xlib return code
    pub fn add_hosts(&self, address: NonNull<XHostAddress>, n: i32) {
        assert_ne!(
            unsafe { XAddHosts(self.display.as_ptr(), address.as_ptr(), n) },
            0
        );
    }

    /// # Panics
    /// Panics on non-zero xlib return code
    pub fn add_to_save_set(&self, window: Window) {
        assert_ne!(unsafe { XAddToSaveSet(self.display.as_ptr(), window) }, 0);
    }

    // TODO: implement all xlib functions that take a display as first argument as methods.

    #[must_use]
    pub fn default_root_window(&self) -> Window {
        unsafe { XDefaultRootWindow(self.display.as_ptr()) }
    }

    /// # Panics
    /// Panics on non-zero xlib return code
    pub fn sync(&self, discard: bool) {
        assert_ne!(
            unsafe { XSync(self.display.as_ptr(), i32::from(discard)) },
            0
        );
    }
}

#[cfg(feature = "xfixes")]
impl Display {
    pub fn hide_cursor(&self, window: Window) {
        unsafe { XFixesHideCursor(self.display.as_ptr(), window) };
    }

    pub fn show_cursor(&self, window: Window) {
        unsafe { XFixesShowCursor(self.display.as_ptr(), window) };
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

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CannotOpenDisplay => write!(f, "Cannot open display"),
            Self::InvalidDisplayName => write!(f, "Invalid display name"),
        }
    }
}
