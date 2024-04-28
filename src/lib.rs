#![allow(unused, dead_code)]

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
    pub fn phase_gate(&self) -> Self {
        Self {
            x: self.x,
            z: self.x ^ self.z,
        }
    }
    // CX(a, b)
    pub fn cx_gate(&self, other: &Self) -> (Self, Self) {
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

impl Display for Pauli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.clone().into() {
                (false, false) => "I",
                (false, true) => "Z",
                (true, false) => "X",
                (true, true) => "Y",
            }
        )
    }
}

impl From<(bool, bool)> for Pauli {
    fn from(value: (bool, bool)) -> Self {
        Self {
            x: value.0,
            z: value.1,
        }
    }
}

impl From<Pauli> for (bool, bool) {
    fn from(value: Pauli) -> Self {
        (value.x, value.z)
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct PauliOperator {
    pub ops: Vec<Pauli>,
    pub s: bool,
}

// make it so we can basically treat this as a Vec<Pauli> and everything will Just Work:tm:
impl std::ops::Deref for PauliOperator {
    type Target = Vec<Pauli>;

    fn deref(&self) -> &Self::Target {
        &self.ops
    }
}

impl PauliOperator {
    /// Apply the hadamard gate to a single bit.
    pub fn hadamard(&mut self, a: usize) {
        self.ops[a] = self.ops[a].hadamard();
    }
    /// Apply the phase gate to a single bit
    pub fn phase_gate(&mut self, a: usize) {
        self.ops[a] = self.ops[a].phase_gate();
    }
    /// Apply the CX gate to two bits
    pub fn cx_gate(&mut self, a: usize, b: usize) {
        (self.ops[a], self.ops[b]) = Pauli::cx_gate(&self.ops[a], &self.ops[b]);
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

impl Display for PauliOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in self.ops.iter() {
            if i.x {
                write!(f, "X ")?;
            } else {
                write!(f, "_ ")?;
            }
        }
        write!(f, "| ")?;
        for i in self.ops.iter() {
            if i.z {
                write!(f, "Z ")?;
            } else {
                write!(f, "_ ")?;
            }
        }
        Ok(())
    }
}
