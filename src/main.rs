use bevy::{prelude::*,
    input::common_conditions::*,
    window::{PresentMode, WindowTheme},
};

// use bevy_editor_pls::prelude::*;

use minigolf::{
    Fonts, 
    GameState, 
    GameStateHandler, 
    GLBPurgeID, 
    GLBStorageID, 
    LevelState, 
    MapSetState,
    OpIndex,
};
use minigolf::user_interface::camera_world::setup_3d_camera;
use minigolf::user_interface::user_interface::{
    game_state_update, 
    fire_ray, 
    release_ray, 
    draw_cursor, 
    setup_ui
};
use minigolf::level_handler::level_handler::{
    init_hole_n, 
    level_state_update, 
    map_set_state_update, 
    setup_ground, 
    setup_light, 
    purge_glb_all
};

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
        .insert_state(LevelState::HoleTutorial)
        .insert_state(MapSetState::Tutorial)
        .insert_state(GameState::LoadingScreen)
        .insert_resource(Fonts::new())
        .insert_resource(OpIndex::new())
        .insert_resource(GLBPurgeID::new())
        .insert_resource(GLBStorageID::new())
        .insert_resource(GameStateHandler::new())
        .add_systems(Startup, setup_ground)
        .add_systems(Startup, setup_light)
        .add_systems(Startup, setup_ui)
        .add_systems(Startup, setup_3d_camera)
        .add_systems(Update, draw_cursor)
        .add_systems(Update, release_ray.run_if(input_just_released(MouseButton::Left)))
        .add_systems(Update, fire_ray.run_if(input_pressed(MouseButton::Left)))
        .add_systems(Update, game_state_update.run_if(input_just_released(KeyCode::ArrowLeft)))
        .add_systems(Update, map_set_state_update.run_if(input_just_released(KeyCode::ArrowRight)))
        .add_systems(Update, level_state_update.run_if(input_just_released(KeyCode::ArrowUp)))
        // .add_systems(OnEnter(MapSetState::Tutorial), init_hole_n)
        .add_systems(OnEnter(LevelState::HoleTutorial), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole1), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole2), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole3), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole4), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole5), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole6), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole7), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole8), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole9), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole10), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole11), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole12), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole13), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole14), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole15), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole16), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole17), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole18), init_hole_n)
        .add_systems(OnExit(LevelState::HoleTutorial), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole1), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole2), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole3), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole4), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole5), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole6), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole7), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole8), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole9), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole10), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole11), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole12), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole13), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole14), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole15), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole16), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole17), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole18), purge_glb_all);
        app.run();
}