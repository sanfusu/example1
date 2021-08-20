#![no_std]
pub mod a {
    use core::ops::RangeInclusive;

    pub trait BitIndex {
        fn offset(&self) -> u32;
        fn len(&self) -> u32;
    }

    impl BitIndex for RangeInclusive<u32> {
        #[inline]
        fn offset(&self) -> u32 {
            *self.start()
        }
        #[inline]
        fn len(&self) -> u32 {
            self.end() - self.start() + 1
        }
    }
    impl BitIndex for u32 {
        #[inline]
        fn offset(&self) -> u32 {
            *self
        }
        #[inline]
        fn len(&self) -> u32 {
            1
        }
    }

    pub trait IntoBits<T: BitIndex>
    where
        Self: Sized + Copy,
    {
        type Output: BitsOps<Self>;
        fn bits(self, range: T) -> Self::Output;
    }

    pub struct Bits<R: BitIndex, V: IntoBits<R>> {
        _range: R,
        _value: V,
    }
    impl<T: BitIndex> IntoBits<T> for u8 {
        type Output = Bits<T, Self>;
        fn bits(self, _range: T) -> Self::Output {
            todo! {}
        }
    }

    pub trait BitsOps<T> {
        fn is_clr(&self) -> bool;
    }

    impl<R: BitIndex> BitsOps<u8> for Bits<R, u8> {
        fn is_clr(&self) -> bool {
            todo! {}
        }
    }
}
pub mod b {
    use core::ops::RangeInclusive;

    use super::a::IntoBits;

    pub struct AlignedAddr<T, const POWER: u32> {
        _addr: T,
    }
    impl<T, const POWER: u32> AlignedAddr<T, POWER>
    where
        T: IntoBits<RangeInclusive<u32>> + IntoBits<u32> + From<u8>,
    {
        pub fn foolish() {
            // 1u8.is_clr(); // ERROR COMPLETION HINT
        }
    }

    pub fn foolish() {
        // 1u8.is_clr(); // CORRECTE COMPLETION HINT
    }
}
