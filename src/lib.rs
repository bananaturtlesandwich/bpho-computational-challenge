#![allow(mixed_script_confusables)]

use plotters::style::full_palette;

mod angles;
mod anim2d;
mod anim3d;
mod kepler;
mod orbits;

#[derive(PartialEq, Clone)]
enum Tab {
    Kepler,
    Orbits,
    Anim2D,
    Anim3D,
    Angles,
}

pub struct App {
    kepler: egui_plotter::Chart<f32>,
    orbits: egui_plotter::Chart<f32>,
    anim2d: egui_plotter::Chart<(f32, instant::Instant, f32)>,
    anim3d: egui_plotter::Chart<(f32, instant::Instant, f32)>,
    angles: egui_plotter::Chart<(usize, Vec<(f32, f32)>)>,
    tab: Tab,
}

impl App {
    pub fn new(ctx: &eframe::CreationContext) -> Self {
        use egui_plotter::*;
        // prevents artifacts on graphs
        ctx.egui_ctx
            .tessellation_options_mut(|tes| tes.feathering = false);
        let mut app = Self {
            kepler: Chart::new(1.0).builder_cb(Box::new(kepler::plot)),
            orbits: Chart::new(1.0).builder_cb(Box::new(orbits::plot)),
            anim2d: Chart::new((1.0, instant::Instant::now(), 1.0))
                .builder_cb(Box::new(anim2d::plot)),
            anim3d: Chart::new((1.0, instant::Instant::now(), 1.0))
                .pitch(0.3)
                .yaw(0.7)
                .mouse(MouseConfig::default().rotate(true))
                .builder_cb(Box::new(anim3d::plot)),
            angles: Chart::new((8, Vec::new())).builder_cb(Box::new(angles::plot)),
            tab: Tab::Kepler,
        };
        app.precompute_angles();
        app
    }
    fn precompute_angles(&mut self) {
        use plotters::prelude::*;
        let planet = &PLANETS[self.angles.get_data().0];
        let vals: Vec<_> = (0_f32..20.0)
            .step(0.001)
            .values()
            .map(|θ| (1.0 - planet.eccentricity * θ.cos()).powi(-2))
            .collect();
        self.angles.get_data_mut().1 = (0.01_f32..20.0)
            .step(0.1)
            .values()
            .map(|y| {
                let mut vals = vals[..(y * 1000.0) as usize].to_vec();
                let len = vals.len();
                for (i, val) in vals[1..len - 2].iter_mut().enumerate() {
                    *val *= if i % 2 == 1 { 4.0 } else { 2.0 }
                }
                (
                    planet.orbit
                        * (1.0 - planet.eccentricity.powi(2)).powf(1.5)
                        // 1/2π
                        *  std::f32::consts::FRAC_1_PI / 2.0
                        // h/3
                        * 0.001
                        / 3.0
                        * vals.into_iter().sum::<f32>(),
                    y,
                )
            })
            .collect();
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
                tab("2D animated orbits", Tab::Anim2D);
                tab("3D animated orbits", Tab::Anim3D);
                tab("Orbit angle vs time", Tab::Angles);
                match self.tab {
                    Tab::Anim2D => {
                        ui.add(
                            egui::Slider::new(&mut self.anim2d.get_data_mut().2, 0.1..=10.0)
                                .suffix(" years/second"),
                        );
                    }
                    Tab::Anim3D => {
                        ui.add(
                            egui::Slider::new(&mut self.anim3d.get_data_mut().2, 0.1..=10.0)
                                .suffix(" years/second"),
                        );
                    }
                    Tab::Angles => {
                        if egui::ComboBox::from_id_source("angles")
                            .show_index(ui, &mut self.angles.get_data_mut().0, PLANETS.len(), |i| {
                                PLANETS[i].name
                            })
                            .changed()
                        {
                            self.precompute_angles()
                        }
                    }
                    _ => (),
                }
            });
            ui.vertical_centered_justified(|ui| match self.tab {
                Tab::Kepler => self.kepler.draw(ui),
                Tab::Orbits => self.orbits.draw(ui),
                Tab::Anim2D => self.anim2d.draw(ui),
                Tab::Anim3D => self.anim3d.draw(ui),
                Tab::Angles => self.angles.draw(ui),
            });
            ui.input(|e| {
                let set = |scale: &mut f32| {
                    *scale *= 0.99_f32.powf(e.scroll_delta.y);
                    *scale /= e.zoom_delta();
                    *scale = scale.clamp(0.01, 1.0);
                };
                match self.tab {
                    Tab::Kepler => set(self.kepler.get_data_mut()),
                    Tab::Orbits => set(self.orbits.get_data_mut()),
                    Tab::Anim2D => set(&mut self.anim2d.get_data_mut().0),
                    Tab::Anim3D => set(&mut self.anim3d.get_data_mut().0),
                    _ => (),
                }
            });
            if matches!(self.tab, Tab::Anim2D | Tab::Anim3D) {
                ctx.request_repaint()
            }
        });
    }
}

struct Planet {
    name: &'static str,
    colour: plotters::style::RGBColor,
    distance: f32,
    eccentricity: f32,
    radius: f32,
    orbit: f32,
    inclination: f32,
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
