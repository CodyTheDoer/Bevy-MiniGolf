use bevy::ecs::entity;
use bevy::prelude::*;

use bevy_rapier3d::{parry::shape::SharedShape, prelude::*};
use bevy_render::mesh::{Indices, VertexAttributeValues};

use uuid::Uuid;

// States
use crate::{
    StateArrow, 
    StateGamePlayStyle,
    StateTurn,
};

// Resources
use crate::{
    BonkHandler,
    BonkMouseXY,
    GameHandler,
    GLBStorageID,
    GolfBall,
    GolfBallPosition,
    Interactable,
    Party,
    PhysicsHandler,
    PurgeHandler,
    RunTrigger,
    SceneInstancePurgedGolfBalls,
    SceneInstanceSpawned,
};

use crate::level_handler::level_handler::level_handler_purge_golf_ball_all;

impl BonkHandler {
    pub fn new() -> Self {
        let direction: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let power: f32 = 0.0;
        let cursor_origin_position: BonkMouseXY = BonkMouseXY::new();
        let cursor_origin_position_updated: bool = false;
        let cursor_bonk_position: BonkMouseXY = BonkMouseXY::new();
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
        bonk_coords: BonkMouseXY
    ) {
        self.cursor_bonk_position = bonk_coords;
        self.cursor_bonk_position_updated = true;
    }

