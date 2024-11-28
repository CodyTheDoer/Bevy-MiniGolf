use bevy::prelude::*;

use bevy_mod_raycast::prelude::*;
// use bevy_rapier3d::prelude::*; 

// use std::collections::HashMap;

use crate::player_handler::leader_board_handler;
// --- State Imports --- //
use crate::{
    StateArrow, StateCameraOrbitEntity, StateGame, StateGameConnection, StateGamePlayStyle, StateLevel, StateMapSet, StateMenu, StateTurn
};

// --- resource Imports --- //
use crate::{
    CameraUi,
    CameraWorld, 
    Fonts,
    GameHandler,
    Interactable,
    LeaderBoard,
    Party,
    RunTrigger,
    StateText,
    TitleText,
};

impl Fonts {
    pub fn new() -> Self {
        let fonts: Vec<TextStyle> = Vec::new();
        Fonts {
            fonts,
        }
    }
}

// pub fn bonk_gizmo(
//     mut gizmos: Gizmos,
//     mut raycast: Raycast,
//     mut bonk: ResMut<BonkHandler>,
//     party_asleep: Res<Party>,
//     party: Res<Party>,
//     scene_meshes: Query<(&Name, &Transform)>,
//     windows: Query<&Window>,
//     camera_query: Query<&Transform, With<CameraWorld>>, // Query only for CameraWorld's Transform
//     rapier_context: Res<RapierContext>,
//     rigid_body_query: Query<(Entity, &RapierRigidBodyHandle)>,
//     scene_meshes_asleep: Query<(Entity, &Name)>,
// ) {
//     let arrow_color = if golf_ball_is_asleep() {
//         Color::srgb(0.0, 1.0, 0.0) // Color the arrow Green if the ball is sleeping
//     } else {
//         Color::srgb(1.0, 0.0, 0.0) // Color the arrow Green if the ball is actively moving
//     };

//     let Some(cursor_position) = windows.single().cursor_position() else {
//         return;
//     };
//     let camera = camera_query.get_single();
//     // Extract the yaw rotation around the y-axis from the camera's quaternion
//     let camera_yaw = camera.unwrap().rotation.to_euler(EulerRot::YXZ).0; // Theta in the rotation vec
//     for (name, transform) in scene_meshes.iter() {
//         let active_player: usize = party.get_active_player_index().try_into().unwrap();
//         if *name.as_str() == *format!("ball{}", active_player).as_str()  && transform.translation != Vec3::new(0.0, 0.0, 0.0) {
//             let ball_position = transform.translation;
            
//             // Calculate the direction from the ball to the intersection point.
//             let mut direction_x = bonk.cursor_origin_position.x - cursor_position.x;
//             let mut direction_y = bonk.cursor_origin_position.y - cursor_position.y;

//             let bonk_magnitude: f32 = 2.5;
//             let adjusted_xy = apply_rotation_matrix_camera_yaw(&camera_yaw, direction_x, direction_y);

//             // Localize arrow to a flat xz plane 
//             let direction_xyz: Vec3 = Vec3::new(adjusted_xy.x, 0.0, adjusted_xy.y).normalize() * (bonk_magnitude * bonk.power);
//             bonk.update_direction(&direction_xyz);

//             // Draw an arrow from the ball in the direction toward the cursor.
//             gizmos.arrow(
//                 ball_position,            // Start position of the arrow (at the ball)
//                 ball_position + direction_xyz, // End position, 12 units away from the cursor
//                 arrow_color.clone(),
//             );
//         }
//     } 
// }

// fn golf_ball_is_asleep() {
//     todo!();
// }

// fn apply_rotation_matrix_camera_yaw(
//     camera_yaw: &f32, // Query only for CameraWorld's Transform
//     direction_x: f32,
//     direction_y: f32,
// ) -> BonkMouseXY {
//     // 2D rotation matrix
//     let rotation_matrix = vec![
//         [camera_yaw.cos(), camera_yaw.sin()],
//         [-camera_yaw.sin(), camera_yaw.cos()],
//     ];

//     let rotated_x = rotation_matrix[0][0] * direction_x + rotation_matrix[0][1] * direction_y;
//     let rotated_y = rotation_matrix[1][0] * direction_x + rotation_matrix[1][1] * direction_y;

//     BonkMouseXY {
//         x: rotated_x,
//         y: rotated_y,
//     }
// }

