use num::{PrimInt, Signed};
use std::fmt;
use std::iter::from_fn;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Parameter<T> {
    Immediate(T),
    Indexed(usize),
    Relative(isize),
}

impl<T: PrimInt> Parameter<T> {
    fn from_value(value: T, mode: usize) -> Parameter<T> {
        match mode {
            0 => Parameter::Indexed(value.to_usize().unwrap()),
            1 => Parameter::Immediate(value),
            2 => Parameter::Relative(value.to_isize().unwrap()),
            _ => panic!("Invalid mode"),
        }
    }
}

impl<T: fmt::Display> fmt::Display for Parameter<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Immediate(val) => write!(f, "IM({})", val),
            Self::Indexed(val) => write!(f, "IN({})", val),
            Self::Relative(val) => write!(f, "RE({})", val),
        }
    }
}

pub enum Instruction<T> {
    Add(Parameter<T>, Parameter<T>, Parameter<T>),
    Multiply(Parameter<T>, Parameter<T>, Parameter<T>),
    Read(Parameter<T>),
    Write(Parameter<T>),
    JumpIfTrue(Parameter<T>, Parameter<T>),
    JumpIfFalse(Parameter<T>, Parameter<T>),
    LessThan(Parameter<T>, Parameter<T>, Parameter<T>),
    Equals(Parameter<T>, Parameter<T>, Parameter<T>),
    RelativeBaseOffset(Parameter<T>),
    Break,
}

impl<T: PrimInt + fmt::Display> fmt::Display for Instruction<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add(a, b, c) => write!(f, "ADD {}, {}, {}", a, b, c),
            Self::Multiply(a, b, c) => write!(f, "MUL {}, {}, {}", a, b, c),
            Self::Read(a) => write!(f, "RED {}", a),
            Self::Write(a) => write!(f, "WRT {}", a),
            Self::JumpIfTrue(a, b) => write!(f, "JIT {}, {}", a, b),
            Self::JumpIfFalse(a, b) => write!(f, "JIF {}, {}", a, b),
            Self::LessThan(a, b, c) => write!(f, "LST {}, {}, {}", a, b, c),
            Self::Equals(a, b, c) => write!(f, "EQL {}, {}, {}", a, b, c),
            Self::RelativeBaseOffset(a) => write!(f, "RBO {}", a),
            Self::Break => write!(f, "BRK"),
        }
    }
}

pub enum ExecutionResult<T, Sink: FnOnce(T)> {
    Running,
    YieldedOutput(T),
    YieldedInput(Sink),
    Completed,
}

#[derive(Debug, Clone)]
pub(crate) struct MMU<T>(Vec<T>);

impl<T: PrimInt + Signed> MMU<T> {
    pub fn get(&self, index: usize) -> T {
        self.0.get(index).copied().unwrap_or_else(T::zero)
    }

