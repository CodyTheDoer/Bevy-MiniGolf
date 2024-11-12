// --- Internal Bevy Plugins --- //
use bevy::{prelude::*,
    input::common_conditions::*,
    window::{PresentMode, WindowTheme},
};

// --- External Plugins --- //
use bevy_rapier3d::prelude::*;
use bevy_editor_pls::prelude::*;

// --- States --- //
use minigolf::{ 
    GameState, 
    LevelState, 
    MapSetState,
};

// --- Resources --- //
use minigolf::{
    Fonts, 
    Interactable,
    GameStateHandler, 
    GLBPurgeID, 
    GLBStorageID, 
    OpIndex,
};

// --- User Interface Import --- //
use minigolf::user_interface::camera_world::{
    setup_3d_camera,
    pan_orbit_camera, 
};
use minigolf::user_interface::camera_world::PanOrbitState;
use minigolf::user_interface::user_interface::{
    game_state_update, 
    fire_ray, 
    release_ray, 
    draw_cursor, 
    setup_ui
};

// --- Level Handler Import --- //
use minigolf::level_handler::level_handler::{
    init_hole_n, 
    level_state_logic, 
    level_state_update, 
    map_set_state_update, 
    setup_ground, 
    setup_light, 
    purge_glb_all
};

fn main() {
    let mut app = App::new();
        app.add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Minigolf".into(),
                    name: Some("bevy.app".into()),
                    resolution: (1280., 720.).into(),
                    resizable: true,
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: true,
                        ..Default::default()
                    },
                    present_mode: PresentMode::AutoVsync,
                    prevent_default_event_handling: false, // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    window_theme: Some(WindowTheme::Dark),
                    visible: true,
                    ..default()
                }),
                ..default()
            }),
        ))

        // --- Additional Plugins --- //
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        // .add_plugins(EditorPlugin::default())

        // --- State Initialization --- //
        .insert_state(GameState::LoadingScreen)
        .insert_state(LevelState::HoleTutorial)
        .insert_state(MapSetState::Tutorial)

        // --- Resource Initialization --- //
        .insert_resource(Fonts::new())
        .insert_resource(GameStateHandler::new())
        .insert_resource(GLBPurgeID::new())
        .insert_resource(GLBStorageID::new())
        .insert_resource(OpIndex::new())

        // --- Startup Systems Initialization --- //
        .add_systems(Startup, setup_ground)
        .add_systems(Startup, setup_light)
        .add_systems(Startup, setup_ui)
        .add_systems(Startup, setup_3d_camera)

        // --- Update Systems Initialization --- //
        .add_systems(Update, pan_orbit_camera.run_if(any_with_component::<PanOrbitState>))
        .add_systems(Update, fire_ray.run_if(input_pressed(MouseButton::Left)))
        .add_systems(Update, release_ray.run_if(input_just_released(MouseButton::Left)))
        .add_systems(Update, game_state_update.run_if(input_just_released(KeyCode::ArrowLeft)))
        .add_systems(Update, level_state_update.run_if(input_just_released(KeyCode::ArrowUp)))
        .add_systems(Update, map_set_state_update.run_if(input_just_released(KeyCode::ArrowRight)))
        .add_systems(Update, draw_cursor)
        .add_systems(Update, level_state_logic)

        // --- OnEnter State Reaction Initialization --- //
        .add_systems(OnEnter(LevelState::HoleTutorial), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole1), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole2), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole3), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole4), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole5), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole6), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole7), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole8), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole9), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole10), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole11), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole12), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole13), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole14), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole15), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole16), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole17), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole18), init_hole_n)

        // --- OnExit State Reaction Initialization --- //
        .add_systems(OnExit(LevelState::HoleTutorial), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole1), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole2), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole3), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole4), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole5), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole6), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole7), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole8), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole9), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole10), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole11), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole12), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole13), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole14), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole15), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole16), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole17), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole18), purge_glb_all)
        
        // --- Active Dev Targets --- //
        .insert_state(ArrowState::Idle)
        .insert_resource(Bonk::new())
        .add_systems(Update, add_physics_query_and_update_scene.run_if(input_just_released(MouseButton::Right)))
        // .add_systems(Update, toggle_arrow_state.after(add_physics_query_and_update_scene))
        // .add_systems(Update, bonk_gizmo.run_if(input_just_released(KeyCode::KeyB)))
        .add_systems(Update, bonk_gizmo.run_if(in_state(ArrowState::DrawingArrow)))
        // .add_systems(Update, bonk.run_if(input_just_released(KeyCode::Space)))
        .add_systems(Update, collision_events_listener)
        .add_systems(Update, check_sleeping_entities)
        .add_systems(Update, start_bonk.run_if(input_just_pressed(MouseButton::Middle)))
        .add_systems(Update, mid_bonk.run_if(input_pressed(MouseButton::Middle)))
        .add_systems(Update, end_bonk.run_if(input_just_released(MouseButton::Middle)));

        app.run();
}






