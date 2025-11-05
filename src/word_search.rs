pub struct Words {
    words: Vec<Vec<char>>,
}

impl Words {
    fn count_dir(&self, row: isize, col: isize, dr: isize, dc: isize) -> bool {
        let rows = self.words.len() as isize;
        let cols = self.words[0].len() as isize;

        if (0..4).any(|i| {
            let r = row + dr * i;
            let c = col + dc * i;
            r < 0 || c < 0 || r >= rows || c >= cols
        }) {
            return false;
        }

        let seq: Vec<char> = (0..4)
            .map(|i| self.words[(row + dr * i) as usize][(col + dc * i) as usize])
            .collect();

        seq == ['X', 'M', 'A', 'S']
    }

    fn count_xs(&self, row: isize, col: isize, slope: isize) -> bool {
        let rows = self.words.len() as isize;
        let cols = self.words[0].len() as isize;

        if (-1..=1).any(|i| {
            let r = row + slope * i;
            let c = col + i;
            r < 0 || c < 0 || r >= rows || c >= cols
        }) {
            return false;
        }

        let seq: Vec<char> = (-1..=1)
            .map(|i| self.words[(row + slope * i) as usize][(col + i) as usize])
            .collect();

        seq == ['M', 'A', 'S'] || seq == ['S', 'A', 'M']
    }

    fn count_xmas(&self, row: usize, col: usize) -> u64 {
        let dirs = [
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];

        dirs.iter()
            .filter(|(dr, dc)| self.count_dir(row as isize, col as isize, *dr, *dc))
            .count() as u64
    }

    fn check_x_shape(&self, row: usize, col: usize) -> bool {
        let slopes = [1, -1];

        slopes
            .into_iter()
            .filter(|&slope| self.count_xs(row as isize, col as isize, slope))
            .count()
            == 2
    }

    pub fn word_search(&self) -> u64 {
        let mut count = 0;
        for (r, row) in self.words.iter().enumerate() {
            for (c, &ch) in row.iter().enumerate() {
                if ch == 'X' {
                    count += self.count_xmas(r, c);
                }
            }
        }
        count
    }
    pub fn x_search(&self) -> u64 {
        let mut count = 0;
        for (r, row) in self.words.iter().enumerate() {
            for (c, &ch) in row.iter().enumerate() {
                if ch == 'A' && self.check_x_shape(r, c) {
                    count += 1;
                }
            }
        }
        count
    }
}

pub fn build_words(words: Vec<Vec<char>>) -> Words {
    Words { words }
}
