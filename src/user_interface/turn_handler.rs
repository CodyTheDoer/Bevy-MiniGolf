use bevy::prelude::*;

// States
use crate::{ 
    Party, StateGame, StateLevel, StateMapSet, StateTurn
};

use std::thread;
use std::time::Duration;

// Resources
use crate::{
    GameHandler,
    RunTrigger,
};

pub fn turn_handler_set_turn_next(
    mut run_trigger: ResMut<RunTrigger>,
    mut game_handler: ResMut<GameHandler>,
    state_game: Res<State<StateGame>>,
    state_level: Res<State<StateLevel>>,
    state_map_set: Res<State<StateMapSet>>,
    mut next_state_turn: ResMut<NextState<StateTurn>>,
    party: ResMut<Party>,
) {
    info!("function: turn_handler_set_turn_next"); 
    {
        match state_game.get() {
            StateGame::InGame => {
                next_state_turn.set(StateTurn::NextTurn);
                if game_handler.get("remote_game") {
    
                } else {
                    let mut load_next_level = false;
                    run_trigger.set_target("golf_ball_handler_party_store_locations", true);
    
                    let owned_finished_count = party.all_players_get_finished_count();
                    let owned_party_size = party.party_size();
                    info!("\nFinished: [{:?}] vs Party: [{:?}]", owned_finished_count, owned_party_size);
                    if owned_finished_count == owned_party_size as i32 {
                        info!("Round Finished: All Players finished!");
                        match state_map_set.get() {
                            StateMapSet::Tutorial => {
                                run_trigger.set_target("turn_handler_end_game", true);
                                run_trigger.set_target("golf_ball_handler_end_game", true);
                                run_trigger.set_target("level_handler_init_level_game_handler_current_level", true);
                            },
                            StateMapSet::WholeCorse => {
                                match state_level.get() {
                                    StateLevel::Hole18 => {
                                        run_trigger.set_target("golf_ball_handler_end_game", true);
                                        run_trigger.set_target("turn_handler_end_game", true);
                                    },
                                    _ => {
                                        run_trigger.set_target("golf_ball_handler_reset_golf_ball_locations", true);
                                        run_trigger.set_target("turn_handler_next_round_prep", true);
                                        run_trigger.set_target("level_handler_next_turn_protocol", true);
                                        next_state_turn.set(StateTurn::Active);
                                        load_next_level = true;
                                    },
                                }
                            },
                            StateMapSet::FrontNine => {
                                match state_level.get() {
                                    StateLevel::Hole9 => {
                                        run_trigger.set_target("turn_handler_end_game", true);
                                        run_trigger.set_target("golf_ball_handler_end_game", true);
                                    },
                                    _ => {
                                        run_trigger.set_target("golf_ball_handler_reset_golf_ball_locations", true);
                                        run_trigger.set_target("turn_handler_next_round_prep", true);
                                        run_trigger.set_target("level_handler_next_turn_protocol", true);
                                        next_state_turn.set(StateTurn::Active);
                                        load_next_level = true;
                                    },
                                }
                            },
                            StateMapSet::BackNine => {
                                match state_level.get() {
                                    StateLevel::Hole18 => {
                                        run_trigger.set_target("turn_handler_end_game", true);
                                        run_trigger.set_target("golf_ball_handler_end_game", true);
                                    },
                                    _ => {
                                        run_trigger.set_target("golf_ball_handler_reset_golf_ball_locations", true);
                                        run_trigger.set_target("turn_handler_next_round_prep", true);
                                        run_trigger.set_target("level_handler_next_turn_protocol", true);
                                        next_state_turn.set(StateTurn::Active);
                                        load_next_level = true;
                                    },
                                }
                            },
                            StateMapSet::SelectAHole => {
                                run_trigger.set_target("turn_handler_end_game", true);
                                run_trigger.set_target("golf_ball_handler_end_game", true);
                                run_trigger.set_target("level_handler_init_level_game_handler_current_level", true);
                            },
                        };
                    } else { // Non Round switching turn logic below
                        // run_trigger.set_target("golf_ball_handler_party_store_locations", true);
                        run_trigger.set_target("party_handler_cycle_active_player", true);
                        next_state_turn.set(StateTurn::Active);
                    }   
                    if load_next_level == true {
                        game_handler.current_level_set_next_level();
                        run_trigger.set_target("level_handler_init_level_game_handler_current_level", true);
                    }         
                }
            },
            StateGame::NotInGame => {},
        };
    }
    run_trigger.set_target("turn_handler_set_turn_next", false);
    info!("post response: turn_handler_set_turn_next: [{}]", run_trigger.get("turn_handler_set_turn_next"));  
}

pub fn turn_handler_end_game(
    mut run_trigger: ResMut<RunTrigger>,
) {
    info!("function: turn_handler_end_game"); 
    {
        run_trigger.set_target("leader_board_log_game", true);
        run_trigger.set_target("game_handler_game_state_exit_routines", true);
    }
    run_trigger.set_target("turn_handler_end_game", false);
    info!("post response: turn_handler_end_game: [{}]", run_trigger.get("turn_handler_end_game"));  
}

pub fn turn_handler_next_round_prep(
    mut run_trigger: ResMut<RunTrigger>,
    mut party: ResMut<Party>,
) {
    info!("function: turn_handler_next_round_prep"); 
    {
        party.next_round_prep();
        party.active_player_set(1);
    }
    run_trigger.set_target("turn_handler_next_round_prep", false);
    info!("post response: turn_handler_next_round_prep: [{}]", run_trigger.get("turn_handler_next_round_prep"));  
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

