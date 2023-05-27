use druid::WidgetExt;
use plotters::prelude::*;

pub fn plot() -> druid::widget::Split<super::State> {
    druid::widget::Split::columns(
        plotters_druid::Plot::new(|_, &super::State { scale, time, speed }, root| {
            let mut chart = ChartBuilder::on(root)
                .set_left_and_bottom_label_area_size(28)
                .build_cartesian_2d(-30.0 * scale..50.0 * scale, -40.0 * scale..40.0 * scale)
                .unwrap();
            chart
                .configure_mesh()
                .bold_line_style(full_palette::GREY_700)
                .light_line_style(full_palette::GREY_800)
                .axis_style(WHITE)
                .label_style(&WHITE)
                .draw()
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
            for planet in super::PLANETS.iter().skip(1) {
                let map = |theta: f32| {
                    let (sin, cos) = theta.to_radians().sin_cos();
                    let r = (planet.distance * (1.0 - planet.eccentricity.powi(2)))
                        / (1.0 - planet.eccentricity * cos);
                    (r * cos, r * sin)
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
                        &(EmptyElement::at(map((time.elapsed().as_secs_f32()
                            * speed.max(0.1) as f32
                            * 360.0)
                            / planet.orbit))
                            + Circle::new(
                                (0, 0),
                                rad,
                                ShapeStyle {
                                    color: planet.colour.into(),
                                    filled: true,
                                    stroke_width: 0,
                                },
                            )
                            + Circle::new(
                                (0, 0),
                                rad,
                                ShapeStyle {
                                    color: WHITE.into(),
                                    filled: false,
                                    stroke_width: 1,
                                },
                            )
                            + Text::new(
                                planet.name,
                                (10, -10),
                                FontDesc::new(FontFamily::Serif, 15.0, FontStyle::Normal)
                                    .color(&WHITE),
                            )),
                    )
                    .unwrap();
            }
        })
        .controller(super::Animate)
        .border(druid::Color::TRANSPARENT, 10.0),
        druid::widget::Flex::column()
            .with_child(druid::widget::Label::dynamic(
                |&super::State { speed, .. }, _| format!("{} yr/sec", speed.max(0.1)),
            ))
            .with_child(
                druid::widget::Slider::new()
                    .axis(druid::widget::Axis::Vertical)
                    .with_range(0.0, 10.0)
                    .with_step(0.5)
                    .annotated(1.0, 0.5)
                    .fix_height(500.0)
                    .lens(super::State::speed),
            )
            .center(),
    )
    .split_point(0.94)
}
