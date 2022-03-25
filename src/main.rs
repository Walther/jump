use bevy::{core::FixedTimestep, prelude::*};
const TIME_STEP: f32 = 1.0 / 60.0;

const JUMP_INITIAL_VELOCITY: f32 = 5.0;
const GRAVITY: f32 = 5.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(player_movement_system)
                .with_system(camera_movement_system),
        )
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // spheres to jump over
    let y = 0;
    for x in 0..=5 {
        let x01 = (x) as f32 / 10.0;
        let y01 = (y) as f32 / 4.0;
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 0.45,
                subdivisions: 32,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("777777").unwrap(),
                // vary key PBR parameters on a grid of spheres to show the effect
                metallic: y01,
                perceptual_roughness: x01,
                ..Default::default()
            }),
            transform: Transform::from_xyz(x as f32 * 3.0, y as f32, 0.0),
            ..Default::default()
        });
    }
    // player
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 0.45,
                subdivisions: 32,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("ff8d0c").unwrap(),
                metallic: 0.5,
                perceptual_roughness: 0.5,
                ..Default::default()
            }),
            transform: Transform::from_xyz(-5.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(Player::default());
    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(50.0, 50.0, 50.0)),
        point_light: PointLight {
            intensity: 600000.,
            range: 100.,
            ..Default::default()
        },
        ..Default::default()
    });
    // camera
    commands
        .spawn_bundle(OrthographicCameraBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 8.0))
                .looking_at(Vec3::new(0.0, 2.5, 0.0), Vec3::Y),
            orthographic_projection: OrthographicProjection {
                scale: 0.01,
                ..Default::default()
            },
            ..OrthographicCameraBundle::new_3d()
        })
        .insert(Camera);
}

#[derive(Component)]
struct Player {
    jumping: JumpState,
    velocity_y: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            jumping: JumpState::OnFloor,
            velocity_y: 0.0,
        }
    }
}

enum JumpState {
    OnFloor,
    InAir,
}

fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform)>,
) {
    let (mut player, mut transform) = query.single_mut();
    // x direction
    let mut direction_x = 0.0;
    if keyboard_input.pressed(KeyCode::Left) {
        direction_x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction_x += 1.0;
    }

    let translation = &mut transform.translation;
    translation.x += direction_x * TIME_STEP;

    // y direction
    if keyboard_input.pressed(KeyCode::Space) {
        match &player.jumping {
            JumpState::OnFloor => {
                player.jumping = JumpState::InAir;
                player.velocity_y = JUMP_INITIAL_VELOCITY;
            }
            JumpState::InAir => {}
        }
    }

    // floor min height
    if translation.y < 0.0 {
        player.jumping = JumpState::OnFloor;
        player.velocity_y = 0.0;
        translation.y = 0.0;
    }

    player.velocity_y -= GRAVITY * TIME_STEP;
    let velocity = player.velocity_y;

    // dbg!(&translation.y);
    match player.jumping {
        JumpState::OnFloor => translation.y = 0.0,
        JumpState::InAir => translation.y += velocity * TIME_STEP,
    }
}

// TODO: remove duplicate code...
#[derive(Component)]
struct Camera;

fn camera_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Camera, &mut Transform)>,
) {
    let (_camera, mut transform) = query.single_mut();
    let mut direction = 0.0;
    if keyboard_input.pressed(KeyCode::Left) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction += 1.0;
    }

    let translation = &mut transform.translation;
    translation.x += direction * TIME_STEP;
}