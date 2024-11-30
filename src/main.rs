// --- Internal Bevy Plugins --- //
use bevy::{prelude::*,
    window::{PresentMode, WindowTheme},
    time::common_conditions::on_timer, 
    utils::Duration,
    input::common_conditions::*,
    // tasks::IoTaskPool,
};

// use dotenv::dotenv;
// use std::env;
// use sqlx::mysql::MySqlPoolOptions;
// use sqlx::MySqlPool;
use tokio::runtime::Runtime;
// use uuid::Uuid;

// --- External Plugins --- //
use bevy_tokio_tasks::{TokioTasksPlugin, TokioTasksRuntime};
use bevy_rapier3d::prelude::*;
use bevy_matchbox::prelude::*;
// use bevy_editor_pls::prelude::*;

// use std::sync::{
//     Arc,
//     Mutex,
// };

// --- States --- //
use minigolf::{ 
    StateArrow, 
    StateCameraOrbitEntity, 
    StateGame, 
    StateGameConnection, 
    StateGamePlayStyle, 
    StateLevel, 
    StateMapSet, 
    StateMenu, 
    StateRunTrigger, 
    StateTurn, 
    StateUpdateRef
};

// --- Resources --- //
use minigolf::{
    CameraHandler,
    DatabasePool,
    Fonts,
    GameHandler,
    LeaderBoard,
    OnlineStateChange,
    Party,
    // Player,
    // PlayerLocal,
    // PlayerAi,
    // PlayerRemote,
    RunTrigger,
};

// --- User Camera World Import --- //
use minigolf::user_interface::camera_world::{
    setup_3d_camera,
    pan_orbit_camera, 
    state_camera_orbit_entity_logic,
};

// --- User Interface Import --- //
use minigolf::user_interface::user_interface::{
    setup_ui,
    update_ui,
};

// // --- Game Handler Import --- //
use minigolf::user_interface::game_handler::{
    game_handler_cycle_current_level,
    game_handler_cycle_state_camera,
    game_handler_cycle_state_map_set,
    game_handler_get_active_ball_location,
    game_handler_reset_active_ball_location,
    game_handler_set_active_ball_location,
    game_handler_state_turn_next_player_turn,
    game_handler_start_game_local,
    game_handler_toggle_state_game,

};

// // --- Party Handler Import --- //
use minigolf::player_handler::party_handler::{
    party_handler_active_player_add_bonk,
    party_handler_active_player_set_ball_location,
    party_handler_active_player_set_hole_completion_state_true,
    party_handler_cycle_active_player,
    party_handler_new_player_ai,
    party_handler_new_player_local,
    party_handler_new_player_remote,
    party_handler_remove_ai,
    party_handler_remove_last_player,
};

// --- Leader Board Handler Import --- //
use minigolf::player_handler::leader_board_handler::{
    leader_board_log_game,
    leader_board_review_last_game,
};

// // --- Database Handler Import --- //
use minigolf::database_handler::{
    database_startup_system,
    query_boot_system,
    first_time_boot_system,
    establish_connection,
};

// // --- Network Handler Import --- //
use minigolf::network_handler::{
    auth_server_handshake,
    network_get_client_state_game,
    server_parse_message,
    start_socket,
    receive_messages,
    remote_state_change_monitor,
};

// // --- User Interface - Menu Handler Import --- //
// use minigolf::user_interface::menu_handler::{
// };

// // --- Level Handler Import --- //
// use minigolf::level_handler::level_handler::{
// };

// // --- Physics Handler Import --- //
// use minigolf::level_handler::physics_handler::{
// };

