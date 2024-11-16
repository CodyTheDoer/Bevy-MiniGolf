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
    LeaderBoardState,
    LevelState,
    MapSetState,
    MenuState,
    PartyState,
    PlayerCompletionState,
    PlayThroughStyleState,
    TurnState,
};

// --- Resources --- //
use minigolf::{
    BonkHandler,
    CameraCoordTracker,
    CameraOrbitEntityStateHandler,
    Fonts, 
    GameStateHandler, 
    GLBStorageID, 
    PanOrbitState,
    PanOrbitSettings,
};

// --- User Interface Import --- //
use minigolf::user_interface::camera_world::{
    setup_3d_camera,
    pan_orbit_camera, 
    camera_orbit_entity_state_logic,
    camera_orbit_entity_state_update,
};
use minigolf::user_interface::user_interface::{
    bonk_gizmo,
    game_state_update, 
    ray_fire, 
    ray_release, 
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
    purge_glb,
    purge_rigid_bodies,
};

// --- Physics Handler Import --- //
use minigolf::level_handler::physics_handler::{
    add_physics_query_and_update_scene,
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
        .insert_state(CameraOrbitEntityState::MainMenu)
        .insert_state(GameState::LoadingScreen)
        .insert_state(LeaderBoardState::Mixed)
        .insert_state(LevelState::MainMenu)
        .insert_state(MapSetState::Tutorial)
        .insert_state(MenuState::NoSelection)
        .insert_state(PartyState::Local)
        .insert_state(PlayerCompletionState::NotInGame)
        .insert_state(PlayThroughStyleState::Proximity)
        .insert_state(TurnState::Idle)

        // --- Resource Initialization --- //
        .insert_resource(BonkHandler::new())
        .insert_resource(CameraOrbitEntityStateHandler::new())
        .insert_resource(CameraCoordTracker::new())
        .insert_resource(Fonts::new())
        .insert_resource(GameStateHandler::new())
        .insert_resource(GLBStorageID::new())

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
        .add_systems(Update, ray_fire.run_if(input_pressed(MouseButton::Left)))
        .add_systems(Update, ray_release.run_if(input_just_released(MouseButton::Left)))
        .add_systems(Update, draw_cursor)
        .add_systems(Update, bonk_gizmo.run_if(in_state(ArrowState::DrawingArrow)))
        // .add_systems(Update, level_state_logic) // uncomment for level based logic every frame
        
        // Camera //
        .add_systems(Update, camera_orbit_entity_state_logic)
        .add_systems(Update, pan_orbit_camera)
        .add_systems(Update, camera_orbit_entity_state_update.run_if(input_just_released(KeyCode::KeyC)))

        // Physics //
        // .add_systems(Update, add_physics_query_and_update_scene.run_if(input_just_released(MouseButton::Right)))
        .add_systems(Update, collision_events_listener)
        .add_systems(Update, bonk_step_start.run_if(input_just_pressed(MouseButton::Left)))
        .add_systems(Update, bonk_step_mid.run_if(input_pressed(MouseButton::Left)))
        .add_systems(Update, bonk_step_end.run_if(input_just_released(MouseButton::Left)))

        // --- OnEnter State Reaction Initialization --- //        
        .add_systems(OnEnter(LevelState::MainMenu), init_hole_n)
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
        .add_systems(OnEnter(LevelState::HoleTutorial), init_hole_n)

        // --- OnExit State Reaction Initialization --- //
        .add_systems(OnExit(LevelState::MainMenu), purge_glb)
        .add_systems(OnExit(LevelState::Hole1), purge_glb)
        .add_systems(OnExit(LevelState::Hole2), purge_glb)
        .add_systems(OnExit(LevelState::Hole3), purge_glb)
        .add_systems(OnExit(LevelState::Hole4), purge_glb)
        .add_systems(OnExit(LevelState::Hole5), purge_glb)
        .add_systems(OnExit(LevelState::Hole6), purge_glb)
        .add_systems(OnExit(LevelState::Hole7), purge_glb)
        .add_systems(OnExit(LevelState::Hole8), purge_glb)
        .add_systems(OnExit(LevelState::Hole9), purge_glb)
        .add_systems(OnExit(LevelState::Hole10), purge_glb)
        .add_systems(OnExit(LevelState::Hole11), purge_glb)
        .add_systems(OnExit(LevelState::Hole12), purge_glb)
        .add_systems(OnExit(LevelState::Hole13), purge_glb)
        .add_systems(OnExit(LevelState::Hole14), purge_glb)
        .add_systems(OnExit(LevelState::Hole15), purge_glb)
        .add_systems(OnExit(LevelState::Hole16), purge_glb)
        .add_systems(OnExit(LevelState::Hole17), purge_glb)
        .add_systems(OnExit(LevelState::Hole18), purge_glb)
        .add_systems(OnExit(LevelState::HoleTutorial), purge_glb)
        
        .add_systems(OnExit(LevelState::Hole1), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::Hole2), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::Hole3), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::Hole4), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::Hole5), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::Hole6), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::Hole7), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::Hole8), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::Hole9), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::Hole10), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::Hole11), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::Hole12), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::Hole13), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::Hole14), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::Hole15), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::Hole16), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::Hole17), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::Hole18), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::HoleTutorial), purge_rigid_bodies)

        // .add_systems(OnEnter(MenuState::LeaderBoard), _______)
        // .add_systems(OnEnter(MenuState::Local), _______)
        // .add_systems(OnEnter(MenuState::Online), _______)
        // .add_systems(OnEnter(MenuState::Preferences), _______)
        .add_systems(OnEnter(MenuState::Tutorial), menu_state_response_tutorial)
        .add_systems(Update, add_physics_query_and_update_scene.run_if(asset_event_listener));
        // .add_systems(Update, );

        app.run();
}

