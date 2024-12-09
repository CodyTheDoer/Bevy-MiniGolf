use bevy::prelude::*;

use bevy_mod_raycast::prelude::*;
use bevy_rapier3d::{parry::shape::SharedShape, prelude::*};
use bevy_render::mesh::{Indices, VertexAttributeValues};

use uuid::Uuid;

// States
use crate::{
    StateArrow, 
    StateLevel,
    StateTurn,
};

// Resources
use crate::{
    BonkHandler,
    BonkMouseXY,
    CameraWorld,
    GameHandler,
    Ground,
    GLBStorageID,
    GolfBall,
    GolfBallPosition,
    Interactable,
    Party,
    PhysicsHandler,
    RunTrigger,
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

    pub fn update_direction(&mut self, direction: &Vec3) {
        self.direction = *direction;
    }

    pub fn update_power(&mut self, power: f32) {
        self.power = power;
    }

    pub fn update_cursor_origin_position(
        &mut self, 
        bonk_coords: BonkMouseXY
    ) {
        self.cursor_origin_position = bonk_coords;
        self.cursor_origin_position_updated = true;
    }

    pub fn update_cursor_bonk_position(
        &mut self, 
        bonk_coords: BonkMouseXY
    ) {
        self.cursor_bonk_position = bonk_coords;
        self.cursor_bonk_position_updated = true;
    }

    pub fn set_cursor_updated(&mut self) {
        self.cursor_origin_position_updated = false;
        self.cursor_bonk_position_updated = false;
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
    mut meshes: ResMut<Assets<Mesh>>,
    scene_meshes: Query<(Entity, &Name, &Handle<Mesh>, &Transform)>,
    ground_query: Query<(Entity, &Handle<Mesh>), With<Ground>>,
    mut gb_query: Query<(Entity, &mut GolfBall)>,
) {
    let ground_sensor = commands
        .spawn(Collider::cylinder(0.1, 2000.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -10.0, 0.0)))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Sensor)
        .insert(Name::new("ground_sensor"))
        .id();

    let players = party.get_all_player_ids();
    for (idx, (mut entity, mut golf_ball)) in gb_query.iter_mut().enumerate() {
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
    for (entity, name, mesh_handle, transform) in scene_meshes.iter() {
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

// Helper: golf_ball_handler_spawn_golf_balls_for_party_members
fn golf_ball_handler_init_golf_ball_uuid(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    glb_storage: &Res<GLBStorageID>, //Arc<[MapID]> //map: Arc<str>,
    player_id: &Uuid,
) {
    if let Some(basic_golf_ball) = glb_storage.glb.get(25) {
        let basic_golf_ball_handle: Handle<Scene> = asset_server.load(
            GltfAssetLabel::Scene(0).from_asset(basic_golf_ball.map),
        );
        let golf_ball = commands
            .spawn(SceneBundle {
                scene: basic_golf_ball_handle.clone(),
                ..default()
            })
            .insert(Interactable)
            .insert(GolfBall(GolfBallPosition{
                uuid: *player_id,
                position: Vec3::ZERO,
                last_position: Vec3::ZERO,
            }))
            .id();         
    } else {
        warn!("Target was not valid. Refer to the GLBStorageID map in the library.");
    };
}

pub fn golf_ball_handler_spawn_golf_balls_for_party_members(
    mut commands: Commands,
    mut run_trigger: ResMut<RunTrigger>,
    party: ResMut<Party>,
    asset_server: Res<AssetServer>,
    glb_storage: Res<GLBStorageID>, //Arc<[MapID]> //map: Arc<str>,
) {
    {
        for player in party.get_all_player_ids().iter() {
            info!("Building Golf Ball for player: [{:?}]", &player);
            golf_ball_handler_init_golf_ball_uuid(
                &mut commands,
                &asset_server,
                &glb_storage,
                &player,
            );
            // party.golf_ball_build_player(&player);
        };
    }
    run_trigger.set_target("golf_ball_handler_spawn_golf_balls_for_party_members", false);
    info!("post response: golf_ball_handler_spawn_golf_balls_for_party_members: {}", run_trigger.get("golf_ball_handler_spawn_golf_balls_for_party_members"));  
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

pub fn golf_ball_handler_end_game(
    commands: Commands,
    mut run_trigger: ResMut<RunTrigger>,
    mut golf_balls: Query<Entity, With<GolfBall>>,
) {
    info!("function: golf_ball_handler_end_game "); 
    {
        level_handler_purge_golf_ball_all(commands, golf_balls);
    }
    run_trigger.set_target("golf_ball_handler_end_game", false);
    info!("post response: golf_ball_handler_end_game: {}", run_trigger.get("golf_ball_handler_end_game"));  
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

pub fn golf_ball_handler_active_player_manual_bonk(
    mut run_trigger: ResMut<RunTrigger>,
    mut party: ResMut<Party>,
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