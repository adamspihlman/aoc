#[derive(Debug)]
pub struct Computer {
    a: u32,
    b: u32,
    c: u32,
    iptr: usize,
    output: String,
}

impl Computer {
    pub fn new(a: u32, b: u32, c: u32) -> Self {
        Self {
            a,
            b,
            c,
            iptr: 0,
            output: String::new(),
        }
    }

    pub fn execute(&mut self, program: Vec<u8>) -> String {
        while self.iptr < program.len() {
            self.do_instruction(program[self.iptr], program[self.iptr + 1]);
        }

        self.output.clone()
    }

    fn do_instruction(&mut self, opcode: u8, operand: u8) {
        match opcode {
            0 => self.adv(operand),
            1 => self.bxl(operand),
            2 => self.bst(operand),
            3 => self.jnz(operand),
            4 => self.bxc(operand),
            5 => self.out(operand),
            6 => self.bdv(operand),
            7 => self.cdv(operand),
            _ => todo!(),
        }
    }

    fn adv(&mut self, operand: u8) {
        self.a = self.div(operand);
        self.next_instruction();
    }

    fn bxl(&mut self, operand: u8) {
        self.b ^= operand as u32;
        self.next_instruction();
    }

    fn bst(&mut self, operand: u8) {
        self.b = self.combo_operand(operand) % 8;
        self.next_instruction();
    }

    fn jnz(&mut self, operand: u8) {
        if self.a == 0 {
            self.next_instruction();
            return;
        }
        self.iptr = operand as usize;
    }

    fn bxc(&mut self, _: u8) {
        self.b ^= self.c;
        self.next_instruction();
    }

    fn out(&mut self, operand: u8) {
        if !self.output.is_empty() {
            self.output.push(',');
        }

        let result = self.combo_operand(operand) % 8;
        self.output.push_str(&result.to_string());

        self.next_instruction();
    }

    fn bdv(&mut self, operand: u8) {
        self.b = self.div(operand);
        self.next_instruction();
    }

    fn cdv(&mut self, operand: u8) {
        self.c = self.div(operand);
        self.next_instruction();
    }

    fn next_instruction(&mut self) {
        self.iptr += 2;
    }

    fn combo_operand(&self, operand: u8) -> u32 {
        match operand {
            0..=3 => operand as u32,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Invalid combo operand '{operand}'"),
        }
    }

    fn div(&self, operand: u8) -> u32 {
        let numerator = self.a;
        let denominator = 2_u32.pow(self.combo_operand(operand));
        numerator / denominator
    }
}
