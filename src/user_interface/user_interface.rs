use bevy::prelude::*;

use bevy_easy_vec_ui::EasyVecUi;

// --- State Imports --- //
use crate::{
    StateArrow,
    StateCameraMenuTarget,
    StateCameraOrbitEntity, 
    StateEngineConnection, 
    StateGame, 
    StateGamePlayStyle, 
    StateLevel, 
    StateMapSet, 
    StateMenu, 
    StateTurn,
};

// --- resource Imports --- //
use crate::{
    BonkHandler,
    XYMatrix,
    CameraWorld,
    GameHandler,
    GolfBall,
    LeaderBoard,
    Party,
    PurgeHandler,
    RunTrigger,
    UserInterface,
};

pub fn apply_rotation_matrix_camera_yaw(
    camera_yaw: &f32, // Query only for CameraWorld's Transform
    direction_x: f32,
    direction_y: f32,
) -> XYMatrix {
    // 2D rotation matrix
    let rotation_matrix = vec![
        [camera_yaw.cos(), camera_yaw.sin()],
        [-camera_yaw.sin(), camera_yaw.cos()],
    ];

    let rotated_x = rotation_matrix[0][0] * direction_x + rotation_matrix[0][1] * direction_y;
    let rotated_y = rotation_matrix[1][0] * direction_x + rotation_matrix[1][1] * direction_y;

    XYMatrix {
        x: rotated_x,
        y: rotated_y,
    }
}

pub fn bonk_gizmo(
    mut gizmos: Gizmos,
    mut bonk: ResMut<BonkHandler>,
    party: Res<Party>,
    golf_balls: Query<(&Transform, &mut GolfBall)>,
    windows: Query<&Window>,
    camera_query: Query<&Transform, With<CameraWorld>>, // Query only for CameraWorld's Transform
    game_handler: Res<GameHandler>,
) {
    let arrow_color = { // Color the arrow Green/Blue if the ball is sleeping
        if game_handler.get("all_sleeping") {
            Color::srgb(0.0, 0.0, 1.0)
        } else { // Color the arrow Red if the ball is actively moving
            Color::srgb(1.0, 0.0, 0.0)
        }
    };
    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };
    let camera = camera_query.get_single();
    // Extract the yaw rotation around the y-axis from the camera's quaternion
    let camera_yaw = camera.unwrap().rotation.to_euler(EulerRot::YXZ).0; // Theta in the rotation vec
    for (transform, golf_ball) in golf_balls.iter() {
        if golf_ball.0.uuid == party.active_player_get_player_id() { // && transform.translation != Vec3::new(0.0, 0.0, 0.0) {
           let ball_position = transform.translation;
            
            // Calculate the direction from the ball to the intersection point.
            let direction_x = bonk.cursor_origin_position.x - cursor_position.x;
            let direction_y = bonk.cursor_origin_position.y - cursor_position.y;

            let bonk_magnitude: f32 = 2.5;
            let adjusted_xy = apply_rotation_matrix_camera_yaw(&camera_yaw, direction_x, direction_y);

            // Localize arrow to a flat xz plane 
            let direction_xyz: Vec3 = Vec3::new(adjusted_xy.x, 0.0, adjusted_xy.y).normalize() * (bonk_magnitude * bonk.power);
            bonk.update_direction(&direction_xyz);

            // Draw an arrow from the ball in the direction toward the cursor.
            gizmos.arrow(
                ball_position,            // Start position of the arrow (at the ball)
                ball_position + direction_xyz, // End position, 12 units away from the cursor
                arrow_color.clone(),
            );
        }
    }
} 

