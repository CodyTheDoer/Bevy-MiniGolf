// --- Internal Bevy Plugins --- //
use bevy::{prelude::*,
    input::common_conditions::*,
    time::common_conditions::on_timer, 
    utils::Duration,
    window::{PresentMode, WindowTheme},
};

// --- External Plugins --- //
// use bevy_editor_pls::prelude::*;
use bevy_matchbox::prelude::*;
use bevy_rapier3d::prelude::*;

// --- States --- //
use minigolf::{ 
};

// --- Resources --- //
use minigolf::{
};

// --- User Interface Import --- //
use minigolf::user_interface::camera_world::{
    setup_3d_camera,
    pan_orbit_camera, 
    camera_orbit_entity_state_logic,
    camera_orbit_entity_state_update,
};
use minigolf::user_interface::user_interface::{
    setup_ui,
    update_ui,
};

// --- User Interface - Game State Handler Import --- //
use minigolf::user_interface::game_state_handler::{
};

// --- User Interface - Turn Handler Import --- //
use minigolf::user_interface::turn_state_handler::{
};

// --- User Interface - Menu Handler Import --- //
use minigolf::user_interface::menu_state_handler::{
};

// --- Level Handler Import --- //
use minigolf::level_handler::level_handler::{
};

// --- Physics Handler Import --- //
use minigolf::level_handler::physics_handler::{
};

fn main() {
    let mut app = App::new();
        app.add_plugins((
            DefaultPlugins.set(
                WindowPlugin {
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
                        prevent_default_event_handling: false,
                        window_theme: Some(WindowTheme::Dark),
                        visible: true,
                        ..default()
                    }),
                    ..default()
                }
            ),
        ))

        // --- Additional Plugins --- //
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugins(RapierDebugRenderPlugin::default())
        // .add_plugins(EditorPlugin::default())

        // --- State Initialization --- //
        // .insert_state(___________::___________)

        // --- Resource Initialization --- //
        // .insert_resource(___________::new())

        // --- Startup Systems Initialization --- //
        .add_systems(Startup, setup_3d_camera)
        // .add_systems(Startup, setup_ui)
        // .add_systems(Startup, ___________)

        // User Interface //
        // .add_systems(Update, ___________)

        // Camera //
        .add_systems(Update, camera_orbit_entity_state_logic)
        .add_systems(Update, pan_orbit_camera)
        .add_systems(Update, camera_orbit_entity_state_update.run_if(input_just_released(KeyCode::KeyC)))
        // .add_systems(Update, update_ui);

        // Physics //
        // .add_systems(Update, ___________)

        // --- OnEnter State Reaction Level Initialization --- //

        // --- OnExit State Reaction Level Purge --- //
    app.run();
}

/*
GameState                   MenuState                   PlayerCompletionState               LevelState
    #[default]                  #[default]                  #[default]                          #[default]
    LoadingScreen,              NoSelection,                NotInGame,                          MainMenu,
    Menus,                      Online,                     HoleIncomplete,                     Hole1,
    GameInitLocal,              Local,                      HoleCompleted,                      Hole2,
    GameInitOnline,             Tutorial,                                                       Hole3,
    InGame,                     LeaderBoard,                                                    Hole4,
    InGamePaused,               Preferences,                                                    Hole5,
    PostGameReview,             Player,                                                         Hole6,
    LeaderBoard,                                                                                Hole7,
    Preferences,                                                                                Hole8,
                                                                                                Hole9,
TurnState                   MapSetState                 PlayThroughStyleState                   Hole10,
    #[default]                  #[default]                  #[default]                          Hole11,
    Idle,                       Tutorial,                   Proximity,                          Hole12,
    Turn,                       WholeCorse,                 SetOrder,                           Hole13,
    TurnReset,                  FrontNine,                                                      Hole14,
    NextTurn,                   BackNine,                                                       Hole15,
    HoleComplete,               SelectAHole,                                                    Hole16,
    GameComplete,                                                                               Hole17,
                                                                                                Hole18,
                                                                                                HoleTutorial
                                                                                                MenuLeaderBoard
LeaderBoardState            ConnectionState             Party {                                 MenuLocal
    #[default]                  #[default]                  players: Arc<[Player]>,             MenuOnline
    Mixed,                      Local,                      active_player: Arc<i32>,            MenuPreferences
    Online,                     Online,                     active_level: Arc<i32>,
    Local,                                              }
    PostGame,
    InGame,
    InGameOnline, 

CameraOrbitEntityState      Player {
    #[default]                  pub player_id: String,
    MainMenu,                   pub hole_completion_state: PlayerCompletionState,
    Ball,                       pub ball_material: Color, // For now custom material/shaders planned
    Cup,                        pub ball_location: Vec3,
    FreePan,                    pub puts_count_total: u32,
    LeaderBoard,                pub puts_hole_1: u32,
    GameInit.                   pub puts_hole_2: u32,
    MenuLocal,                  pub puts_hole_3: u32,
    MenuOnline,                 pub puts_hole_4: u32,
    MenuPreferences,            pub puts_hole_5: u32,
    MenuPlayer,                 pub puts_hole_6: u32,
                                pub puts_hole_7: u32,
                                pub puts_hole_8: u32,
                                pub puts_hole_9: u32,
                                pub puts_hole_10: u32,
                                pub puts_hole_11: u32,
                                pub puts_hole_12: u32,
                                pub puts_hole_13: u32,
                                pub puts_hole_14: u32,
                                pub puts_hole_15: u32,
                                pub puts_hole_16: u32,
                                pub puts_hole_17: u32,
                                pub puts_hole_18: u32,
*/