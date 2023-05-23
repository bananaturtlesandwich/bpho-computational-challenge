fn main() {}

struct Planet {
    name: &'static str,
    mass: f32,
    distance: f32,
    radius: f32,
    rotation: f32,
    orbit: f32,
}

const PLANETS: [Planet; 10] = [
    // my favourite planet >:p
    Planet {
        name: "Sun",
        mass: 332837.0,
        distance: 0.0,
        radius: 109.12,
        rotation: 0.0,
        orbit: 0.0,
    },
    Planet {
        name: "Mercury",
        mass: 0.055,
        distance: 0.387,
        radius: 0.38,
        rotation: 58.65,
        orbit: 0.24,
    },
    Planet {
        name: "Venus",
        mass: 0.815,
        distance: 0.723,
        radius: 0.95,
        rotation: 243.02,
        orbit: 0.62,
    },
    Planet {
        name: "Earth",
        mass: 1.0,
        distance: 1.0,
        radius: 1.0,
        rotation: 1.0,
        orbit: 1.0,
    },
    Planet {
        name: "Mars",
        mass: 0.107,
        distance: 1.523,
        radius: 0.53,
        rotation: 1.03,
        orbit: 1.88,
    },
    Planet {
        name: "Saturn",
        mass: 95.16,
        distance: 9.58,
        radius: 9.45,
        rotation: 0.44,
        orbit: 29.63,
    },
    Planet {
        name: "Jupiter",
        mass: 317.85,
        distance: 5.2,
        radius: 11.21,
        rotation: 0.41,
        orbit: 11.86,
    },
    Planet {
        name: "Uranus",
        mass: 14.5,
        distance: 19.29,
        radius: 4.01,
        rotation: 0.72,
        orbit: 84.75,
    },
    Planet {
        name: "Neptune",
        mass: 17.2,
        distance: 30.25,
        radius: 3.88,
        rotation: 0.67,
        orbit: 166.34,
    },
    Planet {
        name: "Pluto",
        mass: 0.0,
        distance: 39.51,
        radius: 0.19,
        rotation: 6.39,
        orbit: 248.35,
    },
];
