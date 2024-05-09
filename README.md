# Usage
Install rust, and run `cargo run <n>` in the project directory, where `n` is the number of qubits in the circuit. Output will be a list of gates in order.

If n is 0, it will sweep the tableau from figure 5, and output the corresponding circuit, which matches the one from the paper.

# List of files:
`src/main.rs` - Frontend, handles creating and outputting a circuit.
`src/lib.rs` - Backend, handles the sweeping procedure as well as various associated methods for the types.
`src/*_trait_impls.rs` - Boring implementations that are mostly fluff, like `Display`.

# References
<!--Just a temporary list for now in whatever format, to be cleaned up later-->
1. A simple method for sampling random Clifford operators by Ewout van den Berg
2. Improved simulation of stabilizer circuits, by Aaronson and Gottesman
