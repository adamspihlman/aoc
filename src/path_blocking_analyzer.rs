use std::collections::VecDeque;

/// Analyzes 3x3 grid configurations to determine which ones result in
/// the middle obstacle blocking a previously existing path.
///
/// Grid positions are numbered 0-8:
/// ```
/// 0 1 2
/// 3 4 5
/// 6 7 8
/// ```
/// Position 4 (middle) is where the obstacle is added.
/// Each configuration is an 8-bit value where each bit indicates if
/// the corresponding position (excluding middle) is accessible.
pub struct PathBlockingAnalyzer {
    blocking_configs: Vec<u8>,
}

impl Default for PathBlockingAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl PathBlockingAnalyzer {
    /// Creates a new analyzer and finds all blocking configurations
    pub fn new() -> Self {
        let mut analyzer = Self {
            blocking_configs: Vec::new(),
        };
        analyzer.find_blocking_configurations();
        analyzer
    }

    /// Finds all 256 configurations and identifies which ones block paths
    fn find_blocking_configurations(&mut self) {
        for config in 0..=255u8 {
            if self.blocks_path(config) {
                self.blocking_configs.push(config);
            }
        }
    }

    /// Checks if a configuration blocks a path when obstacle is placed in middle
    fn blocks_path(&self, config: u8) -> bool {
        let accessible = self.get_accessible_positions(config);

        if accessible.len() < 2 {
            // Need at least 2 accessible cells to have a path
            return false;
        }

        // Check all pairs of accessible cells
        for i in 0..accessible.len() {
            for j in i + 1..accessible.len() {
                let pos1 = accessible[i];
                let pos2 = accessible[j];

                // Connected when middle is accessible?
                let connected_with_middle = self.are_connected(pos1, pos2, config, true);
                // Connected when middle is blocked?
                let connected_without_middle = self.are_connected(pos1, pos2, config, false);

                // If they were connected but now aren't, the middle blocks the path
                if connected_with_middle && !connected_without_middle {
                    return true;
                }
            }
        }

        false
    }

    /// Returns list of accessible positions based on configuration
    /// Maps 8 bits to positions [0,1,2,3,5,6,7,8] (skipping middle position 4)
    fn get_accessible_positions(&self, config: u8) -> Vec<usize> {
        let mut positions = Vec::new();
        let position_mapping = [0, 1, 2, 3, 5, 6, 7, 8];

        for (bit_idx, &pos) in position_mapping.iter().enumerate() {
            if (config >> bit_idx) & 1 == 1 {
                positions.push(pos);
            }
        }

        positions
    }

    /// BFS to check if two positions are connected
    fn are_connected(&self, start: usize, end: usize, config: u8, middle_accessible: bool) -> bool {
        let mut visited = [false; 9];
        let mut queue = VecDeque::new();

        queue.push_back(start);
        visited[start] = true;

        while let Some(pos) = queue.pop_front() {
            if pos == end {
                return true;
            }

            for neighbor in self.get_neighbors(pos) {
                if visited[neighbor] {
                    continue;
                }

                // Check if we can traverse this neighbor
                if neighbor == 4 {
                    // Middle position - depends on whether it's accessible
                    if !middle_accessible {
                        continue;
                    }
                } else if !self.is_accessible(neighbor, config) {
                    // Other positions - check config
                    continue;
                }

                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }

        false
    }

    /// Gets orthogonal neighbors (up, down, left, right) for a position
    fn get_neighbors(&self, pos: usize) -> Vec<usize> {
        let row = pos / 3;
        let col = pos % 3;
        let mut neighbors = Vec::new();

        // Up
        if row > 0 {
            neighbors.push((row - 1) * 3 + col);
        }
        // Down
        if row < 2 {
            neighbors.push((row + 1) * 3 + col);
        }
        // Left
        if col > 0 {
            neighbors.push(row * 3 + (col - 1));
        }
        // Right
        if col < 2 {
            neighbors.push(row * 3 + (col + 1));
        }

        neighbors
    }

    /// Checks if a position is accessible according to the configuration
    fn is_accessible(&self, pos: usize, config: u8) -> bool {
        if pos == 4 {
            // Middle position - handled separately
            return false;
        }

        let position_mapping = [0, 1, 2, 3, 5, 6, 7, 8];
        let bit_idx = position_mapping.iter().position(|&p| p == pos).unwrap();

        (config >> bit_idx) & 1 == 1
    }

    /// Returns all configurations where placing obstacle in middle blocks a path
    pub fn get_blocking_configurations(&self) -> &[u8] {
        &self.blocking_configs
    }

    /// Pretty-prints a configuration as a 3x3 grid
    pub fn print_configuration(&self, config: u8) {
        println!("Configuration: {:08b}", config);
        for row in 0..3 {
            for col in 0..3 {
                let pos = row * 3 + col;
                if pos == 4 {
                    print!("X "); // Middle obstacle
                } else if self.is_accessible(pos, config) {
                    print!(". ");
                } else {
                    print!("# ");
                }
            }
            println!();
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_blocking() {
        let analyzer = PathBlockingAnalyzer::new();

        // Configuration where diagonal corners connect only through middle
        // Grid:
        // . . #
        // # X .
        // # # .
        // Path exists: 0 -> 1 -> 4(middle) -> 5 -> 8
        // Without middle: {0,1} and {5,8} are disconnected
        // Bit mapping: pos 0->bit0, pos 1->bit1, pos 5->bit4, pos 8->bit7
        let config = 0b10010011; // pos 0, 1, 5, 8 accessible
        assert!(analyzer.blocks_path(config));
    }

    #[test]
    fn test_non_blocking() {
        let analyzer = PathBlockingAnalyzer::new();

        // Only one accessible cell - can't block a path
        let config = 0b00000001;
        assert!(!analyzer.blocks_path(config));
    }

    #[test]
    fn test_alternative_path() {
        let analyzer = PathBlockingAnalyzer::new();

        // Configuration where cells can connect around the middle
        // Grid:
        // . . .
        // # X #
        // # # #
        // Top cells can connect without going through middle
        let config = 0b00000111; // pos 0, 1, 2 accessible
        assert!(!analyzer.blocks_path(config));
    }

    #[test]
    fn test_middle_blocks_left_right_path() {
        let analyzer = PathBlockingAnalyzer::new();

        // Configuration where left and right are only accessible through middle
        // Grid:
        // # # #
        // . X .
        // # # #
        // Left (pos 3) and right (pos 5) can only connect through middle (pos 4)
        // Bit mapping: pos 3 -> bit 3, pos 5 -> bit 4
        let config = 0b00011000; // pos 3 and 5 accessible (bits 3 and 4 set)
        assert!(analyzer.blocks_path(config),
            "Middle obstacle should block path from left to right");
    }
}
