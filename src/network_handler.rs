use bevy::prelude::*;

// --- External Plugins --- //
use bevy_matchbox::prelude::*;
use regex::Regex;
use rmp_serde::encode;
use serde_json;
use uuid::Uuid;

use crate::user_interface::run_trigger;
// --- States --- //
use crate::{ 
    StateCameraOrbitEntity, 
    StateGame, 
    StateEngineConnection, 
    StateGamePlayStyle, 
    StateLevel, 
    StateMapSet, 
    StateMenu,
    StateTurn, 
    StateUpdateRef
};

// --- Resources --- //
use crate::{
    ClientProtocol,
    DatabaseConnection,
    GameHandler,
    OnlineStateChange,
    PacketAllStates,
    PacketHeartBeat,
    Party,
    RunTrigger,
    UpdateIdResource,
};

use crate::database_handler::db_pipeline_sync_local_player;

pub fn auth_server_handshake(
    db: Res<DatabaseConnection>,
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    party: Res<Party>,
    client_protocol: Res<ClientProtocol>,
) {
    let conn = db.get_connection();
    let conn = conn.lock().unwrap(); // Lock the mutex

    let peers: Vec<_> = socket.connected_peers().collect();
    
    let player_info_tuple: (String, String, String) = conn
        .query_row(
            "SELECT player_id, username, email FROM player_table LIMIT 1",
            [],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,  // Fetch player_id as String
                    row.get::<_, String>(1)?,  // Fetch username as String
                    row.get::<_, String>(2)?,  // Fetch email as String
                ))
            },
        )
        .expect("Failed to retrieve player info");
    info!("player_info: {:?}", player_info_tuple.clone());

    let player_info = format!("{}, {}, {}", player_info_tuple.0, player_info_tuple.1, player_info_tuple.2);

    let message = format!{
        "({}, ({}))",
        client_protocol.init_player_connection(),
        player_info,
    };

    // Serialize the PacketAllStates instance to MessagePack format
    let serialized_message = match encode::to_vec(&message) {
        Ok(bytes) => bytes,
        Err(err) => {
            error!("Failed to serialize auth_server_handshake: {:?}", err);
            return;
        }
    };

    for peer in peers {
        info!("Sending serialized auth_server_handshake: {message:?} to {peer}");
        socket.send(serialized_message.clone().into(), peer);
    }
}

pub fn network_get_client_state_all(
    mut run_trigger: ResMut<RunTrigger>,
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    party: Res<Party>,
    state_cam_orbit_entity: Res<State<StateCameraOrbitEntity>>,
    state_game: Res<State<StateGame>>,
    state_game_play_style: Res<State<StateGamePlayStyle>>,
    state_level: Res<State<StateLevel>>,
    state_map_set: Res<State<StateMapSet>>,
    state_menu: Res<State<StateMenu>>,
    state_turn: Res<State<StateTurn>>,
    client_protocol: Res<ClientProtocol>,
) {
    let peers: Vec<_> = socket.connected_peers().collect();
    
    let player_id = format!("{:?}", party.main_player_get_player_id());
    let state_game = format!("{:?}", state_game.get());
    let state_cam_orbit_entity = format!("{:?}", state_cam_orbit_entity.get());
    let state_game_play_style = format!("{:?}", state_game_play_style.get());
    let state_level = format!("{:?}", state_level.get());
    let state_map_set = format!("{:?}", state_map_set.get());
    let state_menu = format!("{:?}", state_menu.get());
    let state_turn = format!("{:?}", state_turn.get());
    
    let player_id = player_id.as_str();
    let state_game = state_game.as_str();
    let state_cam_orbit_entity = state_cam_orbit_entity.as_str();
    let state_game_play_style = state_game_play_style.as_str();
    let state_level = state_level.as_str();
    let state_map_set = state_map_set.as_str();
    let state_menu = state_menu.as_str();
    let state_turn = state_turn.as_str();


    // Create an instance of PacketAllStates struct with the current state values.
    let all_states = PacketAllStates {
        player_id,
        state_game,
        state_cam_orbit_entity,
        state_game_play_style,
        state_level,
        state_map_set,
        state_menu,
        state_turn,
    };

    // Convert the PacketAllStates to JSON format for easy text-based parsing later
    let packet_json = match serde_json::to_string(&all_states) {
        Ok(json) => json,
        Err(err) => {
            error!("Failed to convert PacketAllStates to JSON: {:?}", err);
            return;
        }
    };
    
    let message = format!{
        "({}, ({}))",
        client_protocol.all_states_packet(),
        packet_json,
    };
    info!("Pretty: PacketAllStates: {:#?}", &message);

    // Serialize the PacketAllStates instance to MessagePack format
    let serialized_message = match encode::to_vec(&message) {
        Ok(bytes) => bytes,
        Err(err) => {
            error!("Failed to serialize PacketAllStates: {:?}", err);
            return;
        }
    };

    for peer in peers {
        info!("Sending serialized PacketAllStates to {peer}");
        socket.send(serialized_message.clone().into(), peer);
    }

    run_trigger.set_target("network_get_client_state_all", false);
}

