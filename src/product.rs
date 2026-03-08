#![allow(unused)]

use super::vector::StateVector;
use super::vector::StateVectorTrait;
use super::vector::Basis;
use super::vector::Component;

use super::operator::LinearOperator;
use super::operator::LinearOperatorTrait;

use std::ops::BitOr;
use itertools::iproduct;

pub trait TensorProduct {
	type LHS;
	type RHS;
	type Output;
	fn prod(a: Self::LHS, b: Self::RHS) -> Self::Output;
}

pub trait Commute {
	type Commute;

	fn commute(self) -> Self::Commute;
}

pub trait AssociateLeft {
	type Output;

	fn associate_left(self) -> Self::Output;
}

pub trait AssociateRight {
	type Output;

	fn associate_left(self) -> Self::Output;
}

// BASIS

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

impl<U: Basis, V: Basis> TensorProduct for BasisTensorProduct<U, V> {
	type LHS = U;
	type RHS = V;
	type Output = Self;

	fn prod(lhs: U, rhs: V) -> Self {
		Self(lhs, rhs)
	}
}

// VECTOR

#[derive(Clone)]
pub struct VectorTensorProduct<U: Basis, V: Basis> (StateVector<U>, StateVector<V>);

impl<U: Basis, V: Basis> StateVectorTrait<BasisTensorProduct<U, V>> for VectorTensorProduct<U, V> {
	fn get_component(&self, index: BasisTensorProduct<U, V>) -> Component {
		self.0.inner.get_component(index.0) * self.1.inner.get_component(index.1)
	}
}

impl<U: Basis, V: Basis> TensorProduct for VectorTensorProduct<U, V> {
	type LHS = StateVector<U>;
	type RHS = StateVector<V>;
	type Output = StateVector<BasisTensorProduct<U, V>>;

	fn prod(lhs: StateVector<U>, rhs: StateVector<V>) -> Self::Output {
		StateVector::new(Self(lhs, rhs))
	}
}

impl<U: Basis, V: Basis> BitOr<StateVector<V>> for StateVector<U> {
	type Output = StateVector<BasisTensorProduct<U, V>>;

	fn bitor(self, other: StateVector<V>) -> Self::Output {
		VectorTensorProduct::prod(self, other)
	}
}

// LINEAR OPERATOR

#[derive(Clone)]
pub struct LinearOperatorTensorProduct<U: Basis, V: Basis> (LinearOperator<U>, LinearOperator<V>);

impl<U: Basis, V: Basis> LinearOperatorTrait<BasisTensorProduct<U, V>> for LinearOperatorTensorProduct<U, V> {
	fn get_component(&self, (a, b): (BasisTensorProduct<U, V>, BasisTensorProduct<U, V>)) -> Component {
		self.0.inner.get_component((a.0, b.0)) * self.1.inner.get_component((a.1, b.1))
	}
}

impl<U: Basis, V: Basis> TensorProduct for LinearOperatorTensorProduct<U, V> {
	type LHS = LinearOperator<U>;
	type RHS = LinearOperator<V>;
	type Output = LinearOperator<BasisTensorProduct<U, V>>;

	fn prod(lhs: Self::LHS, rhs: Self::RHS) -> Self::Output {
		LinearOperator::new(Self(lhs, rhs))
	}
}

impl<U: Basis, V: Basis> BitOr<LinearOperator<V>> for LinearOperator<U> {
	type Output = LinearOperator<BasisTensorProduct<U, V>>;

	fn bitor(self, other: LinearOperator<V>) -> Self::Output {
		LinearOperatorTensorProduct::prod(self, other)
	}
}
