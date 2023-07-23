use plotters::prelude::*;

pub fn plot(
    root: &mut DrawingArea<egui_plotter::EguiBackend, plotters::coord::Shift>,
    _: &egui_plotter::Transform,
    (scale, i, orbits): &(f32, usize, Vec<Vec<(f32, f32)>>),
) {
    let centre = &super::PLANETS[*i];
    let (mut maxx, mut minx, mut maxy, mut miny) = (0.0, 0.0, 0.0, 0.0);
    for (x, y) in orbits.iter().flatten() {
        if x > &maxx {
            maxx = *x
        }
        if x < &minx {
            minx = *x
        }
        if y > &maxy {
            maxy = *y
        }
        if y < &miny {
            miny = *y
        }
    }
    let mut chart = ChartBuilder::on(root)
        .build_cartesian_2d(minx * scale..maxx * scale, miny * scale..maxy * scale)
        .unwrap();
    chart
        .plotting_area()
        .draw(&Circle::new(
            (0.0, 0.0),
            (1.0 / scale) as i32,
            ShapeStyle {
                color: centre.colour.into(),
                filled: true,
                stroke_width: 0,
            },
        ))
        .unwrap();
    for (orbit, colour) in orbits.iter().zip(
        super::PLANETS
            .iter()
            .filter_map(|p| (p != centre).then_some(p.colour)),
    ) {
        chart
            .draw_series(LineSeries::new(orbit.iter().copied(), colour))
            .unwrap();
    }
}

impl super::App {
    pub fn centre(&mut self) {
        let (_, i, orbits) = self.centre.get_data_mut();
        let centre = &super::PLANETS[*i];
        *orbits = super::PLANETS
            .iter()
            .filter(|p| p != &centre)
            .map(|planet| {
                let max = match planet.near() {
                    true => 2500.0,
                    false => 250.0,
                } * centre.orbit.max(planet.orbit);
                (0_f32..max + 0.1)
                    .step(
                        max / match planet.near() {
                            true => 500.0,
                            false => 8000.0,
                        },
                    )
                    .values()
                    .map(|θ| {
                        let θ = θ.to_radians();
                        let centre = centre.coord_when(planet, θ);
                        let current = planet.coord(θ);
                        (current.0 - centre.0, current.1 - centre.1)
                    })
                    .collect()
            })
            .collect();
    }
}
