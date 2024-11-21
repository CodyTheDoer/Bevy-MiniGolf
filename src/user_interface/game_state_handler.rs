use bevy::prelude::*;

// States
use crate::{
    CameraOrbitEntityState,
    LeaderBoardState,
    LevelState,
    MenuState,
    TurnState,
    PanOrbitState,
};

// Resources
use crate::{
    Party,
};
// --- OnEnter: Game State --- //

pub fn game_state_response_menus(
    mut party: ResMut<Party>,
    mut next_leader_board_state: ResMut<NextState<LeaderBoardState>>,
    mut next_level: ResMut<NextState<LevelState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut next_turn: ResMut<NextState<TurnState>>,
    mut next_camera_state: ResMut<NextState<CameraOrbitEntityState>>,
    mut pan_orbit_camera_query: Query<&mut PanOrbitState>,
){
    next_menu_state.set(MenuState::NoSelection);
    next_turn.set(TurnState::Idle);
    next_leader_board_state.set(LeaderBoardState::Mixed);
    next_level.set(LevelState::MainMenu);
}


