use bevy::prelude::*;

use bevy_mod_raycast::prelude::*;
use bevy_rapier3d::prelude::*; 

use std::collections::HashMap;

use crate::{
    BonkHandler,
    CameraOrbitEntityState,
    CameraUi, 
    CameraWorld, 
    Fonts, 
    GameHandler,
    GameState,
    Ground, 
    LevelState,
    Interactable,
    MenuState,
    PanOrbitState,
};

use crate::level_handler::physics_handler::{
    apply_rotation_matrix_camera_yaw,
    golf_ball_is_asleep,
};

pub fn bonk_gizmo(
    mut gizmos: Gizmos,
    mut raycast: Raycast,
    mut bonk: ResMut<BonkHandler>,
    scene_meshes: Query<(&Name, &Transform)>,
    windows: Query<&Window>,
    camera_query: Query<&Transform, With<CameraWorld>>, // Query only for CameraWorld's Transform
    rapier_context: Res<RapierContext>,
    rigid_body_query: Query<(Entity, &RapierRigidBodyHandle)>,
    scene_meshes_asleep: Query<(Entity, &Name)>,
) {
    let arrow_color = if golf_ball_is_asleep(rapier_context, rigid_body_query, scene_meshes_asleep) {
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
        if name.as_str() == "ball" && transform.translation != Vec3::new(0.0, 0.0, 0.0) {
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

pub fn game_state_update(
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    match game_state.get() {
        GameState::LoadingScreen => {
            info!("GameState::MenuMain");
            next_game_state.set(GameState::Menus);
        },
        GameState::Menus => {
            info!("GameState::InGame");
            next_game_state.set(GameState::InGame);
        },
        GameState::GameInitLocal => {
        },
        GameState::GameInitOnline => {
        },
        GameState::InGame => {
            info!("GameState::InGamePaused");
            next_game_state.set(GameState::InGamePaused);
        },
        GameState::InGamePaused => {
            info!("GameState::PostGameReview");
            next_game_state.set(GameState::PostGameReview);
        },
        GameState::PostGameReview => {
            info!("GameState::LoadingScreen");
            next_game_state.set(GameState::LoadingScreen);
        },
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
    camera_query: Query<(&Camera, &GlobalTransform), With<CameraWorld>>, // Only query for the CameraWorld    
    interactable_query: Query<Entity, With<Interactable>>,
    scene_meshes: Query<(Entity, &Name)>,
    windows: Query<&Window>,
    mut game_handler: ResMut<GameHandler>,
    mut next_camera_state: ResMut<NextState<CameraOrbitEntityState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_level: ResMut<NextState<LevelState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
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
                    info!("Name: {:?}", name);
                    let owned_name = name.as_str();
                    match owned_name {
                        // --- Main Menu Interface Mapping --- //
                        "main_menu_interface_leaderboard" => {
                            next_menu_state.set(MenuState::LeaderBoard);
                        },
                        "main_menu_interface_leaderboard_board.0" => {
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
                        /* 
                            // Free Options to Build From
                        "main_menu_interface_minigolf" => {},
                        "main_menu_interface_sign_body" => {},
                        */

                        // --- Leader Board Menu Interface Mapping --- //
                        "leaderboard_menu_play_again_text" => {
                            todo!();
                        },
                        "leaderboard_menu_play_again_board.0" => {
                            todo!();
                        },
                        "leaderboard_menu_main_menu_text" => {
                            game_handler.init_main_menu();
                            next_game_state.set(GameState::Menus);
                            next_level.set(LevelState::MainMenu);
                            next_camera_state.set(CameraOrbitEntityState::MainMenu);
                            for mut state in pan_orbit_camera_query.iter_mut() {
                                info!("{:?}", state);
                                state.radius = 38.0;
                                state.pitch = -12.0f32.to_radians();
                                state.yaw = -17.0f32.to_radians();
                            }
                        },
                        "leaderboard_menu_main_menu_board.0" => {
                            game_handler.init_main_menu();
                            next_game_state.set(GameState::Menus);
                            next_level.set(LevelState::MainMenu);
                            next_camera_state.set(CameraOrbitEntityState::MainMenu);
                            for mut state in pan_orbit_camera_query.iter_mut() {
                                info!("{:?}", state);
                                state.radius = 38.0;
                                state.pitch = -12.0f32.to_radians();
                                state.yaw = -17.0f32.to_radians();
                            }
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
    let font = asset_server.load("fonts/MatrixtypeDisplay-KVELZ.ttf");
    let matrix_display = TextStyle {
        font: font.clone(),
        font_size: 42.0,
        ..default()
    };
    let matrix_display_small = TextStyle {
        font: font.clone(),
        font_size: 22.0,
        ..default()
    };
    fonts.fonts.push(matrix_display);
    fonts.fonts.push(matrix_display_small);

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

    // Create a screen-sized UI node as a container
    commands.spawn(NodeBundle {
        style: Style {
            display: Display::Flex,
            align_items: AlignItems::Center,    // Center vertically within the container
            justify_content: JustifyContent::Center, // Center horizontally within the container
            position_type: PositionType::Absolute,
            // Set this node to occupy the entire screen
            width: Val::Percent(100.0),
            height: Val::Percent(100.0), 
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text {
                sections: vec![TextSection::new(
                    "Mini Golf",
                    fonts.fonts[0].clone(),
                )],
                ..default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Percent(2.0), 
                ..default()
            },
            ..default()
        });
    });
}