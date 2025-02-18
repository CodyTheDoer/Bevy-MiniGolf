use bevy::prelude::*;

use bevy_rapier3d::{parry::shape::SharedShape, prelude::*};
use bevy_render::mesh::{Indices, VertexAttributeValues};

use uuid::Uuid;

// States
use crate::{
    StateArrow, 
    StateGamePlayStyle,
};

// Resources
use crate::{
    BonkHandler,
    CheckStateGH,
    CheckStateRT,
    GameHandler,
    GLBStorageID,
    GolfBall,
    GolfBallPosition,
    Interactable,
    Party,
    PhysicsHandler,
    RunTrigger,
    SceneInstanceOutOfBoundGolfBall,
    SceneInstanceRespawnedGolfBall,
    SceneInstancePurgedGolfBalls,
    SceneInstanceSpawnedGolfBalls,
    XYMatrix,
};

use crate::level_handler::level_handler::level_handler_purge_golf_ball_all;

impl BonkHandler {
    pub fn new() -> Self {
        let direction: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let power: f32 = 0.0;
        let cursor_origin_position: XYMatrix = XYMatrix::new();
        let cursor_origin_position_updated: bool = false;
        let cursor_bonk_position: XYMatrix = XYMatrix::new();
        let cursor_bonk_position_updated: bool = false;
        BonkHandler {
            direction,
            power,
            cursor_origin_position,
            cursor_origin_position_updated,
            cursor_bonk_position,
            cursor_bonk_position_updated,
        }
    }

    pub fn set_cursor_updated(&mut self) {
        self.cursor_origin_position_updated = false;
        self.cursor_bonk_position_updated = false;
    }

    pub fn update_cursor_bonk_position(
        &mut self, 
        bonk_coords: XYMatrix
    ) {
        self.cursor_bonk_position = bonk_coords;
        self.cursor_bonk_position_updated = true;
    }

    pub fn update_cursor_origin_position(
        &mut self, 
        bonk_coords: XYMatrix
    ) {
        self.cursor_origin_position = bonk_coords;
        self.cursor_origin_position_updated = true;
    }

    pub fn update_direction(&mut self, direction: &Vec3) {
        self.direction = *direction;
    }

    pub fn update_power(&mut self, power: f32) {
        self.power = power;
    }
}

impl PhysicsHandler {
    pub fn new() -> Self {
        PhysicsHandler
    }
}

