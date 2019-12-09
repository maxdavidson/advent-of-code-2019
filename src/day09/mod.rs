#[cfg(test)]
mod tests {
    use crate::utils::intcode::CPU;
    use itertools::Itertools;

    const INPUT: &str = include_str!("input");

    #[test]
    fn part1() {
        let input = "104,1125899906842624,99";
        assert_eq!(CPU::from_source(input).outputs().last(), Some(1_125_899_906_842_624i64));

        let input = "1102,34915192,34915192,7,4,7,99,0";
        assert_eq!(CPU::from_source(input).outputs().last(), Some(1_219_070_632_396_864i64));

        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        assert_eq!(CPU::<i16>::from_source(input).outputs().join(","), input);

        assert_eq!(CPU::from_source(INPUT).outputs_with(|| 1).last(), Some(2_752_191_671i64));
    }

    #[test]
    fn part2() {
        assert_eq!(CPU::<i64>::from_source(INPUT).outputs_with(|| 2).last(), Some(87571));
    }
}
