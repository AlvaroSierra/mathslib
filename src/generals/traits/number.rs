pub trait Zero {
    fn zero() -> Self;
}

macro_rules! impl_zero {
    ($t:ty, $v:expr) => {
        impl Zero for $t {
            #[inline]
            fn zero() -> Self {
                $v
            }
        }
    };
}

impl_zero!(u8, 0);
impl_zero!(u16, 0);
impl_zero!(u32, 0);
impl_zero!(u64, 0);
impl_zero!(u128, 0);
impl_zero!(usize, 0);
impl_zero!(f32, 0.0);
impl_zero!(f64, 0.0);
impl_zero!(i8, 0);
impl_zero!(i16, 0);
impl_zero!(i32, 0);
impl_zero!(i64, 0);
impl_zero!(i128, 0);
impl_zero!(isize, 0);

pub trait WholeNumber: Zero {}

impl WholeNumber for u8 {}

impl WholeNumber for u16 {}

impl WholeNumber for u32 {}

impl WholeNumber for u64 {}

impl WholeNumber for u128 {}

impl WholeNumber for usize {}

pub trait IntegerNumber: Zero {}

impl IntegerNumber for i8 {}

impl IntegerNumber for i16 {}

impl IntegerNumber for i32 {}

impl IntegerNumber for i64 {}

impl IntegerNumber for i128 {}

impl IntegerNumber for isize {}

impl<T: WholeNumber> IntegerNumber for T {}

pub trait RealNumber {}

impl RealNumber for f32 {}
impl RealNumber for f64 {}

impl<T: IntegerNumber> RealNumber for T {}

pub trait Number {}

impl<T: RealNumber> Number for T {}
