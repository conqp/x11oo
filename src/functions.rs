use crate::util::discard_const_1;
use std::borrow::BorrowMut;
use std::ops::DerefMut;
use x11::xlib::{XAddPixel, XAddToExtensionList, XExtData, XImage};

pub fn add_pixel(image: &mut XImage, value: i64) {
    discard_const_1(unsafe { XAddPixel(image, value) }, "XAddPixel")
}

pub fn add_to_extension_list(structure: &mut &mut XExtData, ext_data: &mut XExtData) {
    discard_const_1(
        unsafe {
            XAddToExtensionList(
                (structure.deref_mut() as *mut XExtData).borrow_mut(),
                ext_data,
            )
        },
        "XAddToExtensionList",
    )
}
