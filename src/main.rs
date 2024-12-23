// --- Internal Bevy Plugins --- //
use bevy::{prelude::*,
    input::common_conditions::{
        input_just_pressed, 
        input_just_released,
        input_pressed,
    },
    time::common_conditions::on_timer,
    utils::Duration, 
    window::{
        PresentMode, 
        // WindowMode::BorderlessFullscreen, 
        WindowTheme,
    },
};

// --- External Plugins --- //
use bevy_easy_vec_ui::BevyEasyVecUiPlugin;
use bevy_rapier3d::prelude::*;
// use bevy_matchbox::prelude::*;

// --- States --- //
use minigolf::{
    StateArrow, 
    // StateCameraMenuTarget,
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
    GameHandler,
    GLBStorageID,
    GolfBall,
    LeaderBoard,
    HeartbeatTimer,
    OnlineStateChange,
    Party,
    PhysicsHandler,
    PurgeHandler,
    RunTrigger,
    SceneInstanceOutOfBoundGolfBall,
    SceneInstancePurgedEnvironment,
    SceneInstancePurgedGolfBalls,
    SceneInstanceRespawnedGolfBall,
    SceneInstanceSpawnedEnvironment,
    SceneInstanceSpawnedGolfBalls,
    StatesRef,
    UpdateIdResource,
};

