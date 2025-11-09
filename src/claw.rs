use regex::Regex;

#[derive(Debug)]
pub struct Claw {
    machines: Vec<Machine>,
}

#[derive(Debug, Clone)]
struct Machine {
    a: Button,
    b: Button,
    prize: Prize,
}

#[derive(Debug, Copy, Clone)]
struct Button {
    pub cost: u64,
    pub x_step: u64,
    pub y_step: u64,
}

#[derive(Debug, Clone, Copy)]
struct Prize {
    pub x: u64,
    pub y: u64,
}

pub struct ClawBuilder {
    machines: Vec<Machine>,
}

impl Claw {
    pub fn min_cost(&mut self) -> u64 {
        self.machines.iter_mut().map(|m| m.min_cost()).sum()
    }
}

impl Machine {
    pub fn min_cost(&mut self) -> u64 {
        if let Some(cost) = self.find_min(0, 0, 100) {
            return cost;
        }
        0
    }

    fn find_min(&mut self, x: u64, y: u64, iterations: i32) -> Option<u64> {
        if x == self.prize.x && y == self.prize.y {
            return Some(0);
        } else if iterations <= 0 {
            return None;
        }

        match (
            self.find_min(x + self.a.x_step, y + self.a.y_step, iterations - 1),
            self.find_min(x + self.b.x_step, y + self.b.y_step, iterations - 1),
        ) {
            (None, None) => return None,
            (None, Some(b_cost)) => return Some(b_cost + self.b.cost),
            (Some(a_cost), None) => return Some(a_cost + self.a.cost),
            (Some(a_cost), Some(b_cost)) => {
                let a_cost = a_cost + self.a.cost;
                let b_cost = b_cost + self.b.cost;
                return Some(std::cmp::min(a_cost, b_cost));
            }
        }
    }
}

#[derive(Debug)]
enum InputType {
    AButton,
    BButton,
    Prize,
    Newline,
}

impl ClawBuilder {
    pub fn new() -> Self {
        Self {
            machines: Vec::new(),
        }
    }

    pub fn machines(mut self, input: &str) -> ClawBuilder {
        let button_regex = Regex::new(r"^Button [AB]: X\+(\d+), Y\+(\d+)").unwrap();
        let prize_regex = Regex::new(r"^Prize: X=(\d+), Y=(\d+)").unwrap();

        let mut machines = Vec::new();
        let mut input_type = InputType::AButton;
        let mut a_button = Button {
            cost: 0,
            x_step: 0,
            y_step: 0,
        };
        let mut b_button = a_button;

        for line in input.lines() {
            match input_type {
                InputType::AButton => {
                    a_button = ClawBuilder::create_button(line, &button_regex, ClawBuilder::A_COST)
                }
                InputType::BButton => {
                    b_button = ClawBuilder::create_button(line, &button_regex, ClawBuilder::B_COST)
                }
                InputType::Prize => {
                    let prize = ClawBuilder::create_prize(line, &prize_regex);
                    let machine = Machine {
                        a: a_button,
                        b: b_button,
                        prize,
                    };
                    machines.push(machine);
                }
                InputType::Newline => {}
            }
            input_type = ClawBuilder::next(input_type);
        }
        self.machines.extend(machines);

        self
    }

    pub fn build(&self) -> Claw {
        Claw {
            machines: self.machines.clone(),
        }
    }

    const A_COST: u64 = 3;
    const B_COST: u64 = 1;

    fn next(input_type: InputType) -> InputType {
        match input_type {
            InputType::AButton => InputType::BButton,
            InputType::BButton => InputType::Prize,
            InputType::Prize => InputType::Newline,
            InputType::Newline => InputType::AButton,
        }
    }

    fn create_button(line: &str, re: &Regex, cost: u64) -> Button {
        let (_, [x_step, y_step]) = re.captures_iter(line).next().unwrap().extract();
        let x_step = x_step.parse::<u64>().unwrap();
        let y_step = y_step.parse::<u64>().unwrap();
        Button {
            cost,
            x_step,
            y_step,
        }
    }

    fn create_prize(line: &str, re: &Regex) -> Prize {
        let (_, [x, y]) = re.captures_iter(line).next().unwrap().extract();
        let x = x.parse::<u64>().unwrap();
        let y = y.parse::<u64>().unwrap();

        Prize { x, y }
    }
}
