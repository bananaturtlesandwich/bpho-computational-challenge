use plotters::prelude::*;

pub fn plot(
    root: &mut DrawingArea<egui_plotter::EguiBackend, plotters::coord::Shift>,
    _: &egui_plotter::Transform,
    &scale: &f32,
) {
    let mut chart = ChartBuilder::on(root)
        .x_label_area_size(40)
        .y_label_area_size(55)
        .build_cartesian_2d(0.0..275.0 * scale, 0.0..275.0 * scale)
        .unwrap();
    chart
        .configure_mesh()
        .x_desc("distance from sun^1.5/AU")
        .y_desc("orbit time/years")
        .bold_line_style(full_palette::GREY_700)
        .light_line_style(full_palette::GREY_800)
        .axis_style(WHITE)
        .label_style(&WHITE)
        .draw()
        .unwrap();
    chart
        .draw_series(LineSeries::new(
            [0.0, 300.0].into_iter().map(best_fit(|| {
                super::PLANETS
                    .iter()
                    .map(|planet| (planet.distance.powf(1.5), planet.orbit))
            })),
            GREEN,
        ))
        .unwrap();
    chart
        .draw_series(PointSeries::of_element(
            super::PLANETS.iter(),
            0,
            RGBAColor::default(),
            &|planet, _, _| {
                EmptyElement::at((planet.distance.powf(1.5), planet.orbit))
                    + Circle::new(
                        (0, 0),
                        planet.radius.max(1.0),
                        ShapeStyle {
                            color: planet.colour.into(),
                            filled: true,
                            stroke_width: 0,
                        },
                    )
                    + Text::new(
                        planet.name,
                        (10, -10),
                        FontDesc::new(FontFamily::Serif, 15.0, FontStyle::Normal).color(&if scale
                            > 0.1
                            && planet.orbit < 2.0
                        {
                            TRANSPARENT
                        } else {
                            WHITE.into()
                        }),
                    )
            },
        ))
        .unwrap();
}

/// returns a line of best fit using the minimum sum of squared errors algorithm
fn best_fit<I: Iterator<Item = (f32, f32)>>(iter: impl Fn() -> I) -> impl Fn(f32) -> (f32, f32) {
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
    move |x: f32| (x, m * x + c)
}
