use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashMap;

use crate::utils::{intcode::CPU, Vec2};

#[derive(Debug, Clone, Copy)]
enum Color {
    Black,
    White,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn cw(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn ccw(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
}

impl<T: num::Zero + num::One + num::Signed> From<Direction> for Vec2<T> {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => Vec2(T::zero(), T::one()),
            Direction::Down => Vec2(T::zero(), -T::one()),
            Direction::Left => Vec2(-T::one(), T::zero()),
            Direction::Right => Vec2(T::one(), T::zero()),
        }
    }
}

impl From<Color> for i64 {
    fn from(color: Color) -> Self {
        match color {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

#[allow(dead_code)]
fn paint(input: &str, starting_color: Color) -> HashMap<Vec2<i16>, Color> {
    let mut cpu = CPU::<i64>::from_source(input);

    let mut position = Vec2::origin();
    let mut direction = Direction::Up;
    let mut painted_panels = HashMap::new();

    painted_panels.insert(position, starting_color);

    let mut step = |color| {
        cpu.outputs_with(move || match color {
            Color::Black => 0,
            Color::White => 1,
        })
        .next_tuple()
    };

    loop {
        let color = painted_panels.get(&position).copied().unwrap_or(Color::Black);
        if let Some((color, turn)) = step(color) {
            painted_panels.insert(
                position,
                match color {
                    0 => Color::Black,
                    1 => Color::White,
                    _ => panic!("Invalid color: {}", color),
                },
            );
            direction = match turn {
                0 => direction.ccw(),
                1 => direction.cw(),
                _ => panic!("Invalid turn: {}", turn),
            };
            position += direction.into();
        } else {
            break;
        }
    }

    painted_panels
}

#[allow(dead_code)]
fn draw(panels: &HashMap<Vec2<i16>, Color>) -> String {
    let bounds = panels.keys().fold(None, |bounds, &Vec2(x, y)| match bounds {
        None => Some((x..=x, y..=y)),
        Some((x_range, y_range)) => Some((
            min(x, *x_range.start())..=max(x, *x_range.end()),
            min(y, *y_range.start())..=max(y, *y_range.end()),
        )),
    });

    let (x_range, y_range) = bounds.unwrap();

    y_range
        .rev()
        .map(|y| {
            x_range
                .clone()
                .map(|x| match panels.get(&Vec2(x, y)) {
                    Some(Color::White) => '#',
                    _ => ' ',
                })
                .join("")
        })
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("input");

    #[test]
    fn part1() {
        let panels = paint(INPUT, Color::Black);
        assert_eq!(panels.len(), 2373);
    }

    #[test]
    fn part2() {
        let panels = paint(INPUT, Color::White);
        let drawing = draw(&panels);
        print!("{}", &drawing);
        assert_eq!(drawing, include_str!("drawing"));
    }
}
