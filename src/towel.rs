use std::collections::HashMap;

#[derive(Debug)]
pub struct Pattern {
    pub pattern: String,
}

#[derive(Debug, Hash, PartialEq, Eq)]
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
        let mut count_map: HashMap<Towel, u64> = HashMap::new();
        count_map.insert(Towel::from(String::new()), 1);

        // Walk backwards through the towel string
        for (i, _) in self.towel.char_indices().rev() {
            let suffix = &self.towel[i..];

            let mut total_count = 0;

            // Check if this suffix starts with any of the patterns
            for pattern in patterns {
                if suffix.starts_with(&pattern.pattern) {
                    // Remove the matching prefix to get the remainder
                    let remainder = &suffix[pattern.pattern.len()..];
                    let remainder_towel = Towel::from(remainder.to_string());

                    // Look up the count for the remainder
                    if let Some(&count) = count_map.get(&remainder_towel) {
                        total_count += count;
                    }
                }
            }

            // Insert the total count for this suffix
            let current_towel = Towel::from(suffix.to_string());
            count_map.insert(current_towel, total_count);
        }

        // Return the count for the full towel string
        *count_map.get(self).unwrap_or(&0)
    }
}
