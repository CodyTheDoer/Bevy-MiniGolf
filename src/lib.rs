use bevy::prelude::*;
use bevy_rapier3d::prelude::RigidBody;

use std::sync::Arc;

pub mod leaderboard_handler;
pub mod level_handler;
pub mod network_handler;
pub mod player_handler;
pub mod user_interface;

#[derive(Resource)]
pub struct Fonts {
    pub fonts: Vec<TextStyle>,
}

impl Fonts {
    pub fn new() -> Self {
        let fonts: Vec<TextStyle> = Vec::new();
        Fonts {
            fonts,
        }
    }
}

#[derive(Clone, Resource)]
pub struct OpIndex {
    pub ui_entities: u32,
    pub state_info_call: u32,
}

impl OpIndex {
    pub fn new() -> Self {
        let ui_entities: u32 = 0;
        let state_info_call: u32 = 0;
        OpIndex {
            ui_entities,
            state_info_call,
        }
    }

    pub fn add_ui_entity(&mut self) {
        self.ui_entities += 1;
    }
}

// --- Physics Handler --- //

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum ArrowState {
    #[default]
    Idle,
    DrawingArrow,
}

#[derive(Clone, Debug)] // could tie into player struct once assembled
pub struct BonkMouseXY {
    pub x: f32,
    pub y: f32, 
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

#[derive(Clone, Debug, Resource)] // could tie into physics/player struct once assembled
pub struct BonkHandler {
    pub direction: Vec3,
    pub power: f32,
    pub cursor_origin_position: BonkMouseXY,
    pub cursor_origin_position_updated: bool,
    pub cursor_bonk_position: BonkMouseXY,
    pub cursor_bonk_position_updated: bool,
}

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

// --- Level Handler --- //

#[derive(Component)]
pub struct Ground;

#[derive(Asset, Component, Debug, TypePath)]
pub struct Interactable; 

#[derive(Clone, Debug, Resource)]
pub struct GLBStorageID {
    glb: Arc<[MapID]>,
}

#[derive(Debug)]
pub struct MapID {
    map: &'static str,
}

pub struct SceneLoadedEvent {
    pub entity: Entity,
}

impl GLBStorageID {
    pub fn new() -> Self {
        let map_paths = [
            "glb/boilerplate_level_tutorial.glb",
            "glb/boilerplate_level_1.glb",
            "glb/boilerplate_level_2.glb",
            "glb/boilerplate_level_3.glb",
            "glb/boilerplate_level_4.glb",
            "glb/boilerplate_level_5.glb",
            "glb/boilerplate_level_6.glb",
            "glb/boilerplate_level_7.glb",
            "glb/boilerplate_level_8.glb",
            "glb/boilerplate_level_9.glb",
            "glb/boilerplate_level_10.glb",
            "glb/boilerplate_level_11.glb",
            "glb/boilerplate_level_12.glb",
            "glb/boilerplate_level_13.glb",
            "glb/boilerplate_level_14.glb",
            "glb/boilerplate_level_15.glb",
            "glb/boilerplate_level_16.glb",
            "glb/boilerplate_level_17.glb",
            "glb/boilerplate_level_18.glb",
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

#[derive(Clone, Debug)]
pub enum InteractableEntities {
    Ground,
    Ent1,
    Ent2,
    Ent3,
    Ent4,
    Ent5,
    Ent6,
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum LevelState {
    #[default]
    HoleTutorial,
    Hole1,
    Hole2,
    Hole3,
    Hole4,
    Hole5,
    Hole6,
    Hole7,
    Hole8,
    Hole9,
    Hole10,
    Hole11,
    Hole12,
    Hole13,
    Hole14,
    Hole15,
    Hole16,
    Hole17,
    Hole18,
}

// --- User Interface --- //

pub struct UserInterface {}

impl UserInterface {
    pub fn select_a_hole_widget() -> i32 {
        let target = 0;
        target
    }
}

#[derive(Clone, Debug)]
pub enum InfoCall {
    Call0,
    Call1,
    Call2,
    Call3,
    Call4,
    Call5,
    Call6,
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum GameState {
    #[default]
    LoadingScreen,
    MenuMain,
    MenuSettings,
    MenuOnline,
    InGame,
    InGamePaused,
    PostGameReview,
}

#[derive(Resource)]
pub struct GameStateHandler {
    current_level: i32,
    maps_index: i32,
    arrow_state: bool,
}

impl GameStateHandler {
    pub fn new() -> Self {
        let current_level = 0;
        let maps_index = 0;
        let arrow_state = false;
        GameStateHandler {
            current_level,
            maps_index,
            arrow_state,
        }
    }

    pub fn get_arrow_state (&self) -> bool {
        self.arrow_state
    }

    pub fn set_arrow_state_true (&mut self) {
        self.arrow_state = true;
    }

    pub fn set_arrow_state_false (&mut self) {
        self.arrow_state = false;
    }
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum MapSetState {
    #[default]
    Tutorial,
    WholeCorse,
    FrontNine,
    BackNine,
    SelectAHole,
}


// --- Rapier Integration --- //
pub fn print_ball_altitude(mut positions: Query<&mut Transform, With<RigidBody>>) {
    for mut transform in positions.iter_mut() {
        // dbg!(transform.rotation.to_axis_angle());
        transform.rotation = Quat::from_rotation_z(270_f32.to_radians());
        println!("Ball altitude: {}", transform.translation.y);
    }
}

// --- User Interface --> CameraUI --- //

#[derive(Asset, Component, TypePath)]
pub struct CameraUi;

// --- User Interface --> CameraWorld --- //

#[derive(Asset, Component, TypePath)]
pub struct CameraWorld;

// --- Active Integration --- //

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum CameraOrbitEntityState {
    #[default]
    Ball,
    Cup,
    FreePan,
}

#[derive(Debug, Resource)]
pub struct CameraOrbitEntityStateHandler {
    current_state: i32,
}

impl CameraOrbitEntityStateHandler {
    pub fn new() -> Self {
        let current_state = 0;
        CameraOrbitEntityStateHandler {
            current_state,
        }
    }
}

use crate::user_interface::camera_world::PanOrbitSettings;

pub fn camera_orbit_entity_state_update(    
    camera_orbit_entity_state: Res<State<CameraOrbitEntityState>>,
    mut next_camera_orbit_entity_state: ResMut<NextState<CameraOrbitEntityState>>,
    mut camera_orbit_entity_state_handler: ResMut<CameraOrbitEntityStateHandler>,
) {
    match camera_orbit_entity_state.get() {
        CameraOrbitEntityState::Ball => {
            info!("CameraOrbitEntityState::Cup");
            camera_orbit_entity_state_handler.current_state = 1;
            next_camera_orbit_entity_state.set(CameraOrbitEntityState::Cup);
        },
        CameraOrbitEntityState::Cup => {
            info!("CameraOrbitEntityState::FreePan");
            camera_orbit_entity_state_handler.current_state = 2;
            next_camera_orbit_entity_state.set(CameraOrbitEntityState::FreePan);
        },
        CameraOrbitEntityState::FreePan => {
            info!("CameraOrbitEntityState::Ball");
            camera_orbit_entity_state_handler.current_state = 0;
            next_camera_orbit_entity_state.set(CameraOrbitEntityState::Ball);
        },
    }
}

#[derive(Debug, Resource)]
pub struct CameraCoordTracker {
    current_coords: Vec3,
}

impl CameraCoordTracker {
    pub fn new() -> Self {
        let current_coords: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        CameraCoordTracker {
            current_coords,
        }
    }
}

pub fn camera_orbit_entity_state_logic(
    mut camera_orbit_entity_state: ResMut<State<CameraOrbitEntityState>>,
    mut camera_coord_tracker: ResMut<CameraCoordTracker>,
    scene_meshes: Query<(Entity, &Name, &Transform)>,
    mut q_rigid_body: Query<(&RigidBody, &Transform)>,
) {
    let mut ball_rigid_body_coords: Vec3 = Vec3::new(0.0, 0.0, 0.0); 
    for (_, transform) in q_rigid_body.iter() {
        ball_rigid_body_coords = transform.translation.clone();
    }

    match camera_orbit_entity_state.get() {
        CameraOrbitEntityState::Ball => {
            for (entity, name, transform) in scene_meshes.iter() {
                if name.as_str() == "ball" {
                    camera_coord_tracker.current_coords = ball_rigid_body_coords;
                    break;
                }
            }        
        }
        CameraOrbitEntityState::Cup => {
            for (entity, name, transform) in scene_meshes.iter() {
                if name.as_str() == "cup" {
                    camera_coord_tracker.current_coords = transform.translation;
                    break;
                }
            }        
        }
        CameraOrbitEntityState::FreePan => {    
        }
    }
}