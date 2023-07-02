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
    // when eccentricity is much less than 1 (in this case zero) t is roughly P * theta
    chart
        .draw_series(LineSeries::new(
            [0.0, 800.0].into_iter().map(|x| (x, planet.orbit * x)),
            WHITE,
        ))
        .unwrap();
    // i've got a bit of brain block on this

    // let theta: Vec<_> = (0_f32..20.0)
    //     .step(0.001)
    //     .values()
    //     .map(|theta| (1.0 - planet.eccentricity * theta.cos()).powi(-2))
    //     .collect();
    // // multiply by coefficients
    // let len = theta.len();
    // chart
    //     .draw_series(LineSeries::new(
    //         (0_f32..20.0).step(0.01).values().map(|x| {
    //             let mut theta = theta[..len - 1 - (x * 1000.0) as usize].to_vec();
    //             for (i, val) in theta[1..len - 2].iter_mut().enumerate() {
    //                 *val *= if i % 2 == 1 { 4.0 } else { 2.0 }
    //             }
    //             (
    //                 x,
    //                 planet.orbit
    //                     * (1.0 - planet.eccentricity.powi(2)).powf(1.5)
    //                     * std::f32::consts::FRAC_1_PI
    //                     / 2.0
    //                     / 1000.0
    //                     / 3.0
    //                     * theta.into_iter().sum::<f32>(),
    //             )
    //         }),
    //         WHITE,
    //     ))
    //     .unwrap();
}
