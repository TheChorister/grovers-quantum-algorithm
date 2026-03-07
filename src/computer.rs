pub use paste::paste;
pub use itertools::iproduct;

macro_rules! qubit {
    ($v:vis $name:ident) => {
        #[derive(Eq, PartialEq, Clone)]
        $v enum $name {
            On,
            Off
        }

        impl Basis for $name {
            fn iter() -> impl Iterator<Item = $name> + Clone {
                vec![
                    Self::On,
                    Self::Off
                ].into_iter()
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match &self {
                    Self::On => write!(f, "1"),
                    Self::Off => write!(f, "0")
                }
            }
        }
    }
}

macro_rules! qstring {
    ($v:vis $name:ident($basis:ident): $($bit:ident),*) => {
        $(
            qubit!($v $bit);
        )*

        paste! {
            #[derive(Clone, PartialEq, Eq)]
            $v struct $basis {
                $([< bit_ $bit:snake >]: $bit),*
            }

            impl crate::vector::Basis for $basis {
                fn iter() -> impl Iterator<Item = $basis> + Clone {
                    iproduct!(
                        $(<$bit as Basis>::iter()),*
                    ).map(|($([< bit_ $bit:snake >]),*)| Self { $([< bit_ $bit:snake >]),* })
                }
            }

            impl Display for $basis {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    $(
                        write!(f, "{}", self.[< bit_ $bit:snake >])?
                    );*;
                    Ok(())
                }
            }

            #[derive(Clone)]
            struct [< Private $name >] {
                $([< bit_ $bit:snake >]: StateVector<$bit>),*
            }

            impl crate::vector::StateVectorTrait<$basis> for [< Private $name >] {
                fn get_component(&self, basis: $basis) -> Component {
                    $(
                        self.[< bit_ $bit:snake >].inner.get_component(basis.[< bit_ $bit:snake >]) *
                    )* Component::ONE
                }
            }
        }
    }
}
