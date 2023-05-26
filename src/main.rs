mod kepler;

fn main() {
    druid::AppLauncher::with_window(
        druid::WindowDesc::new(
            druid::widget::Tabs::new()
                .with_tab("Kepler's third law", kepler::plot())
                .with_tab("hoooiiiiii", druid::widget::Label::new("ohai there!")),
        )
        .title("bpho comp challenge"),
    )
    .launch(State { scale: 1.0 })
    .unwrap();
}

#[derive(Clone, druid::Data)]
pub struct State {
    scale: f32,
}

struct Mouse;

impl druid::widget::Controller<State, plotters_druid::Plot<State>> for Mouse {
    fn event(
        &mut self,
        _: &mut plotters_druid::Plot<State>,
        _: &mut druid::EventCtx,
        event: &druid::Event,
        state: &mut State,
        _: &druid::Env,
    ) {
        if let druid::Event::Wheel(m) = event {
            state.scale = (state.scale * 0.99f32.powf(-m.wheel_delta.y as f32)).clamp(0.01, 1.0);
        }
    }
}

#[allow(dead_code)]
struct Planet {
    name: &'static str,
    colour: plotters::style::RGBColor,
    mass: f32,
    distance: f32,
    radius: f32,
    rotation: f32,
    orbit: f32,
}

use plotters::style::full_palette;

const PLANETS: [Planet; 10] = [
    // my favourite planet >:p
    Planet {
        name: "Sun",
        colour: full_palette::ORANGE,
        mass: 332837.0,
        distance: 0.0,
        radius: 109.12,
        rotation: 0.0,
        orbit: 0.0,
    },
    Planet {
        name: "Mercury",
        colour: full_palette::AMBER_A400,
        mass: 0.055,
        distance: 0.387,
        radius: 0.38,
        rotation: 58.65,
        orbit: 0.24,
    },
    Planet {
        name: "Venus",
        colour: full_palette::ORANGE_A100,
        mass: 0.815,
        distance: 0.723,
        radius: 0.95,
        rotation: 243.02,
        orbit: 0.62,
    },
    Planet {
        name: "Earth",
        colour: full_palette::GREEN,
        mass: 1.0,
        distance: 1.0,
        radius: 1.0,
        rotation: 1.0,
        orbit: 1.0,
    },
    Planet {
        name: "Mars",
        colour: full_palette::RED_700,
        mass: 0.107,
        distance: 1.523,
        radius: 0.53,
        rotation: 1.03,
        orbit: 1.88,
    },
    Planet {
        name: "Saturn",
        colour: full_palette::AMBER_300,
        mass: 95.16,
        distance: 9.58,
        radius: 9.45,
        rotation: 0.44,
        orbit: 29.63,
    },
    Planet {
        name: "Jupiter",
        colour: full_palette::DEEPORANGE_400,
        mass: 317.85,
        distance: 5.2,
        radius: 11.21,
        rotation: 0.41,
        orbit: 11.86,
    },
    Planet {
        name: "Uranus",
        colour: full_palette::CYAN_A400,
        mass: 14.5,
        distance: 19.29,
        radius: 4.01,
        rotation: 0.72,
        orbit: 84.75,
    },
    Planet {
        name: "Neptune",
        colour: full_palette::LIGHTBLUE_A200,
        mass: 17.2,
        distance: 30.25,
        radius: 3.88,
        rotation: 0.67,
        orbit: 166.34,
    },
    Planet {
        name: "Pluto",
        colour: full_palette::GREY,
        mass: 0.0,
        distance: 39.51,
        radius: 0.19,
        rotation: 6.39,
        orbit: 248.35,
    },
];