fn main() {
    let runtime = Runtime::new().expect("Failed to create Tokio runtime");    

    // Use the runtime to block on the async function and get the pool
    let pool = runtime.block_on(establish_connection())
        .expect("Failed to create database connection pool");

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
        .add_plugins(bevy_tokio_tasks::TokioTasksPlugin::default())
        // .add_plugins(RapierDebugRenderPlugin::default())
        // .add_plugins(EditorPlugin::default())
    
        // --- State Initialization --- //
        .insert_state(StateArrow::Idle)
        .insert_state(StateCameraOrbitEntity::Menu)
        .insert_state(StateGame::NotInGame)
        .insert_state(StateGameConnection::Local)
        .insert_state(StateGamePlayStyle::SetOrder)
        .insert_state(StateLevel::MainMenu)
        .insert_state(StateMapSet::Tutorial)
        .insert_state(StateMenu::MenuMainMenu)
        .insert_state(StateRunTrigger::Idle)
        .insert_state(StateTurn::NotInGame)

        // --- Resource Initialization --- //
        .insert_resource(DatabasePool(pool))
        .insert_resource(CameraHandler::new())
        .insert_resource(GameHandler::new())
        .insert_resource(Fonts::new())
        .insert_resource(LeaderBoard::new()) 
        .insert_resource(Party::new())
        .insert_resource(RunTrigger::new())
        
        // --- Event Initialization --- //
        .add_event::<OnlineStateChange>()    

        // --- Startup Systems Initialization --- //
        .add_systems(Startup, setup_3d_camera)
        .add_systems(Startup, setup_ui)
        .add_systems(Startup, database_startup_system)
        
        // Network - Startup //
        .add_systems(Startup, start_socket)

        // Database - Interface //
        .add_systems(Update, first_time_boot_system.run_if(input_just_released(KeyCode::ShiftLeft)))

        // Network - Update //
        .add_systems(Update, receive_messages)
        .add_systems(Update, remote_state_change_monitor)
        .add_systems(Update, auth_server_handshake
            .run_if(|game_handler: Res<GameHandler>|game_handler.is_not_connected())
            .run_if(on_timer(Duration::from_secs(5))))

        // Camera //
        .add_systems(Update, state_camera_orbit_entity_logic)
        .add_systems(Update, pan_orbit_camera)
        .add_systems(Update, update_ui)

        // Run Trigger Systems //        
        .add_systems(Update, game_handler_cycle_state_camera.run_if(|run_trigger: Res<RunTrigger>|run_trigger.game_handler_cycle_state_camera()))
        .add_systems(Update, game_handler_cycle_current_level.run_if(|run_trigger: Res<RunTrigger>|run_trigger.game_handler_cycle_current_level()))
        .add_systems(Update, game_handler_cycle_state_map_set.run_if(|run_trigger: Res<RunTrigger>|run_trigger.game_handler_cycle_state_map_set()))
        .add_systems(Update, game_handler_get_active_ball_location.run_if(|run_trigger: Res<RunTrigger>|run_trigger.game_handler_get_active_ball_location()))
        .add_systems(Update, game_handler_reset_active_ball_location.run_if(|run_trigger: Res<RunTrigger>|run_trigger.game_handler_reset_active_ball_location()))
        .add_systems(Update, game_handler_set_active_ball_location.run_if(|run_trigger: Res<RunTrigger>|run_trigger.game_handler_set_active_ball_location()))
        .add_systems(Update, game_handler_start_game_local.run_if(|run_trigger: Res<RunTrigger>|run_trigger.game_handler_start_game_local()))
        .add_systems(Update, game_handler_state_turn_next_player_turn.run_if(|run_trigger: Res<RunTrigger>|run_trigger.game_handler_state_turn_next_player_turn()))
        .add_systems(Update, game_handler_toggle_state_game.run_if(|run_trigger: Res<RunTrigger>|run_trigger.game_handler_toggle_state_game()))

        .add_systems(Update, leader_board_log_game.run_if(|run_trigger: Res<RunTrigger>|run_trigger.leader_board_log_game()))
        .add_systems(Update, leader_board_review_last_game.run_if(|run_trigger: Res<RunTrigger>|run_trigger.leader_board_review_last_game()))
        
        .add_systems(Update, network_get_client_state_game.run_if(|run_trigger: Res<RunTrigger>|run_trigger.network_get_client_state_game()))
        
        .add_systems(Update, party_handler_active_player_set_ball_location.run_if(|run_trigger: Res<RunTrigger>|run_trigger.party_handler_active_player_set_ball_location()))
        .add_systems(Update, party_handler_active_player_add_bonk.run_if(|run_trigger: Res<RunTrigger>|run_trigger.party_handler_active_player_add_bonk()))
        .add_systems(Update, party_handler_active_player_set_hole_completion_state_true.run_if(|run_trigger: Res<RunTrigger>|run_trigger.party_handler_active_player_set_hole_completion_state_true()))
        
        .add_systems(Update, party_handler_cycle_active_player.run_if(|run_trigger: Res<RunTrigger>|run_trigger.party_handler_cycle_active_player()))
        
        .add_systems(Update, party_handler_new_player_ai.run_if(|run_trigger: Res<RunTrigger>|run_trigger.party_handler_new_player_ai()))
        .add_systems(Update, party_handler_new_player_local.run_if(|run_trigger: Res<RunTrigger>|run_trigger.party_handler_new_player_local()))
        .add_systems(Update, party_handler_new_player_remote.run_if(|run_trigger: Res<RunTrigger>|run_trigger.party_handler_new_player_remote()))
        
        .add_systems(Update, party_handler_remove_last_player.run_if(|run_trigger: Res<RunTrigger>|run_trigger.party_handler_remove_last_player()))
        .add_systems(Update, party_handler_remove_ai.run_if(|run_trigger: Res<RunTrigger>|run_trigger.party_handler_remove_ai()))

        .add_systems(Update, temp_interface);

    app.run();
}

