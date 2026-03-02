use num::complex::Complex;
use std::ops::{ Mul, Add };

type Component = Complex<f64>;

pub trait StateVector: Clone + Sized {
	type BasisVector: Clone + Copy;
	
	fn iter_basis_vectors() -> impl Iterator<Item = Self::BasisVector>;

	// in the form <i | psi>
	fn get_component(&self, basis: Self::BasisVector) -> Component;

	// in the form <self | right>
	fn inner_product(&self, right: &Self) -> Component {
		let mut product: Component = Component::new(0., 0.);
		for basis_vector in Self::iter_basis_vectors() {
			product += right.get_component(basis_vector) * self.get_component(basis_vector).conj()
		}
		product
	}

	// in the form | self><right |
	fn outer_product(&self, right: &Self) -> StateOuterProduct<Self> {
		StateOuterProduct::from((self.clone(), right.clone()))
	}
}

#[derive(Clone)]
pub struct StateVectorSum<T: StateVector> {
	left: T,
	right: T
}

impl<T: StateVector> StateVector for StateVectorSum<T> {
	type BasisVector = T::BasisVector;

	fn iter_basis_vectors() -> impl Iterator<Item = Self::BasisVector> {
		T::iter_basis_vectors()
	}

	fn get_component(&self, basis: Self::BasisVector) -> Component {
		self.left.get_component(basis) + self.right.get_component(basis)
	}
}

impl<T: StateVector> From<(T, T)> for StateVectorSum<T> {
	fn from(pair: (T, T)) -> Self {
		Self {
			left: pair.0,
			right: pair.1
		}
	}
}

#[derive(Clone)]
pub struct RightProduct<A: LinearOperator<B>, B: StateVector> {
	operator: A,
	vector: B
}

impl<Operator: LinearOperator<Space>, Space: StateVector> StateVector for RightProduct<Operator, Space> {
	type BasisVector = Space::BasisVector;

	fn iter_basis_vectors() -> impl Iterator<Item = Self::BasisVector> {
		Space::iter_basis_vectors()
	}

	// <i| M |psi>
	// = <i| \sum M psi_j |j>
	// = <i| \sum <j|psi> M|j>
	// = \sum <j|psi> <i|M|j> 
	fn get_component(&self, i: Self::BasisVector) -> Component {
		let mut value: Component = Component::new(0., 0.);
		for j in Self::iter_basis_vectors() {
			value += self.vector.get_component(j) * self.operator.get_component((i, j))
		}
		value
	}
}

#[derive(Clone)]
pub struct ConjStateVector<T: StateVector> {
	inner: T
}

impl<T: StateVector> StateVector for ConjStateVector<T> {
	type BasisVector = T::BasisVector;

	fn iter_basis_vectors() -> impl Iterator<Item = Self::BasisVector> {
		T::iter_basis_vectors()
	}

	fn get_component(&self, i: Self::BasisVector) -> Component {
		self.inner.get_component()
	}
}

pub trait LinearOperator<Space: StateVector>: Clone {

	// in the form <i | M | j>
	fn get_component(&self, basis_vectors: (Space::BasisVector, Space::BasisVector)) -> Component;

	fn right_product(&self, right: Space) -> RightProduct<Self, Space> {
		RightProduct {
			operator: self.clone(),
			vector: right
		}
	}
}

#[derive(Clone)]
pub struct StateOuterProduct<T: StateVector> {
	left: T,
	right: T
}

impl<T: StateVector> LinearOperator<T> for StateOuterProduct<T> {
	// in the form <i | left><right | j>
	fn get_component(&self, basis_vectors: (T::BasisVector, T::BasisVector)) -> Component {
		self.left.get_component(basis_vectors.0) * self.right.get_component(basis_vectors.1).conj()
	}
}

impl<T: StateVector> From<(T, T)> for StateOuterProduct<T> {
	fn from(pair: (T, T)) -> Self {
		Self {
			left: pair.0,
			right: pair.1
		}
	}
}