pub fn easy_vec_ui(
    mut easy_vec_ui_resource: ResMut<EasyVecUi>,
    state_arrow: Res<State<StateArrow>>,
    state_camera_menu_target: Res<State<StateCameraMenuTarget>>,
    state_camera: Res<State<StateCameraOrbitEntity>>,
    state_game: Res<State<StateGame>>,
    state_engine_connection : Res<State<StateEngineConnection>>,
    state_play_style: Res<State<StateGamePlayStyle>>,
    state_level: Res<State<StateLevel>>,
    state_map_set: Res<State<StateMapSet>>,
    state_menu: Res<State<StateMenu>>,
    state_turn: Res<State<StateTurn>>,
    party: Res<Party>,
    game_handler: ResMut<GameHandler>,
    leader_board: Res<LeaderBoard>,
    run_trigger: Res<RunTrigger>,
    purge_handler: Res<PurgeHandler>
) {
    let left_data_vec = vec![
        String::from(format!("state_arrow: {:?}", *state_arrow)),                                                                                         // 1
        String::from(format!("state_camera: {:?}", *state_camera)),                                                                                       // 2
        String::from(format!("state_camera_menu_target: {:?}", *state_camera_menu_target)),                                                               // 3
        String::from(format!("state_game: {:?}", *state_game)),                                                                                           // 4
        String::from(format!("state_engine_connection: {:?}", *state_engine_connection)),                                                                 // 5
        String::from(format!("state_play_style: {:?}", *state_play_style)),                                                                               // 6
        String::from(format!("state_level: {:?}", *state_level)),                                                                                         // 7
        String::from(format!("state_map_set: {:?}", *state_map_set)),                                                                                     // 8
        String::from(format!("state_menu: {:?}", *state_menu)),                                                                                           // 9
        String::from(format!("state_turn: {:?}", *state_turn)),                                                                                           // 10
        String::from(format!("Remote Game: {:?}", game_handler.get("remote_game"))),                                                                      // 11
        String::from(format!("Current Level: {:?}", game_handler.current_level_get())),                                                                   // 12
        String::from(format!("Party Size: {:?}", party.party_size())),                                                                                    // 13
        String::from(format!("Active Player: {:?}", party.active_player_get_index())),                                                                    // 14 
        String::from(format!("Active Player: player_id: {:?}", party.active_player_get_player_id())),                                                     // 15
        String::from(format!("Active Player: player_type: {:?}", party.active_player_get_player_type())),                                                 // 16
        String::from(format!("Active Player: Bonk Count Level: {:?}", party.active_player_get_bonks_level(game_handler.current_level_get() as usize))),   // 17
        String::from(format!("Active Player: hole_completion_state: {:?}", party.active_player_get_hole_completion_state())),                             // 18
        String::from(format!("Leader Board: Stored Game Records: {:?}", leader_board.get_game_count())),                                                  // 19
        String::from(format!("Active Player Scorecard: {:?}", party.active_player_get_score())),                                                          // 20
        String::from(format!("______________________________________________________________________")),                                                  // 21  
        String::from(format!("Num1: RemoveLastPlayer,   Num3: RemoveAi,")),                                                                               // 22
        String::from(format!("Num7: Add: PlayerLocal,   Num8: Add: PlayerRemote,   Num9: Add: PlayerAI")),                                                // 23
        String::from(format!("KeyB: party.active_player_add_bonk,   Space: toggle_state_game")),                                                          // 24    
        String::from(format!("KeyC: cycle_camera,   KeyV: cycle_camera_menu_target,   KeyP: cycle_active_player")),                                            // 25     
        String::from(format!("KeyA: active_player_set_ball_location,   KeyN: game_handler.next_turn")),                                                   // 26   
        String::from(format!("Keys: start_game_local, KeyQ: AllStatesUpdate,   KeyM: cycle_state_map_set")),                                                                           // 27   
        String::from(format!("KeyU: golf_ball_query, KeyI: add_physics_query_and_update_scene")),                                                         // 28
        String::from(format!("KeyO: debug_names_query, KeyP: party_query")),                                                                              // 29
        String::from(format!("KeyY: last_game_record, Right Mouse: In-Game Bonk, Left mouse: Interact w/world")),                                         // 30
    ];

    let right_data_vec = vec![
        String::from(format!("game_handler: All Sleeping: [{:?}]", game_handler.get("all_sleeping"))),
        String::from(format!("game_handler: Arrow State: [{:?}]", game_handler.get("arrow_state"))),
        String::from(format!("game_handler: Environment Loaded: [{:?}]", game_handler.get("environment_loaded"))),
        String::from(format!("game_handler: Golf Balls Loaded: [{:?}]", game_handler.get("golf_balls_loaded"))),
        String::from(format!("game_handler: In Game: [{:?}]", game_handler.get("in_game"))),
        String::from(format!("game_handler: Network Server Connection: [{:?}]", game_handler.get("network_server_connection"))),
        String::from(format!("game_handler: Remote Game: [{:?}]", game_handler.get("remote_game"))),
        String::from(format!("______________________________________________________________________")),
        String::from(format!("purge_handler: Environment Purged: [{:?}]", purge_handler.get("environment_purged"))),
        String::from(format!("purge_handler: Golf Balls Purged: [{:?}]", purge_handler.get("golf_balls_purged"))),
        String::from(format!("______________________________________________________________________")),
        String::from(format!("add_physics_query_and_update_scene: {:?}", run_trigger.get("add_physics_query_and_update_scene"))),
        String::from(format!("camera_handler_cycle_state_camera: {:?}", run_trigger.get("camera_handler_cycle_state_camera"))),
        String::from(format!("game_handler_game_start: {:?}", run_trigger.get("game_handler_game_start"))),
        String::from(format!("game_handler_game_state_exit_routines: {:?}", run_trigger.get("game_handler_game_state_exit_routines"))),
        String::from(format!("game_handler_game_state_start_routines: {:?}", run_trigger.get("game_handler_game_state_start_routines"))),
        String::from(format!("golf_ball_handler_update_locations_post_bonk: {:?}", run_trigger.get("golf_ball_handler_update_locations_post_bonk"))),
        String::from(format!("golf_ball_handler_party_store_locations: {:?}", run_trigger.get("golf_ball_handler_party_store_locations"))),
        String::from(format!("golf_ball_handler_reset_golf_ball_locations: {:?}", run_trigger.get("golf_ball_handler_reset_golf_ball_locations"))),
        String::from(format!("golf_ball_handler_spawn_golf_balls_for_party_members: {:?}", run_trigger.get("golf_ball_handler_spawn_golf_balls_for_party_members"))),
        String::from(format!("leader_board_log_game: {:?}", run_trigger.get("leader_board_log_game"))),
        String::from(format!("leader_board_review_last_game: {:?}", run_trigger.get("leader_board_review_last_game"))),
        String::from(format!("level_handler_init_level_game_handler_current_level: {:?}", run_trigger.get("level_handler_init_level_game_handler_current_level"))),
        String::from(format!("level_handler_next_turn_protocol: {:?}", run_trigger.get("level_handler_next_turn_protocol"))),
        String::from(format!("level_handler_purge_protocol: {:?}", run_trigger.get("level_handler_purge_protocol"))),
        String::from(format!("level_handler_set_state_next_level: {:?}", run_trigger.get("level_handler_set_state_next_level"))),
        String::from(format!("level_handler_set_state_next_map_set: {:?}", run_trigger.get("level_handler_set_state_next_map_set"))),
        String::from(format!("network_get_client_state_all: {:?}", run_trigger.get("network_get_client_state_all"))),
        String::from(format!("network_get_client_state_game: {:?}", run_trigger.get("network_get_client_state_game"))),
        String::from(format!("party_handler_active_player_add_bonk: {:?}", run_trigger.get("party_handler_active_player_add_bonk"))),
        String::from(format!("party_handler_active_player_set_hole_completion_state_true: {:?}", run_trigger.get("party_handler_active_player_set_hole_completion_state_true"))),
        String::from(format!("party_handler_cycle_active_player: {:?}", run_trigger.get("party_handler_cycle_active_player"))),
        String::from(format!("party_handler_new_player_ai: {:?}", run_trigger.get("party_handler_new_player_ai"))),
        String::from(format!("party_handler_new_player_local: {:?}", run_trigger.get("party_handler_new_player_local"))),
        String::from(format!("party_handler_new_player_remote: {:?}", run_trigger.get("party_handler_new_player_remote"))),
        String::from(format!("party_handler_remove_ai: {:?}", run_trigger.get("party_handler_remove_ai"))),
        String::from(format!("party_handler_remove_last_player: {:?}", run_trigger.get("party_handler_remove_last_player"))),
        String::from(format!("turn_handler_end_game: {:?}", run_trigger.get("turn_handler_end_game"))),
        String::from(format!("turn_handler_next_round_prep: {:?}", run_trigger.get("turn_handler_next_round_prep"))),
        String::from(format!("turn_handler_set_turn_next: {:?}", run_trigger.get("turn_handler_set_turn_next"))),
    ];
    
    easy_vec_ui_resource.inject_vec_left(left_data_vec);
    easy_vec_ui_resource.inject_vec_right(right_data_vec);
}

impl UserInterface {
    pub fn select_a_hole_widget() -> i32 {
        let target = 0;
        target
    }
}