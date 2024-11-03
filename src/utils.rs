pub trait NumExt {
    /// Returns `true` if self is an odd number.
    fn is_odd(self) -> bool;
}

impl NumExt for usize {
    #[inline]
    fn is_odd(self) -> bool {
        (self & 1) == 1
    }
}

impl NumExt for isize {
    #[inline]
    fn is_odd(self) -> bool {
        (self & 1) == 1
    }
}

impl NumExt for i8 {
    #[inline]
    fn is_odd(self) -> bool {
        (self & 1) == 1
    }
}

impl NumExt for i32 {
    #[inline]
    fn is_odd(self) -> bool {
        (self & 1) == 1
    }
}

impl NumExt for i64 {
    #[inline]
    fn is_odd(self) -> bool {
        (self & 1) == 1
    }
}

impl NumExt for i128 {
    #[inline]
    fn is_odd(self) -> bool {
        (self & 1) == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_odd() {
        assert_eq!((1).is_odd(), true);
        assert_eq!((3).is_odd(), true);
        assert_eq!((5).is_odd(), true);
        assert_eq!((7).is_odd(), true);
        assert_eq!((9).is_odd(), true);
        assert_eq!((11).is_odd(), true);
        assert_eq!((0).is_odd(), false);
        assert_eq!((2).is_odd(), false);
        assert_eq!((4).is_odd(), false);
        assert_eq!((6).is_odd(), false);
        assert_eq!((8).is_odd(), false);
        assert_eq!((10).is_odd(), false);
    }
}
