use std::collections::HashSet;
use std::str::FromStr;

const INPUT: &str = include_str!("day_10.txt");

const NORTH: (i64, i64) = (-1, 0);
const SOUTH: (i64, i64) = (1, 0);
const EAST: (i64, i64) = (0, 1);
const WEST: (i64, i64) = (0, -1);

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum PipePiece {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl FromStr for PipePiece {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "|" => Ok(Self::NorthSouth),
            "-" => Ok(Self::EastWest),
            "L" => Ok(Self::NorthEast),
            "J" => Ok(Self::NorthWest),
            "7" => Ok(Self::SouthWest),
            "F" => Ok(Self::SouthEast),
            "." => Ok(Self::Ground),
            "S" => Ok(Self::Start),
            _ => Err(()),
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<PipePiece>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_string().parse().unwrap()).collect())
        .collect()
}

fn find_loop(matrix: &[Vec<PipePiece>]) -> (PipePiece, Vec<(i64, i64)>) {
    let start_pos = matrix
        .iter()
        .enumerate()
        .find_map(|(row_idx, row)| {
            row.iter().enumerate().find_map(|(col_idx, piece)| {
                if *piece == PipePiece::Start {
                    Some((row_idx as i64, col_idx as i64))
                } else {
                    None
                }
            })
        })
        .unwrap();

    [NORTH, SOUTH, EAST, WEST]
        .map(|heading| follow_direction(matrix, start_pos, heading))
        .iter()
        .max_by_key(|path| path.1.len())
        .unwrap()
        .clone()
}

fn find_loop_length(matrix: Vec<Vec<PipePiece>>) -> usize {
    find_loop(&matrix).1.len() / 2
}

fn count_insides(matrix: &[Vec<PipePiece>]) -> usize {
    let path = find_loop(matrix);
    let loop_path: HashSet<(i64, i64)> = HashSet::from_iter(path.1);
    matrix
        .iter()
        .enumerate()
        .map(|(row_idx, row)| {
            let mut is_inside = false;
            row.iter()
                .enumerate()
                .filter(|(col_idx, piece)| {
                    let current = (row_idx as i64, *col_idx as i64);
                    let check_piece = if **piece == PipePiece::Start {
                        &path.0
                    } else {
                        *piece
                    };
                    if loop_path.contains(&current) {
                        if [
                            PipePiece::NorthSouth,
                            PipePiece::NorthWest,
                            PipePiece::NorthEast,
                        ]
                        .contains(check_piece)
                        {
                            is_inside = !is_inside;
                        }
                        return false;
                    }

                    is_inside
                })
                .count()
        })
        .sum()
}

fn get_pos<T>(matrix: &[Vec<T>], pos: (i64, i64)) -> Option<&T> {
    if pos.0 < 0 || pos.1 < 0 {
        return None;
    }
    match matrix.get(pos.0 as usize) {
        Some(row) => row.get(pos.1 as usize),
        None => None,
    }
}

fn follow_direction(
    matrix: &[Vec<PipePiece>],
    start: (i64, i64),
    direction: (i64, i64),
) -> (PipePiece, Vec<(i64, i64)>) {
    let mut current_dir = direction;
    let mut path = vec![start];
    let mut last_heading = direction;
    loop {
        let mut current_pos = *path.last().unwrap();
        current_pos.0 += current_dir.0;
        current_pos.1 += current_dir.1;
        match (get_pos(matrix, current_pos), current_dir) {
            (Some(&PipePiece::NorthSouth), NORTH) => current_dir = NORTH,
            (Some(&PipePiece::NorthSouth), SOUTH) => current_dir = SOUTH,
            (Some(&PipePiece::EastWest), EAST) => current_dir = EAST,
            (Some(&PipePiece::EastWest), WEST) => current_dir = WEST,
            (Some(&PipePiece::NorthEast), SOUTH) => current_dir = EAST,
            (Some(&PipePiece::NorthEast), WEST) => current_dir = NORTH,
            (Some(&PipePiece::NorthWest), SOUTH) => current_dir = WEST,
            (Some(&PipePiece::NorthWest), EAST) => current_dir = NORTH,
            (Some(&PipePiece::SouthWest), NORTH) => current_dir = WEST,
            (Some(&PipePiece::SouthWest), EAST) => current_dir = SOUTH,
            (Some(&PipePiece::SouthEast), NORTH) => current_dir = EAST,
            (Some(&PipePiece::SouthEast), WEST) => current_dir = SOUTH,
            // We found the start!
            (Some(&PipePiece::Start), _) => break,
            // We followed the pipe and ended up in an impossible location
            (Some(&PipePiece::Ground), _) => break,
            (None, _) => break,
            _ => break,
        }
        last_heading = current_dir;
        path.push(current_pos);
    }

    let start_piece = match (direction, last_heading) {
        (NORTH, SOUTH) => PipePiece::NorthSouth,
        (NORTH, WEST) => PipePiece::SouthEast,
        (WEST, NORTH) => PipePiece::SouthEast,
        (NORTH, EAST) => PipePiece::SouthWest,
        (EAST, NORTH) => PipePiece::SouthWest,
        (EAST, WEST) => PipePiece::EastWest,
        (WEST, EAST) => PipePiece::EastWest,
        (SOUTH, EAST) => PipePiece::NorthWest,
        (EAST, SOUTH) => PipePiece::NorthWest,
        (WEST, SOUTH) => PipePiece::NorthEast,
        (SOUTH, NORTH) => PipePiece::NorthSouth,
        _ => PipePiece::Start,
    };
    (start_piece, path)
}

pub fn print_solution() {
    println!("Steps: {}", find_loop_length(parse_input(INPUT)));
    println!("Inside squares: {}", count_insides(&parse_input(INPUT)));
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const SQUARE_LOOP: &str = indoc! {"
        .....
        .S-7.
        .|.|.
        .L-J.
        .....
    "};

    const COMPLEX_LOOP: &str = indoc! {"
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...
    "};

    const FOUR_INNER_TILES: &str = indoc! {"
        ...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........
    "};

    const FOUR_INNER_TILES_NO_PATH: &str = indoc! {"
        ..........
        .S------7.
        .|F----7|.
        .||....||.
        .||....||.
        .|L-7F-J|.
        .|..||..|.
        .L--JL--J.
        ..........
    "};

    const LARGE_SAMPLE: &str = indoc! {"
        .F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ...
    "};

    const SAMPLE_WITH_GARBAGE_PIPES: &str = indoc! {"
        FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L
    "};

    #[test]
    fn test_part_1() {
        assert_eq!(find_loop_length(parse_input(SQUARE_LOOP)), 4);
        assert_eq!(find_loop_length(parse_input(COMPLEX_LOOP)), 8);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(count_insides(&parse_input(SQUARE_LOOP)), 1);
        assert_eq!(count_insides(&parse_input(FOUR_INNER_TILES)), 4);
        assert_eq!(count_insides(&parse_input(FOUR_INNER_TILES_NO_PATH)), 4);
        assert_eq!(count_insides(&parse_input(LARGE_SAMPLE)), 8);
        assert_eq!(count_insides(&parse_input(SAMPLE_WITH_GARBAGE_PIPES)), 10);
    }
}
