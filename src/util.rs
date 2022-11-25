pub(crate) trait ExpectOne {
    fn expect_one(&self);
}

impl ExpectOne for i32 {
    fn expect_one(&self) {
        match self {
            1 => (),
            _ => unreachable!("Function always returns 1."),
        }
    }
}
