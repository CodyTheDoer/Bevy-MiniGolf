use bevy::prelude::*;

// --- External Plugins --- //
use bevy_matchbox::prelude::*;

// --- States --- //
use crate::{ 
    StateCameraOrbitEntity, 
    StateGame, 
    StateGameConnection, 
    StateGamePlayStyle, 
    StateLevel, 
    StateMapSet, 
    StateRunTrigger, 
    StateTurn, 
    StateUpdateRef
};

// --- Resources --- //
use crate::{
    GameHandler,
    OnlineStateChange,
    Party,
    RunTrigger,
};

pub fn network_get_client_state_game(
    mut run_trigger: ResMut<RunTrigger>,
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    mut party: ResMut<Party>,
    state_game: Res<State<StateGame>>,
) {
    let peers: Vec<_> = socket.connected_peers().collect();
    let player_id = party.main_player_get_player_id();
    let state = state_game.get();
    for peer in peers {
        let message = format!{
            "{}::{:?}",
            player_id,
            state,
        };
        info!("Sending message: {message:?} to {peer}");
        socket.send(message.as_bytes().into(), peer);
    }

    
    run_trigger.set_target("network_get_client_state_game", false);
}

pub fn start_socket(mut commands: Commands) {
    let socket = MatchboxSocket::new_reliable("ws://localhost:3536/minigolf");
    commands.insert_resource(socket);
}

pub fn auth_server_handshake(
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

pub fn server_parse_message(
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
                "PartyHandlerActivePlayerAddBonk" => Some(StateUpdateRef::StateRunTrigger(StateRunTrigger::PartyHandlerActivePlayerAddBonk)),
                "PartyHandlerActivePlayerSetBallLocation" => Some(StateUpdateRef::StateRunTrigger(StateRunTrigger::PartyHandlerActivePlayerSetBallLocation)),
                "PartyHandlerActivePlayerSetHoleCompletionStateTrue" => Some(StateUpdateRef::StateRunTrigger(StateRunTrigger::PartyHandlerActivePlayerSetHoleCompletionStateTrue)),
                "PartyHandlerCycleActivePlayer" => Some(StateUpdateRef::StateRunTrigger(StateRunTrigger::PartyHandlerCycleActivePlayer)),

                "network_get_client_state_game" => Some(StateUpdateRef::StateRunTrigger(StateRunTrigger::NetworkGetClientStateGame)),

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

pub fn remote_state_change_monitor(
    mut online_event_listener: EventReader<OnlineStateChange>,
    mut game_handler: ResMut<GameHandler>,
    mut next_state_connection: ResMut<NextState<StateGameConnection>>,
    mut next_state_game: ResMut<NextState<StateGame>>,
    mut next_state_level: ResMut<NextState<StateLevel>>,
    mut next_state_map_set: ResMut<NextState<StateMapSet>>,
    mut next_state_turn: ResMut<NextState<StateTurn>>,
    mut next_state_game_play_style: ResMut<NextState<StateGamePlayStyle>>,
    mut next_state_camera_orbit_entity: ResMut<NextState<StateCameraOrbitEntity>>,
    mut next_state_run_trigger: ResMut<NextState<StateRunTrigger>>,
) {
    for ev in online_event_listener.read() {
        let pushed_state =  game_handler.get_pushed_state();
        match pushed_state {
            StateUpdateRef::StateGameConnection(state_game_connection) => {
                info!("StateGameConnection: {:?}", state_game_connection);
                next_state_connection.set(state_game_connection); 
                game_handler.set_connected_true();
            },
            StateUpdateRef::StateGame(state_game) => {
                info!("StateGame: {:?}", state_game);
                next_state_game.set(state_game); 
            },
            StateUpdateRef::StateLevel(state_level) => {
                info!("StateLevel: {:?}", state_level);
                next_state_level.set(state_level);
            },
            StateUpdateRef::StateMapSet(state_map_set) => {
                info!("StateMapSet: {:?}", state_map_set);
                next_state_map_set.set(state_map_set);
            },
            StateUpdateRef::StateTurn(state_turn) => {
                info!("StateTurn: {:?}", state_turn);
                next_state_turn.set(state_turn);
            },
            StateUpdateRef::StateGamePlayStyle(state_game_play_style) => {
                info!("StateGamePlayStyle: {:?}", state_game_play_style);
                next_state_game_play_style.set(state_game_play_style);
            },
            StateUpdateRef::StateCameraOrbitEntity(state_camera_orbit_entity) => {
                info!("StateCameraOrbitEntity: {:?}", state_camera_orbit_entity);
                next_state_camera_orbit_entity.set(state_camera_orbit_entity);
            },
            StateUpdateRef::StateRunTrigger(state_run_trigger) => {
                info!("StateRunTrigger: {:?}", state_run_trigger);
                next_state_run_trigger.set(state_run_trigger);
            },
        }
    }
}

pub fn receive_messages(
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
                server_parse_message(message, &mut game_handler, &mut online_event_handler);
            },
            Err(e) => error!("Failed to convert message to string: {e}"),
        }
    }
}