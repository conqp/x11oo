pub(crate) trait PanicOnError {
    fn panic_if_zero(&self);
}

impl PanicOnError for i32 {
    fn panic_if_zero(&self) {
        if self == &0 { panic!("X11 Function returned error.") }
    }
}
