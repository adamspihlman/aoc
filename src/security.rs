use std::collections::{HashMap, VecDeque};

use crate::grid;

#[derive(Debug)]
pub struct Security {
    robots: Vec<Robot>,
    num_rows: usize,
    num_cols: usize,
    time: usize,
}

#[derive(Debug, Default)]
pub struct SecurityBuilder {
    robots: Vec<Robot>,
    num_rows: usize,
    num_cols: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Robot {
    pos: grid::Location,
    v: Velocity,
}

#[derive(Debug, Clone, Copy)]
pub struct Velocity {
    pub col_v: i32,
    pub row_v: i32,
}

impl From<(grid::Location, Velocity)> for Robot {
    fn from(value: (grid::Location, Velocity)) -> Self {
        let (pos, v) = value;
        Self { pos, v }
    }
}

impl Security {
    pub fn elapse(&mut self, time: usize) {
        self.time += time;

        self.robots
            .iter_mut()
            .for_each(|r| r.move_location(self.num_rows, self.num_cols, time));
    }

    pub fn get_time(&self) -> usize {
        self.time
    }

    pub fn safety_factor(&self) -> u64 {
        let mut quadrant_counts: HashMap<i32, u64> =
            HashMap::from([(1, 0), (2, 0), (3, 0), (4, 0)]);

        self.robots.iter().for_each(|r| {
            if let Some(quadrant) = r.quadrant(self.num_rows, self.num_cols) {
                *quadrant_counts.entry(quadrant).or_insert(0) += 1;
            }
        });

        quadrant_counts.values().product::<u64>()
    }

    pub fn draw(&mut self) {
        loop {
            self.elapse(1);
            let mut map: Vec<Vec<char>> = vec![vec!['.'; 101]; 103];
            self.robots.iter().for_each(|r| {
                let location = r.get_pos();
                map[location.row][location.col] = 'R';
            });
            let metrics = analyze_grid(&map);
            let score = metrics.pattern_score();
            if score > 20.0 {
                println!("t = {} seconds", self.time);
                println!("score = {score}");
                for row in map.iter() {
                    let cols: String = row.iter().collect();
                    println!("{cols}");
                }
                break;
            }
        }
    }
}

impl Robot {
    pub fn move_location(&mut self, num_rows: usize, num_cols: usize, time: usize) {
        self.pos.row = Robot::get_final_dimension(self.pos.row, num_rows, self.v.row_v, time);
        self.pos.col = Robot::get_final_dimension(self.pos.col, num_cols, self.v.col_v, time);
    }

    pub fn quadrant(&self, num_rows: usize, num_cols: usize) -> Option<i32> {
        let mid_row = num_rows / 2;
        let mid_col = num_cols / 2;

        match (self.pos.row.cmp(&mid_row), self.pos.col.cmp(&mid_col)) {
            (_, std::cmp::Ordering::Equal) => None,
            (std::cmp::Ordering::Equal, _) => None,
            (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => Some(1),
            (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => Some(2),
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => Some(3),
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => Some(4),
        }
    }

    pub fn get_pos(&self) -> grid::Location {
        grid::Location {
            row: self.pos.row,
            col: self.pos.col,
        }
    }

    fn get_final_dimension(
        initial: usize,
        dimension_size: usize,
        velocity: i32,
        time: usize,
    ) -> usize {
        let offset = (time as i32 * velocity).unsigned_abs() as usize % dimension_size;
        if velocity < 0 {
            return initial + if offset > initial { dimension_size } else { 0 } - offset;
        }
        initial + offset
            - if initial + offset >= dimension_size {
                dimension_size
            } else {
                0
            }
    }
}

impl SecurityBuilder {
    pub fn robots(mut self, robots: Vec<Robot>) -> Self {
        self.robots = robots;
        self
    }

    pub fn num_rows(mut self, num_rows: usize) -> Self {
        self.num_rows = num_rows;
        self
    }

    pub fn num_cols(mut self, num_cols: usize) -> Self {
        self.num_cols = num_cols;
        self
    }

    pub fn build(&self) -> Security {
        Security {
            robots: self.robots.clone(),
            num_rows: self.num_rows,
            num_cols: self.num_cols,
            time: 0,
        }
    }
}

// thanks Claude //

// Main structure to hold all heuristic results
#[derive(Debug)]
struct PatternMetrics {
    bounding_box_density: f64,
    largest_cluster_size: usize,
    cluster_count: usize,
    row_variance: f64,
    col_variance: f64,
    total_r_count: usize,
}

impl PatternMetrics {
    // Simple scoring function - higher score = more likely to be a pattern
    fn pattern_score(&self) -> f64 {
        let mut score = 0.0;

        // High bounding box density is good
        score += self.bounding_box_density * 100.0;

        // Large clusters are good
        if self.total_r_count > 0 {
            let cluster_ratio = self.largest_cluster_size as f64 / self.total_r_count as f64;
            score += cluster_ratio * 50.0;
        }

        // Fewer clusters is better (more organized)
        if self.cluster_count > 0 {
            score += (1.0 / self.cluster_count as f64) * 20.0;
        }

        // High variance in rows/cols suggests structure
        score += self.row_variance.min(10.0) * 2.0;
        score += self.col_variance.min(10.0) * 2.0;

        score
    }
}

fn analyze_grid(grid: &[Vec<char>]) -> PatternMetrics {
    // Count total 'R's
    let total_r_count: usize = grid
        .iter()
        .map(|row| row.iter().filter(|&&c| c == 'R').count())
        .sum();

    // Find bounding box
    let bounding_box_density = calculate_bounding_box_density(grid);

    // Find clusters using flood fill
    let (cluster_count, largest_cluster_size) = count_clusters(grid);

    // Calculate row and column variances
    let row_variance = calculate_row_variance(grid);
    let col_variance = calculate_col_variance(grid);

    PatternMetrics {
        bounding_box_density,
        largest_cluster_size,
        cluster_count,
        row_variance,
        col_variance,
        total_r_count,
    }
}

fn calculate_bounding_box_density(grid: &[Vec<char>]) -> f64 {
    let height = grid.len();
    let width = if height > 0 { grid[0].len() } else { 0 };

    let mut min_row = height;
    let mut max_row = 0;
    let mut min_col = width;
    let mut max_col = 0;
    let mut r_count = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            if ch == 'R' {
                min_row = min_row.min(i);
                max_row = max_row.max(i);
                min_col = min_col.min(j);
                max_col = max_col.max(j);
                r_count += 1;
            }
        }
    }

