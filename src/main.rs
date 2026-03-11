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
	program.add(Gate::new(GateType::Hadamard, vec![2]));
	program.add(Gate::new(GateType::Hadamard, vec![3]));
	program.add(Gate::new(GateType::Hadamard, vec![4]));
	program.add(Gate::new(GateType::Hadamard, vec![5]));
	program.add(Gate::new(GateType::Hadamard, vec![6]));
	program.add(Gate::new(GateType::Hadamard, vec![7]));
	program.add(Gate::new(GateType::GroverOracle, vec![0, 1]));
	program.add(Gate::new(GateType::GroverDiffuser, vec![0, 1, 2, 3, 4, 5, 6, 7]));
	program.add(Gate::new(GateType::GroverOracle, vec![0, 3]));
	program.add(Gate::new(GateType::GroverDiffuser, vec![0, 1, 2, 3, 4, 5, 6, 7]));
	program.add(Gate::new(GateType::GroverOracle, vec![0, 3]));
	program.add(Gate::new(GateType::GroverDiffuser, vec![0, 1, 2, 3, 4, 5, 6, 7]));
	program.add(Gate::new(GateType::GroverOracle, vec![0, 3]));
	program.add(Gate::new(GateType::GroverDiffuser, vec![0, 1, 2, 3, 4, 5, 6, 7]));
	program.add(Gate::new(GateType::GroverOracle, vec![0, 3]));
	program.add(Gate::new(GateType::GroverDiffuser, vec![0, 1, 2, 3, 4, 5, 6, 7]));
	program.add(Gate::new(GateType::GroverOracle, vec![0, 3]));
	program.add(Gate::new(GateType::GroverDiffuser, vec![0, 1, 2, 3, 4, 5, 6, 7]));
	program.add(Gate::new(GateType::GroverOracle, vec![0, 3]));
	program.add(Gate::new(GateType::GroverDiffuser, vec![0, 1, 2, 3, 4, 5, 6, 7]));
	program.add(Gate::new(GateType::GroverOracle, vec![0, 3]));
	program.add(Gate::new(GateType::GroverDiffuser, vec![0, 1, 2, 3, 4, 5, 6, 7]));
	program.add(Gate::new(GateType::GroverOracle, vec![0, 3]));
	program.add(Gate::new(GateType::GroverDiffuser, vec![0, 1, 2, 3, 4, 5, 6, 7]));
	program.add(Gate::new(GateType::GroverOracle, vec![0, 3]));
	program.add(Gate::new(GateType::GroverDiffuser, vec![0, 1, 2, 3, 4, 5, 6, 7]));
	program.add(Gate::new(GateType::GroverOracle, vec![0, 3]));
	program.add(Gate::new(GateType::GroverDiffuser, vec![0, 1, 2, 3, 4, 5, 6, 7]));
	println!("{}", program.run());
}
