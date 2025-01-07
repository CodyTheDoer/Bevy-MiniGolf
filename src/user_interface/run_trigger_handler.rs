use bevy::prelude::*;

use crate::{
    CheckStateRT,
    RunTrigger,
};

/*
// Standard Trigger
pub fn _______________________________(
    mut run_trigger: ResMut<RunTrigger>,
) {
    info!("function: _______________________________"); 
    {
        
    }
    run_trigger.set_target("_______________________________", false);
    info!("post response: _______________________________: [{}]", run_trigger.get("_______________________________"));  
}

// Delayed Trigger
pub fn _______________________________(
    mut run_trigger: ResMut<RunTrigger>,
) {
    info!("function: _______________________________"); 
    {
        
        run_trigger.set_target("_______________________________", false);
        info!("post response: _______________________________: [{}]", run_trigger.get("_______________________________"));  
    }
}
*/ 

impl RunTrigger {
    pub fn new() -> Self {
        RunTrigger{
            add_physics_query_and_update_scene: false,
            camera_handler_cycle_state_camera: false,
            game_handler_game_start: false,
            game_handler_game_state_exit_routines: false,
            game_handler_game_state_start_routines: false,
            game_handler_start_local_back_nine: false,
            game_handler_start_local_front_nine: false,
            game_handler_start_local_select_a_hole: false,
            game_handler_start_local_whole_corse: false,
            game_handler_start_tutorial: false,
            golf_ball_handler_update_locations_post_bonk: false,
            golf_ball_handler_end_game: false,
            golf_ball_handler_party_store_locations: false,
            golf_ball_handler_reset_golf_ball_locations: false,
            golf_ball_handler_spawn_golf_balls_for_party_members: false,
            leader_board_log_game: false,
            leader_board_review_last_game: false,
            level_handler_init_level_game_handler_current_level: false,
            level_handler_next_turn_protocol: false,
            level_handler_purge_protocol: false,
            level_handler_set_state_next_level: false,
            level_handler_set_state_next_map_set: false,
            network_get_client_state_all: false,
            network_get_client_state_game: false,
            party_handler_active_player_add_bonk: false,
            party_handler_active_player_set_hole_completion_state_true: false,
            party_handler_cycle_active_player: false,
            party_handler_new_player_ai: false,
            party_handler_new_player_local: false,
            party_handler_new_player_remote: false,
            party_handler_remove_ai: false,
            party_handler_remove_last_player: false,
            party_handler_remove_local_player: false,
            turn_handler_end_game: false,
            turn_handler_next_round_prep: false,
            turn_handler_set_turn_next: false,
            start_movement_listener_turn_handler_set_turn_next: false,
        }
    }

