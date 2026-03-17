use crate::computer::QuString;
use crate::utils::UInt;
use crate::vector::{Basis, Component, StateVector};
use crate::{computer::QuBitBasis, operator::LinearOperator};
use crate::gate::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
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
    Swap,
    GroverOracle,
    GroverDiffuser
}

impl GateType {
    pub fn into<const N: usize>(self, bits: Vec<usize>) -> Option<LinearOperator<QuBitBasis<N>>> {
        match self {
            GateType::Identity => Some(IdentityGate::new()),
            GateType::PauliX => PauliXGate::new(*bits.first()?),
            GateType::PauliY => PauliYGate::new(*bits.first()?),
            GateType::PauliZ => PauliZGate::new(*bits.first()?),
            GateType::Hadamard => HadamardGate::new(*bits.first()?),
            GateType::Phase => PhaseGate::new(*bits.first()?),
            GateType::T => TGate::new(*bits.first()?),
            GateType::CNOT => CNOTGate::new(*bits.first()?, *bits.get(1)?),
            GateType::CZ => CZGate::new(*bits.first()?, *bits.get(1)?),
            GateType::Swap => SwapGate::new(*bits.first()?, *bits.get(1)?),
            GateType::GroverOracle => {
                let mut target_bin: usize = 0;
                for bit in bits.iter() {
                    target_bin += 1 << *bit;
                }
                Some(GroverOracle::new(QuBitBasis::<N>::from(UInt::<N>::try_from(target_bin).ok()?)))
            },
            GateType::GroverDiffuser => GroverDiffuser::new(bits)
        }
    }
}

// BITS <= 32 since then the js can use a normal integer type as opposed to bigint for anything larger
pub const BITS: usize = 8;

#[wasm_bindgen]
pub struct Complex {
    pub re: f64,
    pub im: f64
}

impl From<Component> for Complex {
    fn from(value: Component) -> Self {
        Self {
            re: value.re,
            im: value.im
        }
    }
}

#[wasm_bindgen]
pub struct Program {
    gates: Vec<Gate>,
    cache_state_vector: QuString<BITS>,
    last_cache_index: isize
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Gate {
    pub gate_type: GateType,
    bits: Vec<usize>
}

#[wasm_bindgen]
impl Gate {
    #[wasm_bindgen(constructor)]
    pub fn new(gate: GateType, bits: Vec<usize>) -> Self {
        Gate {
            gate_type: gate,
            bits
        }
    }

    #[wasm_bindgen(setter = bits)]
    pub fn set_bits(&mut self, bits: Vec<usize>) {
        self.bits = bits;
    }

    #[wasm_bindgen(getter = bits)]
    pub fn get_bits(&self) -> Vec<usize> {
        self.bits.clone()
    }
}

#[wasm_bindgen]
impl Program {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            gates: Vec::new(),
            cache_state_vector: Default::default(),
            last_cache_index: -1
        }
    }

    #[wasm_bindgen]
    pub fn get_max_bits() -> usize {
        BITS
    }

    #[wasm_bindgen]
    pub fn add(&mut self, gate: Gate) {
        self.gates.push(gate);
    }

    #[wasm_bindgen]
    pub fn clear(&mut self) {
        self.gates.clear();
        self.last_cache_index = -1;
        self.cache_state_vector = Default::default();
    }

    fn get_state_vector(&mut self) -> StateVector<QuBitBasis<BITS>> {
	    let mut string: QuString<BITS> = self.cache_state_vector.clone();
        for i in ((self.last_cache_index + 1) as usize)..self.gates.len() {
            self.last_cache_index = i as isize;
            let gate = self.gates.get(i as usize).unwrap();
            if let Some(gate_) = gate.gate_type.clone().into(gate.bits.clone()) {
                string = gate_ * string;
            } else {
                println!("malformed gate! {:?}", gate);
            }
        }
        self.cache_state_vector = string.clone();
        string
    }
    
    #[wasm_bindgen]
    pub fn get_probability(&mut self, res: u32) -> f64 {
        let measurable = self.get_state_vector();
        let basis: QuBitBasis<BITS> = match UInt::<BITS>::try_from(res) {
            Ok(v) => v,
            Err(_) => return 0.
        }.into();
        measurable.probability(StateVector::new(basis))
    }

    #[wasm_bindgen]
    pub fn run(&mut self) -> u32 {
        let result = self.get_state_vector().measure::<QuBitBasis<BITS>>();
        for basis in <QuBitBasis::<BITS> as Basis>::iter() {
            if result.probability(StateVector::new(basis.clone())) > 0. { // should be exactly one but to be safe
                return UInt::<BITS>::from(basis).into();
            }
        }
        unreachable!("No results to measurement vector!");
    }

    #[wasm_bindgen]
    pub fn get_gates(&self) -> Vec<Gate> {
        self.gates.clone()
    }

    #[wasm_bindgen]
    pub fn get_component(&mut self, res: u32) -> Complex {
        let measurable = self.get_state_vector();
        let basis: QuBitBasis<BITS> = match UInt::<BITS>::try_from(res) {
            Ok(v) => v,
            Err(_) => return Complex { re: 0., im: 0. }
        }.into();
        measurable.get_component(basis).into()
    }
}
