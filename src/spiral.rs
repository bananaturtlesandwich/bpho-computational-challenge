use plotters::prelude::*;

pub fn plot(
    root: &mut DrawingArea<egui_plotter::EguiBackend, plotters::coord::Shift>,
    _: &egui_plotter::Transform,
    (i1, i2, lines): &(usize, usize, Vec<[(f32, f32); 2]>),
) {
    let (p1, p2) = (&super::PLANETS[*i1], &super::PLANETS[*i2]);
    let scale = p1.distance.max(p2.distance) / 30.0;
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
    for line in lines.iter() {
        chart
            .draw_series(LineSeries::new(line.iter().copied(), WHITE))
            .unwrap();
    }
    for planet in [p1, p2] {
        chart
            .draw_series(LineSeries::new(
                (0.0..361.0).step(2.5).values().map(|θ| {
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
