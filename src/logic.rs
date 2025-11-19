use std::collections::HashSet;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Operation {
    And,
    Xor,
    Or,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Wire {
    name: String,
    value: Option<bool>,
}

impl Wire {
    pub fn new(name: String, value: Option<bool>) -> Self {
        Self { name, value }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Gate {
    input1: Wire,
    input2: Wire,
    output: Wire,
    op: Operation,
}

impl Gate {
    pub fn new(input1: Wire, input2: Wire, output: Wire, op: Operation) -> Self {
        Self {
            input1,
            input2,
            output,
            op,
        }
    }
}

#[derive(Debug)]
pub struct Logic {
    gates: HashSet<Gate>,
}

impl Logic {
    pub fn new(gates: HashSet<Gate>) -> Self {
        Self { gates }
    }
}
