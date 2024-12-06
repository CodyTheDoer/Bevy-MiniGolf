use bevy::prelude::*;
use bevy_rapier3d::prelude::RigidBody;

use uuid::Uuid;

use rusqlite::Connection;
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;

use std::sync::Arc;
use std::sync::Mutex;

// Direct Imports
pub mod network_handler;
pub mod database_handler;

// Directory Imports
pub mod level_handler;
pub mod player_handler;
pub mod user_interface;

#[derive(Resource)]
pub struct DatabaseConnection{
    pub conn: Arc<Mutex<Connection>>,
}

#[derive(Debug, Event)]
pub struct OnlineStateChange;

#[derive(Resource)]
pub struct UpdateIdResource {
    pub update_id: Option<Uuid>,
}

#[derive(Clone, PartialEq, Eq, Hash, Resource,)]
pub struct ClientProtocol {}

impl ClientProtocol {
    pub fn new() -> Self {ClientProtocol{}}

    pub fn init_player_connection(&self) -> String {
        String::from("InitPlayerConnection")
    }

    pub fn all_states_packet(&self) -> String {
        String::from("PacketAllStates")
    }

    pub fn heart_beat_packet(&self) -> String {
        String::from("PacketHeartBeat")
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PacketAllStates<'a> {
    player_id: &'a str,
    state_game: &'a str,
    state_cam_orbit_entity: &'a str,
    state_game_play_style: &'a str,
    state_level: &'a str,
    state_map_set: &'a str,
    state_menu: &'a str,
    state_turn: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PacketHeartbeat<'a> {
    player_id: &'a str,
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
pub enum StateGame {
    #[default]
    NotInGame,
    InGame,
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum StateEngineConnection {
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
pub enum StateTurn {
    #[default]
    NotInGame,
    Idle,
    Active,
    NextTurn,
}

#[derive(Clone, Debug)]
pub enum StateUpdateRef {
    StateEngineConnection(StateEngineConnection),
    StateCameraOrbitEntity(StateCameraOrbitEntity),
    StateGame(StateGame),
    StateLevel(StateLevel),
    StateMapSet(StateMapSet),
    StateGamePlayStyle(StateGamePlayStyle),
    StateTurn(StateTurn),
}

#[derive(Resource)]
pub struct HeartbeatTimer(pub Timer);

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
#[derive(Component,)]
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

// Define marker components to find the entities later
#[derive(Component)]
pub struct TextState;

#[derive(Component)]
pub struct TextTitle;

// --- Player Handler --- //

pub trait Player {
    fn new() -> Self where Self: Sized;
    fn start_game(&mut self);
    fn hole_completed(&mut self);
    fn game_completed(&mut self);
    fn next_round_prep(&mut self);
    fn add_bonk(&mut self, level: usize);
    fn get_bonks(&mut self, level: usize) -> i32;
    fn get_hole_completion_state(&self) -> bool;
    fn set_hole_completion_state(&mut self, hole_completion_state: bool);
    fn get_player_id(&self) -> Uuid;
    fn set_player_id(&mut self, new_id: Uuid);
    fn get_player_type(&self) -> String;
    fn get_score(&self) -> [i32; 18];
    fn get_ball_location(&self) -> Vec3;
    fn set_ball_location(&mut self, location: Vec3);
}

#[derive(Clone, Resource)]
pub struct PlayerLocal {
    pub player_id: Uuid,
    pub player_type: String,
	pub hole_completion_state: bool,
	pub ball_material: Color, // For now custom material/shaders planned
	pub ball_location: Vec3,
	pub score: [i32; 18],
}

#[derive(Clone, Resource)]
pub struct PlayerAi {
    pub player_id: Uuid,
    pub player_type: String,
	pub hole_completion_state: bool,
	pub ball_material: Color, // For now custom material/shaders planned
	pub ball_location: Vec3,
	pub score: [i32; 18],
}

#[derive(Clone, Resource)]
pub struct PlayerRemote {
    pub player_id: Uuid,
    pub player_type: String,
	pub hole_completion_state: bool,
	pub ball_material: Color, // For now custom material/shaders planned
	pub ball_location: Vec3,
	pub score: [i32; 18],
}

// --- Party Handler --- //

#[derive(Resource)]
pub struct Party {
    players: Arc<Mutex<Vec<Arc<Mutex<dyn Player + Send>>>>>,
    active_player: Arc<Mutex<i32>>,
}

// --- Camera --- //

#[derive(Debug, Resource)]
pub struct CameraHandler {
    current_coords: Vec3,
}

#[derive(Asset, Component, TypePath)]
pub struct CameraWorld;

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
    active_ball_location: Arc<Mutex<Option<Vec3>>>,
    arrow_state: bool,
    network_server_connection: bool,
    remote_game: bool,
    remotely_pushed_state: Option<StateUpdateRef>,
    game_id: Option<Uuid>,
}

#[derive(Clone, Resource)]
pub struct GameRecord{
    game_id: Uuid,
    players: Vec<Uuid>,
    scores: Vec<[i32; 18]>,
}

impl GameRecord {
    pub fn unwrap(&self) -> (Uuid, Vec<Uuid>, Vec<[i32; 18]>) {
        (self.game_id, self.players.clone(), self.scores.clone())
    } 
}

#[derive(Debug)]
pub struct MapID {
    map: &'static str,
}

#[derive(Clone, Debug, Resource)]
pub struct GLBStorageID {
    glb: Arc<[MapID]>,
}

#[derive(Asset, Clone, Component, Debug, TypePath)]
pub struct GolfBallTag(pub String);

#[derive(Resource)]
pub struct LeaderBoard {
    current_scores: [i32; 18],
    past_games: Vec<GameRecord>,
}

#[derive(Resource)]
pub struct RunTrigger{
    camera_handler_cycle_state_camera: bool,
    game_handler_game_start: bool,
    game_handler_game_state_exit_routines: bool,
    game_handler_game_state_start_routines: bool,
    game_handler_update_players_manual_static_bonk_current_ball: bool,
    game_handler_update_players_ref_ball_locations: bool,
    game_handler_update_players_reset_ref_ball_locations: bool,
    game_handler_update_players_store_current_ball_locations_to_ref: bool,
    golf_ball_handler_spawn_golf_balls_for_party_members: bool,
    leader_board_log_game: bool,
    leader_board_review_last_game: bool,
    level_handler_init_level_game_handler_current_level: bool,
    level_handler_next_turn_protocol: bool,
    level_handler_purge_protocol: bool,
    level_handler_set_state_next_level: bool,
    level_handler_set_state_next_map_set: bool,
    network_get_client_state_game: bool,
    party_handler_active_player_add_bonk: bool,
    party_handler_active_player_set_ball_location: bool,
    party_handler_active_player_set_hole_completion_state_true: bool,
    party_handler_cycle_active_player: bool,
    party_handler_new_player_ai: bool,
    party_handler_new_player_local: bool,
    party_handler_new_player_remote: bool,
    party_handler_remove_ai: bool,
    party_handler_remove_last_player: bool,
    turn_handler_end_game: bool,
    turn_handler_next_round_prep: bool,
    turn_handler_set_turn_next: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MapSet {
    pub map_set_id: Uuid,
    pub map_set_name: String,
    pub created: OffsetDateTime, // Use time crate's OffsetDateTime to handle timestamp values
    pub last_updated: OffsetDateTime, // Use time crate's OffsetDateTime to handle timestamp values
    pub hole_range_start: i32,
    pub hole_range_end: i32,
    pub file_path_level_1: Option<String>,
    pub file_path_level_2: Option<String>,
    pub file_path_level_3: Option<String>,
    pub file_path_level_4: Option<String>,
    pub file_path_level_5: Option<String>,
    pub file_path_level_6: Option<String>,
    pub file_path_level_7: Option<String>,
    pub file_path_level_8: Option<String>,
    pub file_path_level_9: Option<String>,
    pub file_path_level_10: Option<String>,
    pub file_path_level_11: Option<String>,
    pub file_path_level_12: Option<String>,
    pub file_path_level_13: Option<String>,
    pub file_path_level_14: Option<String>,
    pub file_path_level_15: Option<String>,
    pub file_path_level_16: Option<String>,
    pub file_path_level_17: Option<String>,
    pub file_path_level_18: Option<String>,
}

#[derive(Event)]
pub struct SceneInstanceSpawnedEvent {
    pub entity: Entity,
}

pub struct UserInterface {}

#[derive(Resource)]
pub struct UiUpdateTimer(pub Timer);

#[derive(Debug, Event)]
pub struct UiUpdateEvent;