use itertools::Itertools;

use crate::utils::{Point3D, RangeIntersection, Vector3};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::{Add, RangeInclusive},
    str::FromStr,
};

const INPUT: &str = include_str!("day_22.txt");

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct Shape {
    start: Point3D,
    end: Point3D,
}

macro_rules! impl_range {
    ($($x:expr), *) => {
        $( paste::paste! {
            fn [<$x _range>](&self) -> RangeInclusive<isize> {
                if self.start.$x < self.end.$x {
                    self.start.$x..=self.end.$x
                } else {
                    self.end.$x..=self.start.$x
                }
            }
        })*
    };
}

impl Shape {
    impl_range!(x, y, z);

    fn overlaps(&self, other: &Shape) -> bool {
        self.x_range().intersect(&other.x_range()).is_some()
            && self.y_range().intersect(&other.y_range()).is_some()
            && (self.z_range()).intersect(&other.z_range()).is_some()
    }
}

impl FromStr for Shape {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once('~').ok_or("No delimiter found")?;
        let l: Point3D = a.parse()?;
        let r: Point3D = b.parse()?;
        let (start, end) = if l.z < r.z { (l, r) } else { (r, l) };
        Ok(Self { start, end })
    }
}

impl Add<Vector3<isize>> for Shape {
    type Output = Self;

    fn add(self, offset: Vector3<isize>) -> Self::Output {
        Shape {
            start: self.start + offset,
            end: self.end + offset,
        }
    }
}

fn parse_shapes(input: &str) -> Vec<Shape> {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect_vec()
}

fn z_sort(input: &[Shape]) -> Vec<Shape> {
    input
        .iter()
        .cloned()
        .sorted_by_key(|f| f.start.z)
        .collect_vec()
}

fn drop_shapes(input: &[Shape]) -> Vec<Shape> {
    let sorted = z_sort(input);
    let mut result: Vec<Shape> = Vec::from([sorted[0]]);
    let mut z_map: HashMap<isize, Vec<Shape>> = HashMap::from([(sorted[0].end.z, vec![sorted[0]])]);
    for shape in sorted.iter().skip(1) {
        let first_overlaps = (1..shape.start.z)
            .map_while(|i| {
                let new = *shape + (Point3D::Z_DOWN * i);
                match z_map.get(&new.start.z) {
                    Some(z_supports) => z_supports.iter().all(|e| !e.overlaps(&new)).then_some(new),
                    None => Some(new),
                }
            })
            .last();
        let to_insert = first_overlaps.unwrap_or(*shape);
        result.push(to_insert);
        z_map.entry(to_insert.end.z).or_default().push(to_insert);
    }

    result.into_iter().rev().collect_vec()
}

type Adjacencies = HashMap<Shape, HashSet<Shape>>;
fn build_adjacency_lists(shapes: &[Shape]) -> (Adjacencies, Adjacencies) {
    let z_map = shapes.iter().into_group_map_by(|shape| shape.start.z);
    let z_ends = shapes.iter().into_group_map_by(|shape| shape.end.z);
    let bottom_to_top: HashMap<_, _> = z_ends
        .iter()
        .flat_map(|(z, bottom_shapes)| {
            let top_shapes = z_map.get(&(*z + 1));
            bottom_shapes
                .iter()
                .map(|bottom| match top_shapes {
                    Some(top_shapes) => (
                        **bottom,
                        top_shapes
                            .iter()
                            .filter(|top| {
                                let moved_down = ***top + Point3D::Z_DOWN;
                                bottom.overlaps(&moved_down)
                            })
                            .cloned()
                            .cloned()
                            .collect_vec(),
                    ),
                    None => (**bottom, vec![]),
                })
                .collect_vec()
        })
        .collect();

    let top_to_bottom: HashMap<_, _> = bottom_to_top
        .iter()
        .flat_map(|(a, b)| b.iter().map(|c| (*c, *a)))
        .into_group_map_by(|d| d.0)
        .iter()
        .map(|(k, v)| (*k, v.iter().map(|(_, t)| *t).collect::<HashSet<Shape>>()))
        .collect();

    (
        bottom_to_top
            .iter()
            .map(|(k, v)| (*k, v.iter().cloned().collect()))
            .collect(),
        top_to_bottom,
    )
}

fn find_disintegratable(
    shapes: &[Shape],
    bottom_to_top: &Adjacencies,
    top_to_bottom: &Adjacencies,
) -> HashSet<Shape> {
    shapes
        .iter()
        .filter(|bottom| {
            let top_shapes = bottom_to_top.get(bottom);
            match top_shapes {
                Some(top_shapes) => {
                    // A shape can be disintegrated if ALL of the shapes it holds up have ANY other shape that holds it up => it won't cause anything else to fall
                    top_shapes
                        .iter()
                        .all(|top| top_to_bottom.get(top).map(|t| t.len()).unwrap_or(0) > 1)
                }
                // No shapes above it -> we can count these!
                None => true,
            }
        })
        .cloned()
        .collect()
}

