use itertools::Itertools;

use crate::utils::{Direction, Enumerable2D, Indexed2D, Point, ToMatrixParse};
use std::collections::{HashMap, HashSet, VecDeque};

const INPUT: &str = include_str!("day_23.txt");

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Path,
            '#' => Tile::Forest,
            '>' => Tile::Slope(Direction::East),
            '<' => Tile::Slope(Direction::West),
            'v' => Tile::Slope(Direction::South),
            '^' => Tile::Slope(Direction::North),
            c => unreachable!("Unhandled character: {c}"),
        }
    }
}

fn find_match_in_row(
    hiking_area: &[Vec<Tile>],
    row: usize,
    match_to_find: Tile,
) -> Option<crate::utils::Vector2D<usize>> {
    hiking_area[row]
        .iter()
        .enumerate()
        .find_map(|(col, tile)| (*tile == match_to_find).then(|| Point::new(col, row)))
}

struct Node {
    point: Point,
    steps: usize,
    seen: HashSet<Point>,
}

impl Node {
    fn new(point: Point) -> Self {
        Self {
            point,
            steps: 0,
            seen: HashSet::from([point]),
        }
    }

    fn expand(&self, new_point: Point, steps: usize) -> Self {
        let mut new_seen = self.seen.clone();
        new_seen.insert(new_point);
        Node {
            point: new_point,
            steps: self.steps + steps,
            seen: new_seen,
        }
    }
}

fn find_longest_path(hiking_area: &[Vec<Tile>], slopes_force_direction: bool) -> usize {
    let start_point = find_match_in_row(hiking_area, 0, Tile::Path).expect("to exist");
    let destination =
        find_match_in_row(hiking_area, hiking_area.len() - 1, Tile::Path).expect("to exist");

    let adjacencies =
        simplify_adjacency_list(&to_adjacency_list(hiking_area, slopes_force_direction));
    let mut queue = VecDeque::from([Node::new(start_point)]);
    let mut best = usize::MIN;

    while let Some(current) = queue.pop_front() {
        if current.point == destination {
            best = best.max(current.steps);
            continue;
        }

        for (neighbor, steps) in adjacencies
            .get(&current.point)
            .unwrap_or_else(|| panic!("expected {} to be in the map", current.point))
        {
            if !current.seen.contains(neighbor) {
                queue.push_front(current.expand(*neighbor, *steps));
            }
        }
    }

    best
}

fn to_adjacency_list(
    hiking_area: &[Vec<Tile>],
    slopes_force_direction: bool,
) -> HashMap<Point, Vec<(Point, usize)>> {
    hiking_area
        .iter_2d()
        .enumerate_2d()
        .filter_map(|(point, tile)| match tile {
            Tile::Slope(dir) if slopes_force_direction => {
                (point + *dir).map(|new_pt| (point, vec![(new_pt, 1)]))
            }
            Tile::Path | Tile::Slope(_) => Some((
                point,
                Direction::ALL
                    .iter()
                    .filter_map(|dir| {
                        let new = point + *dir;
                        new.and_then(|new| hiking_area.get_point(new)).and(new)
                    })
                    .filter(|p| {
                        hiking_area
                            .get_point(*p)
                            .map_or(false, |t| *t != Tile::Forest)
                    })
                    .map(|p| (p, 1))
                    .collect(),
            )),
            Tile::Forest => None,
        })
        .collect()
}

fn simplify_adjacency_list(
    adjacency_list: &HashMap<Point, Vec<(Point, usize)>>,
) -> HashMap<Point, Vec<(Point, usize)>> {
    adjacency_list
        .iter()
        .map(|(point, adj)| {
            let simplified = adj
                .iter()
                .map(|(adjacency, steps)| {
                    let mut last_point = point;
                    let mut current = adjacency;
                    let mut steps = *steps;
                    loop {
                        let next_neighbors = adjacency_list[current]
                            .iter()
                            .filter(|(pt, _)| pt != last_point)
                            .collect_vec();
                        if next_neighbors.len() != 1 {
                            break;
                        }

                        steps += next_neighbors[0].1;
                        last_point = current;
                        current = &next_neighbors[0].0;
                    }

                    (*current, steps)
                })
                .collect();

            (*point, simplified)
        })
        .collect()
}

pub fn print_solution() {
    let hiking_area = INPUT.matrix_parse(Tile::from);
    println!(
        "Longest hiking path: {}",
        find_longest_path(&hiking_area, true)
    );
    println!(
        "Longest hiking path with passable slopes: {}",
        find_longest_path(&hiking_area, false)
    );
}

#[cfg(test)]
mod test {
    use crate::utils::ToMatrixParse;

    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const SAMPLE: &str = indoc! {"
        #.#####################
        #.......#########...###
        #######.#########.#.###
        ###.....#.>.>.###.#.###
        ###v#####.#v#.###.#.###
        ###.>...#.#.#.....#...#
        ###v###.#.#.#########.#
        ###...#.#.#.......#...#
        #####.#.#.#######.#.###
        #.....#.#.#.......#...#
        #.#####.#.#.#########v#
        #.#...#...#...###...>.#
        #.#.#v#######v###.###v#
        #...#.>.#...>.>.#.###.#
        #####v#.#.###v#.#.###.#
        #.....#...#...#.#.#...#
        #.#########.###.#.#.###
        #...###...#...#...#.###
        ###.###.#.###v#####v###
        #...#...#.#.>.>.#.>.###
        #.###.###.#.###.#.#v###
        #.....###...###...#...#
        #####################.#
    "};

    #[test]
    fn test_find_longest_path() {
        let hiking_area = SAMPLE.matrix_parse(Tile::from);
        assert_eq!(find_longest_path(&hiking_area, true), 94);
        assert_eq!(find_longest_path(&hiking_area, false), 154);
    }
}
