use bevy::prelude::*;

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
    CameraUi,
    Fonts,
    GameHandler,
    GolfBall,
    LeaderBoard,
    Party,
    PurgeHandler,
    RunTrigger,
    TextState,
    TextTitle,
    // UiUpdateEvent,
    // UiUpdateTimer,
    UserInterface,
};

impl Fonts {
    pub fn new() -> Self {
        let fonts: Vec<TextStyle> = Vec::new();
        Fonts {
            fonts,
        }
    }
}

impl UserInterface {
    pub fn select_a_hole_widget() -> i32 {
        let target = 0;
        target
    }
}

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


pub fn setup_ui(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut fonts: ResMut<Fonts>,
) {
    // Load and setup fonts
    let font = asset_server.load("fonts/MatrixtypeDisplay-KVELZ.ttf");
    let matrix_display = TextStyle {
        font: font.clone(),
        font_size: 42.0,
        ..default()
    };
    let matrix_display_small = TextStyle {
        font: font.clone(),
        font_size: 10.0,
        ..default()
    };
    fonts.fonts.push(matrix_display);
    fonts.fonts.push(matrix_display_small);

    // Set up a 2D camera for the UI
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            camera: Camera {
                order: -1, // Render before the 3D scene
                ..default()
            },
            ..default()
        },
        CameraUi,
    ));

    // Title: Create a screen-sized UI node for the centered title
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                align_items: AlignItems::Center, // Align the title text to the center vertically
                justify_content: JustifyContent::Center, // Center the title text horizontally
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(10.0), // Height is 10% of the screen, to occupy the top area
                top: Val::Percent(0.0),     // Position it at the very top
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "Minigolf Dev Interface", 
                            fonts.fonts[0].clone())],
                        ..default()
                    },
                    ..default()
                },
                TextTitle, // Tag the title text so it can be updated later
            ));
        });

    // HUD: Create a bottom-left UI node for the state information
    let bottom_left_ui = commands
        .spawn(NodeBundle {
            background_color: BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.5)), // Semi-transparent dark background
            style: Style {
                display: Display::Flex,
                align_items: AlignItems::FlexStart, // Align items from the top of the node
                flex_direction: FlexDirection::Column, // Stack items vertically
                justify_content: JustifyContent::FlexStart, // Align from the start (top-left)
                position_type: PositionType::Absolute,
                bottom: Val::Percent(0.0),  // Position at the bottom of the screen
                left: Val::Percent(0.0),    // Align it to the left of the screen
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            ..default()
        }).id();

    for _ in 0..30 {
        commands.entity(bottom_left_ui).with_children(|parent| {
            // Spawn each state text entry and tag it for easy lookup later
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "Initializing...", // Placeholder text
                            fonts.fonts[1].clone(), // Using the smaller font style for HUD
                        )],
                        ..default()
                    },
                    style: Style {
                        position_type: PositionType::Relative,
                        margin: UiRect::vertical(Val::Px(5.0)), // Space between each state entry
                        ..default()
                    },
                    ..default()
                },
                TextState, // Tag the state text to easily find and update it later
            ));
        });
    }

    // HUD: Create a bottom-right UI node for different state information
    let bottom_right_ui = commands
        .spawn(NodeBundle {
            background_color: BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.5)), // Semi-transparent dark background
            style: Style {
                display: Display::Flex,
                align_items: AlignItems::FlexEnd, // Align items from the bottom of the node
                flex_direction: FlexDirection::Column, // Stack items vertically
                justify_content: JustifyContent::FlexEnd, // Align from the end (bottom-right)
                position_type: PositionType::Absolute,
                bottom: Val::Percent(0.0),  // Position at the bottom of the screen
                right: Val::Percent(0.0),   // Align it to the right of the screen
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            ..default()
        }).id();

    for _ in 0..39 {
        commands.entity(bottom_right_ui).with_children(|parent| {
            // Spawn each state text entry and tag it for easy lookup later
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "Initializing...", // Placeholder text
                            fonts.fonts[1].clone(), // Using the smaller font style for HUD
                        )],
                        ..default()
                    },
                    style: Style {
                        position_type: PositionType::Relative,
                        margin: UiRect::vertical(Val::Px(5.0)), // Space between each state entry
                        ..default()
                    },
                    ..default()
                },
                TextState, // Tag the state text to easily find and update it later
            ));
        });
    }
}

