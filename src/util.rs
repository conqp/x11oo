pub(crate) trait PanicOr<T> {
    fn panic_or(self, t: T) -> T;
}

impl<T> PanicOr<T> for i32 {
    fn panic_or(self, t: T) -> T {
        if self == 0 {
            panic!("xlib function returned error")
        }

        t
    }
}
