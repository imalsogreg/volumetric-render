pub mod ad;
pub(crate) mod gaussian;
pub mod integrate;

use frustum::Frustum;
use nalgebra::{Matrix3};
use plotters::coord::Shift;
use plotters::{coord::types::RangedCoordf64, prelude::*};
use std::ops::Range;

use ad::*;
use gaussian::*;
use integrate::*;

type Area<'a> = DrawingArea<BitMapBackend<'a>, Cartesian2d<RangedCoordf64, RangedCoordf64>>;
type AreaS<'a> = DrawingArea<BitMapBackend<'a>, Shift>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let y1: Val = Val::lin(V3::new(0.20, 0.0, 0.0));
    let y2 = Val::lin(V3::new(0.10, 0.20, 0.0));
    let y3 = y1.clone().exp();
    let y3 = y1 * (y2.exp()) + y3;
    let y3 = y3 * Val::c(0.2);
    let gauss = g(P3::new(-0.0, 0.5, 0.0), example_matrix())?;
    let gauss2 = g(P3::new(-1.0, -5.0, 0.0), unit_matrix())?;
    let gauss3 = g(P3::new(-4.0, 0.0, 0.0), Matrix3::new(2.0, 0.1, 1.9, 0.1, 2.0, 0.1, 1.9, 0.1, 2.0))?;
    let gauss4 = g(P3::new(-3.0, -3.0, 0.0), Matrix3::new(2.0, 0.1, 1.0, 0.1, 2.0, 0.1, 1.0, 0.1, 1.0))?;
    let gauss5 = g(P3::new( 3.0, -3.0, -1.0), Matrix3::new(1.0, 0.5, 1.0, 0.1, 2.0, 0.1, 1.0, 0.1, 9.0))?;

    let pdf =
        gauss * Val::c(0.8) +
        gauss2 * Val::c(0.8) +
        gauss3 * Val::c(0.6) +
        gauss4 * Val::c(0.4) +
        gauss5 * Val::c(0.3);
    let pdf_2 = pdf.clone();

    let x = P3::new(1.0, 0.0, 0.0);

    println!(
        "Hello, world! y3: val {:?}, deriv {:?}",
        pdf.f(x),
        pdf.df(x)
    );

    raytrace(pdf_2);
   
    let root = BitMapBackend::new("pdf.png", (600, 600)).into_drawing_area();
    let (left, right) = root.split_vertically(300);
    let ((nw, sw), (ne, se)) = (left.split_horizontally(300), right.split_horizontally(300));

    let charts: Vec<(AreaS, Box<dyn Fn(Val, P3) -> f64>, f64)> = vec![
        (nw, Box::new(|v: Val, x: P3| v.f(x)), 0.0),
        (ne, Box::new(|v: Val, x: P3| v.df(x).x), -1.0),
        (sw, Box::new(|v: Val, x: P3| v.df(x).y), -1.0),
        (se, Box::new(|v: Val, x: P3| v.df(x).z), -1.0),
    ];

    root.fill(&WHITE)?;

    for (root, project, c_min) in charts.into_iter() {
        let my_pdf = pdf.clone();

        let mut f_chart = ChartBuilder::on(&root)
            .margin(20)
            .x_label_area_size(10)
            .y_label_area_size(10)
            .build_cartesian_2d(-5.0f64..5.0f64, -5.0f64..5.0f64)?;

        f_chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .draw()?;

        let plotting_area = f_chart.plotting_area();

        let range = plotting_area.get_pixel_range();

        let (pw, ph) = (range.0.end - range.0.start, range.1.end - range.1.start);
        let (xr, yr) = (f_chart.x_range(), f_chart.y_range());

        plot_pdf_value(plotting_area, my_pdf, xr, yr, pw, ph, project, c_min)?;
    }

    Ok(())
}

fn clamp(v: f64, min: f64, max: f64) -> u8 {
    let dynamic_range = max - min;
    let fraction_of_range = (v - min) / dynamic_range;
    let r = (fraction_of_range * 255.0) as u8;
    r
}

