use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

// use std::collections::HashMap;

// // States
// use crate::{
//     StateArrow,
//     StateLevel,
//     StateMapSet,
// };

// Resources
use crate::{
    GLBStorageID,
    Interactable, 
    // Party,
    // SceneInstanceSpawnedEvent,
    // UserInterface,
};

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

        let _golf_ball_entity = commands
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
