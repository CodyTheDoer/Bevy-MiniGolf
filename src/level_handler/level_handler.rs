use bevy::prelude::*;

use std::collections::HashMap;

use crate::{Ground, Interactable, InteractableEntities, OpIndex};

impl InteractableEntities {
    pub fn from_index(
        op_index: &Res<OpIndex>,
        index: u32,
    ) -> Option<InteractableEntities> {
        let mut interactable_entity_map = HashMap::new();
        interactable_entity_map.insert(0 + op_index.ui_entities, InteractableEntities::Ground);
        interactable_entity_map.insert(1 + op_index.ui_entities, InteractableEntities::Ent1);
        interactable_entity_map.insert(2 + op_index.ui_entities, InteractableEntities::Ent2);
        interactable_entity_map.insert(3 + op_index.ui_entities, InteractableEntities::Ent3);
        interactable_entity_map.insert(4 + op_index.ui_entities, InteractableEntities::Ent4);
        interactable_entity_map.insert(5 + op_index.ui_entities, InteractableEntities::Ent5);
        interactable_entity_map.insert(6 + op_index.ui_entities, InteractableEntities::Ent6);

        interactable_entity_map.get(&index).cloned()
    }

    pub fn entity_info(&self) {
        info!("Entity:\n   {:?}", self);
    }
}

pub fn gltf_handler_init(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut op_index: ResMut<OpIndex>,
) {
    commands.spawn(SceneBundle {
        scene: asset_server
            .load(GltfAssetLabel::Scene(0).from_asset("cube_blue.glb")),
            transform: Transform::from_xyz(-2.0, 0.0, 5.0),
        ..default()
    })
    .insert(Interactable); 
    op_index.add_ui_entity();

    commands.spawn(SceneBundle {
        scene: asset_server
            .load(GltfAssetLabel::Scene(0).from_asset("cube_terracotta.glb")),
            transform: Transform::from_xyz(2.0, 0.0, -5.0),
        ..default()
    })
    .insert(Interactable); 
    op_index.add_ui_entity();
}

pub fn query_and_despawn_scene (
    mut commands: Commands,
    scene_query: Query<(Entity, &Handle<Scene>)>,
    asset_server: Res<AssetServer>,
) {
    // We load the specific scene handle to compare it directly
    let target_handle: Handle<Scene> = asset_server.load("cube_terracotta.glb#Scene0");

    for (entity, scene_handle) in scene_query.iter() {
        // Check if the scene handle matches the target handle
        if scene_handle.id() == target_handle.id() {
            commands.entity(entity).despawn_recursive();
            info!("Despawning entity {:?}", entity);
        }
    }
}

pub fn setup_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut op_index: ResMut<OpIndex>,
) {
    // Circular plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Circle::new(2000.)).into(),
            material: materials.add(Color::srgb(0.1, 0.0, 0.1)),
            transform: Transform {
                translation: Vec3::new(0.0, -0.65, 0.0),
                rotation: Quat::from_rotation_x(-2.0 * (std::f32::consts::PI / 4.0)), //4 = 45 degrees
                ..default()
            },
            ..default()
        },
        Ground,
    ));
    op_index.add_ui_entity();
}

pub fn setup_light(
    mut commands: Commands,
    mut op_index: ResMut<OpIndex>,
) {
    // Light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    op_index.add_ui_entity();
}