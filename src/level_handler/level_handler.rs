use bevy::prelude::*;

use std::collections::HashMap;

use crate::{Ground, Interactable, InteractableEntities, OpIndex};

impl InteractableEntities {
    pub fn from_index(
        op_index: &Res<OpIndex>,
        index: u32,
    ) -> Option<InteractableEntities> {
        let mut interactable_entity_map = HashMap::new();
        interactable_entity_map.insert(0 + op_index.ui_entities, InteractableEntities::Ent0);
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

pub fn setup_gltf(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut op_index: ResMut<OpIndex>,
) {
    let gltf = asset_server.load("cube.glb#Scene0"); // Screen text is set in calculator.rs via CurrentMeshColor::process_entity_children

    // Scene
    commands.spawn(SceneBundle {
        scene: gltf,
        ..Default::default()
    })
    .insert(Interactable); 
    op_index.add_ui_entity();
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