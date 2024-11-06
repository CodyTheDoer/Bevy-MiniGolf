use bevy::prelude::*;

use bevy_mod_raycast::prelude::*;

use std::collections::HashMap;

use crate::{CameraWorld, Fonts, Ground, InfoCall, Interactable, InteractableEntities, OpIndex};

impl InfoCall {
    pub fn from_index(
        index: u32,
    ) -> Option<InfoCall> {
        let mut info_call_map = HashMap::new();
        info_call_map.insert(0, InfoCall::Call0);
        info_call_map.insert(1, InfoCall::Call1);
        info_call_map.insert(2, InfoCall::Call2);
        info_call_map.insert(3, InfoCall::Call3);
        info_call_map.insert(4, InfoCall::Call4);
        info_call_map.insert(5, InfoCall::Call5);
        info_call_map.insert(6, InfoCall::Call6);

        info_call_map.get(&index).cloned()
    }
}

pub fn setup_ui(
    asset_server: Res<AssetServer>,
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
}

pub fn draw_cursor(
    mut raycast: Raycast,
    camera_query: Query<(&Camera, &GlobalTransform), With<CameraWorld>>, // Only query for the CameraWorld    
    ground_query: Query<&GlobalTransform, With<Ground>>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
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

    if let Some((_, intersection)) = hits.first() {
        // Get the intersection point.
        let point = intersection.position();

        // Draw a circle at the intersection point using Gizmos (just above the surface).
        let up = Dir3::Y; 
        gizmos.circle(point + up * 0.05, up, 0.2, Color::WHITE);
    } else {
        let ground = ground_query.single();
        let Some(distance) =
            ray.intersect_plane(ground.translation(), InfinitePlane3d::new(ground.up()))
        else {
            return;
        };

        let point = ray.get_point(distance);

        // Draw a circle just above the ground plane at that position.
        gizmos.circle(point + ground.up() * 0.01, ground.up(), 0.2, Color::WHITE);
    }
}

pub fn fire_ray(
    mut raycast: Raycast,
    op_index: Res<OpIndex>,
    camera_query: Query<(&Camera, &GlobalTransform), With<CameraWorld>>, // Only query for the CameraWorld    
    interactable_query: Query<Entity, With<Interactable>>,
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
            let entity_index = entity.index();
            if let Some(entity) = InteractableEntities::from_index(&op_index, entity_index) {
                match entity {
                    InteractableEntities::Ent0 => {
                        info!("Generic Call Cast");
                    },
                    _ => {
                    },
                }
            } 
        }
    }
}

pub fn release_ray(
    mut raycast: Raycast,
    op_index: Res<OpIndex>,
    camera_query: Query<(&Camera, &GlobalTransform), With<CameraWorld>>, // Only query for the CameraWorld    
    interactable_query: Query<Entity, With<Interactable>>,
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
            let entity_index = entity.index();
            if let Some(entity) = InteractableEntities::from_index(&op_index, entity_index) {
                match entity {
                    InteractableEntities::Ent0 => {
                        info!("Generic Call Release");
                    },
                    _ => {
                    },
                }
            } 
        }
    }
}