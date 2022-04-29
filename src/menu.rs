use super::{despawn_screen, GameState};
use bevy::app::AppExit;
use bevy::prelude::*;
pub struct MainMenuPlugin;

const HEADING_REM: f32 = 80.0;
const BUTTON_WIDTH: f32 = 250.0;
const BUTTON_HEIGHT: f32 = 65.0;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // At start, the menu is not enabled. This will be changed in `menu_setup` when
            // entering the `GameState::MainMenu` state.
            // Current screen in the menu is handled by an independent state from `GameState`
            .add_state(MenuState::Disabled)
            .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(menu_setup))
            // Systems to handle the main menu screen
            .add_system_set(SystemSet::on_enter(MenuState::MainMenu).with_system(main_menu_setup))
            .add_system_set(
                SystemSet::on_exit(MenuState::MainMenu)
                    .with_system(despawn_screen::<OnMainMenuScreen>),
            )
            // Systems to handle the load game screen
            .add_system_set(
                SystemSet::on_enter(MenuState::LoadMenu).with_system(load_game_menu_setup),
            )
            .add_system_set(
                SystemSet::on_exit(MenuState::LoadMenu)
                    .with_system(despawn_screen::<OnLoadGameScreen>),
            )
            // Systems to handle the help menu screen
            .add_system_set(SystemSet::on_enter(MenuState::Help).with_system(help_menu_setup))
            .add_system_set(
                SystemSet::on_exit(MenuState::Help).with_system(despawn_screen::<OnHelpMenuScreen>),
            )
            // Common systems to all screens that handles buttons behaviour
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu)
                    .with_system(menu_action)
                    .with_system(button_system),
            );
    }
}

// State used for the current menu screen
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum MenuState {
    MainMenu,
    Help,
    LoadMenu,
    Disabled,
}

// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
struct OnMainMenuScreen;

// Tag component used to tag entities added on the help menu screen
#[derive(Component)]
struct OnHelpMenuScreen;

// Tag component used to tag entities added on the load game menu screen
#[derive(Component)]
struct OnLoadGameScreen;

// Tag component used to mark wich setting is currently selected
#[derive(Component)]
struct SelectedOption;

// All actions that can be triggered from a button click
#[derive(Component)]
enum MenuButtonAction {
    NewGame,
    Help,
    LoadMenu,
    BackToMainMenu,
    Quit,
}

// This system handles changing all buttons color based on mouse interaction
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in interaction_query.iter_mut() {
        *color = match (*interaction, selected) {
            (Interaction::Clicked, _) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

fn menu_setup(mut menu_state: ResMut<State<MenuState>>) {
    let _ = menu_state.set(MenuState::MainMenu);
}

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/undefined-medium.ttf");
    // Common style for all buttons on the screen
    let button_style = Style {
        size: Size::new(Val::Px(BUTTON_WIDTH), Val::Px(BUTTON_HEIGHT)),
        margin: Rect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: Color::WHITE,
    };

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: Rect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::ORANGE.into(),
            ..default()
        })
        .insert(OnMainMenuScreen)
        .with_children(|parent| {
            // Display the game name
            parent.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect::all(Val::Px(0.5 * HEADING_REM)),
                    ..default()
                },
                text: Text::with_section(
                    "Jump",
                    TextStyle {
                        font: font.clone(),
                        font_size: HEADING_REM,
                        color: Color::WHITE,
                    },
                    Default::default(),
                ),
                ..default()
            });

            // Display four buttons for each action available from the main menu:
            // - new game
            // - load game
            // - help
            // - quit
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(MenuButtonAction::NewGame)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "New Game",
                            button_text_style.clone(),
                            Default::default(),
                        ),
                        ..default()
                    });
                });
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(MenuButtonAction::LoadMenu)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Load Game",
                            button_text_style.clone(),
                            Default::default(),
                        ),
                        ..default()
                    });
                });
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(MenuButtonAction::Help)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Help",
                            button_text_style.clone(),
                            Default::default(),
                        ),
                        ..default()
                    });
                });
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style,
                    color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(MenuButtonAction::Quit)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Quit",
                            button_text_style.clone(),
                            Default::default(),
                        ),
                        ..default()
                    });
                });
        });
}

fn help_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_style = Style {
        size: Size::new(Val::Px(200.0), Val::Px(65.0)),
        margin: Rect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextStyle {
        font: asset_server.load("fonts/undefined-medium.ttf"),
        font_size: 40.0,
        color: Color::WHITE,
    };

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: Rect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::CRIMSON.into(),
            ..default()
        })
        .insert(OnHelpMenuScreen)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Unimplemented",
                    button_text_style.clone(),
                    Default::default(),
                ),
                ..default()
            });
            // Display the back button to return to the main menu screen
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style,
                    color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(MenuButtonAction::BackToMainMenu)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section("Back", button_text_style, Default::default()),
                        ..default()
                    });
                });
        });
}

fn load_game_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_style = Style {
        size: Size::new(Val::Px(200.0), Val::Px(65.0)),
        margin: Rect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextStyle {
        font: asset_server.load("fonts/undefined-medium.ttf"),
        font_size: 40.0,
        color: Color::WHITE,
    };

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: Rect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::CRIMSON.into(),
            ..default()
        })
        .insert(OnLoadGameScreen)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Unimplemented",
                    button_text_style.clone(),
                    Default::default(),
                ),
                ..default()
            });
            // Display the back button to return to the main menu screen
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style,
                    color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(MenuButtonAction::BackToMainMenu)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section("Back", button_text_style, Default::default()),
                        ..default()
                    });
                });
        });
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<State<MenuState>>,
    mut game_state: ResMut<State<GameState>>,
) {
    for (interaction, menu_button_action) in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            match menu_button_action {
                MenuButtonAction::Quit => app_exit_events.send(AppExit),
                MenuButtonAction::NewGame => {
                    game_state.set(GameState::Game).unwrap();
                    menu_state.set(MenuState::Disabled).unwrap();
                }
                MenuButtonAction::LoadMenu => menu_state.set(MenuState::LoadMenu).unwrap(),
                MenuButtonAction::Help => menu_state.set(MenuState::Help).unwrap(),
                MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::MainMenu).unwrap(),
            }
        }
    }
}
