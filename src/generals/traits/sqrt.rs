pub fn sqrt<T: Sqrt>(val: T) -> T {
    val.sqrt()
}

pub trait Sqrt {
    fn sqrt(self) -> Self;
}

macro_rules! impl_sqrt {
    ($t_self:ty) => {
        impl Sqrt for $t_self {
            #[inline]
            fn sqrt(self) -> $t_self {
                self.sqrt()
            }
        }
    };
}

impl_sqrt!(f32);
impl_sqrt!(f64);
