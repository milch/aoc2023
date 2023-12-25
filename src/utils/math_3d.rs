use std::{
    fmt::Display,
    ops::{Add, Mul, Sub},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vector3<Idx> {
    pub(crate) x: Idx,
    pub(crate) y: Idx,
    pub(crate) z: Idx,
}

impl<Idx> Vector3<Idx> {
    pub fn new(x: Idx, y: Idx, z: Idx) -> Self {
        Self { x, y, z }
    }
}

impl<Idx> Display for Vector3<Idx>
where
    Idx: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

// Parse vector from a string of the format "x,y,z"
impl<Idx> FromStr for Vector3<Idx>
where
    Idx: FromStr,
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts
            .next()
            .and_then(|x| x.parse().ok())
            .ok_or("Error parsing x value")?;
        let y = parts
            .next()
            .and_then(|y| y.parse().ok())
            .ok_or("Error parsing y value")?;
        let z = parts
            .next()
            .and_then(|z| z.parse().ok())
            .ok_or("Error parsing z value")?;
        Ok(Self::new(x, y, z))
    }
}

impl<Idx: Add<Output = Idx>> Add for Vector3<Idx> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<Idx: Sub<Output = Idx>> Sub for Vector3<Idx> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl<Idx: Mul<Output = Idx> + Copy> Mul<Idx> for Vector3<Idx> {
    type Output = Self;
    fn mul(self, other: Idx) -> Self {
        Self::new(self.x * other, self.y * other, self.z * other)
    }
}

pub type Point3D = Vector3<isize>;

impl Point3D {
    pub const Z_UP: Self = Self { x: 0, y: 0, z: 1 };
    pub const Z_DOWN: Self = Self { x: 0, y: 0, z: -1 };
}
