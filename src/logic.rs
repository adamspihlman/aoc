use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Operation {
    And,
    Xor,
    Or,
}

#[derive(Debug, Clone)]
pub struct HalfAdder {
    pub x: String,
    pub y: String,
    pub sum: String,
    pub carry: String,
}

#[derive(Debug, Clone)]
pub struct FullAdder {
    pub x: String,
    pub y: String,
    pub c_in: String,
    pub sum: String,
    pub c_out: String,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Gate {
    input1_name: String,
    input2_name: String,
    output_name: String,
    op: Operation,
}

impl Gate {
    pub fn new(
        input1_name: String,
        input2_name: String,
        output_name: String,
        op: Operation,
    ) -> Self {
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

                self.wire_values
                    .insert(gate.output_name.clone(), Some(result));
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

    /// Generate a Graphviz DOT format representation of the circuit
    /// Can be rendered with: dot -Tpng circuit.dot -o circuit.png
    pub fn to_dot(&self) -> String {
        let mut dot = String::from("digraph Circuit {\n");
        dot.push_str("  rankdir=LR;\n");
        dot.push_str("  node [shape=box];\n\n");

        // Define input wires (x and y)
        dot.push_str("  // Input wires\n");
        for (wire, value) in &self.wire_values {
            if wire.starts_with('x') || wire.starts_with('y') {
                let color = match value {
                    Some(true) => "green",
                    Some(false) => "red",
                    None => "gray",
                };
                let label = match value {
                    Some(v) => format!("{}\\n{}", wire, if *v { "1" } else { "0" }),
                    None => format!("{}\\n?", wire),
                };
                dot.push_str(&format!("  \"{}\" [shape=circle, style=filled, fillcolor={}, label=\"{}\"];\n",
                    wire, color, label));
            }
        }

        // Define output wires (z)
        dot.push_str("\n  // Output wires\n");
        for (wire, value) in &self.wire_values {
            if wire.starts_with('z') {
                let color = match value {
                    Some(true) => "green",
                    Some(false) => "red",
                    None => "gray",
                };
                let label = match value {
                    Some(v) => format!("{}\\n{}", wire, if *v { "1" } else { "0" }),
                    None => format!("{}\\n?", wire),
                };
                dot.push_str(&format!("  \"{}\" [shape=doublecircle, style=filled, fillcolor={}, label=\"{}\"];\n",
                    wire, color, label));
            }
        }

        // Define gates and connections
        dot.push_str("\n  // Gates\n");
        for (i, gate) in self.gates.iter().enumerate() {
            let op_name = match gate.op {
                Operation::And => "AND",
                Operation::Or => "OR",
                Operation::Xor => "XOR",
            };
            dot.push_str(&format!("  gate{} [label=\"{}\", style=filled, fillcolor=lightblue];\n",
                i, op_name));
            dot.push_str(&format!("  \"{}\" -> gate{};\n", gate.input1_name, i));
            dot.push_str(&format!("  \"{}\" -> gate{};\n", gate.input2_name, i));
            dot.push_str(&format!("  gate{} -> \"{}\";\n", i, gate.output_name));
        }

        dot.push_str("}\n");
        dot
    }

    /// Print an ASCII representation of the circuit to stdout
    pub fn print_ascii(&self) {
        println!("=== Circuit Visualization ===\n");

        // Sort gates by output name for consistent ordering
        let mut sorted_gates = self.gates.clone();
        sorted_gates.sort_by(|a, b| a.output_name.cmp(&b.output_name));

        for gate in &sorted_gates {
            let input1_val = self.wire_values.get(&gate.input1_name)
                .and_then(|v| *v)
                .map(|b| if b { "1" } else { "0" })
                .unwrap_or("?");
            let input2_val = self.wire_values.get(&gate.input2_name)
                .and_then(|v| *v)
                .map(|b| if b { "1" } else { "0" })
                .unwrap_or("?");
            let output_val = self.wire_values.get(&gate.output_name)
                .and_then(|v| *v)
                .map(|b| if b { "1" } else { "0" })
                .unwrap_or("?");

            let op_str = match gate.op {
                Operation::And => "AND",
                Operation::Or => "OR ",
                Operation::Xor => "XOR",
            };

            println!("{:>6} [{}] ─┐", gate.input1_name, input1_val);
            println!("            {} ──> {} [{}]", op_str, gate.output_name, output_val);
            println!("{:>6} [{}] ─┘", gate.input2_name, input2_val);
            println!();
        }

        // Print summary of inputs and outputs
        println!("=== Inputs ===");
        let mut inputs: Vec<_> = self.wire_values.iter()
            .filter(|(name, _)| name.starts_with('x') || name.starts_with('y'))
            .collect();
        inputs.sort_by_key(|(name, _)| *name);
        for (name, value) in inputs {
            let val_str = value.map(|b| if b { "1" } else { "0" }).unwrap_or("?");
            println!("  {}: {}", name, val_str);
        }

        println!("\n=== Outputs ===");
        let mut outputs: Vec<_> = self.wire_values.iter()
            .filter(|(name, _)| name.starts_with('z'))
            .collect();
        outputs.sort_by_key(|(name, _)| *name);
        for (name, value) in outputs {
            let val_str = value.map(|b| if b { "1" } else { "0" }).unwrap_or("?");
            println!("  {}: {}", name, val_str);
        }
    }

    /// Generate a Mermaid diagram representation of the circuit
    /// Can be viewed in GitHub, VSCode, or at https://mermaid.live
    pub fn to_mermaid(&self) -> String {
        let mut mermaid = String::from("```mermaid\ngraph LR\n");

        // Style input wires
        for (wire, value) in &self.wire_values {
            if wire.starts_with('x') || wire.starts_with('y') {
                let val_str = match value {
                    Some(v) => if *v { "1" } else { "0" },
                    None => "?",
                };
                mermaid.push_str(&format!("  {}[\"{}<br/>{}\"]:::input\n", wire, wire, val_str));
            }
        }

        // Style output wires
        for (wire, value) in &self.wire_values {
            if wire.starts_with('z') {
                let val_str = match value {
                    Some(v) => if *v { "1" } else { "0" },
                    None => "?",
                };
                mermaid.push_str(&format!("  {}[\"{}<br/>{}\"]:::output\n", wire, wire, val_str));
            }
        }

        // Add gates and connections
        for (i, gate) in self.gates.iter().enumerate() {
            let gate_id = format!("G{}", i);
            let op_label = match gate.op {
                Operation::And => "AND",
                Operation::Or => "OR",
                Operation::Xor => "XOR",
            };
            mermaid.push_str(&format!("  {}[\"{}\"]:::gate\n", gate_id, op_label));
            mermaid.push_str(&format!("  {} --> {}\n", gate.input1_name, gate_id));
            mermaid.push_str(&format!("  {} --> {}\n", gate.input2_name, gate_id));
            mermaid.push_str(&format!("  {} --> {}\n", gate_id, gate.output_name));
        }

        // Add styling
        mermaid.push_str("\n  classDef input fill:#90EE90,stroke:#333,stroke-width:2px\n");
        mermaid.push_str("  classDef output fill:#FFB6C1,stroke:#333,stroke-width:2px\n");
        mermaid.push_str("  classDef gate fill:#87CEEB,stroke:#333,stroke-width:2px\n");

        mermaid.push_str("```\n");
        mermaid
    }

    /// Helper: Find gates that use a specific wire as input
    fn find_gates_using_input(&self, wire: &str) -> Vec<&Gate> {
        self.gates
            .iter()
            .filter(|g| g.input1_name == wire || g.input2_name == wire)
            .collect()
    }

    /// Helper: Check if two wires are the inputs to a gate (order independent)
    fn find_gate_with_inputs(&self, wire1: &str, wire2: &str, op: Operation) -> Option<&Gate> {
        self.gates.iter().find(|g| {
            g.op == op
                && ((g.input1_name == wire1 && g.input2_name == wire2)
                    || (g.input1_name == wire2 && g.input2_name == wire1))
        })
    }

    /// Detect all full adder circuits in the logic
    /// Returns a vector of FullAdder structs and a set of gate indices that are part of full adders
    pub fn find_full_adders(&self) -> (Vec<FullAdder>, HashSet<usize>) {
        let mut full_adders = Vec::new();
        let mut used_gate_indices = HashSet::new();

        // Strategy: Look for the pattern of a full adder
        // Full adder has 5 gates:
        // 1. XOR1: temp_sum = x XOR y
        // 2. XOR2: sum = temp_sum XOR c_in
        // 3. AND1: temp_carry1 = x AND y
        // 4. AND2: temp_carry2 = temp_sum AND c_in
        // 5. OR: c_out = temp_carry1 OR temp_carry2

        for (_idx1, gate1) in self.gates.iter().enumerate() {
            if gate1.op != Operation::Xor {
                continue;
            }

            // This is potentially XOR1
            let temp_sum = &gate1.output_name;
            let x = &gate1.input1_name;
            let y = &gate1.input2_name;

            // Look for XOR2: uses temp_sum as one input
            let xor2_candidates = self.find_gates_using_input(temp_sum);
            for xor2 in xor2_candidates {
                if xor2.op != Operation::Xor {
                    continue;
                }

                // Identify c_in (the other input to XOR2)
                let c_in = if xor2.input1_name == *temp_sum {
                    &xor2.input2_name
                } else {
                    &xor2.input1_name
                };
                let sum = &xor2.output_name;

                // Look for AND1: x AND y
                let and1 = match self.find_gate_with_inputs(x, y, Operation::And) {
                    Some(g) => g,
                    None => continue,
                };
                let temp_carry1 = &and1.output_name;

                // Look for AND2: temp_sum AND c_in
                let and2 = match self.find_gate_with_inputs(temp_sum, c_in, Operation::And) {
                    Some(g) => g,
                    None => continue,
                };
                let temp_carry2 = &and2.output_name;

                // Look for OR: temp_carry1 OR temp_carry2
                let or_gate = match self.find_gate_with_inputs(temp_carry1, temp_carry2, Operation::Or) {
                    Some(g) => g,
                    None => continue,
                };
                let c_out = &or_gate.output_name;

                // Found a full adder! Record it and mark gates as used
                // Normalize x and y: ensure x-wire is in x field, y-wire is in y field
                let (normalized_x, normalized_y) = if x.starts_with('x') {
                    (x.clone(), y.clone())
                } else {
                    (y.clone(), x.clone())
                };

                full_adders.push(FullAdder {
                    x: normalized_x,
                    y: normalized_y,
                    c_in: c_in.clone(),
                    sum: sum.clone(),
                    c_out: c_out.clone(),
                });

                // Mark all 5 gates as used
                if let Some(idx) = self.gates.iter().position(|g| g == gate1) {
                    used_gate_indices.insert(idx);
                }
                if let Some(idx) = self.gates.iter().position(|g| g == xor2) {
                    used_gate_indices.insert(idx);
                }
                if let Some(idx) = self.gates.iter().position(|g| g == and1) {
                    used_gate_indices.insert(idx);
                }
                if let Some(idx) = self.gates.iter().position(|g| g == and2) {
                    used_gate_indices.insert(idx);
                }
                if let Some(idx) = self.gates.iter().position(|g| g == or_gate) {
                    used_gate_indices.insert(idx);
                }

                break; // Don't look for more XOR2 candidates for this XOR1
            }
        }

        (full_adders, used_gate_indices)
    }

    /// Detect all half adder circuits in the logic, excluding gates already used in full adders
    /// Returns a vector of HalfAdder structs and a set of gate indices that are part of half adders
    pub fn find_half_adders(&self, exclude_indices: &HashSet<usize>) -> (Vec<HalfAdder>, HashSet<usize>) {
        let mut half_adders = Vec::new();
        let mut used_gate_indices = HashSet::new();

        // Strategy: Look for the pattern of a half adder
        // Half adder has 2 gates:
        // 1. XOR: sum = x XOR y
        // 2. AND: carry = x AND y

        for (idx_xor, xor_gate) in self.gates.iter().enumerate() {
            if xor_gate.op != Operation::Xor {
                continue;
            }

            // Skip if this gate is already part of a full adder
            if exclude_indices.contains(&idx_xor) {
                continue;
            }

            let x = &xor_gate.input1_name;
            let y = &xor_gate.input2_name;
            let sum = &xor_gate.output_name;

            // Look for AND gate with same inputs
            let and_gate = match self.find_gate_with_inputs(x, y, Operation::And) {
                Some(g) => g,
                None => continue,
            };

            // Check if the AND gate is already used in a full adder
            let idx_and = self.gates.iter().position(|g| g == and_gate).unwrap();
            if exclude_indices.contains(&idx_and) {
                continue;
            }

            let carry = &and_gate.output_name;

            // Found a half adder!
            // Normalize x and y: ensure x-wire is in x field, y-wire is in y field
            let (normalized_x, normalized_y) = if x.starts_with('x') {
                (x.clone(), y.clone())
            } else {
                (y.clone(), x.clone())
            };

            half_adders.push(HalfAdder {
                x: normalized_x,
                y: normalized_y,
                sum: sum.clone(),
                carry: carry.clone(),
            });

            used_gate_indices.insert(idx_xor);
            used_gate_indices.insert(idx_and);
        }

        (half_adders, used_gate_indices)
    }

    /// Get all gates that are not part of any detected adder circuits
    pub fn find_unused_gates(&self, used_indices: &HashSet<usize>) -> Vec<&Gate> {
        self.gates
            .iter()
            .enumerate()
            .filter(|(idx, _)| !used_indices.contains(idx))
            .map(|(_, gate)| gate)
            .collect()
    }

    /// Get the total number of gates
    pub fn gate_count(&self) -> usize {
        self.gates.len()
    }
}
