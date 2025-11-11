use std::collections::VecDeque;

// thanks Claude

// Main structure to hold all heuristic results
#[derive(Debug)]
pub struct PatternMetrics {
    bounding_box_density: f64,
    largest_cluster_size: usize,
    cluster_count: usize,
    row_variance: f64,
    col_variance: f64,
    total_r_count: usize,
}

impl From<&Vec<Vec<char>>> for PatternMetrics {
    fn from(value: &Vec<Vec<char>>) -> Self {
        // Count total 'R's
        let total_r_count: usize = value
            .iter()
            .map(|row| row.iter().filter(|&&c| c == 'R').count())
            .sum();

        // Find bounding box
        let bounding_box_density = calculate_bounding_box_density(value);

        // Find clusters using flood fill
        let (cluster_count, largest_cluster_size) = count_clusters(value);

        // Calculate row and column variances
        let row_variance = calculate_row_variance(value);
        let col_variance = calculate_col_variance(value);

        PatternMetrics {
            bounding_box_density,
            largest_cluster_size,
            cluster_count,
            row_variance,
            col_variance,
            total_r_count,
        }
    }
}

impl PatternMetrics {
    // Simple scoring function - higher score = more likely to be a pattern
    pub fn pattern_score(&self) -> f64 {
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
