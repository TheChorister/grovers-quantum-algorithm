use num::complex::Complex;
use std::ops::{ Mul, Div, Add, Neg, Sub, BitXor };

type Component = Complex<f64>;

use crate::vector::Basis;
use crate::vector::StateVector;
use crate::vector::StateVectorTrait;

pub trait LinearOperatorTrait<T: Basis>: dyn_clone::DynClone + 'static {
	// as in <i|M|j>
	fn get_component(&self, index: (T, T)) -> Component;
}

impl<T: Basis> LinearOperatorTrait<T> for (T, T) {
	fn get_component(&self, (a, b): (T, T)) -> Component {
		if a == self.0 && b == self.1 {
			Component::ONE
		} else {
			Component::ZERO
		}
	}
}

#[allow(unused)]
#[derive(Clone)]
pub struct ZeroLinearOperator();

impl<B: Basis> LinearOperatorTrait<B> for ZeroLinearOperator {
	fn get_component(&self, (a, b): (B, B)) -> Component {
		if a == b {
			Component::ONE
		} else {
			Component::ZERO
		}
	}
}

pub struct LinearOperator<T: Basis> {
	pub(in super) inner: Box<dyn LinearOperatorTrait<T>>
}

impl<T: Basis> LinearOperator<T> {
	pub fn new(operator: impl LinearOperatorTrait<T>) -> Self {
		Self {
			inner: Box::new(operator)
		}
	}

	pub fn get_component(&self, i: (T, T)) -> Component {
		self.inner.get_component(i)
	}
}

impl<T: Basis> Clone for LinearOperator<T> {
	fn clone(&self) -> Self {
		LinearOperator {
			inner: dyn_clone::clone_box(&*self.inner)
		}
	}
}

// linear operations on linear operators

// addition

#[derive(Clone)]
struct LinearOperatorAddition<T: Basis>(LinearOperator<T>, LinearOperator<T>);

impl<T: Basis> LinearOperatorTrait<T> for LinearOperatorAddition<T> {
	fn get_component(&self, index: (T, T)) -> Component {
		self.0.get_component(index.clone()) + self.1.get_component(index.clone())
	}
}

impl<B: Basis> Add<LinearOperator<B>> for LinearOperator<B> {
	type Output = LinearOperator<B>;

	fn add(self, other: LinearOperator<B>) -> Self::Output {
		LinearOperator {
			inner: Box::new(
				LinearOperatorAddition(self, other)
			)
		}
	}
}

impl<B: Basis> Add<LinearOperator<B>> for &LinearOperator<B> {
	type Output = LinearOperator<B>;

	fn add(self, other: LinearOperator<B>) -> Self::Output {
		self.clone() + other
	}
}

impl<B: Basis> Add<&LinearOperator<B>> for &LinearOperator<B> {
	type Output = LinearOperator<B>;

	fn add(self, other: &LinearOperator<B>) -> Self::Output {
		self.clone() + other.clone()
	}
}

impl<B: Basis> Add<&LinearOperator<B>> for LinearOperator<B> {
	type Output = LinearOperator<B>;

	fn add(self, other: &LinearOperator<B>) -> Self::Output {
		self + other.clone()
	}
}

// scalar multiplication

#[derive(Clone)]
struct LinearOperatorScalarMultiplication<T: Basis>(Component, LinearOperator<T>);

impl<T: Basis> LinearOperatorTrait<T> for LinearOperatorScalarMultiplication<T> {
	fn get_component(&self, i: (T, T)) -> Component {
		self.0 * self.1.get_component(i.clone())
	}
}

impl<B: Basis> Mul<Component> for LinearOperator<B> {
	type Output = LinearOperator<B>;

	fn mul(self, other: Component) -> Self {
		LinearOperator {
			inner: Box::new(
				LinearOperatorScalarMultiplication(other, self)
			)
		}
	}
}

impl<B: Basis> Mul<Component> for &LinearOperator<B> {
	type Output = LinearOperator<B>;

	fn mul(self, other: Component) -> Self::Output {
		self.clone() * other
	}
}

impl<B: Basis> Mul<LinearOperator<B>> for Component {
	type Output = LinearOperator<B>;

