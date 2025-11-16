#[derive(Debug)]
pub struct Pattern {
    pub pattern: String,
}

#[derive(Debug)]
pub struct Towel {
    pub pattern: String,
}

impl From<String> for Pattern {
    fn from(pattern: String) -> Self {
        Self { pattern }
    }
}

impl From<String> for Towel {
    fn from(pattern: String) -> Self {
        Self { pattern }
    }
}

impl Towel {
    pub fn is_possible(&self, _patterns: &[Pattern]) -> bool {
        todo!()
    }
}