    if r_count == 0 {
        return 0.0;
    }

    let bbox_height = max_row - min_row + 1;
    let bbox_width = max_col - min_col + 1;
    let bbox_area = bbox_height * bbox_width;

    r_count as f64 / bbox_area as f64
}

fn count_clusters(grid: &[Vec<char>]) -> (usize, usize) {
    let height = grid.len();
    let width = if height > 0 { grid[0].len() } else { 0 };

    let mut visited = vec![vec![false; width]; height];
    let mut cluster_count = 0;
    let mut largest_cluster = 0;

    for i in 0..height {
        for j in 0..width {
            if grid[i][j] == 'R' && !visited[i][j] {
                let cluster_size = bfs_cluster(grid, &mut visited, i, j);
                cluster_count += 1;
                largest_cluster = largest_cluster.max(cluster_size);
            }
        }
    }

    (cluster_count, largest_cluster)
}

fn bfs_cluster(
    grid: &[Vec<char>],
    visited: &mut [Vec<bool>],
    start_i: usize,
    start_j: usize,
) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    let mut queue = VecDeque::new();
    let mut size = 0;

    queue.push_back((start_i, start_j));
    visited[start_i][start_j] = true;

    while let Some((i, j)) = queue.pop_front() {
        size += 1;

        // Check 4 adjacent cells
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (di, dj) in directions.iter() {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;

            if ni >= 0 && ni < height as i32 && nj >= 0 && nj < width as i32 {
                let ni = ni as usize;
                let nj = nj as usize;

                if grid[ni][nj] == 'R' && !visited[ni][nj] {
                    visited[ni][nj] = true;
                    queue.push_back((ni, nj));
                }
            }
        }
    }

    size
}

fn calculate_row_variance(grid: &[Vec<char>]) -> f64 {
    let counts: Vec<usize> = grid
        .iter()
        .map(|row| row.iter().filter(|&&c| c == 'R').count())
        .collect();

    calculate_variance(&counts)
}

fn calculate_col_variance(grid: &[Vec<char>]) -> f64 {
    let height = grid.len();
    let width = if height > 0 { grid[0].len() } else { 0 };

    let mut counts = vec![0; width];
    for row in grid.iter() {
        for (j, &ch) in row.iter().enumerate() {
            if ch == 'R' {
                counts[j] += 1;
            }
        }
    }

    calculate_variance(&counts)
}

fn calculate_variance(data: &[usize]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    let mean = data.iter().sum::<usize>() as f64 / data.len() as f64;
    let variance = data
        .iter()
        .map(|&x| {
            let diff = x as f64 - mean;
            diff * diff
        })
        .sum::<f64>()
        / data.len() as f64;

    variance.sqrt() // Return standard deviation
}
