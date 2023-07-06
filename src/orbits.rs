use plotters::prelude::*;

pub fn plot(
    root: &mut DrawingArea<egui_plotter::EguiBackend, plotters::coord::Shift>,
    _: &egui_plotter::Transform,
    scale: &f32,
) {
    let mut chart = ChartBuilder::on(root)
        .build_cartesian_2d(-30.0 * scale..50.0 * scale, -40.0 * scale..40.0 * scale)
        .unwrap();
    chart
        .plotting_area()
        .draw(&Circle::new(
            (0.0, 0.0),
            (1.0 / scale) as i32,
            ShapeStyle {
                color: full_palette::AMBER.into(),
                filled: true,
                stroke_width: 0,
            },
        ))
        .unwrap();
    for planet in super::PLANETS.iter() {
        chart
            .draw_series(LineSeries::new(
                (0..361).map(|θ| {
                    let (sin, cos) = (θ as f32).to_radians().sin_cos();
                    let r = (planet.distance * (1.0 - planet.eccentricity.powi(2)))
                        / (1.0 - planet.eccentricity * cos);
                    (r * cos, r * sin)
                }),
                planet.colour,
            ))
            .unwrap();
    }
}