use bevy_mod_raycast::prelude::*;
use minigolf::CameraWorld;

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
enum ArrowState {
    #[default]
    Idle,
    DrawingArrow,
}

fn toggle_arrow_state(
    mut gsh: ResMut<GameStateHandler>,
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

#[derive(Clone, Debug)] // could tie into player struct once assembled
struct BonkMouseXY {
    x: f32,
    y: f32, 
}

impl BonkMouseXY {
    fn new() -> Self {
        let x: f32 = 0.0;
        let y: f32 = 0.0;
        BonkMouseXY {
            x,
            y,
        }
    }
    
    fn set(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
    
    fn setx(&mut self, x: f32) {
        self.x = x;
    }
    
    fn sety(&mut self, y: f32) {
        self.y = y;
    }
}

#[derive(Clone, Debug, Resource)] // could tie into physics/player struct once assembled
struct Bonk {
    direction: Vec3,
    power: f32,
    cursor_origin_position: BonkMouseXY,
    cursor_origin_position_updated: bool,
    cursor_bonk_position: BonkMouseXY,
    cursor_bonk_position_updated: bool,
}

impl Bonk {
    fn new() -> Self {
        let direction: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let power: f32 = 0.0;
        let cursor_origin_position: BonkMouseXY = BonkMouseXY::new();
        let cursor_origin_position_updated: bool = false;
        let cursor_bonk_position: BonkMouseXY = BonkMouseXY::new();
        let cursor_bonk_position_updated: bool = false;
        Bonk {
            direction,
            power,
            cursor_origin_position,
            cursor_origin_position_updated,
            cursor_bonk_position,
            cursor_bonk_position_updated,
        }
    }

    fn update_direction(&mut self, direction: &Vec3) {
        self.direction = *direction;
    }

    fn update_power(&mut self, power: f32) {
        self.power = power;
    }

    fn update_cursor_origin_position(
        &mut self, 
        bonk_coords: BonkMouseXY
    ) {
        self.cursor_origin_position = bonk_coords;
        self.cursor_origin_position_updated = true;
    }

    fn update_cursor_bonk_position(
        &mut self, 
        bonk_coords: BonkMouseXY
    ) {
        self.cursor_bonk_position = bonk_coords;
        self.cursor_bonk_position_updated = true;
    }

    fn set_cursor_updated(&mut self) {
        self.cursor_origin_position_updated = false;
        self.cursor_bonk_position_updated = false;
    }
}







fn start_bonk( // set's bonk start xy
    windows: Query<&Window>,
    mut arrow_state: ResMut<State<ArrowState>>,
    mut bonk: ResMut<Bonk>,
    mut gsh: ResMut<GameStateHandler>,
    mut next_arrow_state: ResMut<NextState<ArrowState>>,
) {
    info!("Start bonk");
    let mut cursor_xy: BonkMouseXY = BonkMouseXY::new();
    let Some(position) = windows.single().cursor_position() else {
        return;
    };
    cursor_xy.set(position.x, position.y);
    bonk.update_cursor_origin_position(cursor_xy);
    // info!("bonk updated: pressed: {:?}", bonk.cursor_origin_position);
    match arrow_state.get() {
        ArrowState::Idle => {
            toggle_arrow_state(gsh, arrow_state, next_arrow_state);
        },
        _ => {},
    }
}


use bevy::window::WindowResolution;

fn mid_bonk( // Determines bonks power by measuring the difference between origin and current mouse xy
    mut bonk: ResMut<Bonk>,
    windows: Query<&Window>,
) {
    // info!("End bonk");
    let mut cursor_xy: BonkMouseXY = BonkMouseXY::new();
    let Some(position) = windows.single().cursor_position() else {
        return;
    };
    let window_width: f32 = windows.single().width();
    let window_height: f32 = windows.single().height();
    cursor_xy.set(position.x, position.y);
    bonk.update_cursor_bonk_position(cursor_xy);
    // info!("bonk updated: pressed: {:?}", bonk.cursor_bonk_position);

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










fn check_sleeping_entities(
    sleeping_query: Query<(Entity, &Sleeping)>
) {
    for (entity, sleeping) in sleeping_query.iter() {
            println!("Entity {:?} is sleeping", entity);
    }
}



fn end_bonk( // Fires bonk 
    mut arrow_state: ResMut<State<ArrowState>>,
    mut gsh: ResMut<GameStateHandler>,
    mut next_arrow_state: ResMut<NextState<ArrowState>>,
    mut impulses: Query<&mut ExternalImpulse>,
    bonk_res: Res<Bonk>,
    sleeping_query: Query<(Entity, &Sleeping), With<RigidBody>>,
) {
    // let mut asleep = false;
    for (entity, _) in sleeping_query.iter() {
        info!("{:?}", entity);
    }
    if gsh.get_arrow_state() {
        toggle_arrow_state(gsh, arrow_state, next_arrow_state);
    }
    bonk(impulses, bonk_res);
}

fn bonk(
    mut impulses: Query<&mut ExternalImpulse>,
    bonk: Res<Bonk>,
) {
    for mut impulse in impulses.iter_mut() {
        // Reset or set the impulse every frame
        // impulse.impulse = Vec3::new(0.0, 0.0, 20.0);
        info!("bonk.power: {:?}", bonk.power);
        impulse.impulse = bonk.direction * (15.0 * bonk.power);
        impulse.torque_impulse = Vec3::new(0.0, 0.0, 0.0);
    }
}



fn bonk_gizmo(
    mut gizmos: Gizmos,
    mut raycast: Raycast,
    mut bonk: ResMut<Bonk>,
    scene_meshes: Query<(&Name, &Transform)>,
    windows: Query<&Window>,
) {
    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };
    for (name, transform) in scene_meshes.iter() {
        if name.as_str() == "ball" && transform.translation != Vec3::new(0.0, 0.0, 0.0) {
            let ball_position = transform.translation;
            
            // Calculate the direction from the ball to the intersection point.
            let mut direction_x = bonk.cursor_origin_position.x - cursor_position.x;
            let mut direction_y = bonk.cursor_origin_position.y - cursor_position.y;
            
            // Localize arrow to a flat xz plane 
            let direction_xyz: Vec3 = Vec3::new(direction_x, 0.0, direction_y).normalize() * (24.0 * bonk.power);
            bonk.update_direction(&direction_xyz);

            // // Draw an arrow from the ball in the direction toward the cursor.
            gizmos.arrow(
                ball_position,            // Start position of the arrow (at the ball)
                ball_position + direction_xyz, // End position, 12 units away from the cursor
                Color::srgb(0.0, 1.0, 0.0),             // Color of the arrow
            );
        }
    } 
}

fn collision_events_listener(
    mut collision_events: EventReader<CollisionEvent>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _flags) => {
                info!("Collision started between {:?} and {:?}", entity1, entity2);
            }
            CollisionEvent::Stopped(entity1, entity2, _flags) => {
                info!("Collision stopped between {:?} and {:?}", entity1, entity2);
            }
        }
    }
}

