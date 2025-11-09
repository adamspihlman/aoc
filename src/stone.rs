#[derive(Debug)]
pub struct StoneMason {
    stones: Vec<Stone>,
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Stone {
    value: u64,
}

impl From<Vec<u64>> for StoneMason {
    fn from(value: Vec<u64>) -> Self {
        let stones = value.into_iter().map(Stone::from).collect();
        Self { stones }
    }
}

impl From<u64> for Stone {
    fn from(value: u64) -> Self {
        Self { value }
    }
}

impl StoneMason {
    pub fn blink(&self, count: u32) -> u64 {
        self.stones.iter().map(|s| s.blink(count)).sum()
    }

    pub fn get_num_stones(&self) -> u64 {
        self.stones.len() as u64
    }
}

impl Stone {
    pub fn blink(&self, count: u32) -> u64 {
        Stone::compute(self.value, count)
    }

    fn compute(value: u64, count: u32) -> u64 {
        if count == 0 {
            return 1;
        }

        if value == 0 {
            return Stone::compute(1, count - 1);
        }

        let value_str = value.to_string();
        if value_str.len().is_multiple_of(2) {
            let left = value_str[0..value_str.len() / 2].parse::<u64>().unwrap();
            let right = value_str[value_str.len() / 2..value_str.len()]
                .parse::<u64>()
                .unwrap();
            return Stone::compute(left, count - 1) + Stone::compute(right, count - 1);
        }
        Stone::compute(value * 2024, count - 1)
    }
}
