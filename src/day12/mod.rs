use crate::utils::Vec3;
use itertools::Itertools;
use lazy_static::lazy_static;
use num::integer::lcm;
use regex::Regex;
use std::iter::once;
use std::str::FromStr;

fn parse_coords<T: FromStr>(input: &str) -> impl Iterator<Item = Vec3<T>> + '_ {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();
    }
    RE.captures_iter(input).filter_map(|cap| {
        Some(Vec3(
            cap.get(1)?.as_str().parse().ok()?,
            cap.get(2)?.as_str().parse().ok()?,
            cap.get(3)?.as_str().parse().ok()?,
        ))
    })
}

#[derive(Debug, Clone)]
struct Moon {
    pos: Vec3<i32>,
    vel: Vec3<i32>,
}

impl Moon {
    pub fn new(pos: Vec3<i32>) -> Moon {
        Moon { pos, vel: Vec3::origin() }
    }

    pub fn energy(&self) -> i32 {
        let potential_energy = self.pos.abs().sum();
        let kinetic_energy = self.vel.abs().sum();
        potential_energy * kinetic_energy
    }
}

#[derive(Debug)]
struct System {
    moons: Box<[Moon]>,
}

impl System {
    pub fn new(moons: impl Into<Box<[Moon]>>) -> System {
        System { moons: moons.into() }
    }

    pub fn moons(&self) -> impl Iterator<Item = &Moon> {
        self.moons.iter()
    }

    pub fn vecs(&self) -> impl Iterator<Item = Vec3<i32>> + '_ {
        self.moons().flat_map(|moon| once(moon.vel).chain(once(moon.pos)))
    }

    pub fn from_input(input: &str) -> System {
        System::new(parse_coords(input).map(Moon::new).collect::<Vec<_>>())
    }

    pub fn total_energy(&self) -> i32 {
        self.moons.iter().map(Moon::energy).sum()
    }

    pub fn simulate(&mut self) {
        // We can't have multiple mutable references to the same data, so use indices
        for (i0, i1) in (0..self.moons.len()).tuple_combinations() {
            let dv = (self.moons[i1].pos - self.moons[i0].pos).signum();
            self.moons[i0].vel += dv;
            self.moons[i1].vel -= dv;
        }

        for moon in self.moons.iter_mut() {
            moon.pos += moon.vel;
        }
    }
}

#[allow(dead_code)]
fn part1(input: &str, steps: usize) -> i32 {
    let mut system = System::from_input(input);

    for _ in 0..steps {
        system.simulate();
    }

    system.total_energy()
}

#[allow(dead_code)]
fn part2(input: &str) -> usize {
    let mut system = System::from_input(input);

    let initial_vecs = system.vecs().collect_vec();

    let mut t = 0;
    let mut periods = [None; 3];

    while {
        system.simulate();
        t += 1;

        if periods[0].is_none() && system.vecs().map(|v| v.0).eq(initial_vecs.iter().map(|v| v.0)) {
            periods[0] = Some(t);
        }

        if periods[1].is_none() && system.vecs().map(|v| v.1).eq(initial_vecs.iter().map(|v| v.1)) {
            periods[1] = Some(t);
        }

        if periods[2].is_none() && system.vecs().map(|v| v.2).eq(initial_vecs.iter().map(|v| v.2)) {
            periods[2] = Some(t);
        }

        periods.iter().any(Option::is_none)
    } {}

    periods.iter().map(|p| p.unwrap()).fold1(lcm).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("input");

    #[test]
    fn part1_works() {
        assert_eq!(part1(include_str!("test_input"), 10), 179);
        assert_eq!(part1(include_str!("test_input2"), 100), 1940);
        assert_eq!(part1(INPUT, 1000), 5350);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(include_str!("test_input")), 2772);
        assert_eq!(part2(include_str!("test_input2")), 4_686_774_924);
        assert_eq!(part2(INPUT), 467_034_091_553_512);
    }
}
