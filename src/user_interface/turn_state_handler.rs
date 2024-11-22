use bevy::prelude::*;

use bevy_rapier3d::prelude::RapierRigidBodyHandle;

// States
use crate::{
    ArrowState,
    CameraOrbitEntityState,
    GameState,
    LeaderBoardState,
    LevelState,
    MapSetState,
    PanOrbitState,
    TurnState,
};

// Resources
use crate::{
    Party,
    GameHandler,
    GLBStorageID,
    GolfBallTag,
    Ground,
    LevelHandler,
};

use crate::level_handler::physics_handler::{
    add_physics_query_and_update_scene,
};

use crate::level_handler::level_handler::{
    gltf_handler_init_golf_ball_glb, 
    init_level_glb,
    purge_glb_all, 
    purge_rigid_bodies,
};

// --- OnEnter: Turn State --- //

pub fn turn_state_response_hole_complete(
    mut party: ResMut<Party>,
    map_set_state: Res<State<MapSetState>>,
    level: ResMut<State<LevelState>>,
    level_handler: Res<LevelHandler>,
    mut game_handler: ResMut<GameHandler>,
    mut next_leader_board_state: ResMut<NextState<LeaderBoardState>>,
    mut next_camera_state: ResMut<NextState<CameraOrbitEntityState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_level_state: ResMut<NextState<LevelState>>,
    mut next_turn: ResMut<NextState<TurnState>>,
    mut pan_orbit_camera_query: Query<&mut PanOrbitState>,
) {
    party.active_player_finished_hole(); // Reads active player index and updates target Player's state
    
    let current_level = party.get_active_level();
    let party_size = party.get_party_size();
    
    match party_size {
        1 => {
            match map_set_state.get() {
                MapSetState::Tutorial => {
                    party.end_game(); // Sets players to NotInGame
                    next_game_state.set(GameState::PostGameReview);
                    next_turn.set(TurnState::Idle);
                    game_handler.init_postgame_leaderboard(party); // Set's target for level handling
                    next_leader_board_state.set(LeaderBoardState::PostGame);
                    next_level_state.set(LevelState::MenuLeaderBoard);
                    next_camera_state.set(CameraOrbitEntityState::LeaderBoard);
                    for mut state in pan_orbit_camera_query.iter_mut() {
                        info!("{:?}", state);
                        state.radius = 38.0;
                        state.pitch = -12.0f32.to_radians();
                        state.yaw = -17.0f32.to_radians();
                    }
                },
                MapSetState::WholeCorse => {
                    if current_level == 18 {
                        party.end_game(); // Sets players to NotInGame
                        next_game_state.set(GameState::PostGameReview);
                        next_turn.set(TurnState::Idle);
                        game_handler.init_postgame_leaderboard(party); // Set's target for level handling
                        next_leader_board_state.set(LeaderBoardState::PostGame);
                        next_level_state.set(LevelState::MenuLeaderBoard);
                        next_camera_state.set(CameraOrbitEntityState::LeaderBoard);
                        for mut state in pan_orbit_camera_query.iter_mut() {
                            info!("{:?}", state);
                            state.radius = 38.0;
                            state.pitch = -12.0f32.to_radians();
                            state.yaw = -17.0f32.to_radians();
                        }
                    } else {
                        let set_next_level = level_handler.next_level(current_level);
                        next_level_state.set(set_next_level);
                        game_handler.next_level();
                        party.next_level();
                        next_turn.set(TurnState::Turn);
                        next_camera_state.set(CameraOrbitEntityState::Ball);
                        for mut state in pan_orbit_camera_query.iter_mut() {
                            state.radius = 2.0;
                            state.pitch = -8.0f32.to_radians();
                            state.yaw = 22.0f32.to_radians();
                        }
                    }
                },
                MapSetState::FrontNine => {
                    if current_level == 9 {
                        party.end_game(); // Sets players to NotInGame
                        next_game_state.set(GameState::PostGameReview);
                        next_turn.set(TurnState::Idle);
                        game_handler.init_postgame_leaderboard(party); // Set's target for level handling
                        next_leader_board_state.set(LeaderBoardState::PostGame);
                        next_level_state.set(LevelState::MenuLeaderBoard);
                        next_camera_state.set(CameraOrbitEntityState::LeaderBoard);
                        for mut state in pan_orbit_camera_query.iter_mut() {
                            info!("{:?}", state);
                            state.radius = 38.0;
                            state.pitch = -12.0f32.to_radians();
                            state.yaw = -17.0f32.to_radians();
                        }
                    } else {
                        let set_next_level = level_handler.next_level(current_level);
                        next_level_state.set(set_next_level);
                        game_handler.next_level();
                        party.next_level();
                        next_turn.set(TurnState::Turn);
                        next_camera_state.set(CameraOrbitEntityState::Ball);
                        for mut state in pan_orbit_camera_query.iter_mut() {
                            state.radius = 2.0;
                            state.pitch = -8.0f32.to_radians();
                            state.yaw = 22.0f32.to_radians();
                        }
                    }
                },
                MapSetState::BackNine => {
                    if current_level == 18 {
                        party.end_game(); // Sets players to NotInGame
                        next_game_state.set(GameState::PostGameReview);
                        next_turn.set(TurnState::Idle);
                        game_handler.init_postgame_leaderboard(party); // Set's target for level handling
                        next_leader_board_state.set(LeaderBoardState::PostGame);
                        next_level_state.set(LevelState::MenuLeaderBoard);
                        next_camera_state.set(CameraOrbitEntityState::LeaderBoard);
                        for mut state in pan_orbit_camera_query.iter_mut() {
                            info!("{:?}", state);
                            state.radius = 38.0;
                            state.pitch = -12.0f32.to_radians();
                            state.yaw = -17.0f32.to_radians();
                        }
                    } else {
                        let set_next_level = level_handler.next_level(current_level);
                        next_level_state.set(set_next_level);
                        game_handler.next_level();
                        party.next_level();
                        next_turn.set(TurnState::Turn);
                        next_camera_state.set(CameraOrbitEntityState::Ball);
                        for mut state in pan_orbit_camera_query.iter_mut() {
                            state.radius = 2.0;
                            state.pitch = -8.0f32.to_radians();
                            state.yaw = 22.0f32.to_radians();
                        }
                    }
                },
                MapSetState::SelectAHole => {
                    party.end_game(); // Sets players to NotInGame
                    next_game_state.set(GameState::PostGameReview);
                    next_turn.set(TurnState::Idle);
                    game_handler.init_postgame_leaderboard(party); // Set's target for level handling
                    next_leader_board_state.set(LeaderBoardState::PostGame);
                    next_level_state.set(LevelState::MenuLeaderBoard);
                    next_camera_state.set(CameraOrbitEntityState::LeaderBoard);
                    for mut state in pan_orbit_camera_query.iter_mut() {
                        info!("{:?}", state);
                        state.radius = 38.0;
                        state.pitch = -12.0f32.to_radians();
                        state.yaw = -17.0f32.to_radians();
                    }
                },
            };
        },
        2..6 => {
            match map_set_state.get() {
                MapSetState::Tutorial => {
                    party.end_game(); // Sets players to NotInGame
                    next_game_state.set(GameState::PostGameReview);
                    next_turn.set(TurnState::Idle);
                    game_handler.init_postgame_leaderboard(party); // Set's target for level handling
                    next_leader_board_state.set(LeaderBoardState::PostGame);
                    next_level_state.set(LevelState::MenuLeaderBoard);
                    next_camera_state.set(CameraOrbitEntityState::LeaderBoard);
                    for mut state in pan_orbit_camera_query.iter_mut() {
                        info!("{:?}", state);
                        state.radius = 38.0;
                        state.pitch = -12.0f32.to_radians();
                        state.yaw = -17.0f32.to_radians();
                    }
                },
                MapSetState::WholeCorse => {
                    if current_level == 18 {
                        party.end_game(); // Sets players to NotInGame
                        next_game_state.set(GameState::PostGameReview);
                        next_turn.set(TurnState::Idle);
                        game_handler.init_postgame_leaderboard(party); // Set's target for level handling
                        next_leader_board_state.set(LeaderBoardState::PostGame);
                        next_level_state.set(LevelState::MenuLeaderBoard);
                        next_camera_state.set(CameraOrbitEntityState::LeaderBoard);
                        for mut state in pan_orbit_camera_query.iter_mut() {
                            info!("{:?}", state);
                            state.radius = 38.0;
                            state.pitch = -12.0f32.to_radians();
                            state.yaw = -17.0f32.to_radians();
                        }
                    } else {
                        if party.all_finished() {
                            party.next_round_prep();
                            party.next_set_order_player();
                            let set_next_level = level_handler.next_level(current_level);
                            next_level_state.set(set_next_level);
                            game_handler.next_level();
                            party.next_level();
                            next_turn.set(TurnState::Turn);
                            next_camera_state.set(CameraOrbitEntityState::Ball);
                            for mut state in pan_orbit_camera_query.iter_mut() {
                                state.radius = 2.0;
                                state.pitch = -8.0f32.to_radians();
                                state.yaw = 22.0f32.to_radians();
                            }
                        } else {
                            party.next_set_order_player();
                            next_turn.set(TurnState::TurnNext);
                            next_camera_state.set(CameraOrbitEntityState::Ball);
                            for mut state in pan_orbit_camera_query.iter_mut() {
                                state.radius = 2.0;
                                state.pitch = -8.0f32.to_radians();
                                state.yaw = 22.0f32.to_radians();
                            }
                        };
                    }
                },
                MapSetState::FrontNine => {
                    if current_level == 9 {
                        party.end_game(); // Sets players to NotInGame
                        next_game_state.set(GameState::PostGameReview);
                        next_turn.set(TurnState::Idle);
                        game_handler.init_postgame_leaderboard(party); // Set's target for level handling
                        next_leader_board_state.set(LeaderBoardState::PostGame);
                        next_level_state.set(LevelState::MenuLeaderBoard);
                        next_camera_state.set(CameraOrbitEntityState::LeaderBoard);
                        for mut state in pan_orbit_camera_query.iter_mut() {
                            info!("{:?}", state);
                            state.radius = 38.0;
                            state.pitch = -12.0f32.to_radians();
                            state.yaw = -17.0f32.to_radians();
                        }
                    } else {
                    }
                },
                MapSetState::BackNine => {
                    if current_level == 18 {
                        party.end_game(); // Sets players to NotInGame
                        next_game_state.set(GameState::PostGameReview);
                        next_turn.set(TurnState::Idle);
                        game_handler.init_postgame_leaderboard(party); // Set's target for level handling
                        next_leader_board_state.set(LeaderBoardState::PostGame);
                        next_level_state.set(LevelState::MenuLeaderBoard);
                        next_camera_state.set(CameraOrbitEntityState::LeaderBoard);
                        for mut state in pan_orbit_camera_query.iter_mut() {
                            info!("{:?}", state);
                            state.radius = 38.0;
                            state.pitch = -12.0f32.to_radians();
                            state.yaw = -17.0f32.to_radians();
                        }
                    } else {
                    }
                },
                MapSetState::SelectAHole => {
                    party.end_game(); // Sets players to NotInGame
                    next_game_state.set(GameState::PostGameReview);
                    next_turn.set(TurnState::Idle);
                    game_handler.init_postgame_leaderboard(party); // Set's target for level handling
                    next_leader_board_state.set(LeaderBoardState::PostGame);
                    next_level_state.set(LevelState::MenuLeaderBoard);
                    next_camera_state.set(CameraOrbitEntityState::LeaderBoard);
                    for mut state in pan_orbit_camera_query.iter_mut() {
                        info!("{:?}", state);
                        state.radius = 38.0;
                        state.pitch = -12.0f32.to_radians();
                        state.yaw = -17.0f32.to_radians();
                    }
                },
            };
        },
        _ => {},
    }    
}

