use bevy::prelude::*;
use bevy_rapier3d::prelude::RigidBody;

use uuid::Uuid;

// use std::fmt;
use std::sync::Arc;
use std::sync::Mutex;
// use std::sync::MutexGuard;

// Direct Imports
pub mod network_handler;
pub mod database_handler;

// Directory Imports
pub mod level_handler;
pub mod player_handler;
pub mod user_interface;

#[derive(Debug, Event)]
pub struct OnlineStateChange;


#[derive(Clone, Debug)]
pub enum StateUpdateRef {
    StateGameConnection(StateGameConnection),
    StateCameraOrbitEntity(StateCameraOrbitEntity),
    StateGame(StateGame),
    StateLevel(StateLevel),
    StateMapSet(StateMapSet),
    StateGamePlayStyle(StateGamePlayStyle),
    StateRunTrigger(StateRunTrigger),
    StateTurn(StateTurn),
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
    Active,
    NextTurn,
}

// Database //

use sqlx::MySqlPool;

#[derive(Resource)]
pub struct DatabasePool(pub MySqlPool);

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
pub struct StateText;

#[derive(Component)]
pub struct TitleText;

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
    active_ball_location: Option<Vec3>,
    arrow_state: bool,
    network_server_connection: bool,
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

#[derive(Resource)]
pub struct LeaderBoard {
    current_scores: [i32; 18],
    past_games: Vec<GameRecord>,
}

#[derive(Resource)]
pub struct RunTrigger{
    party_handler_active_player_add_bonk: bool,
    party_handler_active_player_set_ball_location: bool,
    party_handler_active_player_set_hole_completion_state_true: bool,
    party_handler_cycle_active_player: bool,
    party_handler_new_player_ai: bool,
    party_handler_new_player_local: bool,
    party_handler_new_player_remote: bool,
    party_handler_remove_ai: bool,
    party_handler_remove_last_player: bool,
    network_get_client_state_game: bool,
    game_handler_cycle_state_camera: bool,
    game_handler_cycle_state_map_set: bool,
    game_handler_cycle_current_level: bool,
    game_handler_get_active_ball_location: bool,
    game_handler_reset_active_ball_location: bool,
    game_handler_set_active_ball_location: bool,
    game_handler_state_turn_next_player_turn: bool,
    game_handler_start_game_local: bool,
    game_handler_toggle_state_game: bool,
    leader_board_log_game: bool,
    leader_board_review_last_game: bool,
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum StateRunTrigger {
    #[default]
    Idle,
    PartyHandlerActivePlayerAddBonk,
    PartyHandlerActivePlayerSetBallLocation,
    PartyHandlerActivePlayerSetHoleCompletionStateTrue,
    PartyHandlerCycleActivePlayer,
    NetworkGetClientStateGame,
    GameHandlerCycleStateCamera,
    GameHandlerCycleStateMapSet,
    GameHandlerCycleCurrentLevel,
    GameHandlerGetActiveBallLocation,
    GameHandlerResetActiveBallLocation,
    GameHandlerSetActiveBallLocation,
    GameHandlerStateTurnNextPlayerTurn,
    GameHandlerStartGameLocal,
    GameHandlerToggleStateGame,
    LeaderBoardLogGame,
    LeaderBoardReviewLastGame,
}

#[derive(Asset, Clone, Component, Debug, TypePath)]
pub struct GolfBallTag(pub usize);

