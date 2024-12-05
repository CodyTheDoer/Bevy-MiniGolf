use bevy::prelude::*;

use crate::RunTrigger;

impl RunTrigger {
    pub fn new() -> Self {
        Self{
            camera_handler_cycle_state_camera: false,
            game_handler_game_start: false,
            game_handler_game_state_change_routines: false,
            game_handler_update_players_ref_ball_locations: false,
            game_handler_update_players_reset_ref_ball_locations : false,
            game_handler_update_players_store_current_ball_locations_to_ref: false,
            leader_board_log_game: false,
            leader_board_review_last_game: false,
            level_handler_set_state_next_level: false,
            level_handler_set_state_next_map_set: false,
            network_get_client_state_game: false,
            party_handler_active_player_add_bonk: false,
            party_handler_active_player_set_ball_location: false,
            party_handler_active_player_set_hole_completion_state_true: false,
            party_handler_cycle_active_player: false,
            party_handler_new_player_ai: false,
            party_handler_new_player_local: false,
            party_handler_new_player_remote: false,
            party_handler_remove_ai: false,
            party_handler_remove_last_player: false,
            turn_handler_set_turn_next: false,
        }
    }

    pub fn get(&self, target: &str) -> bool {
        match target {
            "camera_handler_cycle_state_camera" => {
                self.camera_handler_cycle_state_camera
            },
            "game_handler_game_start" => {
                self.game_handler_game_start
            },
            "game_handler_game_state_change_routines" => {
                self.game_handler_game_state_change_routines
            },
            "game_handler_update_players_ref_ball_locations" => {
                self.game_handler_update_players_ref_ball_locations
            },
            "game_handler_update_players_reset_ref_ball_locations " => {
                self.game_handler_update_players_reset_ref_ball_locations 
            },
            "game_handler_update_players_store_current_ball_locations_to_ref " => {
                self.game_handler_update_players_store_current_ball_locations_to_ref 
            },
            "leader_board_log_game" => {
                self.leader_board_log_game
            },
            "leader_board_review_last_game" => {
                self.leader_board_review_last_game
            },
            "level_handler_set_state_next_level" => {
                self.level_handler_set_state_next_level
            },
            "level_handler_set_state_next_map_set" => {
                self.level_handler_set_state_next_map_set
            },
            "network_get_client_state_game" => {
                self.network_get_client_state_game
            },
            "party_handler_active_player_add_bonk" => {
                self.party_handler_active_player_add_bonk
            },
            "party_handler_active_player_set_ball_location" => {
                self.party_handler_active_player_set_ball_location
            },
            "party_handler_active_player_set_hole_completion_state_true" => {
                self.party_handler_active_player_set_hole_completion_state_true
            },
            "party_handler_cycle_active_player" => {
                self.party_handler_cycle_active_player
            },
            "party_handler_new_player_ai" => {
                self.party_handler_new_player_ai
            },
            "party_handler_new_player_local" => {
                self.party_handler_new_player_local
            },
            "party_handler_new_player_remote" => {
                self.party_handler_new_player_remote
            },
            "party_handler_remove_ai" => {
                self.party_handler_remove_ai
            },
            "party_handler_remove_last_player" => {
                self.party_handler_remove_last_player
            },
            "turn_handler_set_turn_next" => {
                self.turn_handler_set_turn_next
            },
            _ => {false},
        }
    }

