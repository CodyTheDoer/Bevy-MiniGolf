use bevy::prelude::*;

use bevy_mod_raycast::prelude::*;

// --- resource Imports --- //
use crate::{
    CameraWorld, 
    GameHandler, 
    Interactable, 
    Party,
    RunTrigger, 
    StatePanOrbit,
};

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
        gizmos.circle(point + up * 0.0125, up, 0.05, Color::WHITE);
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
    mut raycast: Raycast,
    mut game_handler: ResMut<GameHandler>,
    mut run_trigger: ResMut<RunTrigger>,
    mut pan_orbit_camera_query: Query<&mut StatePanOrbit>,
    camera_query: Query<(&Camera, &GlobalTransform), With<CameraWorld>>, // Only query for the CameraWorld    
    interactable_query: Query<Entity, With<Interactable>>,
    scene_meshes: Query<(Entity, &Name)>,
    windows: Query<&Window>,
    party: Res<Party>,
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
                    let mut menu_camera_adj_left = false;
                    let mut menu_camera_adj_right = false;
                    info!("Name: {:?} Entity: {:?}", name, &entity);
                    info!("Entity Index: {}, Generation: {}", entity.index(), entity.generation());
                    let owned_name = name.as_str();
                    match owned_name {
                        // --- Menu: Main Interface Mapping --- //
                        "main_menu_interface_tutorial" => {
                            run_trigger.set_target("game_handler_start_tutorial", true);
                        },
                        "main_menu_interface_leaderboard" | "main_menu_interface_leaderboard_board.0" => {
                            game_handler.current_level_set_menu_learderboard();
                            run_trigger.set_target("level_handler_init_level_game_handler_current_level", true);
                            menu_camera_adj_left = true;
                        },
                        "main_menu_interface_local" => {
                            run_trigger.set_target("level_handler_purge_protocol", true);
                            game_handler.current_level_set_menu_local();
                            run_trigger.set_target("level_handler_init_level_game_handler_current_level", true);
                            menu_camera_adj_right = true;
                        },
                        "main_menu_interface_online" => {
                            game_handler.current_level_set_menu_online();
                            run_trigger.set_target("level_handler_init_level_game_handler_current_level", true);
                            menu_camera_adj_right = true;
                        },
                        "main_menu_interface_preferences" => {
                            game_handler.current_level_set_menu_preferences();
                            run_trigger.set_target("level_handler_init_level_game_handler_current_level", true);
                            menu_camera_adj_left = true;
                        },
                        "main_menu_player_text" | "main_menu_player_board.0" => {
                            game_handler.current_level_set_menu_player();
                            run_trigger.set_target("level_handler_init_level_game_handler_current_level", true);
                            menu_camera_adj_left = true;
                        }
                        /* 
                            // Free Options to Build From
                        "main_menu_interface_minigolf" => {},
                        "main_menu_interface_sign_body" => {},
                        */

                        // --- Menu: Common Interactions --- //
                        "main_menu_text" | "main_menu_board.0" => {
                            game_handler.current_level_set(0);
                            run_trigger.set_target("level_handler_init_level_game_handler_current_level", true);
                            menu_camera_adj_left = true;
                        },

                        // --- Menu: Leader Board Interface Mapping --- //
                        "leaderboard_menu_play_again_text" | "leaderboard_menu_play_again_board.0" => {
                        },
                        
                        // --- Menu: Local Interface Mapping --- //

                        "local_button_add_player" | "local_button_add_player_symbol" => {
                            run_trigger.set_target("party_handler_new_player_local", true);
                        },
                        "button_add_player" => {
                            if party.get_count_local() < 5 {
                                run_trigger.set_target("party_handler_new_player_local", true);
                            };
                        },
                        "local_button_sub_player" | "local_button_sub_player_symbol" | "button_sub_player" => {
                            run_trigger.set_target("party_handler_remove_local_player", true);
                        },

                        "local_button_add_ai" | "local_button_add_ai_symbol" => {
                            run_trigger.set_target("party_handler_new_player_ai", true);
                        },
                        "local_button_sub_ai" | "local_button_sub_ai_symbol" => {
                            run_trigger.set_target("party_handler_remove_ai", true);
                        },

                        // "local_playstyle_toggle_button_ordered.1" => {commands.insert_resource(NextState(PlayThroughStyleState::SetOrder))},
                        // "local_playstyle_toggle_button_proximity.1" => {commands.insert_resource(NextState(PlayThroughStyleState::Proximity))},

                        "map_set_whole_course_text" | "map_set_whole_course_board.0" => {
                            run_trigger.set_target("game_handler_start_local_whole_corse", true);
                        },
                        "map_set_front_nine_text" | "map_set_front_nine_board.0" => {
                            run_trigger.set_target("game_handler_start_local_front_nine", true);
                        },
                        "map_set_back_nine_text" | "map_set_back_nine_text_board" | "map_set_back_nine_board.0" => {
                            run_trigger.set_target("game_handler_start_local_back_nine", true);
                        },
                        "map_set_select_a_hole_text" | "map_set_select_a_hole_board.0" => {
                            run_trigger.set_target("game_handler_start_local_select_a_hole", true);
                        },

                        "player_name_name_input_block" => {

                        }
                        _ => {},
                    }
                    if menu_camera_adj_left == true {
                        for mut state in pan_orbit_camera_query.iter_mut() {
                            info!("{:?}", state);
                            state.radius = 38.0;
                            state.pitch = -12.0f32.to_radians();
                            state.yaw = -17.0f32.to_radians();
                        }
                    }
                    if menu_camera_adj_right == true {
                        for mut state in pan_orbit_camera_query.iter_mut() {
                            info!("{:?}", state);
                            state.radius = 38.0;
                            state.pitch = -12.0f32.to_radians();
                            state.yaw = 17.0f32.to_radians();
                        }
                    }
                };
            }
        }
    }
}
