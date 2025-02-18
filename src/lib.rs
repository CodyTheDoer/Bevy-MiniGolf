use bevy::prelude::*;

use uuid::Uuid;

use rusqlite::Connection;
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;

use std::sync::Arc;
use std::sync::Mutex;

// Direct Imports
pub mod database_handler;
pub mod game_handler;
pub mod network_handler;

// Directory Imports
pub mod level_handler;
pub mod player_handler;
pub mod user_interface;

#[derive(Clone, Debug, Resource)] 
pub struct BonkHandler {
    pub direction: Vec3,
    pub power: f32,
    pub cursor_origin_position: XYMatrix,
    pub cursor_origin_position_updated: bool,
    pub cursor_bonk_position: XYMatrix,
    pub cursor_bonk_position_updated: bool,
}

#[derive(Debug, Resource)]
pub struct CameraHandler {
    current_coords: Vec3,
}

#[derive(Asset, Component, TypePath)]
pub struct CameraUi;

#[derive(Asset, Component, TypePath)]
pub struct CameraWorld;

pub enum CheckStateGH {
    AllSleeping,
    ArrowState,
    EnvironmentLoaded,
    GolfBallsBonkTrigger,
    GolfBallsLoaded,
    GolfBallsReset,
    GolfBallsStoreLocation,
    InGame,
    RoundStart,
    NetworkServerConnection,
    RemoteGame,
}/*
CheckStateGH::AllSleeping
CheckStateGH::ArrowState
CheckStateGH::EnvironmentLoaded
CheckStateGH::GolfBallsBonkTrigger
CheckStateGH::GolfBallsLoaded
CheckStateGH::GolfBallsReset
CheckStateGH::GolfBallsStoreLocation
CheckStateGH::InGame
CheckStateGH::RoundStart
CheckStateGH::NetworkServerConnection
CheckStateGH::RemoteGame
*/

pub enum CheckStatePH {
    EnvironmentPurged,
    GolfBallsPurged,
}/*
CheckStatePH::EnvironmentPurged
CheckStatePH::GolfBallsPurged
*/

pub enum CheckStateRT {
    AddPhysicsQueryAndUpdateScene,
    CameraHandlerCycleStateCamera,
    GameHandlerGameStart,
    GameHandlerGameStateExitRoutines,
    GameHandlerGameStateStartRoutines,
    GameHandlerStartLocalBackNine,
    GameHandlerStartLocalFrontNine,
    GameHandlerStartLocalSelectAHole,
    GameHandlerStartLocalWholeCorse,
    GameHandlerStartTutorial,
    GolfBallHandlerEndGame,
    GolfBallHandlerPartyStoreLocations,
    GolfBallHandlerResetGolfBallLocations,
    GolfBallHandlerSpawnGolfBallsForPartyMembers,
    GolfBallHandlerUpdateLocationsPostBonk,
    LeaderBoardLogGame,
    LeaderBoardReviewLastGame,
    LevelHandlerInitLevelGameHandlerCurrentLevel,
    LevelHandlerNextTurnProtocol,
    LevelHandlerPurgeProtocol,
    LevelHandlerSetStateNextLevel,
    LevelHandlerSetStateNextMapSet,
    NetworkGetClientStateAll,
    NetworkGetClientStateGame,
    PartyHandlerActivePlayerAddBonk,
    PartyHandlerActivePlayerSetHoleCompletionStateTrue,
    PartyHandlerCycleActivePlayer,
    PartyHandlerNewPlayerAi,
    PartyHandlerNewPlayerLocal,
    PartyHandlerNewPlayerRemote,
    PartyHandlerRemoveAi,
    PartyHandlerRemoveLastPlayer,
    PartyHandlerRemoveLocalPlayer,
    TurnHandlerEndGame,
    TurnHandlerNextRoundPrep,
    TurnHandlerSetTurnNext,
    StartMovementListenerTurnHandlerSetTurnNext,
}/*
CheckStateRT::AddPhysicsQueryAndUpdateScene
CheckStateRT::CameraHandlerCycleStateCamera
CheckStateRT::GameHandlerGameStart
CheckStateRT::GameHandlerGameStateExitRoutines
CheckStateRT::GameHandlerGameStateStartRoutines
CheckStateRT::GameHandlerStartLocalBackNine
CheckStateRT::GameHandlerStartLocalFrontNine
CheckStateRT::GameHandlerStartLocalSelectAHole
CheckStateRT::GameHandlerStartLocalWholeCorse
CheckStateRT::GameHandlerStartTutorial
CheckStateRT::GolfBallHandlerEndGame
CheckStateRT::GolfBallHandlerPartyStoreLocations
CheckStateRT::GolfBallHandlerResetGolfBallLocations
CheckStateRT::GolfBallHandlerSpawnGolfBallsForPartyMembers
CheckStateRT::GolfBallHandlerUpdateLocationsPostBonk
CheckStateRT::LeaderBoardLogGame
CheckStateRT::LeaderBoardReviewLastGame
CheckStateRT::LevelHandlerInitLevelGameHandlerCurrentLevel
CheckStateRT::LevelHandlerNextTurnProtocol
CheckStateRT::LevelHandlerPurgeProtocol
CheckStateRT::LevelHandlerSetStateNextLevel
CheckStateRT::LevelHandlerSetStateNextMapSet
CheckStateRT::NetworkGetClientStateAll
CheckStateRT::NetworkGetClientStateGame
CheckStateRT::PartyHandlerActivePlayerAddBonk
CheckStateRT::PartyHandlerActivePlayerSetHoleCompletionStateTrue
CheckStateRT::PartyHandlerCycleActivePlayer
CheckStateRT::PartyHandlerNewPlayerAi
CheckStateRT::PartyHandlerNewPlayerLocal
CheckStateRT::PartyHandlerNewPlayerRemote
CheckStateRT::PartyHandlerRemoveAi
CheckStateRT::PartyHandlerRemoveLastPlayer
CheckStateRT::PartyHandlerRemoveLocalPlayer
CheckStateRT::TurnHandlerEndGame
CheckStateRT::TurnHandlerNextRoundPrep
CheckStateRT::TurnHandlerSetTurnNext
CheckStateRT::StartMovementListenerTurnHandlerSetTurnNext
*/

