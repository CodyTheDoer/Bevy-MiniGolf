use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, RigidBody};

use std::collections::HashMap;

use crate::print_ball_altitude;
use crate::{
    GameStateHandler,
    GLBStorageID,
    GLBPurgeID,
    Ground, 
    Interactable, 
    InteractableEntities, 
    LevelState,
    MapSetState,
    OpIndex,
    UserInterface,
};

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
            material: materials.add(Color::srgba(0.1, 0.0, 0.1, 1.0)),
            transform: Transform {
                translation: Vec3::new(0.0, -5.0, 0.0),
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

pub fn level_state_update(
    level_state: Res<State<LevelState>>,
    mut next_game_state: ResMut<NextState<LevelState>>,
    mut gsh: ResMut<GameStateHandler>,
) {
    match level_state.get() {
        LevelState::HoleTutorial => {
            gsh.current_level += 1;
            info!("LevelState::Hole1");
            next_game_state.set(LevelState::Hole1);
        },
        LevelState::Hole1 => {
            gsh.current_level += 1;
            info!("LevelState::Hole2");
            next_game_state.set(LevelState::Hole2);
        },
        LevelState::Hole2 => {
            gsh.current_level += 1;
            info!("LevelState::Hole3");
            next_game_state.set(LevelState::Hole3);
        },
        LevelState::Hole3 => {
            gsh.current_level += 1;
            info!("LevelState::Hole4");
            next_game_state.set(LevelState::Hole4);
        },
        LevelState::Hole4 => {
            gsh.current_level += 1;
            info!("LevelState::Hole5");
            next_game_state.set(LevelState::Hole5);
        },
        LevelState::Hole5 => {
            gsh.current_level += 1;
            info!("LevelState::Hole6");
            next_game_state.set(LevelState::Hole6);
        },
        LevelState::Hole6 => {
            gsh.current_level += 1;
            info!("LevelState::Hole7");
            next_game_state.set(LevelState::Hole7);
        },
        LevelState::Hole7 => {
            gsh.current_level += 1;
            info!("LevelState::Hole8");
            next_game_state.set(LevelState::Hole8);
        },
        LevelState::Hole8 => {
            gsh.current_level += 1;
            info!("LevelState::Hole9");
            next_game_state.set(LevelState::Hole9);
        },
        LevelState::Hole9 => {
            gsh.current_level += 1;
            info!("LevelState::Hole10");
            next_game_state.set(LevelState::Hole10);
        },
        LevelState::Hole10 => {
            gsh.current_level += 1;
            info!("LevelState::Hole11");
            next_game_state.set(LevelState::Hole11);
        },
        LevelState::Hole11 => {
            gsh.current_level += 1;
            info!("LevelState::Hole12");
            next_game_state.set(LevelState::Hole12);
        },
        LevelState::Hole12 => {
            gsh.current_level += 1;
            info!("LevelState::Hole13");
            next_game_state.set(LevelState::Hole13);
        },
        LevelState::Hole13 => {
            gsh.current_level += 1;
            info!("LevelState::Hole14");
            next_game_state.set(LevelState::Hole14);
        },
        LevelState::Hole14 => {
            gsh.current_level += 1;
            info!("LevelState::Hole15");
            next_game_state.set(LevelState::Hole15);
        },
        LevelState::Hole15 => {
            gsh.current_level += 1;
            info!("LevelState::Hole16");
            next_game_state.set(LevelState::Hole16);
        },
        LevelState::Hole16 => {
            gsh.current_level += 1;
            info!("LevelState::Hole17");
            next_game_state.set(LevelState::Hole17);
        },
        LevelState::Hole17 => {
            gsh.current_level += 1;
            info!("LevelState::Hole18");
            next_game_state.set(LevelState::Hole18);
        },
        LevelState::Hole18 => {
            gsh.current_level += 1;
            info!("LevelState::HoleTutorial");
            next_game_state.set(LevelState::Physics);
        },
        LevelState::Physics => {
            gsh.current_level = 0;
            info!("LevelState::HoleTutorial");
            next_game_state.set(LevelState::HoleTutorial);
        },
    }
}

pub fn level_state_logic(
    level_state: Res<State<LevelState>>,
    mut positions: Query<&mut Transform, With<RigidBody>>,
) {
    match level_state.get() {
        LevelState::HoleTutorial => {},
        LevelState::Hole1 => {},
        LevelState::Hole2 => {},
        LevelState::Hole3 => {},
        LevelState::Hole4 => {},
        LevelState::Hole5 => {},
        LevelState::Hole6 => {},
        LevelState::Hole7 => {},
        LevelState::Hole8 => {},
        LevelState::Hole9 => {},
        LevelState::Hole10 => {},
        LevelState::Hole11 => {},
        LevelState::Hole12 => {},
        LevelState::Hole13 => {},
        LevelState::Hole14 => {},
        LevelState::Hole15 => {},
        LevelState::Hole16 => {},
        LevelState::Hole17 => {},
        LevelState::Hole18 => {},
        LevelState::Physics => {
            print_ball_altitude(positions);
        },
    }
}

// When entering state 
pub fn init_hole_n(
    asset_server: Res<AssetServer>,
    commands: Commands,
    mut op_index: ResMut<OpIndex>,
    glb_storage: Res<GLBStorageID>,
    gsh: Res<GameStateHandler>,
) {
    info!("Init Hole: Hole {}", gsh.current_level);
    gltf_handler_init_hole_n(asset_server, commands, op_index.into(), glb_storage, gsh.current_level);
}

pub fn gltf_handler_init_hole_n(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut op_index: ResMut<OpIndex>,
    glb_storage: Res<GLBStorageID>, //Arc<[MapID]> //map: Arc<str>,
    hole: i32,
) {
    if let Some(glb_file) = glb_storage.glb.get((hole) as usize) {
        let ball_handle: Handle<Scene> = asset_server.load(
            GltfAssetLabel::Scene(0).from_asset("glb/basic_ball.glb"),
        );
        let cup_handle: Handle<Scene> = asset_server.load(
            GltfAssetLabel::Scene(0).from_asset("glb/basic_cup.glb"),
        );
        let start_handle: Handle<Scene> = asset_server.load(
            GltfAssetLabel::Scene(0).from_asset("glb/basic_start.glb"),
        );
        let map_handle: Handle<Scene> = asset_server.load(
            GltfAssetLabel::Scene(0).from_asset("glb/basic_level_1.glb"),
        );
        let scene_handle: Handle<Scene> = asset_server.load(
            GltfAssetLabel::Scene(0).from_asset(glb_file.map),
        );

        if hole == 0 {
            let root_entity_ball = commands
                .spawn(SceneBundle {
                    scene: ball_handle.clone(),
                    transform: Transform::from_xyz(0.0, 10.0, 60.0),
                    ..default()
                })
                .insert(Interactable)
                .id();   
            let root_entity_cup = commands
                .spawn(SceneBundle {
                    scene: cup_handle.clone(),
                    transform: Transform::from_xyz(0.0, -2.30, -60.0),
                    ..default()
                })
                .insert(Interactable)
                .id();   
            let root_entity_start = commands
                .spawn(SceneBundle {
                    scene: start_handle.clone(),
                    transform: Transform::from_xyz(0.0, 0.0, 60.0),
                    ..default()
                })
                .insert(Interactable)
                .id();  
            let root_entity_map = commands
                .spawn(SceneBundle {
                    scene: map_handle.clone(),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..default()
                })
                .insert(Interactable)
                .id();   
        } else {
            let root_entity = commands
                .spawn(SceneBundle {
                    scene: scene_handle.clone(),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..default()
                })
                .insert(Interactable)
                .insert(Name::new(format!("Hole{}", hole))) // Add a name to help with debugging
                .id(); 
            op_index.add_ui_entity();    
        }
    } else {
        warn!("Target map was not valid. Hole was out of bounds, 0 for the tutorial, 1-18 for the golf holes.");
    };
}

// When exiting state 
pub fn remove_match_from_vec(vec: &mut ResMut<GLBPurgeID>, pattern: &str) {
    if let Some(pos) = vec.glb.iter().position(|x| x == pattern) {
        vec.glb.remove(pos);
    }
}

pub fn gltf_handler_purge(
    mut commands: Commands,
    scene_query: Query<(Entity, &Handle<Scene>)>,
    asset_server: Res<AssetServer>,
    mut purge: ResMut<GLBPurgeID>,
) {
    let targets = purge.clone();
    for asset_to_despawn in targets.glb.iter() {
        let target_asset = format!("{}#Scene0",asset_to_despawn);
        // We load the specific scene handle to compare it directly
        let despawn_target: Handle<Scene> = asset_server.load(target_asset);// format!("{}#Scene0", glb_file)
        for (entity, scene_handle) in scene_query.iter() {
            // Check if the scene handle matches the target handle
            if scene_handle.id() == despawn_target.id() {
                commands.entity(entity).despawn_recursive();
                info!("Despawned Entity: {:?}", entity);
            }
        }
        remove_match_from_vec(&mut purge, asset_to_despawn);
    }
}

pub fn purge_glb_all_prep(
    purge: &mut ResMut<GLBPurgeID>,
    glb_storage: Res<GLBStorageID>,
) {
    let targets = glb_storage.clone();
    // info!("{:?}", targets);
    for asset_to_despawn in targets.glb.iter() {
        purge.glb.push(asset_to_despawn.map.to_string());
    }
}

pub fn purge_glb_all(
    commands: Commands,
    scene_query: Query<(Entity, &Handle<Scene>)>,
    asset_server: Res<AssetServer>,
    mut purge: ResMut<GLBPurgeID>,
    glb_storage: Res<GLBStorageID>,
) {
    purge_glb_all_prep(&mut purge, glb_storage);
    gltf_handler_purge(commands, scene_query, asset_server, purge);
}

pub fn map_set_state_update(
    map_set_state: Res<State<MapSetState>>,
    mut next_map_set_state: ResMut<NextState<MapSetState>>,
    mut gsh: ResMut<GameStateHandler>,
) {
    match map_set_state.get() {
        MapSetState::Tutorial => {
            info!("MapSetState::Tutorial");
            gsh.current_level = 0;
            gsh.maps_index = 1;
            next_map_set_state.set(MapSetState::WholeCorse);
        },
        MapSetState::WholeCorse => {
            info!("MapSetState::WholeCorse");
            gsh.current_level = 0;
            gsh.maps_index = 18;
            next_map_set_state.set(MapSetState::FrontNine);
        },
        MapSetState::FrontNine => {
            info!("MapSetState::FrontNine");
            gsh.current_level = 0;
            gsh.maps_index = 9;
            next_map_set_state.set(MapSetState::BackNine);
        },
        MapSetState::BackNine => {
            info!("MapSetState::BackNine");
            gsh.current_level = 0;
            gsh.maps_index = 9;
            next_map_set_state.set(MapSetState::SelectAHole);
        },
        MapSetState::SelectAHole => {
            info!("MapSetState::SelectAHole");
            gsh.current_level = 0;
            gsh.maps_index = 1;
            let hole = UserInterface::select_a_hole_widget();
            match hole {
                0 => {
                    next_map_set_state.set(MapSetState::Tutorial);
                },
                1 => {},
                2 => {},
                3 => {},
                4 => {},
                5 => {},
                6 => {},
                7 => {},
                8 => {},
                9 => {},
                10 => {},
                11 => {},
                12 => {},
                13 => {},
                14 => {},
                15 => {},
                16 => {},
                17 => {},
                18 => {},
                _ => {},
            }
        },
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
