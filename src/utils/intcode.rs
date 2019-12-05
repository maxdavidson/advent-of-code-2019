#[derive(Debug, Clone)]
pub enum Parameter {
    Indexed(usize),
    Immediate(i32),
}

impl Parameter {
    fn from_value(value: i32, immediate_mode: bool) -> Parameter {
        if immediate_mode {
            Parameter::Immediate(value)
        } else {
            Parameter::Indexed(value as usize)
        }
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Add(Parameter, Parameter, Parameter),
    Multiply(Parameter, Parameter, Parameter),
    Read(Parameter),
    Write(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, Parameter),
    Equals(Parameter, Parameter, Parameter),
    Break,
}

pub trait Input {
    fn read(&mut self) -> Option<i32>;
}

pub trait Output {
    fn write(&mut self, value: i32);
}

impl<F: FnMut() -> Option<i32>> Input for F {
    fn read(&mut self) -> Option<i32> {
        self()
    }
}

impl<F: FnMut(i32)> Output for F {
    fn write(&mut self, value: i32) {
        self(value)
    }
}

#[derive(Debug)]
pub struct CPU<Reader: Input, Writer: Output> {
    pub pc: usize,
    pub memory: Box<[i32]>,
    pub reader: Reader,
    pub writer: Writer,
}

impl<Reader: Input, Writer: Output> CPU<Reader, Writer> {
    fn read_param(&self, param: &Parameter) -> i32 {
        match *param {
            Parameter::Immediate(value) => value,
            Parameter::Indexed(index) => self.memory[index],
        }
    }

    fn write_param(&mut self, dest: &Parameter, value: i32) {
        match *dest {
            Parameter::Immediate(_) => panic!("Can't write an immediate parameter!"),
            Parameter::Indexed(index) => self.memory[index] = value,
        }
    }

    fn fetch_instruction(&self) -> Instruction {
        let value = self.memory[self.pc] as usize;
        let opcode = value % 100;

        let p0 = || Parameter::from_value(self.memory[self.pc + 1], (value / 100) % 10 == 1);
        let p1 = || Parameter::from_value(self.memory[self.pc + 2], (value / 1000) % 10 == 1);
        let p2 = || Parameter::from_value(self.memory[self.pc + 3], (value / 10000) % 10 == 1);

        match opcode {
            1 => Instruction::Add(p0(), p1(), p2()),
            2 => Instruction::Multiply(p0(), p1(), p2()),
            3 => Instruction::Read(p0()),
            4 => Instruction::Write(p0()),
            5 => Instruction::JumpIfTrue(p0(), p1()),
            6 => Instruction::JumpIfFalse(p0(), p1()),
            7 => Instruction::LessThan(p0(), p1(), p2()),
            8 => Instruction::Equals(p0(), p1(), p2()),
            99 => Instruction::Break,
            val => panic!("Oops, {} is not a valid opcode!", val),
        }
    }

    pub fn run(&mut self) {
        loop {
            let instruction = self.fetch_instruction();

            match &instruction {
                Instruction::Add(first, second, dest) => {
                    let result = self.read_param(&first) + self.read_param(&second);
                    self.write_param(&dest, result);
                    self.pc += 4;
                }
                Instruction::Multiply(first, second, dest) => {
                    let result = self.read_param(&first) * self.read_param(&second);
                    self.write_param(&dest, result);
                    self.pc += 4;
                }
                Instruction::Read(dest) => {
                    let result = self.reader.read().unwrap();
                    self.write_param(&dest, result);
                    self.pc += 2;
                }
                Instruction::Write(param) => {
                    self.writer.write(self.read_param(&param));
                    self.pc += 2;
                }
                Instruction::JumpIfTrue(cond, param) => {
                    if self.read_param(&cond) != 0 {
                        self.pc = self.read_param(&param) as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                Instruction::JumpIfFalse(cond, param) => {
                    if self.read_param(&cond) == 0 {
                        self.pc = self.read_param(&param) as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                Instruction::LessThan(first, second, dest) => {
                    let result =
                        if self.read_param(&first) < self.read_param(&second) { 1 } else { 0 };
                    self.write_param(&dest, result);
                    self.pc += 4;
                }

                Instruction::Equals(first, second, dest) => {
                    let result =
                        if self.read_param(&first) == self.read_param(&second) { 1 } else { 0 };
                    self.write_param(&dest, result);
                    self.pc += 4;
                }

                Instruction::Break => {
                    break;
                }
            }
        }
    }
}