    pub fn update_cursor_origin_position(
        &mut self, 
        bonk_coords: BonkMouseXY
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

impl BonkMouseXY {
    pub fn new() -> Self {
        let x: f32 = 0.0;
        let y: f32 = 0.0;
        BonkMouseXY {
            x,
            y,
        }
    }
    
    pub fn set(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
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
) {
    info!("function: add_physics_query_and_update_scene"); 
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
                    .insert(TransformBundle::from(Transform::from_xyz(0.05 * (idx as f32), 0.0, 0.0)));
                    // .insert(Name::new(format!("ball_{}", player.to_string())));
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
        }
        if name.as_str() == "cannon" {
            let mesh = meshes.get(&mesh_handle.clone()).unwrap();
            // Create the collider from the mesh.
            let collider = Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap();
            // Attach collider to the entity of this same object.
            commands
                .entity(entity)
                .insert(collider);
        }
    }
    run_trigger.set_target("add_physics_query_and_update_scene", false);
    info!("post response: add_physics_query_and_update_scene: [{}]", run_trigger.get("add_physics_query_and_update_scene"));  
}

pub fn bonk(
    mut run_trigger: ResMut<RunTrigger>,
    entity: Entity,
    mut commands: Commands,
    bonk: Res<BonkHandler>,
    playstyle: Res<State<StateGamePlayStyle>>,
) {
    let scaled_bonk = bonk.power * 0.00025;
    info!("bonk: [{}]", scaled_bonk);
    commands.entity(entity)
        .insert(ExternalImpulse {
            impulse: bonk.direction * scaled_bonk,
            torque_impulse: Vec3::new(0.0, 0.0, 0.0),
        }
    );   
    run_trigger.set_target("party_handler_active_player_add_bonk", true); 
    match playstyle.get() {
        StateGamePlayStyle::SetOrder => {
            run_trigger.set_target("turn_handler_set_turn_next", true);
        }
        StateGamePlayStyle::Proximity => {}
    }
}

pub fn bonk_step_start( // set's bonk start xy
    windows: Query<&Window>,
    mut bonk: ResMut<BonkHandler>,
    gh: ResMut<GameHandler>,
    arrow_state: ResMut<State<StateArrow>>,
    next_arrow_state: ResMut<NextState<StateArrow>>,
) {
    let mut cursor_xy: BonkMouseXY = BonkMouseXY::new();
    let Some(position) = windows.single().cursor_position() else {
        return;
    };
    cursor_xy.set(position.x, position.y);
    bonk.update_cursor_origin_position(cursor_xy);
    match arrow_state.get() {
        StateArrow::Idle => {
            toggle_arrow_state(gh, arrow_state, next_arrow_state);
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
        info!("Searching for match: [{}]", golf_ball.0.uuid);
        if golf_ball.0.uuid == party.active_player_get_player_id() {
            let mut cursor_xy: BonkMouseXY = BonkMouseXY::new();
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
    gh: ResMut<GameHandler>,
    arrow_state: ResMut<State<StateArrow>>,
    next_arrow_state: ResMut<NextState<StateArrow>>,
    bonk_res: Res<BonkHandler>,
    rapier_context: Res<RapierContext>,
    rigid_body_query: Query<&RapierRigidBodyHandle>,
    mut golf_balls: Query<(Entity, &GolfBall)>,
    commands: Commands,
    party: Res<Party>,
    run_trigger: ResMut<RunTrigger>,
    playstyle: Res<State<StateGamePlayStyle>>,
) {
    if gh.arrow_state_get() {
        toggle_arrow_state(gh, arrow_state, next_arrow_state);
    }

    let mut target_entity: Option<Entity> = None;
    let player = party.active_player_get_player_id();
    for (entity, golf_ball) in golf_balls.iter_mut() {
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
        if golf_ball_is_asleep(rapier_context, rigid_body_query) {
            bonk(run_trigger, target_entity.unwrap(), commands, bonk_res.into(), playstyle);
        }
    }
}

pub fn collision_events_listener(
    mut collision_events: EventReader<CollisionEvent>,
    scene_meshes: Query<(Entity, &Name)>,
    // mut next_turn_state: ResMut<NextState<StateTurn>>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _flags) => {
                // info!("Collision started between {:?} and {:?}", entity1, entity2);
                for (entity, name) in &scene_meshes {
                    let owned_name = name.as_str();
                    if *entity1 == entity {
                        match owned_name {
                            "cup_sensor" => {
                                info!("1: Cups baby!!!!");
                                // next_turn_state.set(StateTurn::HoleComplete);
                            },
                            "ground_sensor" => {
                                info!("1: Ooof grounded...");
                                // next_turn_state.set(StateTurn::TurnReset);
                            },
                            _ => {},
                        }
                    }
                    if *entity2 == entity {
                        match owned_name {
                            "cup_sensor" => {
                                info!("1: Cups baby!!!!");
                                // next_turn_state.set(StateTurn::HoleComplete);
                            },
                            "ground_sensor" => {
                                info!("1: Ooof grounded...");
                                // next_turn_state.set(StateTurn::TurnReset);
                            },
                            _ => {},
                        }
                    }
                    
                }
            }
            CollisionEvent::Stopped(entity1, entity2, _flags) => {
                // info!("Collision stopped between {:?} and {:?}", entity1, entity2);
            }
        }
    }
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

pub fn golf_ball_handler_active_player_manual_bonk(
    mut run_trigger: ResMut<RunTrigger>,
    party: ResMut<Party>,
    mut gb_query: Query<&mut GolfBall>,
) {
    info!("function: golf_ball_handler_active_player_manual_bonk "); 
    {
        let player_id = party.active_player_get_player_id();
        for mut golf_ball in gb_query.iter_mut() {
            if golf_ball.0.uuid == player_id {
                golf_ball.0.position = golf_ball.0.position + Vec3::new(5.0, 5.0, 5.0);
            };
            info!("golf_ball: [{:?}]", golf_ball.0);
        };
        run_trigger.set_target("golf_ball_handler_party_store_locations", true);
    }
    run_trigger.set_target("golf_ball_handler_active_player_manual_bonk", false);
    info!("post response: golf_ball_handler_active_player_manual_bonk: {}", run_trigger.get("golf_ball_handler_active_player_manual_bonk"));  
}

pub fn golf_ball_handler_end_game(
    commands: Commands,
    mut run_trigger: ResMut<RunTrigger>,
    golf_balls: Query<Entity, With<GolfBall>>,
    purge_event_writer: EventWriter<SceneInstancePurgedGolfBalls>,
    mut purge_handler: ResMut<PurgeHandler>,
) {
    info!("function: golf_ball_handler_end_game "); 
    {
        level_handler_purge_golf_ball_all(commands, golf_balls, purge_event_writer, &mut purge_handler);
    }
    run_trigger.set_target("golf_ball_handler_end_game", false);
    info!("post response: golf_ball_handler_end_game: {}", run_trigger.get("golf_ball_handler_end_game"));  
}

// Helper: golf_ball_handler_spawn_golf_balls_for_party_members
fn golf_ball_handler_init_golf_ball_uuid(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    glb_storage: &Res<GLBStorageID>, //Arc<[MapID]> //map: Arc<str>,
    player_id: &Uuid,
    asset_event_writer: &mut EventWriter<SceneInstanceSpawned>,
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
            }))
            .id();

            // Emit a custom AssetEvent for this asset
            asset_event_writer.send(SceneInstanceSpawned {
                    entity: spawned_golf_ball,
                }
            );
    } else {
        warn!("Target was not valid. Refer to the GLBStorageID map in the library.");
    };
}

pub fn golf_ball_handler_party_store_locations(
    mut run_trigger: ResMut<RunTrigger>,
    mut gb_query: Query<&mut GolfBall>,
) {
    info!("function: golf_ball_handler_party_store_locations "); 
    {
        for mut golf_ball in gb_query.iter_mut() {
            golf_ball.0.last_position = golf_ball.0.position;
            info!("golf_ball: [{:?}]", golf_ball.0);
        };
    }
    run_trigger.set_target("golf_ball_handler_party_store_locations", false);
    info!("post response: golf_ball_handler_party_store_locations: {}", run_trigger.get("golf_ball_handler_party_store_locations"));  
}

