#[macro_use]
use thiserror::Error;
use nalgebra::Matrix3;
use std::sync::Arc;

use crate::ad::*;

#[derive(Error, Debug)]
pub enum GaussianError {
    #[error("Could not invert covariance matrix")]
    UninvertableCovarianceMatrix,
}

pub fn unit_matrix() -> Matrix3<f64> {
    Matrix3::new(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0)
}

pub fn example_matrix() -> Matrix3<f64> {
    Matrix3::new(2.0, 1.9, 1.9, 1.9, 2.0, 1.9, 1.9, 1.9, 2.0)
}

pub fn g(mean: P3, covariance: Matrix3<f64>) -> Result<Val, GaussianError> {
    let inverse_covariance = covariance
        .try_inverse()
        .ok_or(GaussianError::UninvertableCovarianceMatrix)?;

    Ok(Val {
        v: Arc::new(move |x: P3| {

            let dist_x = x.x - mean.x;
            let dist_y = x.y - mean.y;
            let dist_z = x.z - mean.z;

            // nalgebra matricies are stored in column-major order
            let m11 = inverse_covariance[0];
            let m21 = inverse_covariance[1];
            let m31 = inverse_covariance[2];
            let m12 = inverse_covariance[3];
            let m22 = inverse_covariance[4];
            let m32 = inverse_covariance[5];
            let m13 = inverse_covariance[6];
            let m23 = inverse_covariance[7];
            let m33 = inverse_covariance[8];

            let dist =
                dist_x * (dist_x * m11 + dist_y * m21 + dist_z * m31) +
                dist_y * (dist_x * m21 + dist_y * m22 + dist_z * m32) +
                dist_z * (dist_x * m31 + dist_y * m32 + dist_z * m33);

            let pdf = (-0.5 * dist).exp();

            let dist_dx =
                2.0*dist_x*m11 + dist_y*m21 + dist_z*m31 + dist_y*m21 + dist_z*m31;
            let dist_dy =
                dist_x*m21 + dist_x*m12 + 2.0*dist_y*m22 + dist_z*m32 + dist_z*m23;
            let dist_dz =
                dist_x*m31 + dist_y*m32 + dist_x*m13 + dist_y * m23 + 2.0*dist_z * m33;

            (pdf, V3::new((-0.5 * dist_dx)*pdf,(-0.5 * dist_dy)*pdf,(-0.5 * dist_dz)*pdf))

        })
    })


}
