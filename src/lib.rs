use bevy::prelude::*;
use bevy_rapier3d::prelude::RigidBody;

use std::fmt;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

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

// --- State Enums --- //
#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum ArrowState {
    #[default]
    Idle,
    DrawingArrow,
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum CameraOrbitEntityState {
    #[default]
    MainMenu,
    Ball,
    Cup,
    FreePan,
    LeaderBoard,
    GameInit,
    MenuLocal,
    MenuOnline,
    MenuPreferences,
    MenuPlayer,
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum GameState {
    #[default]
    GameInitLocal,
    GameInitOnline,
    LeaderBoard,
    LoadingScreen,
    InGame,
    InGamePaused,
    Menus,
    PostGameReview,
    Preferences,
    MenuPlayer,
}

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
    MenuLeaderBoard,
    MenuLocal,
    MenuOnline,
    MenuPreferences,
    MenuPlayer,
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
pub enum MenuState {
    #[default]
    NoSelection,
    Online,
    Local,
    Tutorial,
    LeaderBoard,
    Preferences,
    Player,
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum ConnectionState {
    #[default]
    Local,
    Online,
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum PlayerCompletionState {
    #[default]
    NotInGame,
    HoleIncomplete,
    HoleCompleted,
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum PlayThroughStyleState {
    #[default]
    Proximity,
    RandomSetOrder,
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum TurnState {
    #[default]
    Idle,
    Turn,
    TurnReset,
    NextTurn,
    HoleComplete,
    GameComplete,
}

// --- Player Handler --- //

#[derive(Clone, Resource)]
pub struct Player {
    pub player_id: String,
	pub hole_completion_state: PlayerCompletionState,
	pub ball_material: Color, // For now custom material/shaders planned
	pub ball_location: Vec3,
	pub puts_count_total: u32,
	pub puts_hole_1: u32,
	pub puts_hole_2: u32,
	pub puts_hole_3: u32,
	pub puts_hole_4: u32,
	pub puts_hole_5: u32,
	pub puts_hole_6: u32,
	pub puts_hole_7: u32,
	pub puts_hole_8: u32,
	pub puts_hole_9: u32,
	pub puts_hole_10: u32,
	pub puts_hole_11: u32,
	pub puts_hole_12: u32,
	pub puts_hole_13: u32,
	pub puts_hole_14: u32,
	pub puts_hole_15: u32,
	pub puts_hole_16: u32,
	pub puts_hole_17: u32,
	pub puts_hole_18: u32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            player_id: String::from("Player@email.com"),
            hole_completion_state: PlayerCompletionState::NotInGame,
            ball_material: Color::srgb(1.0, 0.0, 1.0),
            ball_location: Vec3::new(0.0, 0.0, 0.0),
            puts_count_total: 0,
            puts_hole_1: 0,
            puts_hole_2: 0,
            puts_hole_3: 0,
            puts_hole_4: 0,
            puts_hole_5: 0,
            puts_hole_6: 0,
            puts_hole_7: 0,
            puts_hole_8: 0,
            puts_hole_9: 0,
            puts_hole_10: 0,
            puts_hole_11: 0,
            puts_hole_12: 0,
            puts_hole_13: 0,
            puts_hole_14: 0,
            puts_hole_15: 0,
            puts_hole_16: 0,
            puts_hole_17: 0,
            puts_hole_18: 0,
        }
    }

    pub fn start_game(&mut self) {
        self.hole_completion_state = PlayerCompletionState::HoleIncomplete;
        self.ball_location = Vec3::new(0.0, 0.0, 0.0);
        self.puts_count_total = 0;
        self.puts_hole_1 = 0;
        self.puts_hole_2 = 0;
        self.puts_hole_3 = 0;
        self.puts_hole_4 = 0;
        self.puts_hole_5 = 0;
        self.puts_hole_6 = 0;
        self.puts_hole_7 = 0;
        self.puts_hole_8 = 0;
        self.puts_hole_9 = 0;
        self.puts_hole_10 = 0;
        self.puts_hole_11 = 0;
        self.puts_hole_12 = 0;
        self.puts_hole_13 = 0;
        self.puts_hole_14 = 0;
        self.puts_hole_15 = 0;
        self.puts_hole_16 = 0;
        self.puts_hole_17 = 0;
        self.puts_hole_18 = 0;
    }

    pub fn hole_completed(&mut self) {
        self.hole_completion_state = PlayerCompletionState::HoleCompleted;
    }

    pub fn end_game(&mut self) {
        self.hole_completion_state = PlayerCompletionState::NotInGame;
    }

    pub fn add_put(&mut self, hole: i32) {
    }

    pub fn get_id(&self) -> String {
        self.player_id.clone()
    }
}

// --- Party Handler --- //

#[derive(Resource)]
pub struct Party {
    players: Arc<Mutex<Vec<Arc<Mutex<Player>>>>>,
    players_finished: Arc<Mutex<i32>>,
    active_player: Arc<Mutex<i32>>,
    active_level: Arc<Mutex<i32>>,
}

impl Party {
    pub fn new() -> Self {
        let players: Arc<Mutex<Vec<Arc<Mutex<Player>>>>> = Arc::new(Mutex::new(vec![Arc::new(Mutex::new(Player::new()))]));
        let players_finished: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
        let active_player: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
        let active_level: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
        Party {
            players,
            players_finished,
            active_player,
            active_level,
        } 
    }
    
    pub fn add_player(&self) {
        let new_player: Arc<Mutex<Player>> = Arc::new(Mutex::new(Player::new()));
        let mut players_lock = self.players.lock().unwrap(); // Acquire the lock to get mutable access
        players_lock.push(new_player);
    }

    pub fn get_players_finished(&self) -> i32 {
        let count = self.players_finished.lock().unwrap();
        *count
    }

    pub fn log_player_finished(&mut self) {
        let mut count = self.players_finished.lock().unwrap();
        *count += 1;
    }

    pub fn reset_players_finished(&mut self) {
        let mut count = *self.players_finished.lock().unwrap();
        count = 0;
    }

    pub fn get_main_player(&self) -> Player {
        // First, lock the players mutex to get access to the Vec
        let players_lock = self.players.lock().unwrap();

        // Get the active player (Arc<Mutex<Player>>)
        let player_arc = &players_lock[0];

        // Lock the player mutex to get a mutable reference to the player
        let player = player_arc.lock().unwrap();
        
        // Return the dereferenced player
        player.clone()
    }

    pub fn active_player_finished_hole(&mut self) {
        self.log_player_finished();

        // Get the active player index
        let active_player_index = *self.active_player.lock().unwrap();
        
        // First, lock the players mutex to get access to the Vec
        let players_lock = self.players.lock().unwrap();

        // Get the active player (Arc<Mutex<Player>>)
        let player_arc = &players_lock[active_player_index as usize];
        
        // Lock the player mutex to get a mutable reference to the player
        let mut player = player_arc.lock().unwrap();
        player.hole_completed();
    }

    pub fn all_finished(&self) -> bool {
        // Verify if all players have completed
        let player_count: i32 = self.get_party_size().try_into().unwrap();
        self.get_players_finished() == player_count
    }

    pub fn next_proximity_player(&self, ) {
        todo!(); 
    }

    pub fn next_set_order_player(&self, ) {
        todo!(); 
    }

    pub fn get_active_level(&self) -> i32 {
        let active_level = *self.active_level.lock().unwrap();
        active_level
    }

    pub fn get_party_size(&self) -> usize {        
        // First, lock the players mutex to get access to the Vec
        let players_lock = self.players.lock().unwrap();

        // Grab the size of the party
        let party_size = &players_lock.len();
        *party_size 
    }

    pub fn next_level(&mut self, ) {
        todo!(); 
    }

    pub fn start_game(&mut self) {
        // First, lock the players mutex to get access to the Vec
        let players_lock = self.players.lock().unwrap();

        for player in 0..players_lock.len() {
            // Get the active player (Arc<Mutex<Player>>)
            let player_arc = &players_lock[player];
            
            // Lock the player mutex to get a mutable reference to the player
            let mut player = player_arc.lock().unwrap();
            player.start_game();
        }
    }

    pub fn end_game(&mut self) {
        // First, lock the players mutex to get access to the Vec
        let players_lock = self.players.lock().unwrap();

        for player in 0..players_lock.len() {
            // Get the active player (Arc<Mutex<Player>>)
            let player_arc = &players_lock[player];
            
            // Lock the player mutex to get a mutable reference to the player
            let mut player = player_arc.lock().unwrap();
            player.end_game();
        }
    }
}

// --- LeaderBoard Handler --- //


// --- Physics Handler --- //

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

#[derive(Clone, Debug)]
pub struct LevelStateID {
    level: Arc<LevelState>,
}

#[derive(Clone, Debug, Resource)]
pub struct LevelHandler {
    level_states: Arc<[LevelStateID]>,
}

impl LevelHandler {
    pub fn new() -> LevelHandler {
        let level_state_names = [
            LevelState::MainMenu,
            LevelState::Hole1,
            LevelState::Hole2,
            LevelState::Hole3,
            LevelState::Hole4,
            LevelState::Hole5,
            LevelState::Hole6,
            LevelState::Hole7,
            LevelState::Hole8,
            LevelState::Hole9,
            LevelState::Hole10,
            LevelState::Hole11,
            LevelState::Hole12,
            LevelState::Hole13,
            LevelState::Hole14,
            LevelState::Hole15,
            LevelState::Hole16,
            LevelState::Hole17,
            LevelState::Hole18,
            LevelState::HoleTutorial,
            LevelState::MenuLeaderBoard,
            LevelState::MenuLocal,
            LevelState::MenuOnline,
            LevelState::MenuPreferences,
            LevelState::MenuPlayer,
        ];
        let level_state_ids: Vec<LevelStateID> = level_state_names
            .iter()
            .map(|level_state| LevelStateID { level: Arc::new(level_state.clone()) })
            .collect();
        LevelHandler {
            level_states: level_state_ids.into_boxed_slice().into(), // Vec -> Box -> Arc
        }
    }

    pub fn get_level(&self, level: i32) -> LevelState {
        let level_state_id = self.level_states.get(level as usize).unwrap();
        level_state_id.level.as_ref().clone()
    }

    pub fn next_level(&self, current_level: i32) -> LevelState {
        self.get_level(current_level + 1)
    }
}

#[derive(Component)]
pub struct Ground;

#[derive(Asset, Component, Debug, TypePath)]
pub struct Interactable; 

#[derive(Debug)]
pub struct MapID {
    map: &'static str,
}

#[derive(Clone, Debug, Resource)]
pub struct GLBStorageID {
    glb: Arc<[MapID]>,
}

impl GLBStorageID {
    pub fn new() -> Self {
        let map_paths = [
            "glb/menu/main_menu.glb",           // 0
            "glb/map/level_1.glb",              // 1
            "glb/map/level_1.glb",              // 2
            "glb/map/level_1.glb",              // 3
            "glb/map/level_1.glb",              // 4
            "glb/map/level_1.glb",              // 5
            "glb/map/level_1.glb",              // 6
            "glb/map/level_1.glb",              // 7
            "glb/map/level_1.glb",              // 8
            "glb/map/level_1.glb",              // 9
            "glb/map/level_1.glb",              // 10
            "glb/map/level_1.glb",              // 11
            "glb/map/level_1.glb",              // 12
            "glb/map/level_1.glb",              // 13
            "glb/map/level_1.glb",              // 14
            "glb/map/level_1.glb",              // 15
            "glb/map/level_1.glb",              // 16
            "glb/map/level_1.glb",              // 17
            "glb/map/level_1.glb",              // 18
            "glb/map/level_1.glb",              // 19
            "glb/menu/menu_leader_board.glb",   // 20
            "glb/menu/menu_local.glb",          // 21
            "glb/menu/menu_online.glb",         // 22
            "glb/menu/menu_preferences.glb",    // 23
            "glb/menu/menu_player.glb",         // 24
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

pub struct SceneLoadedEvent {
    pub entity: Entity,
}

// --- User Interface --- //

pub struct UserInterface {}

impl UserInterface {
    pub fn select_a_hole_widget() -> i32 {
        let target = 0;
        target
    }
}

// Define marker components to find the entities later
#[derive(Component)]
pub struct StateText;

#[derive(Component)]
pub struct TitleText;





#[derive(Clone, Debug)]
pub enum StateUpdateRef {
    ConnectionState(ConnectionState),
    GameState(GameState),
    LevelState(LevelState),
}

#[derive(Resource)]
pub struct GameHandler {
    current_level: i32,
    arrow_state: bool,
    network_server_connection: bool,
    remotely_pushed_state: Option<StateUpdateRef>,
}

impl GameHandler {
    pub fn new() -> Self {
        let current_level = 0;
        let arrow_state = false;
        let network_server_connection = false;
        let remotely_pushed_state = None;
        GameHandler {
            current_level,
            arrow_state,
            network_server_connection,
            remotely_pushed_state,
        }
    }

    // Level Handling logic
    pub fn next_level(&mut self) {
        self.current_level += 1;
    }

    pub fn get_current_level(&self) -> i32 {
        self.current_level
    }

    pub fn set_current_level(&mut self, level: i32) {
        self.current_level = level;
    }

    pub fn init_postgame_leaderboard(
        &mut self, 
        mut party: ResMut<Party>,
    ) {
        // Eventually submit party info to leaderboard system
        self.set_current_level(20);
    }

    pub fn init_tutorial(&mut self) {
        // Eventually submit party info to leaderboard system
        self.set_current_level(19);
    }

    pub fn init_menu_main(&mut self) {
        // Eventually submit party info to leaderboard system
        self.set_current_level(0);
    }

    pub fn init_menu_leader_board(&mut self) {
        // Eventually submit party info to leaderboard system
        self.set_current_level(20);
    }

    pub fn init_menu_local(&mut self) {
        // Eventually submit party info to leaderboard system
        self.set_current_level(21);
    }

    pub fn init_menu_online(&mut self) {
        // Eventually submit party info to leaderboard system
        self.set_current_level(22);
    }

    pub fn init_menu_preferences(&mut self) {
        self.set_current_level(23);
    }

    pub fn init_menu_player(&mut self) {
        self.set_current_level(24);
    }

    // Bonk UI Logic
    pub fn get_arrow_state(&self) -> bool {
        self.arrow_state
    }

    pub fn set_arrow_state_true(&mut self) {
        self.arrow_state = true;
    }

    pub fn set_arrow_state_false(&mut self) {
        self.arrow_state = false;
    }
    
    // Remote Auth Server Logic
    pub fn is_connected(&self) -> bool {
        self.network_server_connection
    }
    
    pub fn is_not_connected(&self) -> bool {
        if self.network_server_connection == false {
            true
        } else {
            false
        }
    }
    
    pub fn set_connected_false(&mut self) {
        self.network_server_connection = false;
    }
    
    pub fn set_connected_true(&mut self) {
        self.network_server_connection = true;
    }

    pub fn auth_server_handshake_received(
        &mut self, 
        parsed_state: Option<StateUpdateRef>,
    ) {
        self.remotely_pushed_state = Some(parsed_state.unwrap());
    }

    pub fn get_pushed_state(&self) -> StateUpdateRef {
        self.remotely_pushed_state.clone().expect("Push State get failed.")
    }
}

// --- User Interface --> CameraUI --- //

#[derive(Asset, Component, TypePath)]
pub struct CameraUi;

// --- User Interface --> CameraWorld --- //

#[derive(Asset, Component, TypePath)]
pub struct CameraWorld;

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