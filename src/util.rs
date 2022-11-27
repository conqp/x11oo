pub(crate) trait PanicOnError {
    fn panic_if_zero(&self);
}

impl PanicOnError for i32 {
    fn panic_if_zero(&self) {
        if self == &0 {
            panic!("xlib function returned error")
        }
    }
}
