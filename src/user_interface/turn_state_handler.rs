use bevy::prelude::*;

// States
use crate::{
    CameraOrbitEntityState,
    GameState,
    LeaderBoardState,
    LevelState,
    MapSetState,
    PanOrbitState,
    TurnState,
};

// Resources
use crate::{
    Party,
    GameHandler,
    LevelHandler,
};

// --- OnEnter: Turn State --- //

pub fn turn_state_response_hole_complete(
    mut party: ResMut<Party>,
    map_set_state: Res<State<MapSetState>>,
    level: ResMut<State<LevelState>>,
    level_handler: Res<LevelHandler>,
    mut game_handler: ResMut<GameHandler>,
    mut next_leader_board_state: ResMut<NextState<LeaderBoardState>>,
    mut next_camera_state: ResMut<NextState<CameraOrbitEntityState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_level: ResMut<NextState<LevelState>>,
    mut next_turn: ResMut<NextState<TurnState>>,
    mut pan_orbit_camera_query: Query<&mut PanOrbitState>,
) {
    party.active_player_finished_hole(); // Reads active player index and updates target Player's state
    
    let current_level = party.get_active_level();
    let party_size = party.get_party_size();
    
    if party_size == 1 {
        let maps = match map_set_state.get() {
            MapSetState::Tutorial => {
                party.end_game(); // Sets players to NotInGame
                next_game_state.set(GameState::PostGameReview);
                next_turn.set(TurnState::Idle);
                game_handler.init_postgame_leaderboard(party); // Set's target for level handling
                next_leader_board_state.set(LeaderBoardState::PostGame);
                next_level.set(LevelState::MenuLeaderBoard);
                next_camera_state.set(CameraOrbitEntityState::LeaderBoard);
                for mut state in pan_orbit_camera_query.iter_mut() {
                    info!("{:?}", state);
                    state.radius = 38.0;
                    state.pitch = -12.0f32.to_radians();
                    state.yaw = -17.0f32.to_radians();
                }
            },
            MapSetState::WholeCorse => {
                if current_level == 18 {
                    todo!(); // End Game Leaderboard
                } else {
                    let set_next_level = level_handler.next_level(current_level);
                    next_level.set(set_next_level);
                    game_handler.next_level();
                    party.next_level();
                    next_turn.set(TurnState::Turn);
                }
            },
            MapSetState::FrontNine => {
                if current_level == 9 {
                    todo!(); // End Game Leaderboard
                } else {
                    let set_next_level = level_handler.next_level(current_level);
                    next_level.set(set_next_level);
                    game_handler.next_level();
                    party.next_level();
                    next_turn.set(TurnState::Turn);
                }
            },
            MapSetState::BackNine => {
                if current_level == 18 {
                    todo!(); // End Game Leaderboard
                } else {
                    let set_next_level = level_handler.next_level(current_level);
                    next_level.set(set_next_level);
                    game_handler.next_level();
                    party.next_level();
                    next_turn.set(TurnState::Turn);
                }
            },
            MapSetState::SelectAHole => {
                    todo!(); // End Game Leaderboard
            },
        };
    }
    
    // next_turn_state.set(TurnState::)
}

pub fn turn_state_response_turn_reset(
    mut party: ResMut<Party>,
    mut next_turn_state: ResMut<NextState<TurnState>>,
) {
}
// fn turn_state_response_new_game() {}
// fn turn_state_response_next_turn() {}
// fn turn_state_response_game_complete() {}