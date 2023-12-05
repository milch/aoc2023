use std::ops::Range;

const INPUT: &str = include_str!("day_05.txt");

fn numbers_str_to_vec(line: &str) -> Vec<usize> {
    line.split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

type Almanac = (Vec<usize>, Vec<Vec<(Range<usize>, Range<usize>)>>);

fn parse_input(input: &str) -> Almanac {
    let mut iter = input.lines();
    let seeds = numbers_str_to_vec(iter.next().unwrap().strip_prefix("seeds: ").unwrap());

    // Empty line
    iter.next();
    // first line is the kind
    iter.next();

    let mut result_ranges = vec![];
    loop {
        let mut ranges = vec![];
        loop {
            let line = iter.next().unwrap_or("");
            if line.trim().is_empty() {
                break;
            }
            ranges.push(numbers_str_to_vec(line));
        }

        let result = ranges
            .iter()
            .map(|range| {
                let dest_start = range.first().unwrap();
                let src_start = range.get(1).unwrap();
                let len = range.get(2).unwrap();
                (
                    *dest_start..*dest_start + *len,
                    *src_start..*src_start + *len,
                )
            })
            .collect::<Vec<_>>();
        result_ranges.push(result);

        if iter.next().is_none() {
            break;
        }
    }

    (seeds, result_ranges)
}

fn location_number_for_seed(seed: usize, overrides: &[Vec<(Range<usize>, Range<usize>)>]) -> usize {
    overrides.iter().fold(seed, |num, from_to| {
        from_to
            .iter()
            .find(|elem| elem.1.contains(&num))
            .map(|(dest_range, src_range)| {
                let offset = num - src_range.start;
                dest_range.start + offset
            })
            .unwrap_or(num)
    })
}

fn lowest_number_multi(almanac: &Almanac) -> usize {
    let seeds: Vec<_> = almanac
        .0
        .chunks(2)
        .flat_map(|seed_range| {
            if let [start, len] = *seed_range {
                (start..start + len).collect()
            } else {
                vec![]
            }
        })
        .collect();

    seeds
        .iter()
        .map(|seed| location_number_for_seed(*seed, &almanac.1))
        .min()
        .unwrap()
}

fn lowest_number_single(almanac: &Almanac) -> usize {
    almanac
        .0
        .iter()
        .map(|seed| location_number_for_seed(*seed, &almanac.1))
        .min()
        .unwrap()
}

pub fn print_solution() {
    let almanac = parse_input(INPUT);
    println!(
        "Lowest location number (single seeds): {num}",
        num = lowest_number_single(&almanac)
    );
    println!(
        "Lowest location number (multiple seeds): {num}",
        num = lowest_number_multi(&almanac)
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const SAMPLE: &str = indoc! {"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
    "};

    #[test]
    fn test_parse_input() {
        let parsed = parse_input(SAMPLE);
        assert_eq!(parsed.0, vec![79, 14, 55, 13]);
        assert_eq!(
            parsed.1,
            vec![
                vec![(50..52, 98..100), (52..52 + 48, 50..50 + 48)],
                vec![(0..37, 15..15 + 37), (37..39, 52..54), (39..39 + 15, 0..15)],
                vec![
                    (49..57, 53..61,),
                    (0..42, 11..53,),
                    (42..49, 0..7,),
                    (57..61, 7..11,),
                ],
                vec![(88..95, 18..25,), (18..88, 25..95,),],
                vec![(45..68, 77..100,), (81..100, 45..64,), (68..81, 64..77,),],
                vec![(0..1, 69..70,), (1..70, 0..69,),],
                vec![(60..97, 56..93,), (56..60, 93..97,),],
            ]
        );
    }

    #[test]
    fn test_lowest_number_single() {
        let almanac = parse_input(SAMPLE);
        assert_eq!(lowest_number_single(&almanac), 35)
    }

    #[test]
    fn test_lowest_number_multi() {
        let almanac = parse_input(SAMPLE);
        assert_eq!(lowest_number_multi(&almanac), 46)
    }
}
