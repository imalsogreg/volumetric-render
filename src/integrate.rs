use euclid::Scale;
use euclid::Point2D;
use frustum::{Frustum, Vec3, WorldSpace};

use crate::ad::*;

pub fn render(
    v: Val,
    frustum: Frustum,
    distance: f64,
    xy_step: f64,
    depth_step: f64,
) -> impl Iterator<Item = (f64, f64, f64)> {
    let n_rows = (frustum.height as f64 / xy_step).floor() as usize;
    let n_cols = (frustum.width  as f64 / xy_step).floor() as usize;
    (0..(n_rows as usize * n_cols as usize)).map(move |k : usize| {
        let x = (k % n_cols) as f64;
        let y = (k / n_cols) as f64;
        let (start, direction) = frustum.ray_from_ncp(&Point2D::new(x,y)).unwrap();
        let v = integrate(&v, start, direction, distance, depth_step);
        (x,y,v)
    }).into_iter()
}

pub fn integrate(v: &Val, start: P3, direction: Vec3<WorldSpace>, distance: f64, step: f64) -> f64 {
    let n_steps = (distance / step).floor() as u32;
    (0..n_steps)
        .map(|i| {
            let x: P3 = start + direction * Scale::new(step * i as f64);
            let fx = v.f(x);
            fx * step
        })
        .sum()
}
