use crate::util::PanicIfZero;
use std::ptr::NonNull;
use x11::xlib::{XAddPixel, XAddToExtensionList, XExtData, XImage};

pub fn add_pixel(image: NonNull<XImage>, value: i64) {
    unsafe { XAddPixel(image.as_ptr(), value) }.panic_if_zero()
}

pub fn add_to_extension_list(structure: NonNull<*mut XExtData>, ext_data: NonNull<XExtData>) {
    unsafe { XAddToExtensionList(structure.as_ptr(), ext_data.as_ptr()) }.panic_if_zero()
}
