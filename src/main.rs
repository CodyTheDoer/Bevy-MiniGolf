use bevy::{prelude::*,
    ecs::world::World,
    input::common_conditions::*,
    window::{PresentMode, WindowTheme},
};
// use bevy_editor_pls::prelude::*;

use minigolf::{Fonts, OpIndex};
use minigolf::level_handler::level_handler::{gltf_handler_init, setup_ground, setup_light, query_and_despawn_scene, query_and_update_scene};
use minigolf::user_interface::camera_world::setup_3d_camera;
use minigolf::user_interface::user_interface::{fire_ray, release_ray, draw_cursor, setup_ui};

fn main() {
    let mut app = App::new();
        app.add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Calculator Simulator".into(),
                    name: Some("bevy.app".into()),
                    resolution: (1280., 720.).into(),
                    resizable: true,
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: true,
                        ..Default::default()
                    },
                    present_mode: PresentMode::AutoVsync,
                    prevent_default_event_handling: false, // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    window_theme: Some(WindowTheme::Dark),
                    visible: true,
                    ..default()
                }),
                ..default()
            }),
        ))
        // .add_plugins(EditorPlugin::default())
        .insert_state(AppState::Game)
        .insert_resource(Fonts::new())
        .insert_resource(OpIndex::new())
        .add_systems(Startup, gltf_handler_init)
        .add_systems(Startup, setup_ground)
        .add_systems(Startup, setup_light)
        .add_systems(Startup, setup_ui)
        .add_systems(Startup, setup_3d_camera)
        .add_systems(Update, draw_cursor)
        .add_systems(Update, release_ray.run_if(input_just_released(MouseButton::Left)))
        .add_systems(Update, fire_ray.run_if(input_pressed(MouseButton::Left)))
        .add_systems(Update, query_and_despawn_scene.run_if(input_pressed(MouseButton::Right)))
        .add_systems(Update, query_and_update_scene.run_if(input_pressed(MouseButton::Right)))
        .add_systems(Update, app_state_logic)
        .add_systems(Update, app_state_cycle.run_if(input_pressed(KeyCode::ArrowUp)))
        .add_systems(OnEnter(AppState::Game), app_state_game_logic_enter)
        .add_systems(OnExit(AppState::Game), app_state_game_logic_exit)
        .add_systems(OnEnter(AppState::Menu), app_state_menu_logic_enter)
        .add_systems(OnExit(AppState::Menu), app_state_menu_logic_exit)
        .add_systems(OnEnter(AppState::Paused), app_state_paused_logic_enter)
        .add_systems(OnExit(AppState::Paused), app_state_paused_logic_exit);
        app.run();
}

// fn debug_current_gamemode_state(state: Res<State<AppState>>) {
//     eprintln!("Current state: {:?}", state.get());
// }

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
enum AppState {
    #[default]
    Game,
    Menu,
    Paused,
}

fn app_state_cycle(
    app_state: Res<State<AppState>>,
    mut next_game_state: ResMut<NextState<AppState>>,
) {
    match app_state.get() {
        AppState::Game => {
            next_game_state.set(AppState::Menu);
        },
        AppState::Menu => {
            next_game_state.set(AppState::Paused);
        },
        AppState::Paused => {
            next_game_state.set(AppState::Game);
        },
        _ => {},
    }
}

fn app_state_logic(
    app_state: Res<State<AppState>>,
) {
    match app_state.get() {
        AppState::Game => {
            // info!("AppState::Game");
        },
        AppState::Menu => {
            // info!("AppState::Menu");
        },
        AppState::Paused => {
            // info!("AppState::Paused");
        },
        _ => {},
    }
}

fn app_state_game_logic_enter() {
    info!("AppState::Game::OnEnter");
}

fn app_state_game_logic_exit() {
    info!("AppState::Game::OnExit");
}

fn app_state_menu_logic_enter() {
    info!("AppState::Menu::OnEnter");
}

fn app_state_menu_logic_exit() {
    info!("AppState::Menu::OnExit");
}

fn app_state_paused_logic_enter() {
    info!("AppState::Paused::OnEnter");
}

fn app_state_paused_logic_exit() {
    info!("AppState::Paused::OnExit");
}