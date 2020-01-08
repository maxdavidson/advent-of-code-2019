use lazy_static::lazy_static;
use regex::Regex;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Production<'a> {
    name: &'a str,
    amount: i64,
}

#[derive(Debug, Clone)]
struct Reaction<'a> {
    output: Production<'a>,
    inputs: Vec<Production<'a>>,
}

impl Reaction<'_> {
    fn from_line(input: &str) -> Reaction<'_> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+) (\w+)").unwrap();
        }

        let mut productions: Vec<_> = RE
            .find_iter(input)
            .map(|group| group.as_str())
            .map({
                let mut caps = RE.capture_locations();
                move |group| {
                    RE.captures_read(&mut caps, group).unwrap();
                    let get = |i| caps.get(i).map(|(start, end)| &group[start..end]);
                    Production { name: get(2).unwrap(), amount: get(1).unwrap().parse().unwrap() }
                }
            })
            .collect();

        let output = productions.pop().unwrap();
        let inputs = productions;

        Reaction { output, inputs }
    }
}

#[derive(Debug, Clone)]
struct Reactions<'a> {
    reactions: HashMap<&'a str, Reaction<'a>>,
    storage: RefCell<HashMap<&'a str, i64>>,
}

fn div_ceil(lhs: i64, rhs: i64) -> i64 {
    lhs / rhs + if lhs % rhs == 0 { 0 } else { 1 }
}

impl<'a> Reactions<'a> {
    fn from_input(input: &str) -> Reactions<'_> {
        let reactions = input
            .trim()
            .lines()
            .map(Reaction::from_line)
            .map(|reaction| (reaction.output.name, reaction))
            .collect();

        let storage = HashMap::new().into();
        Reactions { reactions, storage }
    }

    fn get_amount(&self, name: &str) -> i64 {
        *self.storage.borrow().get(name).unwrap_or(&0)
    }

    fn update_amount<'b: 'a>(&self, name: &'b str, update: impl FnOnce(i64) -> i64) {
        let mut storage = self.storage.borrow_mut();
        let amount = storage.entry(name).or_insert(0);
        *amount = update(*amount);
    }

    fn produce<'b: 'a>(&self, name: &'b str, requested_amount: i64) {
        if let Some(reaction) = self.reactions.get(name) {
            let current_amount = self.get_amount(name);

            if requested_amount > current_amount {
                let required_amount = requested_amount - current_amount;
                let multiplier = div_ceil(required_amount, reaction.output.amount);

                for input in reaction.inputs.iter() {
                    self.consume(input.name, multiplier * input.amount);
                }

                self.update_amount(name, |prev_amount| {
                    prev_amount + multiplier * reaction.output.amount
                });
            }
        }
    }

    fn consume<'b: 'a>(&self, name: &'b str, amount: i64) {
        self.produce(name, amount);
        self.update_amount(name, |prev_amount| prev_amount - amount);
    }

    fn required_ore_for_fuel(&mut self, amount: i64) -> i64 {
        self.consume("FUEL", amount);
        -self.get_amount("ORE")
    }
}

#[allow(dead_code)]
fn part1(input: &str) -> i64 {
    let mut reactions = Reactions::from_input(input);
    reactions.required_ore_for_fuel(1)
}

#[allow(dead_code)]
fn part2(input: &str) -> i64 {
    let target_ore_amount = 1_000_000_000_000i64;
    let reactions = Reactions::from_input(input);

    let mut start = 0;
    let mut end = target_ore_amount;

    loop {
        if start + 1 == end {
            break start;
        }

        let fuel_amount = (end + start) / 2;
        let required_ore = reactions.clone().required_ore_for_fuel(fuel_amount);

        match required_ore.cmp(&target_ore_amount) {
            Ordering::Greater => {
                end = fuel_amount;
            }
            Ordering::Less => {
                start = fuel_amount;
            }
            Ordering::Equal => {
                break fuel_amount;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("input");

    #[test]
    fn part1_works() {
        assert_eq!(part1(include_str!("test_input0")), 31);
        assert_eq!(part1(include_str!("test_input1")), 165);
        assert_eq!(part1(include_str!("test_input2")), 13312);
        assert_eq!(part1(include_str!("test_input3")), 180_697);
        assert_eq!(part1(include_str!("test_input4")), 2_210_736);

        assert_eq!(part1(INPUT), 1_037_742);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(include_str!("test_input0")), 34_482_758_620);
        assert_eq!(part2(include_str!("test_input1")), 6_323_777_403);
        assert_eq!(part2(include_str!("test_input2")), 82_892_753);
        assert_eq!(part2(include_str!("test_input3")), 5_586_022);
        assert_eq!(part2(include_str!("test_input4")), 460_664);

        assert_eq!(part2(INPUT), 1_572_358);
    }
}
