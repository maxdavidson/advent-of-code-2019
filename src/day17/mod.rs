use std::char;
use std::collections::HashMap;
use std::fmt::Write;
use std::iter::once;

use fancy_regex::Regex as FancyRegex;
use itertools::Itertools;
use lazy_static::lazy_static;

use crate::utils::{
    direction::{Direction, Rotation},
    intcode::CPU,
    Vec2,
};

enum Tile {
    Wall,
    Robot(Direction),
}

fn create_image(input: &str) -> String {
    let mut cpu = CPU::<i64>::from_source(input);
    cpu.outputs().filter_map(|i| char::from_u32(i as u32)).collect()
}

fn parse_image(image: &str) -> HashMap<Vec2<i32>, Tile> {
    image
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| (Vec2(x as i32, y as i32), c))
        })
        .filter_map(|(p, c)| match c {
            '#' => Some((p, Tile::Wall)),
            '^' => Some((p, Tile::Robot(Direction::Down))),
            'v' => Some((p, Tile::Robot(Direction::Up))),
            '<' => Some((p, Tile::Robot(Direction::Left))),
            '>' => Some((p, Tile::Robot(Direction::Right))),
            _ => None,
        })
        .collect()
}

#[allow(dead_code)]
fn part1(input: &str) -> i32 {
    let image = create_image(input);
    let tiles = parse_image(&image);

    let intersections = tiles.keys().filter_map(|&Vec2(x, y)| {
        if tiles.contains_key(&Vec2(x, y))
            && tiles.contains_key(&Vec2(x - 1, y))
            && tiles.contains_key(&Vec2(x + 1, y))
            && tiles.contains_key(&Vec2(x, y - 1))
            && tiles.contains_key(&Vec2(x, y + 1))
        {
            Some((x, y))
        } else {
            None
        }
    });

    intersections.map(|(x, y)| x * y).sum()
}

type PathSegment = (Rotation, usize);

fn create_path(image: &str) -> impl Iterator<Item = PathSegment> {
    let tiles = parse_image(image);

    let (start_pos, start_dir) = tiles
        .iter()
        .find_map(|(p, tile)| if let Tile::Robot(dir) = tile { Some((*p, *dir)) } else { None })
        .expect("No robot found!");

    let mut pos = start_pos;
    let mut dir = start_dir;

    std::iter::from_fn(move || {
        let rot = [Rotation::CW, Rotation::CCW]
            .iter()
            .copied()
            .find(|rot| tiles.contains_key(&pos.translated(dir.rotated(*rot))))?;

        dir = dir.rotated(rot);

        let mut steps = 0;

        loop {
            let next_pos = pos.translated(dir);
            if tiles.contains_key(&next_pos) {
                pos = next_pos;
                steps += 1;
            } else {
                break Some((rot, steps));
            }
        }
    })
}

fn factor_path(path: &[PathSegment]) -> (Vec<usize>, [&[PathSegment]; 3]) {
    lazy_static! {
        static ref SOLVER: FancyRegex =
            FancyRegex::new(r"^(.{1,30}?)\1*(.{1,30}?)(?:\1|\2)*(.{1,30}?)(?:\1|\2|\3)*$").unwrap();
    }

    let raw_path = {
        let mut buf = String::new();
        for (rot, steps) in path {
            let lr = match rot {
                Rotation::CCW => 'R',
                Rotation::CW => 'L',
            };
            if *steps >= 100 {
                panic!("steps is too large!");
            }
            write!(&mut buf, "{}{:02}", lr, steps).unwrap();
        }
        buf
    };

    let caps = SOLVER.captures(&raw_path).expect("No solution found!").unwrap();

    let segments = [
        {
            let cap = caps.get(1).unwrap();
            &path[(cap.start() / 3)..(cap.end() / 3)]
        },
        {
            let cap = caps.get(2).unwrap();
            &path[(cap.start() / 3)..(cap.end() / 3)]
        },
        {
            let cap = caps.get(3).unwrap();
            &path[(cap.start() / 3)..(cap.end() / 3)]
        },
    ];

    let mut instructions = Vec::new();
    let mut current_pos = 0usize;

    while current_pos < path.len() {
        for (i, &segment) in segments.iter().enumerate() {
            let segment_len = segment.len();
            if let Some(window) = path.get(current_pos..current_pos + segment_len) {
                if window == segment {
                    current_pos += segment_len;
                    instructions.push(i);
                }
            }
        }
    }

    (instructions, segments)
}

#[allow(dead_code)]
fn part2(input: &str) -> i64 {
    let image = create_image(input);
    let path: Vec<_> = create_path(&image).collect();
    let (instructions, segments) = factor_path(&path);

    static SEGMENT_SYMBOLS: [char; 3] = ['A', 'B', 'C'];

    let mut message = String::new();

    message.extend(
        instructions.iter().map(|&i| SEGMENT_SYMBOLS[i]).intersperse(',').chain(once('\n')),
    );

    for &segment in &segments {
        for (i, (rot, steps)) in segment.iter().enumerate() {
            if i != 0 {
                message.push(',');
            }
            message.push(match rot {
                Rotation::CCW => 'R',
                Rotation::CW => 'L',
            });
            message.push(',');
            write!(&mut message, "{}", steps).unwrap();
        }
        message.push('\n');
    }

    message.push_str("n\n");

    let mut cpu = CPU::<i64>::from_source(input);
    *cpu.mmu.get_mut(0) = 2;

    let mut message_iter = message.bytes().map(|b| b as i64);

    cpu.outputs_with(|| message_iter.next().unwrap()).last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("input");

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), 7780);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), 1_075_882);
    }
}
