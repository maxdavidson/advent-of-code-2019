#[derive(Debug, Clone)]
struct Program {
    pc: usize,
    mem: Vec<usize>,
}

enum Instruction {
    Add(usize, usize, usize),
    Mul(usize, usize, usize),
    Brk,
}

#[allow(dead_code)]
impl Program {
    pub fn new(mem: Vec<usize>) -> Program {
        Program { pc: 0, mem }
    }

    pub fn memory(&self) -> &[usize] {
        &self.mem
    }

    pub fn from(input: &str) -> Program {
        Program::new(input.split(',').filter_map(|s| s.trim().parse().ok()).collect())
    }

    pub fn run_with_inputs(mut self, noun: usize, verb: usize) -> usize {
        self.mem[1] = noun;
        self.mem[2] = verb;
        self.run()
    }

    pub fn run(mut self) -> usize {
        loop {
            use Instruction::*;
            match self.next_instruction() {
                Add(a, b, c) => {
                    self.mem[c] = self.mem[a] + self.mem[b];
                    self.pc += 4;
                }
                Mul(a, b, c) => {
                    self.mem[c] = self.mem[a] * self.mem[b];
                    self.pc += 4;
                }
                Brk => {
                    break self.mem[0];
                }
            }
        }
    }

    fn next_instruction(&self) -> Instruction {
        use Instruction::*;
        match self.mem[self.pc] {
            1 => Add(self.mem[self.pc + 1], self.mem[self.pc + 2], self.mem[self.pc + 3]),
            2 => Mul(self.mem[self.pc + 1], self.mem[self.pc + 2], self.mem[self.pc + 3]),
            99 => Brk,
            _ => panic!("Oops, not an opcode!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input");

    #[test]
    fn part1() {
        assert_eq!(Program::from("1,9,10,3,2,3,11,0,99,30,40,50").run(), 3500);
        assert_eq!(Program::from("1,0,0,0,99").run(), 2);
        assert_eq!(Program::from("2,3,0,3,99").run(), 2);
        assert_eq!(Program::from("2,4,4,5,99,0").run(), 2);
        assert_eq!(Program::from("1,1,1,4,99,5,6,0,99").run(), 30);

        assert_eq!(Program::from(INPUT).run_with_inputs(12, 2), 5_098_658);
    }

    #[test]
    fn part2() {
        let program = Program::from(INPUT);

        let find_solution = || {
            for noun in 0..100 {
                for verb in 0..100 {
                    if program.clone().run_with_inputs(noun, verb) == 19_690_720 {
                        return 100 * noun + verb;
                    }
                }
            }
            panic!("No solution found")
        };

        assert_eq!(find_solution(), 5064)
    }
}
