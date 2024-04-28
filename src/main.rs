#![allow(unused, dead_code)]

use std::env::args;

use color_eyre::eyre::OptionExt;
use color_eyre::Result as R;

use quantum_project::PauliOperator;
use quantum_project::{I, X, Y, Z};
fn main() -> R<()> {
    color_eyre::install()?;
    
    // Get n from the command line
    let n: usize = args().nth(1).ok_or_eyre("Argument missing: number of qubits.")?.parse()?;

    let (a, b) = PauliOperator::gen_anticommuting_pair(n);
    println!("{}\n{}", a, b);

    Ok(())
}
