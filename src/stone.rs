#[derive(Debug)]
pub struct StoneMason {
    stones: Vec<Stone>,
}

#[derive(Debug)]
struct Stone {
    pub value: u64,
}

impl StoneMason {
    pub fn from(input: Vec<u64>) -> StoneMason {
        let stones = input.iter().map(|&i| Stone { value: i }).collect();
        StoneMason { stones }
    }

    pub fn blink(&mut self, count: u32) {
        for _ in 0..count {
            let mut new_stones = Vec::new();
            for stone in &mut self.stones {
                if let Some(new_stone) = stone.transform() {
                    new_stones.push(new_stone);
                }
            }
            self.stones.extend(new_stones);
        }
    }

    pub fn get_num_stones(&self) -> u64 {
        self.stones.len() as u64
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
            let left = value_str[0..value_str.len() / 2].parse::<u64>().unwrap();
            let right = value_str[value_str.len() / 2..value_str.len()]
                .parse::<u64>()
                .unwrap();
            self.value = left;
            return Some(Stone { value: right });
        }
        self.value *= 2024;
        None
    }
}
