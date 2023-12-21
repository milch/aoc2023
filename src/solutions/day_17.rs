use crate::utils::*;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

const INPUT: &str = include_str!("day_17.txt");

#[derive(Debug, PartialEq, Eq)]
struct Path {
    // traces: List<Trace>,
    trace: Trace,
    cost: usize,
    remaining_estimate: usize,
    consecutive: usize,
}

impl Path {
    fn new(start: Point, direction: Direction) -> Self {
        Self {
            trace: Trace::new(start, direction),
            cost: 0,
            remaining_estimate: 0,
            consecutive: 0,
        }
    }

    fn get_point(&self) -> Point {
        self.trace.get_point()
    }

    fn get_direction(&self) -> Direction {
        self.trace.get_direction()
    }

    fn extend_with_cost(
        &self,
        direction: Direction,
        cost: Option<&usize>,
        distance_to_dest: usize,
        min_steps: usize,
        max_steps: usize,
    ) -> Option<Path> {
        let cost = cost?;
        let new_trace = self.trace.extend(direction)?;
        if self.trace.get_direction() != direction && self.consecutive < min_steps {
            return None;
        }

        let consecutive_steps = if direction == self.trace.get_direction() {
            self.consecutive + 1
        } else {
            1
        };

        if consecutive_steps > max_steps {
            return None;
        }

        Some(Self {
            trace: new_trace,
            cost: self.cost + cost,
            consecutive: consecutive_steps,
            remaining_estimate: self.cost + cost + distance_to_dest,
        })
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.remaining_estimate.cmp(&other.remaining_estimate) {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Less,
        }
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn find_path(matrix: &[Vec<usize>], min_steps: usize, max_steps: usize) -> Option<Path> {
    let origin = Point::origin();
    let destination = Point::new(matrix.last().unwrap().len() - 1, matrix.len() - 1);
    let mut queue: BinaryHeap<Path> = BinaryHeap::from([
        Path::new(origin, Direction::East),
        Path::new(origin, Direction::South),
    ]);
    let mut best_solution = None;
    let mut seen = HashSet::new();

    while let Some(current) = queue.pop() {
        let current_point = current.get_point();
        if current_point == destination {
            let best_solution_cost = best_solution
                .as_ref()
                .map(|s: &Path| s.cost)
                .unwrap_or(usize::MAX);
            best_solution = if best_solution_cost > current.cost {
                Some(current)
            } else {
                best_solution
            };
            continue;
        }

        if !seen.insert((current.trace, current.consecutive)) {
            continue;
        }

        let direction = current.get_direction();
        for dir in [direction.rotate(-90.0), direction, direction.rotate(90.0)] {
            let next_point = match current.get_point() + dir {
                Some(val) => val,
                None => continue,
            };
            if let Some(new_trace) = current.extend_with_cost(
                dir,
                matrix.get_point(next_point),
                next_point.distance(&destination),
                min_steps,
                max_steps,
            ) {
                queue.push(new_trace);
            }
        }
    }

    best_solution
}

pub fn print_solution() {
    let city = INPUT.matrix_parse(|c| c.to_digit(10).unwrap() as usize);
    println!(
        "Least possible heat loss: {}",
        find_path(&city, 1, 3).map(|s| s.cost).unwrap()
    );
    println!(
        "Least possible heat loss with the ultra crucible: {}",
        find_path(&city, 4, 10).map(|s| s.cost).unwrap()
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const SAMPLE: &str = indoc! {"
        2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533
    "};

    #[test]
    fn test_pathing() {
        let city = SAMPLE.matrix_parse(|c| c.to_digit(10).unwrap() as usize);
        assert_eq!(find_path(&city, 1, 3).map(|path| { path.cost }), Some(102));
        assert_eq!(find_path(&city, 4, 10).map(|path| { path.cost }), Some(94));
    }
}
