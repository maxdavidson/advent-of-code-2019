use itertools::Itertools;
use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::HashMap;

use crate::utils::intcode::CPU;

#[allow(dead_code)]
fn count_block_tiles(input: &str) -> usize {
    let mut cpu = CPU::<i64>::from_source(input);
    let tiles: HashMap<_, _> = cpu.outputs().tuples().map(|(x, y, tile)| ((x, y), tile)).collect();
    tiles.values().filter(|tile| **tile == 2).count()
}

#[allow(dead_code)]
fn run_game(input: &str) -> i64 {
    let mut cpu = CPU::<i64>::from_source(input);
    *cpu.mmu.get_mut(0) = 2;

    let mut current_score = 0;

    let ball_x_pos = Cell::new(0);
    let cursor_x_pos = Cell::new(0);

    let get_next_move = || match cursor_x_pos.get().cmp(&ball_x_pos.get()) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    };

    for output in cpu.outputs_with(get_next_move).tuples() {
        match output {
            (-1, 0, score) => {
                current_score = score;
            }
            (x, _, 3) => {
                ball_x_pos.set(x);
            }
            (x, _, 4) => {
                cursor_x_pos.set(x);
            }
            _ => {}
        }
    }

    current_score
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("input");

    #[test]
    fn part1() {
        assert_eq!(count_block_tiles(INPUT), 335);
    }

    #[test]
    fn part2() {
        assert_eq!(run_game(INPUT), 15706);
    }
}
