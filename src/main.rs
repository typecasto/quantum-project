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

    // let (mut a, mut b) = PauliOperator::gen_anticommuting_pair(n);
    let mut a = "+XYYX".try_into()?;
    let mut b = "+YYYX".try_into()?;
    // let mut a = "+IZI".try_into()?;
    // let mut b = "+YYI".try_into()?;
    println!("{}\n{}\n", a, b);
    for gate in PauliOperator::sweep(&mut a, &mut b).iter() {
        println!("{}", gate)
    }

    Ok(())
}
