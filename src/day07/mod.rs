use itertools::Itertools;

use crate::utils::intcode::{ExecutionResult, CPU};

fn parse_input(input: &str) -> Vec<i32> {
    input.trim().split(',').filter_map(|s| s.parse().ok()).collect()
}

#[derive(Debug, Clone)]
struct Amplifier(CPU<i32>);

impl Amplifier {
    fn new(memory: Box<[i32]>, phase: i32) -> Amplifier {
        let mut cpu = CPU::new(memory);

        // Run the CPU until the first input before returning
        loop {
            match cpu.execute_instruction() {
                ExecutionResult::YieldedInput(sink) => {
                    sink(phase);
                    break;
                }
                ExecutionResult::Running => {}
                _ => panic!("This shouldn't happen!"),
            }
        }

        Amplifier(cpu)
    }

    fn run(&mut self, input: i32) -> Option<i32> {
        loop {
            match self.0.execute_instruction() {
                ExecutionResult::YieldedInput(sink) => sink(input),
                ExecutionResult::YieldedOutput(value) => break Some(value),
                ExecutionResult::Completed => break None,
                ExecutionResult::Running => {}
            }
        }
    }
}

#[allow(dead_code)]
fn find_largest_output_signal(input: &str) -> i32 {
    let memory = parse_input(input).into_boxed_slice();

    (0..5)
        .permutations(5)
        .map(|phases| {
            phases
                .into_iter()
                .map(|phase| Amplifier::new(memory.clone(), phase))
                .fold(0, |input, mut amplifier| amplifier.run(input).unwrap())
        })
        .max()
        .unwrap()
}

#[allow(dead_code)]
fn find_largest_output_signal_with_feedback(input: &str) -> i32 {
    let memory = parse_input(input).into_boxed_slice();

    (5..10)
        .permutations(5)
        .map(|phases| {
            let mut amplifiers: Vec<_> =
                phases.into_iter().map(|phase| Amplifier::new(memory.clone(), phase)).collect();

            let mut current_value = 0;

            loop {
                for amplifier in amplifiers.iter_mut() {
                    if let Some(next_value) = amplifier.run(current_value) {
                        current_value = next_value;
                    } else {
                        return current_value;
                    }
                }
            }
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input");

    #[test]
    fn part1() {
        assert_eq!(
            find_largest_output_signal("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
            43210
        );
        assert_eq!(
            find_largest_output_signal(
                "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"
            ),
            54321
        );
        assert_eq!(
            find_largest_output_signal(
                "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"
            ),
            65210
        );
        assert_eq!(find_largest_output_signal(INPUT), 13848);
    }

    #[test]
    fn part2() {
        assert_eq!(
            find_largest_output_signal_with_feedback(
                "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
            ),
            139_629_729
        );
        assert_eq!(
            find_largest_output_signal_with_feedback(
                "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
            ),
            18216
        );
        assert_eq!(find_largest_output_signal_with_feedback(INPUT), 12_932_154);
    }
}
