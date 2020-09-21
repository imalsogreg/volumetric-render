use std::sync::Arc;

use crate::ad::*;

pub type X = u32;

pub struct Covariance {
    m11: f64,
    m12: f64,
    m13: f64,
    m21: f64,
    m22: f64,
    m23: f64,
    m31: f64,
    m32: f64,
    m33: f64,
}

impl Covariance {
    pub fn unit() -> Covariance {
        Covariance {
            m11: 1.0,
            m12: 0.0,
            m13: 0.0,
            m21: 0.0,
            m22: 1.0,
            m23: 0.0,
            m31: 0.0,
            m32: 0.0,
            m33: 1.0,
        }
    }
}

pub fn g(mean: P3, covariance: Covariance) -> Val {
    let dist = Val::distance_from(mean);
    (-dist).exp()
}
