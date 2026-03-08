#[macro_use]
mod vector;
mod operator;
mod product;
mod computer;
mod utils;

use vector::Basis;
use vector::StateVector;
use vector::Component;

use product::BasisTensorProduct;
use computer::*;

use std::fmt::Display;

use paste::paste;

fn main() {
	let frac_1_rt_2: Component = Component::new(std::f64::consts::FRAC_1_SQRT_2, 0.);
	let string: QuString<3> = QuString::default();
	let string2: QuString<3> = QuString::default();
	println!("{}", string.measure::<QuBitBasis<3>>())
}

/*qstring!(pub QuString(QBasis): QuBit1, QuBit2);

fn main() {
	let frac_1_rt_2: Component = Component::new(std::f64::consts::FRAC_1_SQRT_2, 0.);
	let state_vector1: StateVector<QuBit1> = state_vector!(QuBit1: (frac_1_rt_2)|Off> + (frac_1_rt_2)|On>);
	let state_vector2: StateVector<QuBit2> = state_vector!(QuBit2: (Component::ONE) |On>);
    println!("{}", (state_vector1 | state_vector2).measure::<BasisTensorProduct<QuBit1, QuBit2>>());
}
*/