use crate::utils::{
    math_2d::{Direction, Point},
    *,
};
use std::collections::{HashSet, VecDeque};

const INPUT: &str = include_str!("day_16.txt");

fn bfs(
    start: Trace,
    matrix: &[Vec<char>],
    decide_turns: fn(&char, Direction) -> Vec<Direction>,
) -> HashSet<Point> {
    let mut queue: VecDeque<Trace> = VecDeque::from([start]);
    let mut results: HashSet<Point> = HashSet::new();
    let mut seen: HashSet<Trace> = HashSet::new();

    while let Some(current) = queue.pop_front() {
        if seen.contains(&current) {
            continue;
        }
        let elem = matrix.get_point(current.get_point());
        let direction = current.get_direction();
        let new_directions = match elem {
            Some(val) => decide_turns(val, direction),
            None => continue,
        };
        for dir in new_directions {
            if let Some(new_trace) = current.extend(dir) {
                queue.push_back(new_trace);
            }
        }
        seen.insert(current);
        results.insert(current.get_point());
    }

    results
}

fn raytrace(field: &[Vec<char>], start_trace: Trace) -> HashSet<Point> {
    bfs(start_trace, field, |chr, dir| match (chr, dir) {
        ('.', _)
        | ('|', Direction::North)
        | ('|', Direction::South)
        | ('-', Direction::East)
        | ('-', Direction::West) => vec![dir],
        ('|', _) => vec![Direction::North, Direction::South],
        ('-', _) => vec![Direction::East, Direction::West],
        ('\\', Direction::East)
        | ('\\', Direction::West)
        | ('/', Direction::North)
        | ('/', Direction::South) => vec![dir.rotate(-90.0)],
        ('\\', _) | ('/', _) => vec![dir.rotate(90.0)],
        _ => unreachable!("Unhandled character?"),
    })
    .into_iter()
    .collect()
}

fn best_trace(field: &[Vec<char>]) -> usize {
    let top_and_bottom = (0..field[0].len()).flat_map(|i| {
        [
            raytrace(field, Trace::new(Point::new(i, 0), Direction::South)),
            raytrace(
                field,
                Trace::new(Point::new(i, field.len() - 1), Direction::North),
            ),
        ]
    });
    let left_and_right = field.iter().enumerate().flat_map(|(row_idx, row)| {
        [
            raytrace(field, Trace::new(Point::new(0, row_idx), Direction::East)),
            raytrace(
                field,
                Trace::new(Point::new(row.len() - 1, row_idx), Direction::West),
            ),
        ]
    });

    top_and_bottom
        .chain(left_and_right)
        .map(|res| res.len())
        .max()
        .unwrap()
}

pub fn print_solution() {
    let field = INPUT.matrix();
    let rays = raytrace(&field, Trace::new(Point::origin(), Direction::East));
    println!("Number of energized points: {}", rays.len());
    println!("Best energized points: {}", best_trace(&field));
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use itertools::Itertools;
    use pretty_assertions::assert_eq;

    const SAMPLE: &str = indoc! {r"
        .|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....
    "};

    const SAMPLE_TRACED: &str = indoc! {r"
        ######....
        .#...#....
        .#...#####
        .#...##...
        .#...##...
        .#...##...
        .#..####..
        ########..
        .#######..
        .#...#.#..
    "};

    #[test]
    fn test_raytrace() {
        let field = SAMPLE.matrix();
        let rays = raytrace(&field, Trace::new(Point::origin(), Direction::East));

        let rays_string = field
            .iter()
            .enumerate()
            .map(|(row_idx, row)| {
                row.iter()
                    .enumerate()
                    .map(|(col_idx, _)| {
                        if rays.contains(&Point::new(col_idx, row_idx)) {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .join("")
            })
            .join("\n");
        assert_eq!(SAMPLE_TRACED.trim(), rays_string.trim());
        assert_eq!(rays.len(), 46);
    }

    #[test]
    fn test_best_trace() {
        let field = SAMPLE.matrix();

        assert_eq!(best_trace(&field), 51)
    }
}