pub fn update_ui(
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
    mut query: Query<&mut Text, With<TextState>>,
    run_trigger: Res<RunTrigger>,
    purge_handler: Res<PurgeHandler>
) {
    let state_texts_left = vec![
        format!("state_arrow: {:?}", *state_arrow),                                                                                         // 1
        format!("state_camera: {:?}", *state_camera),                                                                                       // 2
        format!("state_camera_menu_target: {:?}", *state_camera_menu_target),                                                               // 3
        format!("state_game: {:?}", *state_game),                                                                                           // 4
        format!("state_engine_connection: {:?}", *state_engine_connection),                                                                 // 5
        format!("state_play_style: {:?}", *state_play_style),                                                                               // 6
        format!("state_level: {:?}", *state_level),                                                                                         // 7
        format!("state_map_set: {:?}", *state_map_set),                                                                                     // 8
        format!("state_menu: {:?}", *state_menu),                                                                                           // 9
        format!("state_turn: {:?}", *state_turn),                                                                                           // 10
        format!("Remote Game: {:?}", game_handler.get("remote_game")),                                                                      // 11
        format!("Current Level: {:?}", game_handler.current_level_get()),                                                                   // 12
        format!("Party Size: {:?}", party.party_size()),                                                                                    // 13
        format!("Active Player: {:?}", party.active_player_get_index()),                                                                    // 14 
        format!("Active Player: player_id: {:?}", party.active_player_get_player_id()),                                                     // 15
        format!("Active Player: player_type: {:?}", party.active_player_get_player_type()),                                                 // 16
        format!("Active Player: Bonk Count Level: {:?}", party.active_player_get_bonks_level(game_handler.current_level_get() as usize)),   // 17
        format!("Active Player: hole_completion_state: {:?}", party.active_player_get_hole_completion_state()),                             // 18
        format!("Leader Board: Stored Game Records: {:?}", leader_board.get_game_count()),                                                  // 19
        format!("Active Player Scorecard: {:?}", party.active_player_get_score()),                                                          // 20
        format!("______________________________________________________________________"),                                                  // 21  
        format!("Num1: RemoveLastPlayer,   Num3: RemoveAi,"),                                                                               // 22
        format!("Num7: Add: PlayerLocal,   Num8: Add: PlayerRemote,   Num9: Add: PlayerAI"),                                                // 23
        format!("KeyB: party.active_player_add_bonk,   Space: toggle_state_game"),                                                          // 24    
        format!("KeyC: cycle_camera,   KeyV: cycle_camera_menu_target,   KeyP: cycle_active_player"),                                            // 25     
        format!("KeyA: active_player_set_ball_location,   KeyN: game_handler.next_turn"),                                                   // 26   
        format!("Keys: start_game_local, KeyQ: AllStatesUpdate,   KeyM: cycle_state_map_set"),                                                                           // 27   
        format!("KeyU: golf_ball_query, KeyI: add_physics_query_and_update_scene"),                                                         // 28
        format!("KeyO: debug_names_query, KeyP: party_query"),                                                                              // 29
        format!("KeyY: last_game_record, Right Mouse: In-Game Bonk, Left mouse: Interact w/world"),                                         // 30
    ];

    let state_texts_right = vec![
        format!("game_handler: All Sleeping: [{:?}]", game_handler.get("all_sleeping")),
        format!("game_handler: Arrow State: [{:?}]", game_handler.get("arrow_state")),
        format!("game_handler: Environment Loaded: [{:?}]", game_handler.get("environment_loaded")),
        format!("game_handler: Golf Balls Loaded: [{:?}]", game_handler.get("golf_balls_loaded")),
        format!("game_handler: In Game: [{:?}]", game_handler.get("in_game")),
        format!("game_handler: Network Server Connection: [{:?}]", game_handler.get("network_server_connection")),
        format!("game_handler: Remote Game: [{:?}]", game_handler.get("remote_game")),
        format!("______________________________________________________________________"),
        format!("purge_handler: Environment Purged: [{:?}]", purge_handler.get("environment_purged")),
        format!("purge_handler: Golf Balls Purged: [{:?}]", purge_handler.get("golf_balls_purged")),
        format!("______________________________________________________________________"),
        format!("add_physics_query_and_update_scene: {:?}", run_trigger.get("add_physics_query_and_update_scene")),
        format!("camera_handler_cycle_state_camera: {:?}", run_trigger.get("camera_handler_cycle_state_camera")),
        format!("game_handler_game_start: {:?}", run_trigger.get("game_handler_game_start")),
        format!("game_handler_game_state_exit_routines: {:?}", run_trigger.get("game_handler_game_state_exit_routines")),
        format!("game_handler_game_state_start_routines: {:?}", run_trigger.get("game_handler_game_state_start_routines")),
        format!("golf_ball_handler_update_locations_post_bonk: {:?}", run_trigger.get("golf_ball_handler_update_locations_post_bonk")),
        format!("golf_ball_handler_party_store_locations: {:?}", run_trigger.get("golf_ball_handler_party_store_locations")),
        format!("golf_ball_handler_reset_golf_ball_locations: {:?}", run_trigger.get("golf_ball_handler_reset_golf_ball_locations")),
        format!("golf_ball_handler_spawn_golf_balls_for_party_members: {:?}", run_trigger.get("golf_ball_handler_spawn_golf_balls_for_party_members")),
        format!("leader_board_log_game: {:?}", run_trigger.get("leader_board_log_game")),
        format!("leader_board_review_last_game: {:?}", run_trigger.get("leader_board_review_last_game")),
        format!("level_handler_init_level_game_handler_current_level: {:?}", run_trigger.get("level_handler_init_level_game_handler_current_level")),
        format!("level_handler_next_turn_protocol: {:?}", run_trigger.get("level_handler_next_turn_protocol")),
        format!("level_handler_purge_protocol: {:?}", run_trigger.get("level_handler_purge_protocol")),
        format!("level_handler_set_state_next_level: {:?}", run_trigger.get("level_handler_set_state_next_level")),
        format!("level_handler_set_state_next_map_set: {:?}", run_trigger.get("level_handler_set_state_next_map_set")),
        format!("network_get_client_state_all: {:?}", run_trigger.get("network_get_client_state_all")),
        format!("network_get_client_state_game: {:?}", run_trigger.get("network_get_client_state_game")),
        format!("party_handler_active_player_add_bonk: {:?}", run_trigger.get("party_handler_active_player_add_bonk")),
        format!("party_handler_active_player_set_hole_completion_state_true: {:?}", run_trigger.get("party_handler_active_player_set_hole_completion_state_true")),
        format!("party_handler_cycle_active_player: {:?}", run_trigger.get("party_handler_cycle_active_player")),
        format!("party_handler_new_player_ai: {:?}", run_trigger.get("party_handler_new_player_ai")),
        format!("party_handler_new_player_local: {:?}", run_trigger.get("party_handler_new_player_local")),
        format!("party_handler_new_player_remote: {:?}", run_trigger.get("party_handler_new_player_remote")),
        format!("party_handler_remove_ai: {:?}", run_trigger.get("party_handler_remove_ai")),
        format!("party_handler_remove_last_player: {:?}", run_trigger.get("party_handler_remove_last_player")),
        format!("turn_handler_end_game: {:?}", run_trigger.get("turn_handler_end_game")),
        format!("turn_handler_next_round_prep: {:?}", run_trigger.get("turn_handler_next_round_prep")),
        format!("turn_handler_set_turn_next: {:?}", run_trigger.get("turn_handler_set_turn_next")),
    ];
    
    // Collect into a vector of mutable references
    let mut text_components: Vec<Mut<Text>> = query.iter_mut().collect();
    
    // Update left column
    for (i, state_text) in state_texts_left.iter().enumerate() {
        if i < text_components.len() {
            text_components[i].sections[0].value = state_text.clone();
        }
    }
    
    // Update right column
    for (i, state_text) in state_texts_right.iter().enumerate() {
        let right_index = i + state_texts_left.len();
        if right_index < text_components.len() {
            text_components[right_index].sections[0].value = state_text.clone();
        }
    }
}