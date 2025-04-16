#![allow(dead_code)]

use derive_more::{Add, AddAssign, Constructor, Debug, Display, Neg, Sub};
use image::Rgb;
use std::ops::{Div, DivAssign, Mul, MulAssign};

#[derive(Copy, Debug, Display, Add, AddAssign, Neg, Constructor, Clone, Sub)]
#[display("[ {} {} {} ]", x, y, z)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        let mut newvec = self.clone();
        newvec *= rhs;
        newvec
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Vec3 {
        self * (1.0 / rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl Vec3 {
    pub fn length(&self) -> f64 {
        self.norm().sqrt()
    }

    pub fn norm(&self) -> f64 {
        let x = self.x;
        let y = self.y;
        let z = self.z;

        x * x + y * y + z * z
    }

    pub fn unit(&self) -> Vec3 {
        let l = self.length();
        Vec3::new(self.x / l, self.y / l, self.z / l)
    }
}

pub fn dot(left: Vec3, right: Vec3) -> f64 {
    left.x * right.x + left.y * right.y + left.z * right.z
}
pub fn cross(left: Vec3, right: Vec3) -> Vec3 {
    Vec3 {
        x: left.y * right.z - left.z * right.y,
        y: left.z * right.x - left.x * right.z,
        z: left.x * right.y - left.y * right.x,
    }
}

// Color values are from 0.0 to 1.0
#[derive(Debug, Display, Add)]
#[display("{{ {} {} {} }}", r, g, b)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }
}

impl Into<Rgb<u8>> for Color {
    fn into(self) -> Rgb<u8> {
        let ir = (255.999 * self.r) as u8;
        let ig = (255.999 * self.g) as u8;
        let ib = (255.999 * self.b) as u8;
        Rgb([ir, ig, ib])
    }
}

impl Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b,
        }
    }
}
