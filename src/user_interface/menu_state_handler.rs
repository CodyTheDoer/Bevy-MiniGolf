use bevy::prelude::*;

// States
use crate::{
    CameraOrbitEntityState,
    GameState,
    LeaderBoardState,
    LevelState,
    MapSetState,
    MenuState,
    PanOrbitState,
    TurnState,
};

// Resources
use crate::{
    Party,
    GameHandler,
};

// --- OnEnter: Menu State --- //

pub fn menu_state_response_local(
    mut game_handler: ResMut<GameHandler>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_level_state: ResMut<NextState<LevelState>>,
    mut next_camera_state: ResMut<NextState<CameraOrbitEntityState>>,
    mut pan_orbit_camera_query: Query<&mut PanOrbitState>,
) {
    game_handler.init_menu_local();
    next_game_state.set(GameState::GameInitLocal);
    next_level_state.set(LevelState::MenuLocal);    
    next_camera_state.set(CameraOrbitEntityState::MenuLocal);
    for mut state in pan_orbit_camera_query.iter_mut() {
        info!("{:?}", state);
        state.radius = 38.0;
        state.pitch = -12.0f32.to_radians();
        state.yaw = 17.0f32.to_radians();
    }
}

pub fn menu_state_response_online(
    mut game_handler: ResMut<GameHandler>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_level_state: ResMut<NextState<LevelState>>,
    mut next_camera_state: ResMut<NextState<CameraOrbitEntityState>>,
    mut pan_orbit_camera_query: Query<&mut PanOrbitState>,
) {
    game_handler.init_menu_online();
    next_game_state.set(GameState::GameInitOnline);
    next_level_state.set(LevelState::MenuOnline);    
    next_camera_state.set(CameraOrbitEntityState::GameInit);
    for mut state in pan_orbit_camera_query.iter_mut() {
        info!("{:?}", state);
        state.radius = 38.0;
        state.pitch = -12.0f32.to_radians();
        state.yaw = 17.0f32.to_radians();
    }
}

pub fn menu_state_response_leader_board(
    mut game_handler: ResMut<GameHandler>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_level_state: ResMut<NextState<LevelState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut next_camera_state: ResMut<NextState<CameraOrbitEntityState>>,
    mut pan_orbit_camera_query: Query<&mut PanOrbitState>,
) {
    game_handler.init_menu_leader_board();
    next_menu_state.set(MenuState::LeaderBoard);
    next_game_state.set(GameState::LeaderBoard);
    next_level_state.set(LevelState::MenuLeaderBoard);
    next_camera_state.set(CameraOrbitEntityState::LeaderBoard);
    for mut state in pan_orbit_camera_query.iter_mut() {
        info!("{:?}", state);
        state.radius = 38.0;
        state.pitch = -12.0f32.to_radians();
        state.yaw = -7.0f32.to_radians();
    }
}

pub fn menu_state_response_tutorial(
    mut party: ResMut<Party>,
    mut game_handler: ResMut<GameHandler>,
    mut next_level_state: ResMut<NextState<LevelState>>,
    mut next_leader_board_state: ResMut<NextState<LeaderBoardState>>,
    mut next_camera_state: ResMut<NextState<CameraOrbitEntityState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut next_map_set_state: ResMut<NextState<MapSetState>>,
    mut next_turn_state: ResMut<NextState<TurnState>>,
    mut pan_orbit_camera_query: Query<&mut PanOrbitState>,
) {
    party.start_game();
    game_handler.init_tutorial();
    next_level_state.set(LevelState::HoleTutorial);
    next_menu_state.set(MenuState::NoSelection);
    next_leader_board_state.set(LeaderBoardState::InGame);
    next_map_set_state.set(MapSetState::Tutorial);
    next_game_state.set(GameState::InGame);
    next_turn_state.set(TurnState::Turn);
    next_camera_state.set(CameraOrbitEntityState::Ball);
    for mut state in pan_orbit_camera_query.iter_mut() {
        state.radius = 2.0;
        state.pitch = -8.0f32.to_radians();
        state.yaw = 22.0f32.to_radians();
    }
}

pub fn menu_state_response_player(
    mut game_handler: ResMut<GameHandler>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_level_state: ResMut<NextState<LevelState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut next_camera_state: ResMut<NextState<CameraOrbitEntityState>>,
    mut pan_orbit_camera_query: Query<&mut PanOrbitState>,
) {
    game_handler.init_menu_player();
    next_level_state.set(LevelState::MenuPlayer);
    next_menu_state.set(MenuState::Player);
    next_game_state.set(GameState::MenuPlayer);
    next_camera_state.set(CameraOrbitEntityState::MenuPlayer);
    for mut state in pan_orbit_camera_query.iter_mut() {
        info!("{:?}", state);
        state.radius = 38.0;
        state.pitch = -12.0f32.to_radians();
        state.yaw = -10.0f32.to_radians();
    }
}

pub fn menu_state_response_preferences(
    mut game_handler: ResMut<GameHandler>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_level_state: ResMut<NextState<LevelState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut next_camera_state: ResMut<NextState<CameraOrbitEntityState>>,
    mut pan_orbit_camera_query: Query<&mut PanOrbitState>,
) {
    game_handler.init_menu_preferences();
    next_menu_state.set(MenuState::Preferences);
    next_game_state.set(GameState::Preferences);
    next_level_state.set(LevelState::MenuPreferences);
    next_camera_state.set(CameraOrbitEntityState::MenuPreferences);
    for mut state in pan_orbit_camera_query.iter_mut() {
        info!("{:?}", state);
        state.radius = 38.0;
        state.pitch = -12.0f32.to_radians();
        state.yaw = -12.0f32.to_radians();
    }
}