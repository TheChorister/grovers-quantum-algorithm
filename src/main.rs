#[macro_use]
mod vector;
mod operator;
mod computer;
mod utils;
mod gate;
pub mod interface;

pub use interface::*;

fn main() {
	let mut program = Program::new();
	program.add(GateType::PauliX, 0);
	program.add(GateType::Hadamard, 0);
	program.add(GateType::Hadamard, 1);
	println!("{}", program.run());
}
