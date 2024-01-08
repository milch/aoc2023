use std::{
    collections::{BTreeSet, VecDeque},
    rc::Rc,
};

use itertools::Itertools;
use rand::Rng;

const INPUT: &str = include_str!("day_25.txt");

fn parse_edges(input: &str) -> Vec<(Rc<str>, Rc<str>)> {
    input
        .lines()
        .flat_map(|line| {
            let (start, ends_str) = line.split_once(": ").expect("Missing delimiter ': '");
            ends_str
                .split_ascii_whitespace()
                .map(|end| (start.into(), end.into()))
        })
        .collect()
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Color {
    Red,
    White,
}

fn to_adjacencies(vertices: &[Rc<str>], edges: &[(Rc<str>, Rc<str>)]) -> Vec<Vec<usize>> {
    let edges_indices = edges
        .iter()
        .map(|(l, r)| {
            (
                vertices.iter().position(|e| l == e).unwrap(),
                vertices.iter().position(|e| e == r).unwrap(),
            )
        })
        .collect_vec();

    vertices
        .iter()
        .enumerate()
        .map(|(idx, _)| {
            edges_indices
                .iter()
                .filter_map(|(l, r)| {
                    (*l == idx)
                        .then_some(*r)
                        .or_else(|| (*r == idx).then_some(*l))
                })
                .collect_vec()
        })
        .collect()
}

fn form_groups(edges: &[(Rc<str>, Rc<str>)]) -> usize {
    let vertices = edges
        .iter()
        .flat_map(|(l, r)| [l, r])
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect_vec();

    let adj_list = to_adjacencies(&vertices, edges);

    let mut rng = rand::thread_rng();
    loop {
        let mut colors = vec![None; vertices.len()];
        let first = rng.gen_range(0..vertices.len());
        let second = loop {
            let n = rng.gen_range(0..vertices.len());
            if n != first {
                break n;
            }
        };
        let mut queue = VecDeque::new();
        queue.push_back((first, Color::White));
        queue.push_back((second, Color::Red));
        while let Some((idx, color)) = queue.pop_front() {
            colors[idx] = Some(color);

            adj_list[idx].iter().for_each(|other_idx| {
                if colors[*other_idx].is_none() {
                    queue.push_back((*other_idx, color));
                }
            });
        }

        let cut_count = adj_list
            .iter()
            .enumerate()
            .map(|(left, neighbors)| {
                let left_color = colors[left].unwrap();
                neighbors
                    .iter()
                    .map(|right| {
                        if colors[*right].unwrap() == left_color {
                            0
                        } else {
                            1
                        }
                    })
                    .sum::<usize>()
            })
            .sum::<usize>();
        // We double count everything because the adjacency list has both
        // forwards and backwards adjacencies
        if cut_count == 6 {
            let white_count = colors
                .iter()
                .filter(|&&c| c == Some(Color::White))
                .collect_vec()
                .len();
            return white_count * (vertices.len() - white_count);
        }
    }
}

pub fn print_solution() {
    let wires = parse_edges(INPUT);
    println!("Subgraphs: {}", form_groups(&wires))
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const SAMPLE: &str = indoc! {"
        jqt: rhn xhk nvd
        rsh: frs pzl lsr
        xhk: hfx
        cmg: qnr nvd lhk bvb
        rhn: xhk bvb hfx
        bvb: xhk hfx
        pzl: lsr hfx nvd
        qnr: nvd
        ntq: jqt hfx bvb xhk
        nvd: lhk
        lsr: lhk
        rzs: qnr cmg lsr rsh
        frs: qnr lhk lsr
    "};

    #[test]
    fn test_form_groups() {
        let wires = parse_edges(SAMPLE);
        assert_eq!(form_groups(&wires), 54)
    }
}
