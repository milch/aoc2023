use num::Integer;
use std::{collections::HashMap, str::FromStr};

const INPUT: &str = include_str!("day_08.txt");

struct MapInstruction {
    left: String,
    right: String,
}

struct Map {
    instructions: Vec<char>,
    path: HashMap<String, MapInstruction>,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let instructions = match lines.next() {
            Some(val) => val.chars().collect(),
            None => return Err(()),
        };

        // empty line
        lines.next();

        let path = lines.fold(HashMap::new(), |mut map, line| {
            let (from, branches) = line.split_once(" = ").unwrap();

            let (left, right) = branches.split_once(", ").unwrap();
            map.insert(
                from.to_string(),
                MapInstruction {
                    left: left.trim_start_matches('(').to_string(),
                    right: right.trim_end_matches(')').to_string(),
                },
            );

            map
        });
        Ok(Map { instructions, path })
    }
}

fn count_walks(map: &Map, start: &String, end_condition: fn(&String) -> bool) -> usize {
    let mut current = start;
    map.instructions
        .iter()
        .cycle()
        .take_while(|ins| {
            let way = map.path.get(current).unwrap();
            current = match **ins {
                'L' => &way.left,
                'R' => &way.right,
                _ => panic!(),
            };
            end_condition(current)
        })
        .count()
        + 1 // Need + 1 because take_while doesn't yield the last element
}

fn count_ghost_walks(map: &Map) -> usize {
    let starting_nodes = map.path.keys().filter(|key| key.ends_with('A'));
    let steps_to_z = starting_nodes.map(|node| count_walks(map, node, |str| !str.ends_with('Z')));
    steps_to_z.reduce(|lcm, num| lcm.lcm(&num)).unwrap()
}

pub fn print_solution() {
    let map: Map = INPUT.parse().unwrap();
    println!(
        "Steps required to get to ZZZ: {steps}",
        steps = count_walks(&map, &"AAA".to_string(), |str| str != "ZZZ")
    );
    println!(
        "Steps required as ghost: {steps}",
        steps = count_ghost_walks(&map)
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const SAMPLE_1: &str = indoc! {"
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ
    "};

    const SAMPLE_2: &str = indoc! {"
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ
    "};

    #[test]
    fn test_count_walks() {
        let start = "AAA".to_string();
        assert_eq!(
            count_walks(&SAMPLE_1.parse().unwrap(), &start, |str| str != "ZZZ"),
            2
        );
        assert_eq!(
            count_walks(&SAMPLE_2.parse().unwrap(), &start, |str| str != "ZZZ"),
            6
        );
    }

    const SAMPLE_GHOSTS: &str = indoc! {"
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
    "};

    #[test]
    fn test_count_ghost_walks() {
        assert_eq!(count_ghost_walks(&SAMPLE_GHOSTS.parse().unwrap()), 6);
    }
}
