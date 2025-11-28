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
    pub x: u64,
    pub y: u64,
}

#[derive(Debug, Clone, Copy)]
struct Prize {
    pub x: u64,
    pub y: u64,
}

#[derive(Default)]
pub struct ClawBuilder {
    machines: Vec<Machine>,
    prize_offset: u64,
}

impl Claw {
    pub fn min_cost(&self) -> u64 {
        self.machines.iter().map(|m| m.min_cost()).sum()
    }
}

impl Machine {
    pub fn min_cost(&self) -> u64 {
        let b_numerator = self.get_b_numerator();
        let b_denominator = self.get_b_denominator();

        if !b_numerator.is_multiple_of(b_denominator) {
            return 0;
        }

        let b_press = b_numerator / b_denominator;

        let a_numerator = self.get_a_numerator(b_press);
        let a_denominator = self.a.y;

        if !a_numerator.is_multiple_of(a_denominator) {
            return 0;
        }
        let a_press = a_numerator / a_denominator;

        a_press * self.a.cost + b_press * self.b.cost
    }

    fn get_b_numerator(&self) -> u64 {
        let first = self.prize.x * self.a.y;
        let second = self.prize.y * self.a.x;

        first.abs_diff(second)
    }

    fn get_b_denominator(&self) -> u64 {
        let first = self.a.y * self.b.x;
        let second = self.a.x * self.b.y;

        first.abs_diff(second)
    }

    fn get_a_numerator(&self, b_press: u64) -> u64 {
        let first = self.prize.y;
        let second = b_press * self.b.y;

        first.abs_diff(second)
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
    pub fn prize_offset(mut self, prize_offset: u64) -> ClawBuilder {
        self.prize_offset = prize_offset;
        self
    }

    pub fn machines(mut self, input: &str) -> ClawBuilder {
        let button_regex =
            Regex::new(r"^Button [AB]: X\+(\d+), Y\+(\d+)").expect("button regex should compile");
        let prize_regex =
            Regex::new(r"^Prize: X=(\d+), Y=(\d+)").expect("prize regex should compile");

        let mut machines = Vec::new();
        let mut input_type = InputType::AButton;
        let mut a_button = Button {
            cost: 0,
            x: 0,
            y: 0,
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
                    let prize = ClawBuilder::create_prize(line, &prize_regex, self.prize_offset);
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
        let (_, [x, y]) = re
            .captures_iter(line)
            .next()
            .expect("button line should match regex")
            .extract();
        let x = x.parse::<u64>().expect("button x should be valid u64");
        let y = y.parse::<u64>().expect("button y should be valid u64");
        Button { cost, x, y }
    }

    fn create_prize(line: &str, re: &Regex, prize_offset: u64) -> Prize {
        let (_, [x, y]) = re
            .captures_iter(line)
            .next()
            .expect("prize line should match regex")
            .extract();
        let x = x.parse::<u64>().expect("prize x should be valid u64") + prize_offset;
        let y = y.parse::<u64>().expect("prize y should be valid u64") + prize_offset;

        Prize { x, y }
    }
}
