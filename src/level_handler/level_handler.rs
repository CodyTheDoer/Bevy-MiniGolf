use bevy::prelude::*;

// States
use crate::{
    StateLevel,
    StateMapSet,
};

// Resources
use crate::{
    Interactable,
    GameHandler,
    GLBStorageID,
    Ground, 
    MapID,
    RunTrigger,
};

impl GLBStorageID {
    pub fn new() -> Self {
        let map_paths = [
            // "glb/menu/main_menu.glb",           //  0
            "glb/menu/menu_all.glb",            //  0
            "glb/map/level_1.glb",              //  1
            "glb/map/level_2.glb",              //  2
            "glb/map/level_3.glb",              //  3
            "glb/map/level_4.glb",              //  4
            "glb/map/level_5.glb",              //  5
            "glb/map/level_6.glb",              //  6
            "glb/map/level_7.glb",              //  7
            "glb/map/level_8.glb",              //  8
            "glb/map/level_9.glb",              //  9
            "glb/map/level_10.glb",             // 10
            "glb/map/level_11.glb",             // 11
            "glb/map/level_12.glb",             // 12
            "glb/map/level_13.glb",             // 13
            "glb/map/level_14.glb",             // 14
            "glb/map/level_15.glb",             // 15
            "glb/map/level_16.glb",             // 16
            "glb/map/level_17.glb",             // 17
            "glb/map/level_18.glb",             // 18
            "glb/map/level_tutorial.glb",       // 19
            "glb/menu/menu_leader_board.glb",   // 20
            "glb/menu/menu_local.glb",          // 21
            "glb/menu/menu_online.glb",         // 22
            "glb/menu/menu_preferences.glb",    // 23
            "glb/menu/menu_player.glb",         // 24
            "glb/map/golf_ball.glb",            // 25
        ];
        let map_ids: Vec<MapID> = map_paths
            .iter()
            .map(|&path| MapID { map: path })
            .collect();
        GLBStorageID {
            glb: map_ids.into_boxed_slice().into(), // Vec -> Box -> Arc
        }
    }
}

pub fn level_handler_boot_protocals(
    il_asset_server: Res<AssetServer>,
    il_commands_init: Commands,
    il_commands_purge: Commands,
    il_scene_meshes: Query<(Entity, &Name)>,
    il_glb_storage: Res<GLBStorageID>,
    mut il_gh: ResMut<GameHandler>,
    sg_commands: Commands,
    sg_meshes: ResMut<Assets<Mesh>>,
    sg_materials: ResMut<Assets<StandardMaterial>>,
    sl_commands: Commands,
) {
    let level: i32 = 0;
    il_gh.current_level_set(level);
    level_handler_init_level_game_handler_current_level(il_asset_server, il_commands_init, il_commands_purge,  il_scene_meshes, il_glb_storage, il_gh);
    setup_ground(sg_commands, sg_meshes, sg_materials);
    setup_light(sl_commands);
}

pub fn level_handler_init_level_game_handler_current_level(
    asset_server: Res<AssetServer>,
    commands_init: Commands,
    commands_purge: Commands,
    scene_meshes: Query<(Entity, &Name)>,
    glb_storage: Res<GLBStorageID>,
    gh: ResMut<GameHandler>,
) {
    info!("level_handler_init_level_game_handler_current_level: [{}]", gh.current_level);
    level_handler_init_level(asset_server, commands_init, glb_storage, gh.current_level);
    level_handler_purge_glb_all(commands_purge, scene_meshes);
}