use bevy_rapier3d::prelude::*;

pub fn turn_state_response_turn_reset(
    mut commands: Commands,
    mut scene_meshes: Query<(Entity, &Name, &mut Transform)>,
) {
    info!("Turn: Reset");
    for (entity, name, mut transform) in scene_meshes.iter_mut() {
        info!("Entity: {:?}", name);
        if name.as_str() == "ball" {
            let target_location: Vec3 = Vec3::new(0.0, 0.0, 0.0);  
            transform.translation = target_location;
            
            commands.entity(entity)
                .insert(Velocity {
                    linvel: Vec3::ZERO,
                    angvel: Vec3::ZERO,
                }
            );
        }
    }      
}

pub fn turn_state_response_turn_next(
    mut party: ResMut<Party>,
    mut golf_balls: Query<(Entity, &GolfBallTag)>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    glb_storage: Res<GLBStorageID>, //Arc<[MapID]> //map: Arc<str>,
    mut next_turn_state: ResMut<NextState<TurnState>>,
    // mut asset_event_writer: EventWriter<AssetEvent<Mesh>>,
) {
    // party.next_set_order_player();
    let active_player: usize = party.get_active_player().try_into().unwrap();
    info!("active_player: {:?}", active_player.clone());
    let mut golf_ball_exists = false;
    for (entity, tag) in golf_balls.iter_mut() {
        if tag.0 == active_player {
            golf_ball_exists = true;
        }
    }
    match golf_ball_exists {
        false => {
            gltf_handler_init_golf_ball_glb(asset_server, commands, glb_storage);
            next_turn_state.set(TurnState::Turn);
        },
        true => {
            next_turn_state.set(TurnState::Turn);
        },
    }
    info!("golf_ball_exists: {:?}", golf_ball_exists);
}

// fn turn_state_response_new_game() {}
// fn turn_state_response_game_complete() {}