const INPUT: &str = include_str!("day_06.txt");

type TimesTable = Vec<(usize, usize)>;

fn parse_input(input: &str) -> TimesTable {
    let (times, distances) = input.split_once('\n').unwrap();
    times
        .strip_prefix("Time: ")
        .unwrap_or(times)
        .split_ascii_whitespace()
        .map(|num| num.parse().unwrap())
        .zip(
            distances
                .strip_prefix("Distance: ")
                .unwrap_or(distances)
                .split_ascii_whitespace()
                .map(|num| num.parse().unwrap()),
        )
        .collect()
}

fn count_ways_to_win(race_time: usize, distance_to_beat: usize) -> usize {
    (1..race_time)
        .map(|hold_time| {
            let remaining_time = race_time - hold_time;
            let race_distance = remaining_time * hold_time;
            if race_distance > distance_to_beat {
                1
            } else {
                0
            }
        })
        .sum::<usize>()
}

fn product_of_ways_to_win(times_table: TimesTable) -> usize {
    times_table
        .iter()
        .map(|(time, distance)| count_ways_to_win(*time, *distance))
        .product()
}

fn really_bad_kerning(input: &str) -> usize {
    let (time_str, distance_str) = input.split_once('\n').unwrap();
    let race_time = time_str
        .strip_prefix("Time: ")
        .unwrap_or(time_str)
        .matches(char::is_numeric)
        .collect::<Vec<_>>()
        .join("")
        .parse()
        .unwrap();
    let distance = distance_str
        .strip_prefix("Distance: ")
        .unwrap_or(distance_str)
        .matches(char::is_numeric)
        .collect::<Vec<_>>()
        .join("")
        .parse()
        .unwrap();
    count_ways_to_win(race_time, distance)
}

pub fn print_solution() {
    println!(
        "Product of number of ways to win: {win}",
        win = product_of_ways_to_win(parse_input(INPUT))
    );
    println!(
        "Number of ways to win with really bad kerning: {win}",
        win = really_bad_kerning(INPUT)
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const SAMPLE: &str = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
    "};

    #[test]
    fn test_parse() {
        assert_eq!(parse_input(SAMPLE), vec![(7, 9), (15, 40), (30, 200)])
    }

    #[test]
    fn test_product_of_ways_to_win() {
        assert_eq!(product_of_ways_to_win(parse_input(SAMPLE)), 288);
    }

    #[test]
    fn test_really_bad_kerning() {
        assert_eq!(really_bad_kerning(SAMPLE), 71503)
    }
}
