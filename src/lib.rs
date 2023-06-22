use druid::WidgetExt;
use plotters::style::full_palette;
#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

mod anim2d;
mod anim3d;
mod kepler;
mod orbits;

#[derive(Clone, druid::Lens)]
pub struct State {
    scale: f32,
    time: instant::Instant,
    speed: f64,
}

impl druid::Data for State {
    fn same(&self, other: &Self) -> bool {
        self.scale == other.scale && self.speed == other.speed
    }
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub fn entry() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    main()
}

pub fn main() {
    druid::AppLauncher::with_window(
        druid::WindowDesc::new(
            druid::widget::Tabs::new()
                .with_tab("Kepler's third law", kepler::plot())
                .with_tab("orbits", orbits::plot())
                .with_tab("2d animation", anim2d::plot())
                .with_tab("3d animation", anim3d::plot())
                .on_click(|_, State { scale, .. }, _| *scale = 1.0),
        )
        .title("bpho comp challenge"),
    )
    .launch(State {
        scale: 1.0,
        time: instant::Instant::now(),
        speed: 1.0,
    })
    .unwrap();
}

pub struct Mouse;

impl druid::widget::Controller<State, plotters_druid::Plot<State>> for Mouse {
    fn event(
        &mut self,
        _: &mut plotters_druid::Plot<State>,
        _: &mut druid::EventCtx,
        event: &druid::Event,
        State { scale, .. }: &mut State,
        _: &druid::Env,
    ) {
        if let druid::Event::Wheel(m) = event {
            *scale = (*scale * 0.99_f32.powf(-m.wheel_delta.y as f32)).clamp(0.01, 1.0);
        }
    }
}

pub struct Animate;

impl druid::widget::Controller<State, plotters_druid::Plot<State>> for Animate {
    fn event(
        &mut self,
        _: &mut plotters_druid::Plot<State>,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        State { scale, .. }: &mut State,
        _: &druid::Env,
    ) {
        match event {
            druid::Event::AnimFrame(_) => {
                ctx.request_paint();
                ctx.request_anim_frame()
            }
            druid::Event::Wheel(m) => {
                *scale = (*scale * 0.99_f32.powf(-m.wheel_delta.y as f32)).clamp(0.01, 1.0)
            }
            _ => (),
        }
    }
    fn lifecycle(
        &mut self,
        _: &mut plotters_druid::Plot<State>,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        _: &State,
        _: &druid::Env,
    ) {
        if let druid::LifeCycle::WidgetAdded = event {
            ctx.request_anim_frame()
        }
    }
}

#[allow(dead_code)]
struct Planet {
    name: &'static str,
    colour: plotters::style::RGBColor,
    mass: f32,
    distance: f32,
    eccentricity: f32,
    radius: f32,
    rotation: f32,
    orbit: f32,
    inclination: f32,
}

const PLANETS: [Planet; 10] = [
    // my favourite planet >:p
    Planet {
        name: "Sun",
        colour: full_palette::ORANGE,
        mass: 332837.0,
        distance: 0.0,
        eccentricity: 0.0,
        radius: 109.12,
        rotation: 0.0,
        orbit: 0.0,
        inclination: 0.0,
    },
    Planet {
        name: "Mercury",
        colour: full_palette::AMBER_A400,
        mass: 0.055,
        distance: 0.387,
        eccentricity: 0.21,
        radius: 0.38,
        rotation: 58.65,
        orbit: 0.24,
        inclination: 7.0,
    },
    Planet {
        name: "Venus",
        colour: full_palette::ORANGE_A100,
        mass: 0.815,
        distance: 0.723,
        eccentricity: 0.01,
        radius: 0.95,
        rotation: 243.02,
        orbit: 0.62,
        inclination: 3.39,
    },
    Planet {
        name: "Earth",
        colour: full_palette::GREEN,
        mass: 1.0,
        distance: 1.0,
        eccentricity: 0.02,
        radius: 1.0,
        rotation: 1.0,
        orbit: 1.0,
        inclination: 0.0,
    },
    Planet {
        name: "Mars",
        colour: full_palette::RED_700,
        mass: 0.107,
        eccentricity: 0.09,
        distance: 1.523,
        radius: 0.53,
        rotation: 1.03,
        orbit: 1.88,
        inclination: 1.85,
    },
    Planet {
        name: "Jupiter",
        colour: full_palette::DEEPORANGE_400,
        mass: 317.85,
        distance: 5.2,
        eccentricity: 0.05,
        radius: 11.21,
        rotation: 0.41,
        orbit: 11.86,
        inclination: 1.31,
    },
    Planet {
        name: "Saturn",
        colour: full_palette::AMBER_300,
        mass: 95.16,
        distance: 9.58,
        eccentricity: 0.06,
        radius: 9.45,
        rotation: 0.44,
        orbit: 29.63,
        inclination: 2.49,
    },
    Planet {
        name: "Uranus",
        colour: full_palette::CYAN_A400,
        mass: 14.5,
        distance: 19.29,
        eccentricity: 0.05,
        radius: 4.01,
        rotation: 0.72,
        orbit: 84.75,
        inclination: 0.77,
    },
    Planet {
        name: "Neptune",
        colour: full_palette::LIGHTBLUE_A200,
        mass: 17.2,
        distance: 30.25,
        eccentricity: 0.01,
        radius: 3.88,
        rotation: 0.67,
        orbit: 166.34,
        inclination: 1.77,
    },
    Planet {
        name: "Pluto",
        colour: full_palette::GREY,
        mass: 0.0,
        distance: 39.51,
        eccentricity: 0.25,
        radius: 0.19,
        rotation: 6.39,
        orbit: 248.35,
        inclination: 17.5,
    },
];
