// --- Internal Bevy Plugins --- //
use bevy::{prelude::*,
    input::common_conditions::*,
    window::{PresentMode, WindowTheme},
};

// --- External Plugins --- //
use bevy_rapier3d::prelude::*;
// use bevy_editor_pls::prelude::*;

// --- States --- //
use minigolf::{ 
    ArrowState,
    CameraOrbitEntityState,
    GameState, 
    LevelState, 
    MapSetState,
};

// --- Resources --- //
use minigolf::{
    BonkHandler,
    CameraCoordTracker,
    CameraOrbitEntityStateHandler,
    Fonts, 
    GameStateHandler, 
    GLBStorageID, 
    OpIndex,
};

// --- User Interface Import --- //
use minigolf::user_interface::camera_world::{
    setup_3d_camera,
    pan_orbit_camera, 
    camera_orbit_entity_state_logic,
    camera_orbit_entity_state_update,
};
use minigolf::user_interface::user_interface::{
    game_state_update, 
    fire_ray, 
    release_ray, 
    draw_cursor, 
    setup_ui
};

// --- Level Handler Import --- //
use minigolf::level_handler::level_handler::{
    init_hole_n, 
    level_state_logic, 
    level_state_update, 
    map_set_state_update, 
    setup_ground, 
    setup_light, 
    purge_glb_all
};

// --- Physics Handler Import --- //
use minigolf::level_handler::physics_handler::{
    add_physics_query_and_update_scene,
    bonk_gizmo,
    bonk_step_start,
    bonk_step_mid,
    bonk_step_end,
    collision_events_listener,
    performance_physics_setup,
};

fn main() {
    let mut app = App::new();
        app.add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Minigolf".into(),
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

        // --- Additional Plugins --- //
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugins(RapierDebugRenderPlugin::default())
        // .add_plugins(EditorPlugin::default())

        // --- State Initialization --- //
        .insert_state(ArrowState::Idle)
        .insert_state(CameraOrbitEntityState::Ball)
        .insert_state(GameState::LoadingScreen)
        .insert_state(LevelState::HoleTutorial)
        .insert_state(MapSetState::Tutorial)

        // --- Resource Initialization --- //
        .insert_resource(BonkHandler::new())
        .insert_resource(CameraOrbitEntityStateHandler::new())
        .insert_resource(CameraCoordTracker::new())
        .insert_resource(Fonts::new())
        .insert_resource(GameStateHandler::new())
        .insert_resource(GLBStorageID::new())
        .insert_resource(OpIndex::new())

        // --- Startup Systems Initialization --- //
        .add_systems(Startup, setup_ground)
        .add_systems(Startup, setup_light)
        .add_systems(Startup, setup_ui)
        .add_systems(Startup, setup_3d_camera)
        .add_systems(Startup, performance_physics_setup)

        // --- Update Systems Initialization --- //
        // states //
        .add_systems(Update, game_state_update.run_if(input_just_released(KeyCode::ArrowLeft)))
        .add_systems(Update, level_state_update.run_if(input_just_released(KeyCode::ArrowUp)))
        .add_systems(Update, map_set_state_update.run_if(input_just_released(KeyCode::ArrowRight)))

        // User Interface //
        .add_systems(Update, fire_ray.run_if(input_pressed(MouseButton::Left)))
        .add_systems(Update, release_ray.run_if(input_just_released(MouseButton::Left)))
        .add_systems(Update, draw_cursor)
        .add_systems(Update, bonk_gizmo.run_if(in_state(ArrowState::DrawingArrow)))
        // .add_systems(Update, level_state_logic) // uncomment for level based logic every frame
        
        // Camera //
        .add_systems(Update, camera_orbit_entity_state_update.run_if(input_just_released(KeyCode::KeyC)))
        .add_systems(Update, camera_orbit_entity_state_logic)
        .add_systems(Update, pan_orbit_camera)

        // Physics //
        .add_systems(Update, add_physics_query_and_update_scene.run_if(input_just_released(MouseButton::Right)))
        .add_systems(Update, collision_events_listener)
        .add_systems(Update, bonk_step_start.run_if(input_just_pressed(MouseButton::Middle)))
        .add_systems(Update, bonk_step_mid.run_if(input_pressed(MouseButton::Middle)))
        .add_systems(Update, bonk_step_end.run_if(input_just_released(MouseButton::Middle)))

        // --- OnEnter State Reaction Initialization --- //
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

        // --- OnExit State Reaction Initialization --- //
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

