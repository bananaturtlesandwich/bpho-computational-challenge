#![allow(mixed_script_confusables)]

use egui_plotter::Chart;
use plotters::style::full_palette;

mod angles;
mod centre;
mod kepler;
mod orbits;
mod spiral;

#[derive(PartialEq, Clone)]
enum Tab {
    Kepler,
    Orbits,
    Angles,
    Spiral,
    Centre,
}

pub struct App {
    kepler: Chart<f32>,
    orbits: Chart<(f32, instant::Instant, f32, bool, bool)>,
    angles: Chart<(usize, Vec<(f32, f32)>)>,
    spiral: Chart<(usize, usize, Vec<[(f32, f32); 2]>)>,
    centre: Chart<(f32, usize, Vec<Vec<(f32, f32)>>)>,
    tab: Tab,
}

impl App {
    pub fn new(ctx: &eframe::CreationContext) -> Self {
        // prevents artifacts on graphs
        ctx.egui_ctx
            .tessellation_options_mut(|tes| tes.feathering = false);
        let mut app = Self {
            kepler: Chart::new(1.0).builder_cb(Box::new(kepler::plot)),
            orbits: Chart::new((1.0, instant::Instant::now(), 1.0, false, false))
                .pitch(0.3)
                .yaw(-0.7)
                .mouse(egui_plotter::MouseConfig::default().rotate(true))
                .builder_cb(Box::new(orbits::plot)),
            angles: Chart::new((8, Vec::new())).builder_cb(Box::new(angles::plot)),
            spiral: Chart::new((1, 2, Vec::new())).builder_cb(Box::new(spiral::plot)),
            centre: Chart::new((1.0, 2, Vec::new())).builder_cb(Box::new(centre::plot)),
            tab: Tab::Kepler,
        };
        app.angles();
        app.spiral();
        app.centre();
        app
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        use eframe::egui;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                let mut tab = |name: &str, demo: Tab| {
                    if ui.selectable_label(self.tab == demo, name).clicked() {
                        self.tab = demo;
                    }
                };
                tab("Kepler's third law", Tab::Kepler);
                tab("Orbits", Tab::Orbits);
                tab("Orbit angle vs time", Tab::Angles);
                tab("Spirographs", Tab::Spiral);
                tab("Relative orbits", Tab::Centre);
                fn planets(ui: &mut egui::Ui, id: &str, i: &mut usize) -> egui::Response {
                    egui::ComboBox::from_id_source(id)
                        .show_index(ui, i, PLANETS.len(), |i| PLANETS[i].name)
                }
                match self.tab {
                    Tab::Orbits => {
                        ui.checkbox(&mut self.orbits.get_data_mut().3, "animated");
                        ui.checkbox(&mut self.orbits.get_data_mut().4, "3d");
                        if self.orbits.get_data().3 {
                            ui.add(
                                egui::Slider::new(&mut self.orbits.get_data_mut().2, 0.1..=10.0)
                                    .suffix(" years/second"),
                            );
                        }
                    }
                    Tab::Angles
                        if planets(ui, "angles", &mut self.angles.get_data_mut().0).changed() =>
                    {
                        self.angles()
                    }
                    Tab::Spiral
                        if planets(ui, "p1", &mut self.spiral.get_data_mut().0).changed()
                            | planets(ui, "p2", &mut self.spiral.get_data_mut().1).changed() =>
                    {
                        self.spiral()
                    }
                    Tab::Centre
                        if planets(ui, "centre", &mut self.centre.get_data_mut().1).changed() =>
                    {
                        self.centre()
                    }
                    _ => (),
                }
                if matches!(self.tab, Tab::Kepler | Tab::Orbits | Tab::Centre) {
                    ui.label("zoom: scroll/pinch");
                }
                if matches!(self.tab, Tab::Orbits if self.orbits.get_data().4) {
                    ui.label("rotate: click + drag");
                }
            });
            ui.vertical_centered_justified(|ui| match self.tab {
                Tab::Kepler => self.kepler.draw(ui),
                Tab::Orbits => self.orbits.draw(ui),
                Tab::Angles => self.angles.draw(ui),
                Tab::Spiral => self.spiral.draw(ui),
                Tab::Centre => self.centre.draw(ui),
            });
            ui.input(|e| {
                let set = |scale: &mut f32| {
                    *scale *= 0.99_f32.powf(e.scroll_delta.y);
                    *scale /= e.zoom_delta();
                    *scale = scale.clamp(0.01, 1.0);
                };
                match self.tab {
                    Tab::Kepler => set(self.kepler.get_data_mut()),
                    Tab::Orbits => set(&mut self.orbits.get_data_mut().0),
                    Tab::Centre => set(&mut self.centre.get_data_mut().0),
                    _ => (),
                }
            });
            if matches!(self.tab, Tab::Orbits if self.orbits.get_data().3) {
                ctx.request_repaint()
            }
        });
    }
}

