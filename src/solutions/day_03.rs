const INPUT: &str = include_str!("day_03.txt");

fn sum_adjacent(input: &str) -> u32 {
    let matrix: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    let rows = matrix.len();
    let cols = matrix.first().unwrap().len();
    let mut sum = 0;
    for row in 0..rows {
        let mut col = 0;
        while col < cols {
            if matrix[row][col].is_ascii_digit() {
                let row_above = if row > 1 { row - 1 } else { 0 };
                let row_below = (row + 1).min(rows - 1);
                let col_left = if col > 1 { col - 1 } else { 0 };
                let mut col_right = (col + 1).min(cols - 1);
                let mut nums = vec![matrix[row][col]];
                while col_right < cols && matrix[row][col_right].is_ascii_digit() {
                    nums.push(matrix[row][col_right]);
                    col_right += 1
                }
                col_right = if col_right < cols {
                    col_right
                } else {
                    cols - 1
                };

                if matrix[row_above..=row_below].iter().any(|r| {
                    r[col_left..=col_right]
                        .iter()
                        .any(|char| char.is_ascii_punctuation() && *char != '.')
                }) {
                    sum += String::from_iter(nums).parse::<u32>().unwrap();
                }
                col = col_right + 1
            } else {
                col += 1;
            }
        }
    }
    sum
}

fn sum_gear_ratios(input: &str) -> u32 {
    let matrix: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    let mut r = 0;
    let mut all_numbers = vec![];
    let mut stars = vec![];
    matrix.iter().for_each(|row| {
        let mut c = 0;
        // row, start_index..end_index, number
        let mut current_number = (r, c, c, vec![]);
        row.iter().for_each(|chr| {
            if chr.is_ascii_digit() {
                if current_number.3.is_empty() {
                    current_number.1 = c;
                }
                current_number.3.push(chr);
                current_number.2 = c;
            } else if !current_number.3.is_empty() {
                all_numbers.push((
                    current_number.0,
                    current_number.1..=current_number.2,
                    String::from_iter(current_number.3.clone())
                        .parse::<u32>()
                        .unwrap(),
                ));
                current_number = (r, c, c, vec![]);
            }
            if *chr == '*' {
                stars.push((r, c));
            }
            c += 1;
        });
        if !current_number.3.is_empty() {
            all_numbers.push((
                current_number.0,
                current_number.1..=current_number.2,
                String::from_iter(current_number.3.clone())
                    .parse::<u32>()
                    .unwrap(),
            ));
        }
        r += 1;
    });

    stars
        .iter()
        .map(|(r, c)| {
            let row_range = (r - 1)..=(r + 1);
            let adjacent_numbers = all_numbers
                .iter()
                .filter(|num| {
                    row_range.contains(&num.0)
                        && (num.1.contains(&(c - 1))
                            || num.1.contains(c)
                            || num.1.contains(&(c + 1)))
                })
                .map(|num| num.2)
                .collect::<Vec<_>>();
            if adjacent_numbers.len() == 2 {
                adjacent_numbers.iter().product()
            } else {
                0
            }
        })
        .sum()
}

pub fn print_solution() {
    println!("Sum of adjacent numbers: {sum}", sum = sum_adjacent(INPUT));
    println!("Sum of gear ratios: {sum}", sum = sum_gear_ratios(INPUT));
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        467..114.
        ...*.....
        ..35..633
        ......#..
        617*.....
        .....+.58
        ..592....
        ......755
        ...$.*...
        .664.598.
    "};

    #[test]
    fn test_sum_adjacent() {
        assert_eq!(sum_adjacent(SAMPLE), 4361);
    }

    #[test]
    fn test_sum_gear_ratios() {
        assert_eq!(sum_gear_ratios(SAMPLE), 467835);
    }
}
