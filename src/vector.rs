use num::complex::Complex;
use std::collections::HashMap;
use std::ops::{ Mul, Div, Add, Neg, Sub };
use std::fmt::Display;

pub type Component = Complex<f64>;

pub trait Basis: ConcreteBasis<Self> + std::hash::Hash + PartialEq + Eq + Clone + Display + dyn_clone::DynClone + 'static {	
	fn iter() -> impl Iterator<Item = Self> + Clone;
}

impl<T: Basis> StateVectorTrait<T> for T {
	fn get_component(&self, index: T) -> Component {
		if &index == self {
			Component::ONE
		} else {
			Component::ZERO
		}
	}
}

pub trait ConcreteBasis<T: Basis> {
	// must all be orthonormal (i.e. <i|j> = 1 if i == j else 0)
	fn iter() -> Vec<StateVector<T>>;
}

impl<T: Basis> ConcreteBasis<T> for T {
	fn iter() -> Vec<StateVector<T>> {
		<Self as Basis>::iter().map(|basis| StateVector::new(basis)).collect()
	}
}

pub trait StateVectorTrait<T: Basis>: dyn_clone::DynClone + 'static {
	// as in <i|psi>
	fn get_component(&self, i: T) -> Component;
}

#[allow(unused)]
#[derive(Clone)]
pub struct ZeroStateVector();

impl ZeroStateVector {
	#[allow(unused)]
	pub fn new() -> Self {
		Self()
	}
}

impl<B: Basis> StateVectorTrait<B> for ZeroStateVector {
	fn get_component(&self, _: B) -> Component {
		Component::ZERO
	}
}

pub struct StateVector<T: Basis> {
	inner: Box<dyn StateVectorTrait<T>>,
	cache: HashMap<T, Component>
}

impl<T: Basis> Clone for StateVector<T> {
	fn clone(&self) -> Self {
		StateVector {
			cache: self.cache.clone(),
			inner: dyn_clone::clone_box(&*self.inner)
		}
	}
}

impl<B: Basis> StateVector<B> {
	pub fn new(state_vector: impl StateVectorTrait<B>) -> Self {
		let mut this = Self {
			cache: HashMap::new(),
			inner: Box::new(state_vector)
		};
		this.build_cache();
		this
	}

	pub fn build_cache(&mut self) {
		for basis in <B as Basis>::iter() {
			self.cache.insert(basis.clone(), self.inner.get_component(basis));
		}
	}

	pub fn is_normal(&self) -> bool {
		let mut arg: f64 = 0.;
		for basis in <B as Basis>::iter() {
			arg += self.get_component(basis).norm_sqr();
		}
		// we will use a floating point range of 1e-12
		(arg - 1.) < 1e-12
	}

	pub fn probability(&self, state: StateVector<B>) -> f64 {
		(state * self).norm_sqr()
	}

	pub fn measure<T: ConcreteBasis<B>>(&self) -> StateVector<B> {
		if !self.is_normal() {
			panic!("Cannot measure a state vector that is not normalized! ({})", self);
		}
		let mut cumulative_probability: f64 = 0.;
		let random = fastrand::f64();
		for basis in T::iter() {
			cumulative_probability += (basis.clone() * self).norm_sqr();
			if cumulative_probability >= random {
				return basis;
			}
		}
		unreachable!("Attempted to measure with non-orthonormal alternative basis!")
	}

	pub fn get_component(&self, i: B) -> Component {
		match self.cache.get(&i) {
			Some(c) => *c,
			None => self.inner.get_component(i) // it would be sensible to add to the cache here but I have an immutable reference
		}
	}
}

impl<B: Basis> Display for StateVector<B> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut has_written = false;
		for basis in <B as Basis>::iter() {
			let component = self.get_component(basis.clone());
			if component != Component::ZERO {
				if has_written {
					write!(f, " + ")?
				}
				if component.re == 0. {
					write!(f, "{}i |{}⟩", component.im, basis)?;
				} else if component.im == 0. {
					write!(f, "{} |{}⟩", component.re, basis)?;
				}
				else { write!(f, "({}) |{}⟩", component, basis)?; }
				has_written = true;
			}
		}
		Ok(())
    }
}

// linear operations on state vectors

// conjugation

#[derive(Clone)]
struct StateVectorConj<T: Basis>(StateVector<T>);

impl<T: Basis> StateVectorTrait<T> for StateVectorConj<T> {
	fn get_component(&self, i: T) -> Component {
		self.0.get_component(i).conj()
	}
}

impl<T: Basis> StateVector<T> {
	pub fn conj(self) -> StateVector<T> {
		StateVector::new(StateVectorConj(self))
	}
}

// addition

#[derive(Clone)]
struct StateVectorAddition<T: Basis>(StateVector<T>, StateVector<T>);

impl<T: Basis> StateVectorTrait<T> for StateVectorAddition<T> {
	fn get_component(&self, i: T) -> Component {
		self.0.get_component(i.clone()) + self.1.get_component(i.clone())
	}
}

impl<B: Basis> Add<StateVector<B>> for StateVector<B> {
	type Output = StateVector<B>;

	fn add(self, other: Self) -> Self {
		StateVector::new(StateVectorAddition(self, other))
	}
}

impl<B: Basis> Add<StateVector<B>> for &StateVector<B> {
	type Output = StateVector<B>;

	fn add(self, other: StateVector<B>) -> StateVector<B> {
		self.clone() + other
	}
}

