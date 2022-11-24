#[inline]
pub(crate) fn discard_const_1(retval: i32, name: &str) {
    match retval {
        1 => (),
        _ => unreachable!("{} always returns 1.", name),
    }
}