// Helper: level_handler_init_level_game_handler
fn level_handler_init_level(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    glb_storage: Res<GLBStorageID>, //Arc<[MapID]> //map: Arc<str>,
    level: i32,
) {
    if let Some(scene_glb_file) = glb_storage.glb.get((level) as usize) {
        let scene_handle: Handle<Scene> = asset_server.load(
            GltfAssetLabel::Scene(0).from_asset(scene_glb_file.map),
        );
        let _scene_entities = commands
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

// Helper: level_handler_boot_protocals
fn setup_ground(
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

// Helper: level_handler_boot_protocals
fn setup_light(
    mut commands: Commands,
) {
    // Light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

pub fn level_handler_set_state_next_level(
    mut run_trigger: ResMut<RunTrigger>,
    state_level: Res<State<StateLevel>>,
    mut next_level: ResMut<NextState<StateLevel>>,
) {
    info!("function: level_handler_set_state_next_level"); 
    match state_level.get() {
        StateLevel::Hole1 => {
            next_level.set(StateLevel::Hole2);
        },
        StateLevel::Hole2 => {
            next_level.set(StateLevel::Hole3);
        },
        StateLevel::Hole3 => {
            next_level.set(StateLevel::Hole4);
        },
        StateLevel::Hole4 => {
            next_level.set(StateLevel::Hole5);
        },
        StateLevel::Hole5 => {
            next_level.set(StateLevel::Hole6);
        },
        StateLevel::Hole6 => {
            next_level.set(StateLevel::Hole7);
        },
        StateLevel::Hole7 => {
            next_level.set(StateLevel::Hole8);
        },
        StateLevel::Hole8 => {
            next_level.set(StateLevel::Hole9);
        },
        StateLevel::Hole9 => {
            next_level.set(StateLevel::Hole10);
        },
        StateLevel::Hole10 => {
            next_level.set(StateLevel::Hole11);
        },
        StateLevel::Hole11 => {
            next_level.set(StateLevel::Hole12);
        },
        StateLevel::Hole12 => {
            next_level.set(StateLevel::Hole13);
        },
        StateLevel::Hole13 => {
            next_level.set(StateLevel::Hole14);
        },
        StateLevel::Hole14 => {
            next_level.set(StateLevel::Hole15);
        },
        StateLevel::Hole15 => {
            next_level.set(StateLevel::Hole16);
        },
        StateLevel::Hole16 => {
            next_level.set(StateLevel::Hole17);
        },
        StateLevel::Hole17 => {
            next_level.set(StateLevel::Hole18);
        },
        _ => {},
    };
    run_trigger.set_target("level_handler_set_state_next_level", false);
}

pub fn level_handler_set_state_next_map_set(
    mut run_trigger: ResMut<RunTrigger>,
    state_map_set: Res<State<StateMapSet>>,
    mut next_state_map_set: ResMut<NextState<StateMapSet>>,
){
    info!("function: level_handler_set_state_next_map_set"); 
    match state_map_set.get() {
        StateMapSet::Tutorial => {
            info!("StateMapSet::WholeCorse");
            next_state_map_set.set(StateMapSet::WholeCorse);
        },
        StateMapSet::WholeCorse => {
            info!("StateMapSet::FrontNine");
            next_state_map_set.set(StateMapSet::FrontNine);
        },
        StateMapSet::FrontNine => {
            info!("StateMapSet::BackNine");
            next_state_map_set.set(StateMapSet::BackNine);
        },
        StateMapSet::BackNine => {
            info!("StateMapSet::SelectAHole");
            next_state_map_set.set(StateMapSet::SelectAHole);
        },
        StateMapSet::SelectAHole => {
            info!("StateMapSet::Tutorial");
            next_state_map_set.set(StateMapSet::Tutorial);
        },
    };
    run_trigger.set_target("level_handler_set_state_next_map_set", false);
    info!("post response: level_handler_set_state_next_map_set: {}", run_trigger.get("level_handler_set_state_next_map_set"));  
}

// When exiting state 
pub fn level_handler_purge_entity(
    commands: &mut Commands,
    entity: Entity,
) {
    // Access the rigid body from the physics world using its handle
    if Some(entity).is_some()  {
        commands.entity(entity).despawn_recursive()
    };
}        

pub fn level_handler_purge_glb_all(
    mut commands: Commands,
    scene_meshes: Query<(Entity, &Name)>,
) {
    for (entity, _) in scene_meshes.iter() {
        // Access the rigid body from the physics world using its handle
        level_handler_purge_entity(&mut commands, entity);
    }        
}

// pub fn level_handler_purge_rigid_bodies(
//     mut commands: Commands,
//     rigid_bodies: Query<(Entity, &RapierRigidBodyHandle)>,
// ) {
//     for (entity, _) in rigid_bodies.iter() {
//         // Access the rigid body from the physics world using its handle
//         commands.entity(entity).despawn_recursive();
//     }      
// }