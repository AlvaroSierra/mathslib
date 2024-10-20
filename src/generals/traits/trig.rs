pub trait Trig {
    fn sin(self) -> Self;
    fn asin(self) -> Self;
    fn asinh(self) -> Self;

    fn cos(self) -> Self;
    fn acos(self) -> Self;
    fn acosh(self) -> Self;

    fn tan(self) -> Self;
    fn atan(self) -> Self;
    fn atan2(self, other: Self) -> Self;
    fn atanh(self) -> Self;
}

macro_rules! impl_trig {
    ($t_self:ty) => {
        impl Trig for $t_self {
            #[inline]
            fn sin(self) -> Self {
                self.sin()
            }

            #[inline]
            fn asin(self) -> Self {
                self.asin()
            }
            #[inline]
            fn asinh(self) -> Self {
                self.asinh()
            }

            #[inline]
            fn cos(self) -> Self {
                self.cos()
            }
            #[inline]
            fn acos(self) -> Self {
                self.acos()
            }
            #[inline]
            fn acosh(self) -> Self {
                self.acosh()
            }

            #[inline]
            fn tan(self) -> Self {
                self.tan()
            }
            #[inline]
            fn atan(self) -> Self {
                self.atan()
            }
            #[inline]
            fn atan2(self, other: Self) -> Self {
                self.atan2(other)
            }
            #[inline]
            fn atanh(self) -> Self {
                self.atanh()
            }
        }
    };
}

impl_trig!(f32);
impl_trig!(f64);
