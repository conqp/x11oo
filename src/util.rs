pub(crate) trait Returns<T> {
    fn returns(&self, retval: T);
}

impl Returns<i32> for i32 {
    fn returns(&self, retval: i32) {
        match self {
            _ if *self == retval => (),
            _ => unreachable!("Always returns {}.", retval),
        }
    }
}
