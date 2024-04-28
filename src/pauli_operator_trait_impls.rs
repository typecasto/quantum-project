use std::fmt::Display;

use super::Pauli;

use super::PauliOperator;

use super::{I, X, Y, Z};

use color_eyre::Report;

// make it so we can basically treat this as a Vec<Pauli> and everything will Just Work:tm:
impl std::ops::Deref for PauliOperator {
    type Target = Vec<Pauli>;
    fn deref(&self) -> &Self::Target {
        &self.ops
    }
}

// same but in a mutable context
impl std::ops::DerefMut for PauliOperator {
    fn deref_mut(&mut self) -> &mut Vec<Pauli> {
        &mut self.ops
    }
}

impl Display for PauliOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.sign {"-"} else {"+"})?;
        for i in self.ops.iter() {
            write!(f, "{}", i)?;
        }
        write!(f, " - ")?;
        for i in self.ops.iter() {
            if i.x {
                write!(f, "X")?;
            } else {
                write!(f, "_")?;
            }
        }
        write!(f, "|")?;
        for i in self.ops.iter() {
            if i.z {
                write!(f, "Z")?;
            } else {
                write!(f, "_")?;
            }
        }
        Ok(())
    }
}

// Must match the regex /[+-][IXYZ]+/
impl TryFrom<&str> for PauliOperator {
    type Error = color_eyre::Report;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let chars = &mut value.chars();
        let sign = match chars.next() {
            Some('-') => true,
            Some('+') => false,
            _ => return Err(Report::msg("Sign required, must be '+' or '-'")),
        };
        let mut ops = vec![];
        for x in chars {
            ops.push(match x {
                'I' => I,
                'X' => X,
                'Y' => Y,
                'Z' => Z,
                _ => return Err(Report::msg("Paulis can be only 'I', 'X', 'Y', and 'Z'.")),
                
            });
        };
        Ok(Self {ops, sign})
    }
}
