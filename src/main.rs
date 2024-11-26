// --- Internal Bevy Plugins --- //
use bevy::{prelude::*,
    input::common_conditions::*,
    // time::common_conditions::on_timer, 
    // utils::Duration,
    window::{PresentMode, WindowTheme},
};

// --- External Plugins --- //
// use bevy_editor_pls::prelude::*;
use bevy_matchbox::prelude::*;
use bevy_rapier3d::prelude::*;

use std::sync::{
    Arc,
    Mutex,
};

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
    StateTurn,
};

// --- Resources --- //
use minigolf::{
    CameraHandler,
    Fonts,
    GameHandler,
    GolfBallTag,
    Party,
    Player,
    PlayerLocal,
    PlayerAi,
    PlayerRemote,
    RunTrigger,
};

// --- User Interface Import --- //
use minigolf::user_interface::camera_world::{
    setup_3d_camera,
    pan_orbit_camera, 
    state_camera_orbit_entity_logic,
};
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
};

// // --- User Interface - Turn Handler Import --- //
// use minigolf::user_interface::turn_handler::{
// };

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
        .insert_state(StateArrow::Idle)
        .insert_state(StateCameraOrbitEntity::Menu)
        .insert_state(StateGame::NotInGame)
        .insert_state(StateGameConnection::Local)
        .insert_state(StateGamePlayStyle::SetOrder)
        .insert_state(StateLevel::MainMenu)
        .insert_state(StateMapSet::Tutorial)
        .insert_state(StateMenu::MenuMainMenu)
        .insert_state(StateTurn::NotInGame)

        // --- Resource Initialization --- //
        .insert_resource(CameraHandler::new())
        .insert_resource(GameHandler::new())
        .insert_resource(Fonts::new())
        .insert_resource(Party::new())

        // --- Startup Systems Initialization --- //
        .add_systems(Startup, setup_3d_camera)
        .add_systems(Startup, setup_ui)

        // Camera //
        .add_systems(Update, state_camera_orbit_entity_logic)
        .add_systems(Update, pan_orbit_camera)
        .add_systems(Update, update_ui)

        // Dev Systems //        
        .insert_resource(RunTrigger::new())
        .add_systems(Update, party_handler_active_player_set_ball_location.run_if(|run_trigger: Res<RunTrigger>|run_trigger.party_handler_active_player_set_ball_location()))
        .add_systems(Update, party_handler_active_player_add_bonk.run_if(|run_trigger: Res<RunTrigger>|run_trigger.party_handler_active_player_add_bonk()))
        .add_systems(Update, party_handler_cycle_active_player.run_if(|run_trigger: Res<RunTrigger>|run_trigger.party_handler_cycle_active_player()))
        .add_systems(Update, party_handler_active_player_set_hole_completion_state_true.run_if(|run_trigger: Res<RunTrigger>|run_trigger.party_handler_active_player_set_hole_completion_state_true()))
        .add_systems(Update, game_handler_cycle_state_camera.run_if(|run_trigger: Res<RunTrigger>|run_trigger.game_handler_cycle_state_camera()))
        .add_systems(Update, game_handler_cycle_current_level.run_if(|run_trigger: Res<RunTrigger>|run_trigger.game_handler_cycle_current_level()))
        .add_systems(Update, game_handler_cycle_state_map_set.run_if(|run_trigger: Res<RunTrigger>|run_trigger.game_handler_cycle_state_map_set()))
        .add_systems(Update, game_handler_get_active_ball_location.run_if(|run_trigger: Res<RunTrigger>|run_trigger.game_handler_get_active_ball_location()))
        .add_systems(Update, game_handler_reset_active_ball_location.run_if(|run_trigger: Res<RunTrigger>|run_trigger.game_handler_reset_active_ball_location()))
        .add_systems(Update, game_handler_set_active_ball_location.run_if(|run_trigger: Res<RunTrigger>|run_trigger.game_handler_set_active_ball_location()))
        .add_systems(Update, game_handler_start_game_local.run_if(|run_trigger: Res<RunTrigger>|run_trigger.game_handler_start_game_local()))
        .add_systems(Update, game_handler_state_turn_next_player_turn.run_if(|run_trigger: Res<RunTrigger>|run_trigger.game_handler_state_turn_next_player_turn()))
        .add_systems(Update, game_handler_toggle_state_game.run_if(|run_trigger: Res<RunTrigger>|run_trigger.game_handler_toggle_state_game()))
            
        .add_systems(Update, temp_interface);

    app.run();
}

