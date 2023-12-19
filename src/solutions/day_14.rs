use std::collections::HashSet;

use itertools::Itertools;

use crate::utils::{
    math_2d::{Direction, Point},
    Indexed2D, ToMatrix,
};

const INPUT: &str = include_str!("day_14.txt");

fn tilt(field: &mut [Vec<char>], direction: Direction) {
    let by_rows = if direction == Direction::North {
        (0..field.len()).collect_vec()
    } else {
        (0..field.len()).rev().collect_vec()
    };
    for row_idx in by_rows {
        let by_cols = if direction == Direction::West {
            (0..field[row_idx].len()).collect_vec()
        } else {
            (0..field[row_idx].len()).rev().collect_vec()
        };
        for col_idx in by_cols {
            let start_point = Point::new(col_idx, row_idx);
            let current = start_point + direction.opposite();
            if let (Some(mut current_pos), Some('.')) = (current, &field.get_point(start_point)) {
                while let Some(val) = field.get_point(current_pos) {
                    match val {
                        '#' => break,
                        'O' => {
                            field.swap_points(current_pos, start_point);
                            break;
                        }
                        _ => (),
                    }
                    if let Some(new_pos) = current_pos + direction.opposite() {
                        current_pos = new_pos
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

fn to_string(field: &[Vec<char>]) -> String {
    field.iter().map(|line| line.iter().join("")).join("\n")
}

fn spin_cycle(field: &mut [Vec<char>], seen: &mut HashSet<String>) -> bool {
    tilt(field, Direction::North);
    tilt(field, Direction::West);
    tilt(field, Direction::South);
    tilt(field, Direction::East);

    !seen.insert(to_string(field))
}

fn spin_until(field: &mut [Vec<char>], times: usize) -> usize {
    let mut seen = HashSet::new();
    let mut found_loads = vec![];

    let mut start_idx: Option<usize> = None;
    for i in 0..times {
        if spin_cycle(field, &mut seen) {
            if let Some(first_double) = start_idx {
                start_idx = Some(2 * first_double - i);
                break;
            } else {
                start_idx = Some(i);
                seen.clear()
            }
        }

        if start_idx.is_none() {
            found_loads.push(calculate_load(field));
        }
    }

    let (_, cycle) = found_loads.split_at(start_idx.unwrap() + 1);

    *cycle
        .iter()
        .cycle()
        .take((times - (start_idx.unwrap() + 1)) % cycle.len())
        .last()
        .unwrap()
}

fn calculate_load(field: &[Vec<char>]) -> usize {
    field
        .iter()
        .enumerate()
        .map(|(row_idx, row)| {
            let load = field.len() - row_idx;
            load * row.iter().filter(|chr| **chr == 'O').count()
        })
        .sum()
}

pub fn print_solution() {
    let mut field = INPUT.matrix();
    tilt(&mut field, Direction::North);
    println!(
        "Total load on north support beams: {}",
        calculate_load(&field)
    );

    let mut spin_field = INPUT.matrix();
    let result = spin_until(&mut spin_field, 1000000000);
    println!(
        "Total load on north support beams after a billion spin cycles: {}",
        result
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const SAMPLE: &str = indoc! {"
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
    "};

    const TILTED_SAMPLE: &str = indoc! {"
        OOOO.#.O..
        OO..#....#
        OO..O##..O
        O..#.OO...
        ........#.
        ..#....#.#
        ..O..#.O.O
        ..O.......
        #....###..
        #....#....
    "};

    const CYCLED_ONCE: &str = indoc! {"
        .....#....
        ....#...O#
        ...OO##...
        .OO#......
        .....OOO#.
        .O#...O#.#
        ....O#....
        ......OOOO
        #...O###..
        #..OO#....
    "};

    const CYCLED_TWICE: &str = indoc! {"
        .....#....
        ....#...O#
        .....##...
        ..O#......
        .....OOO#.
        .O#...O#.#
        ....O#...O
        .......OOO
        #..OO###..
        #.OOO#...O
    "};

    #[test]
    fn test_tilt() {
        let mut regular = SAMPLE.matrix();
        tilt(&mut regular, Direction::North);
        assert_eq!(regular, TILTED_SAMPLE.matrix())
    }

    #[test]
    fn test_calculate_load() {
        let mut regular = SAMPLE.matrix();
        tilt(&mut regular, Direction::North);
        assert_eq!(calculate_load(&regular), 136);
    }

    #[test]
    fn test_cycle() {
        let mut seen = HashSet::new();
        let mut regular = SAMPLE.matrix();
        spin_cycle(&mut regular, &mut seen);
        assert_eq!(regular, CYCLED_ONCE.matrix());
        spin_cycle(&mut regular, &mut seen);
        assert_eq!(regular, CYCLED_TWICE.matrix());
    }

    #[test]
    fn test_load_after_cycle() {
        let mut field = SAMPLE.matrix();
        assert_eq!(spin_until(&mut field, 1000000000), 64);
    }
}
