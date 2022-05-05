use bevy::{pbr::StandardMaterial, prelude::Color};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

const OBSTACLE_COUNT: u32 = 40;
const LIGHT_COUNT: u32 = 150;
const LEVEL_MIN_X: i32 = -10;
const LEVEL_MAX_X: i32 = 200;

/// A representation of a game level
pub struct Level {
    /// List of obstacles
    pub obstacles: Vec<Obstacle>,
    /// List of coordinates of the lights
    pub lights: Vec<(f32, f32)>,
    /// Seed used for generating this level
    pub seed: u64,
    /// Background objects
    pub bg_objects: Vec<BgObject>,
}

impl Level {
    pub fn new(seed: u64) -> Level {
        // "ChaCha8Rng is an excellent choice for a deterministic master generator"
        // https://rust-random.github.io/book/guide-seeding.html
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        let mut obstacles = Vec::new();
        let mut lights = Vec::new();
        let mut bg_objects = Vec::new();

        // Obstacles
        for _ in 0..OBSTACLE_COUNT {
            // TODO: better location algorithm for making sure every level is winnable
            let x: f32 = rng.gen_range(1.0..(LEVEL_MAX_X as f32));
            let y: f32 = rng.gen_range(0.0..1.0);

            let material = random_material(&mut rng);

            let obstacle = Obstacle { x, y, material };
            obstacles.push(obstacle);
        }

        // Lights
        for _ in 0..LIGHT_COUNT {
            let x: f32 = rng.gen_range(1.0..(LEVEL_MAX_X as f32));
            let y: f32 = rng.gen_range(0.0..10.0);
            lights.push((x, y));
        }

        // Background wall
        for x in LEVEL_MIN_X..LEVEL_MAX_X {
            for y in 0..10 {
                let x = x as f32;
                let y = y as f32;
                let z: f32 = -rng.gen_range(1.0..2.0);
                let material = random_material(&mut rng);
                let bg_object = BgObject { x, y, z, material };
                bg_objects.push(bg_object);
            }
        }

        Level {
            obstacles,
            lights,
            seed,
            bg_objects,
        }
    }
}

pub struct Obstacle {
    pub x: f32,
    pub y: f32,
    pub material: StandardMaterial,
}

pub struct BgObject {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub material: StandardMaterial,
}

pub fn random_material(rng: &mut ChaCha8Rng) -> StandardMaterial {
    // TODO: better material variance
    let metallic: f32 = rng.gen_range(0.0..1.0);
    let perceptual_roughness: f32 = rng.gen_range(0.0..1.0);
    let color: u64 = rng.gen_range(0..99_99_99);
    let color: String = format!("{:06}", color);

    StandardMaterial {
        base_color: Color::hex(color).unwrap(),
        metallic,
        perceptual_roughness,
        ..Default::default()
    }
}
