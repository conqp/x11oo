use std::ptr::NonNull;
use x11::xlib::{XAddPixel, XAddToExtensionList, XExtData, XImage};

/// # Panics
/// Panics on non-zero xlib return code
pub fn add_pixel(image: NonNull<XImage>, value: i64) {
    assert_eq!(unsafe { XAddPixel(image.as_ptr(), value) }, 0);
}

/// # Panics
/// Panics on non-zero xlib return code
pub fn add_to_extension_list(structure: NonNull<*mut XExtData>, ext_data: NonNull<XExtData>) {
    assert_eq!(
        unsafe { XAddToExtensionList(structure.as_ptr(), ext_data.as_ptr()) },
        0
    );
}
