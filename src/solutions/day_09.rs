use std::collections::VecDeque;

const INPUT: &str = include_str!("day_09.txt");

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect()
}

fn extrapolate(numbers: &[i64]) -> Vec<VecDeque<i64>> {
    let mut result = vec![VecDeque::from(numbers.to_vec())];
    loop {
        let diffs = result
            .last()
            .unwrap()
            .iter()
            .zip(result.last().unwrap().iter().skip(1))
            .fold(VecDeque::new(), |mut diffs, (elem, next)| {
                diffs.push_back(next - elem);
                diffs
            });
        let all_equal = diffs.iter().all(|e| e == diffs.back().unwrap());
        result.push(diffs);
        if all_equal {
            break;
        }
    }

    // Part 1: Extrapolate forwards
    if let Some(last) = result.last_mut() {
        let item = last.back().unwrap();
        last.push_back(*item);
    }

    // Part 2: Extrapolate backwards
    if let Some(last) = result.last_mut() {
        let item = last.back().unwrap();
        last.push_front(*item);
    }

    for i in (0..result.len() - 1).rev() {
        if let Some([current, previous]) = result.get_mut(i..=i + 1) {
            // Part 1
            current.push_back(current.back().unwrap() + previous.back().unwrap());
            // Part 2
            current.push_front(current.front().unwrap() - previous.front().unwrap());
        }
    }

    result
}

fn last_of_first_elem(numbers: &[VecDeque<i64>]) -> i64 {
    *numbers.first().unwrap().back().unwrap()
}

fn first_of_first_elem(numbers: &[VecDeque<i64>]) -> i64 {
    *numbers.first().unwrap().front().unwrap()
}

fn sum_all_extrapolations(numbers: &[Vec<i64>], fun: fn(&[VecDeque<i64>]) -> i64) -> i64 {
    numbers.iter().map(|line| fun(&extrapolate(line))).sum()
}

pub fn print_solution() {
    let input = parse_input(INPUT);
    println!(
        "Sum of ending extrapolations: {}",
        sum_all_extrapolations(&input, last_of_first_elem)
    );
    println!(
        "Sum of beginning extrapolations: {}",
        sum_all_extrapolations(&input, first_of_first_elem)
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const SAMPLE: &str = indoc! {"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    "};

    #[test]
    fn test_extrapolate() {
        // Part 1
        assert_eq!(last_of_first_elem(&extrapolate(&[0, 3, 6, 9, 12, 15])), 18);
        assert_eq!(last_of_first_elem(&extrapolate(&[1, 3, 6, 10, 15, 21])), 28);
        assert_eq!(
            last_of_first_elem(&extrapolate(&[10, 13, 16, 21, 30, 45])),
            68
        );
        // Part 2
        assert_eq!(first_of_first_elem(&extrapolate(&[0, 3, 6, 9, 12, 15])), -3);
        assert_eq!(first_of_first_elem(&extrapolate(&[1, 3, 6, 10, 15, 21])), 0);
        assert_eq!(
            first_of_first_elem(&extrapolate(&[10, 13, 16, 21, 30, 45])),
            5
        );
    }

    #[test]
    fn test_summing() {
        assert_eq!(
            sum_all_extrapolations(&parse_input(SAMPLE), last_of_first_elem),
            114
        );
        assert_eq!(
            sum_all_extrapolations(&parse_input(SAMPLE), first_of_first_elem),
            2
        );
    }
}
