use num::pow;

use crate::operator::LinearOperator;
use crate::computer::QuBitBasis;
use crate::operator::LinearOperatorTrait;
use crate::vector::Component;

// https://en.wikipedia.org/wiki/Quantum_logic_gate

#[derive(Clone)]
pub struct IdentityGate();

impl IdentityGate {
    pub fn new<const N: usize>() -> LinearOperator<QuBitBasis<N>> {
        LinearOperator::new(IdentityGate())
    }
}

impl<const N: usize> LinearOperatorTrait<QuBitBasis<N>> for IdentityGate {
    fn get_component(&self, index: (QuBitBasis<N>, QuBitBasis<N>)) -> Component {
        if index.0 == index.1 {
            Component::ONE
        } else {
            Component::ZERO
        }
    }
}

#[derive(Clone)]
pub struct PauliXGate {
    bit: usize
}

impl PauliXGate {
    pub fn new<const N: usize>(bit: usize) -> Option<LinearOperator<QuBitBasis<N>>> {
        if bit >= N {
            println!("Tried to create X gate on non-existent bit!");
            None
        } else {
            Some(LinearOperator::new(Self { bit }))
        }
    }
}

impl<const N: usize> LinearOperatorTrait<QuBitBasis<N>> for PauliXGate {
    fn get_component(&self, index: (QuBitBasis<N>, QuBitBasis<N>)) -> Component {
        if index.0.get(self.bit) == index.1.get(self.bit) {
            Component::ZERO
        } else {
            if index.0.eq_except(&index.1, &[self.bit]) {
                Component::ONE
            } else {
                Component::ZERO
            }
        }
    }
}

#[derive(Clone)]
pub struct PauliYGate {
    bit: usize
}

impl PauliYGate {
    pub fn new<const N: usize>(bit: usize) -> Option<LinearOperator<QuBitBasis<N>>> {
        if bit >= N {
            println!("Tried to create Pauli Y gate on non-existent bit!");
            None
        } else {
            Some(LinearOperator::new(Self { bit }))
        }
    }
}

impl<const N: usize> LinearOperatorTrait<QuBitBasis<N>> for PauliYGate {
    fn get_component(&self, index: (QuBitBasis<N>, QuBitBasis<N>)) -> Component {
        (if index.0.eq_except(&index.1, &[self.bit]) {
            Component::ONE
        } else {
            Component::ZERO
        }) * (match (index.0.get(self.bit), index.1.get(self.bit)) {
            (false, false) | (true, true) => Component::ZERO,
            (false, true) => -Component::I,
            (true, false) => Component::I
        })
    }
}

#[derive(Clone)]
pub struct PauliZGate {
    bit: usize
}

impl PauliZGate {
    pub fn new<const N: usize>(bit: usize) -> Option<LinearOperator<QuBitBasis<N>>> {
        if bit >= N {
            println!("Tried to create Pauli Z gate on non-existent bit!");
            None
        } else {
            Some(LinearOperator::new(Self { bit }))
        }
    }
}

impl<const N: usize> LinearOperatorTrait<QuBitBasis<N>> for PauliZGate {
    fn get_component(&self, index: (QuBitBasis<N>, QuBitBasis<N>)) -> Component {
        (if index.0.eq_except(&index.1, &[self.bit]) {
            Component::ONE
        } else {
            Component::ZERO
        }) * (match (index.0.get(self.bit), index.1.get(self.bit)) {
            (false, false) => Component::ONE,
            (true, true) => -Component::ONE,
            (false, true) | (true, false) => Component::ZERO
        })
    }
}

#[derive(Clone)]
pub struct HadamardGate {
    bit: usize
}

impl HadamardGate {
    pub fn new<const N: usize>(bit: usize) -> Option<LinearOperator<QuBitBasis<N>>> {
        if bit >= N {
            println!("Tried to create Hadamard gate on non-existent bit!");
            None
        } else {
            Some(LinearOperator::new(Self { bit }))
        }
    }
}

