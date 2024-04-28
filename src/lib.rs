#![allow(unused, dead_code)]

//! Code in this repository is heavily based on the paper:
//! "A simple method for sampling random Clifford operators"
//! by Ewout van den Berg
//! published in 2021 

use color_eyre::Result as R;
use std::fmt::Display;
use std::iter::repeat_with;
use std::num::NonZeroU64;

pub const I: Pauli = Pauli { x: false, z: false };
pub const Z: Pauli = Pauli { x: false, z: true };
pub const X: Pauli = Pauli { x: true, z: false };
pub const Y: Pauli = Pauli { x: true, z: true };

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Pauli {
    x: bool,
    z: bool,
}

impl Pauli {
    /// H(a)
    pub fn hadamard(&self) -> Self {
        Self {
            x: self.z,
            z: self.x,
        }
    }
    /// S(a)
    pub fn phase(&self) -> Self {
        Self {
            x: self.x,
            z: self.x ^ self.z,
        }
    }
    // CX(a, b)
    pub fn cnot(&self, other: &Self) -> (Self, Self) {
        // a=self, b=other
        (
            Self {
                z: self.z ^ other.z,
                x: self.x,
            },
            Self {
                x: self.x ^ other.x,
                z: other.z,
            },
        )
    }
    pub fn gen_random() -> Self {
        (fastrand::bool(), fastrand::bool()).into()
    }
}

// extract these to their own files, they're just fluff
mod pauli_trait_impls;

#[derive(PartialEq, Eq, Clone)]
pub struct PauliOperator {
    pub ops: Vec<Pauli>,
    pub s: bool,
}

impl PauliOperator {
    /// Apply the hadamard gate to a single bit.
    pub fn hadamard(&mut self, a: usize) {
        self.ops[a] = self.ops[a].hadamard();
    }
    /// Apply the phase gate to a single bit
    pub fn phase(&mut self, a: usize) {
        self.ops[a] = self.ops[a].phase();
    }
    /// Apply the CX gate to two bits
    pub fn cnot(&mut self, a: usize, b: usize) {
        (self.ops[a], self.ops[b]) = Pauli::cnot(&self.ops[a], &self.ops[b]);
    }
    /// Returns true if this operator commutes with the other (e.g. PQ = QP)
    /// and false if they anticommute (e.g. PQ = -QP)
    pub fn commutes(&self, other: &Self) -> bool {
        let mut total = 0;
        assert_eq!(self.ops.len(), other.ops.len());
        for i in 0..self.ops.len() {
            // two paulis anticommute iff they're different and neither is the identity
            if self.ops[i] != other.ops[i] && self.ops[i] != I && other.ops[i] != I {
                total += 1;
            }
        }
        // if there's an odd number of anticommuting paulis, the operators anticommute
        return total % 2 == 0;
    }
    /// Generate two anticommuting pauli operators
    pub fn gen_anticommuting_pair(n: usize) -> (Self, Self) {
        loop {
            let a = Self::gen_random(n);
            let b = Self::gen_random(n);
            if !a.commutes(&b) {
                return (a, b);
            }
            // else {
            //     println!("{}\n{}", a, b);
            // }
        }
    }
    /// Generate a (pseudo)random PauliOperator, which may be the identity
    fn gen_random(n: usize) -> Self {
        Self {
            ops: repeat_with(Pauli::gen_random).take(n).collect(),
            s: true, //todo: i have no clue how the state bit works.
        }
    }
}

mod pauli_operator_trait_impls;

pub enum Gate {
    Hadamard(usize),
    Phase(usize),
    CNot(usize, usize)
}
