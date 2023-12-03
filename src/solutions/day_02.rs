use std::collections::HashMap;

const INPUT: &str = include_str!("day_02.txt");

fn parse_games(input: &str) -> Vec<(u32, Vec<HashMap<&str, u32>>)> {
    input
        .lines()
        .map(|game| {
            let mut game_parts = game.split(": ");
            let game_and_id = game_parts.next().unwrap();
            let id: u32 = game_and_id.split(' ').last().unwrap().parse().unwrap();
            let sets = game_parts.next().unwrap();
            (
                id,
                sets.split("; ")
                    .map(|set| {
                        set.split(", ")
                            .map(|cube| {
                                let mut parts = cube.split(' ');
                                let count: u32 = parts.next().unwrap().parse().unwrap();
                                let color = parts.next().unwrap();
                                (color, count)
                            })
                            .collect::<HashMap<_, _>>()
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>()
}

fn sum_possible_games(input: &str, possible_values: &HashMap<&str, u32>) -> u32 {
    let games = parse_games(input);

    games
        .iter()
        .filter(|(_, game)| {
            game.iter().all(|set| {
                set.iter()
                    .all(|(color, count)| matches!((possible_values.get(color), count), (Some(possible), count) if possible >= count))
            })
        })
        .map(|(id, _)| id)
        .sum()
}

fn sum_cube_power(input: &str) -> u32 {
    let games = parse_games(input);

    games
        .iter()
        .map(|(_, game)| {
            game.iter()
                .fold(HashMap::new(), |mut acc, set| {
                    set.iter().for_each(|(color, count)| {
                        let entry = acc.entry(*color).or_insert(*count);
                        *entry = (*entry).max(*count);
                    });
                    acc
                })
                .values()
                .product::<u32>()
        })
        .sum()
}

pub fn print_solution() {
    println!(
        "Sum: {s}",
        s = sum_possible_games(
            INPUT,
            &HashMap::from([("blue", 14), ("red", 12), ("green", 13)])
        )
    );
    println!("Power: {s}", s = sum_cube_power(INPUT,));
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "};

    #[test]
    fn test_sum_possible_games() {
        assert_eq!(
            sum_possible_games(
                SAMPLE,
                &HashMap::from([("blue", 14), ("red", 12), ("green", 13)])
            ),
            8
        )
    }

    #[test]
    fn test_sum_cube_power() {
        assert_eq!(sum_cube_power(SAMPLE), 2286);
    }
}