pub fn add_physics_query_and_update_scene(
    party: Res<Party>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    scene_meshes: Query<(Entity, &Name, &Handle<Mesh>)>,
    mut gb_query: Query<(Entity, &mut GolfBall)>,
    mut run_trigger: ResMut<RunTrigger>,
    mut game_handler: ResMut<GameHandler>,
) {
    game_handler.add_physics_attempts_add_one();
    if game_handler.add_physics_attempts_get() >= 3 {
        // run load map again and reset count
        run_trigger.set_target(CheckStateRT::LevelHandlerInitLevelGameHandlerCurrentLevel, true);
        game_handler.add_physics_attempts_reset();
    }
    info!("function: add_physics_query_and_update_scene: Env Loaded: [{}]", game_handler.get(CheckStateGH::EnvironmentLoaded)); 
    if game_handler.get(CheckStateGH::EnvironmentLoaded) && game_handler.get(CheckStateGH::GolfBallsLoaded) {
        {
            commands
                .spawn(Collider::cylinder(0.1, 2000.0))
                .insert(TransformBundle::from(Transform::from_xyz(0.0, -10.0, 0.0)))
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(Sensor)
                .insert(Name::new("ground_sensor"));

            let players = party.all_players_get_ids();
            for (idx, (entity, golf_ball)) in gb_query.iter_mut().enumerate() {
                for player in &players {
                    if player == &golf_ball.0.uuid {
                        let collider = Collider::ball(0.022);
                        commands
                            .entity(entity)
                            .insert(collider)
                            .insert(RigidBody::Dynamic)
                            .insert(Damping {
                                angular_damping: 3.0,
                                ..default()
                            })
                            .insert(ExternalImpulse::default())
                            .insert(ColliderMassProperties::Density(1.0))
                            .insert(GravityScale(1.0))
                            .insert(Ccd::enabled())
                            .insert(TransformBundle::from(Transform::from_xyz(0.05 * (idx as f32), 0.0, 0.0)))
                            .insert(Name::new(format!("golf_ball_{}", player.to_string())));
                        info!("Built Golf Ball: [{}]", format!("golf_ball_{}", player.to_string()));
                    }
                }
            }

            // iterate over all meshes in the scene and match them by their name.
            for (entity, name, mesh_handle) in scene_meshes.iter() {
                if name.as_str() == "cup" {
                    // Create the collider from the mesh.
                    let mesh = meshes.get(&mesh_handle.clone()).unwrap();
                    let collider = Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap();
                    // Attach collider to the entity of this same object.
                    commands
                        .entity(entity)
                        .insert(collider);
                    info!("Built Cup collider from mesh...");
                }
                if name.as_str() == "cup_sensor" {
                    let collider = Collider::cuboid(0.04, 0.01, 0.04);
                    // Attach collider to the entity of this same object.
                    commands
                        .entity(entity)
                        // .insert(material_color.into())
                        .insert(collider)
                        .insert(ActiveEvents::COLLISION_EVENTS)
                        .insert(Sensor);
                    info!("Built Cup Sensor...");
                }
                if name.as_str() == "green" {
                    let mesh = meshes.get(&mesh_handle.clone()).unwrap();

                    let mut flags = TriMeshFlags::default();
                    flags.set(TriMeshFlags::FIX_INTERNAL_EDGES, true);

                    let (vtx, idx) =
                            extract_mesh_vertices_indices(mesh).unwrap();
                    let collider: Collider = SharedShape::trimesh_with_flags(vtx, idx, flags).into();

                    // Create the collider from the mesh.
                    // let collider = Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap();

                    // Attach collider to the entity of this same object.
                    commands
                        .entity(entity)
                        .insert(collider)
                        .insert(RigidBody::Fixed);
                    info!("Built Rigid Body green...");
                }
                if name.as_str() == "cannon" {
                    let mesh = meshes.get(&mesh_handle.clone()).unwrap();
                    // Create the collider from the mesh.
                    let collider = Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap();
                    // Attach collider to the entity of this same object.
                    commands
                        .entity(entity)
                        .insert(collider);
                    info!("Built Cannon...");
                }
            }
        }
        run_trigger.set_target(CheckStateRT::AddPhysicsQueryAndUpdateScene, false);
        info!("post response: add_physics_query_and_update_scene: [{}]", run_trigger.get(CheckStateRT::AddPhysicsQueryAndUpdateScene));
    }  
}

pub fn bonk(
    mut run_trigger: ResMut<RunTrigger>,
    entity: Entity,
    mut commands: Commands,
    bonk: Res<BonkHandler>,
    playstyle: Res<State<StateGamePlayStyle>>,
    game_handler: &mut ResMut<GameHandler>,
) {
    let scaled_bonk = bonk.power * 0.00025;
    info!("bonk: [{}]", scaled_bonk);
    commands.entity(entity)
        .insert(ExternalImpulse {
            impulse: bonk.direction * scaled_bonk,
            torque_impulse: Vec3::new(0.0, 0.0, 0.0),
        }
    );   
    run_trigger.set_target(CheckStateRT::PartyHandlerActivePlayerAddBonk, true); 
    match playstyle.get() {
        StateGamePlayStyle::SetOrder => {
            game_handler.set_target(CheckStateGH::AllSleeping, false);
            run_trigger.set_target(CheckStateRT::StartMovementListenerTurnHandlerSetTurnNext, true);
        }
        StateGamePlayStyle::Proximity => {}
    }
}

pub fn bonk_step_start( // set's bonk start xy
    windows: Query<&Window>,
    mut bonk: ResMut<BonkHandler>,
    mut game_handler: ResMut<GameHandler>,
    arrow_state: ResMut<State<StateArrow>>,
    next_arrow_state: ResMut<NextState<StateArrow>>,
) {
    let mut cursor_xy: XYMatrix = XYMatrix::new();
    let Some(position) = windows.single().cursor_position() else {
        return;
    };
    cursor_xy.set(position.x, position.y);
    bonk.update_cursor_origin_position(cursor_xy);
    match arrow_state.get() {
        StateArrow::Idle => {
            toggle_arrow_state(&mut game_handler, arrow_state, next_arrow_state);
        },
        _ => {},
    }
}