impl<B: Basis> Add<&StateVector<B>> for &StateVector<B> {
	type Output = StateVector<B>;

	fn add(self, other: &StateVector<B>) -> StateVector<B> {
		self.clone() + other.clone()
	}
}

impl<B: Basis> Add<&StateVector<B>> for StateVector<B> {
	type Output = StateVector<B>;

	fn add(self, other: &StateVector<B>) -> StateVector<B> {
		self + other.clone()
	}
}

// scalar multiplication

#[derive(Clone)]
struct StateVectorScalarMultiplication<T: Basis>(Component, StateVector<T>);

impl<T: Basis> StateVectorTrait<T> for StateVectorScalarMultiplication<T> {
	fn get_component(&self, i: T) -> Component {
		self.0 * self.1.get_component(i.clone())
	}
}

impl<B: Basis> Mul<Component> for StateVector<B> {
	type Output = StateVector<B>;

	fn mul(self, other: Component) -> Self {
		StateVector::new(StateVectorScalarMultiplication(other, self))
	}
}

impl<B: Basis> Mul<StateVector<B>> for Component {
	type Output = StateVector<B>;

	fn mul(self, other: StateVector<B>) -> StateVector<B> {
		other * self
	}
}

impl<B: Basis> Mul<Component> for &StateVector<B> {
	type Output = StateVector<B>;

	fn mul(self, other: Component) -> StateVector<B> {
		self.clone() * other
	}
}

impl<B: Basis> Mul<&StateVector<B>> for Component {
	type Output = StateVector<B>;

	fn mul(self, other: &StateVector<B>) -> StateVector<B> {
		self * other.clone()
	}
}

// division

impl<B: Basis> Div<Component> for StateVector<B> {
	type Output = StateVector<B>;

	fn div(self, other: Component) -> Self {
		StateVector::new(StateVectorScalarMultiplication(other.inv(), self))
	}
}

impl<B: Basis> Div<Component> for &StateVector<B> {
	type Output = StateVector<B>;

	fn div(self, other: Component) -> StateVector<B> {
		self.clone() / other
	}
}

// negation

#[derive(Clone)]
struct StateVectorNeg<T: Basis>(StateVector<T>);

impl<T: Basis> StateVectorTrait<T> for StateVectorNeg<T> {
	fn get_component(&self, i: T) -> Component {
		- self.0.get_component(i.clone())
	}
}

impl<B: Basis> Neg for StateVector<B> {
	type Output = StateVector<B>;

	fn neg(self) -> Self::Output {
		StateVector::new(StateVectorNeg(self))
	}
}

impl<B: Basis> Neg for &StateVector<B> {
	type Output = StateVector<B>;

	fn neg(self) -> Self::Output {
		-self.clone()
	}
}

// subtraction

impl<B: Basis> Sub<StateVector<B>> for StateVector<B> {
	type Output = StateVector<B>;

	fn sub(self, other: StateVector<B>) -> Self {
		StateVector::new(StateVectorAddition(self, - other))
	}
}

impl<B: Basis> Sub<StateVector<B>> for &StateVector<B> {
	type Output = StateVector<B>;

	fn sub(self, other: StateVector<B>) -> Self::Output {
		self.clone() - other
	}
}

impl<B: Basis> Sub<&StateVector<B>> for StateVector<B> {
	type Output = StateVector<B>;

	fn sub(self, other: &StateVector<B>) -> Self {
		self - other.clone()
	}
}

impl<B: Basis> Sub<&StateVector<B>> for &StateVector<B> {
	type Output = StateVector<B>;

	fn sub(self, other: &StateVector<B>) -> Self::Output {
		self.clone() - other.clone()
	}
}

// inner product (as in <A|B>)

impl<B: Basis> Mul<StateVector<B>> for StateVector<B> {
	type Output = Component;

	fn mul(self, other: StateVector<B>) -> Self::Output {
		let mut prod: Component = Component::ZERO;

		for i in <B as Basis>::iter() {
			prod += self.get_component(i.clone()).conj() * other.get_component(i)
		}

		prod
	}
}

impl<B: Basis> Mul<StateVector<B>> for &StateVector<B> {
	type Output = Component;

	fn mul(self, other: StateVector<B>) -> Self::Output {
		self.clone() * other
	}
}

impl<B: Basis> Mul<&StateVector<B>> for &StateVector<B> {
	type Output = Component;

	fn mul(self, other: &StateVector<B>) -> Self::Output {
		self.clone() * other.clone()
	}
}

impl<B: Basis> Mul<&StateVector<B>> for StateVector<B> {
	type Output = Component;

	fn mul(self, other: &StateVector<B>) -> Self::Output {
		self * other.clone()
	}
}

// macro

#[allow(unused)]
macro_rules! state_vector {
	($basis:ty: $(+ ($coefficient:expr)  |$basis_vector:ident>)+) => {
		{
			let mut state_vector: StateVector<$basis> = StateVector::new(crate::vector::ZeroStateVector::new());
			$(state_vector = state_vector + $coefficient * StateVector::new(<$basis>::$basis_vector);)*
			state_vector
		}
	};
	($basis:ty: ($initial_coefficient:expr)  |$initial_basis_vector:ident> $(+ ($coefficient:expr)  |$basis_vector:ident>)*) => {
		state_vector!($basis: + ($initial_coefficient) |$initial_basis_vector> $(+ ($coefficient)  |$basis_vector>)*)
	}
}
