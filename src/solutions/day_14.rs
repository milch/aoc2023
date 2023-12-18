use std::collections::HashSet;

use itertools::Itertools;

use crate::utils::math_2d::{Direction, Indexed2D, Point};

const INPUT: &str = include_str!("day_14.txt");

fn tilt(field: &mut [Vec<char>], direction: Direction) {
    let by_rows = if direction.is_y() && direction.dy < 0 {
        (0..field.len()).collect_vec()
    } else {
        (0..field.len()).rev().collect_vec()
    };
    for row_idx in by_rows {
        let by_cols = if direction.is_x() && direction.dx < 0 {
            (0..field[row_idx].len()).collect_vec()
        } else {
            (0..field[row_idx].len()).rev().collect_vec()
        };
        for col_idx in by_cols {
            let start_point = Point::new(col_idx, row_idx);
            let mut current_pos = start_point + direction.opposite();
            if let Some('.') = field.get_point(start_point) {
                while let Some(val) = field.get_point(current_pos) {
                    match val {
                        '#' => break,
                        'O' => {
                            field.swap_points(current_pos, start_point);
                            break;
                        }
                        _ => (),
                    }
                    if direction.is_y() && current_pos.y == 0
                        || direction.is_x() && current_pos.x == 0
                    {
                        break;
                    }
                    current_pos += direction.opposite();
                }
            }
        }
    }
}

fn to_string(field: &[Vec<char>]) -> String {
    field.iter().map(|line| line.iter().join("")).join("\n")
}

fn spin_cycle(field: &mut [Vec<char>], seen: &mut HashSet<String>) -> bool {
    tilt(field, Direction::north());
    tilt(field, Direction::west());
    tilt(field, Direction::south());
    tilt(field, Direction::east());

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
            found_loads.push(calculate_load(&field));
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

fn parse_field(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect_vec()).collect_vec()
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
    let mut field = parse_field(INPUT);
    tilt(&mut field, Direction::north());
    println!(
        "Total load on north support beams: {}",
        calculate_load(&field)
    );

    let mut spin_field = parse_field(INPUT);
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
        let mut regular = parse_field(SAMPLE);
        tilt(&mut regular, Direction::north());
        assert_eq!(regular, parse_field(TILTED_SAMPLE))
    }

    #[test]
    fn test_calculate_load() {
        let mut regular = parse_field(SAMPLE);
        tilt(&mut regular, Direction::north());
        assert_eq!(calculate_load(&regular), 136);
    }

    #[test]
    fn test_cycle() {
        let mut seen = HashSet::new();
        let mut regular = parse_field(SAMPLE);
        spin_cycle(&mut regular, &mut seen);
        assert_eq!(regular, parse_field(CYCLED_ONCE));
        spin_cycle(&mut regular, &mut seen);
        assert_eq!(regular, parse_field(CYCLED_TWICE));
    }

    #[test]
    fn test_load_after_cycle() {
        let mut field = parse_field(SAMPLE);
        assert_eq!(spin_until(&mut field, 1000000000), 64);
    }
}
