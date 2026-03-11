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
	program.add(Gate::new(GateType::Hadamard, vec![0]));
	program.add(Gate::new(GateType::Hadamard, vec![1]));
	program.add(Gate::new(GateType::GroverOracle, vec![0]));
	program.add(Gate::new(GateType::GroverDiffuser, vec![0, 1]));
	println!("{}", program.run());
}
