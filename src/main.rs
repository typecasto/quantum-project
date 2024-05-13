#![allow(unused, dead_code)]

use std::env::args;

use color_eyre::eyre::OptionExt;
use color_eyre::Result as R;

use quantum_project::{Clifford, PauliOperator};
use quantum_project::{I, X, Y, Z};
fn main() -> R<()> {
    color_eyre::install()?;

    // Get n from the command line
    let n: usize = args()
        .nth(1)
        .ok_or_eyre("Please give a number of qubits to generate a random circuit, or 0 to generate the circuit from figure 5.")?
        .parse()?;
    let circuit = if n == 0 {
        // if it equals zero, give the circuit from the paper
        eprintln!("Given size 0, recreating the circuit from figure 5 instead.");
        let mut c = Clifford {
            circuit: vec![],
            tableau: ["+XYYX", "+YYYX", "+IZI", "+YYI", "+IX", "+IZ", "+Z", "+X"]
                .map(|x| x.try_into().unwrap())
                .to_vec(),
        };
        c.sweep();
        c.circuit
    } else {
        // otherwise generate a circuit of size n
        Clifford::gen_circuit(n).circuit
    };
    for gate in circuit.iter() {
        println!("{}", gate)
    }

    Ok(())
}
