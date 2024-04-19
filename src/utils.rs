pub trait NumExt {
    /// Returns `true` if self is an odd number.
    fn is_odd(self) -> bool;
}

impl NumExt for usize {
    #[inline]
    fn is_odd(self) -> bool {
        self % 2 == 1
    }
}

impl NumExt for isize {
    #[inline]
    fn is_odd(self) -> bool {
        self % 2 == 1
    }
}
