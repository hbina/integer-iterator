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
pub struct DigitIterator<T>(pub T, bool); // TODO :: Where T is an integer?

/// Extends a type `T` with the ability to become a [`DigitIterator`].
pub trait IntoDigits<T> {
    fn digits(self) -> DigitIterator<T>;
}

macro_rules! impl_digit_positive {
    ($T:ty) => {
        impl IntoDigits<$T> for $T {
            fn digits(self) -> DigitIterator<$T> {
                DigitIterator(self, self == 0)
            }
        }

        impl Iterator for DigitIterator<$T> {
            type Item = u8;

            fn next(&mut self) -> Option<Self::Item> {
                if self.1 {
                    self.1 = false;
                    Some(0)
                } else if self.0 != 0 {
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
                DigitIterator(-self, self == 0)
            }
        }

        impl Iterator for DigitIterator<$T> {
            type Item = u8;

            fn next(&mut self) -> Option<Self::Item> {
                if self.1 {
                    self.1 = false;
                    Some(0)
                } else if self.0 != 0 {
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
fn test_all_numbers() {
    for x in u128::MIN..10000u128 {
        let length = x
            .digits()
            .enumerate()
            .map(|(idx, _)| idx + 1)
            .last()
            .unwrap();
        assert_eq!(length, x.to_string().len());
    }

    for x in 0..-10000i128 {
        let length = x
            .digits()
            .enumerate()
            .map(|(idx, _)| idx + 1)
            .last()
            .unwrap();
        assert_eq!(length, x.to_string().len());
    }
}
