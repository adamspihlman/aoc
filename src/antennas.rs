#[derive(Debug)]
pub struct Antennas {
    map: Vec<Vec<char>>,
}

impl Antennas {
    pub fn from(map: Vec<Vec<char>>) -> Antennas {
        Antennas { map }
    }

    pub fn distinct_antinodes(&self) -> u64 {
        0
    }
}
