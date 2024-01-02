use crate::utils::Vector3;
use itertools::Itertools;
use std::{fmt::Display, ops::RangeInclusive, str::FromStr};

const INPUT: &str = include_str!("day_24.txt");

struct Hailstone {
    p: Vector3<f64>,
    v: Vector3<f64>,
}

impl Hailstone {
    fn intersection_times(&self, other: &Hailstone) -> (f64, f64) {
        let t1 = (other.v.y * (self.p.x - other.p.x) + other.v.x * (other.p.y - self.p.y))
            / (other.v.x * self.v.y - self.v.x * other.v.y);
        let t2 = (self.v.y * (self.p.x - other.p.x) + self.v.x * (other.p.y - self.p.y))
            / (other.v.x * self.v.y - self.v.x * other.v.y);
        (t1, t2)
    }

    fn at(&self, t: f64) -> Vector3<f64> {
        Vector3 {
            x: self.p.x + self.v.x * t,
            y: self.p.y + self.v.y * t,
            z: self.p.z + self.v.z * t,
        }
    }

    fn add_v_xy<T: Into<f64>>(&self, v: (T, T)) -> Hailstone {
        Hailstone {
            p: self.p,
            v: Vector3 {
                x: self.v.x + v.0.into(),
                y: self.v.y + v.1.into(),
                z: self.v.z,
            },
        }
    }
}

impl Display for Hailstone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}, {}, {}, @ {}, {}, {}",
            self.p.x, self.p.y, self.p.z, self.v.x, self.v.y, self.v.z
        ))
    }
}

impl FromStr for Hailstone {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos_str, vel_str) = s
            .split_once(" @ ")
            .ok_or("Missing '@' delimiter between pos and velocity")?;

        Ok(Hailstone {
            p: pos_str.parse()?,
            v: vel_str.parse()?,
        })
    }
}

fn times_normal(times: (f64, f64)) -> bool {
    times.0.is_finite() && times.1.is_finite() && times.0 > 0.0 && times.1 > 0.0
}

fn check_intersections(hailstones: &[Hailstone], in_range: RangeInclusive<f64>) -> usize {
    hailstones
        .iter()
        .tuple_combinations()
        .filter(|(l, r)| {
            let times = l.intersection_times(r);
            if times_normal(times) {
                let pt = l.at(times.0);
                in_range.contains(&pt.x) && in_range.contains(&pt.y)
            } else {
                false
            }
        })
        .count()
}

fn find_rock_throw(hailstones: &[Hailstone]) -> f64 {
    let first = &hailstones[0];
    let second = &hailstones[1];
    let search_range = -350..350;
    let rock = (search_range.clone())
        .cartesian_product(search_range.clone())
        .filter(|(vx, vy)| {
            let first_mod = first.add_v_xy((-*vx, -*vy));
            hailstones[1..4]
                .iter()
                .flat_map(|stone| {
                    let other_mod = stone.add_v_xy((-*vx, -*vy));
                    let times = first_mod.intersection_times(&other_mod);
                    let first_intersect = first_mod.at(times.0);
                    let other_intersect = other_mod.at(times.1);
                    [
                        (first_intersect.x, first_intersect.y),
                        (other_intersect.x, other_intersect.y),
                    ]
                })
                .all_equal()
        })
        .find_map(|(vx, vy)| {
            let t = first
                .add_v_xy((-vx, -vy))
                .intersection_times(&second.add_v_xy((-vx, -vy)));
            let (t2, t1) = if t.0 > t.1 { (t.1, t.0) } else { (t.0, t.1) };
            let diff = t1 - t2;
            let z = first.at(t1).z - second.at(t2).z;
            let vz = z / diff;

            let first_intersection = first.at(t.0);
            let rock = Hailstone {
                p: Vector3 {
                    x: first_intersection.x - (vx as f64) * t.0,
                    y: first_intersection.y - (vy as f64) * t.0,
                    z: first_intersection.z - vz * t.0,
                },
                v: Vector3 {
                    x: (vx).into(),
                    y: (vy).into(),
                    z: vz,
                },
            };

            hailstones[1..]
                .iter()
                .all(|stone| {
                    let times = rock.intersection_times(stone);
                    (times.0 - times.1).abs() <= 0.01
                })
                .then_some(rock)
        })
        .expect("No rock found :(");

    rock.p.x + rock.p.y + rock.p.z
}

pub fn print_solution() {
    let hailstones = INPUT
        .lines()
        .map(Hailstone::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    println!(
        "Number of intersections in xy plane: {}",
        check_intersections(&hailstones, 200000000000000.0..=400000000000000.0)
    );

    println!(
        "Sum of coordinates of rock position: {}",
        find_rock_throw(&hailstones)
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const SAMPLE: &str = indoc! {"
        19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3
    "};

    #[test]
    fn test_intersect() {
        let hailstones = SAMPLE
            .lines()
            .map(Hailstone::from_str)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(check_intersections(&hailstones, 7.0..=27.0), 2);
    }

    #[test]
    fn test_rock_throw() {
        let hailstones = SAMPLE
            .lines()
            .map(Hailstone::from_str)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(find_rock_throw(&hailstones), 47.0);
    }
}
