use plotters::prelude::*;

pub fn plot(
    root: &mut DrawingArea<egui_plotter::EguiBackend, plotters::coord::Shift>,
    _: &egui_plotter::Transform,
    &index: &usize,
) {
    let mut chart = ChartBuilder::on(root)
        .set_left_and_bottom_label_area_size(25)
        .build_cartesian_2d(0_f32..800.0, 0_f32..20.0)
        .unwrap();
    chart
        .configure_mesh()
        // .x_desc("time/years")
        // .y_desc("orbit angle/radians")
        .bold_line_style(full_palette::GREY_700)
        .light_line_style(full_palette::GREY_800)
        .axis_style(WHITE)
        .label_style(&WHITE)
        .draw()
        .unwrap();
    let planet = &super::PLANETS[index];
    // when eccentricity is zero t is roughly P * theta
    chart
        .draw_series(LineSeries::new(
            [0.0, 800.0].into_iter().map(|x| (x, planet.orbit * x)),
            WHITE,
        ))
        .unwrap();

    let vals: Vec<_> = (0_f32..20.0)
        .step(0.001)
        .values()
        .map(|theta| (1.0 - planet.eccentricity * theta.cos()).powi(-2))
        .collect();
    // multiply by coefficients
    chart
        .draw_series(LineSeries::new(
            (0.01_f32..20.0).step(0.01).values().map(|y| {
                let mut theta = vals[..(y * 1000.0) as usize].to_vec();
                let len = theta.len();
                for (i, val) in theta[1..len - 2].iter_mut().enumerate() {
                    *val *= if i % 2 == 1 { 4.0 } else { 2.0 }
                }
                (
                    planet.orbit
                        * (1.0 - planet.eccentricity.powi(2)).powf(1.5)
                        // 1/(2*pi)
                        * std::f32::consts::FRAC_1_PI
                        / 2.0
                        // h/3
                        * 0.001
                        / 3.0
                        * theta.into_iter().sum::<f32>(),
                    y,
                )
            }),
            WHITE,
        ))
        .unwrap();
}