fn temp_interface(
    mut run_trigger: ResMut<RunTrigger>,
    keys: Res<ButtonInput<KeyCode>>,
    party: Res<Party>,
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
                party.remove_last_player();
            },
        };
    };
    if keys.just_released(KeyCode::Numpad3) {
        info!("just_released: Numpad3");  
        match state_game.get() {
            StateGame::InGame => {},
            StateGame::NotInGame => {
                party.remove_ai();
            },
        };
    };
    if keys.just_released(KeyCode::Numpad7) {
        info!("just_released: Numpad7");  
        match state_game.get() {
            StateGame::InGame => {},
            StateGame::NotInGame => {
                let owned_party_size: i32 = party.get_party_size() as i32;
                let new_player_local = PlayerLocal {
                    player_id: String::from(format!("PlayerLocal{}@email.com", owned_party_size + 1)),
                    hole_completion_state: false,
                    ball_material: Color::srgb(1.0, 0.0, 1.0),
                    ball_location: Vec3::new(0.0, 0.0, 0.0),
                    bonks_level: 0,
                    bonks_game: 0,
                };

                let new_player = Arc::new(Mutex::new(new_player_local));
                party.add_player(new_player);
            },
        };
    };
    if keys.just_released(KeyCode::Numpad8) {
        info!("just_released: Numpad7");  
        match state_game.get() {
            StateGame::InGame => {},
            StateGame::NotInGame => {
                let owned_party_size: i32 = party.get_party_size() as i32;
                let new_player_remote = PlayerRemote {
                    player_id: String::from(format!("PlayerRemote{}@email.com", owned_party_size + 1)),
                    hole_completion_state: false,
                    ball_material: Color::srgb(1.0, 0.0, 1.0),
                    ball_location: Vec3::new(0.0, 0.0, 0.0),
                    bonks_level: 0,
                    bonks_game: 0,
                };

                let new_player = Arc::new(Mutex::new(new_player_remote));
                party.add_player(new_player);
            },
        };
    };
    if keys.just_released(KeyCode::Numpad9) {
        info!("just_released: Numpad9");   
        match state_game.get() {
            StateGame::InGame => {},
            StateGame::NotInGame => {
                let owned_party_size: i32 = party.get_party_size() as i32;
                let new_player_ai = PlayerAi::new();
                let new_player = Arc::new(Mutex::new(new_player_ai));
                party.add_player(new_player);
            },
        };
    };
}

/*
pub enum StateArrow {
    #[default]
    Idle,
    DrawingArrow,
}

pub enum StateCameraOrbitEntity {
    #[default]
    Menu,
    Ball,
    Cup,
    FreePan,
    LeaderBoard,
}

pub enum StateLevel {
    #[default]
    MainMenu,
    Hole1,
    Hole2,
    Hole3,
    Hole4,
    Hole5,
    Hole6,
    Hole7,
    Hole8,
    Hole9,
    Hole10,
    Hole11,
    Hole12,
    Hole13,
    Hole14,
    Hole15,
    Hole16,
    Hole17,
    Hole18,
    HoleTutorial,
    MenuLeaderBoard,
    MenuLocal,
    MenuOnline,
    MenuPreferences,
    MenuPlayer,
}

pub enum StateMapSet {
    #[default]
    Tutorial,
    WholeCorse,
    FrontNine,
    BackNine,
    SelectAHole,
}

pub enum StateMenu {
    #[default]
    MenuMainMenu,
    MenuLeaderBoard,
    MenuLocal,
    MenuOnline,
    MenuPreferences,
    MenuPlayer,
}

pub enum StateGame {
    #[default]
    NotInGame,
    InGame,
}

pub enum StateGameConnection {
    #[default]
    Local,
    Online,
}

pub enum StateGamePlayStyle {
    #[default]
    SetOrder,
    Proximity,
}

pub enum StateTurn {
    #[default]
    NotInGame,
    Active,
    NextTurn,
}
*/





