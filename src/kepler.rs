use druid::WidgetExt;
use plotters::prelude::*;

pub fn plot() -> druid::widget::Container<super::State> {
    plotters_druid::Plot::new(|_, &super::State { scale, .. }, root| {
        let mut chart = ChartBuilder::on(root)
            .set_left_and_bottom_label_area_size(33)
            .build_cartesian_2d(0.0..275.0 * scale, 0.0..275.0 * scale)
            .unwrap();
        chart
            .configure_mesh()
            .x_desc("(a/AU)^(3/2)")
            .y_desc("T/Yr")
            .bold_line_style(full_palette::GREY_700)
            .light_line_style(full_palette::GREY_800)
            .axis_style(WHITE)
            .label_style(&WHITE)
            .draw()
            .unwrap();
        chart
            .draw_series(LineSeries::new(
                [0, 300].into_iter().map(best_fit(|| {
                    super::PLANETS
                        .iter()
                        .skip(1)
                        .map(|planet| (planet.orbit, planet.distance.powf(1.5)))
                })),
                GREEN,
            ))
            .unwrap();
        chart
            .draw_series(PointSeries::of_element(
                // we don't want to include the sun
                super::PLANETS.iter().skip(1),
                0,
                RGBAColor::default(),
                &|planet, _, _| {
                    let rad = planet.radius.max(1.0);
                    EmptyElement::at((planet.orbit, planet.distance.powf(1.5)))
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
                            FontDesc::new(FontFamily::Serif, 15.0, FontStyle::Normal).color(&WHITE),
                        )
                },
            ))
            .unwrap();
    })
    .controller(super::Mouse)
    .border(druid::Color::TRANSPARENT, 10.0)
}

/// returns a line of best fit using minimum sum of squared errors for the data
fn best_fit<I: Iterator<Item = (f32, f32)>>(iter: impl Fn() -> I) -> impl Fn(i32) -> (f32, f32) {
    let len = iter().count() as f32;
    let (meanx, meany) = iter()
        .reduce(|(acc_x, acc_y), (x, y)| (acc_x + x, acc_y + y))
        .map(|(sum_x, sum_y)| (sum_x / len, sum_y / len))
        .unwrap_or((0.0, 0.0));
    let (numer, denom) = iter().fold((0.0, 0.0), |(numer, denom), (x, y)| {
        (
            numer + ((x - meanx) * (y - meany)),
            denom + (x - meanx).powi(2),
        )
    });
    let m = numer / denom;
    let c = meany - m * meanx;
    move |x: i32| (x as f32, m * x as f32 + c)
}
