use crate::grid::Location;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Keypad {
    numeric: Vec<Vec<char>>,
    directional: Vec<Vec<char>>,
    numeric_map: HashMap<char, Location>,
    directional_map: HashMap<char, Location>,
}

impl Keypad {
    pub fn new() -> Self {
        let numeric = vec![
            vec!['7', '8', '9'],
            vec!['4', '5', '6'],
            vec!['1', '2', '3'],
            vec!['#', '0', 'A'],
        ];

        let directional = vec![
            vec!['#', '^', 'A'],
            vec!['<', 'v', '>'],
        ];

        let mut numeric_map = HashMap::new();
        for (row, row_chars) in numeric.iter().enumerate() {
            for (col, &ch) in row_chars.iter().enumerate() {
                numeric_map.insert(ch, Location { row, col });
            }
        }

        let mut directional_map = HashMap::new();
        for (row, row_chars) in directional.iter().enumerate() {
            for (col, &ch) in row_chars.iter().enumerate() {
                directional_map.insert(ch, Location { row, col });
            }
        }

        Self {
            numeric,
            directional,
            numeric_map,
            directional_map,
        }
    }
}
