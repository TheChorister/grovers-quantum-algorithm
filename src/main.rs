#[macro_use]
mod vector;
mod operator;
mod product;
mod computer;
mod utils;
mod gate;
pub mod interface;

pub use interface::*;

use computer::*;

use crate::gate::*;

fn main() {
	let mut string: QuString<2> = Default::default();
	string = PauliXGate::new(0).unwrap() * string;
	string = SwapGate::new(0, 1).unwrap() * string;
	println!("{}", string.measure::<QuBitBasis<2>>())
}