pub fn draw_cursor(
    mut raycast: Raycast,
    camera_query: Query<(&Camera, &GlobalTransform), With<CameraWorld>>, // Only query for the CameraWorld    
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {    
    let (camera, camera_transform) = match camera_query.get_single() {
        Ok(result) => result,
        Err(_) => return, // Exit if the camera is not found or multiple cameras are detected
    };
    
    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    let hits = raycast.cast_ray(ray, &RaycastSettings::default());

    if let Some((_, intersection)) = hits.first() {
        // Get the intersection point.
        let point = intersection.position();

        // Draw a circle at the intersection point using Gizmos (just above the surface).
        let up = Dir3::Y; 
        gizmos.circle(point + up * 0.05, up, 0.05, Color::WHITE);
    }
}

pub fn ray_fire(
    mut raycast: Raycast,
    camera_query: Query<(&Camera, &GlobalTransform), With<CameraWorld>>, // Only query for the CameraWorld    
    interactable_query: Query<(Entity, &Name), With<Interactable>>,
    windows: Query<&Window>,
) {    
    let (camera, camera_transform) = match camera_query.get_single() {
        Ok(result) => result,
        Err(_) => {
            warn!("No CameraWorld found or multiple CameraWorlds detected.");
            return;
        },
    };

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    let hits = raycast.cast_ray(ray, &RaycastSettings::default());

    // Loop through the raycast hits and detect if we hit an interactable entity
    for (entity, _intersection) in hits {
        if Some(interactable_query.get(*entity)).is_some() {
        }
    }
}

pub fn ray_release(
    // mut party: ResMut<Party>,
    mut raycast: Raycast,
    camera_query: Query<(&Camera, &GlobalTransform), With<CameraWorld>>, // Only query for the CameraWorld    
    interactable_query: Query<Entity, With<Interactable>>,
    scene_meshes: Query<(Entity, &Name)>,
    windows: Query<&Window>,
    // map_set_state: Res<State<StateMapSet>>,
    // mut game_handler: ResMut<GameHandler>,
    // mut pan_orbit_camera_query: Query<&mut StatePanOrbit>,
) {    
    let (camera, camera_transform) = match camera_query.get_single() {
        Ok(result) => result,
        Err(_) => {
            warn!("No CameraWorld found or multiple CameraWorlds detected.");
            return;
        },
    };

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    let hits = raycast.cast_ray(ray, &RaycastSettings::default());

    // Loop through the raycast hits and detect if we hit an interactable entity
    for (entity, _intersection) in hits {
        if Some(interactable_query.get(*entity)).is_some() {
            for (target_entity, name) in scene_meshes.iter() {
                if *entity == target_entity {
                    info!("Name: {:?} Entity: {:?}", name, &entity);
                    info!("Entity Index: {}, Generation: {}", entity.index(), entity.generation());
                    let owned_name = name.as_str();
                    match owned_name {
                        // --- Menu: Main Interface Mapping --- //
                        "main_menu_interface_leaderboard" | "main_menu_interface_leaderboard_board.0" => {
                        },
                        "main_menu_interface_local" => {
                        },
                        "main_menu_interface_online" => {
                        },
                        "main_menu_interface_preferences" => {
                        },
                        "main_menu_interface_tutorial" => {
                        },
                        "main_menu_player_text" | "main_menu_player_board.0" => {
                        }
                        /* 
                            // Free Options to Build From
                        "main_menu_interface_minigolf" => {},
                        "main_menu_interface_sign_body" => {},
                        */

                        // --- Menu: Common Interactions --- //
                        "main_menu_text" | "main_menu_board.0" => {
                        },

                        // --- Menu: Leader Board Interface Mapping --- //
                        "leaderboard_menu_play_again_text" | "leaderboard_menu_play_again_board.0" => {
                        },
                        
                        // --- Menu: Local Interface Mapping --- //

                        "local_button_add_player" | "local_button_add_player_symbol" => {
                        },
                        "local_button_sub_player" | "local_button_sub_player_symbol" => {
                        },

                        "local_button_add_ai" | "local_button_add_ai_symbol" => {
                        },
                        "local_button_sub_ai" | "local_button_sub_ai_symbol" => {
                        },

                        // "local_playstyle_toggle_button_ordered.1" => {commands.insert_resource(NextState(PlayThroughStyleState::SetOrder))},
                        // "local_playstyle_toggle_button_proximity.1" => {commands.insert_resource(NextState(PlayThroughStyleState::Proximity))},

                        "map_set_whole_course_text" | "map_set_whole_course_board.0" => {
                        },
                        "map_set_front_nine_text" | "map_set_front_nine_board.0" => {
                        },
                        "map_set_back_nine_text" | "map_set_back_nine_board.0" => {
                        },
                        "map_set_select_a_hole_text" | "map_set_select_a_hole_board.0" => {
                        },
                        _ => {},
                    }
                };
            }
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
        font_size: 14.0,
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
                            "Minigolf State Refactor", 
                            fonts.fonts[0].clone())],
                        ..default()
                    },
                    ..default()
                },
                TitleText, // Tag the title text so it can be updated later
            ));
        });

    // HUD: Create a bottom-left UI node for the state information
    let bottom_left_ui = commands
        .spawn(NodeBundle {
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

    for _ in 0..25 {
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
                StateText, // Tag the state text to easily find and update it later
            ));
        });
    }

    // HUD: Create a bottom-right UI node for different state information
    let bottom_right_ui = commands
        .spawn(NodeBundle {
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

    for _ in 0..12 {
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
                StateText, // Tag the state text to easily find and update it later
            ));
        });
    }
}

