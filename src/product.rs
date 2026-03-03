use super::vector::StateVector;
use super::vector::StateVectorTrait;
use super::vector::Basis;
use super::vector::Component;

use super::operator::LinearOperator;
use super::operator::LinearOperatorTrait;

use itertools::iproduct;

// MACROS

pub trait Commute {
	type Commuted: Basis;
	fn commute(self) -> Self::Commuted;
}

pub trait AssociateLeft: Basis {
    type Associated: Basis;
    fn associate_left(self) -> Self::Associated;
}

pub trait AssociateRight: Basis {
    type Associated: Basis;
    fn associate_right(self) -> Self::Associated;
}

macro_rules! binary_commute {
	($class:ident) => {
		impl<U: Basis, V: Basis> Commute for $class<U, V> {
			type Commuted = $class<V, U>;
			fn commute(self) -> Self::Commuted {
				$class(self.1, self.0)
			}
		}

		impl<T: Basis, U: Basis, V: Basis> AssociateLeft for $class<T, BasisTensorProduct<U, V>> {
			type Associated = $class<BasisProduct<T, U>, V>;
			fn associate_left(self) -> Self::Associated {
				$class($class(self.0, self.1.0), self.1.1)
			}
		}
	}
}

macro_rules! commute {
	($class:ident) => {
		binary_commute!($class);
		//ternary_commute!($class);
	}
}

// BASIS TENSOR PRODUCT

#[derive(Clone, PartialEq, Eq)]
pub struct BasisTensorProduct<U: Basis, V: Basis>(U, V);

impl<U: Basis, V: Basis> Basis for BasisTensorProduct<U, V> {
	fn iter() -> impl Iterator<Item = Self> + Clone {
		iproduct!(<U as Basis>::iter(), <V as Basis>::iter()).map(|(u, v)| Self(u, v))
	}
}

impl<U: Basis, V: Basis> std::fmt::Display for BasisTensorProduct<U, V> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {})", self.0, self.1)
	}
}

commute!(BasisTensorProduct);

// VECTOR TENSOR PRODUCT

#[derive(Clone)]
pub struct VectorTensorProduct<U: Basis, V: Basis> (StateVector<U>, StateVector<V>);

impl<U: Basis, V: Basis> StateVectorTrait<BasisTensorProduct<U, V>> for VectorTensorProduct<U, V> {
	fn get_component(&self, index: BasisTensorProduct<U, V>) -> Component {
		self.0.inner.get_component(index.0) * self.1.inner.get_component(index.1)
	}
}

commute!(VectorTensorProduct);

// LINEAR OPERATOR TENSOR PRODUCT

#[derive(Clone)]
pub struct LinearOperatorTensorProduct<U: Basis, V: Basis> (LinearOperator<U>, LinearOperator<V>);

impl<U: Basis, V: Basis> LinearOperatorTrait<BasisTensorProduct<U, V>> for LinearOperatorTensorProduct<U, V> {
	fn get_component(&self, (a, b): (BasisTensorProduct<U, V>, BasisTensorProduct<U, V>)) -> Component {
		self.0.inner.get_component((a.0, b.0)) * self.1.inner.get_component((a.1, b.1))
	}
}

commute!(LinearOperatorTensorProduct);
