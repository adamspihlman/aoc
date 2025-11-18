use crate::grid::Location;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Keypad {
    numeric_map: HashMap<char, Location>,
    directional_map: HashMap<char, Location>,
}

impl Default for Keypad {
    fn default() -> Self {
        Self::new()
    }
}

impl Keypad {
    pub fn new() -> Self {
        let numeric = [
            vec!['7', '8', '9'],
            vec!['4', '5', '6'],
            vec!['1', '2', '3'],
            vec!['#', '0', 'A'],
        ];

        let directional = [
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
            numeric_map,
            directional_map,
        }
    }

    pub fn complexity(&self, code: &str) -> u64 {
        let sequence_length = self.get_sequence_length(code);
        let numeric_part = self.get_numeric_part(code);

        sequence_length * numeric_part
    }

    fn get_sequence_length(&self, code: &str) -> u64 {
        let numpad_moves = self.numpad_to_directional(code);
        let first_directional_moves = self.directional_to_directional(&numpad_moves);
        let second_directional_moves = self.directional_to_directional(&first_directional_moves);

        second_directional_moves.len() as u64
    }

    fn get_numeric_part(&self, code: &str) -> u64 {
        code[..code.len() - 1].parse::<u64>().unwrap()
    }

    fn directional_to_directional(&self, code: &str) -> String {
        let mut result = String::new();
        let mut current_pos = *self.directional_map.get(&'A').unwrap();

        for ch in code.chars() {
            let target_pos = *self.directional_map.get(&ch).unwrap();
            result.push_str(&self.get_directional_moves(current_pos, target_pos));
            result.push('A');
            current_pos = target_pos;
        }

        result
    }

    fn get_directional_moves(&self, current: Location, target: Location) -> String {
        let mut moves = String::new();
        let row_diff = target.row as isize - current.row as isize;
        let col_diff = target.col as isize - current.col as isize;

        if row_diff < 0 && col_diff > 0 {
            for _ in 0..col_diff {
                moves.push('>');
            }
            for _ in 0..(-row_diff) {
                moves.push('^');
            }
        } else if row_diff > 0 && col_diff < 0 {
            for _ in 0..row_diff {
                moves.push('v');
            }
            for _ in 0..(-col_diff) {
                moves.push('<');
            }
        } else {
            if row_diff < 0 {
                for _ in 0..(-row_diff) {
                    moves.push('^');
                }
            } else if row_diff > 0 {
                for _ in 0..row_diff {
                    moves.push('v');
                }
            }

            if col_diff < 0 {
                for _ in 0..(-col_diff) {
                    moves.push('<');
                }
            } else if col_diff > 0 {
                for _ in 0..col_diff {
                    moves.push('>');
                }
            }
        }

        moves
    }

    fn numpad_to_directional(&self, code: &str) -> String {
        let mut result = String::new();
        let mut current_pos = *self.numeric_map.get(&'A').unwrap();

        for ch in code.chars() {
            let target_pos = *self.numeric_map.get(&ch).unwrap();
            result.push_str(&self.get_numpad_moves(current_pos, target_pos));
            result.push('A');
            current_pos = target_pos;
        }

        result
    }

    fn get_numpad_moves(&self, current: Location, target: Location) -> String {
        let mut moves = String::new();
        let row_diff = target.row as isize - current.row as isize;
        let col_diff = target.col as isize - current.col as isize;

        if row_diff < 0 && col_diff < 0 {
            if current.row == 3 && target.col == 0 {
                for _ in 0..(-row_diff) {
                    moves.push('^');
                }
                for _ in 0..(-col_diff) {
                    moves.push('<');
                }
            } else {
                for _ in 0..(-col_diff) {
                    moves.push('<');
                }
                for _ in 0..(-row_diff) {
                    moves.push('^');
                }
            }
        } else if row_diff > 0 && col_diff > 0 {
            for _ in 0..col_diff {
                moves.push('>');
            }
            for _ in 0..row_diff {
                moves.push('v');
            }
        } else {
            if row_diff < 0 {
                for _ in 0..(-row_diff) {
                    moves.push('^');
                }
            } else if row_diff > 0 {
                for _ in 0..row_diff {
                    moves.push('v');
                }
            }

            if col_diff < 0 {
                for _ in 0..(-col_diff) {
                    moves.push('<');
                }
            } else if col_diff > 0 {
                for _ in 0..col_diff {
                    moves.push('>');
                }
            }
        }

        moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numpad_to_directional() {
        let keypad = Keypad::new();
        let result = keypad.numpad_to_directional("029A");
        assert_eq!(result, "<A^A^^>AvvvA");
    }

    #[test]
    fn test_numpad_to_directional_980a() {
        let keypad = Keypad::new();
        let result = keypad.numpad_to_directional("980A");
        assert_eq!(result, "^^^A<AvvvA>A");
    }

    #[test]
    fn test_numpad_to_directional_179a() {
        let keypad = Keypad::new();
        let result = keypad.numpad_to_directional("179A");
        assert_eq!(result, "^<<A^^A>>AvvvA");
    }

    #[test]
    fn test_numpad_to_directional_456a() {
        let keypad = Keypad::new();
        let result = keypad.numpad_to_directional("456A");
        assert_eq!(result, "^^<<A>A>AvvA");
    }

    #[test]
    fn test_numpad_to_directional_379a() {
        let keypad = Keypad::new();
        let result = keypad.numpad_to_directional("379A");
        assert_eq!(result, "^A<<^^A>>AvvvA");
    }

    #[test]
    fn test_numpad_to_directional_101a() {
        let keypad = Keypad::new();
        let result = keypad.numpad_to_directional("101A");
        assert_eq!(result, "^<<A>vA^<A>>vA");
    }

    #[test]
    fn test_directional_to_directional_029a() {
        let keypad = Keypad::new();
        let result = keypad.directional_to_directional("<A^A^^>AvvvA");
        assert_eq!(result, "v<<A>>^A<A>A<AAv>A^Av<AAA>^A");
    }

    #[test]
    fn test_sequence_lengths() {
        let keypad = Keypad::new();

        let codes = vec!["029A", "980A", "179A", "456A", "379A"];

        for code in codes {
            let result = keypad.get_sequence_length(code);
            println!("{}: {}", code, result);
        }
    }

    #[test]
    fn test_get_numeric_part() {
        let keypad = Keypad::new();

        assert_eq!(keypad.get_numeric_part("029A"), 29);
        assert_eq!(keypad.get_numeric_part("980A"), 980);
        assert_eq!(keypad.get_numeric_part("179A"), 179);
        assert_eq!(keypad.get_numeric_part("456A"), 456);
        assert_eq!(keypad.get_numeric_part("379A"), 379);
    }

}
