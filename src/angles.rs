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
    let map = |theta: i32, eccentricity: f32| {
        // h = theta/n = 1/1000 because a = 0 so n = 1000 * theta
        (
            planet.orbit * (1.0 - eccentricity.powi(2)).powf(1.5) * 1.0
                / (2.0 * std::f32::consts::PI),
            // computing f(x0)...f(xn) is hella expensive :/
            // * (0..1000 * theta)
            //     .map(|theta| theta as f32 / (1.0 - eccentricity * (theta as f32).cos()).powi(2))
            //     .enumerate()
            //     .fold(0.0, |acc, (i, y)| {
            //         if i == 0 || i == 1000 * theta as usize - 1 {
            //             acc + y
            //         } else if i % 2 == 1 {
            //             acc + 4.0 * y
            //         } else {
            //             acc + 2.0 * y
            //         }
            //     }),
            theta as f32,
        )
    };
    chart
        .draw_series(LineSeries::new((0..800).map(|arg| map(arg, 0.0)), WHITE))
        .unwrap();
}
