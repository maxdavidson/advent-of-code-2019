use num::{integer::gcd, Integer, Signed, ToPrimitive};
use std::cmp;
use std::collections::{BTreeMap, HashSet, VecDeque};
use std::hash::Hash;
use std::iter;

use crate::utils::Vec2;

fn parse(input: &str) -> impl Iterator<Item = Vec2<i16>> + '_ {
    input.trim().lines().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().filter_map(move |(x, c)| match c {
            '#' => Some(Vec2(x as i16, y as i16)),
            _ => None,
        })
    })
}

impl<T: ToPrimitive + Copy> Vec2<T> {
    fn to_f64(self) -> Option<Vec2<f64>> {
        Some(Vec2(self.0.to_f64()?, self.1.to_f64()?))
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Direction<T>(Vec2<T>);

impl<T: Integer + Copy> Direction<T> {
    pub fn from_vec2(vec2: Vec2<T>) -> Option<Direction<T>> {
        let Vec2(x, y) = vec2;
        if x.is_zero() && y.is_zero() {
            None
        } else {
            let divisor = gcd(x, y);
            Some(Direction(Vec2(x / divisor, y / divisor)))
        }
    }
}

impl<T: ToPrimitive + Copy> Direction<T> {
    fn angle(&self) -> f64 {
        self.0.to_f64().unwrap().angle(Vec2(0.0, 1.0))
    }
}

impl<T: ToPrimitive + Copy + PartialOrd> PartialOrd for Direction<T> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        other.angle().partial_cmp(&self.angle())
    }
}

impl<T: ToPrimitive + Copy + Eq + PartialOrd> Ord for Direction<T> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.partial_cmp(other).unwrap_or(cmp::Ordering::Equal)
    }
}

fn find_best_monitoring_station<'a, T: Integer + Signed + Hash + Copy + 'a>(
    asteroids: impl Iterator<Item = &'a Vec2<T>> + Clone,
) -> Option<(Vec2<T>, usize)> {
    asteroids
        .clone()
        .map(|a0| {
            let directions: HashSet<_> =
                asteroids.clone().filter_map(|a1| Direction::from_vec2(*a1 - *a0)).collect();

            (*a0, directions.len())
        })
        .max_by_key(|(_, count)| *count)
}

#[allow(dead_code)]
fn part1(input: &str) -> (Vec2<i16>, usize) {
    let asteroids: Vec<_> = parse(input).collect();

    find_best_monitoring_station(asteroids.iter()).expect("No best asteroid found!")
}

#[allow(dead_code)]
fn part2(input: &str) -> Option<i16> {
    let (best_asteroid, offset_groups) = {
        let asteroids: Vec<_> = parse(input).collect();
        let (best_asteroid, _) = find_best_monitoring_station(asteroids.iter()).unwrap();

        let mut seen = BTreeMap::new();

        for asteroid in asteroids {
            let diff = asteroid - best_asteroid;
            if let Some(direction) = Direction::from_vec2(diff) {
                seen.entry(direction).or_insert_with(Vec::new).push(diff);
            }
        }

        let offset_groups: Vec<_> = seen
            .into_iter()
            .map(|(_, mut v)| {
                v.sort_by_cached_key(|v| v.len_sqr());
                VecDeque::from(v)
            })
            .collect();

        (best_asteroid, offset_groups)
    };

    let shoot_asteroids = move || {
        let mut group_index = 0;
        let mut offset_groups = offset_groups;
        iter::from_fn(move || {
            let offset_group = offset_groups.get_mut(group_index)?;
            let offset = offset_group.pop_front()?;

            if offset_group.is_empty() {
                offset_groups.remove(group_index);
            } else {
                group_index = (group_index + 1) % offset_groups.len();
            }

            Some(offset + best_asteroid)
        })
    };

    let Vec2(x, y) = shoot_asteroids().nth(199)?;

    Some(x * 100 + y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(part1(include_str!("test_input0")), (Vec2(3, 4), 8));
        assert_eq!(part1(include_str!("test_input1")), (Vec2(5, 8), 33));
        assert_eq!(part1(include_str!("test_input2")), (Vec2(1, 2), 35));
        assert_eq!(part1(include_str!("test_input3")), (Vec2(6, 3), 41));
        assert_eq!(part1(include_str!("test_input4")), (Vec2(11, 13), 210));
        assert_eq!(part1(include_str!("input")), (Vec2(11, 11), 221));
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(include_str!("test_input0")), None);
        assert_eq!(part2(include_str!("test_input4")), Some(802));
        assert_eq!(part2(include_str!("input")), Some(806));
    }
}
