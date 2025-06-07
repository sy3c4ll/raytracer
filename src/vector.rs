use std::ops::{
    Add, AddAssign, BitXor, BitXorAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg,
    Sub, SubAssign,
};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub const I: Self = Self::new(1., 0., 0.);
    pub const J: Self = Self::new(0., 1., 0.);
    pub const K: Self = Self::new(0., 0., 1.);

    pub const X: usize = 0;
    pub const Y: usize = 1;
    pub const Z: usize = 2;

    pub const fn unit(axis: usize) -> Self {
        Self {
            x: if axis == Self::X { 1. } else { 0. },
            y: if axis == Self::Y { 1. } else { 0. },
            z: if axis == Self::Z { 1. } else { 0. },
        }
    }
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub const fn from_array([x, y, z]: [f64; 3]) -> Self {
        Self { x, y, z }
    }
    pub const fn to_array(self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
    pub const fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
    pub const fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
    pub const fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
    pub const fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
    pub const fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
    pub const fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
    pub const fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
    pub const fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
    pub const fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub const fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
    pub const fn cross_assign(&mut self, rhs: Self) {
        *self = self.cross(rhs)
    }
    pub const fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
    pub const fn index(&self, index: usize) -> &f64 {
        match index % 3 {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => unreachable!(),
        }
    }
    pub const fn index_mut(&mut self, index: usize) -> &mut f64 {
        match index % 3 {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => unreachable!(),
        }
    }

    pub fn sq(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn abs(self) -> f64 {
        self.sq().sqrt()
    }
    pub fn norm(self) -> Self {
        self / self.abs()
    }
    pub fn rotate_on_axis(mut self, axis: usize, theta: f64) -> Self {
        let theta = theta.to_radians();
        let cos = theta.cos();
        let sin = theta.sin();

        let c1 = self[axis + 1];
        let c2 = self[axis + 2];

        self[axis + 1] = cos * c1 - sin * c2;
        self[axis + 2] = sin * c1 + cos * c2;

        self
    }
}

impl From<[f64; 3]> for Vector {
    fn from(item: [f64; 3]) -> Self {
        Self::from_array(item)
    }
}

impl From<Vector> for [f64; 3] {
    fn from(item: Vector) -> Self {
        item.to_array()
    }
}

impl Add for Vector {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.add(rhs)
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.add_assign(rhs)
    }
}

impl Sub for Vector {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.sub(rhs)
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        self.sub_assign(rhs)
    }
}

impl Mul<f64> for Vector {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        self.mul(rhs)
    }
}

impl MulAssign<f64> for Vector {
    fn mul_assign(&mut self, rhs: f64) {
        self.mul_assign(rhs)
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Self::Output {
        rhs.mul(self)
    }
}

impl Div<f64> for Vector {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        self.div(rhs)
    }
}

impl DivAssign<f64> for Vector {
    fn div_assign(&mut self, rhs: f64) {
        self.div_assign(rhs)
    }
}

impl Mul for Vector {
    type Output = f64;
    fn mul(self, rhs: Self) -> Self::Output {
        self.dot(rhs)
    }
}

impl BitXor for Vector {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        self.cross(rhs)
    }
}

impl BitXorAssign for Vector {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.cross_assign(rhs)
    }
}

impl Neg for Vector {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.neg()
    }
}

impl Index<usize> for Vector {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        self.index(index)
    }
}

impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.index_mut(index)
    }
}
