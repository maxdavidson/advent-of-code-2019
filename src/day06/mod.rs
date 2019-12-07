use std::collections::{HashMap, HashSet};
use std::iter::from_fn;

/// Iterate over pairs (child -> parent)
fn adjencency_list(input: &str) -> impl Iterator<Item = (&str, &str)> {
    input.trim().lines().filter_map(|s| s.find(')').map(|i| (&s[i + 1..], &s[..i])))
}

struct Orbits<'a>(HashMap<&'a str, &'a str>);

impl Orbits<'_> {
    pub fn from_input(input: &str) -> Orbits {
        Orbits(adjencency_list(input).collect())
    }

    pub fn objects(&self) -> impl Iterator<Item = &str> {
        self.0.keys().copied()
    }

    fn get_parent(&self, object: &str) -> Option<&str> {
        self.0.get(object).copied()
    }

    pub fn walk_parents<'a>(&'a self, mut object: &'a str) -> impl Iterator<Item = &str> {
        from_fn(move || {
            if let Some(parent) = self.get_parent(object) {
                object = parent;
                Some(parent)
            } else {
                None
            }
        })
    }
}

#[allow(dead_code)]
fn total_direct_and_indirect_orbits(input: &str) -> usize {
    let orbits = Orbits::from_input(input);

    orbits.objects().map(|obj| orbits.walk_parents(obj).count()).sum()
}

#[allow(dead_code)]
fn min_required_orbital_transfers(input: &str) -> usize {
    let orbits = Orbits::from_input(input);

    let you_path: HashSet<_> = orbits.walk_parents("YOU").collect();
    let san_path: HashSet<_> = orbits.walk_parents("SAN").collect();

    you_path.symmetric_difference(&san_path).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input");

    #[test]
    fn part1() {
        assert_eq!(
            total_direct_and_indirect_orbits(
                "COM)B\nB)C\nC)D\n\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L"
            ),
            42
        );
        assert_eq!(total_direct_and_indirect_orbits(INPUT), 145_250);
    }

    #[test]
    fn part2() {
        assert_eq!(
            min_required_orbital_transfers(
                "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN"
            ),
            4
        );
        assert_eq!(min_required_orbital_transfers(INPUT), 274);
    }
}