fn plot_pdf_value(
    plotting_area: &Area,
    pdf: Val,
    xr: Range<f64>,
    yr: Range<f64>,
    pw: i32,
    ph: i32,
    project: impl Fn(Val, P3) -> f64,
    c_min: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    for (x, y, c) in pdf_slice(pdf, xr, yr, -1.0, (pw as usize, ph as usize), project) {
        plotting_area.draw_pixel(
            (x, y),
            &RGBColor(clamp(c as f64, c_min, 1.0), clamp(c as f64, 0.0, 1.0), 0),
        )?;
    }
    Ok(())
}

fn pdf_slice(
    val: Val,
    x_range: Range<f64>,
    y_range: Range<f64>,
    z: f64,
    samples: (usize, usize),
    project: impl Fn(Val, P3) -> f64,
) -> impl Iterator<Item = (f64, f64, f64)> {
    let step = (
        (x_range.end - x_range.start) / samples.0 as f64,
        (y_range.end - y_range.start) / samples.1 as f64,
    );
    (0..(samples.0 * samples.1))
        .map(move |k| {
            let c = (
                x_range.start + step.0 * (k % samples.0) as f64,
                y_range.start + step.1 * (k / samples.0) as f64,
            );
            (c.0, c.1, project(val.clone(), P3::new(c.0, c.1, z)))
        })
        .into_iter()
}

pub fn raytrace(v: Val) -> Result<(), Box<dyn std::error::Error>> {

    let mut frustum = Frustum {
        origin: P3::new(-5.0, -5.0, -10.0),
        target: P3::new(0.0, 0.0, 0.0),
        fovy: 90.0,
        ncp: 1.0,
        fcp: 10.0,
        width: 300,
        height: 300,
    };
    for i in (0..40) {

        let fname = String::from(format!("movie/raytrace{:05}.png", i));
        println!("Working on frame {}", i);

        let root = BitMapBackend::new(fname.as_str(), (600,300)).into_drawing_area();
        root.fill(&WHITE)?;

        for eye in (0..2) {

            let eye_offset_x = 300.0 * (eye as f64);
            let eye_offset_phase = (eye as f64) * 3.14459 / 4.0 ;
            let mut chart = ChartBuilder::on(&root)
                .margin(20)
                .x_label_area_size(10)
                .y_label_area_size(10)
                .build_cartesian_2d(0.0..600.0, 0.0..300.0)?;
            chart
                .configure_mesh()
                .disable_x_mesh()
                .disable_y_mesh()
                .draw()?;
            let plotting_area = chart.plotting_area();

            let my_v = v.clone();
            let frame_rate = 20.0;
            let circle_freq = 0.5;
            let bounce_freq = 0.5;
            let circle_phase = 2.0 * 3.14159 * (i as f64 + eye_offset_phase) as f64 * circle_freq / frame_rate;
            // let bounce_phase = 2.0 * 3.14159 * i as f64 * bounce_freq / frame_rate;
            frustum.origin.x = 10.0 * circle_phase.cos();
            frustum.origin.z = 10.0 * circle_phase.sin();
            // frustum.origin.y = 0.2 * bounce_phase.cos();
            //
            fn to_range(v: f64, min: f64, max: f64) -> u8 {
                let v_norm  = (v - min) / (max - min);
                let v_clamp = v_norm.max(0.0).min(1.0);
                (v_clamp * 255.0) as u8
            }

            for (x,y,(v,p)) in render(my_v, frustum, 50.0, 1.0, 0.1) {
                let c = (v * 155.0) as u8;

                // plotting_area.draw_pixel((x,y), &RGBColor(
                //     to_range(p.x, -10.0, 10.0),
                //     to_range(p.y, -10.0, 10.0),
                //     to_range(p.z, -10.0, 10.0)
                // ))?;

                plotting_area.draw_pixel((x + eye_offset_x + 0.0,y), &RGBColor(
                    to_range(v, 0.0, 2.0),
                    to_range(v, 0.0, 2.0),
                    to_range(v.log(10.0), -2.5, 1.5)
                ))?;
            }

        }
    }
    Ok(())
}
