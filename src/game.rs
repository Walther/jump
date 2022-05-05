use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::{core::FixedTimestep, prelude::*};

use crate::level::Level;

use super::{despawn_screen, GameState};

/// Lockstep for the game engine
const TIME_STEP: f32 = 1.0 / 60.0;

/// Initial upwareds velocity for the jump
const JUMP_INITIAL_VELOCITY: f32 = 5.0;
/// Gravity constant for the jump
const GRAVITY: f32 = 5.0;

/// Default movement speed in the autoscroller
const SCROLL_VELOCITY: f32 = 2.0;
/// Boost velocity when the boost button is pressed
const BOOST_VELOCITY: f32 = 5.0;

/// Radius of the spheres, both for player and obstacles
const SPHERE_RADIUS: f32 = 0.5;

/// Fake unit for font-related calculations for visual consistency
const REM: f32 = 24.0;

/// Initial fixed testing seed, will use a dynamic one later on
const FIXED_RNG_SEED: u64 = 0x1234_5678;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app // load-bearing comment, better readability for chains below
            .add_system_set(SystemSet::on_enter(GameState::Game).with_system(game_setup))
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(player_movement_system)
                    .with_system(camera_movement_system)
                    .with_system(check_for_collisions)
                    .with_system(fps_text_update_system)
                    .with_system(score_text_update_system),
            )
            .add_event::<CollisionEvent>()
            .add_system_set(
                SystemSet::on_exit(GameState::Game).with_system(despawn_screen::<OnGameScreen>),
            );
    }
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

/// set up a simple 3D scene
fn game_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // TODO: load a seed given by the user in the Load game menu
    let level = Level::new(FIXED_RNG_SEED);

    // spheres to jump over
    for obstacle in level.obstacles {
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: SPHERE_RADIUS,
                    subdivisions: 32,
                })),
                material: materials.add(obstacle.material),
                transform: Transform::from_xyz(obstacle.x, obstacle.y, 0.0),
                ..Default::default()
            })
            .insert(Obstacle)
            .insert(Collider);
    }

    // lights
    for (x, y) in level.lights {
        commands.spawn_bundle(PointLightBundle {
            transform: Transform::from_translation(Vec3::new(x, y, 10.0)),
            point_light: PointLight {
                intensity: 10_000.,
                range: 15.,
                shadows_enabled: true,
                ..Default::default()
            },
            ..Default::default()
        });
    }

    // background objects
    for bg_object in level.bg_objects {
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(bg_object.material),
            transform: Transform::from_xyz(bg_object.x, bg_object.y, bg_object.z),
            ..Default::default()
        });
    }

    // background wall
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Quad {
            size: (1000.0, 1000.0).into(),
            flip: false,
        })),
        material: materials.add(StandardMaterial {
            base_color: Color::hex("444444").unwrap(),
            metallic: 0.5,
            perceptual_roughness: 1.0,
            ..Default::default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, -5.0),
        ..Default::default()
    });

    // floor
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box {
            min_x: -1_000.0,
            max_x: 1_000.0,
            min_y: -10.0,
            max_y: -0.5,
            min_z: -5.0,
            max_z: 5.0,
        })),
        material: materials.add(StandardMaterial {
            base_color: Color::hex("272822").unwrap(),
            metallic: 0.5,
            perceptual_roughness: 0.5,
            ..Default::default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, -5.0),
        ..Default::default()
    });

    // player
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: SPHERE_RADIUS,
                subdivisions: 32,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                perceptual_roughness: 0.01,
                metallic: 0.8,
                reflectance: 1.0,
                ..Default::default()
            }),
            transform: Transform::from_xyz(-5.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(Player::default());

    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 0.0, 8.0)
                .looking_at(Vec3::new(0.0, 2.5, 0.0), Vec3::Y),
            ..default()
        })
        .insert(Camera::default());

    // fps counter
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(0.5 * REM),
                    left: Val::Px(0.5 * REM),
                    ..default()
                },
                ..default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/undefined-medium.ttf"),
                            font_size: REM,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/undefined-medium.ttf"),
                            font_size: REM,
                            color: Color::WHITE,
                        },
                    },
                ],
                ..default()
            },
            ..default()
        })
        .insert(FpsText);

    // score counter
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(0.5 * REM),
                    right: Val::Px(0.5 * REM),
                    ..default()
                },
                ..default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Score: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/undefined-medium.ttf"),
                            font_size: REM,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/undefined-medium.ttf"),
                            font_size: REM,
                            color: Color::WHITE,
                        },
                    },
                ],
                ..default()
            },
            ..default()
        })
        .insert(ScoreText);

    // seed
    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::Center,
            position_type: PositionType::Absolute,
            position: Rect {
                bottom: Val::Px(0.0),
                left: Val::Px(0.5 * REM),
                ..default()
            },
            ..default()
        },
        text: Text {
            sections: vec![TextSection {
                value: format!("Seed: {:#x}", FIXED_RNG_SEED),
                style: TextStyle {
                    font: asset_server.load("fonts/undefined-medium.ttf"),
                    font_size: REM,
                    color: Color::WHITE,
                },
            }],
            ..default()
        },
        ..default()
    });
}

