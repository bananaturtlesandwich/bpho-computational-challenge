use plotters::prelude::*;

pub fn plot(
    root: &mut DrawingArea<egui_plotter::EguiBackend, plotters::coord::Shift>,
    _: &egui_plotter::Transform,
    (i, points): &(usize, Vec<(f32, f32)>),
) {
    let planet = &super::PLANETS[*i];
    let mut chart = ChartBuilder::on(root)
        .set_left_and_bottom_label_area_size(25)
        .build_cartesian_2d(0_f32..planet.orbit * 3.0, 0_f32..20.0)
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
    /*
      when eccentricity is zero:
    - (1 - ecc^2)^3/2 evaluates to 1 so no need to include in calculation
    - integral is always θ since 1 / (1 - ecc * cosθ) gives the line y = 1 so no need to estimate
    - therefore t = y * planet.orbit * 1/2π
    - however this doesn't scale well so i'll reverse it to  y = t/(planet.orbit * 1/2π)
    */
    chart
        .draw_series(LineSeries::new(
            [0.0, planet.orbit * 3.0]
                .into_iter()
                .map(|x| (x, x / planet.orbit / std::f32::consts::FRAC_1_PI * 2.0)),
            WHITE,
        ))
        .unwrap();
    chart
        .draw_series(LineSeries::new(points.iter().copied(), GREEN))
        .unwrap();
}
