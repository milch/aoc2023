use itertools::Itertools;

use crate::utils::{Direction, Enumerable2D, Indexed2D, OptionFlatMap, ToMatrixParse, Vector2D};
use std::collections::HashSet;

const INPUT: &str = include_str!("day_21.txt");

#[derive(Debug)]
enum Tile {
    Rock,
    Plot,
    Start,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Plot,
            '#' => Tile::Rock,
            'S' => Tile::Start,
            c => unreachable!("Unhandled char: {c}"),
        }
    }
}

fn find_start(field: &[Vec<Tile>]) -> Vector2D<usize> {
    field
        .iter_2d()
        .enumerate_2d()
        .find_map(|(pt, tile)| {
            if matches!(tile, Tile::Start) {
                Some(pt)
            } else {
                None
            }
        })
        .unwrap()
}

fn possible_steps(field: &[Vec<Tile>], step_count: usize, wrap: bool) -> usize {
    let start_point: Vector2D<isize> = find_start(field).into();

    let mut seen_even = HashSet::new();
    let mut seen_odd = HashSet::new();

    let mut result = (0..step_count).fold(HashSet::from([start_point]), |previous_set, step| {
        let mut new_set = HashSet::new();
        for point in previous_set {
            for dir in Direction::ALL {
                let (new_point, tile) = if wrap {
                    let new_point = point + dir;
                    (Some(new_point), Some(field.get_point_wrap(new_point)))
                } else {
                    let new_point_us = Vector2D {
                        x: point.x as usize,
                        y: point.y as usize,
                    } + dir;
                    (
                        new_point_us.map(|_| point + dir),
                        new_point_us.flat_map(|p| field.get_point(p)),
                    )
                };
                if let Some(Tile::Start) | Some(Tile::Plot) = tile {
                    let unwrapped = new_point.unwrap();
                    if step % 2 == 0 {
                        if seen_even.insert(unwrapped) {
                            new_set.insert(unwrapped);
                        }
                    } else if seen_odd.insert(unwrapped) {
                        new_set.insert(unwrapped);
                    }
                }
            }
        }
        new_set
    });
    result.extend(if step_count % 2 == 0 {
        seen_odd
    } else {
        seen_even
    });
    result.len()
}

fn estimate_steps(field: &[Vec<Tile>], step_count: usize) -> usize {
    // f(x) = ax^2 + bx + c
    let step_size = field.len() as isize;
    let start = step_size / 2;
    let x_vals = (start..).step_by(step_size as usize).take(5).collect_vec();
    let f = x_vals
        .iter()
        .map(|steps| possible_steps(field, *steps as usize, true) as isize)
        .collect_vec();

    let diffs = f.iter().tuple_windows().map(|(a, b)| b - a).collect_vec();

    let diffs_of_diffs = diffs
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec();

    let c = f[0];
    let a = diffs[0];
    let d = diffs_of_diffs[0];
    let n = (step_count as isize - start) / step_size;

    (n * n * d / 2 + n * (a - d / 2) + c) as usize
}

pub fn print_solution() {
    let field = INPUT.matrix_parse(Tile::from);
    println!(
        "Possible places after 64 steps: {}",
        possible_steps(&field, 64, false)
    );

    println!(
        "Possible places after 26501365 steps to infinity: {}",
        estimate_steps(&field, 26_501_365)
    );
}

#[cfg(test)]
mod test {
    use crate::utils::ToMatrixParse;

    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const SAMPLE: &str = indoc! {"
        ...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........
    "};

    #[test]
    fn test_possible_steps_bounded() {
        let field = SAMPLE.matrix_parse(Tile::from);

        assert_eq!(possible_steps(&field, 6, false), 16)
    }

    #[test]
    fn test_possible_steps_unbounded() {
        let field = SAMPLE.matrix_parse(Tile::from);

        assert_eq!(possible_steps(&field, 6, true), 16);
        assert_eq!(possible_steps(&field, 10, true), 50);
        assert_eq!(possible_steps(&field, 50, true), 1594);
        assert_eq!(possible_steps(&field, 100, true), 6536);
        assert_eq!(possible_steps(&field, 500, true), 167004);
        assert_eq!(possible_steps(&field, 1000, true), 668697);
        assert_eq!(possible_steps(&field, 5000, true), 16733044);
    }
}
