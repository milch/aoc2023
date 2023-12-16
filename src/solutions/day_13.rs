use itertools::Itertools;

const INPUT: &str = include_str!("day_13.txt");

#[derive(Debug)]
enum ReflectionPosition {
    Horizontal(usize),
    Vertical(usize),
}

fn find_reflection(field: &[Vec<char>], expected_mismatches: usize) -> ReflectionPosition {
    let row_reflection =
        field
            .iter()
            .enumerate()
            .tuple_windows()
            .find_map(|((top_idx, _), (bot_idx, _))| {
                let top_range = (0..=top_idx).rev();
                let bot_range = bot_idx..field.len();

                let mismatches = top_range
                    .zip(bot_range)
                    .map(|(l, r)| {
                        field[l]
                            .iter()
                            .zip(field[r].clone())
                            .filter(|(one, two)| **one != *two)
                            .count()
                    })
                    .sum::<usize>();

                if mismatches == expected_mismatches {
                    Some(top_idx)
                } else {
                    None
                }
            });
    if let Some(row) = row_reflection {
        return ReflectionPosition::Horizontal(row + 1);
    }

    let columns = field.first().unwrap().len();

    let column_reflection = (0..columns)
        .tuple_windows()
        .find_map(|(left_idx, right_idx)| {
            let left_range = (0..=left_idx).rev();
            let right_range = right_idx..columns;
            let mismatches = left_range
                .zip(right_range)
                .map(|(l, r)| field.iter().filter(|row| row[l] != row[r]).count())
                .sum::<usize>();

            if mismatches == expected_mismatches {
                Some(left_idx)
            } else {
                None
            }
        });

    if let Some(col) = column_reflection {
        return ReflectionPosition::Vertical(col + 1);
    }

    panic!("One of these should match!")
}

fn summarize_reflections(input: &str, expected_mismatches: usize) -> usize {
    input
        .split("\n\n")
        .map(|field| {
            match find_reflection(
                &field.lines().map(|l| l.chars().collect_vec()).collect_vec(),
                expected_mismatches,
            ) {
                ReflectionPosition::Horizontal(pos) => pos * 100,
                ReflectionPosition::Vertical(pos) => pos,
            }
        })
        .sum()
}

pub fn print_solution() {
    println!("Summarized notes: {}", summarize_reflections(INPUT, 0));
    println!(
        "Summarized notes with smudges: {}",
        summarize_reflections(INPUT, 1)
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const SAMPLE: &str = indoc! {"
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
    "};

    #[test]
    fn test_something() {
        assert_eq!(summarize_reflections(SAMPLE, 0), 405);
        assert_eq!(summarize_reflections(SAMPLE, 1), 400);
    }
}
