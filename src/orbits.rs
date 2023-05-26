use druid::WidgetExt;
use plotters::prelude::*;

pub fn plot() -> druid::widget::Container<super::State> {
    plotters_druid::Plot::new(|_, data: &super::State, root| {
        let mut chart = ChartBuilder::on(&root)
            .set_left_and_bottom_label_area_size(28)
            .build_cartesian_2d(
                -30.0 * data.scale..50.0 * data.scale,
                -40.0 * data.scale..40.0 * data.scale,
            )
            .unwrap();
        chart
            .configure_mesh()
            .bold_line_style(&full_palette::GREY_700)
            .light_line_style(&full_palette::GREY_800)
            .axis_style(&WHITE)
            .label_style(&WHITE)
            .draw()
            .unwrap();
        chart
            .plotting_area()
            .draw(&Circle::new(
                (0.0, 0.0).into(),
                (1.0 / data.scale) as i32,
                ShapeStyle {
                    color: full_palette::AMBER.into(),
                    filled: true,
                    stroke_width: 0,
                },
            ))
            .unwrap();
        for planet in super::PLANETS.iter().skip(1) {
            chart
                .draw_series(LineSeries::new(
                    (0..361).map(|theta| {
                        let (sin, cos) = (theta as f32).to_radians().sin_cos();
                        let r = (planet.distance * (1.0 - planet.eccentricity.powi(2)))
                            / (1.0 - planet.eccentricity * cos);
                        (r * cos, r * sin)
                    }),
                    planet.colour,
                ))
                .unwrap();
        }
    })
    .controller(super::Mouse)
    .border(druid::Color::TRANSPARENT, 10.0)
}
