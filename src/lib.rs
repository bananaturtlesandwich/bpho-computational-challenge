use plotters::style::full_palette;

mod anim2d;
mod anim3d;
mod kepler;
mod orbits;

#[derive(PartialEq, Clone)]
enum Demo {
    Kepler,
    Orbits,
    Anim2D,
    Anim3D,
}

pub struct App {
    kepler: egui_plotter::Chart<f32>,
    orbits: egui_plotter::Chart<f32>,
    anim2d: egui_plotter::Chart<(f32, instant::Instant, f32)>,
    anim3d: egui_plotter::Chart<(f32, instant::Instant, f32)>,
    current: Demo,
}

impl App {
    pub fn new(ctx: &eframe::CreationContext) -> Self {
        use egui_plotter::*;
        // prevents artifacts on graphs
        ctx.egui_ctx
            .tessellation_options_mut(|tes| tes.feathering = false);
        Self {
            kepler: Chart::new(1.0).builder_cb(Box::new(kepler::plot)),
            orbits: Chart::new(1.0).builder_cb(Box::new(orbits::plot)),
            anim2d: Chart::new((1.0, instant::Instant::now(), 1.0))
                .builder_cb(Box::new(anim2d::plot)),
            anim3d: Chart::new((1.0, instant::Instant::now(), 1.0))
                .pitch(0.3)
                .yaw(0.7)
                .mouse(MouseConfig::default().rotate(true))
                .builder_cb(Box::new(anim3d::plot)),
            current: Demo::Kepler,
        }
    }
    fn animating(&self) -> bool {
        matches!(self.current, Demo::Anim2D | Demo::Anim3D)
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        use eframe::egui;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                let mut tab = |name: &str, demo: Demo| {
                    if ui.selectable_label(self.current == demo, name).clicked() {
                        self.current = demo;
                    }
                };
                tab("Kepler's third law", Demo::Kepler);
                tab("Orbits", Demo::Orbits);
                tab("2D animated orbits", Demo::Anim2D);
                tab("3D animated orbits", Demo::Anim3D);
                ui.set_enabled(self.animating());
                let mut stub = 1.0;
                ui.add(
                    egui::Slider::new(
                        match self.current.clone() {
                            Demo::Anim2D => &mut self.anim2d.get_data_mut().2,
                            Demo::Anim3D => &mut self.anim3d.get_data_mut().2,
                            _ => &mut stub,
                        },
                        0.1..=10.0,
                    )
                    .suffix(" years/second"),
                );
            });
            ui.vertical_centered_justified(|ui| match self.current {
                Demo::Kepler => self.kepler.draw(ui),
                Demo::Orbits => self.orbits.draw(ui),
                Demo::Anim2D => self.anim2d.draw(ui),
                Demo::Anim3D => self.anim3d.draw(ui),
            });
            ui.input(|e| {
                let set = |scale: &mut f32| {
                    *scale *= 0.99_f32.powf(e.scroll_delta.y);
                    *scale /= e.zoom_delta();
                    *scale = scale.clamp(0.01, 1.0);
                };
                match self.current {
                    Demo::Kepler => set(self.kepler.get_data_mut()),
                    Demo::Orbits => set(self.orbits.get_data_mut()),
                    Demo::Anim2D => set(&mut self.anim2d.get_data_mut().0),
                    Demo::Anim3D => set(&mut self.anim3d.get_data_mut().0),
                }
            });
            if self.animating() {
                ctx.request_repaint()
            }
        });
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
