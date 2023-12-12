const INPUT: &str = include_str!("day_11.txt");

#[derive(Debug, PartialEq, Eq)]
enum Space {
    Empty(usize),
    Galaxy,
}

impl From<char> for Space {
    fn from(value: char) -> Self {
        match value {
            '.' => Space::Empty(1),
            '#' => Space::Galaxy,
            _ => panic!("Unexpected space"),
        }
    }
}

fn vastly_expand_space(input: &str, multiplier: usize) -> Vec<Vec<Space>> {
    let mut expanded_lines: Vec<Vec<Space>> = vec![];
    for line in input.lines() {
        let new_line = if line.chars().all(|c| c == '.') {
            line.chars().map(|_| Space::Empty(multiplier)).collect()
        } else {
            line.chars().map(Space::from).collect()
        };
        expanded_lines.push(new_line);
    }
    for col_idx in (0..expanded_lines.first().unwrap().len()).rev() {
        let all_empty = expanded_lines
            .iter()
            .all(|l| matches!(l[col_idx], Space::Empty(_)));
        if all_empty {
            for line in expanded_lines.iter_mut() {
                line[col_idx] = Space::Empty(multiplier)
            }
        }
    }
    expanded_lines
}

fn make_pairs(galaxy_count: usize) -> Vec<(usize, usize)> {
    (0..galaxy_count - 1)
        .flat_map(|i| (i..galaxy_count).map(|j| (i, j)).collect::<Vec<_>>())
        .filter(|(i, j)| i != j)
        .collect()
}

fn find_galaxies(space: &[Vec<Space>]) -> Vec<(usize, usize)> {
    space
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(col_idx, elem)| {
                    if *elem == Space::Galaxy {
                        return Some((row_idx, col_idx));
                    };

                    None
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn sum_distances(space: &[Vec<Space>]) -> usize {
    let galaxies = find_galaxies(space);
    make_pairs(galaxies.len())
        .iter()
        .map(|(l, r)| {
            let lhs = galaxies[*l];
            let rhs = galaxies[*r];
            let min_x = lhs.1.min(rhs.1);
            let max_x = lhs.1.max(rhs.1);
            let min_y = lhs.0.min(rhs.0);
            let max_y = lhs.0.max(rhs.0);
            let empties_y = (min_y..max_y)
                .map(|y| {
                    space[y]
                        .iter()
                        .find_map(|e| match e {
                            Space::Empty(mul) => Some(mul),
                            _ => None,
                        })
                        .unwrap()
                })
                .sum::<usize>();
            let empties_x = (min_x..max_x)
                .map(|x| {
                    space
                        .iter()
                        .find_map(|l| match l[x] {
                            Space::Empty(mul) => Some(mul),
                            _ => None,
                        })
                        .unwrap()
                })
                .sum::<usize>();
            empties_y + empties_x
        })
        .sum()
}

pub fn print_solution() {
    println!(
        "Distances between the galaxies (empty space doubled): {}",
        sum_distances(&vastly_expand_space(INPUT, 2))
    );
    println!(
        "Distances between the galaxies (empty space * 1M): {}",
        sum_distances(&vastly_expand_space(INPUT, 1000000))
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const SAMPLE: &str = indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "};

    #[test]
    fn test_make_pairs() {
        assert_eq!(make_pairs(3), vec![(0, 1), (0, 2), (1, 2)]);
        assert_eq!(
            make_pairs(4),
            vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3),]
        );
        assert_eq!(
            make_pairs(find_galaxies(&vastly_expand_space(SAMPLE, 2)).len()).len(),
            36
        );
    }

    #[test]
    fn test_sum_distances() {
        assert_eq!(sum_distances(&vastly_expand_space(SAMPLE, 2)), 374);
        assert_eq!(sum_distances(&vastly_expand_space(SAMPLE, 10)), 1030);
        assert_eq!(sum_distances(&vastly_expand_space(SAMPLE, 100)), 8410);
    }
}
