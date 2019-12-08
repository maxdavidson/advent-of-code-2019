use num::PrimInt;
use std::iter::from_fn;

#[derive(Debug, Clone, Copy)]
pub enum Parameter<T: PrimInt> {
    Indexed(usize),
    Immediate(T),
}

impl<T: PrimInt> Parameter<T> {
    fn from_value(value: T, immediate_mode: bool) -> Parameter<T> {
        if immediate_mode {
            Parameter::Immediate(value)
        } else {
            Parameter::Indexed(value.to_usize().unwrap())
        }
    }
}

#[derive(Debug)]
pub enum Instruction<T: PrimInt> {
    Add(Parameter<T>, Parameter<T>, Parameter<T>),
    Multiply(Parameter<T>, Parameter<T>, Parameter<T>),
    Read(Parameter<T>),
    Write(Parameter<T>),
    JumpIfTrue(Parameter<T>, Parameter<T>),
    JumpIfFalse(Parameter<T>, Parameter<T>),
    LessThan(Parameter<T>, Parameter<T>, Parameter<T>),
    Equals(Parameter<T>, Parameter<T>, Parameter<T>),
    Break,
}

pub enum ExecutionResult<T, Sink: FnOnce(T)> {
    Running,
    YieldedOutput(T),
    YieldedInput(Sink),
    Completed,
}

#[derive(Debug, Clone)]
pub struct CPU<T> {
    pc: usize,
    memory: Box<[T]>,
}

impl<T: PrimInt> CPU<T> {
    pub fn new(memory: impl Into<Box<[T]>>) -> CPU<T> {
        CPU { pc: 0, memory: memory.into() }
    }

    fn read_param(&self, param: Parameter<T>) -> T {
        match param {
            Parameter::Immediate(value) => value,
            Parameter::Indexed(index) => self.memory[index],
        }
    }

    fn write_param(&mut self, dest: Parameter<T>, value: T) {
        match dest {
            Parameter::Immediate(_) => panic!("Can't write an immediate parameter!"),
            Parameter::Indexed(index) => self.memory[index] = value,
        }
    }

    fn fetch_instruction(&self) -> Instruction<T> {
        let value = self.memory[self.pc].to_usize().unwrap();
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

    pub fn execute_instruction(&mut self) -> ExecutionResult<T, impl FnOnce(T) + '_> {
        match self.fetch_instruction() {
            Instruction::Add(first, second, dest) => {
                let result = self.read_param(first) + self.read_param(second);
                self.write_param(dest, result);
                self.pc += 4;
                ExecutionResult::Running
            }
            Instruction::Multiply(first, second, dest) => {
                let result = self.read_param(first) * self.read_param(second);
                self.write_param(dest, result);
                self.pc += 4;
                ExecutionResult::Running
            }
            Instruction::Read(dest) => ExecutionResult::YieldedInput(move |result| {
                self.write_param(dest, result);
                self.pc += 2;
            }),
            Instruction::Write(param) => {
                self.pc += 2;
                ExecutionResult::YieldedOutput(self.read_param(param))
            }
            Instruction::JumpIfTrue(cond, param) => {
                if self.read_param(cond) != T::zero() {
                    self.pc = self.read_param(param).to_usize().unwrap();
                } else {
                    self.pc += 3;
                }
                ExecutionResult::Running
            }
            Instruction::JumpIfFalse(cond, param) => {
                if self.read_param(cond) == T::zero() {
                    self.pc = self.read_param(param).to_usize().unwrap();
                } else {
                    self.pc += 3;
                }
                ExecutionResult::Running
            }
            Instruction::LessThan(first, second, dest) => {
                let result = if self.read_param(first) < self.read_param(second) {
                    T::one()
                } else {
                    T::zero()
                };
                self.write_param(dest, result);
                self.pc += 4;
                ExecutionResult::Running
            }
            Instruction::Equals(first, second, dest) => {
                let result = if self.read_param(first) == self.read_param(second) {
                    T::one()
                } else {
                    T::zero()
                };
                self.write_param(dest, result);
                self.pc += 4;
                ExecutionResult::Running
            }
            Instruction::Break => ExecutionResult::Completed,
        }
    }

    /// Returns an iterator over the outputs
    pub fn run(mut self, mut get_input: impl FnMut() -> T) -> impl Iterator<Item = T> {
        from_fn(move || loop {
            match self.execute_instruction() {
                ExecutionResult::YieldedInput(sink) => sink(get_input()),
                ExecutionResult::YieldedOutput(value) => break Some(value),
                ExecutionResult::Completed => break None,
                ExecutionResult::Running => {}
            }
        })
    }
}
