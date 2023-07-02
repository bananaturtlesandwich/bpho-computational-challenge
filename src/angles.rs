use plotters::prelude::*;

pub fn plot(
    root: &mut DrawingArea<egui_plotter::EguiBackend, plotters::coord::Shift>,
    _: &egui_plotter::Transform,
    (index, points): &(usize, Vec<(f32, f32)>),
) {
    let planet = &super::PLANETS[*index];
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
    // when eccentricity is zero t is roughly P * theta
    chart
        .draw_series(LineSeries::new(
            [0.0, 800.0].into_iter().map(|x| (x, planet.orbit * x)),
            WHITE,
        ))
        .unwrap();

    // multiply by coefficients
    chart
        .draw_series(LineSeries::new(points.iter().cloned(), WHITE))
        .unwrap();
}
