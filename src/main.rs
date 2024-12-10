// --- Internal Bevy Plugins --- //
use bevy::{prelude::*,
    input::common_conditions::{
        input_just_pressed, 
        input_just_released,
        input_pressed,
    },
    utils::Duration, 
    window::{PresentMode, WindowTheme},
};

// --- External Plugins --- //
use bevy_rapier3d::prelude::*;
// use bevy_matchbox::prelude::*;

// --- States --- //
use minigolf::{
    StateArrow, 
    StateCameraOrbitEntity, 
    StateEngineConnection, 
    StateGame, 
    StateGamePlayStyle, 
    StateLevel, 
    StateMapSet, 
    StateMenu, 
    StateTurn,
};

// --- Resources --- //
use minigolf::{
    BonkHandler,
    CameraHandler,
    ClientProtocol,
    DatabaseConnection,
    Fonts,
    GameHandler,
    GLBStorageID,
    GolfBall,
    LeaderBoard,
    HeartbeatTimer,
    OnlineStateChange,
    Party,
    PhysicsHandler,
    RunTrigger,
    UiUpdateEvent,
    UiUpdateTimer,
    UpdateIdResource,
};

// --- User Camera World Import --- //
use minigolf::{
    database_handler::db_pipeline_init_local_player,
    game_handler::{
        game_handler_game_start,
        game_handler_game_state_exit_routines,
        game_handler_game_state_start_routines,
    },
    level_handler::{
        level_handler::{
            level_handler_boot_protocals,
            level_handler_init_level_game_handler_current_level,
            level_handler_next_turn_protocol,
            level_handler_purge_protocol,
            level_handler_set_state_next_level,
            level_handler_set_state_next_map_set,
        },
        physics_handler::{
            add_physics_query_and_update_scene,

            golf_ball_handler_active_player_manual_bonk,
            golf_ball_handler_end_game,
            golf_ball_handler_party_store_locations,
            golf_ball_handler_reset_golf_ball_locations,
            golf_ball_handler_spawn_golf_balls_for_party_members,

            bonk_step_start,
            bonk_step_mid,
            bonk_step_end,
            // collision_events_listener,
            
            performance_physics_setup,
        },
    },
    player_handler::{
        leader_board_handler::{
            leader_board_log_game,
            leader_board_review_last_game,
        },
        party_handler::{
            party_handler_active_player_add_bonk,
            party_handler_active_player_set_hole_completion_state_true,
            party_handler_cycle_active_player,
            party_handler_new_player_ai,
            party_handler_new_player_local,
            party_handler_new_player_remote,
            party_handler_remove_ai,
            party_handler_remove_last_player,
        },
    },
    network_handler::network_get_client_state_game,
    // network_handler::{
    //     auth_server_handshake,
    //     heartbeat_system,
    //     network_get_client_state_game,
    //     start_socket,
    //     receive_messages,
    //     remote_state_change_monitor,
    // },
    user_interface::{
        camera_handler::{
            camera_handler_cycle_state_camera,
            setup_3d_camera,
            pan_orbit_camera, 
            state_camera_orbit_entity_logic,
        },
        turn_handler::{
            turn_handler_end_game,
            turn_handler_next_round_prep,
            turn_handler_set_turn_next,
        },
        ray_system_handler::{
            draw_cursor,
            ray_fire,
            ray_release,
        },
        user_interface::{
            bonk_gizmo,
            setup_ui,
            update_ui,
        },
    },
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
        .add_plugins(bevy_tokio_tasks::TokioTasksPlugin::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        // .add_plugins(EditorPlugin::default())
    
        // --- State Initialization --- //
        .insert_state(StateArrow::Idle)
        .insert_state(StateCameraOrbitEntity::Menu)
        .insert_state(StateEngineConnection::Local)
        .insert_state(StateGame::NotInGame)
        .insert_state(StateGamePlayStyle::SetOrder)
        .insert_state(StateLevel::MainMenu)
        .insert_state(StateMapSet::Tutorial)
        .insert_state(StateMenu::MenuMainMenu)
        .insert_state(StateTurn::NotInGame)

        // --- Timer Initialization --- //
        .insert_resource(HeartbeatTimer(Timer::new(Duration::from_secs(5), TimerMode::Repeating)))
        .insert_resource(UiUpdateTimer(Timer::new(Duration::from_millis(250), TimerMode::Repeating)))

        // --- Resource Initialization --- //
        .insert_resource(BonkHandler::new())
        .insert_resource(DatabaseConnection::new("game_data.db"))
        .insert_resource(CameraHandler::new())
        .insert_resource(ClientProtocol::new())
        .insert_resource(GameHandler::new())
        .insert_resource(GLBStorageID::new())
        .insert_resource(Fonts::new())
        .insert_resource(LeaderBoard::new()) 
        .insert_resource(Party::new())
        .insert_resource(PhysicsHandler::new())
        .insert_resource(RunTrigger::new())
        .insert_resource(UpdateIdResource { update_id: None })

        // --- Event Initialization --- //
        .add_event::<OnlineStateChange>()
        .add_event::<UiUpdateEvent>()  

        // --- Startup Systems Initialization --- //
        .add_systems(Startup, level_handler_boot_protocals)
        .add_systems(Startup, setup_3d_camera)
        .add_systems(Startup, setup_ui)
        .add_systems(Startup, db_pipeline_init_local_player)
        .add_systems(Startup, performance_physics_setup)
        // .add_systems(Startup, start_socket)

        // Network - Update //
        // .add_systems(Update, auth_server_handshake
            // .run_if(|game_handler: Res<GameHandler>|game_handler.is_not_connected())
            // .run_if(on_timer(Duration::from_millis(500))))
        // .add_systems(Update, heartbeat_system)
        // .add_systems(Update, receive_messages)
        // .add_systems(Update, remote_state_change_monitor)

        // Physics //
        .add_systems(Update, bonk_step_start.run_if(input_just_pressed(MouseButton::Right)))
        .add_systems(Update, bonk_step_mid.run_if(input_pressed(MouseButton::Right)))
        .add_systems(Update, bonk_step_end.run_if(input_just_released(MouseButton::Right)))

        // Camera //
        .add_systems(Update, state_camera_orbit_entity_logic)
        .add_systems(Update, pan_orbit_camera)

        // User Interface //
        .add_systems(Update, draw_cursor)
        .add_systems(Update, ray_fire.run_if(input_just_pressed(MouseButton::Left)))
        .add_systems(Update, ray_release.run_if(input_just_released(MouseButton::Left)))
        .add_systems(Update, bonk_gizmo.run_if(in_state(StateArrow::DrawingArrow)))
        .add_systems(Update, update_ui)

        // Run Trigger Systems //        
        .add_systems(Update, camera_handler_cycle_state_camera.run_if(|run_trigger: Res<RunTrigger>|run_trigger.camera_handler_cycle_state_camera()))
      
        .add_systems(Update, game_handler_game_start.run_if(|run_trigger: Res<RunTrigger>|run_trigger.game_handler_game_start()))
        .add_systems(Update, game_handler_game_state_exit_routines.run_if(|run_trigger: Res<RunTrigger>|run_trigger.game_handler_game_state_exit_routines()))
        .add_systems(Update, game_handler_game_state_start_routines.run_if(|run_trigger: Res<RunTrigger>|run_trigger.game_handler_game_state_start_routines()))

        .add_systems(Update, golf_ball_handler_active_player_manual_bonk.run_if(|run_trigger: Res<RunTrigger>|run_trigger.golf_ball_handler_active_player_manual_bonk()))
        .add_systems(Update, golf_ball_handler_end_game.run_if(|run_trigger: Res<RunTrigger>|run_trigger.golf_ball_handler_end_game()))
        .add_systems(Update, golf_ball_handler_party_store_locations.run_if(|run_trigger: Res<RunTrigger>|run_trigger.golf_ball_handler_party_store_locations()))
        .add_systems(Update, golf_ball_handler_reset_golf_ball_locations.run_if(|run_trigger: Res<RunTrigger>|run_trigger.golf_ball_handler_reset_golf_ball_locations()))
        .add_systems(Update, golf_ball_handler_spawn_golf_balls_for_party_members.run_if(|run_trigger: Res<RunTrigger>|run_trigger.golf_ball_handler_spawn_golf_balls_for_party_members()))

        .add_systems(Update, leader_board_log_game.run_if(|run_trigger: Res<RunTrigger>|run_trigger.leader_board_log_game()))
        .add_systems(Update, leader_board_review_last_game.run_if(|run_trigger: Res<RunTrigger>|run_trigger.leader_board_review_last_game()))
        
        .add_systems(Update, level_handler_init_level_game_handler_current_level.run_if(|run_trigger: Res<RunTrigger>|run_trigger.level_handler_init_level_game_handler_current_level()))
        .add_systems(Update, level_handler_next_turn_protocol.run_if(|run_trigger: Res<RunTrigger>|run_trigger.level_handler_next_turn_protocol()))
        .add_systems(Update, level_handler_purge_protocol.run_if(|run_trigger: Res<RunTrigger>|run_trigger.level_handler_purge_protocol()))
        .add_systems(Update, level_handler_set_state_next_level.run_if(|run_trigger: Res<RunTrigger>|run_trigger.level_handler_set_state_next_level()))
        .add_systems(Update, level_handler_set_state_next_map_set.run_if(|run_trigger: Res<RunTrigger>|run_trigger.level_handler_set_state_next_map_set()))

        .add_systems(Update, network_get_client_state_game.run_if(|run_trigger: Res<RunTrigger>|run_trigger.network_get_client_state_game()))
        
        .add_systems(Update, party_handler_active_player_add_bonk.run_if(|run_trigger: Res<RunTrigger>|run_trigger.party_handler_active_player_add_bonk()))
        .add_systems(Update, party_handler_active_player_set_hole_completion_state_true.run_if(|run_trigger: Res<RunTrigger>|run_trigger.party_handler_active_player_set_hole_completion_state_true()))
        
        .add_systems(Update, party_handler_cycle_active_player.run_if(|run_trigger: Res<RunTrigger>|run_trigger.party_handler_cycle_active_player()))
        
        .add_systems(Update, party_handler_new_player_ai.run_if(|run_trigger: Res<RunTrigger>|run_trigger.party_handler_new_player_ai()))
        .add_systems(Update, party_handler_new_player_local.run_if(|run_trigger: Res<RunTrigger>|run_trigger.party_handler_new_player_local()))
        .add_systems(Update, party_handler_new_player_remote.run_if(|run_trigger: Res<RunTrigger>|run_trigger.party_handler_new_player_remote()))
        
        .add_systems(Update, party_handler_remove_last_player.run_if(|run_trigger: Res<RunTrigger>|run_trigger.party_handler_remove_last_player()))
        .add_systems(Update, party_handler_remove_ai.run_if(|run_trigger: Res<RunTrigger>|run_trigger.party_handler_remove_ai()))

        .add_systems(Update, turn_handler_end_game.run_if(|run_trigger: Res<RunTrigger>|run_trigger.turn_handler_end_game()))
        .add_systems(Update, turn_handler_next_round_prep.run_if(|run_trigger: Res<RunTrigger>|run_trigger.turn_handler_next_round_prep()))
        .add_systems(Update, turn_handler_set_turn_next.run_if(|run_trigger: Res<RunTrigger>|run_trigger.turn_handler_set_turn_next()))

        .add_systems(Update, golf_ball_query.run_if(input_just_pressed(KeyCode::KeyU)))
        .add_systems(Update, add_physics_query_and_update_scene.run_if(input_just_pressed(KeyCode::KeyI)))
        .add_systems(Update, debug_names_query.run_if(input_just_pressed(KeyCode::KeyO)))
        .add_systems(Update, party_query.run_if(input_just_pressed(KeyCode::KeyP)))
        .add_systems(Update, temp_interface);

    app.run();
}

fn debug_names_query(query: Query<(&Name, &GolfBall)>) {
    for (name, golf_ball) in query.iter() {
        info!("Entity Name: {}, GolfBall UUID: {}", name.as_str(), golf_ball.0.uuid);
    }
}

fn golf_ball_query(
    golf_balls: Query<(Entity, &GolfBall)>,
) {
    for (entity, golf_ball) in golf_balls.iter() {
        info!("Entity: {:?}, GolfBall: {:?}", entity, golf_ball.0);
    }
}

fn party_query(
    party: Res<Party>,
) {
    info!("Party ID's and Scores: [{:?}]", party.get_all_player_ids_and_scores());
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
        run_trigger.set_target("camera_handler_cycle_state_camera", true);
    };
    if keys.just_released(KeyCode::KeyM) {
        info!("just_released: KeyM");  
        match state_game.get() {
            StateGame::InGame => {},
            StateGame::NotInGame => {
                run_trigger.set_target("level_handler_set_state_next_map_set", true);
            },
        };
    };
    if keys.just_released(KeyCode::KeyN) {
        info!("just_released: KeyN");  
        match state_game.get() {
            StateGame::NotInGame => {},
            StateGame::InGame => {
                run_trigger.set_target("turn_handler_set_turn_next", true);
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
    if keys.just_released(KeyCode::KeyQ) {
        info!("just_released: KeyQ");  
        run_trigger.set_target("network_get_client_state_all", true);
    };
    if keys.just_released(KeyCode::KeyS) {
        info!("just_released: KeyS");  
        match state_game.get() {
            StateGame::InGame => {},
            StateGame::NotInGame => {
                run_trigger.set_target("game_handler_game_start", true);
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

// pub fn devfn_receive_messages_map_set(
//     mut socket: ResMut<MatchboxSocket<SingleChannel>>,
//     mut client_map_sets: ResMut<HashMap<Uuid, OffsetDateTime>>,
//     mut game_handler: ResMut<GameHandler>,
//     mut online_event_handler: EventWriter<OnlineStateChange>,
// ) {
//     for (peer, state) in socket.update_peers() {
//         info!("{peer}: {state:?}");
//     }

//     for (_id, message) in socket.receive() {
//         // Attempt to deserialize the message into a summary or a full map set
//         if let Ok(summary) = decode::<Vec<(Uuid, OffsetDateTime)>>(&message) {
//             // Summary received, now crosscheck and determine which maps are missing or outdated
//             let mut request_full_map_sets = false;

//             for (map_set_id, timestamp) in summary {
//                 if let Some(existing_timestamp) = client_map_sets.get(&map_set_id) {
//                     if existing_timestamp < &timestamp {
//                         // Local version is outdated
//                         request_full_map_sets = true;
//                         break;
//                     }
//                 } else {
//                     // Local version is missing this map set
//                     request_full_map_sets = true;
//                     break;
//                 }
//             }

//             if request_full_map_sets {
//                 // Send a request to the host for the full map set data
//                 let request_message = "REQUEST_FULL_MAP_SETS".as_bytes();
//                 socket.send(request_message.into(), _id);
//                 info!("Requested full map sets from the host.");
//             }
//         } else if let Ok(map_sets) = decode::<Vec<MapSet>>(&message) {
//             // Full map set data received, update the client database
//             for map_set in map_sets {
//                 client_map_sets.insert(map_set.map_set_id.clone(), map_set.last_updated);
//             }
//             info!("Updated local map sets from received full map set data.");
//         } else {
//             error!("Failed to parse incoming message.");
//         }
//     }
// }