// --- User Camera World Import --- //
use minigolf::{
    database_handler::db_pipeline_init_local_player,
    game_handler::{
        game_handler_game_start,
        game_handler_game_state_exit_routines,
        game_handler_game_state_start_routines,
        game_handler_start_local_back_nine,
        game_handler_start_local_front_nine,
        game_handler_start_local_select_a_hole,
        game_handler_start_local_whole_corse,
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

            bonk_step_start,
            bonk_step_mid,
            bonk_step_end,
            
            collision_events_listener,

            golf_ball_handler_update_locations_post_bonk,
            golf_ball_handler_end_game,
            golf_ball_handler_respawn_golf_ball_uuid,
            golf_ball_handler_party_store_locations,
            golf_ball_handler_reset_golf_ball_locations,
            golf_ball_handler_spawn_golf_balls_for_party_members,
            golf_balls_update_sleep_status,

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
    network_handler::{
        // auth_server_handshake,
        // heartbeat_system,
        network_get_client_state_all,
        network_get_client_state_game,
        // receive_messages,
        // remote_state_change_monitor,
        // start_socket,
    },
    user_interface::{
        camera_handler::{
            camera_handler_cycle_state_camera,
            // camera_handler_cycle_state_camera_menu_target,
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
            easy_vec_ui,
            updated_states_ref,
        },
    },
};

fn main() {
    let mut app = App::new();
        app.add_plugins((
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(Window {
                        // mode: BorderlessFullscreen,
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
        .add_plugins(BevyEasyVecUiPlugin::init("fonts/MatrixtypeDisplay-KVELZ.ttf")
            .camera_layer(-1)
            .title("Minigolf Backend Server: UI")
            .title_font_size(42.0) // Default is 42
            .data_font_size(10.0) // Default is 12
            .build()
        )
        // .add_plugins(RapierDebugRenderPlugin::default())
        // .add_plugins(EditorPlugin::default())
    
        // --- State Initialization --- //
        .insert_state(StateArrow::Idle)
        // .insert_state(StateCameraMenuTarget::Main)
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

        // --- Resource Initialization --- //
        .insert_resource(BonkHandler::new())
        .insert_resource(DatabaseConnection::new("game_data.db"))
        .insert_resource(CameraHandler::new())
        .insert_resource(ClientProtocol::new())
        .insert_resource(GameHandler::new())
        .insert_resource(GLBStorageID::new())
        .insert_resource(LeaderBoard::new()) 
        .insert_resource(Party::new())
        .insert_resource(PhysicsHandler::new())
        .insert_resource(PurgeHandler::new())
        .insert_resource(RunTrigger::new())
        .insert_resource(StatesRef::new())
        .insert_resource(UpdateIdResource { update_id: None })

        // --- Event Initialization --- //
        .add_event::<SceneInstanceOutOfBoundGolfBall>()
        .add_event::<SceneInstancePurgedEnvironment>()
        .add_event::<SceneInstancePurgedGolfBalls>()
        .add_event::<SceneInstanceRespawnedGolfBall>()
        .add_event::<SceneInstanceSpawnedEnvironment>()
        .add_event::<SceneInstanceSpawnedGolfBalls>()
        .add_event::<OnlineStateChange>()

        // --- Startup Systems Initialization --- //
        .add_systems(Startup, level_handler_boot_protocals)
        .add_systems(Startup, setup_3d_camera)
        .add_systems(Startup, db_pipeline_init_local_player)
        .add_systems(Startup, performance_physics_setup)

        // // Network //
        // .add_systems(Startup, start_socket)
        // .add_systems(Update, auth_server_handshake
        //     .run_if(|game_handler: Res<GameHandler>|!game_handler.get("network_server_connection"))
        //     .run_if(on_timer(Duration::from_millis(500))))
        // .add_systems(Update, heartbeat_system)
        // .add_systems(Update, receive_messages)
        // .add_systems(Update, remote_state_change_monitor)

        // Physics //
        .add_systems(Update, bonk_step_start.run_if(input_just_pressed(MouseButton::Right)))
        .add_systems(Update, bonk_step_mid.run_if(input_pressed(MouseButton::Right)))
        .add_systems(Update, bonk_step_end.run_if(input_just_released(MouseButton::Right)))
        .add_systems(Update, collision_events_listener)

        // Camera //
        .add_systems(Update, state_camera_orbit_entity_logic)
        .add_systems(Update, pan_orbit_camera)

        // User Interface //
        .add_systems(Update, draw_cursor)
        .add_systems(Update, ray_fire.run_if(input_just_pressed(MouseButton::Left)))
        .add_systems(Update, ray_release.run_if(input_just_released(MouseButton::Left)))
        .add_systems(Update, bonk_gizmo.run_if(in_state(StateArrow::DrawingArrow)))
        .add_systems(Update, easy_vec_ui)

        // Run Trigger Systems //
        .add_systems(Update, add_physics_query_and_update_scene.run_if(|run_trigger: Res<RunTrigger>|run_trigger.add_physics_query_and_update_scene()))

        .add_systems(Update, camera_handler_cycle_state_camera.run_if(|run_trigger: Res<RunTrigger>|run_trigger.camera_handler_cycle_state_camera()))
        // .add_systems(Update, camera_handler_cycle_state_camera_menu_target.run_if(|run_trigger: Res<RunTrigger>|run_trigger.camera_handler_cycle_state_camera_menu_target()))

        .add_systems(Update, game_handler_game_start.run_if(|run_trigger: Res<RunTrigger>|run_trigger.game_handler_game_start()))
        .add_systems(Update, game_handler_game_state_exit_routines.run_if(|run_trigger: Res<RunTrigger>|run_trigger.game_handler_game_state_exit_routines()))
        .add_systems(Update, game_handler_game_state_start_routines.run_if(|run_trigger: Res<RunTrigger>|run_trigger.game_handler_game_state_start_routines()))

        .add_systems(Update, game_handler_start_local_back_nine.run_if(|run_trigger: Res<RunTrigger>|run_trigger.game_handler_start_local_back_nine()))
        .add_systems(Update, game_handler_start_local_front_nine.run_if(|run_trigger: Res<RunTrigger>|run_trigger.game_handler_start_local_front_nine()))
        .add_systems(Update, game_handler_start_local_select_a_hole.run_if(|run_trigger: Res<RunTrigger>|run_trigger.game_handler_start_local_select_a_hole()))
        .add_systems(Update, game_handler_start_local_whole_corse.run_if(|run_trigger: Res<RunTrigger>|run_trigger.game_handler_start_local_whole_corse()))

        .add_systems(Update, golf_ball_handler_update_locations_post_bonk.run_if(|run_trigger: Res<RunTrigger>|run_trigger.golf_ball_handler_update_locations_post_bonk()))
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

        .add_systems(Update, network_get_client_state_all.run_if(|run_trigger: Res<RunTrigger>|run_trigger.network_get_client_state_all()))
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

        .add_systems(Update, start_movement_listener_turn_handler_set_turn_next.run_if(|run_trigger: Res<RunTrigger>|run_trigger.start_movement_listener_turn_handler_set_turn_next()))

        .add_systems(Update, temp_interface)
        .add_systems(Update, debug_with_optional_parent.run_if(input_just_pressed(KeyCode::KeyT)))
        .add_systems(Update, last_game_record.run_if(input_just_pressed(KeyCode::KeyY)))
        .add_systems(Update, golf_ball_query.run_if(input_just_pressed(KeyCode::KeyU)))
        .add_systems(Update, debug_names_query.run_if(input_just_pressed(KeyCode::KeyO)))
        .add_systems(Update, party_query.run_if(input_just_pressed(KeyCode::KeyP)))
        .add_systems(Update, listening_function_local_all_sleeping)
        .add_systems(Update, listening_function_local_add_physics
            .run_if(on_timer(Duration::from_millis(500))))
        .add_systems(Update, listening_function_local_all_finished
            .run_if(on_timer(Duration::from_millis(250))))
        .add_systems(Update, listening_function_local_respawn_add_physics)
        .add_systems(Update, listening_function_purge_events)
        .add_systems(Update, listening_function_spawned_environment_events)
        .add_systems(Update, listening_function_spawned_golf_ball_events)
        .add_systems(Update, golf_ball_handler_respawn_golf_ball)
        .add_systems(Update, golf_ball_handler_update_locations_while_in_game)
        .add_systems(Update, golf_balls_update_sleep_status)
        .add_systems(Update, updated_states_ref);

    app.run();
}

fn debug_names_query(query: Query<(&Name, &GolfBall)>) {
    for (name, golf_ball) in query.iter() {
        info!("Entity Name: {}, GolfBall UUID: {}", name.as_str(), golf_ball.0.uuid);
    }
}

fn debug_with_optional_parent(query: Query<(&GolfBall, Option<&Parent>)>) {
    for (golf_ball, parent) in query.iter() {
        info!(
            "GolfBall UUID: {:?}, Parent: {:?}",
            golf_ball.0.uuid,
            parent.map(|p| p.get())
        );
    }
}

fn golf_ball_handler_update_locations_while_in_game(
    mut gb_query: Query<(&mut GolfBall, &Transform)>,
    state_game: Res<State<StateGame>>,
) {
    if state_game.get() == &StateGame::InGame {
        for (mut golf_ball, transform) in gb_query.iter_mut() {
            golf_ball.0.position = transform.translation;
        };
    };
}

fn golf_ball_handler_respawn_golf_ball(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    glb_storage: Res<GLBStorageID>, //Arc<[MapID]> //map: Arc<str>,
    mut oob_event_reader: EventReader<SceneInstanceOutOfBoundGolfBall>,
    mut asset_event_writer: EventWriter<SceneInstanceRespawnedGolfBall>,
    mut game_handler: ResMut<GameHandler>,
) {
    for event in oob_event_reader.read() {
        info!("golf Ball Out Of Bounds: [{:?}]", event);
        let info_vec = &event.info_vec;
        for player in info_vec.iter() {
            let player_id = player.0;
            let location = player.1;
            golf_ball_handler_respawn_golf_ball_uuid(&mut commands, &asset_server, &glb_storage, &player_id, &location, &mut asset_event_writer, &mut game_handler);
        };
    }
}

fn golf_ball_query(
    golf_balls: Query<(Entity, &GolfBall)>,
) {
    for (entity, golf_ball) in golf_balls.iter() {
        info!("Entity: {:?}, GolfBall: {:?}", entity, golf_ball.0);
    }
}

fn last_game_record(
    mut run_trigger: ResMut<RunTrigger>,
) {
    run_trigger.set_target("leader_board_review_last_game", true);
}

fn listening_function_local_all_finished(
    mut run_trigger: ResMut<RunTrigger>,
    game_handler: Res<GameHandler>,
    party: Res<Party>,
) {
    if party.all_finished() && !game_handler.get("remote_game") {
        run_trigger.set_target("turn_handler_set_turn_next", true);
    }
}

fn listening_function_local_all_sleeping(
    state_game: Res<State<StateGame>>,
    mut game_handler: ResMut<GameHandler>,
    golf_balls: Query<&GolfBall>,
) {
    if state_game.get() == &StateGame::InGame {
        let mut sleeping: usize = 0;
        let mut total: usize = 0;    
        for (idx, golf_ball) in golf_balls.iter().enumerate() {
            total = idx + 1;
            if golf_ball.0.sleeping == true {
                sleeping += 1;
            }
        }
        
        if sleeping == total {
            game_handler.set_target("all_sleeping", true);
        } else {
            game_handler.set_target("all_sleeping", false);
        }
    }
}

fn listening_function_local_add_physics(
    mut run_trigger: ResMut<RunTrigger>,
    mut game_handler: ResMut<GameHandler>,
    query: Query<&RapierRigidBodyHandle, With<GolfBall>>,
) {
    let mut count = 0;
    for _ in query.iter() {
        count += 1;
    }
    if !game_handler.get("remote_game") && game_handler.get("in_game") && game_handler.get("round_start") && count == 0 {
        game_handler.set_target("round_start", false);
        run_trigger.set_target("add_physics_query_and_update_scene", true);
    }
}

fn listening_function_local_respawn_add_physics(
    mut respawn_golf_ball: EventReader<SceneInstanceRespawnedGolfBall>,
    mut commands: Commands,
    mut gb_query: Query<(Entity, &mut GolfBall, &mut Transform)>,
) {
    for event in respawn_golf_ball.read() {
        info!("Init: listening_function_local_respawn_add_physics");
        info!("Respawn: [{:?}]", event);
        let id = event.id;
        let point = event.location;
        for (entity, mut golf_ball, mut transform) in gb_query.iter_mut() {
            info!("Golf Ball Pre: [{}], Point: [{}], Real [{:?}]", format!("golf_ball_{}", id.to_string()), point, golf_ball);
            if golf_ball.0.uuid == id {
                transform.translation = point;
                golf_ball.0.last_position = point;
                let collider = Collider::ball(0.022);
                commands
                    .entity(entity)
                    .insert(collider)
                    .insert(RigidBody::Dynamic)
                    .insert(Damping {
                        angular_damping: 3.0,
                        ..default()
                    })
                    .insert(ExternalImpulse::default())
                    .insert(ColliderMassProperties::Density(1.0))
                    .insert(GravityScale(1.0))
                    .insert(Ccd::enabled())
                    .insert(Name::new(format!("golf_ball_{}", id.to_string())));
                    // .insert(TransformBundle::from(Transform::from_xyz(golf_ball.0.last_position.x, golf_ball.0.last_position.y, golf_ball.0.last_position.y)));
            }
            info!("Golf Ball Post: [{}], Target: [{}], Real [{:?}]", format!("golf_ball_{}", id.to_string()), point, golf_ball);
        }
        for (entity, golf_ball, transform) in gb_query.iter() {
            info!("Respawn: [{:?}]::[{}]::[{:?}]", golf_ball, entity, transform);
        };
    }
}

fn listening_function_purge_events(
    mut game_handler: ResMut<GameHandler>,
    mut purge_handler: ResMut<PurgeHandler>,
    mut purge_event_reader_environment: EventReader<SceneInstancePurgedEnvironment>,
    mut purge_event_reader_golf_balls: EventReader<SceneInstancePurgedGolfBalls>,
) {
    for event in purge_event_reader_environment.read() {
        info!("Environment Purged: [{:?}]", event);
        purge_handler.set_target("environment_purged", true);
        game_handler.set_target("environment_loaded", false);
    }
    for event in purge_event_reader_golf_balls.read() {
        info!("Environment Purged: [{:?}]", event);
        purge_handler.set_target("golf_balls_purged", true);
        game_handler.set_target("golf_balls_loaded", false);
    }
}

fn listening_function_spawned_environment_events(
    mut asset_event_reader: EventReader<SceneInstanceSpawnedEnvironment>,
    mut game_handler: ResMut<GameHandler>,
    mut purge_handler: ResMut<PurgeHandler>,
    mut run_trigger: ResMut<RunTrigger>,
) {
    for event in asset_event_reader.read() {
        info!("Entity: [{:?}]", event);
        purge_handler.set_target("environment_purged", false);
        game_handler.set_target("environment_loaded", true);
        match game_handler.get("in_game") {
            true => {
                info!("listening_function_spawned_environment_events: In Game: Triggering Golf Ball pipeline");
                run_trigger.set_target("golf_ball_handler_spawn_golf_balls_for_party_members", true);
            },
            false => {
                info!("listening_function_spawned_environment_events: Not In Game");},
        }
    }
}

fn listening_function_spawned_golf_ball_events(
    mut asset_event_reader: EventReader<SceneInstanceSpawnedGolfBalls>,
    mut game_handler: ResMut<GameHandler>,
    mut purge_handler: ResMut<PurgeHandler>,
    mut run_trigger: ResMut<RunTrigger>,
) {
    for event in asset_event_reader.read() {
        info!("Entity: [{:?}]", event);
        purge_handler.set_target("golf_balls_purged", false);
        game_handler.set_target("golf_balls_loaded", true);
        run_trigger.set_target("add_physics_query_and_update_scene", true);
    }
}

fn party_query(
    party: Res<Party>,
) {
    info!("Party ID's and Scores: [{:?}]", party.all_players_get_ids_and_scores());
}

fn start_movement_listener_turn_handler_set_turn_next(
    mut run_trigger: ResMut<RunTrigger>,
    game_handler: Res<GameHandler>,
) {
    info!("function: start_movement_listener_turn_handler_set_turn_next"); 
    {
        if game_handler.get("all_sleeping") {
            run_trigger.set_target("golf_ball_handler_update_locations_post_bonk", true);
            run_trigger.set_target("golf_ball_handler_party_store_locations", true);
            run_trigger.set_target("turn_handler_set_turn_next", true);
            run_trigger.set_target("start_movement_listener_turn_handler_set_turn_next", false);
            info!("post response: start_movement_listener_turn_handler_set_turn_next: [{}]", run_trigger.get("start_movement_listener_turn_handler_set_turn_next"));  
        }
    }
}

//-----------------------------------------------------------------------------------//

fn temp_interface(
    mut run_trigger: ResMut<RunTrigger>,
    keys: Res<ButtonInput<KeyCode>>,
    state_game: Res<State<StateGame>>,
) {
    if keys.just_released(KeyCode::KeyA) { // should trigger with new turn
        info!("just_released: KeyA");  
        match state_game.get() {
            StateGame::NotInGame => {},
            StateGame::InGame => {
                run_trigger.set_target("party_handler_active_player_set_hole_completion_state_true", true);
            },
        };
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
    if keys.just_released(KeyCode::KeyC) {
        info!("just_released: KeyC");
        run_trigger.set_target("camera_handler_cycle_state_camera", true);
    };
    if keys.just_released(KeyCode::KeyI) {
        info!("just_released: KeyI");
        run_trigger.set_target("add_physics_query_and_update_scene", true);
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
                run_trigger.set_target("party_handler_cycle_active_player", true);
            },
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
                run_trigger.set_target("level_handler_purge_protocol", true);
                run_trigger.set_target("game_handler_game_start", true);
            },
        };
    };
    if keys.just_released(KeyCode::KeyV) {
        info!("just_released: KeyV");  
        run_trigger.set_target("camera_handler_cycle_state_camera_menu_target", true);
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