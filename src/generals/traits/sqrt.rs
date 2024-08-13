pub fn sqrt<T: Sqrt<T>>(val: T) -> T {
    val.sqrt()
}

pub trait Sqrt<VAL> {
    fn sqrt(self) -> VAL;
}

macro_rules! impl_sqrt {
    ($t_self:ty, $mthd:expr) => {
        impl Sqrt<$t_self> for $t_self {
            #[inline]
            fn sqrt(self) -> $t_self {
                ($mthd)(self)
            }
        }
    };
}

impl_sqrt!(f32, f32::sqrt);
impl_sqrt!(f64, f64::sqrt);
