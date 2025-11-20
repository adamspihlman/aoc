#[derive(Debug)]
pub struct Lock {
    heights: Vec<usize>,
}

#[derive(Debug)]
pub struct Key {
    heights: Vec<usize>,
}

impl From<Vec<usize>> for Lock {
    fn from(heights: Vec<usize>) -> Self {
        Self { heights }
    }
}

impl From<Vec<usize>> for Key {
    fn from(heights: Vec<usize>) -> Self {
        Self { heights }
    }
}

impl Lock {
    pub fn fits(&self, key: &Key) -> bool {
        for (idx, height) in self.heights.iter().enumerate() {
            if key.heights[idx] + height >= 6 {
                return false;
            }
        }
        true
    }
}
