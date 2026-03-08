#[macro_use]
mod vector;
mod operator;
mod product;
pub mod computer;
mod utils;
pub mod gate;

use computer::*;

use crate::gate::*;

fn main() {
	let mut string: QuString<2> = Default::default();
	string = PauliXGate::new(0).unwrap() * string;
	string = SwapGate::new(0, 1).unwrap() * string;
	println!("{}", string.measure::<QuBitBasis<2>>())
}
