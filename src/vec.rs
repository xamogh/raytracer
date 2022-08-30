use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign},
};
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64 , e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    // dot product of two vectors
    // to see how similar to vectors are i.e find angle between them
    // if dot product = [0 => perpendicular], [1 => parallel], [-1, opposite]
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    // cross product of two vectors
    // to calculate a vector perpendicular from given two vectors
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.y() * other.z() - self.z() * other.y(),
                self.z() * other.x() - self.x() * other.z(),
                self.x() * other.y() - self.y() * other.x(),
            ],
        }
    }

    // magnitude of vector
    pub fn length(&self) -> f64 {
        self.dot(self).sqrt()
    }

    // unit vectors, division uses operator overloading i guess? :#
    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn write_color(&self) -> () {
        println!(
            "{} {} {}",
            (255.999 * self.x()) as u64,
            (255.999 * self.y()) as u64,
            (255.999 * self.z()) as u64
        );
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &f64 {
        &self.e[index]
    }
}
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        &mut self.e[index]
    }
}

// vec + vec
impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()],
        }
    }
}

// vec += vec
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) -> () {
        *self = Vec3 {
            e: [self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()],
        }
    }
}

// vec - vec
impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Vec3 {
        Vec3 {
            e: [self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z()],
        }
    }
}

// vec -= vec
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) -> () {
        *self = Vec3 {
            e: [self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z()],
        }
    }
}

// vec * f64
impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            e: [self.x() * rhs, self.y() * rhs, self.z() * rhs],
        }
    }
}

// vec *= f64
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Vec3 {
            e: [self.x() * rhs, self.y() * rhs, self.z() * rhs],
        }
    }
}

// f64 * vec
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [self * rhs.x(), self * rhs.y(), self * rhs.z()],
        }
    }
}

// vec / f64
impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Vec3 {
        Vec3 {
            e: [self.x() / rhs, self.y() / rhs, self.z() / rhs],
        }
    }
}

// vec /= f64
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) -> () {
        *self = Vec3 {
            e: [self.x() / rhs, self.y() / rhs, self.x() / rhs],
        }
    }
}
