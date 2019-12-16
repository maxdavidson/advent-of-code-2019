use std::collections::{
    hash_map::{Entry, HashMap},
    VecDeque,
};

use crate::utils::{
    direction::{Direction, DIRECTIONS},
    intcode::CPU,
    Vec2,
};

#[derive(Debug)]
enum Tile {
    Wall,
    Target,
    EmptyVisited,
    EmptyUnvisited(Droid),
}

#[derive(Debug, Clone)]
struct Droid {
    cpu: CPU<i64>,
}

impl Droid {
    pub fn go(mut self, direction: Direction) -> Tile {
        let next_move = match direction {
            Direction::Up => 1,
            Direction::Down => 2,
            Direction::Left => 3,
            Direction::Right => 4,
        };

        let next_output = self.cpu.outputs_with(move || next_move).next();

        match next_output {
            Some(0) => Tile::Wall,
            Some(1) => Tile::EmptyUnvisited(self),
            Some(2) => Tile::Target,
            _ => panic!("Unexpeted CPU output!"),
        }
    }
}

type Tiles = HashMap<Vec2<i64>, (Tile, usize)>;

fn explore_map(input: &str) -> Tiles {
    let origin = Vec2::origin();

    let mut tiles = HashMap::new();
    tiles.insert(origin, {
        let cpu = CPU::from_source(input);
        let droid = Droid { cpu };
        (Tile::EmptyUnvisited(droid), 0usize)
    });

    let mut queue = VecDeque::new();
    queue.push_back(origin);

    while let Some(pos) = queue.pop_front() {
        let (droid, dist) = if let Entry::Occupied(mut o) = tiles.entry(pos) {
            let dist = if let (Tile::EmptyUnvisited(_), dist) = o.get() {
                *dist
            } else {
                continue;
            };

            if let (Tile::EmptyUnvisited(droid), dist) = o.insert((Tile::EmptyVisited, dist)) {
                (droid, dist)
            } else {
                continue;
            }
        } else {
            continue;
        };

        for &dir in &DIRECTIONS {
            let next_pos = pos.translated(dir);
            if let Entry::Vacant(v) = tiles.entry(next_pos) {
                let next_tile = droid.clone().go(dir);
                let next_dist = dist + 1;
                v.insert((next_tile, next_dist));
                queue.push_back(next_pos);
            }
        }
    }

    tiles
}

fn find_target(tiles: &Tiles) -> Option<(Vec2<i64>, usize)> {
    tiles.iter().find_map(
        |(pos, (tile, dist))| {
            if let Tile::Target = tile {
                Some((*pos, *dist))
            } else {
                None
            }
        },
    )
}

#[allow(dead_code)]
fn part1(input: &str) -> usize {
    let tiles = explore_map(input);
    let (_, target_dist) = find_target(&tiles).unwrap();
    target_dist
}

#[allow(dead_code)]
fn part2(input: &str) -> usize {
    let tiles = explore_map(input);
    let (target_pos, _) = find_target(&tiles).unwrap();

    let mut dists = HashMap::new();
    dists.insert(target_pos, 0);

    let mut queue = VecDeque::new();
    queue.push_back(target_pos);

    while let Some(pos) = queue.pop_front() {
        let dist = *dists.get(&pos).expect("No dist for pos");
        match tiles.get(&pos) {
            None | Some((Tile::Wall, _)) => {}
            _ => {
                for &dir in &DIRECTIONS {
                    let next_pos = pos.translated(dir);

                    if let Entry::Vacant(dists_entry) = dists.entry(next_pos) {
                        dists_entry.insert(dist + 1);
                        queue.push_back(next_pos);
                    }
                }
            }
        }
    }

    dists.values().copied().max().expect("No max value")
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("input");

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), 250);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), 332);
    }
}
