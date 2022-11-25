use crate::util::IsAlways;
use std::borrow::BorrowMut;
use std::ops::DerefMut;
use x11::xlib::{XAddPixel, XAddToExtensionList, XExtData, XImage};

pub fn add_pixel(image: &mut XImage, value: i64) {
    unsafe { XAddPixel(image, value) }.is_always(1)
}

pub fn add_to_extension_list(structure: &mut &mut XExtData, ext_data: &mut XExtData) {
    unsafe {
        XAddToExtensionList(
            (structure.deref_mut() as *mut XExtData).borrow_mut(),
            ext_data,
        )
    }
    .is_always(1)
}
