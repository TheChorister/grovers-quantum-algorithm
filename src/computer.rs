use crate::{utils::UInt, vector::{ Basis, StateVector }};

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct QuBitBasis<const N: usize> {
    data: [bool; N]
}

impl<const N: usize> QuBitBasis<N> {
    pub fn get(&self, i: usize) -> bool {
        self.data[i]
    }
    
    pub fn eq_except(&self, other: &Self, index: &[usize]) -> bool {
        for i in 0..N {
            if index.contains(&i) {
                continue;
            }
            if self.get(i) != other.get(i) {
                return false;
            }
        }
        true
    }
}

impl<const N: usize> Basis for QuBitBasis<N> {
    fn iter() -> impl Iterator<Item = Self> + Clone {
        (0usize..(1 << N)).map(|v| UInt::try_from(v).unwrap()).map(|v| v.into())
    }
}

impl<const N: usize> From<UInt<N>> for QuBitBasis<N> {
    fn from(other: UInt<N>) -> Self {
        Self {
            data: other.bits
        }
    }
}

impl<const N: usize> From<QuBitBasis<N>> for UInt<N> {
    fn from(other: QuBitBasis<N>) -> Self {
        Self {
            bits: other.data
        }
    }
}

use std::fmt::Display;

impl<const N: usize> Display for QuBitBasis<N> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for bit in self.data.iter() {
            if *bit {
                write!(f, "1")?
            } else {
                write!(f, "0")?
            }
        }
        Ok(())
    }
}

pub type QuString<const N: usize> = StateVector<QuBitBasis<N>>;

impl<const N: usize> Default for QuString<N> {
    fn default() -> Self {
        StateVector::new(QuBitBasis::from(UInt::<N>::zero()))
    }
}
