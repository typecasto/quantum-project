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
    pub fn phase(&self) -> Self {
        Self {
            x: self.x,
            z: self.x ^ self.z,
        }
    }
    // CX(a, b)
    pub fn cnot(&self, target: &Self) -> (Self, Self) {
        // a=self, b=other
        (
            Self {
                z: self.z ^ target.z,
                x: self.x,
            },
            Self {
                x: self.x ^ target.x,
                z: target.z,
            },
        )
    }
    pub fn gen_random() -> Self {
        (fastrand::bool(), fastrand::bool()).into()
    }
}

// extract these to their own files, they're just fluff
mod pauli_trait_impls;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Gate {
    Hadamard(usize),
    Phase(usize),
    CNot(usize, usize),
    Swap(usize, usize),
}

impl Display for Gate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gate::Hadamard(a) => write!(f, "Hadamard({})", a),
            Gate::Phase(a) => write!(f, "Phase({})", a),
            Gate::CNot(a, b) => write!(f, "CNot({}, {})", a, b),
            Gate::Swap(a, b) => write!(f, "Swap({}, {})", a, b),
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct PauliOperator {
    pub ops: Vec<Pauli>,
    pub sign: bool,
}

impl PauliOperator {
    /// Apply the hadamard gate to a single bit.
    pub fn hadamard(&mut self, a: usize) {
        self.sign ^= self[a] == Y;
        self[a] = self[a].hadamard();
    }
    /// Apply the phase gate to a single bit
    pub fn phase(&mut self, a: usize) {
        self.sign ^= self[a] == Y;
        self[a] = self[a].phase();
    }
    /// Apply the CX gate to two bits, a is control, b is target
    pub fn cnot(&mut self, a: usize, b: usize) {
        let ((xa, za), (xb, zb)) = (self[a].into(), self[b].into());
        self.sign ^= xa && zb && (xb ^ za ^ true);
        (self[a], self[b]) = Pauli::cnot(&self[a], &self[b]);
    }
    /// Apply a certain gate
    pub fn apply(&mut self, gate: &Gate) {
        match gate {
            Gate::Hadamard(a) => self.hadamard(*a),
            Gate::Phase(a) => self.phase(*a),
            Gate::CNot(a, b) => self.cnot(*a, *b),
            Gate::Swap(a, b) => {
                //? I'm pretty sure that 3 CNOTs will maintain the sign bit,
                //? so this could just be replaced with `self.swap(*a, *b)`, but
                //? I'm not sure how to rigerously prove that, so I'm just leaving
                //? it as 3 CNOTs just on the off chance my intiution is wrong.
                self.cnot(*a, *b);
                self.cnot(*b, *a);
                self.cnot(*a, *b);
            }
        }
    }
    /// Returns true if this operator commutes with the other (e.g. PQ = QP)
    /// and false if they anticommute (e.g. PQ = -QP)
    pub fn commutes(&self, other: &Self) -> bool {
        let mut total = 0;
        assert_eq!(self.len(), other.len());
        for i in 0..self.len() {
            // two paulis anticommute iff they're different and neither is the identity
            if self[i] != other[i] && self[i] != I && other[i] != I {
                total += 1;
            }
        }
        // if there's an odd number of anticommuting paulis, the operators anticommute
        return total % 2 == 0;
    }
    /// Generate two anticommuting pauli operators
    pub fn gen_anticommuting_pair(n: usize) -> (Self, Self) {
        // just keep generating random paulis until they anticommute
        loop {
            let a = Self::gen_random(n);
            if a.iter().all(|x| x == &I) {
                // make sure the first isn't the identity
                continue;
            }
            let b = Self::gen_random(n);
            if !a.commutes(&b) {
                return (a, b);
            }
        }
    }
    /// Generate a (pseudo)random PauliOperator, which may be the identity
    fn gen_random(n: usize) -> Self {
        Self {
            ops: repeat_with(Pauli::gen_random).take(n).collect(),
            //? This is something I'm really not sure about.
            //? Is the sign just a random variable? Or is it a function of the paulis
            //? in the operator? I think it's random because paper 2 specifies
            //? +IZ and +ZI instead of just labeling them IZ and ZI, implying that
            //? there could in theory be a -IZ and -ZI, and they're stored as a
            //? separate column in the matrix, but I can't find anywhere that's
            //? explicitly laid out. Working under this assumption for now.
            sign: fastrand::bool(),
        }
    }
    /// Pad this to the left with the identity, e.g. XZYI.left_pad(5) -> IXZYI
    ///
    /// Does nothing if `new_size <= self.len()`
    pub fn left_pad(&mut self, new_size: usize) {
        while self.len() > new_size {
            self.insert(0, I);
        }
    }
    /// Sweeps a pair of tableaus to the +XIIII... and +ZIIII... states and returns
    /// the circuit used to do so.
    pub fn sweep(&mut self, other: &mut Self) -> Circuit {
        use Gate::*; // Put the gate names in scope
        let mut circuit = Circuit::new();
        let (a, b) = (self, other); // give them more convenient names
        assert_eq!(a.len(), b.len());

        // Step 1: Clear all of the z_a bits using H and S
        println!("--- Step 1 ---");
        for i in 0..a.len() {
            // For all positions where z_a is 1, apply S if x_a = 1, else H
            if a[i].z {
                let gate = if a[i].x { Phase(i) } else { Hadamard(i) };
                a.apply(&gate);
                b.apply(&gate);
                circuit.push(gate);
                println!("{}\n{}\n", a, b); //debug
            }
        }

        // Step 2: Clear all but one x_a bits using CNOT gates
        println!("--- Step 2 ---");
        let first_position = loop {
            // get list of indices for which a_x is 1
            let j: Vec<_> = (0..a.len()).filter(|i| a[*i].x).collect();
            assert_ne!(j.len(), 0);
            if j.len() == 1 {
                break j[0];
            }
            for chunk in j.chunks_exact(2) {
                let [c, t] = chunk else {
                    unreachable!("chunks_exact is guaranteed to return 2-chunks")
                };
                let gate = CNot(*c, *t);
                a.apply(&gate);
                b.apply(&gate);
                circuit.push(gate);
            }
            println!("{}\n{}\n", a, b); //debug
        };

        // Step 3: Move the x_a bit into the first position
        if first_position != 0 {
            println!("--- Step 3 ---");
            let gate = Swap(0, first_position);
            a.apply(&gate);
            b.apply(&gate);
            circuit.push(gate);
            println!("{}\n{}\n", a, b); //debug
        }

        // Step 4: Clear out the 2nd row by repeating steps 1 and 2 (steps 4b and 4c)
        if !(b[0] == Z && b[1..].iter().all(|p| p == &I)) {
            // if b != +/-ZIII...

            println!("--- Step 4a ---");
            let gate = Hadamard(0);
            a.apply(&gate);
            b.apply(&gate);
            circuit.push(gate);
            println!("{}\n{}\n", a, b); //debug

            // Step 4a: Repeat step 1 on the 2nd row
            println!("--- Step 4a ---");
            for i in 0..a.len() {
                // For all positions where z_b is 1, apply S if x_b = 1, else H
                if b[i].z {
                    let gate = if b[i].x { Phase(i) } else { Hadamard(i) };
                    a.apply(&gate);
                    b.apply(&gate);
                    circuit.push(gate);
                    println!("{}\n{}\n", a, b); //debug
                }
            }

            // Step 4b: Repeat step 2 on the 2nd row
            println!("--- Step 4b ---");
            let first_position = loop {
                // get list of indices for which b_x is 1
                let j: Vec<_> = (0..b.len()).filter(|i| b[*i].x).collect();
                assert_ne!(j.len(), 0);
                if j.len() == 1 {
                    break j[0];
                }
                for chunk in j.chunks_exact(2) {
                    let [c, t] = chunk else {
                        unreachable!("chunks_exact is guaranteed to return 2-chunks")
                    };
                    let gate = CNot(*c, *t);
                    a.apply(&gate);
                    b.apply(&gate);
                    circuit.push(gate);
                }
                println!("{}\n{}\n", a, b); //debug
            };
            // Step 4d
            println!("--- Step 4d ---");
            let gate = Hadamard(0);
            a.apply(&gate);
            b.apply(&gate);
            circuit.push(gate);
            println!("{}\n{}\n", a, b); //debug
        }
        circuit
    }
}

mod pauli_operator_trait_impls;

type Circuit = Vec<Gate>;
pub struct Clifford {
    circuit: Circuit,
    tableau: Vec<PauliOperator>, // POs yet to be determined just aren't placed in the array
                                 // The size is implicit, it is the size of the first PO in the the tableau.
                                 // Sizes of later elements may be smaller.
}
