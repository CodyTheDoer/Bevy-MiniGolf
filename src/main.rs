// --- Internal Bevy Plugins --- //
use bevy::{prelude::*,
    window::{PresentMode, WindowTheme},
    time::common_conditions::on_timer, 
    utils::Duration,
    // input::common_conditions::*,
};

// --- External Plugins --- //
use bevy_rapier3d::prelude::*;
use bevy_matchbox::prelude::*;
// use bevy_editor_pls::prelude::*;

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
    StateRunTrigger,
    StateTurn,
    StateUpdateRef,
};

// --- Resources --- //
use minigolf::{
    CameraHandler,
    Fonts,
    GameHandler,
    LeaderBoard,
    Party,
    Player,
    PlayerLocal,
    PlayerAi,
    PlayerRemote,
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
};

// --- Leader Board Handler Import --- //
use minigolf::player_handler::leader_board_handler::{
    leader_board_log_game,
    leader_board_review_last_game,
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
        .insert_resource(RunTrigger::new())
        .insert_resource(LeaderBoard::new()) 

        // --- Event Initialization --- //
        .add_event::<OnlineStateChange>()    

        // --- Startup Systems Initialization --- //
        .add_systems(Startup, setup_3d_camera)
        .add_systems(Startup, setup_ui)
        
        // Network - Startup //
        .add_systems(Startup, start_socket)

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
        
        .add_systems(Update, party_handler_active_player_set_ball_location.run_if(|run_trigger: Res<RunTrigger>|run_trigger.party_handler_active_player_set_ball_location()))
        .add_systems(Update, party_handler_active_player_add_bonk.run_if(|run_trigger: Res<RunTrigger>|run_trigger.party_handler_active_player_add_bonk()))
        .add_systems(Update, party_handler_cycle_active_player.run_if(|run_trigger: Res<RunTrigger>|run_trigger.party_handler_cycle_active_player()))
        .add_systems(Update, party_handler_active_player_set_hole_completion_state_true.run_if(|run_trigger: Res<RunTrigger>|run_trigger.party_handler_active_player_set_hole_completion_state_true()))
        
        .add_systems(Update, temp_interface);

    app.run();
}

fn start_socket(mut commands: Commands) {
    let socket = MatchboxSocket::new_reliable("ws://localhost:3536/minigolf");
    commands.insert_resource(socket);
}

fn auth_server_handshake(
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    party: Res<Party>,
) {
    let peers: Vec<_> = socket.connected_peers().collect();
        
    // grab the reference to the main player
    let player = party.get_active_player_clone();
        
    // grab the player id
    let player_id = player.lock().unwrap().get_player_id();

    for peer in peers {
        let message = format!{
            "{}",
            player_id,
        };
        info!("Sending message: {message:?} to {peer}");
        socket.send(message.as_bytes().into(), peer);
    }
}

#[derive(Debug, Event)]
pub struct OnlineStateChange;

