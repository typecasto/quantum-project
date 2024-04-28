use super::Pauli;

use std::fmt::Display;

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
