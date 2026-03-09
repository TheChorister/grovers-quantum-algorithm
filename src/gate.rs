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
    bit1: usize,
    bit2: usize
}

impl CNOTGate {
    pub fn new<const N: usize>(bit1: usize, bit2: usize) -> Option<LinearOperator<QuBitBasis<N>>> {
        if bit1 >= N || bit2 >= N {
            println!("Tried to create CNOT gate on non-existent bits!");
            None
        } else {
            Some(LinearOperator::new(Self { bit1, bit2 }))
        }
    }
}

impl<const N: usize> LinearOperatorTrait<QuBitBasis<N>> for CNOTGate {
    fn get_component(&self, index: (QuBitBasis<N>, QuBitBasis<N>)) -> Component {
        (if index.0.eq_except(&index.1, &[self.bit1, self.bit2]) {
            Component::ONE
        } else {
            Component::ZERO
        }) * (match ((index.0.get(self.bit1), index.0.get(self.bit2)), (index.1.get(self.bit1), index.1.get(self.bit2))) {
            ((false, false), (false, false)) | ((false, true), (false, true)) => Component::ONE,
            ((true, false), (true, true)) => Component::ONE,
            ((true, true), (true, false)) => Component::ONE,
            _ => Component::ZERO
        })
    }
}

#[derive(Clone)]
pub struct CZGate {
    bit1: usize,
    bit2: usize
}

impl CZGate {
    pub fn new<const N: usize>(bit1: usize, bit2: usize) -> Option<LinearOperator<QuBitBasis<N>>> {
        if bit1 >= N || bit2 >= N {
            println!("Tried to create controlled z gate on non-existent bits!");
            None
        } else {
            Some(LinearOperator::new(Self { bit1, bit2 }))
        }
    }
}

impl<const N: usize> LinearOperatorTrait<QuBitBasis<N>> for CZGate {
    fn get_component(&self, index: (QuBitBasis<N>, QuBitBasis<N>)) -> Component {
        (if index.0.eq_except(&index.1, &[self.bit1, self.bit2]) {
            Component::ONE
        } else {
            Component::ZERO
        }) * (match ((index.0.get(self.bit1), index.0.get(self.bit2)), (index.1.get(self.bit1), index.1.get(self.bit2))) {
            ((false, false), (false, false)) | ((false, true), (false, true))
            | ((true, false), (true, false)) => Component::ONE,
            ((true, true), (true, true)) => -Component::ONE,
            _ => Component::ZERO
        })
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