pub fn add_physics_query_and_update_scene(
    mut arrow_state: ResMut<State<ArrowState>>,
    mut next_arrow_state: ResMut<NextState<ArrowState>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    scene_meshes: Query<(Entity, &Name, &Handle<Mesh>, &Transform), Added<Name>>,
) {
    // iterate over all meshes in the scene and match them by their name.
    for (entity, name, mesh_handle, transform) in scene_meshes.iter() {
        if name.as_str() == "ball" {
            let mesh = meshes.get(&mesh_handle.clone()).unwrap();
            let collider = Collider::ball(1.0);
            commands
                .entity(entity)
                .insert(collider)
                .insert(RigidBody::Dynamic)
                .insert(ExternalImpulse::default())
                .insert(ColliderMassProperties::Density(10.0))
                .insert(GravityScale(10.0))
                .insert(Transform::from_xyz(0.0, 5.0, -20.0));
        }
        if name.as_str() == "cup" {
            let mesh = meshes.get(&mesh_handle.clone()).unwrap();
            // Create the collider from the mesh.
            let collider = Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap();
            // let collider = Collider::cuboid(0.8, 0.5, 0.8);
            // Attach collider to the entity of this same object.
            commands
                .entity(entity)
                .insert(collider)
                .insert(Transform::from_xyz(0.0, 0.0, 0.0));
        }
        if name.as_str() == "cup_sensor" {
            let mesh = meshes.get(&mesh_handle.clone()).unwrap();
            // Create the collider from the mesh.
            // let mut collider = Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap();
            let collider = Collider::cuboid(0.8, 0.5, 0.8);
            // Attach collider to the entity of this same object.
            commands
                .entity(entity)
                .insert(collider)
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(Sensor);
        }
        if name.as_str() == "start" {
            let mesh = meshes.get(&mesh_handle.clone()).unwrap();
            // Create the collider from the mesh.
            let collider = Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap();
            // Attach collider to the entity of this same object.
            commands
                // .insert(Transform::from_xyz(0.0, 0.0, 60.0))
                .entity(entity)
                .insert(collider)
                .insert(Transform::from_xyz(0.0, 0.0, 0.0));
        }
        if name.as_str() == "green" {
            let mesh = meshes.get(&mesh_handle.clone()).unwrap();
            // Create the collider from the mesh.
            let collider = Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap();
            // Attach collider to the entity of this same object.
            commands
                .entity(entity)
                .insert(collider)
                .insert(Transform::from_xyz(0.0, 0.0, 0.0));
        }
        if name.as_str() == "cannon" {
            let mesh = meshes.get(&mesh_handle.clone()).unwrap();
            // Create the collider from the mesh.
            let collider = Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap();
            // Attach collider to the entity of this same object.
            commands
                .entity(entity)
                .insert(collider)
                .insert(RigidBody::Fixed)
                .insert(Transform::from_xyz(0.0, 0.0, 0.0));
        }
    }
}