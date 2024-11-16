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

// --- Party Handler --- //
#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum PartyState {
    #[default]
    Local,
    Online,
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum PlayerCompletionStatus {
    #[default]
    NotInGame,
    HoleIncomplete,
    HoleCompleted,
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum PlaythroughStyleState {
    #[default]
    Proximity,
    RandomSetOrder,
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum TurnState {
    #[default]
    Idle,
    Player1,
    Player2,
    Player3,
    Player4,
    Player5,
    Player6,
}

// --- LeaderBoard Handler --- //
#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum LeaderBoardState {
    #[default]
    Mixed,
    Online,
    Local,
    PostGame,
    InGame,
    InGameOnline,
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
#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum LevelState {
    #[default]
    MainMenu,
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
    HoleTutorial,
}

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
            "glb/main_menu.glb",
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
            "glb/boilerplate_level_tutorial.glb",
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

// --- User Interface --- //

pub struct UserInterface {}

impl UserInterface {
    pub fn select_a_hole_widget() -> i32 {
        let target = 0;
        target
    }
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum GameState {
    #[default]
    LoadingScreen,
    MenuMain,
    MenuSettings,
    MenuOnline,
    OnlineGameInit,
    GameInit,
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



#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum CameraOrbitEntityState {
    #[default]
    MainMenu,
    Ball,
    Cup,
    FreePan,
    LeaderBoard,
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

    // Bundle to spawn our custom camera easily
#[derive(Bundle, Default)]
pub struct PanOrbitCameraBundle {
    pub camera: Camera3dBundle,
    pub state: PanOrbitState,
    pub settings: PanOrbitSettings,
}

    // The internal state of the pan-orbit controller
#[derive(Component, Debug)]
pub struct PanOrbitState {
    pub center: Vec3,
    pub radius: f32,
    pub upside_down: bool,
    pub pitch: f32,
    pub yaw: f32,
}

    // The configuration of the pan-orbit controller
#[derive(Component)]
pub struct PanOrbitSettings {
    /// World units per pixel of mouse motion
    pub pan_sensitivity: f32,
    /// Radians per pixel of mouse motion
    pub orbit_sensitivity: f32,
    /// Exponent per pixel of mouse motion
    pub zoom_sensitivity: f32,
    /// Key to hold for panning
    pub pan_key: Option<KeyCode>,
    /// Key to hold for orbiting
    pub orbit_key: Option<KeyCode>,
    /// Key to hold for zooming
    pub zoom_key: Option<KeyCode>,
    /// What action is bound to the scroll wheel?
    pub scroll_action: Option<PanOrbitAction>,
    /// For devices with a notched scroll wheel, like desktop mice
    pub scroll_line_sensitivity: f32,
    /// For devices with smooth scrolling, like touchpads
    pub scroll_pixel_sensitivity: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PanOrbitAction {
    Pan,
    Orbit,
    Zoom,
}

impl Default for PanOrbitState {
    fn default() -> Self {
        PanOrbitState {
            center: Vec3::ZERO,
            radius: 1.0,
            upside_down: false,
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}

impl Default for PanOrbitSettings {
    fn default() -> Self {
        PanOrbitSettings {
            pan_sensitivity: 0.001, // 1000 pixels per world unit
            orbit_sensitivity: 0.1f32.to_radians(), // 0.1 degree per pixel
            zoom_sensitivity: 0.01,
            pan_key: Some(KeyCode::ControlLeft),
            orbit_key: Some(KeyCode::AltLeft),
            zoom_key: Some(KeyCode::ShiftLeft),
            scroll_action: Some(PanOrbitAction::Zoom),
            scroll_line_sensitivity: 16.0, // 1 "line" == 16 "pixels of motion"
            scroll_pixel_sensitivity: 1.0,
        }
    }
}

// --- Active Integration --- //