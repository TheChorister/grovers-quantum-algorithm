use std::ops::{ Deref, DerefMut };
use crate::{utils::UInt, vector::{ Basis, StateVector, ZeroStateVector }};

#[derive(Clone, Eq, PartialEq)]
pub struct QuBitBasis<const N: usize> {
    data: [bool; N]
}

impl<const N: usize> Basis for QuBitBasis<N> {
    fn iter() -> impl Iterator<Item = Self> + Clone {
        (0u128..(1 << N)).map(|v| UInt::try_from(v).unwrap()).map(|v| v.into())
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

pub struct QuString<const N: usize> {
    state_vector: StateVector<QuBitBasis<N>>
}

impl<const N: usize> QuString<N> {
    pub fn new() -> Self {
        Self {
            state_vector: StateVector::new(ZeroStateVector::new())
        }
    }
}

impl<const N: usize> Default for QuString<N> {
    fn default() -> Self {
        Self {
            state_vector: StateVector::new(QuBitBasis::from(UInt::<N>::try_from(0u8).unwrap()))
        }
    }
}

impl<const N: usize> Deref for QuString<N> {
    type Target = StateVector<QuBitBasis<N>>;
    fn deref(&self) -> &Self::Target {
        &self.state_vector
    }
}

impl<const N: usize> DerefMut for QuString<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state_vector
    }
}
