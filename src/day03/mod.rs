use num::{Num, Signed};
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::str::FromStr;

use crate::utils::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(val: char) -> Result<Direction, Self::Error> {
        match val {
            'U' => Ok(Direction::Up),
            'D' => Ok(Direction::Down),
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

impl<T: Num + Signed + Copy> Vec2<T> {
    fn up() -> Vec2<T> {
        Vec2(T::zero(), T::one())
    }

    fn down() -> Vec2<T> {
        Vec2(T::zero(), -T::one())
    }

    fn left() -> Vec2<T> {
        Vec2(-T::one(), T::zero())
    }

    fn right() -> Vec2<T> {
        Vec2(T::one(), T::zero())
    }

    fn translate(&mut self, direction: Direction) {
        match direction {
            Direction::Up => *self += Self::up(),
            Direction::Down => *self += Self::down(),
            Direction::Left => *self += Self::left(),
            Direction::Right => *self += Self::right(),
        }
    }
}

fn pairs<T: FromStr>(input: &str) -> impl Iterator<Item = (Direction, T)> + '_ {
    input.split(',').filter_map(|s| {
        let (dir, len) = s.trim().split_at(1);
        Some((Direction::try_from(dir.chars().nth(0)?).ok()?, len.parse().ok()?))
    })
}

fn visited_points(pairs: impl Iterator<Item = (Direction, i32)>) -> HashMap<Vec2<i32>, usize> {
    let steps = pairs.flat_map(|(dir, len)| (0..len).map(move |_| dir));

    let mut visited = HashMap::new();
    let mut current_point = Vec2(0, 0);

    for (index, dir) in steps.enumerate() {
        current_point.translate(dir);
        visited.entry(current_point).or_insert(index + 1);
    }

    visited
}

#[allow(dead_code)]
fn part1(input: &str) -> i32 {
    let mut all_visited_iter = input.lines().map(pairs).map(visited_points);

    let mut intersections: HashSet<_> =
        all_visited_iter.next().unwrap().into_iter().map(|(vec2, _)| vec2).collect();

    for visited in all_visited_iter {
        intersections.retain(|vec2| visited.contains_key(vec2));
    }

    intersections.into_iter().map(|Vec2(x, y)| x.abs() + y.abs()).min().unwrap()
}

#[allow(dead_code)]
fn part2(input: &str) -> usize {
    let all_visited: Vec<_> = input.lines().map(pairs).map(visited_points).collect();

    let mut all_visited_iter = all_visited.iter();

    let mut intersections: HashSet<_> =
        all_visited_iter.next().unwrap().iter().map(|(vec2, _)| vec2).collect();

    for visited in all_visited_iter {
        intersections.retain(|vec2| visited.contains_key(vec2));
    }

    intersections
        .iter()
        .map(|vec2| all_visited.iter().filter_map(|visited| visited.get(vec2)).sum())
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input");

    #[test]
    fn part1_works() {
        assert_eq!(part1("R8,U5,L5,D3\nU7,R6,D4,L4"), 6);
        assert_eq!(
            part1("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"),
            159
        );
        assert_eq!(
            part1(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            135
        );

        assert_eq!(part1(INPUT), 1626);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2("R8,U5,L5,D3\nU7,R6,D4,L4"), 30);
        assert_eq!(
            part2("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"),
            610
        );
        assert_eq!(
            part2(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            410
        );
        assert_eq!(part2(INPUT), 27330);
    }
}