#[derive(PartialEq)]
struct Planet {
    name: &'static str,
    colour: plotters::style::RGBColor,
    distance: f32,
    eccentricity: f32,
    radius: f32,
    orbit: f32,
    inclination: f32,
}

impl Planet {
    fn coord(&self, θ: f32) -> (f32, f32) {
        let (sin, cos) = θ.sin_cos();
        let r =
            (self.distance * (1.0 - self.eccentricity.powi(2))) / (1.0 - self.eccentricity * cos);
        (r * cos, r * sin)
    }
    fn position(&self, θ: f32) -> (f32, f32, f32) {
        let (x, y) = self.coord(θ);
        let (sin, cos) = self.inclination.to_radians().sin_cos();
        (x * cos, x * sin, y)
    }
    fn angle(&self, years: f32) -> f32 {
        2.0 * std::f32::consts::PI * years / self.orbit
    }
    fn coord_when(&self, planet: &Self, θ: f32) -> (f32, f32) {
        self.coord(θ * planet.orbit / self.orbit)
    }
    fn near(&self) -> bool {
        self.distance <= 1.0
    }
}

const PLANETS: [Planet; 9] = [
    Planet {
        name: "Mercury",
        colour: full_palette::AMBER_A400,
        distance: 0.387,
        eccentricity: 0.21,
        radius: 0.38,
        orbit: 0.24,
        inclination: 7.0,
    },
    Planet {
        name: "Venus",
        colour: full_palette::ORANGE_A100,
        distance: 0.723,
        eccentricity: 0.01,
        radius: 0.95,
        orbit: 0.62,
        inclination: 3.39,
    },
    Planet {
        name: "Earth",
        colour: full_palette::GREEN,
        distance: 1.0,
        eccentricity: 0.02,
        radius: 1.0,
        orbit: 1.0,
        inclination: 0.0,
    },
    Planet {
        name: "Mars",
        colour: full_palette::RED_700,
        eccentricity: 0.09,
        distance: 1.523,
        radius: 0.53,
        orbit: 1.88,
        inclination: 1.85,
    },
    Planet {
        name: "Jupiter",
        colour: full_palette::DEEPORANGE_400,
        distance: 5.2,
        eccentricity: 0.05,
        radius: 11.21,
        orbit: 11.86,
        inclination: 1.31,
    },
    Planet {
        name: "Saturn",
        colour: full_palette::AMBER_300,
        distance: 9.58,
        eccentricity: 0.06,
        radius: 9.45,
        orbit: 29.63,
        inclination: 2.49,
    },
    Planet {
        name: "Uranus",
        colour: full_palette::CYAN_A400,
        distance: 19.29,
        eccentricity: 0.05,
        radius: 4.01,
        orbit: 84.75,
        inclination: 0.77,
    },
    Planet {
        name: "Neptune",
        colour: full_palette::LIGHTBLUE_A200,
        distance: 30.25,
        eccentricity: 0.01,
        radius: 3.88,
        orbit: 166.34,
        inclination: 1.77,
    },
    // my favourite planet >:p
    Planet {
        name: "Pluto",
        colour: full_palette::GREY,
        distance: 39.51,
        eccentricity: 0.25,
        radius: 0.19,
        orbit: 248.35,
        inclination: 17.5,
    },
];
