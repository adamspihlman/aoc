#[derive(Debug, Default)]
pub struct Secret {}

impl Secret {
    pub fn generate(seed: u64, iterations: usize) -> u64 {
        if iterations == 0 {
            return seed;
        }

        let mult = Secret::mix_and_prune(seed, seed * 64);
        let div = Secret::mix_and_prune(mult, mult / 32);
        let result = Secret::mix_and_prune(div, div * 2048);

        Secret::generate(result, iterations - 1)
    }

    fn mix_and_prune(secret: u64, value: u64) -> u64 {
        (secret ^ value) % 16777216
    }
}
