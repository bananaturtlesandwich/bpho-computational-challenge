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
    let mut chart = ChartBuilder::on(root)
        .build_cartesian_2d(
            far.coord(std::f32::consts::PI).0 * 1.1..far.coord(0.0).0 * 1.1,
            far.coord(std::f32::consts::FRAC_PI_2).1 * 1.1
                ..far.coord(std::f32::consts::FRAC_PI_2 * 3.0).1 * 1.1,
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
                (0_f32..361.0)
                    .step(2.5)
                    .values()
                    .map(|θ| planet.coord(θ.to_radians())),
                planet.colour,
            ))
            .unwrap();
    }
}
impl super::App {
    pub fn spiral(&mut self) {
        let (i1, i2, points) = self.spiral.get_data_mut();
        let (p1, p2) = (&super::PLANETS[*i1], &super::PLANETS[*i2]);
        let max = 10.0 * p1.orbit.max(p2.orbit);
        *points = (0.0..max)
            .step(max / 1234.0)
            .values()
            .map(|years| [p1.coord(p1.angle(years)), p2.coord(p2.angle(years))])
            .collect()
    }
}
