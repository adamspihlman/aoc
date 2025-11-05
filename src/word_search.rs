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

    fn count_xmas(&self, row: usize, col: usize) -> u64 {
        self.count_right(row, col)
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