pub fn update_ui(
    state_arrow: Res<State<StateArrow>>,
    state_camera: Res<State<StateCameraOrbitEntity>>,
    state_game: Res<State<StateGame>>,
    state_game_connection : Res<State<StateGameConnection>>,
    state_play_style: Res<State<StateGamePlayStyle>>,
    state_level: Res<State<StateLevel>>,
    state_map_set: Res<State<StateMapSet>>,
    state_menu: Res<State<StateMenu>>,
    state_turn: Res<State<StateTurn>>,
    party: Res<Party>,
    mut game_handler: ResMut<GameHandler>,
    leader_board: Res<LeaderBoard>,
    mut query: Query<&mut Text, With<StateText>>,
    run_trigger: Res<RunTrigger>,
) {
    let state_texts_left = vec![
        format!("state_arrow: {:?}", *state_arrow),                                                                                         // 1
        format!("state_camera: {:?}", *state_camera),                                                                                       // 2
        format!("state_game: {:?}", *state_game),                                                                                           // 3
        format!("state_game_connection: {:?}", *state_game_connection),                                                                     // 4
        format!("state_play_style: {:?}", *state_play_style),                                                                               // 5
        format!("state_level: {:?}", *state_level),                                                                                         // 6
        format!("state_map_set: {:?}", *state_map_set),                                                                                     // 7
        format!("state_menu: {:?}", *state_menu),                                                                                           // 8
        format!("state_turn: {:?}", *state_turn),                                                                                           // 9
        format!("Party Size: {:?}", party.get_party_size()),                                                                                // 10
        format!("Current Level: {:?}", game_handler.get_current_level()),                                                                   // 11
        format!("Active Player: {:?}", party.get_active_player_index()),                                                                          // 12 
        format!("Active Player: player_id: {:?}", party.active_player_get_player_id()),                                                     // 13
        format!("Active Player: player_type: {:?}", party.active_player_get_player_type()),                                                 // 14
        format!("Active Player: Ball Location: {:?}", game_handler.get_active_ball_location()),                                             // 15 
        format!("Active Player: Bonk Count Level: {:?}", party.active_player_get_bonks_level(game_handler.get_current_level() as usize)),   // 16
        format!("Active Player: hole_completion_state: {:?}", party.active_player_get_hole_completion_state()),                             // 17
        format!("Leader Board: Stored Game Records: {:?}", leader_board.get_game_count()),                                                  // 18
        format!("______________________________________________________________________"),                                                  // 19  
        format!("Num1: RemoveLastPlayer,   Num3: RemoveAi,"),                                                                               // 20
        format!("Num7: Add: PlayerLocal,   Num8: Add: PlayerRemote,   Num9: Add: PlayerAI"),                                                // 21
        format!("KeyB: party.active_player_add_bonk,   Space: toggle_state_game"),                                                          // 22    
        format!("KeyC: cycle_camera,   KeyM: cycle_state_map_set,   KeyP: cycle_active_player"),                                            // 23     
        format!("KeyA: active_player_set_ball_location,   KeyN: game_handler.next_turn"),                                                   // 24   
        format!("Keys: start_game_local, "),                                                                                                // 25   
    ];

    let state_texts_right = vec![        
        format!("active_player_add_bonk: {:?}", run_trigger.get("active_player_add_bonk")),                                                 // 1
        format!("active_player_set_ball_location: {:?}", run_trigger.get("active_player_set_ball_location")),                               // 2
        format!("cycle_active_player: {:?}", run_trigger.get("cycle_active_player")),                                                       // 3
        format!("cycle_camera: {:?}", run_trigger.get("cycle_camera")),                                                                     // 4
        format!("cycle_state_map_set: {:?}", run_trigger.get("cycle_state_map_set")),                                                       // 5
        format!("game_handler_get_active_ball_location: {:?}", run_trigger.get("game_handler_get_active_ball_location")),                   // 6
        format!("game_handler_reset_active_ball_location: {:?}", run_trigger.get("game_handler_reset_active_ball_location")),               // 7
        format!("game_handler_set_active_ball_location: {:?}", run_trigger.get("game_handler_set_active_ball_location")),                   // 8
        format!("set_hole_completion_state_true: {:?}", run_trigger.get("set_hole_completion_state_true")),                                 // 9
        format!("state_turn_next_player_turn: {:?}", run_trigger.get("state_turn_next_player_turn")),                                       // 10
        format!("start_game_local: {:?}", run_trigger.get("start_game_local")),                                                             // 11
        format!("toggle_state_game: {:?}", run_trigger.get("toggle_state_game")),                                                           // 12
    ];
    
    // Collect into a vector of mutable references
    let mut text_components: Vec<Mut<Text>> = query.iter_mut().collect();
    
    // Update first 21 entities (left column)
    for (i, state_text) in state_texts_left.iter().enumerate() {
        if i < text_components.len() {
            text_components[i].sections[0].value = state_text.clone();
        }
    }
    
    // Update last 7 entities (right column)
    for (i, state_text) in state_texts_right.iter().enumerate() {
        let right_index = i + state_texts_left.len();
        if right_index < text_components.len() {
            text_components[right_index].sections[0].value = state_text.clone();
        }
    }
}