// .insert_state(ArrowState::Idle)
// .insert_state(CameraOrbitEntityState::MainMenu)
// .insert_state(GameState::LoadingScreen)
// .insert_state(LeaderBoardState::Mixed)
// .insert_state(LevelState::MainMenu)
// .insert_state(MapSetState::Tutorial)
// .insert_state(MenuState::NoSelection)
// .insert_state(PartyState::Local)
// .insert_state(PlayerCompletionState::NotInGame)
// .insert_state(PlayThroughStyleState::Proximity)
// .insert_state(TurnState::Idle)

fn asset_event_listener(
    mut ev_asset: EventReader<AssetEvent<Mesh>>,
    // mut assets: ResMut<Assets<Mesh>>,
) -> bool {
    let mut event_occurred = false;
    for event in ev_asset.read() {
        event_occurred = true;
    };
    event_occurred
}

fn menu_state_response_tutorial(
    mut gsh: ResMut<GameStateHandler>,
    mut next_camera_state: ResMut<NextState<CameraOrbitEntityState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_map_set_state: ResMut<NextState<MapSetState>>,
    mut next_player_completion_state: ResMut<NextState<PlayerCompletionState>>,
    mut next_turn_state: ResMut<NextState<TurnState>>,
    mut camera_query: Query<&mut PanOrbitState>,
) {
    gsh.set_current_level(19);
    next_camera_state.set(CameraOrbitEntityState::Ball);
    next_game_state.set(GameState::InGame);
    next_map_set_state.set(MapSetState::Tutorial);
    next_player_completion_state.set(PlayerCompletionState::HoleIncomplete);
    next_turn_state.set(TurnState::Player1);
    for mut state in camera_query.iter_mut() {
        info!("{:?}", state);
        state.radius = 2.0;
        state.pitch = -8.0f32.to_radians();
        state.yaw = 22.0f32.to_radians();
    }
}

fn menu_state_response_leader_board() {}

fn menu_state_response_local() {}

fn menu_state_response_online() {}

fn menu_state_response_preferences() {}

// fn map_set_state_response_tutorial(
//     mut next_level_state: ResMut<NextState<LevelState>>,
// ) {
//     next_level_state.set(LevelState::HoleTutorial);
// }
fn map_set_state_response_whole_course() {}
fn map_set_state_response_front_nine() {}
fn map_set_state_response_back_nine() {}
fn map_set_state_response_select_a_hole() {}