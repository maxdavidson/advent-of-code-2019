fn required_fuel_excluding_fuel_mass(mass: i32) -> i32 {
    mass / 3 - 2
}

fn required_fuel_including_fuel_mass(mass: i32) -> i32 {
    let fuel = required_fuel_excluding_fuel_mass(mass);
    if fuel <= 0 {
        0
    } else {
        fuel + required_fuel_including_fuel_mass(fuel)
    }
}

fn parse_lines<T: std::str::FromStr>(input: &str) -> impl Iterator<Item = T> + '_ {
    input.lines().filter_map(|line| line.parse().ok())
}

#[allow(dead_code)]
fn part1(input: &str) -> i32 {
    parse_lines(input).map(required_fuel_excluding_fuel_mass).sum()
}

#[allow(dead_code)]
fn part2(input: &str) -> i32 {
    parse_lines(input).map(required_fuel_including_fuel_mass).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input");

    #[test]
    fn computes_required_fuel_excluding_fuel_mass() {
        assert_eq!(required_fuel_excluding_fuel_mass(12), 2);
        assert_eq!(required_fuel_excluding_fuel_mass(14), 2);
        assert_eq!(required_fuel_excluding_fuel_mass(1969), 654);
        assert_eq!(required_fuel_excluding_fuel_mass(100_756), 33583);
    }

    #[test]
    fn computes_required_fuel_including_fuel_mass() {
        assert_eq!(required_fuel_including_fuel_mass(12), 2);
        assert_eq!(required_fuel_including_fuel_mass(1969), 966);
        assert_eq!(required_fuel_including_fuel_mass(100_756), 50346);
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), 3_465_154)
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), 5_194_864)
    }
}
