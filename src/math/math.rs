#![allow(dead_code)]

use rand::prelude::*;
use std::f64;

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = f64::consts::PI;

pub fn deg_to_rad(deg: f64) -> f64 {
    deg * PI / 180.0
}

pub fn sample_square<T: Rng>(rng: &mut T) -> (f64, f64) {
    let dist = rand::distr::Uniform::new(-0.5, 0.5).unwrap();
    let samp: (f64, f64) = (dist.sample(rng), dist.sample(rng));
    samp
}

pub fn lin_to_gamma(lin: f64) -> f64 {
    lin.sqrt()
}
