use plotters::prelude::*;

pub fn plot(
    root: &mut DrawingArea<egui_plotter::EguiBackend, plotters::coord::Shift>,
    trans: &egui_plotter::Transform,
    &(scale, time, speed): &(f32, instant::Instant, f32),
) {
    let mut chart = ChartBuilder::on(root)
        .build_cartesian_3d(
            -30.0 * scale..50.0 * scale,
            -40.0 * scale..40.0 * scale,
            -20.0 * scale..20.0 * scale,
        )
        .unwrap();
    chart.with_projection(|mut pb| {
        pb.yaw = trans.yaw;
        pb.pitch = trans.pitch;
        pb.into_matrix()
    });
    chart
        .plotting_area()
        .draw(&Circle::new(
            (0.0, 0.0, 0.0),
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
                (0_f32..361.0)
                    .step(2.5)
                    .values()
                    .map(|θ| planet.position(θ.to_radians())),
                planet.colour,
            ))
            .unwrap();
        let rad = planet.radius.max(1.0);
        chart
            .plotting_area()
            .draw(
                &(EmptyElement::at(
                    planet.position(planet.angle(time.elapsed().as_secs_f32() * speed)),
                ) + Circle::new(
                    (0, 0),
                    rad,
                    ShapeStyle {
                        color: planet.colour.into(),
                        filled: true,
                        stroke_width: 0,
                    },
                ) + Circle::new(
                    (0, 0),
                    rad,
                    ShapeStyle {
                        color: WHITE.into(),
                        filled: false,
                        stroke_width: 1,
                    },
                ) + Text::new(
                    planet.name,
                    (10, -10),
                    FontDesc::new(FontFamily::Serif, 15.0, FontStyle::Normal).color(&WHITE),
                )),
            )
            .unwrap();
    }
}
