use crate::computer::QuString;
use crate::utils::UInt;
use crate::vector::{Basis, StateVector};
use crate::{computer::QuBitBasis, operator::LinearOperator};
use crate::gate::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub enum GateType {
    Identity,
    PauliX,
    PauliY,
    PauliZ,
    Hadamard,
    Phase,
    T,
    CNOT,
    CZ,
    Swap
}

impl GateType {
    pub fn into<const N: usize>(self, bit: usize) -> Option<LinearOperator<QuBitBasis<N>>> {
        match self {
            GateType::Identity => Some(IdentityGate::new()),
            GateType::PauliX => PauliXGate::new(bit),
            GateType::PauliY => PauliYGate::new(bit),
            GateType::PauliZ => PauliZGate::new(bit),
            GateType::Hadamard => HadamardGate::new(bit),
            GateType::Phase => PhaseGate::new(bit),
            GateType::T => TGate::new(bit),
            GateType::CNOT => CNOTGate::new(bit, bit + 1),
            GateType::CZ => CZGate::new(bit, bit + 1),
            GateType::Swap => SwapGate::new(bit, bit + 1),
        }
    }
}

// BITS <= 32 since then the js can use a normal integer type as opposed to bigint for anything larger
pub const BITS: usize = 6;

#[wasm_bindgen]
pub struct Program {
    gates: Vec<(GateType, usize)>
}

#[wasm_bindgen]
impl Program {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            gates: Vec::new()
        }
    }

    #[wasm_bindgen]
    pub fn get_max_bits() -> usize {
        BITS
    }

    #[wasm_bindgen]
    pub fn add(&mut self, type_: GateType, bit: usize) {
        self.gates.push((type_, bit));
    }

    #[wasm_bindgen]
    pub fn clear(&mut self) {
        self.gates.clear();
    }

    fn get_state_vector(&self) -> StateVector<QuBitBasis<BITS>> {
	    let mut string: QuString<BITS> = Default::default();
        for gate in self.gates.iter() {
            if let Some(gate) = gate.0.clone().into(gate.1) {
                string = gate * string;
            }
        }
        string
    }
    
    #[wasm_bindgen]
    pub fn get_probability(&self, res: u32) -> f64 {
        let measurable = self.get_state_vector();
        let basis: QuBitBasis<BITS> = match UInt::<BITS>::try_from(res) {
            Ok(v) => v,
            Err(_) => return 0.
        }.into();
        measurable.probability(StateVector::new(basis))
    }

    #[wasm_bindgen]
    pub fn run(&self) -> u32 {
        let result = self.get_state_vector().measure::<QuBitBasis<BITS>>();
        for basis in <QuBitBasis::<BITS> as Basis>::iter() {
            if result.probability(StateVector::new(basis.clone())) > 0. { // should be exactly one but to be safe
                return UInt::<BITS>::from(basis).into();
            }
        }
        unreachable!("No results to measurement vector!");
    }
}
