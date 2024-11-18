use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use std::collections::HashMap;

use crate::{
    ArrowState,
    GameHandler,
    GLBStorageID,
    Ground, 
    Interactable, 
    LevelState,
    MapSetState,
    UserInterface,
};

use crate::level_handler::physics_handler::{
    add_physics_query_and_update_scene,
};

pub fn setup_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Circular plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Circle::new(2000.)).into(),
            material: materials.add(Color::srgba(0.1, 0.0, 0.1, 1.0)),
            transform: Transform {
                translation: Vec3::new(0.0, -15.0, 0.0),
                rotation: Quat::from_rotation_x(-2.0 * (std::f32::consts::PI / 4.0)), //4 = 45 degrees
                ..default()
            },
            ..default()
        },
        Ground,
    ));
}

pub fn setup_light(
    mut commands: Commands,
) {
    // Light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

// When entering state 
pub fn init_level_glb(
    asset_server: Res<AssetServer>,
    commands: Commands,
    glb_storage: Res<GLBStorageID>,
    gsh: Res<GameHandler>,
) {
    info!("Init Hole: Hole {}", gsh.current_level);
    gltf_handler_init_level_glb(asset_server, commands, glb_storage, gsh.current_level);
}

pub fn gltf_handler_init_level_glb(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    glb_storage: Res<GLBStorageID>, //Arc<[MapID]> //map: Arc<str>,
    hole: i32,
) {
    if let Some(glb_file) = glb_storage.glb.get((hole) as usize) {
        let scene_handle: Handle<Scene> = asset_server.load(
            GltfAssetLabel::Scene(0).from_asset(glb_file.map),
        );

        let root_entity = commands
            .spawn(SceneBundle {
                scene: scene_handle.clone(),
                ..default()
            })
            .insert(Interactable)
            .insert(Name::new(format!("Hole{}", hole))) // Add a name to help with debugging
            .id(); 
    } else {
        warn!("Target map was not valid. Hole was out of bounds, 0 for the main menu, 1-18 for the holes, 19 for the tutorial.");    };
}

// When exiting state 
pub fn purge_glb(
    mut commands: Commands,
    scene_meshes: Query<(Entity, &Name)>,
) {
    for (entity, _) in scene_meshes.iter() {
        // Access the rigid body from the physics world using its handle
        if Some(entity).is_some()  {
            commands.entity(entity).despawn_recursive()
        };
    }        
}

// When exiting state 
pub fn purge_rigid_bodies(
    mut commands: Commands,
    rigid_bodies: Query<(Entity, &RapierRigidBodyHandle)>,
) {
    for (entity, _) in rigid_bodies.iter() {
        // Access the rigid body from the physics world using its handle
        commands.entity(entity).despawn_recursive();
    }      
}

pub fn map_set_state_logic(
    map_set_state: Res<State<MapSetState>>,
) {
    match map_set_state.get() {
        MapSetState::Tutorial => {},
        MapSetState::WholeCorse => {},
        MapSetState::FrontNine => {},
        MapSetState::BackNine => {},
        MapSetState::SelectAHole => {},
    }
}