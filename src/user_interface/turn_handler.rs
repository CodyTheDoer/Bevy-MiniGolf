use bevy::prelude::*;

// States
use crate::{
    StateGame,
    StateTurn,
};

// Resources
use crate::RunTrigger;

pub fn turn_handler_set_turn_next(
    mut run_trigger: ResMut<RunTrigger>,
    state_game: Res<State<StateGame>>,
    mut next_state_turn: ResMut<NextState<StateTurn>>,
) {
    info!("function: turn_handler_set_turn_next"); 
    match state_game.get() {
        StateGame::InGame => {
            run_trigger.set_target("game_handler_reset_active_ball_location", true);
            next_state_turn.set(StateTurn::NextTurn);
            run_trigger.set_target("party_handler_cycle_active_player", true);
            run_trigger.set_target("game_handler_get_active_ball_location", true);
            next_state_turn.set(StateTurn::Active);
        },
        StateGame::NotInGame => {},
    };
    run_trigger.set_target("turn_handler_set_turn_next", false);
    info!("post response: turn_handler_set_turn_next");  
}