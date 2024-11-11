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

#[derive(Clone, Resource)]
pub struct GLBPurgeID {
    glb: Vec<String>,
}

impl GLBPurgeID {
    pub fn new() -> Self {
        let glb: Vec<String> = Vec::new();
        GLBPurgeID {
            glb,
        }
    }
}

// --- User Interface --> CameraUI --- //

#[derive(Asset, Component, TypePath)]
pub struct CameraUi;

// --- User Interface --> CameraWorld --- //

#[derive(Asset, Component, TypePath)]
pub struct CameraWorld;

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
pub enum MapSetState {
    #[default]
    Tutorial,
    WholeCorse,
    FrontNine,
    BackNine,
    SelectAHole,
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
}

impl GameStateHandler {
    pub fn new() -> Self {
        let current_level = 0;
        let maps_index = 0;
        GameStateHandler {
            current_level,
            maps_index,
        }
    }
}


// --- Rapier Integration --- //
pub fn print_ball_altitude(mut positions: Query<&mut Transform, With<RigidBody>>) {
    for mut transform in positions.iter_mut() {
        // dbg!(transform.rotation.to_axis_angle());
        transform.rotation = Quat::from_rotation_z(270_f32.to_radians());
        println!("Ball altitude: {}", transform.translation.y);
    }
}