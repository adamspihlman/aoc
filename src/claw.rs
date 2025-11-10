use std::{sync::mpsc, thread};

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
    prize_offset: u64,
}

impl Claw {
    pub fn min_cost(&mut self) -> u64 {
        thread::scope(|s| {
            let (tx, rx) = mpsc::channel();
            while let Some(mut machine) = self.machines.pop() {
                let tx_clone = tx.clone();
                s.spawn(move || {
                    let min_cost = machine.min_cost();
                    tx_clone.send(min_cost).unwrap();
                });
            }
            drop(tx);
            rx.iter().sum::<u64>()
        })
    }
}

impl Machine {
    fn get_max_presses(button: &Button, prize: &Prize) -> i64 {
        let x_multiple = prize.x / button.x;
        let y_multiple = prize.y / button.y;

        if prize.x.is_multiple_of(button.x)
            && prize.y.is_multiple_of(button.y)
            && x_multiple == y_multiple
        {
            return x_multiple as i64;
        }
        std::cmp::min(x_multiple, y_multiple) as i64
    }

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

        let b_cheaper = b_normal_cost < a_normal_cost;
        let (cheap, expensive) = if b_cheaper {
            (&self.b, &self.a)
        } else {
            (&self.a, &self.b)
        };

        let mut cheap_press = Machine::get_max_presses(cheap, &self.prize);
        let mut expensive_press = 0;

        loop {
            if cheap_press < 0 || expensive_press < 0 {
                return 0;
            }

            let cur_x = cheap.x * cheap_press as u64 + expensive.x * expensive_press as u64;
            let cur_y = cheap.y * cheap_press as u64 + expensive.y * expensive_press as u64;

            let remaining_x = self.prize.x - cur_x;
            let remaining_y = self.prize.y - cur_y;

            if remaining_x.is_multiple_of(expensive.x)
                && remaining_y.is_multiple_of(expensive.y)
                && remaining_x / expensive.x == remaining_y / expensive.y
            {
                expensive_press = (remaining_x / expensive.x) as i64;
                break;
            } else {
                cheap_press -= 1;
                expensive_press = 0;
            }
        }

        cheap.cost as u64 * cheap_press as u64 + expensive.cost as u64 * expensive_press as u64
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

    fn create_prize(line: &str, re: &Regex, prize_offset: u64) -> Prize {
        let (_, [x, y]) = re.captures_iter(line).next().unwrap().extract();
        let x = x.parse::<u64>().unwrap() + prize_offset;
        let y = y.parse::<u64>().unwrap() + prize_offset;

        Prize {
            x,
            x_f: x as f64,
            y,
            y_f: y as f64,
        }
    }
}
