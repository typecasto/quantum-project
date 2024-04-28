use std::fmt::Display;

use super::Pauli;

use super::PauliOperator;

// make it so we can basically treat this as a Vec<Pauli> and everything will Just Work:tm:
impl std::ops::Deref for PauliOperator {
    type Target = Vec<Pauli>;

    fn deref(&self) -> &Self::Target {
        &self.ops
    }
}

impl Display for PauliOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in self.ops.iter() {
            write!(f, "{}", i)?;
        }
        write!(f, " - ")?;
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
