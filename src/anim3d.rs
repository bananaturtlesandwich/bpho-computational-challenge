use plotters::prelude::*;

pub fn plot(
    root: &mut DrawingArea<egui_plotter::EguiBackend, plotters::coord::Shift>,
    trans: &egui_plotter::Transform,
    &(scale, time, speed): &(f32, instant::Instant, f32),
) {
    let mut chart = ChartBuilder::on(root)
        .set_left_and_bottom_label_area_size(25)
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
    let style = WHITE.into_text_style(chart.plotting_area());
    chart
        .configure_axes()
        .bold_grid_style(full_palette::GREY_700)
        .light_grid_style(full_palette::GREY_800)
        .label_style(style)
        .draw()
        .unwrap();
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
    for planet in super::PLANETS.iter().skip(1) {
        let map = |theta: f32| {
            let (sin, cos) = theta.to_radians().sin_cos();
            let r = (planet.distance * (1.0 - planet.eccentricity.powi(2)))
                / (1.0 - planet.eccentricity * cos);
            let (x, y) = (r * cos, r * sin);
            let (sin, cos) = planet.inclination.to_radians().sin_cos();
            (x * cos, x * sin, y)
        };
        chart
            .draw_series(LineSeries::new(
                (0_f32..361.0).step(2.5).values().map(map),
                planet.colour,
            ))
            .unwrap();
        let rad = planet.radius.max(1.0);
        chart
            .plotting_area()
            .draw(
                &(EmptyElement::at(map(
                    time.elapsed().as_secs_f32() * speed * 360.0 / planet.orbit
                )) + Circle::new(
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
