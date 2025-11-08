#[derive(Debug)]
pub struct Topograph {
    map: Vec<Vec<u32>>,
}

impl Topograph {
    pub fn from(map: Vec<Vec<u32>>) -> Topograph {
        Topograph { map }
    }
}
