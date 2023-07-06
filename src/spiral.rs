use plotters::prelude::*;

pub fn plot(
    root: &mut DrawingArea<egui_plotter::EguiBackend, plotters::coord::Shift>,
    _: &egui_plotter::Transform,
    (i1, i2, lines): &(usize, usize, Vec<[(f32, f32); 2]>),
) {
    let (p1, p2) = (&super::PLANETS[*i1], &super::PLANETS[*i2]);
    let far = match p1.distance > p2.distance {
        true => p1,
        false => p2,
    };
    // axis bounds are the same as the maximums and minimums of the largest orbit (slightly adjusted)
    let x = |θ: f32| {
        let cos = θ.cos();
        (far.distance * (1.0 - far.eccentricity.powi(2))) / (1.0 - far.eccentricity * cos)
            * cos
            * 1.1
    };
    let y = |θ: f32| {
        let (sin, cos) = θ.sin_cos();
        (far.distance * (1.0 - far.eccentricity.powi(2))) / (1.0 - far.eccentricity * cos)
            * sin
            * 1.1
    };

    let mut chart = ChartBuilder::on(root)
        .build_cartesian_2d(
            x(std::f32::consts::PI)..x(0.0),
            y(std::f32::consts::PI / 2.0)..y(std::f32::consts::PI / 2.0 * 3.0),
        )
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
