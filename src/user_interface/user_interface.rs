use bevy::prelude::*;

use bevy_mod_raycast::prelude::*;
use bevy_rapier3d::prelude::*; 

use std::collections::HashMap;

// --- State Imports --- //
use crate::{
    CameraOrbitEntityState,
    GameState,
    LeaderBoardState,
    LevelState,
    MapSetState,
    MenuState,
    PanOrbitState,
    ConnectionState,
    PlayThroughStyleState,
    TurnState,
};

// --- resource Imports --- //
use crate::{
    BonkHandler,
    CameraUi, 
    CameraWorld, 
    Fonts, 
    GameHandler,
    Ground, 
    Interactable,
    Party,
    StateText,
    TitleText,
};

use crate::level_handler::physics_handler::{
    apply_rotation_matrix_camera_yaw,
    golf_ball_is_asleep,
};

pub fn bonk_gizmo(
    mut gizmos: Gizmos,
    mut raycast: Raycast,
    mut bonk: ResMut<BonkHandler>,
    party_asleep: Res<Party>,
    party: Res<Party>,
    scene_meshes: Query<(&Name, &Transform)>,
    windows: Query<&Window>,
    camera_query: Query<&Transform, With<CameraWorld>>, // Query only for CameraWorld's Transform
    rapier_context: Res<RapierContext>,
    rigid_body_query: Query<(Entity, &RapierRigidBodyHandle)>,
    scene_meshes_asleep: Query<(Entity, &Name)>,
) {
    let arrow_color = if golf_ball_is_asleep(rapier_context, rigid_body_query, scene_meshes_asleep, party_asleep) {
        Color::srgb(0.0, 1.0, 0.0) // Color the arrow Green if the ball is sleeping
    } else {
        Color::srgb(1.0, 0.0, 0.0) // Color the arrow Green if the ball is actively moving
    };

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };
    let camera = camera_query.get_single();
    // Extract the yaw rotation around the y-axis from the camera's quaternion
    let camera_yaw = camera.unwrap().rotation.to_euler(EulerRot::YXZ).0; // Theta in the rotation vec
    for (name, transform) in scene_meshes.iter() {
        let active_player: usize = party.get_active_player().try_into().unwrap();
        if *name.as_str() == *format!("ball{}", active_player).as_str()  && transform.translation != Vec3::new(0.0, 0.0, 0.0) {
            let ball_position = transform.translation;
            
            // Calculate the direction from the ball to the intersection point.
            let mut direction_x = bonk.cursor_origin_position.x - cursor_position.x;
            let mut direction_y = bonk.cursor_origin_position.y - cursor_position.y;

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
    mut party: ResMut<Party>,
    mut raycast: Raycast,
    camera_query: Query<(&Camera, &GlobalTransform), With<CameraWorld>>, // Only query for the CameraWorld    
    interactable_query: Query<Entity, With<Interactable>>,
    scene_meshes: Query<(Entity, &Name)>,
    windows: Query<&Window>,
    map_set_state: Res<State<MapSetState>>,
    mut game_handler: ResMut<GameHandler>,
    mut next_camera_state: ResMut<NextState<CameraOrbitEntityState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_leader_board_state: ResMut<NextState<LeaderBoardState>>,
    mut next_level_state: ResMut<NextState<LevelState>>,
    mut next_turn_state: ResMut<NextState<TurnState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut next_map_set_state: ResMut<NextState<MapSetState>>,
    mut pan_orbit_camera_query: Query<&mut PanOrbitState>,
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
                            next_menu_state.set(MenuState::LeaderBoard);
                        },
                        "main_menu_interface_local" => {
                            next_menu_state.set(MenuState::Local);
                        },
                        "main_menu_interface_online" => {
                            next_menu_state.set(MenuState::Online);
                        },
                        "main_menu_interface_preferences" => {
                            next_menu_state.set(MenuState::Preferences);
                        },
                        "main_menu_interface_tutorial" => {
                            next_menu_state.set(MenuState::Tutorial);
                        },
                        "main_menu_player_text" | "main_menu_player_board.0" => {
                            next_menu_state.set(MenuState::Player);
                        }
                        /* 
                            // Free Options to Build From
                        "main_menu_interface_minigolf" => {},
                        "main_menu_interface_sign_body" => {},
                        */

                        // --- Menu: Common Interactions --- //
                        "main_menu_text" | "main_menu_board.0" => {
                            game_handler.init_menu_main();
                            next_game_state.set(GameState::Menus);
                            next_level_state.set(LevelState::MainMenu);
                            next_camera_state.set(CameraOrbitEntityState::MainMenu);
                            for mut state in pan_orbit_camera_query.iter_mut() {
                                info!("{:?}", state);
                                state.radius = 38.0;
                                state.pitch = -12.0f32.to_radians();
                                state.yaw = -17.0f32.to_radians();
                            }
                        },

                        // --- Menu: Leader Board Interface Mapping --- //
                        "leaderboard_menu_play_again_text" | "leaderboard_menu_play_again_board.0" => {
                            party.start_game();
                            next_leader_board_state.set(LeaderBoardState::InGame);
                            match map_set_state.get() {
                                MapSetState::Tutorial => {
                                    game_handler.set_current_level(19);
                                    next_level_state.set(LevelState::HoleTutorial);
                                },
                                MapSetState::WholeCorse => {
                                    game_handler.set_current_level(1);
                                    next_level_state.set(LevelState::Hole1);
                                    party.set_starting_level(MapSetState::WholeCorse);
                                },
                                MapSetState::FrontNine => {
                                    game_handler.set_current_level(1);
                                    next_level_state.set(LevelState::Hole1);
                                    party.set_starting_level(MapSetState::FrontNine);
                                },
                                MapSetState::BackNine => {
                                    game_handler.set_current_level(10);
                                    next_level_state.set(LevelState::Hole10);
                                    party.set_starting_level(MapSetState::BackNine);
                                },
                                MapSetState::SelectAHole => {},
                            };
                            next_game_state.set(GameState::InGame);
                            party.start_game();
                            next_turn_state.set(TurnState::Turn);
                            next_camera_state.set(CameraOrbitEntityState::Ball);
                            for mut state in pan_orbit_camera_query.iter_mut() {
                                info!("{:?}", state);
                                state.radius = 2.0;
                                state.pitch = -8.0f32.to_radians();
                                state.yaw = 22.0f32.to_radians();
                            }
                        },
                        
                        // --- Menu: Local Interface Mapping --- //

                        "local_button_add_player" | "local_button_add_player_symbol" => {party.add_player()},
                        "local_button_sub_player" | "local_button_sub_player_symbol" => {party.remove_player()},

                        "local_button_add_ai" | "local_button_add_ai_symbol" => {party.add_ai()},
                        "local_button_sub_ai" | "local_button_sub_ai_symbol" => {party.remove_ai()},

                        // "local_playstyle_toggle_button_ordered.1" => {commands.insert_resource(NextState(PlayThroughStyleState::SetOrder))},
                        // "local_playstyle_toggle_button_proximity.1" => {commands.insert_resource(NextState(PlayThroughStyleState::Proximity))},

                        "map_set_whole_course_text" | "map_set_whole_course_board.0" => {
                            next_map_set_state.set(MapSetState::WholeCorse);
                            party.set_starting_level(MapSetState::WholeCorse);
                            next_game_state.set(GameState::InGame);
                            next_menu_state.set(MenuState::NoSelection);
                            next_turn_state.set(TurnState::Turn);
                            game_handler.set_current_level(1);
                            next_level_state.set(LevelState::Hole1);
                            next_camera_state.set(CameraOrbitEntityState::Ball);
                        },
                        "map_set_front_nine_text" | "map_set_front_nine_board.0" => {
                            next_map_set_state.set(MapSetState::FrontNine);
                            party.set_starting_level(MapSetState::FrontNine);
                            next_game_state.set(GameState::InGame);
                            next_menu_state.set(MenuState::NoSelection);
                            next_turn_state.set(TurnState::Turn);
                            game_handler.set_current_level(1);
                            next_level_state.set(LevelState::Hole1);
                            next_camera_state.set(CameraOrbitEntityState::Ball);
                        },
                        "map_set_back_nine_text" | "map_set_back_nine_board.0" => {
                            next_map_set_state.set(MapSetState::BackNine);
                            party.set_starting_level(MapSetState::BackNine);
                            next_game_state.set(GameState::InGame);
                            next_menu_state.set(MenuState::NoSelection);
                            next_turn_state.set(TurnState::Turn);
                            game_handler.set_current_level(10);
                            next_level_state.set(LevelState::Hole10);
                            next_camera_state.set(CameraOrbitEntityState::Ball);
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
                            "Minigolf", 
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

    for _ in 0..13 {
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
}

pub fn update_ui(
    state_game: Res<State<GameState>>,
    state_menu: Res<State<MenuState>>,
    state_turn: Res<State<TurnState>>,
    state_map_set: Res<State<MapSetState>>,
    state_play_through_style: Res<State<PlayThroughStyleState>>,
    state_leader_board: Res<State<LeaderBoardState>>,
    state_level: Res<State<LevelState>>,
    state_party_connection: Res<State<ConnectionState>>,
    state_pan_orbit_camera: Res<State<CameraOrbitEntityState>>,
    party: Res<Party>,
    mut query: Query<&mut Text, With<StateText>>,
) {
    let state_texts = vec![
        format!("Server Connection: {:?}", *state_party_connection),
        format!("Game: {:?}", *state_game),
        format!("Menu: {:?}", *state_menu),
        format!("Turn: {:?}", *state_turn),
        format!("Map Set: {:?}", *state_map_set),
        format!("Play Through Style: {:?}", *state_play_through_style),
        format!("Leader Board: {:?}", *state_leader_board),
        format!("Level: {:?}", *state_level),
        format!("Camera Orbit Entity: {:?}", *state_pan_orbit_camera),
        format!("Party Size: {:?}", party.get_party_size()),
        format!("Party Size w/AI: {:?}", party.get_party_size_w_ai()),
        format!("Active Level: {:?}", party.get_active_level()),
        format!("Active Player: {:?}", party.get_active_player()),
    ];
    // Update the text for the state information
    for (mut text, state_text) in query.iter_mut().zip(state_texts.iter()) {
        text.sections[0].value = state_text.clone();
    }
}