pub fn bonk_step_mid( // Determines bonks power by measuring the difference between origin and current mouse xy
    mut bonk_res: ResMut<BonkHandler>,
    windows: Query<&Window>,
    mut golf_balls: Query<&GolfBall>,
    party: Res<Party>,
) {
    for golf_ball in golf_balls.iter_mut() {
        if golf_ball.0.uuid == party.active_player_get_player_id() {
            let mut cursor_xy: XYMatrix = XYMatrix::new();
            let Some(position) = windows.single().cursor_position() else {
                return;
            };
            let window_width: f32 = windows.single().width();
            let window_height: f32 = windows.single().height();
            cursor_xy.set(position.x, position.y);
            bonk_res.update_cursor_bonk_position(cursor_xy);
        
            // find length of pixels from origin to release
            let difference_x = bonk_res.cursor_origin_position.x - bonk_res.cursor_bonk_position.x;
            let difference_y = bonk_res.cursor_origin_position.y - bonk_res.cursor_bonk_position.y;
        
            // Compute Euclidean distance between origin and current cursor position
            let distance = (difference_x.powi(2) + difference_y.powi(2)).sqrt();
        
            // Calculate the maximum possible distance (screen diagonal)
            let max_distance = (window_width.powi(2) + window_height.powi(2)).sqrt();
        
            // Normalize power based on distance ratio
            let mut bonk_power = distance / max_distance;
            
            // adjust bonk power to always deliver a positive value
            if bonk_power < 0.0 {
                bonk_power *= -1.0;
            };
        
            // shrinking the drag length to set power by doubling the value, 
            // anything drag 50% of the screen or over will deliver 100%
            if bonk_power >= 0.25 {
                bonk_power = 1.0;
            } else {
                bonk_power *= 4.0;
            };
            
            bonk_res.power = bonk_power;
            bonk_res.set_cursor_updated();      
        }
    }
}

pub fn bonk_step_end( // Fires bonk 
    buttons: Res<ButtonInput<MouseButton>>,
    mut game_handler: ResMut<GameHandler>,
    arrow_state: ResMut<State<StateArrow>>,
    next_arrow_state: ResMut<NextState<StateArrow>>,
    bonk_res: Res<BonkHandler>,
    rapier_context: Res<RapierContext>,
    rigid_body_query: Query<(Entity, &Name, &RapierRigidBodyHandle)>,
    commands: Commands,
    party: Res<Party>,
    run_trigger: ResMut<RunTrigger>,
    playstyle: Res<State<StateGamePlayStyle>>,
    golf_balls: Query<(Entity, &mut GolfBall, &Name)>,
) {
    if game_handler.get(CheckStateGH::ArrowState) {
        toggle_arrow_state(&mut game_handler, arrow_state, next_arrow_state);
    }

    let mut target_entity: Option<Entity> = None;
    let player = party.active_player_get_player_id();
    for (entity, golf_ball, _) in golf_balls.iter() {
        if buttons.just_released(MouseButton::Right) {
            if player == golf_ball.0.uuid{
                let owned_bonk_power = bonk_res.power.clone();
                if owned_bonk_power != 0.0 {
                    target_entity = Some(entity);
                }
            };
        }
    }

    if target_entity.is_some() {
        if golf_ball_is_asleep(rapier_context, rigid_body_query, golf_balls, &mut game_handler) {
            bonk(run_trigger, target_entity.unwrap(), commands, bonk_res.into(), playstyle, &mut game_handler);
        }
    }
}

