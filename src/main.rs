#![allow(unused, dead_code)]

use std::env::args;

use color_eyre::eyre::OptionExt;
use color_eyre::Result as R;

use quantum_project::{Clifford, PauliOperator};
use quantum_project::{I, X, Y, Z};
fn main() -> R<()> {
    color_eyre::install()?;
    
    // Get n from the command line
    let n: usize = args().nth(1).ok_or_eyre("Argument missing: number of qubits.")?.parse()?;
    // let mut c = Clifford {
    //     circuit: vec![],
    //     tableau: ["+XYYX", "+YYYX", "+IZI", "+YYI", "+IX", "+IZ", "+Z", "+X"].map(|x| x.try_into().unwrap()).to_vec(),
    // };
    // c.sweep();
    for gate in Clifford::gen_circuit(20).circuit.iter() {
        println!("{}", gate)
    }

    Ok(())
}