impl<const N: usize> LinearOperatorTrait<QuBitBasis<N>> for HadamardGate {
    fn get_component(&self, index: (QuBitBasis<N>, QuBitBasis<N>)) -> Component {
        (if index.0.eq_except(&index.1, &[self.bit]) {
            Component::ONE
        } else {
            Component::ZERO
        }) * (match (index.0.get(self.bit), index.1.get(self.bit)) {
            (true, true) => -Component::ONE,
            _ => Component::ONE
        }) * Component::new(std::f64::consts::FRAC_1_SQRT_2, 0.)
    }
}

#[derive(Clone)]
pub struct PhaseGate {
    bit: usize
}

impl PhaseGate {
    pub fn new<const N: usize>(bit: usize) -> Option<LinearOperator<QuBitBasis<N>>> {
        if bit >= N {
            println!("Tried to create phase gate on non-existent bit!");
            None
        } else {
            Some(LinearOperator::new(Self { bit }))
        }
    }
}

impl<const N: usize> LinearOperatorTrait<QuBitBasis<N>> for PhaseGate {
    fn get_component(&self, index: (QuBitBasis<N>, QuBitBasis<N>)) -> Component {
        (if index.0.eq_except(&index.1, &[self.bit]) {
            Component::ONE
        } else {
            Component::ZERO
        }) * (match (index.0.get(self.bit), index.1.get(self.bit)) {
            (true, true) => Component::I,
            (false, false) => Component::ONE,
            _ => Component::ZERO
        })
    }
}

#[derive(Clone)]
pub struct TGate {
    bit: usize
}

impl TGate {
    pub fn new<const N: usize>(bit: usize) -> Option<LinearOperator<QuBitBasis<N>>> {
        if bit >= N {
            println!("Tried to create T gate on non-existent bit!");
            None
        } else {
            Some(LinearOperator::new(Self { bit }))
        }
    }
}

impl<const N: usize> LinearOperatorTrait<QuBitBasis<N>> for TGate {
    fn get_component(&self, index: (QuBitBasis<N>, QuBitBasis<N>)) -> Component {
        (if index.0.eq_except(&index.1, &[self.bit]) {
            Component::ONE
        } else {
            Component::ZERO
        }) * (match (index.0.get(self.bit), index.1.get(self.bit)) {
            (true, true) => Component::exp(Component::new(0., std::f64::consts::FRAC_PI_4)),
            (false, false) => Component::ONE,
            _ => Component::ZERO
        })
    }
}

#[derive(Clone)]
pub struct CNOTGate {
    control: Vec<usize>,
    target: usize
}

impl CNOTGate {
    pub fn new<const N: usize>(target: usize, control: Vec<usize>) -> Option<LinearOperator<QuBitBasis<N>>> {
        if target >= N {
            println!("Tried to create CNOT gate on non-existent bits!");
            None
        } else {
            for bit in control.iter() {
                if *bit >= N {
                    println!("Tried to create CNOT gate on non-existent bits!");
                    return None
                }
            }
            Some(LinearOperator::new(Self { target, control }))
        }
    }
}

impl<const N: usize> LinearOperatorTrait<QuBitBasis<N>> for CNOTGate {
    fn get_component(&self, index: (QuBitBasis<N>, QuBitBasis<N>)) -> Component {
        let mut is1 = true;
        for bit in self.control.iter() {
            if !(index.0.get(*bit) && index.1.get(*bit)) {
                is1 = false;
                break;
            }
        }
        (if index.0.eq_except(&index.1, &[self.target]) {
            Component::ONE
        } else {
            Component::ZERO
        }) * (if is1 {
            if index.0.get(self.target) != index.1.get(self.target) {
                Component::ONE
            } else {
                Component::ZERO
            }
        } else if index.0.get(self.target) == index.1.get(self.target) {
            Component::ONE
        }
        else {
            Component::ZERO
        })
    }
}

#[derive(Clone)]
pub struct CZGate {
    bits: Vec<usize>
}

impl CZGate {
    pub fn new<const N: usize>(bits: Vec<usize>) -> Option<LinearOperator<QuBitBasis<N>>> {
        for bit in bits.iter() {
            if *bit >= N {
                println!("Tried to create CZGate over non-existent bit!");
                return None
            }
        }
        Some(LinearOperator::new(Self { bits }))
    }
}

