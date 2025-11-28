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

        let directional = [vec!['#', '^', 'A'], vec!['<', 'v', '>']];

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

    pub fn complexity(
        &self,
        code: &str,
        depth: usize,
        cache: &mut HashMap<String, HashMap<usize, u64>>,
    ) -> u64 {
        let sequence_length = self.get_sequence_length(code, depth, cache);
        let numeric_part = self.get_numeric_part(code);

        sequence_length * numeric_part
    }

    fn get_sequence_length(
        &self,
        code: &str,
        depth: usize,
        cache: &mut HashMap<String, HashMap<usize, u64>>,
    ) -> u64 {
        let numpad_result = self.numpad_to_directional(code);

        if depth == 0 {
            numpad_result.len() as u64
        } else {
            self.directional_to_directional(&numpad_result, depth, cache)
        }
    }

    fn get_numeric_part(&self, code: &str) -> u64 {
        code[..code.len() - 1]
            .parse::<u64>()
            .expect("code prefix should be numeric")
    }

    fn directional_to_directional(
        &self,
        code: &str,
        depth: usize,
        cache: &mut HashMap<String, HashMap<usize, u64>>,
    ) -> u64 {
        if let Some(depth_map) = cache.get(code) {
            if let Some(&length) = depth_map.get(&depth) {
                return length;
            }
        }

        let mut result = String::new();
        let mut current_pos = *self
            .directional_map
            .get(&'A')
            .expect("'A' should be in directional map");

        for ch in code.chars() {
            let target_pos = *self
                .directional_map
                .get(&ch)
                .expect("character should be in directional map");
            result.push_str(&self.get_directional_moves(current_pos, target_pos));
            result.push('A');
            current_pos = target_pos;
        }

        let total_length = if depth == 1 {
            result.len() as u64
        } else {
            let mut length = 0;
            let mut segment = String::new();

            for ch in result.chars() {
                segment.push(ch);
                if ch == 'A' {
                    length += self.directional_to_directional(&segment, depth - 1, cache);
                    segment.clear();
                }
            }

            length
        };

        cache
            .entry(code.to_string())
            .or_default()
            .insert(depth, total_length);

        total_length
    }

    fn get_directional_moves(&self, current: Location, target: Location) -> String {
        let mut moves = String::new();
        let row_diff = target.row as isize - current.row as isize;
        let col_diff = target.col as isize - current.col as isize;

        if row_diff < 0 && col_diff > 0 {
            if current.col == 0 && target.row == 0 {
                for _ in 0..col_diff {
                    moves.push('>');
                }
                for _ in 0..(-row_diff) {
                    moves.push('^');
                }
            } else {
                for _ in 0..(-row_diff) {
                    moves.push('^');
                }
                for _ in 0..col_diff {
                    moves.push('>');
                }
            }
        } else if row_diff > 0 && col_diff < 0 {
            if current.row == 0 && target.col == 0 {
                for _ in 0..row_diff {
                    moves.push('v');
                }
                for _ in 0..(-col_diff) {
                    moves.push('<');
                }
            } else {
                for _ in 0..(-col_diff) {
                    moves.push('<');
                }
                for _ in 0..row_diff {
                    moves.push('v');
                }
            }
        } else if row_diff < 0 && col_diff < 0 {
            for _ in 0..(-col_diff) {
                moves.push('<');
            }
            for _ in 0..(-row_diff) {
                moves.push('^');
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
        let mut current_pos = *self
            .numeric_map
            .get(&'A')
            .expect("'A' should be in numeric map");

        for ch in code.chars() {
            let target_pos = *self
                .numeric_map
                .get(&ch)
                .expect("character should be in numeric map");
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
            if current.col == 0 && target.row == 3 {
                for _ in 0..col_diff {
                    moves.push('>');
                }
                for _ in 0..row_diff {
                    moves.push('v');
                }
            } else {
                for _ in 0..row_diff {
                    moves.push('v');
                }
                for _ in 0..col_diff {
                    moves.push('>');
                }
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
        let mut cache = HashMap::new();
        let result = keypad.directional_to_directional("<A^A^^>AvvvA", 1, &mut cache);
        assert_eq!(result, 28);
    }

    #[test]
    fn test_sequence_lengths() {
        let keypad = Keypad::new();
        let mut cache = HashMap::new();

        assert_eq!(keypad.get_sequence_length("029A", 2, &mut cache), 68);
        assert_eq!(keypad.get_sequence_length("980A", 2, &mut cache), 60);
        assert_eq!(keypad.get_sequence_length("179A", 2, &mut cache), 68);
        assert_eq!(keypad.get_sequence_length("456A", 2, &mut cache), 64);
        assert_eq!(keypad.get_sequence_length("379A", 2, &mut cache), 64);
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