    pub fn set_target(&mut self, target: &str, state: bool) {
        match target {
            "camera_handler_cycle_state_camera" => {
                self.camera_handler_cycle_state_camera = state;
                info!("response: camera_handler_cycle_state_camera: {}", self.get("camera_handler_cycle_state_camera"));  
            },
            "game_handler_game_start" => {
                self.game_handler_game_start = state;
                info!("response: game_handler_game_start: {}", self.get("game_handler_game_start"));
            }
            "game_handler_game_state_change_routines" => {
                self.game_handler_game_state_change_routines = state;
                info!("response: game_handler_game_state_change_routines: {}", self.get("game_handler_game_state_change_routines"));
            }
            "game_handler_update_players_ref_ball_locations" => {
                self.game_handler_update_players_ref_ball_locations = state;
                info!("response: game_handler_update_players_ref_ball_locations: {}", self.get("game_handler_update_players_ref_ball_locations"));
            }
            "game_handler_update_players_reset_ref_ball_locations " => {
                self.game_handler_update_players_reset_ref_ball_locations  = state;
                info!("response: game_handler_update_players_reset_ref_ball_locations : {}", self.get("game_handler_update_players_reset_ref_ball_locations "));
            }
            "game_handler_update_players_store_current_ball_locations_to_ref " => {
                self.game_handler_update_players_store_current_ball_locations_to_ref  = state;
                info!("response: game_handler_update_players_store_current_ball_locations_to_ref : {}", self.get("game_handler_update_players_store_current_ball_locations_to_ref "));
            }
            "leader_board_log_game" => {
                self.leader_board_log_game = state;
                info!("response: leader_board_log_game: {}", self.get("leader_board_log_game"));
            }
            "leader_board_review_last_game" => {
                self.leader_board_review_last_game = state;
                info!("response: leader_board_review_last_game: {}", self.get("leader_board_review_last_game"));
            }
            "level_handler_set_state_next_level" => {
                self.level_handler_set_state_next_level = state;
                info!("response: level_handler_set_state_next_level: {}", self.get("level_handler_set_state_next_level"));  
            },
            "level_handler_set_state_next_map_set" => {
                self.level_handler_set_state_next_map_set = state;
                info!("response: level_handler_set_state_next_map_set: {}", self.get("level_handler_set_state_next_map_set"));
            }
            "network_get_client_state_game" => {
                self.network_get_client_state_game = state;
                info!("response: network_get_client_state_game: {}", self.get("network_get_client_state_game"));  
            },
            "party_handler_active_player_add_bonk" => {
                self.party_handler_active_player_add_bonk = state;
                info!("response: party_handler_active_player_add_bonk: {}", self.get("party_handler_active_player_add_bonk"));  
            },
            "party_handler_active_player_set_ball_location" => {
                self.party_handler_active_player_set_ball_location = state;
                info!("response: party_handler_active_player_set_ball_location: {}", self.get("party_handler_active_player_set_ball_location"));  
            },
            "party_handler_active_player_set_hole_completion_state_true" => {
                self.party_handler_active_player_set_hole_completion_state_true = state;
                info!("response: party_handler_active_player_set_hole_completion_state_true: {}", self.get("party_handler_active_player_set_hole_completion_state_true"));
            }
            "party_handler_cycle_active_player" => {
                self.party_handler_cycle_active_player = state;
                info!("response: party_handler_cycle_active_player: {}", self.get("party_handler_cycle_active_player"));  
            },
            "party_handler_new_player_ai" => {
                self.party_handler_new_player_ai = state;
                info!("response: party_handler_new_player_ai: {}", self.get("party_handler_new_player_ai"));  
            },
            "party_handler_new_player_local" => {
                self.party_handler_new_player_local = state;
                info!("response: party_handler_new_player_local: {}", self.get("party_handler_new_player_local"));  
            },
            "party_handler_new_player_remote" => {
                self.party_handler_new_player_remote = state;
                info!("response: party_handler_new_player_remote: {}", self.get("party_handler_new_player_remote"));  
            },
            "party_handler_remove_ai" => {
                self.party_handler_remove_ai = state;
                info!("response: party_handler_remove_ai: {}", self.get("party_handler_remove_ai"));  
            },
            "party_handler_remove_last_player" => {
                self.party_handler_remove_last_player = state;
                info!("response: party_handler_remove_last_player: {}", self.get("party_handler_remove_last_player"));  
            },
            "turn_handler_set_turn_next" => {
                self.turn_handler_set_turn_next = state;
                info!("response: turn_handler_set_turn_next: {}", self.get("turn_handler_set_turn_next"));
            }
            _ => {
                info!("Unrecognized Input: RunTrigger: {:?}", target);
            },
        }
    }

    pub fn camera_handler_cycle_state_camera(&self) -> bool {
        self.camera_handler_cycle_state_camera
    }

    pub fn game_handler_game_start(&self) -> bool {
        self.game_handler_game_start
    }

    pub fn game_handler_game_state_change_routines(&self) -> bool {
        self.game_handler_game_state_change_routines
    }

    pub fn game_handler_update_players_ref_ball_locations(&self) -> bool {
        self.game_handler_update_players_ref_ball_locations
    }

    pub fn game_handler_update_players_reset_ref_ball_locations (&self) -> bool {
        self.game_handler_update_players_reset_ref_ball_locations 
    }

    pub fn game_handler_update_players_store_current_ball_locations_to_ref (&self) -> bool {
        self.game_handler_update_players_store_current_ball_locations_to_ref 
    }

    pub fn leader_board_log_game(&self) -> bool {
        self.leader_board_log_game
    }

    pub fn leader_board_review_last_game(&self) -> bool {
        self.leader_board_review_last_game
    }

    pub fn level_handler_set_state_next_level(&self) -> bool {
        self.level_handler_set_state_next_level
    }

    pub fn level_handler_set_state_next_map_set(&self) -> bool {
        self.level_handler_set_state_next_map_set
    }

    pub fn network_get_client_state_game(&self) -> bool {
        self.network_get_client_state_game
    }

    pub fn party_handler_active_player_add_bonk(&self) -> bool {
        self.party_handler_active_player_add_bonk
    }

    pub fn party_handler_active_player_set_ball_location(&self) -> bool {
        self.party_handler_active_player_set_ball_location
    }

    pub fn party_handler_active_player_set_hole_completion_state_true(&self) -> bool {
        self.party_handler_active_player_set_hole_completion_state_true
    }

    pub fn party_handler_cycle_active_player(&self) -> bool {
        self.party_handler_cycle_active_player
    }

    pub fn party_handler_new_player_ai(&self) -> bool {
        self.party_handler_new_player_ai
    }

    pub fn party_handler_new_player_local(&self) -> bool {
        self.party_handler_new_player_local
    }

    pub fn party_handler_new_player_remote(&self) -> bool {
        self.party_handler_new_player_remote
    }

    pub fn party_handler_remove_ai(&self) -> bool {
        self.party_handler_remove_ai
    }

    pub fn party_handler_remove_last_player(&self) -> bool {
        self.party_handler_remove_last_player
    }

    pub fn turn_handler_set_turn_next(&self) -> bool {
        self.turn_handler_set_turn_next
    }
}