impl<const N: usize> LinearOperatorTrait<QuBitBasis<N>> for CZGate {
    fn get_component(&self, index: (QuBitBasis<N>, QuBitBasis<N>)) -> Component {
        if index.0.eq(&index.1) {
            for bit in self.bits.iter() {
                if !index.0.get(*bit) {
                    return Component::ONE;
                }
            }
            -Component::ONE
        } else {
            Component::ZERO
        }
    }
}

#[derive(Clone)]
pub struct SwapGate {
    bit1: usize,
    bit2: usize
}

impl SwapGate {
    pub fn new<const N: usize>(bit1: usize, bit2: usize) -> Option<LinearOperator<QuBitBasis<N>>> {
        if bit1 >= N || bit2 >= N {
            println!("Tried to create swap gate on non-existent bits!");
            None
        } else {
            Some(LinearOperator::new(Self { bit1, bit2 }))
        }
    }
}

impl<const N: usize> LinearOperatorTrait<QuBitBasis<N>> for SwapGate {
    fn get_component(&self, index: (QuBitBasis<N>, QuBitBasis<N>)) -> Component {
        (if index.0.eq_except(&index.1, &[self.bit1, self.bit2]) {
            Component::ONE
        } else {
            Component::ZERO
        }) * (match ((index.0.get(self.bit1), index.0.get(self.bit2)), (index.1.get(self.bit1), index.1.get(self.bit2))) {
            ((false, false), (false, false)) | ((false, true), (true, false))
            | ((true, false), (false, true)) | ((true, true), (true, true))=> Component::ONE,
            _ => Component::ZERO
        })
    }
}

#[derive(Clone)]
pub struct GroverOracle<const N: usize> {
    target: QuBitBasis<N>
}

impl<const N: usize> GroverOracle<N> {
    pub fn new(target: QuBitBasis<N>) -> LinearOperator<QuBitBasis<N>> {
        LinearOperator::new(Self { target })
    }
}

impl<const N: usize> LinearOperatorTrait<QuBitBasis<N>> for GroverOracle<N> {
    fn get_component(&self, index: (QuBitBasis<N>, QuBitBasis<N>)) -> Component {
        if index.0 == index.1 {
            if index.0 == self.target {
                -Component::ONE
            } else {
                Component::ONE
            }
        } else {
            Component::ZERO
        }
    }
}

#[derive(Clone)]
pub struct GroverDiffuser {
    bits: Vec<usize>
}

impl GroverDiffuser {
    pub fn new<const N: usize>(bits: Vec<usize>) -> Option<LinearOperator<QuBitBasis<N>>> {
        for bit in bits.iter() {
            if *bit >= N {
                println!("Tried to create GroverDiffuser over non-existent bit!");
                return None
            }
        }
        Some(LinearOperator::new(Self { bits }))
    }
}

impl<const N: usize> LinearOperatorTrait<QuBitBasis<N>> for GroverDiffuser {
    fn get_component(&self, index: (QuBitBasis<N>, QuBitBasis<N>)) -> Component {
        // https://github.com/Qiskit/textbook/blob/main/notebooks/ch-algorithms/grover.ipynb
        // 2|s><s| - I where s = \sum 1/rt2^N |i> => (|s><s| = \sum \sum 1/2^N |i><j| =>) <m|s><s|n> = 1/sqrt 2^N * 1/sqrt 2^N = 1/2^N
        // thus <i|(2|s><s| - I)|j> = 2/2^N - \dirac_delta_i,j
        let not_bits: Vec<usize> = (0..N).into_iter().filter(|b| !self.bits.contains(b)).collect();
        (Component::ONE / (pow(2, self.bits.len() - 1) as f64) - (if index.0.eq_except(&index.1, not_bits.as_slice()) {
            Component::ONE
        } else { Component::ZERO })) * (if index.0.eq_except(&index.1, self.bits.as_slice()) {
            Component::ONE
        } else {
            Component::ZERO
        })
    }
}
