#[derive(Debug)]
pub struct Pattern {
    pub pattern: String,
}

#[derive(Debug)]
pub struct Towel {
    pub towel: String,
}

impl From<String> for Pattern {
    fn from(pattern: String) -> Self {
        Self { pattern }
    }
}

impl From<String> for Towel {
    fn from(towel: String) -> Self {
        Self { towel }
    }
}

impl Towel {
    pub fn is_possible(&self, patterns: &[Pattern]) -> bool {
        if self.towel.is_empty() {
            return true;
        }

        let matching_patterns: Vec<&Pattern> = patterns
            .iter()
            .filter(|p| self.towel.starts_with(&p.pattern))
            .collect();

        if matching_patterns.is_empty() {
            return false;
        }

        matching_patterns.iter().any(|p| {
            let remainder = &self.towel[p.pattern.len()..];
            let new_towel = Towel::from(remainder.to_string());
            new_towel.is_possible(patterns)
        })
    }

    pub fn count_possibilities(&self, patterns: &[Pattern]) -> u64 {
        if self.towel.is_empty() {
            return 1;
        }

        let matching_patterns: Vec<&Pattern> = patterns
            .iter()
            .filter(|p| self.towel.starts_with(&p.pattern))
            .collect();

        if matching_patterns.is_empty() {
            return 0;
        }

        matching_patterns
            .iter()
            .map(|p| {
                let remainder = &self.towel[p.pattern.len()..];
                let new_towel = Towel::from(remainder.to_string());
                new_towel.count_possibilities(patterns)
            })
            .sum()
    }
}