pub fn collision_events_listener(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut party: ResMut<Party>,
    mut respawn_event_writer: EventWriter<SceneInstanceOutOfBoundGolfBall>,
    mut run_trigger: ResMut<RunTrigger>,
    mut game_handler: ResMut<GameHandler>,
    golf_balls: Query<(Entity, &GolfBall)>,
    scene_meshes: Query<(Entity, &Name)>,
) {
    let mut out_of_bounds = false;
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _flags) => {
                // info!("Collision started between {:?} and {:?}", entity1, entity2);
                for (entity, name) in &scene_meshes {
                    let owned_name = name.as_str();
                    if *entity1 == entity {
                        for (golf_ball_ent, golf_ball) in golf_balls.iter() {
                            if *entity2 == golf_ball_ent {
                                match owned_name {
                                    "cup_sensor" => {
                                        info!("1: Cups baby!!!!!");
                                        info!("1: Golf Ball: [{:?}]", golf_ball.0);
                                        party.player_set_hole_completion_state(golf_ball.0.uuid, true);
                                        commands.entity(golf_ball_ent).despawn();
                                        run_trigger.set_target(CheckStateRT::StartMovementListenerTurnHandlerSetTurnNext, false);
                                        run_trigger.set_target(CheckStateRT::TurnHandlerSetTurnNext, true);
                                    },
                                    "ground_sensor" => {
                                        info!("1: Ooof grounded...");
                                        info!("2: Golf Ball: [{:?}]", golf_ball.0);
                                        out_of_bounds = true;
                                    },
                                    _ => {},
                                }
                            }
                        }
                    }
                    if *entity2 == entity {
                        for (golf_ball_ent, golf_ball) in golf_balls.iter() {
                            if *entity1 == golf_ball_ent {
                                match owned_name {
                                    "cup_sensor" => {
                                        info!("2: Cups baby!!!!!");
                                        info!("1: Golf Ball: [{:?}]", golf_ball.0);
                                        party.player_set_hole_completion_state(golf_ball.0.uuid, true);
                                        commands.entity(golf_ball_ent).despawn();
                                        run_trigger.set_target(CheckStateRT::StartMovementListenerTurnHandlerSetTurnNext, false);
                                        run_trigger.set_target(CheckStateRT::TurnHandlerSetTurnNext, true);
                                    },
                                    "ground_sensor" => {
                                        info!("1: Ooof grounded...");
                                        info!("2: Golf Ball: [{:?}]", golf_ball.0);
                                        out_of_bounds = true;
                                    },
                                    _ => {},
                                }
                            }
                        }
                    }
                    
                }
            }
            CollisionEvent::Stopped(entity1, entity2, _flags) => {
                info!("Collision stopped between {:?} and {:?}", entity1, entity2);
            }
        }
    }
    if out_of_bounds == true {
        game_handler.set_target(CheckStateGH::GolfBallsBonkTrigger, false);
        let mut info_vec: Vec<(Uuid, Vec3)> = Vec::new();
        
        for (_, golf_ball) in golf_balls.iter() {
            info_vec.push((golf_ball.0.uuid, golf_ball.0.last_position));
        };
        for (golf_ball_ent, _) in golf_balls.iter() {
            commands.entity(golf_ball_ent).despawn();
        };

        info!("out_of_bounds: respawn_event_writer[InfoVec] \n {:?}", info_vec);
        
        respawn_event_writer.send(SceneInstanceOutOfBoundGolfBall {
            info_vec: info_vec,
        });        
    };
}

// Helper function for ^^^add_physics_query_and_update_scene^^^
fn extract_mesh_vertices_indices( 
    mesh: &Mesh,
) -> Option<(
    Vec<bevy_rapier3d::na::Point3<bevy_rapier3d::prelude::Real>>,
    Vec<[u32; 3]>,
)> {
    use bevy_rapier3d::math::Real;
    use bevy_rapier3d::na::Point3;

    let vertices = mesh.attribute(Mesh::ATTRIBUTE_POSITION)?;
    let indices = mesh.indices()?;

    let vtx: Vec<_> = match vertices {
        VertexAttributeValues::Float32(vtx) => Some(
            vtx.chunks(3)
                .map(|v| Point3::new(v[0] as Real, v[1] as Real, v[2] as Real))
                .collect(),
        ),
        VertexAttributeValues::Float32x3(vtx) => Some(
            vtx.iter()
                .map(|v| Point3::new(v[0] as Real, v[1] as Real, v[2] as Real))
                .collect(),
        ),
        _ => None,
    }?;

    let idx = match indices {
        Indices::U16(idx) => idx
            .chunks_exact(3)
            .map(|i| [i[0] as u32, i[1] as u32, i[2] as u32])
            .collect(),
        Indices::U32(idx) => idx.chunks_exact(3).map(|i| [i[0], i[1], i[2]]).collect(),
    };

    Some((vtx, idx))
}

