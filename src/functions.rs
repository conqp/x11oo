use crate::util::PanicOnError;
use std::borrow::BorrowMut;
use std::ptr::NonNull;
use x11::xlib::{XAddPixel, XAddToExtensionList, XExtData, XImage};

pub fn add_pixel(image: &mut XImage, value: i64) {
    unsafe { XAddPixel(image, value) }.panic_if_zero()
}

pub fn add_to_extension_list(structure: &mut NonNull<XExtData>, ext_data: &mut XExtData) {
    unsafe { XAddToExtensionList(structure.as_ptr().borrow_mut(), ext_data) }.panic_if_zero()
}
