use bevy::prelude::*;

// States
use crate::{ 
    StateGame, 
    StateTurn,
};

// Resources
use crate::{
    GameHandler,
    RunTrigger,
};

pub fn turn_handler_set_turn_next(
    mut run_trigger: ResMut<RunTrigger>,
    game_handler: ResMut<GameHandler>,
    state_game: Res<State<StateGame>>,
    mut next_state_turn: ResMut<NextState<StateTurn>>,
) {
    info!("function: turn_handler_set_turn_next"); 
    match state_game.get() {
        StateGame::InGame => {
            if game_handler.remote_game_get() {

            } else {    
                run_trigger.set_target("game_handler_update_players_reset_ref_ball_locations", true);
                next_state_turn.set(StateTurn::NextTurn);
                run_trigger.set_target("party_handler_cycle_active_player", true);
                run_trigger.set_target("game_handler_update_players_store_current_ball_locations_to_ref", true);
                next_state_turn.set(StateTurn::Active);
            }
        },
        StateGame::NotInGame => {},
    };
    run_trigger.set_target("turn_handler_set_turn_next", false);
    info!("post response: turn_handler_set_turn_next");  
}

/*
game_handler: ResMut<GameHandler>,
state_map_set: Res<State<StateMapSet>>,
match state_map_set.get() {
    StateMapSet::Tutorial => {
    },
    StateMapSet::WholeCorse => {
        if game_handler.current_level_get() == 18 {

        }
    },
    StateMapSet::FrontNine => {
    },
    StateMapSet::BackNine => {
    },
    StateMapSet::SelectAHole => {
    },
};
*/