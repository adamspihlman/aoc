use std::collections::HashMap;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Operation {
    And,
    Xor,
    Or,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Gate {
    input1_name: String,
    input2_name: String,
    output_name: String,
    op: Operation,
}

impl Gate {
    pub fn new(input1_name: String, input2_name: String, output_name: String, op: Operation) -> Self {
        Self {
            input1_name,
            input2_name,
            output_name,
            op,
        }
    }
}

#[derive(Debug)]
pub struct Logic {
    gates: Vec<Gate>,
    wire_values: HashMap<String, Option<bool>>,
}

impl Logic {
    pub fn new(gates: Vec<Gate>, wire_values: HashMap<String, Option<bool>>) -> Self {
        Self { gates, wire_values }
    }

    pub fn propagate_once(&mut self) -> bool {
        let mut changes_made = false;

        for gate in &self.gates {
            if let Some(Some(_)) = self.wire_values.get(&gate.output_name) {
                continue;
            }

            let input1_val = self.wire_values.get(&gate.input1_name);
            let input2_val = self.wire_values.get(&gate.input2_name);

            if let (Some(Some(v1)), Some(Some(v2))) = (input1_val, input2_val) {
                let result = match gate.op {
                    Operation::And => v1 & v2,
                    Operation::Or => v1 | v2,
                    Operation::Xor => v1 ^ v2,
                };

                self.wire_values.insert(gate.output_name.clone(), Some(result));
                changes_made = true;
            }
        }

        changes_made
    }

    pub fn propagate_until_stable(&mut self) {
        while self.propagate_once() {}
    }

    pub fn get_output_number(&self) -> u64 {
        let mut z_wires: Vec<(&String, &Option<bool>)> = self
            .wire_values
            .iter()
            .filter(|(name, _)| name.starts_with('z'))
            .collect();

        z_wires.sort_by_key(|(name, _)| *name);

        let mut result = 0u64;
        for (i, (_, value)) in z_wires.iter().enumerate() {
            if let Some(true) = value {
                result |= 1 << i;
            }
        }

        result
    }
}
