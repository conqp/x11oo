pub(crate) trait PanicIfZero {
    fn panic_if_zero(&self);
}

impl PanicIfZero for i32 {
    fn panic_if_zero(&self) {
        if self == &0 {
            panic!("xlib function returned error")
        }
    }
}