fn server_parse_message(
    message: &str,
    game_handler: &mut ResMut<GameHandler>,
    online_event_handler: &mut EventWriter<OnlineStateChange>,
) {
    let split: &Vec<&str> = &message.split("::").collect();
    if split.len() != 2 {
        info!("Invalid message format");
        return;
    }

    if game_handler.is_not_connected() { // handle a new connection.
        let parsed_state = match split[0] {
            "StateGameConnection" => match split[1] {
                "Online" => Some(StateUpdateRef::StateGameConnection(StateGameConnection::Online)),
                _ => None,
            }
            _ => None,
        };
        
        if let Some(state) = parsed_state {
            info!("Updated state valid: {:?}", state);
            game_handler.auth_server_handshake_received(Some(state));
            online_event_handler.send(OnlineStateChange);
        } else {
            info!("Updated state invalid");
        }
    } else {
        let parsed_state = match split[0] {
            "StateGame" => match split[1] {
                "InGame" => Some(StateUpdateRef::StateGame(StateGame::InGame)),
                "NotInGame" => Some(StateUpdateRef::StateGame(StateGame::NotInGame)),
                _ => None,
            },
            "StateLevel" => match split[1] {
                "MainMenu" => Some(StateUpdateRef::StateLevel(StateLevel::MainMenu)),
                "Hole1" => Some(StateUpdateRef::StateLevel(StateLevel::Hole1)),
                "Hole2" => Some(StateUpdateRef::StateLevel(StateLevel::Hole2)),
                "Hole3" => Some(StateUpdateRef::StateLevel(StateLevel::Hole3)),
                "Hole4" => Some(StateUpdateRef::StateLevel(StateLevel::Hole4)),
                "Hole5" => Some(StateUpdateRef::StateLevel(StateLevel::Hole5)),
                "Hole6" => Some(StateUpdateRef::StateLevel(StateLevel::Hole6)),
                "Hole7" => Some(StateUpdateRef::StateLevel(StateLevel::Hole7)),
                "Hole8" => Some(StateUpdateRef::StateLevel(StateLevel::Hole8)),
                "Hole9" => Some(StateUpdateRef::StateLevel(StateLevel::Hole9)),
                "Hole10" => Some(StateUpdateRef::StateLevel(StateLevel::Hole10)),
                "Hole11" => Some(StateUpdateRef::StateLevel(StateLevel::Hole11)),
                "Hole12" => Some(StateUpdateRef::StateLevel(StateLevel::Hole12)),
                "Hole13" => Some(StateUpdateRef::StateLevel(StateLevel::Hole13)),
                "Hole14" => Some(StateUpdateRef::StateLevel(StateLevel::Hole14)),
                "Hole15" => Some(StateUpdateRef::StateLevel(StateLevel::Hole15)),
                "Hole16" => Some(StateUpdateRef::StateLevel(StateLevel::Hole16)),
                "Hole17" => Some(StateUpdateRef::StateLevel(StateLevel::Hole17)),
                "Hole18" => Some(StateUpdateRef::StateLevel(StateLevel::Hole18)),
                "HoleTutorial" => Some(StateUpdateRef::StateLevel(StateLevel::HoleTutorial)),
                _ => None,
            },
            "StateCameraOrbitEntity" => match split[1] {
                "Menu" => Some(StateUpdateRef::StateCameraOrbitEntity(StateCameraOrbitEntity::Menu)),
                "Ball" => Some(StateUpdateRef::StateCameraOrbitEntity(StateCameraOrbitEntity::Ball)),
                "Cup" => Some(StateUpdateRef::StateCameraOrbitEntity(StateCameraOrbitEntity::Cup)),
                "FreePan" => Some(StateUpdateRef::StateCameraOrbitEntity(StateCameraOrbitEntity::FreePan)),
                "LeaderBoard" => Some(StateUpdateRef::StateCameraOrbitEntity(StateCameraOrbitEntity::LeaderBoard)),
                _ => None,
            },
            "StateMapSet" => match split[1] {
                "Tutorial" => Some(StateUpdateRef::StateMapSet(StateMapSet::Tutorial)),
                "WholeCorse" => Some(StateUpdateRef::StateMapSet(StateMapSet::WholeCorse)),
                "FrontNine" => Some(StateUpdateRef::StateMapSet(StateMapSet::FrontNine)),
                "BackNine" => Some(StateUpdateRef::StateMapSet(StateMapSet::BackNine)),
                "SelectAHole" => Some(StateUpdateRef::StateMapSet(StateMapSet::SelectAHole)),
                _ => None,
            },
            "StateGamePlayStyle" => match split[1] {
                "SetOrder" => Some(StateUpdateRef::StateGamePlayStyle(StateGamePlayStyle::SetOrder)),
                "Proximity" => Some(StateUpdateRef::StateGamePlayStyle(StateGamePlayStyle::Proximity)),
                _ => None,
            },
            "StateTurn" => match split[1] {
                "NotInGame" => Some(StateUpdateRef::StateTurn(StateTurn::NotInGame)),
                "Active" => Some(StateUpdateRef::StateTurn(StateTurn::Active)),
                "NextTurn" => Some(StateUpdateRef::StateTurn(StateTurn::NextTurn)),
                _ => None,
            },
            "StateRunTrigger" => match split[1] {
                "Idle" => Some(StateUpdateRef::StateRunTrigger(StateRunTrigger::Idle)),
                "PartyHandlerActivePlayerAddBonk" => Some(StateUpdateRef::StateRunTrigger(StateRunTrigger::PartyHandlerActivePlayerAddBonk)),
                "PartyHandlerActivePlayerSetBallLocation" => Some(StateUpdateRef::StateRunTrigger(StateRunTrigger::PartyHandlerActivePlayerSetBallLocation)),
                "PartyHandlerActivePlayerSetHoleCompletionStateTrue" => Some(StateUpdateRef::StateRunTrigger(StateRunTrigger::PartyHandlerActivePlayerSetHoleCompletionStateTrue)),
                "PartyHandlerCycleActivePlayer" => Some(StateUpdateRef::StateRunTrigger(StateRunTrigger::PartyHandlerCycleActivePlayer)),

                "GameHandlerCycleStateCamera" => Some(StateUpdateRef::StateRunTrigger(StateRunTrigger::GameHandlerCycleStateCamera)),
                "GameHandlerCycleStateMapSet" => Some(StateUpdateRef::StateRunTrigger(StateRunTrigger::GameHandlerCycleStateMapSet)),
                "GameHandlerCycleCurrentLevel" => Some(StateUpdateRef::StateRunTrigger(StateRunTrigger::GameHandlerCycleCurrentLevel)),
                "GameHandlerGetActiveBallLocation" => Some(StateUpdateRef::StateRunTrigger(StateRunTrigger::GameHandlerGetActiveBallLocation)),
                "GameHandlerResetActiveBallLocation" => Some(StateUpdateRef::StateRunTrigger(StateRunTrigger::GameHandlerResetActiveBallLocation)),
                "GameHandlerSetActiveBallLocation" => Some(StateUpdateRef::StateRunTrigger(StateRunTrigger::GameHandlerSetActiveBallLocation)),
                "GameHandlerStateTurnNextPlayerTurn" => Some(StateUpdateRef::StateRunTrigger(StateRunTrigger::GameHandlerStateTurnNextPlayerTurn)),
                "GameHandlerStartGameLocal" => Some(StateUpdateRef::StateRunTrigger(StateRunTrigger::GameHandlerStartGameLocal)),
                "GameHandlerToggleStateGame" => Some(StateUpdateRef::StateRunTrigger(StateRunTrigger::GameHandlerToggleStateGame)),

                "LeaderBoardLogGame" => Some(StateUpdateRef::StateRunTrigger(StateRunTrigger::LeaderBoardLogGame)),
                "LeaderBoardReviewLastGame" => Some(StateUpdateRef::StateRunTrigger(StateRunTrigger::LeaderBoardReviewLastGame)),
                _ => None,
            },
            _ => None,
        };
        
        if let Some(state) = parsed_state {
            info!("Updated state valid: {:?}", state);
            game_handler.auth_server_handshake_received(Some(state));
            online_event_handler.send(OnlineStateChange);
        } else {
            info!("Updated state invalid");
        }
    }
}

