use std::ops::*;
use std::sync::Arc;

use euclid::{Point3D, UnknownUnit, Vector3D};

pub type P3 = Point3D<f64, UnknownUnit>;

pub type V3 = Vector3D<f64, UnknownUnit>;

#[derive(Clone)]
pub struct Val {
    pub v: Arc<dyn Fn(P3) -> (f64, V3)>,
}

impl Val {
    pub fn c(x: f64) -> Val {
        Val {
            v: Arc::new(move |_| (x, V3::new(0.0, 0.0, 0.0))),
        }
    }

    pub fn lin(v: V3) -> Val {
        Val {
            v: Arc::new(move |x: P3| (v.x * x.x + v.y * x.y + v.z * x.z, v)),
        }
    }

    pub fn f(&self, x: P3) -> f64 {
        (self.v)(x).0
    }

    pub fn df(&self, x: P3) -> V3 {
        (self.v)(x).1
    }

    pub fn exp(self) -> Val {
        Val {
            v: Arc::new(move |x: P3| {
                let (g_x, dg_x) = (self.v)(x);
                let dfg_x = scale(dg_x, g_x.exp());
                let fg_x = g_x.exp();
                (fg_x, dfg_x)
            }),
        }
    }

    // dist = sqrt((x - x_0)^2 + (y - y_0)^2)
    // dist = ((x - x_0)^2 + (y - y_0)^2 + (z - z_0)^2) ^ 0.5
    // dist = (f . g)(x), f(x) = sqrt(x), g = ((x - x_0)^2 + C + D)
    // d_dist/dx = f'(g(x)) * g'(x)
    // d_dist/dx =  0.5( dist_squared )^(-0.5) * 2*(x - x_0)
    pub fn distance_from(p: P3) -> Val {
        Val {
            v: Arc::new(move |x: P3| {
                let dist_squared =
                    (x.x - p.x).powf(2.0) + (x.y - p.y).powf(2.0) + (x.z - p.z).powf(2.0);
                let dist_squared = if dist_squared == 0.0 { 1e-10 } else { dist_squared };
                let dist = dist_squared.powf(0.5);
                let dx = (0.5 * dist_squared).powf(-0.5) * 2.0 * (x.x - p.x);
                let dy = (0.5 * dist_squared).powf(-0.5) * 2.0 * (x.y - p.y);
                let dz = (0.5 * dist_squared).powf(-0.5) * 2.0 * (x.z - p.z);
                let d = V3::new(dx, dy, dz);
                (dist, d)
            }),
        }
    }

    //     pub fn compose(self, g: Val) -> Val {
    //         Val {
    //             v: Arc::new( move |x : P3|{
    //                 let (g_x,dg_x) = (g.v)(x);
    //                 let (f_g_x, df_g_x) = (self.v)(g_x);
    //                 let deriv = df_x_x *
    //                 let (y2,d2) = (g.v)(x);

    //             } )
    //         }
    //     }
}

impl Add for Val {
    type Output = Val;
    fn add(self, other: Val) -> Val {
        Val {
            v: Arc::new(move |x: P3| {
                let (y1, d1) = (self.v)(x);
                let (y2, d2) = (other.v)(x);
                return (y1 + y2, d1 + d2);
            }),
        }
    }
}

fn scale(vect: V3, coef: f64) -> V3 {
    V3::new(coef * vect.x, coef * vect.y, coef * vect.z)
}

impl Mul for Val {
    type Output = Val;
    fn mul(self, other: Val) -> Val {
        Val {
            v: Arc::new(move |x: P3| {
                let (y1, d1) = (self.v)(x);
                let (y2, d2) = (other.v)(x);
                return (y1 * y2, scale(d2, y1) + scale(d1, y2));
            }),
        }
    }
}

impl Sub for Val {
    type Output = Val;
    fn sub(self, other: Val) -> Val {
        Val {
            v: Arc::new(move |x: P3| {
                let (y1, d1) = (self.v)(x);
                let (y2, d2) = (other.v)(x);
                return (y1 - y2, d1 - d2);
            }),
        }
    }
}

impl Neg for Val {
    type Output = Val;
    fn neg(self) -> Val {
        Val {
            v: Arc::new(move |x: P3| {
                let (y, d) = (self.v)(x);
                return (-y, -d);
            }),
        }
    }
}

impl Div for Val {
    type Output = Val;
    fn div(self, other: Val) -> Val {
        Val {
            v: Arc::new(move |x: P3| {
                let (y1, d1) = (self.v)(x);
                let (y2, d2) = (other.v)(x);
                let numerator = scale(d1, 1.0 / y2) - scale(d2, 1.0 / y1);
                let denominator = y2 * y2;
                (y1 / y2, numerator / denominator)
            }),
        }
    }
}
