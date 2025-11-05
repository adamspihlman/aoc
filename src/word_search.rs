pub struct Words {
    words: Vec<Vec<char>>,
}

impl Words {
    fn count_right(&self, row: usize, col: usize) -> u64 {
        let row_data = &self.words[row];
        if col + 3 >= row_data.len() {
            return 0;
        }
        match (row_data[col + 1], row_data[col + 2], row_data[col + 3]) {
            ('M', 'A', 'S') => 1,
            _ => 0,
        }
    }
    fn count_left(&self, row: usize, col: usize) -> u64 {
        let row_data = &self.words[row];
        if col < 3 {
            return 0;
        }
        match (row_data[col - 1], row_data[col - 2], row_data[col - 3]) {
            ('M', 'A', 'S') => 1,
            _ => 0,
        }
    }

    fn count_down(&self, row: usize, col: usize) -> u64 {
        if row + 3 >= self.words[row].len() {
            return 0;
        }
        match (
            &self.words[row + 1][col],
            &self.words[row + 2][col],
            &self.words[row + 3][col],
        ) {
            ('M', 'A', 'S') => 1,
            _ => 0,
        }
    }

    fn count_up(&self, row: usize, col: usize) -> u64 {
        if row < 3 {
            return 0;
        }
        match (
            &self.words[row - 1][col],
            &self.words[row - 2][col],
            &self.words[row - 3][col],
        ) {
            ('M', 'A', 'S') => 1,
            _ => 0,
        }
    }

    fn count_up_right(&self, row: usize, col: usize) -> u64 {
        if row < 3 || col + 3 >= self.words[0].len() {
            return 0;
        }
        match (
            &self.words[row - 1][col + 1],
            &self.words[row - 2][col + 2],
            &self.words[row - 3][col + 3],
        ) {
            ('M', 'A', 'S') => 1,
            _ => 0,
        }
    }

    fn count_up_left(&self, row: usize, col: usize) -> u64 {
        if row < 3 || col < 3 {
            return 0;
        }
        match (
            &self.words[row - 1][col - 1],
            &self.words[row - 2][col - 2],
            &self.words[row - 3][col - 3],
        ) {
            ('M', 'A', 'S') => 1,
            _ => 0,
        }
    }

    fn count_down_left(&self, row: usize, col: usize) -> u64 {
        if row + 3 >= self.words.len() || col < 3 {
            return 0;
        }
        match (
            &self.words[row + 1][col - 1],
            &self.words[row + 2][col - 2],
            &self.words[row + 3][col - 3],
        ) {
            ('M', 'A', 'S') => 1,
            _ => 0,
        }
    }

    fn count_down_right(&self, row: usize, col: usize) -> u64 {
        if row + 3 >= self.words.len() || col + 3 >= self.words[0].len() {
            return 0;
        }
        match (
            &self.words[row + 1][col + 1],
            &self.words[row + 2][col + 2],
            &self.words[row + 3][col + 3],
        ) {
            ('M', 'A', 'S') => 1,
            _ => 0,
        }
    }

    fn count_xmas(&self, row: usize, col: usize) -> u64 {
        self.count_right(row, col)
            + self.count_left(row, col)
            + self.count_down(row, col)
            + self.count_up(row, col)
            + self.count_up_right(row, col)
            + self.count_up_left(row, col)
            + self.count_down_left(row, col)
            + self.count_down_right(row, col)
    }

    pub fn word_search(&self) -> u64 {
        let mut count = 0;

        for (row_idx, row_val) in self.words.iter().enumerate() {
            for (col_idx, col_val) in row_val.iter().enumerate() {
                if col_val == &'X' {
                    count += self.count_xmas(row_idx, col_idx);
                }
            }
        }
        count
    }
}

pub fn build_words(words: Vec<Vec<char>>) -> Words {
    Words { words }
}
