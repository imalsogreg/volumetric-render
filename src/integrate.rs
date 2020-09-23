use euclid::Scale;
use euclid::Point2D;
use rayon::prelude::*;
use frustum::{Frustum, Vec3, WorldSpace};

use crate::ad::*;

pub fn render(
    v: Val,
    frustum: Frustum,
    distance: f64,
    xy_step: f64,
    depth_step: f64,
) -> Vec<(f64, f64, f64)> {
    let n_rows = (frustum.height as f64 / xy_step).floor() as u32;
    let n_cols = (frustum.width  as f64 / xy_step).floor() as u32;
    let ks : Vec<u32> = (0..(n_rows as u32 * n_cols as u32)).into_iter().collect();
    ks.into_par_iter().map(move |k : u32| {
        let x = (k % n_cols) as f64;
        let y = (k / n_cols) as f64;
        let (start, direction) = frustum.ray_from_ncp(&Point2D::new(x,y)).unwrap();
        let v = integrate(&v, start, direction, distance, depth_step);
        (x,y,v)
    }).collect()
}

pub fn integrate(v: &Val, start: P3, direction: Vec3<WorldSpace>, distance: f64, step: f64) -> f64 {
    let n_steps = (distance / step).floor() as u32;
    let steps : Vec<u32> = (0..n_steps).into_iter().collect();
    steps
        .iter()
        .map(move |i| {
            let x: P3 = start + direction * Scale::new(step * (*i) as f64);
            let fx = v.f(x);
            fx * step
        })
        .sum()
}
