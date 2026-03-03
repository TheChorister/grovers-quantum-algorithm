#[macro_use]
mod vector;
mod operator;
mod product;

use vector::Basis;
use vector::StateVector;
use vector::Component;

use std::fmt::Display;

#[derive(Eq, PartialEq, Clone, Debug)]
enum QuBit {
	On,
	Off
}

impl Basis for QuBit {
	fn iter() -> impl Iterator<Item = QuBit> + Clone {
		vec![
			Self::On,
			Self::Off
		].into_iter()
	}
}

impl Display for QuBit {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self)
    }
}

fn main() {
	let frac_1_rt_2: Component = Component::new(std::f64::consts::FRAC_1_SQRT_2, 0.);
	let state_vector: StateVector<QuBit> = state_vector!(QuBit: (frac_1_rt_2)|Off> + (frac_1_rt_2)|On>);
    println!("{}", state_vector.measure::<QuBit>());
}