    pub fn get(&self, target: CheckStateRT) -> bool {
        match target {
            CheckStateRT::AddPhysicsQueryAndUpdateScene => {
                self.add_physics_query_and_update_scene
            },
            CheckStateRT::CameraHandlerCycleStateCamera => {
                self.camera_handler_cycle_state_camera
            },
            CheckStateRT::GameHandlerGameStart => {
                self.game_handler_game_start
            },
            CheckStateRT::GameHandlerGameStateExitRoutines => {
                self.game_handler_game_state_exit_routines
            },
            CheckStateRT::GameHandlerGameStateStartRoutines => {
                self.game_handler_game_state_start_routines
            },
            CheckStateRT::GameHandlerStartLocalBackNine => {
                self.game_handler_start_local_back_nine
            },
            CheckStateRT::GameHandlerStartLocalFrontNine => {
                self.game_handler_start_local_front_nine
            },
            CheckStateRT::GameHandlerStartLocalSelectAHole => {
                self.game_handler_start_local_select_a_hole
            },
            CheckStateRT::GameHandlerStartLocalWholeCorse => {
                self.game_handler_start_local_whole_corse
            },
            CheckStateRT::GameHandlerStartTutorial => {
                self.game_handler_start_tutorial
            },
            CheckStateRT::GolfBallHandlerEndGame => {
                self.golf_ball_handler_end_game 
            },
            CheckStateRT::GolfBallHandlerPartyStoreLocations => {
                self.golf_ball_handler_party_store_locations 
            },
            CheckStateRT::GolfBallHandlerResetGolfBallLocations => {
                self.golf_ball_handler_party_store_locations 
            },
            CheckStateRT::GolfBallHandlerSpawnGolfBallsForPartyMembers => {
                self.golf_ball_handler_spawn_golf_balls_for_party_members 
            },
            CheckStateRT::GolfBallHandlerUpdateLocationsPostBonk => {
                self.golf_ball_handler_update_locations_post_bonk 
            },
            CheckStateRT::LeaderBoardLogGame => {
                self.leader_board_log_game
            },
            CheckStateRT::LeaderBoardReviewLastGame => {
                self.leader_board_review_last_game
            },
            CheckStateRT::LevelHandlerInitLevelGameHandlerCurrentLevel => {
                self.level_handler_init_level_game_handler_current_level
            },
            CheckStateRT::LevelHandlerNextTurnProtocol => {
                self.level_handler_next_turn_protocol
            },
            CheckStateRT::LevelHandlerPurgeProtocol => {
                self.level_handler_purge_protocol
            },
            CheckStateRT::LevelHandlerSetStateNextLevel => {
                self.level_handler_set_state_next_level
            },
            CheckStateRT::LevelHandlerSetStateNextMapSet => {
                self.level_handler_set_state_next_map_set
            },
            CheckStateRT::NetworkGetClientStateAll => {
                self.network_get_client_state_all
            },
            CheckStateRT::NetworkGetClientStateGame => {
                self.network_get_client_state_game
            },
            CheckStateRT::PartyHandlerActivePlayerAddBonk => {
                self.party_handler_active_player_add_bonk
            },
            CheckStateRT::PartyHandlerActivePlayerSetHoleCompletionStateTrue => {
                self.party_handler_active_player_set_hole_completion_state_true
            },
            CheckStateRT::PartyHandlerCycleActivePlayer => {
                self.party_handler_cycle_active_player
            },
            CheckStateRT::PartyHandlerNewPlayerAi => {
                self.party_handler_new_player_ai
            },
            CheckStateRT::PartyHandlerNewPlayerLocal => {
                self.party_handler_new_player_local
            },
            CheckStateRT::PartyHandlerNewPlayerRemote => {
                self.party_handler_new_player_remote
            },
            CheckStateRT::PartyHandlerRemoveAi => {
                self.party_handler_remove_ai
            },
            CheckStateRT::PartyHandlerRemoveLastPlayer => {
                self.party_handler_remove_last_player
            },
            CheckStateRT::PartyHandlerRemoveLocalPlayer => {
                self.party_handler_remove_local_player
            },
            CheckStateRT::TurnHandlerEndGame => {
                self.turn_handler_end_game
            },
            CheckStateRT::TurnHandlerNextRoundPrep => {
                self.turn_handler_next_round_prep
            },
            CheckStateRT::TurnHandlerSetTurnNext => {
                self.turn_handler_set_turn_next
            },
            CheckStateRT::StartMovementListenerTurnHandlerSetTurnNext => {
                self.start_movement_listener_turn_handler_set_turn_next
            },
        }
    }

