// obviously inefficient but for our purposes ok
#[derive(Clone)]
pub struct UInt<const N: usize> {
    pub(super) bits: [bool; N]
}

impl<const N: usize> UInt<N> {
    pub fn zero() -> Self {
        Self {
            bits: [false; N]
        }
    }
}

macro_rules! impl_try_from {
    ($type_:ty) => {
        impl<const N: usize> TryFrom<$type_> for UInt<N> {
            type Error = ();

            fn try_from(value: $type_) -> Result<Self, Self::Error> {
                if value >= (1 << N) {
                    Err(())
                } else {
                    let mut bits: [bool; N] = [false; N];
                    for i in 0..N {
                        bits[i] = (value & (1 << i)) > 0;
                    }
                    Ok(Self {
                        bits
                    })
                }
            }
        }

        impl<const N: usize> From<UInt<N>> for $type_ {
            fn from(repr: UInt<N>) -> $type_ {
                let mut value: $type_ = 0;
                for i in 0..N {
                    value += (repr.bits[i] as $type_) << i;
                }
                value
            }
        }
    };
    ($($type_:ty),*) => {
        $(impl_try_from!($type_);)*
    }
}

impl_try_from!(u8, u16, u32, u64, u128);
