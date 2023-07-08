use plotters::prelude::*;

pub fn plot(
    root: &mut DrawingArea<egui_plotter::EguiBackend, plotters::coord::Shift>,
    _: &egui_plotter::Transform,
    (i, points): &(usize, Vec<(f32, f32)>),
) {
    let planet = &super::PLANETS[*i];
    let mut chart = ChartBuilder::on(root)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(0_f32..planet.orbit * 3.0, 0_f32..20.0)
        .unwrap();
    chart
        .configure_mesh()
        .x_desc("time/years")
        .y_desc("orbit angle/radians")
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

impl super::App {
    pub fn angles(&mut self) {
        let planet = &super::PLANETS[self.angles.get_data().0];
        let vals: Vec<_> = (0_f32..20.0)
            .step(0.001)
            .values()
            .map(|θ| (1.0 - planet.eccentricity * θ.cos()).powi(-2))
            .collect();
        self.angles.get_data_mut().1 = (0.01_f32..20.0)
            .step(0.1)
            .values()
            .map(|y| {
                let mut vals = vals[..(y * 1000.0) as usize].to_vec();
                let len = vals.len();
                for (i, val) in vals[1..len - 2].iter_mut().enumerate() {
                    *val *= if i % 2 == 1 { 4.0 } else { 2.0 }
                }
                (
                    planet.orbit
                        * (1.0 - planet.eccentricity.powi(2)).powf(1.5)
                        // 1/2π
                        *  std::f32::consts::FRAC_1_PI / 2.0
                        // h/3
                        * 0.001
                        / 3.0
                        * vals.into_iter().sum::<f32>(),
                    y,
                )
            })
            .collect();
    }
}
