use bevy::prelude::*;

// States
use crate::{
    StateLevel, 
    StateMapSet,
    StatePanOrbit,
};

// Resources
use crate::{
    CheckStatePH,
    CheckStateRT,
    Interactable,
    GameHandler,
    GLBStorageID,
    GolfBall,
    Ground, 
    MapID,
    PurgeHandler,
    RunTrigger,
    SceneInstancePurgedEnvironment,
    SceneInstancePurgedGolfBalls,
    SceneInstanceSpawnedEnvironment,
};

impl GLBStorageID {
    pub fn new() -> Self {
        let map_paths = [
            "glb/menu/main_menu.glb",           //  0
            "glb/map/level_1.glb",              //  1
            "glb/map/level_2.glb",              //  2
            "glb/map/level_3.glb",              //  3
            "glb/map/level_4.glb",              //  4
            "glb/map/level_5.glb",              //  5
            "glb/map/level_6.glb",              //  6
            "glb/map/level_7.glb",              //  7
            "glb/map/level_8.glb",              //  8
            "glb/map/level_9.glb",              //  9
            "glb/map/level_10.glb",              // 10
            "glb/map/level_11.glb",              // 11
            "glb/map/level_12.glb",              // 12
            "glb/map/level_13.glb",              // 13
            "glb/map/level_14.glb",              // 14
            "glb/map/level_15.glb",              // 15
            "glb/map/level_16.glb",              // 16
            "glb/map/level_17.glb",              // 17
            "glb/map/level_18.glb",              // 18
            "glb/map/level_tutorial.glb",       // 19
            "glb/menu/menu_leader_board.glb",   // 20
            "glb/menu/menu_local.glb",          // 21
            "glb/menu/menu_online.glb",         // 22
            "glb/menu/menu_preferences.glb",    // 23
            "glb/menu/menu_player.glb",         // 24
            "glb/entities/golf_ball.glb",       // 25
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
    mut game_handler: ResMut<GameHandler>,
    mut run_trigger: ResMut<RunTrigger>,
    sg_commands: Commands,
    sg_meshes: ResMut<Assets<Mesh>>,
    sg_materials: ResMut<Assets<StandardMaterial>>,
    sl_commands: Commands,
) {
    game_handler.current_level_set(0);
    run_trigger.set_target(CheckStateRT::LevelHandlerInitLevelGameHandlerCurrentLevel, true);
    setup_ground(sg_commands, sg_meshes, sg_materials);
    setup_light(sl_commands);
}

pub fn level_handler_purge_protocol(
    mut run_trigger: ResMut<RunTrigger>,
    sm_commands: Commands,
    scene_meshes: Query<(Entity, &Name)>,
    gb_commands: Commands,
    golf_balls: Query<Entity, With<GolfBall>>,
    purge_event_writer_environment: EventWriter<SceneInstancePurgedEnvironment>,
    purge_event_writer_golf_ball: EventWriter<SceneInstancePurgedGolfBalls>,
    purge_handler: Res<PurgeHandler>,
) {
    
    info!("function: level_handler_purge_protocol"); 
    {
        if purge_handler.get(CheckStatePH::EnvironmentPurged) == false {
            level_handler_purge_env_glb_all(sm_commands, scene_meshes, purge_event_writer_environment);
        }
        if purge_handler.get(CheckStatePH::GolfBallsPurged) == false {
            level_handler_purge_golf_ball_all(gb_commands, golf_balls, purge_event_writer_golf_ball);
        }
    }
    run_trigger.set_target(CheckStateRT::LevelHandlerPurgeProtocol, false);
    info!("post response: level_handler_purge_protocol: [{}]", run_trigger.get(CheckStateRT::LevelHandlerPurgeProtocol));  
}

pub fn level_handler_init_level_game_handler_current_level(
    mut run_trigger: ResMut<RunTrigger>,
    lhi_asset_server: Res<AssetServer>,
    lhi_commands: Commands,
    glb_storage: Res<GLBStorageID>,
    mut purge_handler: ResMut<PurgeHandler>,
    mut asset_event_writer: EventWriter<SceneInstanceSpawnedEnvironment>,
    mut pan_orbit_camera_query: Query<&mut StatePanOrbit>,
    game_handler: Res<GameHandler>
) {
    info!("level_handler_init_level_game_handler_current_level: [{}]", game_handler.current_level);
    {
        info!("Purge Handler: Environment: [{}] Golf Balls [{}]", purge_handler.get(CheckStatePH::EnvironmentPurged), purge_handler.get(CheckStatePH::GolfBallsPurged));
        // Write in testing for purge states:
        if purge_handler.get(CheckStatePH::EnvironmentPurged) && purge_handler.get(CheckStatePH::GolfBallsPurged) {
            level_handler_init_level(lhi_asset_server, lhi_commands, glb_storage, game_handler.current_level, &mut asset_event_writer);
            purge_handler.set_target(CheckStatePH::EnvironmentPurged, false);
            run_trigger.set_target(CheckStateRT::LevelHandlerInitLevelGameHandlerCurrentLevel, false);
            info!("post response: level_handler_init_level_game_handler_current_level: [{}]", run_trigger.get(CheckStateRT::LevelHandlerInitLevelGameHandlerCurrentLevel));
            if game_handler.current_level_get() == 0 { // Easy way to fix the boot camera. Probably could do better
                for mut state in pan_orbit_camera_query.iter_mut() {
                    info!("{:?}", state);
                    state.radius = 38.0;
                    state.pitch = -12.0f32.to_radians();
                    state.yaw = -17.0f32.to_radians();
                }
            }
        } else {
            run_trigger.set_target(CheckStateRT::LevelHandlerPurgeProtocol, true);
        }
    }
}

// Helper: level_handler_init_level_game_handler
fn level_handler_init_level(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    glb_storage: Res<GLBStorageID>, //Arc<[MapID]> //map: Arc<str>,
    level: i32,
    asset_event_writer: &mut EventWriter<SceneInstanceSpawnedEnvironment>,
) {
    info!("level_handler_init_level: Running");
    if let Some(scene_glb_file) = glb_storage.glb.get((level) as usize) {
        let scene_handle: Handle<Scene> = asset_server.load(
            GltfAssetLabel::Scene(0).from_asset(scene_glb_file.map),
        );
        info!("level_handler_init_level: Loading: [{:?}]", scene_handle);
        let scene_entities = commands
            .spawn(SceneBundle {
                scene: scene_handle.clone(),
                ..default()
            })
            .insert(Interactable)
            .id(); 
        info!("level_handler_init_level: Completed");

        // Emit a custom AssetEvent for this asset
        asset_event_writer.send(SceneInstanceSpawnedEnvironment {
                entity: scene_entities,
            }
        );
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
    mut game_handler: ResMut<GameHandler>,
) {
    info!("function: level_handler_set_state_next_level"); 
    {
        match state_level.get() {
            StateLevel::Hole1 => {
                next_level.set(StateLevel::Hole2);
                game_handler.current_level_set(2);
            },
            StateLevel::Hole2 => {
                next_level.set(StateLevel::Hole3);
                game_handler.current_level_set(3);
            },
            StateLevel::Hole3 => {
                next_level.set(StateLevel::Hole4);
                game_handler.current_level_set(4);
            },
            StateLevel::Hole4 => {
                next_level.set(StateLevel::Hole5);
                game_handler.current_level_set(5);
            },
            StateLevel::Hole5 => {
                next_level.set(StateLevel::Hole6);
                game_handler.current_level_set(6);
            },
            StateLevel::Hole6 => {
                next_level.set(StateLevel::Hole7);
                game_handler.current_level_set(7);
            },
            StateLevel::Hole7 => {
                next_level.set(StateLevel::Hole8);
                game_handler.current_level_set(8);
            },
            StateLevel::Hole8 => {
                next_level.set(StateLevel::Hole9);
                game_handler.current_level_set(9);
            },
            StateLevel::Hole9 => {
                next_level.set(StateLevel::Hole10);
                game_handler.current_level_set(10);
            },
            StateLevel::Hole10 => {
                next_level.set(StateLevel::Hole11);
                game_handler.current_level_set(11);
            },
            StateLevel::Hole11 => {
                next_level.set(StateLevel::Hole12);
                game_handler.current_level_set(12);
            },
            StateLevel::Hole12 => {
                next_level.set(StateLevel::Hole13);
                game_handler.current_level_set(13);
            },
            StateLevel::Hole13 => {
                next_level.set(StateLevel::Hole14);
                game_handler.current_level_set(14);
            },
            StateLevel::Hole14 => {
                next_level.set(StateLevel::Hole15);
                game_handler.current_level_set(15);
            },
            StateLevel::Hole15 => {
                next_level.set(StateLevel::Hole16);
                game_handler.current_level_set(16);
            },
            StateLevel::Hole16 => {
                next_level.set(StateLevel::Hole17);
                game_handler.current_level_set(17);
            },
            StateLevel::Hole17 => {
                next_level.set(StateLevel::Hole18);
                game_handler.current_level_set(18);
            },
            _ => {},
        };
    }
    run_trigger.set_target(CheckStateRT::LevelHandlerSetStateNextLevel, false);
    info!("post response: level_handler_set_state_next_level: {}", run_trigger.get(CheckStateRT::LevelHandlerSetStateNextLevel)); 
}

pub fn level_handler_set_state_next_map_set(
    mut run_trigger: ResMut<RunTrigger>,
    state_map_set: Res<State<StateMapSet>>,
    mut next_state_map_set: ResMut<NextState<StateMapSet>>,
){
    info!("function: level_handler_set_state_next_map_set"); 
    {
        match state_map_set.get() {
            StateMapSet::ToBeSelected => {
                info!("StateMapSet::Tutorial");
                next_state_map_set.set(StateMapSet::Tutorial);
            },
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
                info!("StateMapSet::ToBeSelected");
                next_state_map_set.set(StateMapSet::ToBeSelected);
            },
        };
    }
    run_trigger.set_target(CheckStateRT::LevelHandlerSetStateNextMapSet, false);
    info!("post response: level_handler_set_state_next_map_set: {}", run_trigger.get(CheckStateRT::LevelHandlerSetStateNextMapSet));  
}

pub fn level_handler_next_turn_protocol(
    mut run_trigger: ResMut<RunTrigger>,
) {
    info!("function: level_handler_next_turn_protocol"); 
    {
        run_trigger.set_target(CheckStateRT::LevelHandlerSetStateNextLevel, true);
        run_trigger.set_target(CheckStateRT::LevelHandlerPurgeProtocol, true);
    }
    run_trigger.set_target(CheckStateRT::LevelHandlerNextTurnProtocol, false);
    info!("post response: level_handler_next_turn_protocol: [{}]", run_trigger.get(CheckStateRT::LevelHandlerNextTurnProtocol));  
}

// When exiting state 
pub fn level_handler_purge_entity(
    commands: &mut Commands,
    entity: Entity,
) {
    // Access the rigid body from the physics world using its handle
    if commands.get_entity(entity).is_some() {
        commands.entity(entity).despawn();
    }
}

pub fn level_handler_purge_env_glb_all(
    mut commands: Commands,
    scene_meshes: Query<(Entity, &Name)>,
    mut purge_event_writer: EventWriter<SceneInstancePurgedEnvironment>,
) {
    info!("\n[ PURGING ENVIRONMENT!!! ---  PURGING ENVIRONMENT!!! ---  PURGING ENVIRONMENT!!! ]");
    for (entity, _) in scene_meshes.iter() {
        info!("Entity: [{:?}]", entity);
        level_handler_purge_entity(&mut commands, entity);
    }
    purge_event_writer.send(SceneInstancePurgedEnvironment{});
}

pub fn level_handler_purge_golf_ball_all(
    mut commands: Commands,
    mut golf_balls: Query<Entity, With<GolfBall>>, 
    mut purge_event_writer: EventWriter<SceneInstancePurgedGolfBalls>,
) {
    info!("\n[ PURGING GOLF BALLS !!! ---  PURGING GOLF BALLS !!! ---  PURGING GOLF BALLS !!! ]");
    for entity in golf_balls.iter_mut() {
        info!("Entity: [{:?}]", entity);
        level_handler_purge_entity(&mut commands, entity);
    }
    purge_event_writer.send(SceneInstancePurgedGolfBalls{});
}

impl PurgeHandler {
    pub fn new() -> Self {
        PurgeHandler{
            environment_purged: true,
            golf_balls_purged: true,
        }
    }
    
    pub fn get(&self, target: CheckStatePH) -> bool {
        match target {
            CheckStatePH::EnvironmentPurged => {
                self.environment_purged
            },
            CheckStatePH::GolfBallsPurged => {
                self.golf_balls_purged
            },
        }
    }

    pub fn set_target(&mut self, target: CheckStatePH, state: bool) {
        match target {
            CheckStatePH::EnvironmentPurged => {
                self.environment_purged = state;
                info!("response: environment_purged: {}", self.get(CheckStatePH::EnvironmentPurged));  
            },
            CheckStatePH::GolfBallsPurged => {
                self.golf_balls_purged = state;
                info!("response: golf_balls_purged: {}", self.get(CheckStatePH::GolfBallsPurged));  
            },
        }
    }
}


