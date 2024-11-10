use bevy::prelude::*;

use bevy_mod_raycast::prelude::*;

use std::collections::HashMap;

use crate::{
    CameraUi, 
    CameraWorld, 
    Fonts, 
    GameState,
    // GameStateHandler,
    Ground, 
    InfoCall, 
    Interactable, 
    InteractableEntities, 
    OpIndex,
};

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
            // warn!("No CameraWorld found or multiple CameraWorlds detected.");
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
                    InteractableEntities::Ground => {
                        info!("Cast: Ground");
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
                info!("Entity: {:?}", entity);
                match entity {
                    InteractableEntities::Ground => {
                        info!("Release: Ground");
                    },
                    _ => {
                    },
                }
            } 
        }
    }
}

pub fn game_state_update(
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    match game_state.get() {
        GameState::LoadingScreen => {
            info!("GameState::MenuMain");
            next_game_state.set(GameState::MenuMain);
        },
        GameState::MenuMain => {
            info!("GameState::MenuSettings");
            next_game_state.set(GameState::MenuSettings);
        },
        GameState::MenuSettings => {
            info!("GameState::MenuOnline");
            next_game_state.set(GameState::MenuOnline);
        },
        GameState::MenuOnline => {
            info!("GameState::InGame");
            next_game_state.set(GameState::InGame);
        },
        GameState::InGame => {
            info!("GameState::PostGameReview");
            next_game_state.set(GameState::PostGameReview);
        },
        GameState::PostGameReview => {
            info!("GameState::LoadingScreen");
            next_game_state.set(GameState::LoadingScreen);
        },
    }
}

pub fn game_state_logic(
    game_state: Res<State<GameState>>,
) {
    match game_state.get() {
        GameState::LoadingScreen => {},
        GameState::MenuMain => {},
        GameState::MenuSettings => {},
        GameState::MenuOnline => {},
        GameState::InGame => {},
        GameState::PostGameReview => {},
    }
}
