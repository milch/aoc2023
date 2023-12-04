use std::collections::HashSet;

const INPUT: &str = include_str!("day_04.txt");

fn parse_numbers(numbers_str: &str) -> impl Iterator<Item = u32> + '_ {
    numbers_str
        .split_ascii_whitespace()
        .map(|num| num.parse().unwrap())
}

fn count_winners(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| {
            let game = line.split_once(": ").unwrap().1;
            let (winners_str, numbers_str) = game.split_once(" | ").unwrap();
            let winners: HashSet<u32> = parse_numbers(winners_str).collect();
            parse_numbers(numbers_str)
                .filter(|num| winners.contains(num))
                .count() as u32
        })
        .collect()
}

fn sum_of_winning_numbers(input: &str) -> u32 {
    count_winners(input)
        .iter()
        .map(|winning_count| {
            if *winning_count == 0 {
                0
            } else {
                2_u32.pow(winning_count - 1)
            }
        })
        .sum()
}

fn determine_scratchcard_copies(input: &str) -> Vec<u32> {
    let winning_numbers = count_winners(input);
    winning_numbers.iter().enumerate().fold(
        vec![1; winning_numbers.len()],
        |mut copies, (idx, number_of_winners)| {
            let indices_that_get_copies = idx + 1..=idx + *number_of_winners as usize;
            indices_that_get_copies.for_each(|copy_idx| {
                copies[copy_idx] += copies[idx];
            });
            copies
        },
    )
}

fn sum_of_scratchcard_copies(input: &str) -> u32 {
    determine_scratchcard_copies(input).iter().sum()
}

pub fn print_solution() {
    println!(
        "Sum of winning numbers: {sum}",
        sum = sum_of_winning_numbers(INPUT)
    );
    println!(
        "Sum of scratchcard copies: {sum}",
        sum = sum_of_scratchcard_copies(INPUT)
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};

    #[test]
    fn test_sum_winning_numbers() {
        assert_eq!(sum_of_winning_numbers(SAMPLE), 13);
    }

    #[test]
    fn test_determine_scratchcard_copies() {
        assert_eq!(
            determine_scratchcard_copies(SAMPLE),
            vec![1, 2, 4, 8, 14, 1]
        );
    }

    #[test]
    fn test_sum_of_scratchcard_copies() {
        assert_eq!(sum_of_scratchcard_copies(SAMPLE), 30);
    }
}
