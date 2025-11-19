#[derive(Debug, Default, Clone)]
pub struct Secret {
    value: u64,
    price: u64,
    change: Option<i32>,
}

pub fn generate(secret: u64, iterations: usize) -> u64 {
    if iterations == 0 {
        return secret;
    }

    let mult = mix_and_prune(secret, secret * 64);
    let div = mix_and_prune(mult, mult / 32);
    let result = mix_and_prune(div, div * 2048);

    generate(result, iterations - 1)
}

fn mix_and_prune(secret: u64, value: u64) -> u64 {
    (secret ^ value) % 16777216
}

impl From<u64> for Secret {
    fn from(value: u64) -> Self {
        Self::new(value, None)
    }
}

impl Secret {
    pub fn new(value: u64, change: Option<i32>) -> Self {
        let price = Secret::price(value);
        Self {
            value,
            price,
            change,
        }
    }

    pub fn generate(&self) -> Self {
        let value = generate(self.value, 1);
        let price = Secret::price(value);
        let change = Some(price as i32 - self.price as i32);
        Self {
            value,
            price,
            change,
        }
    }

    pub fn get_change(&self) -> Option<i32> {
        self.change
    }

    pub fn get_price(&self) -> u64 {
        self.price
    }

    fn price(value: u64) -> u64 {
        value % 10
    }

    pub fn generate_sequence_map(
        &self,
        iterations: usize,
    ) -> std::collections::HashMap<Vec<i32>, u64> {
        use std::collections::{HashMap, VecDeque};

        let mut map = HashMap::new();
        let mut current = self.clone();
        let mut change_window: VecDeque<i32> = VecDeque::with_capacity(4);

        for iteration in 0..iterations {
            if let Some(change) = current.get_change() {
                change_window.push_back(change);

                if change_window.len() > 4 {
                    change_window.pop_front();
                }

                if change_window.len() == 4 {
                    let sequence: Vec<i32> = change_window.iter().copied().collect();
                    map.entry(sequence).or_insert(current.get_price());
                }
            }

            if iteration < iterations - 1 {
                current = current.generate();
            }
        }

        map
    }
}

pub fn combine_sequence_maps(
    map1: std::collections::HashMap<Vec<i32>, u64>,
    map2: std::collections::HashMap<Vec<i32>, u64>,
) -> std::collections::HashMap<Vec<i32>, u64> {
    let mut result = map1.clone();

    for (sequence, price) in map2 {
        result
            .entry(sequence)
            .and_modify(|existing_price| *existing_price += price)
            .or_insert(price);
    }

    result
}

pub fn max_price(map: &std::collections::HashMap<Vec<i32>, u64>) -> Option<u64> {
    map.values().max().copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_generation() {
        let mut secret = Secret::from(123);
        assert_eq!(secret.value, 123);
        assert_eq!(secret.price, 3);
        assert_eq!(secret.change, None);

        secret = secret.generate();
        assert_eq!(secret.value, 15887950);
        assert_eq!(secret.price, 0);
        assert_eq!(secret.change, Some(-3));

        secret = secret.generate();
        assert_eq!(secret.value, 16495136);
        assert_eq!(secret.price, 6);
        assert_eq!(secret.change, Some(6));

        secret = secret.generate();
        assert_eq!(secret.value, 527345);
        assert_eq!(secret.price, 5);
        assert_eq!(secret.change, Some(-1));

        secret = secret.generate();
        assert_eq!(secret.value, 704524);
        assert_eq!(secret.price, 4);
        assert_eq!(secret.change, Some(-1));

        secret = secret.generate();
        assert_eq!(secret.value, 1553684);
        assert_eq!(secret.price, 4);
        assert_eq!(secret.change, Some(0));

        secret = secret.generate();
        assert_eq!(secret.value, 12683156);
        assert_eq!(secret.price, 6);
        assert_eq!(secret.change, Some(2));
    }

    #[test]
    fn test_generate_sequence_map() {
        let secret = Secret::from(123);
        let map = secret.generate_sequence_map(7);

        assert_eq!(map.get(&vec![-3, 6, -1, -1]), Some(&4));
        assert_eq!(map.get(&vec![6, -1, -1, 0]), Some(&4));
        assert_eq!(map.get(&vec![-1, -1, 0, 2]), Some(&6));
        assert_eq!(map.len(), 3);
    }

    #[test]
    fn test_combine_sequence_maps() {
        use std::collections::HashMap;

        let mut map1 = HashMap::new();
        map1.insert(vec![1, 2, 3, 4], 10);
        map1.insert(vec![5, 6, 7, 8], 20);
        map1.insert(vec![9, 10, 11, 12], 30);

        let mut map2 = HashMap::new();
        map2.insert(vec![1, 2, 3, 4], 15);
        map2.insert(vec![5, 6, 7, 8], 5);
        map2.insert(vec![13, 14, 15, 16], 40);

        let combined = combine_sequence_maps(map1, map2);

        assert_eq!(combined.get(&vec![1, 2, 3, 4]), Some(&25));
        assert_eq!(combined.get(&vec![5, 6, 7, 8]), Some(&25));
        assert_eq!(combined.get(&vec![9, 10, 11, 12]), Some(&30));
        assert_eq!(combined.get(&vec![13, 14, 15, 16]), Some(&40));
        assert_eq!(combined.len(), 4);
    }

    #[test]
    fn test_max_price() {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        map.insert(vec![1, 2, 3, 4], 10);
        map.insert(vec![5, 6, 7, 8], 50);
        map.insert(vec![9, 10, 11, 12], 30);

        assert_eq!(max_price(&map), Some(50));

        let empty_map: HashMap<Vec<i32>, u64> = HashMap::new();
        assert_eq!(max_price(&empty_map), None);
    }
}
