#[derive(Debug)]
pub struct Computer {
    a: u64,
    b: u64,
    c: u64,
    program: Vec<u8>,
}

impl Computer {
    pub fn new(a: u64, b: u64, c: u64, program: Vec<u8>) -> Self {
        Self { a, b, c, program }
    }
}