pub fn network_get_client_state_game(
    mut run_trigger: ResMut<RunTrigger>,
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    party: Res<Party>,
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

pub fn receive_messages(
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    mut game_handler: ResMut<GameHandler>,
    mut online_event_handler: EventWriter<OnlineStateChange>,
    party: ResMut<Party>,
    db: Res<DatabaseConnection>,
    update_id_res: ResMut<UpdateIdResource>,
    run_trigger: ResMut<RunTrigger>,
) {
    for (peer, state) in socket.update_peers() {
        info!("{peer}: {state:?}");
    }

    let mut parse_message = false;
    let mut op_message = None;
    for (_id, message) in socket.receive() {
        info!("Received message: {:?}", message.clone());
        match std::str::from_utf8(&message) {
            Ok(message) => {
                let owned_message = message.to_owned();
                op_message = Some(owned_message);
                parse_message = true;
            },
            Err(e) => error!("receive_messages: Failed to convert message to string: {e}"),
        }
    }
    if parse_message == true{
        server_parse_message(
            op_message.unwrap().as_str(), 
            &mut game_handler, 
            &mut online_event_handler, 
            party, 
            db, 
            update_id_res,
            run_trigger,
        );
    };
}

pub fn remote_state_change_monitor(
    mut online_event_listener: EventReader<OnlineStateChange>,
    mut game_handler: ResMut<GameHandler>,
    mut next_state_connection: ResMut<NextState<StateEngineConnection>>,
    mut next_state_game: ResMut<NextState<StateGame>>,
    mut next_state_level: ResMut<NextState<StateLevel>>,
    mut next_state_map_set: ResMut<NextState<StateMapSet>>,
    mut next_state_turn: ResMut<NextState<StateTurn>>,
    mut next_state_game_play_style: ResMut<NextState<StateGamePlayStyle>>,
    mut next_state_camera_orbit_entity: ResMut<NextState<StateCameraOrbitEntity>>,
) {
    for _ev in online_event_listener.read() {
        let pushed_state =  game_handler.get_pushed_state();
        match pushed_state {
            StateUpdateRef::StateEngineConnection(state_engine_connection) => {
                info!("StateEngineConnection: {:?}", state_engine_connection);
                next_state_connection.set(state_engine_connection); 
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
        }
    }
}

pub fn send_client_heartbeat(
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    party: Res<Party>,
    client_protocol: Res<ClientProtocol>,
) {
    let peers: Vec<_> = socket.connected_peers().collect();

    // Get the player ID
    let player_id = party.main_player_get_player_id();
    let player_id_str = player_id.to_string();

    // Create an instance of PacketHeartBeat
    let heartbeat_packet = PacketHeartBeat {
        player_id: player_id_str.as_str(),
    };

    // Convert the PacketHeartBeat to JSON format
    let packet_json = match serde_json::to_string(&heartbeat_packet) {
        Ok(json) => json,
        Err(err) => {
            error!("Failed to convert PacketHeartBeat to JSON: {:?}", err);
            return;
        }
    };

    // Construct the message in the same format as other packets
    let message = format!(
        "({}, ({}))",
        client_protocol.heart_beat_packet(),
        packet_json,
    );
    info!("Sending PacketHeartBeat message: {:#?}", &message);

    // Serialize the PacketHeartBeat instance to MessagePack format
    let serialized_message = match encode::to_vec(&message) {
        Ok(bytes) => bytes,
        Err(err) => {
            error!("Failed to serialize PacketHeartBeat: {:?}", err);
            return;
        }
    };

    // Send the heartbeat message to all connected peers
    for peer in peers {
        info!("Sending serialized PacketHeartBeat to peer {:?}", peer);
        socket.send(serialized_message.clone().into(), peer);
    }
}

pub fn server_parse_message(
    message: &str,
    game_handler: &mut ResMut<GameHandler>,
    online_event_handler: &mut EventWriter<OnlineStateChange>,
    party: ResMut<Party>,
    db: Res<DatabaseConnection>,
    mut update_id_res: ResMut<UpdateIdResource>,
    mut run_trigger: ResMut<RunTrigger>,
) {
    info!("server_parse_message: Initiated");
    info!("message: {}", &message);

    // Regex to match any command, with an optional UUID or other payload
    let re = Regex::new(r#"^\(([^,]+), ([a-zA-Z]+)(\("([^"]+)"\))?\)$"#).unwrap();
    
    if let Some(caps) = re.captures(message) {
        let player_id = String::from(party.main_player_get_player_id());

        let target_client = caps.get(1).map_or("", |m| m.as_str());
        let command = caps.get(2).map_or("", |m| m.as_str());

        if target_client == player_id {
            match command {
                "SyncExistingPlayerId" => {
                    // Handle SyncExistingPlayerId command
                    if let Some(uuid_str) = caps.get(4).map(|m| m.as_str()) {
                        if let Ok(parsed_uuid) = Uuid::parse_str(uuid_str) {
                            let update_id = Some(parsed_uuid);
                            info!("parsed_uuid: {:?}", update_id);
                            update_id_res.update_id = update_id;
                            db_pipeline_sync_local_player(db, party, update_id_res);
                        }
                    }
                },
                "InitPlayerConnection" => {
                    // Handle InitPlayerConnection command
                    if game_handler.is_not_connected() {
                        let parsed_state = Some(StateUpdateRef::StateEngineConnection(
                            StateEngineConnection::Online,
                        ));
                        info!("Parsed state update: {:?}", parsed_state);
                        game_handler.auth_server_handshake_received(parsed_state);
                        online_event_handler.send(OnlineStateChange);
                    }
                },
                "RunTrigger" => {
                    if let Some(trigger) = caps.get(4).map(|m| m.as_str()) {
                        info!("run_trigger: {:?}", trigger);
                        run_trigger.set_target(trigger, true);
                    }
                },
                _ => {
                    // Handle unknown commands
                    info!("Unknown command: {}", command);
                },
            }
        } else {
            info!("Target client ID does not match player ID.");
        }
    } else {
        info!("Invalid message format: {}", message);
    }
}

pub fn start_socket(mut commands: Commands) {
    let socket = MatchboxSocket::new_reliable("ws://localhost:3536/minigolf");
    commands.insert_resource(socket);
}