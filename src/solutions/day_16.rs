use crate::utils::{
    math_2d::{Direction, Point},
    Indexed2D, ToMatrix,
};
use std::collections::{HashSet, VecDeque};

const INPUT: &str = include_str!("day_16.txt");

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Trace {
    current_point: Point,
    direction: Direction,
}

impl Trace {
    fn new(start: Point, direction: Direction) -> Self {
        Self {
            current_point: start,
            direction,
        }
    }

    fn extend(&self, direction: Direction) -> Option<Trace> {
        let next = self.current_point + direction;
        Some(Self {
            current_point: next?,
            direction,
        })
    }
}

fn raytrace(field: &[Vec<char>], start_trace: Trace) -> HashSet<Point> {
    let mut queue: VecDeque<Trace> = VecDeque::from([start_trace]);
    let mut passed_over: HashSet<Point> = HashSet::new();
    let mut seen: HashSet<Trace> = HashSet::new();

    while let Some(current) = queue.pop_back() {
        let direction = current.direction;
        if seen.contains(&current) {
            continue;
        }
        let new_traces = match (field.get_point(current.current_point), direction) {
            (Some('.'), _)
            | (Some('|'), Direction::North)
            | (Some('|'), Direction::South)
            | (Some('-'), Direction::East)
            | (Some('-'), Direction::West) => vec![current.extend(direction)],
            (Some('|'), _) => vec![
                current.extend(Direction::North),
                current.extend(Direction::South),
            ],
            (Some('-'), _) => vec![
                current.extend(Direction::East),
                current.extend(Direction::West),
            ],
            (Some('\\'), Direction::East)
            | (Some('\\'), Direction::West)
            | (Some('/'), Direction::North)
            | (Some('/'), Direction::South) => vec![current.extend(direction.rotate(-90.0))],
            (Some('\\'), _) | (Some('/'), _) => vec![current.extend(direction.rotate(90.0))],
            // Fell off the map
            (None, _) => continue,
            _ => unreachable!("Unhandled character?"),
        };
        for trace in new_traces.into_iter().flatten() {
            queue.push_back(trace)
        }
        seen.insert(current);
        passed_over.insert(current.current_point);
    }

    passed_over
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
