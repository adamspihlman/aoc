#[derive(Debug, Default)]
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

    fn price(value: u64) -> u64 {
        value % 10
    }
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
    }
}
