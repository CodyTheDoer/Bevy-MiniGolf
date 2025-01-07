use bevy::{prelude::*,
    utils::Duration, 
};

// States
use crate::{ 
    StateGame, 
    StateLevel, 
    StateMapSet, 
    StateTurn,
};

// Resources
use crate::{
    CheckStateGH,
    CheckStateRT, 
    GameHandler,
    Party, 
    RunTrigger,
    SpawnPhysicsCheckTimer,
};

pub fn turn_handler_set_turn_next(
    mut commands: Commands,
    mut run_trigger: ResMut<RunTrigger>,
    mut game_handler: ResMut<GameHandler>,
    state_game: Res<State<StateGame>>,
    state_level: Res<State<StateLevel>>,
    state_map_set: Res<State<StateMapSet>>,
    mut next_state_turn: ResMut<NextState<StateTurn>>,
    party: ResMut<Party>,
) {
    info!("function: turn_handler_set_turn_next"); 
    let mut physics_timer_check = false;
    {
        match state_game.get() {
            StateGame::InGame => {
                if !game_handler.get(CheckStateGH::GolfBallsReset) {    
                    next_state_turn.set(StateTurn::NextTurn);
                    if game_handler.get(CheckStateGH::RemoteGame) {
        
                    } else {
                        let mut load_next_level = false;
                        run_trigger.set_target(CheckStateRT::GolfBallHandlerPartyStoreLocations, true);
        
                        let owned_finished_count = party.all_players_get_finished_count();
                        let owned_party_size = party.party_size();
                        info!("\nFinished: [{:?}] vs Party: [{:?}]", owned_finished_count, owned_party_size);
                        
                        // if all players have completed the round:
                        if owned_finished_count == owned_party_size as i32 {
                            info!("Round Finished: All Players finished!");
                            match state_map_set.get() {
                                StateMapSet::ToBeSelected => {warn!("Impossible non-selection of map state, crashing..."); panic!()},
                                StateMapSet::Tutorial => {
                                    run_trigger.set_target(CheckStateRT::TurnHandlerEndGame, true);
                                    run_trigger.set_target(CheckStateRT::GolfBallHandlerEndGame, true);
                                    run_trigger.set_target(CheckStateRT::LevelHandlerInitLevelGameHandlerCurrentLevel, true);
                                },
                                StateMapSet::WholeCorse => {
                                    match state_level.get() {
                                        StateLevel::Hole18 => {
                                            run_trigger.set_target(CheckStateRT::GolfBallHandlerEndGame, true);
                                            run_trigger.set_target(CheckStateRT::TurnHandlerEndGame, true);
                                        },
                                        _ => { // Cycling through map set levels
                                            load_next_level = true;
                                        },
                                    }
                                },
                                StateMapSet::FrontNine => {
                                    match state_level.get() {
                                        StateLevel::Hole9 => {
                                            run_trigger.set_target(CheckStateRT::TurnHandlerEndGame, true);
                                            run_trigger.set_target(CheckStateRT::GolfBallHandlerEndGame, true);
                                        },
                                        _ => { // Cycling through map set levels
                                            load_next_level = true;
                                        },
                                    }
                                },
                                StateMapSet::BackNine => {
                                    match state_level.get() {
                                        StateLevel::Hole18 => {
                                            run_trigger.set_target(CheckStateRT::TurnHandlerEndGame, true);
                                            run_trigger.set_target(CheckStateRT::GolfBallHandlerEndGame, true);
                                        },
                                        _ => { // Cycling through map set levels
                                            load_next_level = true;
                                        },
                                    }
                                },
                                StateMapSet::SelectAHole => {
                                    run_trigger.set_target(CheckStateRT::TurnHandlerEndGame, true);
                                    run_trigger.set_target(CheckStateRT::GolfBallHandlerEndGame, true);
                                    run_trigger.set_target(CheckStateRT::LevelHandlerInitLevelGameHandlerCurrentLevel, true);
                                },
                            };
                        } else { // Non Round switching turn logic below
                            run_trigger.set_target(CheckStateRT::PartyHandlerCycleActivePlayer, true);
                            next_state_turn.set(StateTurn::Active);
                        }   
                        
                        if load_next_level == true {
                            run_trigger.set_target(CheckStateRT::GolfBallHandlerResetGolfBallLocations, true);
                            run_trigger.set_target(CheckStateRT::TurnHandlerNextRoundPrep, true);
                            run_trigger.set_target(CheckStateRT::LevelHandlerNextTurnProtocol, true);
                            next_state_turn.set(StateTurn::Active);
                            physics_timer_check = true;
                            game_handler.current_level_set_next_level();
                            run_trigger.set_target(CheckStateRT::LevelHandlerInitLevelGameHandlerCurrentLevel, true);
                        }         
                    }
                }
            },
            StateGame::NotInGame => {},
        };
    }
    if physics_timer_check == true {
        commands.spawn((
            SpawnPhysicsCheckTimer {
                timer: Timer::new(Duration::from_millis(2500), TimerMode::Once), 
            },
        ));
    };
    run_trigger.set_target(CheckStateRT::TurnHandlerSetTurnNext, false);
    info!("post response: turn_handler_set_turn_next: [{}]", run_trigger.get(CheckStateRT::TurnHandlerSetTurnNext));  
}

pub fn turn_handler_end_game(
    mut run_trigger: ResMut<RunTrigger>,
) {
    info!("function: turn_handler_end_game"); 
    {
        run_trigger.set_target(CheckStateRT::LeaderBoardLogGame, true);
        run_trigger.set_target(CheckStateRT::GameHandlerGameStateExitRoutines, true);
    }
    run_trigger.set_target(CheckStateRT::TurnHandlerEndGame, false);
    info!("post response: turn_handler_end_game: [{}]", run_trigger.get(CheckStateRT::TurnHandlerEndGame));  
}

pub fn turn_handler_next_round_prep(
    mut run_trigger: ResMut<RunTrigger>,
    mut game_handler: ResMut<GameHandler>,
    mut party: ResMut<Party>,
) {
    info!("function: turn_handler_next_round_prep"); 
    {
        game_handler.set_target(CheckStateGH::RoundStart, true);
        party.next_round_prep();
        party.active_player_set(1);
    }
    run_trigger.set_target(CheckStateRT::TurnHandlerNextRoundPrep, false);
    info!("post response: turn_handler_next_round_prep: [{}]", run_trigger.get(CheckStateRT::TurnHandlerNextRoundPrep));  
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