    pub fn get_mut(&mut self, index: usize) -> &mut T {
        if self.0.len() <= index {
            self.0.resize_with(index + 1, T::zero);
        }
        self.0.get_mut(index).unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct CPU<T> {
    pc: usize,
    rbo: isize,
    pub(crate) mmu: MMU<T>,
}

impl<T> CPU<T> {
    pub fn new(memory: Vec<T>) -> CPU<T> {
        CPU { pc: 0, rbo: 0, mmu: MMU(memory) }
    }
}

impl<T: FromStr> CPU<T> {
    pub fn from_source(input: &str) -> CPU<T> {
        CPU::new(
            input
                .trim()
                .split(',')
                .map(|s| s.parse().ok().unwrap_or_else(|| panic!("Failed parsing {}", s)))
                .collect(),
        )
    }
}

impl<T: PrimInt + Signed> CPU<T> {
    pub fn outputs(&mut self) -> impl Iterator<Item = T> + '_ {
        self.outputs_with(|| panic!("No input provided!"))
    }

    pub fn outputs_with<'a, 'b: 'a>(
        &'a mut self,
        mut get_input: impl FnMut() -> T + 'b,
    ) -> impl Iterator<Item = T> + '_ {
        from_fn(move || loop {
            let instruction = self.fetch_instruction()?;
            match self.execute_instruction(instruction) {
                ExecutionResult::YieldedInput(sink) => sink(get_input()),
                ExecutionResult::YieldedOutput(value) => break Some(value),
                ExecutionResult::Completed => break None,
                ExecutionResult::Running => {}
            }
        })
    }

    pub fn fetch_instruction(&self) -> Option<Instruction<T>> {
        let value = self.mmu.get(self.pc).to_usize()?;
        let opcode = value % 100;

        let p0 = || Parameter::from_value(self.mmu.get(self.pc + 1), (value / 100) % 10);
        let p1 = || Parameter::from_value(self.mmu.get(self.pc + 2), (value / 1000) % 10);
        let p2 = || Parameter::from_value(self.mmu.get(self.pc + 3), (value / 10000) % 10);

        let instruction = match opcode {
            1 => Instruction::Add(p0(), p1(), p2()),
            2 => Instruction::Multiply(p0(), p1(), p2()),
            3 => Instruction::Read(p0()),
            4 => Instruction::Write(p0()),
            5 => Instruction::JumpIfTrue(p0(), p1()),
            6 => Instruction::JumpIfFalse(p0(), p1()),
            7 => Instruction::LessThan(p0(), p1(), p2()),
            8 => Instruction::Equals(p0(), p1(), p2()),
            9 => Instruction::RelativeBaseOffset(p0()),
            99 => Instruction::Break,
            val => {
                panic!("Oops, {} is not a valid opcode!", val);
            }
        };

        Some(instruction)
    }

    pub fn execute_instruction(
        &mut self,
        instruction: Instruction<T>,
    ) -> ExecutionResult<T, impl FnOnce(T) + '_> {
        match instruction {
            Instruction::Add(first, second, dest) => {
                *self.get_param_mut(dest) = self.get_param(first) + self.get_param(second);
                self.pc += 4;
                ExecutionResult::Running
            }
            Instruction::Multiply(first, second, dest) => {
                *self.get_param_mut(dest) = self.get_param(first) * self.get_param(second);
                self.pc += 4;
                ExecutionResult::Running
            }
            Instruction::Read(dest) => ExecutionResult::YieldedInput(move |result| {
                *self.get_param_mut(dest) = result;
                self.pc += 2;
            }),
            Instruction::Write(param) => {
                self.pc += 2;
                ExecutionResult::YieldedOutput(self.get_param(param))
            }
            Instruction::JumpIfTrue(cond, param) => {
                if !self.get_param(cond).is_zero() {
                    self.pc = self.get_param(param).to_usize().unwrap();
                } else {
                    self.pc += 3;
                };
                ExecutionResult::Running
            }
            Instruction::JumpIfFalse(cond, param) => {
                if self.get_param(cond).is_zero() {
                    self.pc = self.get_param(param).to_usize().unwrap();
                } else {
                    self.pc += 3;
                };
                ExecutionResult::Running
            }
            Instruction::LessThan(first, second, dest) => {
                *self.get_param_mut(dest) = if self.get_param(first) < self.get_param(second) {
                    T::one()
                } else {
                    T::zero()
                };
                self.pc += 4;
                ExecutionResult::Running
            }
            Instruction::Equals(first, second, dest) => {
                *self.get_param_mut(dest) = if self.get_param(first) == self.get_param(second) {
                    T::one()
                } else {
                    T::zero()
                };
                self.pc += 4;
                ExecutionResult::Running
            }
            Instruction::RelativeBaseOffset(param) => {
                self.rbo += self.get_param(param).to_isize().unwrap();
                self.pc += 2;
                ExecutionResult::Running
            }
            Instruction::Break => ExecutionResult::Completed,
        }
    }

    fn get_param(&self, param: Parameter<T>) -> T {
        match param {
            Parameter::Immediate(value) => value,
            Parameter::Indexed(index) => self.mmu.get(index),
            Parameter::Relative(index) => self.mmu.get((self.rbo + index) as usize),
        }
    }

    fn get_param_mut(&mut self, dest: Parameter<T>) -> &mut T {
        match dest {
            Parameter::Immediate(_) => panic!("Can't write an immediate parameter!"),
            Parameter::Indexed(index) => self.mmu.get_mut(index),
            Parameter::Relative(index) => self.mmu.get_mut((self.rbo + index) as usize),
        }
    }
}
