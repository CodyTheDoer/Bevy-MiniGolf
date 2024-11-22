use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use std::collections::HashMap;

// States
use crate::{
    ArrowState,
    LevelState,
    MapSetState,
};

// Resources
use crate::{
    GameHandler,
    GLBStorageID,
    Ground, 
    Interactable, 
    Party,
    SceneInstanceSpawnedEvent,
    UserInterface,
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
    info!("Init Level: {}", gsh.current_level);
    gltf_handler_init_level_glb(asset_server, commands, glb_storage, gsh.current_level);
}

pub fn gltf_handler_init_level_glb(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    glb_storage: Res<GLBStorageID>, //Arc<[MapID]> //map: Arc<str>,
    level: i32,
) {
    if let Some(scene_glb_file) = glb_storage.glb.get((level) as usize) {
        let scene_handle: Handle<Scene> = asset_server.load(
            GltfAssetLabel::Scene(0).from_asset(scene_glb_file.map),
        );
        let scene_entities = commands
            .spawn(SceneBundle {
                scene: scene_handle.clone(),
                ..default()
            })
            .insert(Interactable)
            .id(); 
    } else {
        warn!("Target was not valid. Refer to the GLBStorageID map in the library.");
    };
}

pub fn gltf_handler_init_golf_ball_glb(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    glb_storage: Res<GLBStorageID>, //Arc<[MapID]> //map: Arc<str>,
    // mut asset_event_writer: EventWriter<AssetEvent<Mesh>>,
) {
    if let Some(golf_ball_glb_file) = glb_storage.glb.get((25) as usize) {

        let golf_ball_handle: Handle<Scene> = asset_server.load(
            GltfAssetLabel::Scene(0).from_asset(golf_ball_glb_file.map),
        );

        let golf_ball_entity = commands
            .spawn(SceneBundle {
                scene: golf_ball_handle.clone(),
                ..default()
            })
            .insert(Interactable)
            .id(); 

        // // Emit a custom AssetEvent for this asset
        // asset_event_writer.send(
        //     AssetEvent::Created {
        //         handle: golf_ball_handle.typed::<Mesh>(),
        //     }
        // );
    };
}

// When exiting state 
pub fn purge_entity(
    mut commands: Commands,
    entity: Entity,
) {
    // Access the rigid body from the physics world using its handle
    if Some(entity).is_some()  {
        commands.entity(entity).despawn_recursive()
    };
}        

pub fn purge_glb_all(
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

pub fn purge_rigid_bodies(
    mut commands: Commands,
    rigid_bodies: Query<(Entity, &RapierRigidBodyHandle)>,
) {
    for (entity, _) in rigid_bodies.iter() {
        // Access the rigid body from the physics world using its handle
        commands.entity(entity).despawn_recursive();
    }      
}