use cached::proc_macro::cached;

const INPUT: &str = include_str!("day_12.txt");

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum SpringStatus {
    Healthy,
    Damaged,
    Unknown,
}

impl From<char> for SpringStatus {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Healthy,
            '?' => Self::Unknown,
            '#' => Self::Damaged,
            _ => panic!(),
        }
    }
}

impl ToString for SpringStatus {
    fn to_string(&self) -> String {
        match self {
            SpringStatus::Unknown => "?".to_string(),
            SpringStatus::Damaged => "#".to_string(),
            SpringStatus::Healthy => ".".to_string(),
        }
    }
}

fn parse_line<S: Into<String>>(line: S) -> (Vec<SpringStatus>, Vec<usize>) {
    let s = line.into();
    let (springs, arrangements) = s.split_once(' ').unwrap();
    (
        springs.chars().map(SpringStatus::from).collect(),
        arrangements
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect(),
    )
}

fn unfold(line: &str) -> String {
    let (springs, arrangements) = line.split_once(' ').unwrap();
    let unfolded_springs = (0..5).map(|_| springs).collect::<Vec<_>>().join("?");
    let unfolded_arrangements = (0..5).map(|_| arrangements).collect::<Vec<_>>().join(",");

    [unfolded_springs, unfolded_arrangements].join(" ")
}

fn count_one(springs: &[SpringStatus], runs: &[usize]) -> usize {
    count_one_helper(springs, runs, None)
}

#[cached(
    key = "String",
    // There's probably a better way to do this but :shrug:
    convert = r#"{ format!("{:?}{:?}{:?}", springs, runs, current) }"#
)]
fn count_one_helper(springs: &[SpringStatus], runs: &[usize], current: Option<usize>) -> usize {
    if let Some(run) = current {
        // Not possible
        if run > springs.len() {
            return 0;
        }

        if springs.len() == 1 {
            if runs.is_empty() {
                match springs.first().unwrap() {
                    SpringStatus::Damaged => return run,
                    SpringStatus::Unknown => return 1,
                    SpringStatus::Healthy if run == 0 => return 1,
                    SpringStatus::Healthy => return 0,
                }
            } else {
                return 0;
            }
        }
    } else if springs.len() == 1 {
        match (springs.first().unwrap(), runs) {
            (SpringStatus::Healthy, &[]) => return 1,
            (SpringStatus::Damaged, &[1]) => return 1,
            (SpringStatus::Unknown, &[1]) => return 1,
            (SpringStatus::Unknown, &[]) => return 1,
            _ => return 0,
        }
    }

    if current == Some(0) && runs.is_empty() {
        if springs.iter().all(|s| *s != SpringStatus::Damaged) {
            // All others must be Healthy => 1
            return 1;
        } else {
            // This is not possible => 0
            return 0;
        }
    }

    let (first, rest) = springs.split_first().unwrap();
    match (first, current) {
        // This one MUST be healthy to create a new run
        (SpringStatus::Unknown, Some(0)) => count_one_helper(rest, runs, None),
        // Continue the run until we hit 0
        (SpringStatus::Unknown, Some(run)) => count_one_helper(rest, runs, Some(run - 1)),
        // We can either start a new run or count this as healthy
        (SpringStatus::Unknown, None) => {
            let (next_run, rest_runs) = runs.split_first().unwrap();
            count_one_helper(rest, rest_runs, Some(next_run - 1))
                + count_one_helper(rest, runs, None)
        }

        // This one is damaged, but we are at 0 => not possible
        (SpringStatus::Damaged, Some(0)) => 0,
        // Continue the run until we hit 0
        (SpringStatus::Damaged, Some(run)) => count_one_helper(rest, runs, Some(run - 1)),
        // Start a new run
        (SpringStatus::Damaged, None) => {
            let (next_run, rest_runs) = runs.split_first().unwrap();
            count_one_helper(rest, rest_runs, Some(next_run - 1))
        }

        // This one is healthy, and we just hit 0 => move on
        (SpringStatus::Healthy, Some(0)) => count_one_helper(rest, runs, None),
        // This one is healthy, but the run isn't finished => not possible
        (SpringStatus::Healthy, Some(_)) => 0,
        // Just keep going
        (SpringStatus::Healthy, None) => count_one_helper(rest, runs, None),
    }
}

fn count_all(input: &str, pre_parse: fn(&str) -> String) -> usize {
    input
        .lines()
        .map(|line| {
            let (springs, arrangements) = parse_line(pre_parse(line));
            count_one(&springs, &arrangements)
        })
        .sum()
}

pub fn print_solution() {
    println!(
        "Number of arrangements: {}",
        count_all(INPUT, |i| i.to_string())
    );
    println!(
        "Number of unfolded arrangements: {}",
        count_all(INPUT, unfold)
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const SAMPLE: &str = indoc! {"
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1
    "};

    #[test]
    fn test_count_one() {
        let make_slice = |s: &str| s.chars().map(SpringStatus::from).collect::<Vec<_>>();
        assert_eq!(count_one(&make_slice("???"), &[1]), 3);
        assert_eq!(count_one(&make_slice("??"), &[2]), 1);
        assert_eq!(count_one(&make_slice("?"), &[1]), 1);
        assert_eq!(count_one(&make_slice("?#?#??"), &[3]), 1);
        assert_eq!(count_one(&make_slice("?##"), &[3]), 1);
        assert_eq!(count_one(&make_slice("?###????????"), &[5]), 2);

        assert_eq!(
            SAMPLE
                .lines()
                .map(|l| {
                    let (springs, arrs) = l.split_once(' ').unwrap();
                    count_one(
                        &make_slice(springs),
                        &arrs
                            .split(',')
                            .map(|n| n.parse().unwrap())
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>(),
            vec![1, 4, 1, 1, 4, 10]
        );
        assert_eq!(count_one(&make_slice("???.###"), &[1, 1, 3]), 1);
        assert_eq!(count_one(&make_slice(".??..??...?##."), &[1, 1, 3]), 4);
        assert_eq!(count_one(&make_slice("?#?#?#?#?#?#?#?"), &[1, 3, 1, 6]), 1);
        assert_eq!(count_one(&make_slice("????.#...#..."), &[4, 1, 1]), 1);
        assert_eq!(count_one(&make_slice("????.######..#####."), &[1, 6, 5]), 4);
        assert_eq!(count_one(&make_slice("?###????????"), &[3, 2, 1]), 10);
    }

    #[test]
    fn test_count_all() {
        assert_eq!(count_all(SAMPLE, |i| i.to_string()), 21);
    }

    #[test]
    fn test_count_unfolded() {
        let count = |s: &str| {
            let line = parse_line(unfold(s));
            count_one(&line.0, &line.1)
        };

        assert_eq!(count(".# 1"), 1);

        assert_eq!(count("???.### 1,1,3"), 1);
        assert_eq!(count(".??..??...?##. 1,1,3"), 16384);
        assert_eq!(count("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(count("????.#...#... 4,1,1"), 16);
        assert_eq!(count("????.######..#####. 1,6,5"), 2500);
        assert_eq!(count("?###???????? 3,2,1"), 506250);

        assert_eq!(count_all(SAMPLE, unfold), 525152);
    }
}
