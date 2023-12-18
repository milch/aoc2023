use std::collections::VecDeque;

use itertools::Itertools;

const INPUT: &str = include_str!("day_15.txt");

fn hash(string: &str) -> usize {
    string
        .chars()
        .fold(0usize, |hash, char| ((hash + char as usize) * 17) % 256)
}

fn hash_sequence(sequence: &str) -> usize {
    sequence
        .chars()
        .filter(|c| *c != '\n')
        .join("")
        .split(',')
        .map(hash)
        .sum()
}

const DEFAULT_VEC: VecDeque<(&str, u8)> = VecDeque::new();
fn hashmap_algorithm(sequence: &str) -> usize {
    let mut boxes: [VecDeque<(&str, u8)>; 256] = [DEFAULT_VEC; 256];
    sequence.trim().split(',').for_each(|operation| {
        if operation.ends_with('-') {
            let label = &operation[0..operation.len() - 1];
            let box_idx = hash(label);
            if let Some(existing_idx) =
                (0..boxes[box_idx].len()).find(|i| boxes[box_idx][*i].0 == label)
            {
                boxes[box_idx].remove(existing_idx);
            }
        } else {
            let (label, num_str) = operation.split_once('=').unwrap();
            let num = num_str.parse().unwrap();
            let one_box = &mut boxes[hash(label)];
            match one_box.iter_mut().find(|(lbl, _)| *lbl == label) {
                Some(existing) => *existing = (label, num),
                None => one_box.push_back((label, num)),
            }
        };
    });

    boxes
        .iter()
        .enumerate()
        .map(|(box_number, one_box)| {
            one_box
                .iter()
                .enumerate()
                .map(|(lens_idx, lens)| (1 + box_number) * (1 + lens_idx) * lens.1 as usize)
                .sum::<usize>()
        })
        .sum()
}

pub fn print_solution() {
    println!("Checksum: {}", hash_sequence(INPUT));
    println!("Focusing power: {}", hashmap_algorithm(INPUT));
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    const SAMPLE: &str = "HASH";

    const SAMPLE_SEQ: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_hash() {
        assert_eq!(hash(SAMPLE), 52);
    }

    #[test]
    fn test_hash_sequence() {
        assert_eq!(hash_sequence(SAMPLE_SEQ), 1320);
    }

    #[test]
    fn test_hashmap_algorithm() {
        assert_eq!(hashmap_algorithm(SAMPLE_SEQ), 145);
    }
}
