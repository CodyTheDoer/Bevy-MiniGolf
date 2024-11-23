use bevy::prelude::*;
use bevy_rapier3d::prelude::RigidBody;

// use std::fmt;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

pub mod leaderboard_handler;
pub mod level_handler;
pub mod network_handler;
pub mod player_handler;
pub mod user_interface;

#[derive(Clone, Debug)]
enum NetworkInterface {
    InitHandshake,
    HandshakeConfirmed,

    PlayerCheckProfile,
    PlayerUpload,
    PlayerUpdate,

    RequestLeaderBoard,
    UpdateLeaderBoard,
    
    FriendsGet,
    FriendSearch,
    FriendAdd,
    FriendRemove,

    PartyGet,
    PartyAddFriend,
    PartyAddAI,
    PartyRemovePlayer,
    PartyJoin,
    PartyLeave,
    
    FindMatchInit,
    FindMatchCancel,

    InGameQuit,
    InGameBonk,
    InGameHoleCompletePlayer,
}

#[derive(Clone, Debug)]
pub enum RemoteStateUpdate {
    // ConnectionState(ConnectionState),
    // GameState(GameState),
    // LevelState(LevelState),
}

// --- State Enums --- //

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum StateArrow {
    #[default]
    Idle,
    DrawingArrow,
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum StateCameraOrbitEntity {
    #[default]
    Menu,
    Ball,
    Cup,
    FreePan,
    LeaderBoard,
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum StateLevel {
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
    MenuLeaderBoard,
    MenuLocal,
    MenuOnline,
    MenuPreferences,
    MenuPlayer,
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum StateMapSet {
    #[default]
    Tutorial,
    WholeCorse,
    FrontNine,
    BackNine,
    SelectAHole,
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum StateMenu {
    #[default]
    MenuMainMenu,
    MenuLeaderBoard,
    MenuLocal,
    MenuOnline,
    MenuPreferences,
    MenuPlayer,
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum StateGame {
    #[default]
    NotInGame,
    InGame,
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum StateGameConnection {
    #[default]
    Local,
    Online,
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum StateGamePlayStyle {
    #[default]
    SetOrder,
    Proximity,
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum StateTurn {
    #[default]
    NotInGame,
    Idle,
    Active,
}

// World //

#[derive(Component)]
pub struct Ground;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PanOrbitAction {
    Pan,
    Orbit,
    Zoom,
}

#[derive(Bundle, Default)]
pub struct PanOrbitCameraBundle {
    pub camera: Camera3dBundle,
    pub state: StatePanOrbit,
    pub settings: PanOrbitSettings,
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

// The internal state of the pan-orbit controller
#[derive(Component, Debug)]
pub struct StatePanOrbit {
    pub center: Vec3,
    pub radius: f32,
    pub upside_down: bool,
    pub pitch: f32,
    pub yaw: f32,
}

#[derive(Asset, Component, TypePath)]
pub struct CameraUi;

// --- User Interface --- //

#[derive(Asset, Component, Debug, TypePath)]
pub struct Interactable; 

#[derive(Resource)]
pub struct Fonts {
    pub fonts: Vec<TextStyle>,
}

#[derive(Resource)]
pub struct GameHandler {
    current_level: i32,
    arrow_state: bool,
    network_server_connection: bool,
    remotely_pushed_state: Option<RemoteStateUpdate>,
}

#[derive(Resource)]
pub struct RunTrigger{
    pub cycle_camera: bool,
    pub cycle_state_map_set: bool,
    pub toggle_state_game: bool,
}

impl RunTrigger {
    pub fn new() -> Self {
        Self{
            cycle_camera: false,
            cycle_state_map_set: false,
            toggle_state_game: false,
        }
    }

    pub fn get(&self, target: &str) -> bool {
        match target {
            "cycle_camera" => {
                self.cycle_camera
            },
            "cycle_state_map_set" => {
                self.cycle_state_map_set
            },
            "toggle_state_game" => {
                self.toggle_state_game
            },
            _ => {false},
        }
    }

    pub fn set_target(&mut self, target: &str, state: bool) {
        match target {
            "cycle_camera" => {
                self.cycle_camera = state;
            },
            "cycle_state_map_set" => {
                self.cycle_state_map_set = state;
            }
            "toggle_state_game" => {
                self.toggle_state_game = state;
            }
            _ => {},
        }
    }
}

// Define marker components to find the entities later
#[derive(Component)]
pub struct StateText;

#[derive(Component)]
pub struct TitleText;

// --- Player Handler --- //

#[derive(Clone, Resource)]
pub struct Player {
    pub player_id: String,
	pub hole_completion_state: bool,
	pub ball_material: Color, // For now custom material/shaders planned
	pub ball_location: Vec3,
	pub bonks_game: u32,
	pub bonks_level: u32,
}

// --- Party Handler --- //

#[derive(Resource)]
pub struct Party {
    players: Arc<Mutex<Vec<Arc<Mutex<Player>>>>>,
    players_finished: Arc<Mutex<i32>>,
    active_player: Arc<Mutex<i32>>,
    active_level: Arc<Mutex<i32>>,
    ai_count: Arc<Mutex<i32>>,
    remote_count: Arc<Mutex<i32>>,
}

// --- Camera --- //

#[derive(Debug, Resource)]
pub struct CameraHandler {
    current_coords: Vec3,
}

#[derive(Asset, Component, TypePath)]
pub struct CameraWorld;
