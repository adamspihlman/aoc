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
    pub cost: f64,
    pub x: u64,
    pub x_f: f64,
    pub y: u64,
    pub y_f: f64,
}

#[derive(Debug, Clone, Copy)]
struct Prize {
    pub x: u64,
    pub x_f: f64,
    pub y: u64,
    pub y_f: f64,
}

#[derive(Default)]
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
        let prize_distance = (self.prize.x_f.powi(2) + self.prize.y_f.powi(2)).sqrt();

        let b_distance = (self.b.x_f.powi(2) + self.b.y_f.powi(2)).sqrt();

        let prize_b_angle =
            (self.prize.y_f / self.prize.x_f).atan() - (self.b.y_f / self.b.x_f).atan();
        let b_distance_on_prize = b_distance / prize_b_angle.cos();
        let b_normal_cost = self.b.cost * (prize_distance / b_distance_on_prize);

        let a_distance = (self.a.x_f.powi(2) + self.a.y_f.powi(2)).sqrt();

        let prize_a_angle =
            (self.prize.y_f / self.prize.x_f).atan() - (self.a.y_f / self.a.x_f).atan();
        let a_distance_on_prize = a_distance / prize_a_angle.cos();
        let a_normal_cost = self.a.cost * (prize_distance / a_distance_on_prize);

        let mut b_press = 0;
        let mut a_press = 0;

        let b_cheaper = b_normal_cost < a_normal_cost;

        if b_cheaper {
            b_press = std::cmp::min(100, b_normal_cost.round() as i32);
        } else {
            a_press = std::cmp::min(100, a_normal_cost.round() as i32);
        }

        loop {
            if b_press < 0 || a_press < 0 {
                return 0;
            }
            let cur_x = self.b.x * b_press as u64 + self.a.x * a_press as u64;
            let cur_y = self.b.y * b_press as u64 + self.a.y * a_press as u64;

            if cur_x == self.prize.x && cur_y == self.prize.y {
                break;
            } else if cur_x > self.prize.x || cur_y > self.prize.y {
                if b_cheaper {
                    b_press -= 1;
                    a_press = 0;
                } else {
                    a_press -= 1;
                    b_press = 0;
                }
                continue;
            } else {
                if b_cheaper {
                    a_press += 1;
                } else {
                    b_press += 1;
                }
                continue;
            }
        }

        self.b.cost as u64 * b_press as u64 + self.a.cost as u64 * a_press as u64
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
    pub fn machines(mut self, input: &str) -> ClawBuilder {
        let button_regex = Regex::new(r"^Button [AB]: X\+(\d+), Y\+(\d+)").unwrap();
        let prize_regex = Regex::new(r"^Prize: X=(\d+), Y=(\d+)").unwrap();

        let mut machines = Vec::new();
        let mut input_type = InputType::AButton;
        let mut a_button = Button {
            cost: 0.0,
            x: 0,
            x_f: 0.0,
            y: 0,
            y_f: 0.0,
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

    const A_COST: f64 = 3.0;
    const B_COST: f64 = 1.0;

    fn next(input_type: InputType) -> InputType {
        match input_type {
            InputType::AButton => InputType::BButton,
            InputType::BButton => InputType::Prize,
            InputType::Prize => InputType::Newline,
            InputType::Newline => InputType::AButton,
        }
    }

    fn create_button(line: &str, re: &Regex, cost: f64) -> Button {
        let (_, [x, y]) = re.captures_iter(line).next().unwrap().extract();
        let x = x.parse::<u64>().unwrap();
        let y = y.parse::<u64>().unwrap();
        Button {
            cost,
            x,
            x_f: x as f64,
            y,
            y_f: y as f64,
        }
    }

    fn create_prize(line: &str, re: &Regex) -> Prize {
        let (_, [x, y]) = re.captures_iter(line).next().unwrap().extract();
        let x = x.parse::<u64>().unwrap();
        let y = y.parse::<u64>().unwrap();

        Prize {
            x,
            x_f: x as f64,
            y,
            y_f: y as f64,
        }
    }
}
