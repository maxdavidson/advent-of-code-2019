use crate::utils::intcode::CPU;

impl CPU<i32> {
    fn run_with_no_input(mut self) -> i32 {
        self.outputs().last();
        self.mmu.get(0)
    }

    #[allow(dead_code)]
    fn run_with_noun_and_verb(mut self, noun: i32, verb: i32) -> i32 {
        *self.mmu.get_mut(1) = noun;
        *self.mmu.get_mut(2) = verb;
        self.run_with_no_input()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input");

    #[test]
    fn part1() {
        assert_eq!(CPU::from_source("1,9,10,3,2,3,11,0,99,30,40,50").run_with_no_input(), 3500);
        assert_eq!(CPU::from_source("1,0,0,0,99").run_with_no_input(), 2);
        assert_eq!(CPU::from_source("2,3,0,3,99").run_with_no_input(), 2);
        assert_eq!(CPU::from_source("2,4,4,5,99,0").run_with_no_input(), 2);
        assert_eq!(CPU::from_source("1,1,1,4,99,5,6,0,99").run_with_no_input(), 30);

        assert_eq!(CPU::from_source(INPUT).run_with_noun_and_verb(12, 2), 5_098_658);
    }

    #[test]
    fn part2() {
        let cpu = &CPU::from_source(INPUT);

        let find_solution = || {
            for noun in 0..100 {
                for verb in 0..100 {
                    if cpu.clone().run_with_noun_and_verb(noun, verb) == 19_690_720 {
                        return 100 * noun + verb;
                    }
                }
            }
            panic!("No solution found")
        };

        assert_eq!(find_solution(), 5064)
    }
}
