use crate::utils::intcode::CPU;
use std::collections::VecDeque;

#[allow(dead_code)]
fn diagnostic_code(input: &str, system_id: i32) -> i32 {
    let memory: Vec<i32> = input.trim().split(',').map(|s| s.parse().unwrap()).collect();

    let mut input = VecDeque::new();
    input.push_back(system_id);

    let mut output = VecDeque::new();

    let mut cpu = CPU {
        pc: 0,
        memory: memory.into_boxed_slice(),
        reader: || input.pop_front(),
        writer: |val| output.push_back(val),
    };

    cpu.run();

    let mut it = output.into_iter().rev();

    let result = it.next().expect("No output!");

    if !it.all(|val| val == 0) {
        panic!("All codes must be 0!");
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input");

    #[test]
    fn diagnostic_code_works() {
        let input0 = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        assert_eq!(diagnostic_code(input0, 0), 0);
        assert_eq!(diagnostic_code(input0, -1), 1);
        assert_eq!(diagnostic_code(input0, 1), 1);
        assert_eq!(diagnostic_code(input0, 2), 1);

        let input1 = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
        assert_eq!(diagnostic_code(input1, 0), 0);
        assert_eq!(diagnostic_code(input1, -1), 1);
        assert_eq!(diagnostic_code(input1, 1), 1);
        assert_eq!(diagnostic_code(input1, 2), 1);

        let input2 = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        assert_eq!(diagnostic_code(input2, 7), 999);
        assert_eq!(diagnostic_code(input2, 8), 1000);
        assert_eq!(diagnostic_code(input2, 9), 1001);
    }

    #[test]
    fn part1_works() {
        assert_eq!(diagnostic_code(INPUT, 1), 11_193_703);
    }

    #[test]
    fn part2_works() {
        assert_eq!(diagnostic_code(INPUT, 5), 12_410_607);
    }
}
