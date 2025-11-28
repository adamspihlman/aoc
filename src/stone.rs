use std::collections::HashMap;

#[derive(Debug)]
pub struct StoneMason {
    stones: HashMap<Stone, u64>,
}

#[derive(Debug, Copy, Hash, PartialEq, Eq, Clone)]
struct Stone {
    value: u64,
}

impl From<Vec<u64>> for StoneMason {
    fn from(value: Vec<u64>) -> Self {
        let mut stones = HashMap::new();
        value.into_iter().map(Stone::from).for_each(|s| {
            stones.entry(s).and_modify(|count| *count += 1).or_insert(1);
        });
        Self { stones }
    }
}

impl From<u64> for Stone {
    fn from(value: u64) -> Self {
        Self { value }
    }
}

impl StoneMason {
    pub fn blink(&mut self, count: u32) {
        for _ in 0..count {
            let mut next_stones: HashMap<Stone, u64> = HashMap::new();
            for (stone, amount) in &self.stones {
                let mut stone = *stone;
                if let Some(new_stone) = stone.transform() {
                    next_stones
                        .entry(new_stone)
                        .and_modify(|c| *c += amount)
                        .or_insert(*amount);
                }
                next_stones
                    .entry(stone)
                    .and_modify(|c| *c += amount)
                    .or_insert(*amount);
            }
            self.stones = next_stones;
        }
    }

    pub fn get_num_stones(&self) -> u64 {
        self.stones.values().sum()
    }
}

impl Stone {
    pub fn transform(&mut self) -> Option<Stone> {
        if self.value == 0 {
            self.value = 1;
            return None;
        }

        let value_str = self.value.to_string();
        if value_str.len().is_multiple_of(2) {
            let left = value_str[0..value_str.len() / 2]
                .parse::<u64>()
                .expect("left half should be valid u64");
            let right = value_str[value_str.len() / 2..value_str.len()]
                .parse::<u64>()
                .expect("right half should be valid u64");
            self.value = left;
            return Some(Stone::from(right));
        }
        self.value *= 2024;
        None
    }
}