fn count_chain_reaction(
    all_shapes: &[Shape],
    disintegratable_shapes: &HashSet<Shape>,
    bottom_to_top: &Adjacencies,
    top_to_bottom: &Adjacencies,
) -> usize {
    all_shapes
        .iter()
        .filter(|shape| !disintegratable_shapes.contains(shape))
        .map(|shape| {
            let mut queue = VecDeque::from_iter(
                bottom_to_top[&shape]
                    .iter()
                    .filter(|top| top_to_bottom[top].len() == 1),
            );
            let mut seen = HashSet::from([shape]);
            while let Some(bottom) = queue.pop_front() {
                if !seen.insert(bottom) {
                    continue;
                }
                for top in &bottom_to_top[&bottom] {
                    if seen.is_superset(&top_to_bottom[top].iter().collect()) {
                        queue.push_back(top);
                    }
                }
            }
            seen.remove(shape);
            seen.len()
        })
        .sum()
}

pub fn print_solution() {
    let dropped = drop_shapes(&parse_shapes(INPUT));
    println!("Dropped");
    let (bottom_to_top, top_to_bottom) = build_adjacency_lists(&dropped);
    let disintegratable = find_disintegratable(&dropped, &bottom_to_top, &top_to_bottom);
    println!("Number of disintegratable boxes: {}", disintegratable.len());
    println!(
        "Number of  {}",
        count_chain_reaction(&dropped, &disintegratable, &bottom_to_top, &top_to_bottom)
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const SAMPLE: &str = indoc! {"
        2,0,5~2,2,5
        0,1,6~2,1,6
        1,1,8~1,1,9
        1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
    "};

    // This one has two indepedendent sub-graphs, where on one of them there are
    // two nodes at the bottom supporting the rest of the tower. Everything up
    // from that is the same.
    const THE_TWO_TOWERS: &str = indoc! {"
        2,0,5~2,2,5
        0,1,6~2,1,6
        1,1,8~1,1,9
        1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
        5,5,1~5,7,1
        6,5,1~6,7,1
        5,5,2~7,5,2
        5,7,3~7,7,3
        5,5,4~5,7,4
        7,5,5~7,7,5
        5,6,6~7,6,6
        6,6,8~6,6,9
    "};

    const DROPPED: &str = indoc! {"
       1,1,5~1,1,6
       0,1,4~2,1,4
       2,0,3~2,2,3
       0,0,3~0,2,3
       0,2,2~2,2,2
       0,0,2~2,0,2
       1,0,1~1,2,1
    "};

    #[test]
    fn test_parse() {
        let x = Shape {
            start: Point3D::new(0, 0, 0),
            end: Point3D::new(0, 0, 2),
        };
        assert_eq!(x, "0,0,0~0,0,2".parse().unwrap());
    }

    #[test]
    fn test_dropping_shapes() {
        let shapes = parse_shapes(SAMPLE);
        let dropped = drop_shapes(&shapes);
        assert_eq!(
            dropped.iter().sorted_by_key(|x| x.start.z).collect_vec(),
            parse_shapes(DROPPED)
                .iter()
                .sorted_by_key(|x| x.start.z)
                .collect_vec()
        );
    }

    #[test]
    fn test_overlaps() {
        let a: Shape = "0,0,0~0,0,2".parse().unwrap();
        let b: Shape = "1,1,1~1,1,3".parse().unwrap();
        assert!(!a.overlaps(&b));
        assert!(!b.overlaps(&a));
        let a: Shape = "0,2,2~2,2,2".parse().unwrap();
        let b: Shape = "0,0,2~2,0,2".parse().unwrap();
        assert!(!a.overlaps(&b));
        assert!(!b.overlaps(&a));
        let a: Shape = "1,1,0~1,1,1".parse().unwrap();
        let b: Shape = "0,1,0~2,1,0".parse().unwrap();
        assert!(b.overlaps(&a));
        assert!(a.overlaps(&b));
    }

    #[test]
    fn test_find_disintegratable() {
        let dropped = drop_shapes(&parse_shapes(SAMPLE));
        let (bottom_to_top, top_to_bottom) = build_adjacency_lists(&dropped);
        assert_eq!(
            find_disintegratable(&dropped, &bottom_to_top, &top_to_bottom).len(),
            5
        );
    }

    #[test]
    fn test_chain_reaction() {
        let shapes = drop_shapes(&parse_shapes(SAMPLE));
        let (bottom_to_top, top_to_bottom) = build_adjacency_lists(&shapes);
        let disintegratable = find_disintegratable(&shapes, &bottom_to_top, &top_to_bottom);
        assert_eq!(
            count_chain_reaction(&shapes, &disintegratable, &bottom_to_top, &top_to_bottom),
            7
        );
        let shapes = drop_shapes(&parse_shapes(THE_TWO_TOWERS));
        let (bottom_to_top, top_to_bottom) = build_adjacency_lists(&shapes);
        let disintegratable = find_disintegratable(&shapes, &bottom_to_top, &top_to_bottom);
        assert_eq!(
            count_chain_reaction(&shapes, &disintegratable, &bottom_to_top, &top_to_bottom),
            8
        );
    }
}
