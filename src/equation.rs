#[derive(Debug)]
pub struct Equation {
    result: u64,
    terms: Vec<u64>,
}

impl Equation {
    pub fn from(line: &str) -> Equation {
        let mut iter = line.split(':');
        let result = iter.next().unwrap().parse::<u64>().unwrap();
        let terms: Vec<u64> = iter
            .next()
            .unwrap()
            .split_whitespace()
            .map(|t| t.parse::<u64>().unwrap())
            .collect();
        Equation { result, terms }
    }

    pub fn is_solvable(&self) -> bool {
        Equation::exists_solution(self.result, &self.terms)
    }

    pub fn get_result(&self) -> u64 {
        self.result
    }

    fn exists_solution(target: u64, terms: &[u64]) -> bool {
        if terms.is_empty() {
            return false;
        }
        if terms.len() == 1 {
            return terms[0] == target;
        }

        let cur = terms[terms.len() - 1];
        let remaining = &terms[0..terms.len() - 1];

        Equation::exists_solution(target - cur, remaining)
            || (target.is_multiple_of(cur) && Equation::exists_solution(target / cur, remaining))
    }
}