fn remote_state_change_monitor(
    mut online_event_listener: EventReader<OnlineStateChange>,
    mut game_handler: ResMut<GameHandler>,
    mut next_state_connection: ResMut<NextState<StateGameConnection>>,
    mut next_state_game: ResMut<NextState<StateGame>>,
) {
    for ev in online_event_listener.read() {
        let pushed_state =  game_handler.get_pushed_state();
        match pushed_state {
            StateUpdateRef::StateGameConnection(state_game_connection) => {
                next_state_connection.set(state_game_connection); 
                game_handler.set_connected_true();
            },
            StateUpdateRef::StateGame(state_game) => {
                next_state_game.set(state_game); 
            },
            StateUpdateRef::StateLevel(_) => {
                info!("StateLevel received");
            },
            StateUpdateRef::StateMapSet(_) => {
                info!("StateMapSet received");
            },
            StateUpdateRef::StateTurn(_) => {
                info!("StateTurn received");
            },
            StateUpdateRef::StateGamePlayStyle(_) => {
                info!("StateGamePlayStyle received");
            },
            StateUpdateRef::StateCameraOrbitEntity(_) => {
                info!("StateCameraOrbitEntity received");
            },
            StateUpdateRef::StateRunTrigger(_) => {
                info!("StateRunTrigger received");
            },
        }
    }
}


fn receive_messages(
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    mut game_handler: ResMut<GameHandler>,
    mut online_event_handler: EventWriter<OnlineStateChange>,
) {
    for (peer, state) in socket.update_peers() {
        info!("{peer}: {state:?}");
    }

    for (_id, message) in socket.receive() {
        match std::str::from_utf8(&message) {
            Ok(message) => {
                info!("Received message: {message:?}");
                // let split: &Vec<&str> = &message.split("::").collect();
                // for i in 0..split.len() {
                //     info!("split{}: {:?}", i, split[i]);
                // }
                server_parse_message(message, &mut game_handler, &mut online_event_handler);
            },
            Err(e) => error!("Failed to convert message to string: {e}"),
        }
    }
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
                let new_player_local = PlayerLocal::new();
                let new_player = Arc::new(Mutex::new(new_player_local));
                party.add_player(new_player);
            },
        };
    };
    if keys.just_released(KeyCode::Numpad8) {
        info!("just_released: Numpad8");  
        match state_game.get() {
            StateGame::InGame => {},
            StateGame::NotInGame => {
                let new_player_remote = PlayerRemote::new();
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
                let new_player_ai = PlayerAi::new();
                let new_player = Arc::new(Mutex::new(new_player_ai));
                party.add_player(new_player);
            },
        };
    };
}
