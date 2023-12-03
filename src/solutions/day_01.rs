const INPUT: &str = include_str!("day_01.txt");

fn find_first_last_number(numbers: Vec<u32>) -> (u32, u32) {
    (*numbers.first().unwrap(), *numbers.last().unwrap())
}

fn digits_in_line(line: &str) -> Vec<u32> {
    line.chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn calibration_values(input: &str, tokenize_line: fn(&str) -> Vec<u32>) -> Vec<u32> {
    input
        .lines()
        .map(|line| {
            let tokenized = tokenize_line(line);
            let (first, last) = find_first_last_number(tokenized);

            first * 10 + last
        })
        .collect()
}

fn matches_string(chars: &[char], start: usize, expected_string: &str) -> bool {
    chars
        .get(start..start + expected_string.len())
        .map(|chars| chars.iter().collect::<String>())
        .unwrap_or(String::from(""))
        == *expected_string
}

fn tokenize(line: &str) -> Vec<u32> {
    let lowercase_line = line.to_lowercase();
    let chars = lowercase_line.chars().collect::<Vec<_>>();
    let mut ptr = 0;
    let mut result = vec![];
    while let Some(char) = chars.get(ptr) {
        match char {
            x if x.is_ascii_digit() => {
                result.push(x.to_digit(10).unwrap());
            }
            // Do not consume the ptr, because oneight counts as both 1 and 8
            'o' if matches_string(&chars, ptr, "one") => {
                result.push(1);
            }
            't' if matches_string(&chars, ptr, "two") => {
                result.push(2);
            }
            't' if matches_string(&chars, ptr, "three") => {
                result.push(3);
            }
            'f' if matches_string(&chars, ptr, "four") => {
                result.push(4);
            }
            'f' if matches_string(&chars, ptr, "five") => {
                result.push(5);
            }
            's' if matches_string(&chars, ptr, "six") => {
                result.push(6);
            }
            's' if matches_string(&chars, ptr, "seven") => {
                result.push(7);
            }
            'e' if matches_string(&chars, ptr, "eight") => {
                result.push(8);
            }
            'n' if matches_string(&chars, ptr, "nine") => {
                result.push(9);
            }
            _ => {}
        }
        ptr += 1
    }
    result
}

fn sum_calibration(input: &str, tokenize_line: fn(&str) -> Vec<u32>) -> u32 {
    calibration_values(input, tokenize_line).iter().sum()
}

pub fn print_solution() {
    println!(
        "Sum of all calibration values (simple): {sum}",
        sum = sum_calibration(INPUT, digits_in_line)
    );
    println!(
        "Sum of all calibration values (complex): {sum}",
        sum = sum_calibration(INPUT, tokenize)
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "};

    #[test]
    fn test_calibration_values() {
        assert_eq!(
            calibration_values(SAMPLE, digits_in_line),
            vec![12, 38, 15, 77]
        );
        assert_eq!(sum_calibration(SAMPLE, digits_in_line), 142);
    }

    const COMPLEX_SAMPLE: &str = indoc! {"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    "};

    #[test]
    fn test_complex_calibration_values() {
        assert_eq!(
            calibration_values(COMPLEX_SAMPLE, tokenize),
            vec![29, 83, 13, 24, 42, 14, 76]
        );
        assert_eq!(sum_calibration(COMPLEX_SAMPLE, tokenize), 281);
    }

    #[test]
    fn test_tokenize() {
        assert_eq!(tokenize("two1nine"), vec![2, 1, 9]);
        assert_eq!(
            tokenize("1onetwothreefourfivesixseveneight8nine"),
            vec![1, 1, 2, 3, 4, 5, 6, 7, 8, 8, 9]
        );
    }
}