pub fn golf_ball_handler_update_locations_post_bonk(
    mut run_trigger: ResMut<RunTrigger>,
    party: ResMut<Party>,
    mut gb_query: Query<(&mut GolfBall, &Transform)>,
    mut game_handler: ResMut<GameHandler>,
) {
    info!("function: golf_ball_handler_update_locations_post_bonk "); 
    {
        if game_handler.get(CheckStateGH::GolfBallsBonkTrigger) {
            info!("golf_ball_handler_update_locations_post_bonk: ");
            let player_id = party.active_player_get_player_id();
            for (mut golf_ball, transform) in gb_query.iter_mut() {
                info!("golf_ball before: [{:?}]", golf_ball.0);
                if golf_ball.0.uuid == player_id {
                    golf_ball.0.position = transform.translation;
                };
                info!("golf_ball after: [{:?}]", golf_ball.0);
            };
            run_trigger.set_target(CheckStateRT::GolfBallHandlerUpdateLocationsPostBonk, false);
            info!("post response: golf_ball_handler_update_locations_post_bonk: {}", run_trigger.get(CheckStateRT::GolfBallHandlerUpdateLocationsPostBonk));  
        } else {
            game_handler.set_target(CheckStateGH::GolfBallsBonkTrigger, true);
        }
    }
}

pub fn golf_ball_handler_end_game(
    commands: Commands,
    mut run_trigger: ResMut<RunTrigger>,
    golf_balls: Query<Entity, With<GolfBall>>,
    purge_event_writer: EventWriter<SceneInstancePurgedGolfBalls>,
) {
    info!("function: golf_ball_handler_end_game "); 
    {
        level_handler_purge_golf_ball_all(commands, golf_balls, purge_event_writer);
    }
    run_trigger.set_target(CheckStateRT::GolfBallHandlerEndGame, false);
    info!("post response: golf_ball_handler_end_game: {}", run_trigger.get(CheckStateRT::GolfBallHandlerEndGame));  
}

// Helper: golf_ball_handler_spawn_golf_balls_for_party_members
fn golf_ball_handler_init_golf_ball_uuid(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    glb_storage: &Res<GLBStorageID>, //Arc<[MapID]> //map: Arc<str>,
    player_id: &Uuid,
    asset_event_writer: &mut EventWriter<SceneInstanceSpawnedGolfBalls>,
) {
    if let Some(basic_golf_ball) = glb_storage.glb.get(25) {
        let basic_golf_ball_handle: Handle<Scene> = asset_server.load(
            GltfAssetLabel::Scene(0).from_asset(basic_golf_ball.map),
        );
        let name = format!("golf_ball_{}", player_id.to_string());
        info!("Generated Name: {}", name);
        let spawned_golf_ball = commands
            .spawn((
                SceneBundle {
                    scene: basic_golf_ball_handle.clone(),
                    ..default()
                },
                Name::new(name.clone()),
            ))
            .insert(Interactable)
            .insert(GolfBall(GolfBallPosition{
                uuid: *player_id,
                position: Vec3::ZERO,
                last_position: Vec3::ZERO,
                sleeping: false,
            }))
            .id();

            // Emit a custom AssetEvent for this asset
            asset_event_writer.send(SceneInstanceSpawnedGolfBalls {
                    entity: spawned_golf_ball,
                }
            );
    } else {
        warn!("Target was not valid. Refer to the GLBStorageID map in the library.");
    };
}

