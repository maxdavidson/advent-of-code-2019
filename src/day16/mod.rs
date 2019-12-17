use std::char;

const BASE_PATTERN: [i64; 4] = [0, 1, 0, -1];

fn parse(input: &str) -> Vec<i64> {
    input.trim().chars().map(|c| c.to_digit(10).unwrap() as i64).collect()
}

fn format(output: Vec<i64>) -> String {
    output.into_iter().map(|n| char::from_digit(n as u32, 10).unwrap()).take(8).collect()
}

#[allow(dead_code)]
fn part1(input: &str) -> String {
    let mut digits = parse(input);

    for _ in 0..100 {
        for i in 0..digits.len() - 1 {
            digits[i] = digits
                .iter()
                .enumerate()
                .skip(i)
                .map(|(j, &d)| d * BASE_PATTERN[((j + 1) / (i + 1)) % 4])
                .sum::<i64>()
                .abs()
                % 10;
        }
    }

    format(digits)
}

#[allow(dead_code)]
fn part2(input: &str) -> String {
    let offset = input[..7].parse().unwrap();
    let digits = parse(input);
    let digits_len = digits.len();

    let mut digits: Vec<_> =
        digits.into_iter().cycle().take(10_000 * digits_len).skip(offset).collect();

    for _ in 0..100 {
        for i in (0..digits.len() - 1).rev() {
            digits[i] = (digits[i] + digits[i + 1]) % 10;
        }
    }

    format(digits)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("input");

    #[test]
    fn part1_works() {
        assert_eq!(part1("80871224585914546619083218645595"), "24176176");
        assert_eq!(part1("19617804207202209144916044189917"), "73745418");
        assert_eq!(part1("69317163492948606335995924319873"), "52432133");

        assert_eq!(part1(INPUT), "44098263");
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2("03036732577212944063491565474664"), "84462026");
        assert_eq!(part2("02935109699940807407585447034323"), "78725270");
        assert_eq!(part2("03081770884921959731165446850517"), "53553731");

        assert_eq!(part2(INPUT), "12482168");
    }
}