#[derive(Clone, PartialEq, Eq, Hash, Resource,)]
pub struct ClientProtocol {}

impl ClientProtocol {
    pub fn new() -> Self {ClientProtocol{}}

    pub fn all_states_packet(&self) -> String {
        String::from("PacketAllStates")
    }

    pub fn heart_beat_packet(&self) -> String {
        String::from("PacketHeartBeat")
    }

    pub fn init_player_connection(&self) -> String {
        String::from("InitPlayerConnection")
    }
}

#[derive(Resource)]
pub struct DatabaseConnection{
    pub conn: Arc<Mutex<Connection>>,
}

#[derive(Resource)]
pub struct HeartbeatTimer(pub Timer);

#[derive(Asset, Clone, Component, Debug, TypePath)]
pub struct Interactable; 

#[derive(Resource)]
pub struct GameHandler {
    check_all_sleeping: bool,
    check_arrow_state: bool,
    check_environment_loaded: bool,
    check_golf_balls_bonk_trigger: bool,
    check_golf_balls_loaded: bool,
    check_golf_balls_reset: bool,
    check_golf_balls_store_location: bool,
    check_in_game: bool,
    check_round_start: bool,
    check_network_server_connection: bool,
    check_remote_game: bool,
    current_level: i32,
    add_physics_attempts: i32,
    game_id: Option<Uuid>,
    remotely_pushed_state: Option<StateUpdateRef>,
}

#[derive(Clone, Resource)]
pub struct GameRecord{
    game_id: Uuid,
    players: Vec<Uuid>,
    scores: Vec<[i32; 18]>,
}

#[derive(Clone, Debug, Resource)]
pub struct GLBStorageID {
    glb: Arc<[MapID]>,
}

#[derive(Asset, Clone, Component, Debug, TypePath)]
pub struct GolfBall (pub GolfBallPosition);

#[derive(Clone, Debug)]
pub struct GolfBallPosition {
    pub uuid: Uuid,
    pub position: Vec3,
    pub last_position: Vec3,
    pub sleeping: bool,
}

#[derive(Component)]
pub struct Ground;

#[derive(Resource)]
pub struct LeaderBoard {
    current_scores: [i32; 18],
    past_games: Vec<GameRecord>,
}

#[derive(Debug)]
pub struct MapID {
    map: &'static str,
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

#[derive(Debug, Event)]
pub struct OnlineStateChange;

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
    pub orbit_key: Option<MouseButton>,
    /// Key to hold for zooming
    pub zoom_key: Option<KeyCode>,
    /// What action is bound to the scroll wheel?
    pub scroll_action: Option<PanOrbitAction>,
    /// For devices with a notched scroll wheel, like desktop mice
    pub scroll_line_sensitivity: f32,
    /// For devices with smooth scrolling, like touchpads
    pub scroll_pixel_sensitivity: f32,
}

#[derive(Resource)]
pub struct Party {
    players: Arc<Mutex<Vec<Arc<Mutex<dyn Player + Send>>>>>,
    active_player: Arc<Mutex<i32>>,
    pub ai_vec: Option<Vec<usize>>,
}

