#[derive(Debug)]
pub struct Equation {
    result: u64,
    terms: Vec<u64>,
}

pub enum Operator {
    Add,
    Multiply,
    Concat,
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

    pub fn is_solvable(&self, ops: &[Operator]) -> bool {
        Equation::exists_solution(self.result, &self.terms, ops)
    }

    pub fn get_result(&self) -> u64 {
        self.result
    }

    fn exists_solution(target: u64, terms: &[u64], ops: &[Operator]) -> bool {
        if terms.is_empty() {
            return false;
        }
        if terms.len() == 1 {
            return terms[0] == target;
        }

        let cur = terms[terms.len() - 1];
        let remaining = &terms[0..terms.len() - 1];

        for op in ops {
            match op {
                Operator::Add => {
                    let solved =
                        target >= cur && Equation::exists_solution(target - cur, remaining, ops);
                    if solved {
                        return true;
                    }
                }
                Operator::Multiply => {
                    let solved = target.is_multiple_of(cur)
                        && Equation::exists_solution(target / cur, remaining, ops);
                    if solved {
                        return true;
                    }
                }
                Operator::Concat => {
                    let cur_str = cur.to_string();
                    let target_str = target.to_string();

                    if target_str.ends_with(&cur_str) && target_str.len() > cur_str.len() {
                        let next = target_str[0..target_str.len() - cur_str.len()]
                            .parse::<u64>()
                            .unwrap();
                        if Equation::exists_solution(next, remaining, ops) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}