pub fn golf_ball_handler_reset_golf_ball_locations(
    mut run_trigger: ResMut<RunTrigger>,
    mut gb_query: Query<&mut GolfBall>,
) {
    info!("function: golf_ball_handler_reset_golf_ball_locations "); 
    {
        for mut golf_ball in gb_query.iter_mut() {
            info!("golf_ball: [{:?}]", golf_ball.0);
            golf_ball.0.position = golf_ball.0.last_position;
        };
    }
    run_trigger.set_target("golf_ball_handler_reset_golf_ball_locations", false);
    info!("post response: golf_ball_handler_reset_golf_ball_locations: {}", run_trigger.get("golf_ball_handler_reset_golf_ball_locations"));  
}

pub fn golf_ball_handler_spawn_golf_balls_for_party_members(
    mut commands: Commands,
    mut run_trigger: ResMut<RunTrigger>,
    party: ResMut<Party>,
    asset_server: Res<AssetServer>,
    glb_storage: Res<GLBStorageID>, //Arc<[MapID]> //map: Arc<str>,
    mut asset_event_writer: EventWriter<SceneInstanceSpawned>,
    gb_query: Query<(Entity, &Name)>,
    golf_balls: Query<(Entity, &GolfBall)>,
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

            /*
            level_handler::physics_handler: Building Golf Ball for player: [01938dc7-1def-7eb3-a771-000436e79280]
            level_handler::physics_handler: Generated Name: golf_ball_01938dc7-1def-7eb3-a771-000436e79280
            level_handler::physics_handler: name: ["ball"], Entity: [Entity { index: 210, generation: 1 }]

            ray_system_handler: Name: "ball" Entity: Entity { index: 374, generation: 2 }
            */

            for (golf_ball, name) in gb_query.iter() {
                if name.as_str() == "ball" {
                    info!("name: [{:?}], Entity: [{:?}]", &name, &golf_ball);
                    commands.entity(golf_ball).insert(Name::new(format!("ball_{}", player.to_string())));
                    info!("name: [{:?}], Entity: [{:?}]", &name, &golf_ball);
                }
            };
            
            // party.golf_ball_build_player(&player);
        };

        // info!("\n\nPlayer: [{:?}], Golf Ball: [{:?}]", player_id.to_owned(), golf_ball.0);
                        // commands.entity(spawned_golf_ball).push_children(&[entity]);
                        // commands.entity(parent.unwrap().get()).insert(Name::new(name.clone()));
        
    }
    run_trigger.set_target("golf_ball_handler_spawn_golf_balls_for_party_members", false);
    info!("post response: golf_ball_handler_spawn_golf_balls_for_party_members: {}", run_trigger.get("golf_ball_handler_spawn_golf_balls_for_party_members"));  
}

pub fn golf_ball_is_asleep(
    rapier_context: Res<RapierContext>,
    query: Query<&RapierRigidBodyHandle>,
) -> bool {
    let mut results = false;
    for rb_handle in query.iter() {
        // Access the rigid body from the physics world using its handle
        if let Some(rigid_body) = rapier_context.bodies.get(rb_handle.0) {
            // Check if the rigid body is currently sleeping
            if rigid_body.is_sleeping() {
                // println!("Entity {:?} is sleeping", entity);
                results = true;
            }
        }
    }       
    results
}

pub fn performance_physics_setup(mut rapier_config: ResMut<RapierConfiguration>) {
    // Set fixed timestep mode
    rapier_config.timestep_mode = TimestepMode::Fixed {
        dt: 1.0 / 60.0,       // Physics update rate
        substeps: 4,          // Number of physics steps per frame
    };

    // Enable/disable physics systems
    rapier_config.physics_pipeline_active = true;  // Enable physics simulation
    rapier_config.query_pipeline_active = true;    // Enable collision detection queries
    
    // Gravity configuration
    rapier_config.gravity = Vec3::new(0.0, -9.81, 0.0); // Standard gravity
}

fn toggle_arrow_state(
    mut gh: ResMut<GameHandler>,
    state: ResMut<State<StateArrow>>,
    mut next_state: ResMut<NextState<StateArrow>>,
) {
    match state.get() {
        StateArrow::DrawingArrow => {
            info!("Entering StateArrow::Idle");
            gh.arrow_state_set_false();
            next_state.set(StateArrow::Idle);
        },
        StateArrow::Idle => {
            info!("Entering StateArrow::DrawingArrow");
            gh.arrow_state_set_true();
            next_state.set(StateArrow::DrawingArrow);
        },
    }
}