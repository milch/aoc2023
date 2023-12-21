use crate::utils::{Direction, Vector2D};
use itertools::Itertools;

const INPUT: &str = include_str!("day_18.txt");

struct Instruction {
    direction: Direction,
    count: usize,

    color_count: usize,
    color_direction: Direction,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let (direction, count, color) = value.split_ascii_whitespace().collect_tuple().unwrap();

        let hex_meters = &color[2..color.len() - 2];
        let dir_code = color.chars().nth_back(1).unwrap();

        Self {
            direction: direction.into(),
            count: count.parse().unwrap(),
            color_count: usize::from_str_radix(hex_meters, 16).unwrap(),
            // direction to dig: 0 means R, 1 means D, 2 means L, and 3 means U.
            color_direction: match dir_code {
                '0' => Direction::East,
                '1' => Direction::South,
                '2' => Direction::West,
                '3' => Direction::North,
                _ => unreachable!(),
            },
        }
    }
}

fn calculate_area(instructions: &[(Direction, usize)]) -> isize {
    let points = lengths_to_points(instructions);

    points
        .iter()
        .tuple_windows()
        .fold(0, |sum, (left, right)| {
            sum + (left.y + right.y) * (left.x - right.x) / 2
        })
        .abs()
        + instructions
            .iter()
            .map(|&(_, count)| count as isize)
            .sum::<isize>()
            / 2
        + 1
}

fn lengths_to_points(instructions: &[(Direction, usize)]) -> Vec<Vector2D<isize>> {
    let mut result = vec![Vector2D::origin()];

    for &(direction, count) in instructions {
        let last_point = result[result.len() - 1];
        let direction: Vector2D<isize> = direction.into();
        result.push(last_point + direction * (count) as isize)
    }

    result
}

pub fn print_solution() {
    let instructions: Vec<Instruction> = INPUT.lines().map(Instruction::from).collect();
    println!(
        "Lagoon area: {}",
        calculate_area(
            &instructions
                .iter()
                .map(|ins| (ins.direction, ins.count))
                .collect_vec()
        )
    );
    println!(
        "Lagoon area: (color parsed) {}",
        calculate_area(
            &instructions
                .iter()
                .map(|ins| (ins.color_direction, ins.color_count))
                .collect_vec()
        )
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const SAMPLE: &str = indoc! {"
        R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)
    "};

    #[test]
    fn test_calculate_area() {
        let instructions: Vec<Instruction> = SAMPLE.lines().map(Instruction::from).collect();
        assert_eq!(
            calculate_area(
                &instructions
                    .iter()
                    .map(|ins| (ins.direction, ins.count))
                    .collect_vec()
            ),
            62
        );

        assert_eq!(
            calculate_area(
                &instructions
                    .iter()
                    .map(|ins| (ins.color_direction, ins.color_count))
                    .collect_vec()
            ),
            952408144115
        );
    }
}