//-----------------------------------------------------------------------------------//

fn temp_interface(
    mut run_trigger: ResMut<RunTrigger>,
    keys: Res<ButtonInput<KeyCode>>,
    state_game: Res<State<StateGame>>,
) {
    if keys.just_released(KeyCode::Space) {
        info!("just_released: Space");  
        run_trigger.set_target("game_handler_toggle_state_game", true);
    };
    if keys.just_released(KeyCode::KeyB) {
        info!("just_released: KeyB");  
        match state_game.get() {
            StateGame::NotInGame => {},
            StateGame::InGame => {
                run_trigger.set_target("party_handler_active_player_add_bonk", true);
            },
        };
    };
    if keys.just_released(KeyCode::KeyA) { // should trigger with new turn
        info!("just_released: KeyA");  
        match state_game.get() {
            StateGame::NotInGame => {},
            StateGame::InGame => {
                run_trigger.set_target("party_handler_active_player_set_hole_completion_state_true", true);
            },
        };
    };
    if keys.just_released(KeyCode::KeyC) {
        info!("just_released: KeyC");  
        match state_game.get() {
            StateGame::NotInGame => {},
            StateGame::InGame => {
                run_trigger.set_target("game_handler_cycle_state_camera", true);
            },
        };
    };
    if keys.just_released(KeyCode::KeyM) {
        info!("just_released: KeyM");  
        match state_game.get() {
            StateGame::InGame => {},
            StateGame::NotInGame => {
                run_trigger.set_target("game_handler_cycle_state_map_set", true);
            },
        };
    };
    if keys.just_released(KeyCode::KeyN) {
        info!("just_released: KeyN");  
        match state_game.get() {
            StateGame::NotInGame => {},
            StateGame::InGame => {
                run_trigger.set_target("game_handler_state_turn_next_player_turn", true);
            },
        };
    };
    if keys.just_released(KeyCode::KeyP) {
        info!("just_released: KeyP");  
        match state_game.get() {
            StateGame::NotInGame => {},
            StateGame::InGame => {
                run_trigger.set_target("party_handler_cycle_active_player", true);},
            };
        };
    if keys.just_released(KeyCode::KeyS) {
        info!("just_released: KeyS");  
        match state_game.get() {
            StateGame::InGame => {},
            StateGame::NotInGame => {
                run_trigger.set_target("game_handler_start_game_local", true);
            },
        };
    };
    if keys.just_released(KeyCode::Numpad1) {
        info!("just_released: Numpad1");  
        match state_game.get() {
            StateGame::InGame => {},
            StateGame::NotInGame => {
                run_trigger.set_target("party_handler_remove_last_player", true);
            },
        };
    };
    if keys.just_released(KeyCode::Numpad3) {
        info!("just_released: Numpad3");  
        match state_game.get() {
            StateGame::InGame => {},
            StateGame::NotInGame => {
                run_trigger.set_target("party_handler_remove_ai", true);
            },
        };
    };
    if keys.just_released(KeyCode::Numpad7) {
        info!("just_released: Numpad7");  
        match state_game.get() {
            StateGame::InGame => {},
            StateGame::NotInGame => {
                run_trigger.set_target("party_handler_new_player_local", true);
            },
        };
    };
    if keys.just_released(KeyCode::Numpad8) {
        info!("just_released: Numpad8");  
        match state_game.get() {
            StateGame::InGame => {},
            StateGame::NotInGame => {
                run_trigger.set_target("party_handler_new_player_remote", true);
            },
        };
    };
    if keys.just_released(KeyCode::Numpad9) {
        info!("just_released: Numpad9");   
        match state_game.get() {
            StateGame::InGame => {},
            StateGame::NotInGame => {
                run_trigger.set_target("party_handler_new_player_ai", true);
            },
        };
    };
}