// Helper: golf_ball_handler_spawn_golf_balls_for_party_members
pub fn golf_ball_handler_respawn_golf_ball_uuid(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    glb_storage: &Res<GLBStorageID>, //Arc<[MapID]> //map: Arc<str>,
    player_id: &Uuid,
    player_position: &Vec3,
    asset_event_writer: &mut EventWriter<SceneInstanceRespawnedGolfBall>,
    game_handler: &mut ResMut<GameHandler>,
) {
    info!("golf_ball_handler_respawn_golf_ball_uuid: [{:?}]::[{:?}]", player_id, player_position);
    if let Some(basic_golf_ball) = glb_storage.glb.get(25) {
        let basic_golf_ball_handle: Handle<Scene> = asset_server.load(
            GltfAssetLabel::Scene(0).from_asset(basic_golf_ball.map),
        );
        let name = format!("golf_ball_{}", player_id.to_string());
        info!("Generated Name: {}", name);
        let spawned_golf_ball = commands
            .spawn((
                SceneBundle {
                    scene: basic_golf_ball_handle.clone(),
                    // transform: Transform::from_xyz(player_position.x, player_position.y, player_position.y),
                    ..default()
                },
                Name::new(name.clone()),
            ))
            .insert(Interactable)
            .insert(GolfBall(GolfBallPosition{
                uuid: *player_id,
                position: *player_position,
                last_position: *player_position,
                sleeping: false,
            }))
            // .insert(TransformBundle::from(Transform::from_xyz(player_position.x, player_position.y, player_position.y)))
            .id();

        info!("Generated Golf Ball: {}", spawned_golf_ball);
        info!("Generated Event: [SceneInstanceRespawnedGolfBall]");

        // Emit a custom AssetEvent for this asset
        asset_event_writer.send(SceneInstanceRespawnedGolfBall {
                entity: spawned_golf_ball,
                id: *player_id,
                location: *player_position,
            }
        );
        
        game_handler.set_target(CheckStateGH::GolfBallsStoreLocation, false);
    } else {
        warn!("Target was not valid. Refer to the GLBStorageID map in the library.");
    };
}

pub fn golf_ball_handler_party_store_locations(
    mut run_trigger: ResMut<RunTrigger>,
    mut gb_query: Query<&mut GolfBall>,
    mut game_handler: ResMut<GameHandler>,
) {
    info!("function: golf_ball_handler_party_store_locations "); 
    {
        if game_handler.get(CheckStateGH::GolfBallsStoreLocation) {
            for mut golf_ball in gb_query.iter_mut() {
                info!("golf_ball before: [{:?}]", golf_ball.0);
                golf_ball.0.last_position = golf_ball.0.position;
                info!("golf_ball after: [{:?}]", golf_ball.0);
            };
        } else {
            // This is a single fire trigger to stop the storing of location for reset golfballs.
            game_handler.set_target(CheckStateGH::GolfBallsStoreLocation, true);
        };
    }
    run_trigger.set_target(CheckStateRT::GolfBallHandlerPartyStoreLocations, false);
    info!("post response: golf_ball_handler_party_store_locations: {}", run_trigger.get(CheckStateRT::GolfBallHandlerPartyStoreLocations));  
}

pub fn golf_ball_handler_reset_golf_ball_locations(
    mut run_trigger: ResMut<RunTrigger>,
    mut gb_query: Query<(&GolfBall, &mut Transform)>,
) {
    info!("function: golf_ball_handler_reset_golf_ball_locations "); 
    {
        for (golf_ball, mut transform) in gb_query.iter_mut() {
            transform.translation = golf_ball.0.last_position;
            info!("golf_ball: [{:?}]", golf_ball.0);
        };
    }
    run_trigger.set_target(CheckStateRT::GolfBallHandlerResetGolfBallLocations, false);
    info!("post response: golf_ball_handler_reset_golf_ball_locations: {}", run_trigger.get(CheckStateRT::GolfBallHandlerResetGolfBallLocations));  
}

pub fn golf_ball_handler_spawn_golf_balls_for_party_members(
    mut commands: Commands,
    mut run_trigger: ResMut<RunTrigger>,
    party: ResMut<Party>,
    asset_server: Res<AssetServer>,
    glb_storage: Res<GLBStorageID>, //Arc<[MapID]> //map: Arc<str>,
    mut asset_event_writer: EventWriter<SceneInstanceSpawnedGolfBalls>,
    gb_query: Query<(Entity, &Name)>,
) {
    info!("function: golf_ball_handler_spawn_golf_balls_for_party_members"); 
    {
        for player in party.all_players_get_ids().iter() {
            info!("Building Golf Ball for player: [{:?}]", &player);
            golf_ball_handler_init_golf_ball_uuid(
                &mut commands,
                &asset_server,
                &glb_storage,
                &player,
                &mut asset_event_writer,
            );

            for (golf_ball, name) in gb_query.iter() {
                if name.as_str() == "ball" {
                    info!("name: [{:?}], Entity: [{:?}]", &name, &golf_ball);
                    commands.entity(golf_ball).insert(Name::new(format!("ball_{}", player.to_string())));
                    info!("name: [{:?}], Entity: [{:?}]", &name, &golf_ball);
                }
            };
        };   
    }
    run_trigger.set_target(CheckStateRT::GolfBallHandlerSpawnGolfBallsForPartyMembers, false);
    info!("post response: golf_ball_handler_spawn_golf_balls_for_party_members: {}", run_trigger.get(CheckStateRT::GolfBallHandlerSpawnGolfBallsForPartyMembers));  
}