    pub fn set_target(&mut self, target: CheckStateRT, state: bool) {
        match target {
            CheckStateRT::AddPhysicsQueryAndUpdateScene => {
                self.add_physics_query_and_update_scene = state;
                info!("response: add_physics_query_and_update_scene: {}", self.get(CheckStateRT::AddPhysicsQueryAndUpdateScene));  
            },
            CheckStateRT::CameraHandlerCycleStateCamera => {
                self.camera_handler_cycle_state_camera = state;
                info!("response: camera_handler_cycle_state_camera: {}", self.get(CheckStateRT::CameraHandlerCycleStateCamera));  
            },
            CheckStateRT::GameHandlerGameStart => {
                self.game_handler_game_start = state;
                info!("response: game_handler_game_start: {}", self.get(CheckStateRT::GameHandlerGameStart));
            }
            CheckStateRT::GameHandlerGameStateExitRoutines => {
                self.game_handler_game_state_exit_routines = state;
                info!("response: game_handler_game_state_exit_routines: {}", self.get(CheckStateRT::GameHandlerGameStateExitRoutines));
            }
            CheckStateRT::GameHandlerGameStateStartRoutines => {
                self.game_handler_game_state_start_routines = state;
                info!("response: game_handler_game_state_start_routines: {}", self.get(CheckStateRT::GameHandlerGameStateStartRoutines));
            }
            CheckStateRT::GameHandlerStartLocalBackNine => {
                self.game_handler_start_local_back_nine = state;
                info!("response: game_handler_start_local_back_nine: {}", self.get(CheckStateRT::GameHandlerStartLocalBackNine));
            }
            CheckStateRT::GameHandlerStartLocalFrontNine => {
                self.game_handler_start_local_front_nine = state;
                info!("response: game_handler_start_local_front_nine: {}", self.get(CheckStateRT::GameHandlerStartLocalFrontNine));
            }
            CheckStateRT::GameHandlerStartLocalSelectAHole => {
                self.game_handler_start_local_select_a_hole = state;
                info!("response: game_handler_start_local_select_a_hole: {}", self.get(CheckStateRT::GameHandlerStartLocalSelectAHole));
            }
            CheckStateRT::GameHandlerStartLocalWholeCorse => {
                self.game_handler_start_local_whole_corse = state;
                info!("response: game_handler_start_local_whole_corse: {}", self.get(CheckStateRT::GameHandlerStartLocalWholeCorse));
            }
            CheckStateRT::GameHandlerStartTutorial => {
                self.game_handler_start_tutorial = state;
                info!("response: game_handler_start_tutorial: {}", self.get(CheckStateRT::GameHandlerStartTutorial));
            }
            CheckStateRT::GolfBallHandlerEndGame => {
                self.golf_ball_handler_end_game = state;
                info!("response: golf_ball_handler_end_game: {}", self.get(CheckStateRT::GolfBallHandlerEndGame));
            }
            CheckStateRT::GolfBallHandlerPartyStoreLocations => {
                self.golf_ball_handler_party_store_locations = state;
                info!("response: golf_ball_handler_party_store_locations: {}", self.get(CheckStateRT::GolfBallHandlerPartyStoreLocations));
            }
            CheckStateRT::GolfBallHandlerResetGolfBallLocations => {
                self.golf_ball_handler_reset_golf_ball_locations = state;
                info!("response: golf_ball_handler_reset_golf_ball_locations: {}", self.get(CheckStateRT::GolfBallHandlerResetGolfBallLocations));
            }
            CheckStateRT::GolfBallHandlerSpawnGolfBallsForPartyMembers => {
                self.golf_ball_handler_spawn_golf_balls_for_party_members = state;
                info!("response: golf_ball_handler_spawn_golf_balls_for_party_members: {}", self.get(CheckStateRT::GolfBallHandlerSpawnGolfBallsForPartyMembers));
            }
            CheckStateRT::GolfBallHandlerUpdateLocationsPostBonk => {
                self.golf_ball_handler_update_locations_post_bonk = state;
                info!("response: golf_ball_handler_update_locations_post_bonk: {}", self.get(CheckStateRT::GolfBallHandlerUpdateLocationsPostBonk));
            }
            CheckStateRT::LeaderBoardLogGame => {
                self.leader_board_log_game = state;
                info!("response: leader_board_log_game: {}", self.get(CheckStateRT::LeaderBoardLogGame));
            }
            CheckStateRT::LeaderBoardReviewLastGame => {
                self.leader_board_review_last_game = state;
                info!("response: leader_board_review_last_game: {}", self.get(CheckStateRT::LeaderBoardReviewLastGame));
            }
            CheckStateRT::LevelHandlerInitLevelGameHandlerCurrentLevel => {
                self.level_handler_init_level_game_handler_current_level = state;
                info!("response: level_handler_init_level_game_handler_current_level: {}", self.get(CheckStateRT::LevelHandlerInitLevelGameHandlerCurrentLevel));
            }
            CheckStateRT::LevelHandlerNextTurnProtocol => {
                self.level_handler_next_turn_protocol = state;
                info!("response: level_handler_next_turn_protocol: {}", self.get(CheckStateRT::LevelHandlerNextTurnProtocol));
            }
            CheckStateRT::LevelHandlerPurgeProtocol => {
                self.level_handler_purge_protocol = state;
                info!("response: level_handler_purge_protocol: {}", self.get(CheckStateRT::LevelHandlerPurgeProtocol));
            }
            CheckStateRT::LevelHandlerSetStateNextLevel => {
                self.level_handler_set_state_next_level = state;
                info!("response: level_handler_set_state_next_level: {}", self.get(CheckStateRT::LevelHandlerSetStateNextLevel));  
            },
            CheckStateRT::LevelHandlerSetStateNextMapSet => {
                self.level_handler_set_state_next_map_set = state;
                info!("response: level_handler_set_state_next_map_set: {}", self.get(CheckStateRT::LevelHandlerSetStateNextMapSet));
            }
            CheckStateRT::NetworkGetClientStateAll => {
                self.network_get_client_state_all = state;
                info!("response: network_get_client_state_all: {}", self.get(CheckStateRT::NetworkGetClientStateAll));  
            },
            CheckStateRT::NetworkGetClientStateGame => {
                self.network_get_client_state_game = state;
                info!("response: network_get_client_state_game: {}", self.get(CheckStateRT::NetworkGetClientStateGame));  
            },
            CheckStateRT::PartyHandlerActivePlayerAddBonk => {
                self.party_handler_active_player_add_bonk = state;
                info!("response: party_handler_active_player_add_bonk: {}", self.get(CheckStateRT::PartyHandlerActivePlayerAddBonk));  
            },
            CheckStateRT::PartyHandlerActivePlayerSetHoleCompletionStateTrue => {
                self.party_handler_active_player_set_hole_completion_state_true = state;
                info!("response: party_handler_active_player_set_hole_completion_state_true: {}", self.get(CheckStateRT::PartyHandlerActivePlayerSetHoleCompletionStateTrue));
            }
            CheckStateRT::PartyHandlerCycleActivePlayer => {
                self.party_handler_cycle_active_player = state;
                info!("response: party_handler_cycle_active_player: {}", self.get(CheckStateRT::PartyHandlerCycleActivePlayer));  
            },
            CheckStateRT::PartyHandlerNewPlayerAi => {
                self.party_handler_new_player_ai = state;
                info!("response: party_handler_new_player_ai: {}", self.get(CheckStateRT::PartyHandlerNewPlayerAi));  
            },
            CheckStateRT::PartyHandlerNewPlayerLocal => {
                self.party_handler_new_player_local = state;
                info!("response: party_handler_new_player_local: {}", self.get(CheckStateRT::PartyHandlerNewPlayerLocal));  
            },
            CheckStateRT::PartyHandlerNewPlayerRemote => {
                self.party_handler_new_player_remote = state;
                info!("response: party_handler_new_player_remote: {}", self.get(CheckStateRT::PartyHandlerNewPlayerRemote));  
            },
            CheckStateRT::PartyHandlerRemoveAi => {
                self.party_handler_remove_ai = state;
                info!("response: party_handler_remove_ai: {}", self.get(CheckStateRT::PartyHandlerRemoveAi));  
            },
            CheckStateRT::PartyHandlerRemoveLastPlayer => {
                self.party_handler_remove_last_player = state;
                info!("response: party_handler_remove_last_player: {}", self.get(CheckStateRT::PartyHandlerRemoveLastPlayer));  
            },
            CheckStateRT::PartyHandlerRemoveLocalPlayer => {
                self.party_handler_remove_local_player = state;
                info!("response: party_handler_remove_local_player: {}", self.get(CheckStateRT::PartyHandlerRemoveLocalPlayer));  
            },
            CheckStateRT::TurnHandlerEndGame => {
                self.turn_handler_end_game = state;
                info!("response: turn_handler_end_game: {}", self.get(CheckStateRT::TurnHandlerEndGame));
            }
            CheckStateRT::TurnHandlerNextRoundPrep => {
                self.turn_handler_next_round_prep = state;
                info!("response: turn_handler_next_round_prep: {}", self.get(CheckStateRT::TurnHandlerNextRoundPrep));
            }
            CheckStateRT::TurnHandlerSetTurnNext => {
                self.turn_handler_set_turn_next = state;
                info!("response: turn_handler_set_turn_next: {}", self.get(CheckStateRT::TurnHandlerSetTurnNext));
            }
            CheckStateRT::StartMovementListenerTurnHandlerSetTurnNext => {
                self.start_movement_listener_turn_handler_set_turn_next = state;
                info!("response: start_movement_listener_turn_handler_set_turn_next: {}", self.get(CheckStateRT::StartMovementListenerTurnHandlerSetTurnNext));
            }
        }
    }
}