	fn mul(self, other: LinearOperator<B>) -> LinearOperator<B> {
		LinearOperator {
			inner: Box::new(
				LinearOperatorScalarMultiplication(self, other)
			)
		}
	}
}


impl<B: Basis> Mul<&LinearOperator<B>> for Component {
	type Output = LinearOperator<B>;

	fn mul(self, other: &LinearOperator<B>) -> Self::Output {
		self * other.clone()
	}
}

// division

impl<B: Basis> Div<Component> for LinearOperator<B> {
	type Output = LinearOperator<B>;

	fn div(self, other: Component) -> Self::Output {
		LinearOperator {
			inner: Box::new(
				LinearOperatorScalarMultiplication(other.inv(), self)
			)
		}
	}
}

impl<B: Basis> Div<Component> for &LinearOperator<B> {
	type Output = LinearOperator<B>;

	fn div(self, other: Component) -> Self::Output {
		self.clone() / other
	}
}

// negation

impl<B: Basis> Neg for LinearOperator<B> {
	type Output = LinearOperator<B>;

	fn neg(self) -> Self::Output {
		self * -Component::ONE
	}
}

impl<B: Basis> Neg for &LinearOperator<B> {
	type Output = LinearOperator<B>;

	fn neg(self) -> Self::Output {
		- self.clone()
	}
}

// subtraction

impl<B: Basis> Sub<LinearOperator<B>> for LinearOperator<B> {
	type Output = LinearOperator<B>;

	fn sub(self, other: LinearOperator<B>) -> Self::Output {
		LinearOperator {
			inner: Box::new(
				LinearOperatorAddition(self, - other)
			)
		}
	}
}

impl<B: Basis> Sub<LinearOperator<B>> for &LinearOperator<B> {
	type Output = LinearOperator<B>;

	fn sub(self, other: LinearOperator<B>) -> Self::Output {
		self.clone() - other
	}
}

impl<B: Basis> Sub<&LinearOperator<B>> for &LinearOperator<B> {
	type Output = LinearOperator<B>;

	fn sub(self, other: &LinearOperator<B>) -> Self::Output {
		self.clone() - other.clone()
	}
}

impl<B: Basis> Sub<&LinearOperator<B>> for LinearOperator<B> {
	type Output = LinearOperator<B>;

	fn sub(self, other: &LinearOperator<B>) -> Self::Output {
		self - other.clone()
	}
}

// product

/*
AB|psi> = A (B|psi>)
B|psi>  = \sum psi_i B|i>
		= \sum <i|psi> B|i>
AB|psi> = \sum <i|psi> AB|i>
		= \sum <i|psi> \sum <j|B|psi> A|j>
=> <n|AB|m> = <n| \sum <i|m> \sum <j|B|m> A|j>
			= \sum <j|B|m> <n|A|j>
			= \sum <n|A|j> <j|B|m>
*/

#[derive(Clone)]
struct LinearOperatorProduct<T: Basis>(LinearOperator<T>, LinearOperator<T>);

impl<B: Basis> LinearOperatorTrait<B> for LinearOperatorProduct<B> {	
	fn get_component(&self, (n, m): (B, B)) -> Component {
		let mut sum: Component = Component::ZERO;

		for j in <B as Basis>::iter() {
			sum += self.0.get_component((n.clone(), j.clone())) * self.1.get_component((j, m.clone()));
		}

		sum
	}
}

impl<B: Basis> Mul<LinearOperator<B>> for LinearOperator<B> {
	type Output = LinearOperator<B>;

	fn mul(self, other: LinearOperator<B>) -> Self::Output {
		LinearOperator {
			inner: Box::new(
				LinearOperatorProduct(self, other) 
			)
		}
	}
}

impl<B: Basis> Mul<LinearOperator<B>> for &LinearOperator<B> {
	type Output = LinearOperator<B>;

	fn mul(self, other: LinearOperator<B>) -> Self::Output {
		self.clone() * other
	}
}

impl<B: Basis> Mul<&LinearOperator<B>> for &LinearOperator<B> {
	type Output = LinearOperator<B>;

	fn mul(self, other: &LinearOperator<B>) -> Self::Output {
		self.clone() * other.clone()
	}
}

impl<B: Basis> Mul<&LinearOperator<B>> for LinearOperator<B> {
	type Output = LinearOperator<B>;