#[derive(Component)]
struct Player {
    jumping: JumpState,
    collided: bool,
    velocity_x: f32,
    velocity_y: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            jumping: JumpState::OnFloor,
            velocity_x: SCROLL_VELOCITY,
            velocity_y: 0.0,
            collided: false,
        }
    }
}

enum JumpState {
    OnFloor,
    InAir,
}

#[derive(Component)]
struct Obstacle;

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct FpsText;

// A unit struct to help identify the Score UI component, since there may be many Text components
#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct Collider;

#[derive(Default)]
struct CollisionEvent;

fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform)>,
) {
    // fallibility check needed as entities don't exist yet in menus
    let (mut player, mut transform) = match query.get_single_mut() {
        Ok(val) => val,
        Err(_) => return,
    };

    if player.collided {
        return;
    }

    // x direction
    if keyboard_input.pressed(KeyCode::Right) {
        player.velocity_x = BOOST_VELOCITY;
    } else {
        player.velocity_x = SCROLL_VELOCITY;
    }

    let translation = &mut transform.translation;
    translation.x += player.velocity_x * TIME_STEP;

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
struct Camera {
    velocity_x: f32,
    stopped: bool,
}

impl Camera {
    fn default() -> Self {
        Camera {
            velocity_x: SCROLL_VELOCITY,
            stopped: false,
        }
    }
}

fn camera_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Camera, &mut Transform)>,
) {
    // fallibility check needed as entities don't exist yet in menus
    let (mut camera, mut transform) = match query.get_single_mut() {
        Ok(val) => val,
        Err(_) => return,
    };
    if camera.stopped {
        return;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        camera.velocity_x = BOOST_VELOCITY;
    } else {
        camera.velocity_x = SCROLL_VELOCITY;
    }

    let translation = &mut transform.translation;
    translation.x += camera.velocity_x * TIME_STEP;
}

fn fps_text_update_system(
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    // fallibility check needed as entities don't exist yet in menus
    let mut fpstext = match query.get_single_mut() {
        Ok(val) => val,
        Err(_) => return,
    };
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            // Update the value of the second section
            fpstext.sections[1].value = format!("{:.2}", average);
        }
    }
}

fn score_text_update_system(
    mut query: Query<&mut Text, With<ScoreText>>,
    mut playertransform_query: Query<(&Player, &Transform)>,
) {
    // fallibility check needed as entities don't exist yet in menus
    let (_player, transform) = match playertransform_query.get_single_mut() {
        Ok(val) => val,
        Err(_) => return,
    };
    let mut scoretext = match query.get_single_mut() {
        Ok(val) => val,
        Err(_) => return,
    };
    scoretext.sections[1].value = format!("{:.2}", transform.translation.x);
}

fn check_for_collisions(
    mut player_query: Query<(&mut Player, &Transform)>,
    collider_query: Query<(Entity, &Transform, Option<&Obstacle>), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
    mut camera_query: Query<&mut Camera>,
) {
    // fallibility check needed as entities don't exist yet in menus
    let (mut player, player_trans) = match player_query.get_single_mut() {
        Ok(val) => val,
        Err(_) => return,
    };
    let mut camera = match camera_query.get_single_mut() {
        Ok(val) => val,
        Err(_) => return,
    };
    let (x1, y1) = (player_trans.translation.x, player_trans.translation.y);
    // Simple sphere collision based on center and radius
    for (_sphere_ent, sphere_trans, _sphere_obs) in collider_query.iter() {
        let (x2, y2) = (sphere_trans.translation.x, sphere_trans.translation.y);
        let distance = ((x2 - x1).powf(2.0) + (y2 - y1).powf(2.0)).sqrt();
        if distance <= SPHERE_RADIUS * 2.0 {
            collision_events.send_default();
            player.collided = true;
            camera.stopped = true;
        }
    }
}