pub fn golf_balls_update_sleep_status(
    rapier_context: Res<RapierContext>,
    query: Query<(Entity, &Name, &RapierRigidBodyHandle)>,
    mut golf_balls: Query<(Entity, &mut GolfBall, &Name)>,
) {
    for (_entity, name, rb_handle) in query.iter() {
        // Access the rigid body from the physics world using its handle
        if let Some(rigid_body) = rapier_context.bodies.get(rb_handle.0) {
            // Check if the rigid body is currently sleeping
            if rigid_body.is_sleeping() {
                // println!("Entity {:?} is sleeping", entity);
                for (_, mut golf_ball, golf_ball_name) in golf_balls.iter_mut() {
                    if name == golf_ball_name {
                        golf_ball.0.sleeping = true
                    }
                }
            } else {
                for (_, mut golf_ball, golf_ball_name) in golf_balls.iter_mut() {
                    if name == golf_ball_name {
                        golf_ball.0.sleeping = false
                    }
                }
            }
        }
    }
}

pub fn golf_ball_is_asleep(
    rapier_context: Res<RapierContext>,
    query: Query<(Entity, &Name, &RapierRigidBodyHandle)>,
    mut golf_balls: Query<(Entity, &mut GolfBall, &Name)>,
    game_handler: &mut ResMut<GameHandler>,
) -> bool {
    for (_entity, name, rb_handle) in query.iter() {
        // Access the rigid body from the physics world using its handle
        if let Some(rigid_body) = rapier_context.bodies.get(rb_handle.0) {
            // Check if the rigid body is currently sleeping
            if rigid_body.is_sleeping() {
                // println!("Entity {:?} is sleeping", entity);
                for (_, mut golf_ball, golf_ball_name) in golf_balls.iter_mut() {
                    if name == golf_ball_name {
                        golf_ball.0.sleeping = true
                    }
                }
            } else {
                for (_, mut golf_ball, golf_ball_name) in golf_balls.iter_mut() {
                    if name == golf_ball_name {
                        golf_ball.0.sleeping = false
                    }
                }
            }
        }
    }
    game_handler.get(CheckStateGH::AllSleeping)
}

pub fn performance_physics_setup(mut rapier_config: ResMut<RapierConfiguration>) {
    // Set fixed timestep mode
    rapier_config.timestep_mode = TimestepMode::Fixed {
        dt: 1.0 / 60.0,       // Physics update rate
        substeps: 12,          // Number of physics steps per frame
    };

    // Enable/disable physics systems
    rapier_config.physics_pipeline_active = true;  // Enable physics simulation
    rapier_config.query_pipeline_active = true;    // Enable collision detection queries
    
    // Gravity configuration
    rapier_config.gravity = Vec3::new(0.0, -9.81, 0.0); // Standard gravity
}

fn toggle_arrow_state(
    game_handler: &mut ResMut<GameHandler>,
    state: ResMut<State<StateArrow>>,
    mut next_state: ResMut<NextState<StateArrow>>,
) {
    match state.get() {
        StateArrow::DrawingArrow => {
            info!("Entering StateArrow::Idle");
            game_handler.set_target(CheckStateGH::ArrowState, false);
            next_state.set(StateArrow::Idle);
        },
        StateArrow::Idle => {
            info!("Entering StateArrow::DrawingArrow");
            game_handler.set_target(CheckStateGH::ArrowState, true);
            next_state.set(StateArrow::DrawingArrow);
        },
    }
}

impl XYMatrix {
    pub fn new() -> Self {
        let x: f32 = 0.0;
        let y: f32 = 0.0;
        XYMatrix {
            x,
            y,
        }
    }
    
    pub fn set(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}