	fn mul(self, other: &LinearOperator<B>) -> Self::Output {
		self * other.clone()
	}
}

// state vector product (as in A|psi>)
/*
A|psi> = \sum psi_i A|i>
=> <i| A|psi> = \sum psi_j <i|A|j>
*/

#[derive(Clone)]
struct LinearOperatorStateVectorProduct<T: Basis>(LinearOperator<T>, StateVector<T>);

impl<B: Basis> StateVectorTrait<B> for LinearOperatorStateVectorProduct<B> {	
	fn get_component(&self, i: B) -> Component {
		let mut sum: Component = Component::ZERO;

		for j in <B as Basis>::iter() {
			sum += self.0.get_component((i.clone(), j.clone())) * self.1.get_component(j);
		}

		sum
	}
}


impl<B: Basis> Mul<StateVector<B>> for LinearOperator<B> {
	type Output = StateVector<B>;

	fn mul(self, other: StateVector<B>) -> Self::Output {
		StateVector::new(LinearOperatorStateVectorProduct(self, other))
	}
}

impl<B: Basis> Mul<StateVector<B>> for &LinearOperator<B> {
	type Output = StateVector<B>;

	fn mul(self, other: StateVector<B>) -> Self::Output {
		self.clone() * other
	}
}

impl<B: Basis> Mul<&StateVector<B>> for &LinearOperator<B> {
	type Output = StateVector<B>;

	fn mul(self, other: &StateVector<B>) -> Self::Output {
		self.clone() * other.clone()
	}
}

impl<B: Basis> Mul<&StateVector<B>> for LinearOperator<B> {
	type Output = StateVector<B>;

	fn mul(self, other: &StateVector<B>) -> Self::Output {
		self * other.clone()
	}
}

impl<B: Basis> Mul<LinearOperator<B>> for StateVector<B> {
	type Output = StateVector<B>;

	fn mul(self, other: LinearOperator<B>) -> Self::Output {
		(other * self).conj()
	}
}

impl<B: Basis> Mul<LinearOperator<B>> for &StateVector<B> {
	type Output = StateVector<B>;

	fn mul(self, other: LinearOperator<B>) -> Self::Output {
		(other * self).conj()
	}
}

impl<B: Basis> Mul<&LinearOperator<B>> for StateVector<B> {
	type Output = StateVector<B>;

	fn mul(self, other: &LinearOperator<B>) -> Self::Output {
		(other * self).conj()
	}
}

impl<B: Basis> Mul<&LinearOperator<B>> for &StateVector<B> {
	type Output = StateVector<B>;

	fn mul(self, other: &LinearOperator<B>) -> Self::Output {
		(other * self).conj()
	}
}

// outer product (as in |A><B|)

/*
<n|A><B|m> = (<n|A>)(<B|m>)
*/

#[derive(Clone)]
struct LinearOperatorStateVectorOuterProduct<T: Basis>(StateVector<T>, StateVector<T>);

impl<B: Basis> LinearOperatorTrait<B> for LinearOperatorStateVectorOuterProduct<B> {
	fn get_component(&self, (n, m): (B, B)) -> Component {
		self.0.get_component(n) * self.1.get_component(m).conj()
	}
}

impl<B: Basis> BitXor<StateVector<B>> for StateVector<B> {
	type Output = LinearOperator<B>;

	fn bitxor(self, other: StateVector<B>) -> Self::Output {
		LinearOperator {
			inner: Box::new(
				LinearOperatorStateVectorOuterProduct(self, other)
			)
		}
	}
}

impl<B: Basis> BitXor<StateVector<B>> for &StateVector<B> {
	type Output = LinearOperator<B>;

	fn bitxor(self, other: StateVector<B>) -> Self::Output {
		self.clone() ^ other
	}
}

impl<B: Basis> BitXor<&StateVector<B>> for StateVector<B> {
	type Output = LinearOperator<B>;

	fn bitxor(self, other: &StateVector<B>) -> Self::Output {
		self ^ other.clone()
	}
}

impl<B: Basis> BitXor<&StateVector<B>> for &StateVector<B> {
	type Output = LinearOperator<B>;

	fn bitxor(self, other: &StateVector<B>) -> Self::Output {
		self.clone() ^ other.clone()
	}
}
