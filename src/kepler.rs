use druid::WidgetExt;
use plotters::prelude::*;

pub fn plot() -> druid::widget::Container<()> {
    plotters_druid::Plot::new(|_, _, root| {
        let mut chart = ChartBuilder::on(&root)
            .set_left_and_bottom_label_area_size(33)
            .build_cartesian_2d(0f32..275.0, 0f32..275.0)
            .unwrap();
        chart
            .configure_mesh()
            .x_desc("(a/AU)^(3/2)")
            .y_desc("T/Yr")
            .bold_line_style(&full_palette::GREY_700)
            .light_line_style(&full_palette::GREY_800)
            .axis_style(&full_palette::WHITE)
            .label_style(&full_palette::WHITE)
            .draw()
            .unwrap();
        chart
            .draw_series(PointSeries::of_element(
                // we don't want to include the sun
                super::PLANETS.into_iter().skip(1),
                0,
                RGBAColor::default(),
                &|planet, _, _| {
                    EmptyElement::at((planet.orbit, planet.distance.powf(1.5)))
                        + Circle::new(
                            (0, 0),
                            planet.radius.max(1.0),
                            ShapeStyle {
                                color: planet.colour.into(),
                                filled: true,
                                stroke_width: 1,
                            },
                        )
                        + Text::new(
                            planet.name,
                            (10, -10),
                            FontDesc::new(FontFamily::SansSerif, 12.0, FontStyle::Normal)
                                .color(&WHITE),
                        )
                },
            ))
            .unwrap();
    })
    .border(druid::Color::TRANSPARENT, 10.0)
}
