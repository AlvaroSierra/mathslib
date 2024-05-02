/// Implements a standard trait for exponential for most of the types
///
///
///
/// # TODO
/// - Implement assert_near_equals to handle the floating point number imprecision
/// - Include saturated, checked and wraparound functions in the trait
/// - Implement for referenced values
/// - Implement solving for complex numbers when required

#[cfg(test)]
mod tests {
    use super::{pow, Pow};
    ///
    /// These tests don't include validation for real numbers to the power of a float (int^float) due to the possibility of resulting in a complex number not yet implemented
    use std::fmt::Debug;

    fn test<T: Pow<P, Q> + Copy, P: Clone, Q: PartialEq + Debug>(base: T, exp: P, result: Q) {
        assert_eq!(base.pow(exp.clone()), result);
        assert_eq!(pow(base, exp), result)
    }

    #[test]
    fn pow_fif() {
        self::test(0.4f64, -5, 97.65624999999994);
    }

    #[test]
    fn pow_fuf() {
        self::test(0.4f64, 5, 0.010240000000000006);
    }

    #[test]
    fn pow_uif() {
        self::test(5u16, -2i16, 0.04);
    }

    #[test]
    fn pow_uff() {
        self::test(5u16, 0.5f32, 2.23606797749979);
    }

    #[test]
    fn pow_iif() {
        self::test(-2i32, -2i32, 0.25);
    }

    #[test]
    fn pow_uuu() {
        self::test(2u8, 2u8, 4u8);
    }

    #[test]
    fn pow_iui() {
        self::test(-2i8, 3u8, -8i8);
    }
}

/// Sets a to the
pub fn pow<T: Pow<P, Q>, P, Q>(base: T, exp: P) -> Q {
    base.pow(exp)
}

/// Trait exposing the `pow` function to raise a number to the power of
///
///
///
/// The trait is implemented for
/// - i8 to power of u8 - u32 returns i8 - isize
/// - i16 to power of u8 - u32 returns i16 - isize
/// - i32 to power of u8 - u32 returns i32 - isize
/// - i64 to power of u8 - u32 returns i64 - isize
/// - i168 to power of u8 - u32 returns i168 - isize
/// - isize to power of u8 - u32 returns isize
/// - u8 to power of u8 - u32 returns u8 - usize
/// - u16 to power of u8 - u32 returns u16 - usize
/// - u32 to power of u8 - u32 returns u32 - usize
/// - u64 to power of u8 - u32 returns u64 - usize
/// - u168 to power of u8 - u32 returns u168 - usize
/// - usize to power of u8 - u32 returns usize
/// - f32 to the power of f32 returning an f32 and f64
/// - f64 to the power of f32 returning an f64
/// - i8 - i32 to the power of f32 returning f64
/// - i8 - i16 to the power of f32 returning f32
/// - u8 - u32 to the power of f32 returning f64
/// - u8 - u16 to the power of f32 returning f32
/// - f32 - f64 to the power of i8 - i32 returning f32-f64
/// - i8 - i32 to the power of i8 - i32 returning f64
/// - i8 - i16 to the power of i8 - i32 returning f32
/// - u8 - u32 to the power of i8 - i32 returning f64
/// - u8 - u16 to the power of i8 - i32 returning f32
///
/// Note if the operation within real numbers it returns T::NaN
pub trait Pow<EXP, OUT> {
    fn pow(self, exp: EXP) -> OUT;
}

macro_rules! impl_pow {
    ($t_self:ty, $exp:ty, $out:ty, $mthd:expr) => {
        impl Pow<$exp, $out> for $t_self {
            #[inline]
            fn pow(self, exp: $exp) -> $out {
                ($mthd)(self.into(), exp.into())
            }
        }
    };
}

macro_rules! implu_pow {
    ($t_self:ty, $t_out:ty, $mthd:expr) => {
        impl_pow!($t_self, u8, $t_out, $mthd);
        impl_pow!($t_self, u16, $t_out, $mthd);
        impl_pow!($t_self, u32, $t_out, $mthd);
    };
}

// For signed ints outputs
implu_pow!(i8, i8, i8::pow);
implu_pow!(i8, i16, i16::pow);
implu_pow!(i8, i32, i32::pow);
implu_pow!(i8, i64, i64::pow);
implu_pow!(i8, i128, i128::pow);
implu_pow!(i8, isize, isize::pow);

implu_pow!(i16, i16, i16::pow);
implu_pow!(i16, i32, i32::pow);
implu_pow!(i16, i64, i64::pow);
implu_pow!(i16, i128, i128::pow);
implu_pow!(i16, isize, isize::pow);

implu_pow!(i32, i32, i32::pow);
implu_pow!(i32, i64, i64::pow);
implu_pow!(i32, i128, i128::pow);

implu_pow!(i64, i64, i64::pow);
implu_pow!(i64, i128, i128::pow);

implu_pow!(i128, i128, i128::pow);

implu_pow!(isize, isize, isize::pow);

// For unsigned ints outputs
implu_pow!(u8, u8, u8::pow);
implu_pow!(u8, u16, u16::pow);
implu_pow!(u8, u32, u32::pow);
implu_pow!(u8, u64, u64::pow);
implu_pow!(u8, u128, u128::pow);
implu_pow!(u8, usize, usize::pow);

implu_pow!(u16, u16, u16::pow);
implu_pow!(u16, u32, u32::pow);
implu_pow!(u16, u64, u64::pow);
implu_pow!(u16, u128, u128::pow);
implu_pow!(u16, usize, usize::pow);

implu_pow!(u32, u32, u32::pow);
implu_pow!(u32, u64, u64::pow);
implu_pow!(u32, u128, u128::pow);

implu_pow!(u64, u64, u64::pow);
implu_pow!(u64, u128, u128::pow);

implu_pow!(u128, u128, u128::pow);

implu_pow!(usize, usize, usize::pow);

// For any float outputs
impl_pow!(f32, f32, f32, f32::powf);
impl_pow!(f32, f32, f64, f64::powf);
impl_pow!(f64, f32, f64, f64::powf);

impl_pow!(i8, f32, f32, f32::powf);
impl_pow!(i16, f32, f32, f32::powf);

impl_pow!(i8, f32, f64, f64::powf);
impl_pow!(i16, f32, f64, f64::powf);
impl_pow!(i32, f32, f64, f64::powf);

impl_pow!(u8, f32, f32, f32::powf);
impl_pow!(u16, f32, f32, f32::powf);

impl_pow!(u8, f32, f64, f64::powf);
impl_pow!(u16, f32, f64, f64::powf);
impl_pow!(u32, f32, f64, f64::powf);

macro_rules! implf_pow {
    ($exp:ty) => {
        impl_pow!(f32, $exp, f32, f32::powi);
        impl_pow!(f64, $exp, f64, f64::powi);

        impl_pow!(i8, $exp, f32, f32::powi);
        impl_pow!(i16, $exp, f32, f32::powi);

        impl_pow!(i8, $exp, f64, f64::powi);
        impl_pow!(i16, $exp, f64, f64::powi);
        impl_pow!(i32, $exp, f64, f64::powi);

        impl_pow!(u8, $exp, f32, f32::powi);
        impl_pow!(u16, $exp, f32, f32::powi);

        impl_pow!(u8, $exp, f64, f64::powi);
        impl_pow!(u16, $exp, f64, f64::powi);
        impl_pow!(u32, $exp, f64, f64::powi);
    };
}

implf_pow!(i32);
implf_pow!(i16);
implf_pow!(i8);
