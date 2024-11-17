use bevy::prelude::*;
use bevy_mod_raycast::prelude::*;
use bevy_rapier3d::{parry::shape::SharedShape, prelude::*};
use bevy_render::mesh::{Indices, VertexAttributeValues};

use crate::{
    ArrowState, 
    BonkHandler,
    BonkMouseXY,
    CameraWorld,
    GameHandler,
    Ground,
    LevelState,
    TurnState,
};

use std::f32::consts::PI;

pub fn add_physics_query_and_update_scene(
    mut arrow_state: ResMut<State<ArrowState>>,
    mut next_arrow_state: ResMut<NextState<ArrowState>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    scene_meshes: Query<(Entity, &Name, &Handle<Mesh>, &Transform)>,
    ground_query: Query<(Entity, &Handle<Mesh>), With<Ground>>,
) {
    let ground_sensor = commands
        .spawn(Collider::cylinder(0.1, 2000.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -10.0, 0.0)))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Sensor)
        .insert(Name::new("ground_sensor"))
        .id();

    // iterate over all meshes in the scene and match them by their name.
    for (entity, name, mesh_handle, transform) in scene_meshes.iter() {
        if name.as_str() == "ball" {
            let collider = Collider::ball(0.022);
            commands
                .entity(entity)
                .insert(collider)
                .insert(RigidBody::Dynamic)
                .insert(Damping {
                    angular_damping: 1.65,
                    ..default()
                })
                .insert(ExternalImpulse::default())
                .insert(ColliderMassProperties::Density(1.0))
                .insert(GravityScale(1.0))
                .insert(Ccd::enabled());
        }
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

fn extract_mesh_vertices_indices( // Helper function for ^add_physics_query_and_update_scene^
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

pub fn apply_rotation_matrix_camera_yaw(
    camera_yaw: &f32, // Query only for CameraWorld's Transform
    direction_x: f32,
    direction_y: f32,
) -> BonkMouseXY {
    // 2D rotation matrix
    let rotation_matrix = vec![
        [camera_yaw.cos(), camera_yaw.sin()],
        [-camera_yaw.sin(), camera_yaw.cos()],
    ];

    let rotated_x = rotation_matrix[0][0] * direction_x + rotation_matrix[0][1] * direction_y;
    let rotated_y = rotation_matrix[1][0] * direction_x + rotation_matrix[1][1] * direction_y;

    BonkMouseXY {
        x: rotated_x,
        y: rotated_y,
    }
}

pub fn bonk(
    mut impulses: Query<&mut ExternalImpulse>,
    bonk: Res<BonkHandler>,
) {
    for mut impulse in impulses.iter_mut() {
        // Reset or set the impulse every frame
        let scaled_bonk = bonk.power * 0.00025;
        impulse.impulse = bonk.direction * scaled_bonk;
        impulse.torque_impulse = Vec3::new(0.0, 0.0, 0.0);
    }
}

pub fn bonk_step_start( // set's bonk start xy
    windows: Query<&Window>,
    mut bonk: ResMut<BonkHandler>,
    mut gsh: ResMut<GameHandler>,
    mut arrow_state: ResMut<State<ArrowState>>,
    mut next_arrow_state: ResMut<NextState<ArrowState>>,
) {
    let mut cursor_xy: BonkMouseXY = BonkMouseXY::new();
    let Some(position) = windows.single().cursor_position() else {
        return;
    };
    cursor_xy.set(position.x, position.y);
    bonk.update_cursor_origin_position(cursor_xy);
    match arrow_state.get() {
        ArrowState::Idle => {
            toggle_arrow_state(gsh, arrow_state, next_arrow_state);
        },
        _ => {},
    }
}

pub fn bonk_step_mid( // Determines bonks power by measuring the difference between origin and current mouse xy
    mut bonk: ResMut<BonkHandler>,
    windows: Query<&Window>,
) {
    let mut cursor_xy: BonkMouseXY = BonkMouseXY::new();
    let Some(position) = windows.single().cursor_position() else {
        return;
    };
    let window_width: f32 = windows.single().width();
    let window_height: f32 = windows.single().height();
    cursor_xy.set(position.x, position.y);
    bonk.update_cursor_bonk_position(cursor_xy);

    // find length of pixels from origin to release
    let difference_x = bonk.cursor_origin_position.x - bonk.cursor_bonk_position.x;
    let difference_y = bonk.cursor_origin_position.y - bonk.cursor_bonk_position.y;

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

    bonk.power = bonk_power;
    bonk.set_cursor_updated();
}

pub fn bonk_step_end( // Fires bonk 
    mut arrow_state: ResMut<State<ArrowState>>,
    mut gsh: ResMut<GameHandler>,
    mut next_arrow_state: ResMut<NextState<ArrowState>>,
    mut impulses: Query<&mut ExternalImpulse>,
    bonk_res: Res<BonkHandler>,
    rapier_context: Res<RapierContext>,
    rigid_body_query: Query<(Entity, &RapierRigidBodyHandle)>,
    scene_meshes: Query<(Entity, &Name)>,
) {
    if gsh.get_arrow_state() {
        toggle_arrow_state(gsh, arrow_state, next_arrow_state);
    }
    if bonk_res.power != 0.0 {
        if golf_ball_is_asleep(rapier_context, rigid_body_query, scene_meshes) {
            bonk(impulses, bonk_res);
        }
    }
}

pub fn collision_events_listener(
    mut collision_events: EventReader<CollisionEvent>,
    scene_meshes: Query<(Entity, &Name)>,
    mut next_turn_state: ResMut<NextState<TurnState>>,
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
                                next_turn_state.set(TurnState::HoleComplete);
                            },
                            "ground_sensor" => {
                                info!("1: Ooof grounded...");
                                next_turn_state.set(TurnState::TurnReset);
                            },
                            _ => {},
                        }
                    }
                    if *entity2 == entity {
                        match owned_name {
                            "cup_sensor" => {info!("2: Cups baby!!!!")},
                            "ground_sensor" => {info!("2: Ooof grounded...")},
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

pub fn golf_ball_is_asleep(
    rapier_context: Res<RapierContext>,
    query: Query<(Entity, &RapierRigidBodyHandle)>,
    scene_meshes: Query<(Entity, &Name)>,
) -> bool {
    let mut results = false;
    // iterate over all meshes in the scene and match them by their name.
    for (entity, name) in scene_meshes.iter() {
        if name.as_str() == "ball" {
            let active_entity = entity;
            for (entity, rb_handle) in query.iter() {
                // Access the rigid body from the physics world using its handle
                if let Some(rigid_body) = rapier_context.bodies.get(rb_handle.0) {
                    // Check if the rigid body is currently sleeping
                    if active_entity == entity {
                        if rigid_body.is_sleeping() {
                            // println!("Entity {:?} is sleeping", entity);
                            results = true;
                        }
                    }
                }
            }        
        }
    }
    results
}

fn toggle_arrow_state(
    mut gsh: ResMut<GameHandler>,
    mut state: ResMut<State<ArrowState>>,
    mut next_state: ResMut<NextState<ArrowState>>,
) {
    match state.get() {
        ArrowState::DrawingArrow => {
            info!("Entering ArrowState::Idle");
            gsh.set_arrow_state_false();
            next_state.set(ArrowState::Idle);
        },
        ArrowState::Idle => {
            info!("Entering ArrowState::DrawingArrow");
            gsh.set_arrow_state_true();
            next_state.set(ArrowState::DrawingArrow);
        },
    }
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