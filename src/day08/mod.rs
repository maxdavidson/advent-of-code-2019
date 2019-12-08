use bytecount::count;

use std::iter::from_fn;

pub const BLACK: u8 = b'0';
pub const WHITE: u8 = b'1';
pub const TRANSPARENT: u8 = b'2';

fn layers(input: &str, layer_size: usize) -> impl Iterator<Item = &[u8]> + '_ {
    input.as_bytes().chunks_exact(layer_size)
}

#[allow(dead_code)]
fn part1(input: &str, layer_size: usize) -> usize {
    let layer = layers(input, layer_size).min_by_key(|layer| count(layer, BLACK)).unwrap();

    count(layer, WHITE) * count(layer, TRANSPARENT)
}

#[allow(dead_code)]
fn draw_image(input: &str, width: usize, height: usize) {
    let mut layer_iterators =
        layers(input, width * height).map(|layer| layer.iter()).collect::<Vec<_>>();

    let pixels = from_fn(move || {
        let mut pixel = TRANSPARENT;

        for it in layer_iterators.iter_mut() {
            let next_pixel = *it.next()?;
            if pixel == TRANSPARENT {
                pixel = next_pixel;
            }
        }

        Some(pixel)
    });

    for row in pixels.collect::<Vec<_>>().chunks_exact(width) {
        for pixel in row {
            print!("{}", if *pixel == BLACK { ' ' } else { 'x' });
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input");

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT, 25 * 6), 2032);
    }
}