#[derive(Resource)]
pub struct PhysicsHandler;

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
}

#[derive(Clone, Resource)]
pub struct PlayerAi {
    pub player_id: Uuid,
    pub player_type: String,
	pub hole_completion_state: bool,
	pub ball_material: Color, // For now custom material/shaders planned
	pub score: [i32; 18],
}

#[derive(Clone, Resource)]
pub struct PlayerLocal {
    pub player_id: Uuid,
    pub player_type: String,
	pub hole_completion_state: bool,
	pub ball_material: Color, // For now custom material/shaders planned
	pub score: [i32; 18],
}

#[derive(Clone, Resource)]
pub struct PlayerRemote {
    pub player_id: Uuid,
    pub player_type: String,
	pub hole_completion_state: bool,
	pub ball_material: Color, // For now custom material/shaders planned
	pub score: [i32; 18],
}

#[derive(Clone, Resource)]
pub struct PurgeHandler {
    pub environment_purged: bool,
    pub golf_balls_purged: bool,
}

#[derive(Component)]
pub struct ResetTimer {
    pub timer: Timer,
}

#[derive(Debug, Event)]
pub struct SceneInstancePurgedEnvironment {}

#[derive(Debug, Event)]
pub struct SceneInstancePurgedGolfBalls {}

#[derive(Debug, Event)]
pub struct SceneInstanceRespawnedGolfBall {
    pub entity: Entity,
    pub id: Uuid, 
    pub location: Vec3,
}

#[derive(Debug, Event)]
pub struct SceneInstanceOutOfBoundGolfBall {
    pub info_vec: Vec<(Uuid, Vec3)>,
}

#[derive(Debug, Event)]
pub struct SceneInstanceSpawnedEnvironment {
    pub entity: Entity,
}

#[derive(Debug, Event)]
pub struct SceneInstanceSpawnedGolfBalls {
    pub entity: Entity,
}

#[derive(Component)]
pub struct SpawnPhysicsCheckTimer {
    pub timer: Timer,
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
    ToBeSelected,
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
    InGame,
}

#[derive(Component, Debug)]
pub struct StatePanOrbit {
    pub center: Vec3,
    pub radius: f32,
    pub upside_down: bool,
    pub pitch: f32,
    pub yaw: f32,
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
pub struct StatesRef {
    all_states: Vec<String>,
}

impl StatesRef {
    pub fn new() -> StatesRef {
        let all_states: Vec<String> = Vec::new();
        StatesRef {
            all_states,
        }
    }
}

#[derive(Resource)]
pub struct RunTrigger{
    add_physics_query_and_update_scene:bool,
    camera_handler_cycle_state_camera: bool,
    game_handler_game_start: bool,
    game_handler_game_state_exit_routines: bool,
    game_handler_game_state_start_routines: bool,
    game_handler_start_local_back_nine: bool,
    game_handler_start_local_front_nine: bool,
    game_handler_start_local_select_a_hole: bool,
    game_handler_start_local_whole_corse: bool,
    game_handler_start_tutorial: bool,
    golf_ball_handler_end_game: bool,
    golf_ball_handler_party_store_locations: bool,
    golf_ball_handler_reset_golf_ball_locations: bool,
    golf_ball_handler_spawn_golf_balls_for_party_members: bool,
    golf_ball_handler_update_locations_post_bonk: bool,
    leader_board_log_game: bool,
    leader_board_review_last_game: bool,
    level_handler_init_level_game_handler_current_level: bool,
    level_handler_next_turn_protocol: bool,
    level_handler_purge_protocol: bool,
    level_handler_set_state_next_level: bool,
    level_handler_set_state_next_map_set: bool,
    network_get_client_state_all: bool,
    network_get_client_state_game: bool,
    party_handler_active_player_add_bonk: bool,
    party_handler_active_player_set_hole_completion_state_true: bool,
    party_handler_cycle_active_player: bool,
    party_handler_new_player_ai: bool,
    party_handler_new_player_local: bool,
    party_handler_new_player_remote: bool,
    party_handler_remove_ai: bool,
    party_handler_remove_last_player: bool,
    party_handler_remove_local_player: bool,
    turn_handler_end_game: bool,
    turn_handler_next_round_prep: bool,
    turn_handler_set_turn_next: bool,
    start_movement_listener_turn_handler_set_turn_next: bool,
}

#[derive(Resource)]
pub struct UpdateIdResource {
    pub update_id: Option<Uuid>,
}

pub struct UserInterface {}

#[derive(Clone, Debug)]
pub struct XYMatrix {
    pub x: f32,
    pub y: f32, 
}