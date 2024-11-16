use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use std::collections::HashMap;

use crate::{
    ArrowState,
    GameStateHandler,
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

pub fn level_state_update(
    level_state: Res<State<LevelState>>,
    mut next_game_state: ResMut<NextState<LevelState>>,
    mut gsh: ResMut<GameStateHandler>,
) {
    match level_state.get() {
        LevelState::MainMenu => {
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
            next_game_state.set(LevelState::HoleTutorial);
        },
        LevelState::HoleTutorial => {
            gsh.current_level = 0;
            info!("LevelState::Hole1");
            next_game_state.set(LevelState::MainMenu);
        },
    }
}

pub fn level_state_logic(
    level_state: Res<State<LevelState>>,
    mut positions: Query<&mut Transform, With<RigidBody>>,
) {
    match level_state.get() {
        LevelState::MainMenu => {},
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
        LevelState::HoleTutorial => {},
    }
}


use crate::LoadedSceneHandle;

// When entering state 
pub fn init_hole_n(
    asset_server: Res<AssetServer>,
    asset_server_2: Res<AssetServer>,
    loaded_scene_handle: Option<Res<LoadedSceneHandle>>,
    commands: Commands,
    commands_2: Commands,
    glb_storage: Res<GLBStorageID>,
    gsh: Res<GameStateHandler>,
    mut arrow_state: ResMut<State<ArrowState>>,
    mut next_arrow_state: ResMut<NextState<ArrowState>>,
    mut mut_commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    scene_meshes: Query<(Entity, &Name, &Handle<Mesh>, &Transform)>,
    ground_query: Query<(Entity, &Handle<Mesh>), With<Ground>>,
) {
    info!("Init Hole: Hole {}", gsh.current_level);
    gltf_handler_init_hole_n(asset_server, commands, glb_storage, gsh.current_level);
    check_scene_loaded(asset_server_2, loaded_scene_handle, commands_2, arrow_state, next_arrow_state, mut_commands, meshes, scene_meshes, ground_query);
}

// This system checks if the scene has finished loading
pub fn check_scene_loaded(
    asset_server: Res<AssetServer>,
    loaded_scene_handle: Option<Res<LoadedSceneHandle>>,
    mut commands: Commands,
    mut arrow_state: ResMut<State<ArrowState>>,
    mut next_arrow_state: ResMut<NextState<ArrowState>>,
    mut mut_commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    scene_meshes: Query<(Entity, &Name, &Handle<Mesh>, &Transform)>,
    ground_query: Query<(Entity, &Handle<Mesh>), With<Ground>>,
) {
    if let Some(loaded_scene_handle) = loaded_scene_handle {
        let load_state = asset_server.get_load_state(&loaded_scene_handle.handle);

        if load_state == Some(LoadState::Loaded) {
            // Scene has finished loading, now proceed to add physics
            info!("Scene has finished loading, proceeding to add physics.");

            add_physics_query_and_update_scene(
                arrow_state,
                next_arrow_state,
                mut_commands,
                meshes,
                scene_meshes,
                ground_query,
            );

            // Remove the handle resource, as it is no longer needed
            commands.remove_resource::<LoadedSceneHandle>();
        }
    }
}

use bevy::asset::LoadState;

pub fn gltf_handler_init_hole_n(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    glb_storage: Res<GLBStorageID>, //Arc<[MapID]> //map: Arc<str>,
    hole: i32,
) {
    if let Some(glb_file) = glb_storage.glb.get((hole) as usize) {
        let ball_handle: Handle<Scene> = asset_server.load(
            GltfAssetLabel::Scene(0).from_asset("glb/scaled/golf_ball.glb"),
        );
        let map_handle: Handle<Scene> = asset_server.load(
            GltfAssetLabel::Scene(0).from_asset("glb/scaled/level_1.glb"),
        );
        let scene_handle: Handle<Scene> = asset_server.load(
            GltfAssetLabel::Scene(0).from_asset(glb_file.map),
        );

        if hole == 19 {
            // let root_entity_ball = commands // Eventually will be attached to player.
            //     .spawn(SceneBundle {
            //         scene: ball_handle.clone(),
            //         ..default()
            //     })
            //     .insert(Interactable)
            //     .id();   
            let root_entity_map = commands
                .spawn(SceneBundle {
                    scene: map_handle.clone(),
                    ..default()
                })
                .insert(Interactable)
                .insert(Name::new(format!("Hole{}", hole))) // Add a name to help with debugging
                .id();
            
            commands.insert_resource(LoadedSceneHandle { handle: scene_handle });
        } else {
            let root_entity = commands
                .spawn(SceneBundle {
                    scene: scene_handle.clone(),
                    ..default()
                })
                .insert(Interactable)
                .insert(Name::new(format!("Hole{}", hole))) // Add a name to help with debugging
                .id(); 
        }
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
