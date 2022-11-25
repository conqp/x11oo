pub(crate) trait IsAlways<T> {
    fn is_always(&self, retval: T);
}

impl IsAlways<i32> for i32 {
    fn is_always(&self, retval: i32) {
        match self {
            _ if *self == retval => (),
            _ => unreachable!("Always returns {}.", retval),
        }
    }
}
