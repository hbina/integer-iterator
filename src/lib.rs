/// A simple wrapper type over the integer type `T` because `Rust` reserves the right to implement
/// `Iterator` for them.
///
/// # Example
///
/// ```rust
/// # use integer_iterator::DigitIterator;
/// # pub fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let mut y = DigitIterator(30u32);
/// assert_eq!(y.next(), Some(0));
/// assert_eq!(y.next(), Some(3));
/// assert_eq!(y.next(), None);
/// # Ok(())
/// # }
/// ```
///
pub struct DigitIterator<T>(pub T); // TODO :: Where T is an integer?

/// Extends a type `T` with the ability to become a [`DigitIterator`].
pub trait IntoDigits<T> {
    fn digits(self) -> DigitIterator<T>;
}

macro_rules! impl_digit_positive {
    ($T:ty) => {
        impl IntoDigits<$T> for $T {
            fn digits(self) -> DigitIterator<$T> {
                DigitIterator(self)
            }
        }

        impl Iterator for DigitIterator<$T> {
            type Item = u8;

            fn next(&mut self) -> Option<Self::Item> {
                if self.0 != 0 {
                    let remainder = (self.0 % (10 as $T)) as Self::Item;
                    self.0 /= (10 as $T);
                    Some(remainder)
                } else {
                    None
                }
            }
        }
    };
}

macro_rules! impl_digit_negative {
    ($T:ty) => {
        impl IntoDigits<$T> for $T {
            fn digits(self) -> DigitIterator<$T> {
                DigitIterator(-self)
            }
        }

        impl Iterator for DigitIterator<$T> {
            type Item = u8;

            fn next(&mut self) -> Option<Self::Item> {
                if self.0 != 0 {
                    let remainder = (self.0 % (10 as $T)) as Self::Item;
                    self.0 /= (10 as $T);
                    Some(remainder)
                } else {
                    None
                }
            }
        }
    };
}

impl_digit_positive!(u8);
impl_digit_positive!(u16);
impl_digit_positive!(u32);
impl_digit_positive!(u64);
impl_digit_positive!(u128);

impl_digit_negative!(i8);
impl_digit_negative!(i16);
impl_digit_negative!(i32);
impl_digit_negative!(i64);
impl_digit_negative!(i128);

#[test]
fn basic() {
    let mut positive = 1234u32.digits();
    assert_eq!(positive.next(), Some(4));
    assert_eq!(positive.next(), Some(3));
    assert_eq!(positive.next(), Some(2));
    assert_eq!(positive.next(), Some(1));

    let mut negative = (-1234i32).digits();
    assert_eq!(negative.next(), Some(4));
    assert_eq!(negative.next(), Some(3));
    assert_eq!(negative.next(), Some(2));
    assert_eq!(negative.next(), Some(1));
}
