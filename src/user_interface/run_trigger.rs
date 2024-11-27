use bevy::prelude::*;

use crate::RunTrigger;

impl RunTrigger {
    pub fn new() -> Self {
        Self{
            party_handler_active_player_add_bonk: false,
            party_handler_active_player_set_ball_location: false,
            party_handler_cycle_active_player: false,
            party_handler_active_player_set_hole_completion_state_true: false,
            game_handler_cycle_state_camera: false,
            game_handler_cycle_state_map_set: false,
            game_handler_cycle_current_level: false,
            game_handler_get_active_ball_location: false,
            game_handler_reset_active_ball_location: false,
            game_handler_set_active_ball_location: false,
            game_handler_state_turn_next_player_turn: false,
            game_handler_start_game_local: false,
            game_handler_toggle_state_game: false,
            leader_board_log_game: false,
            leader_board_review_last_game: false,
        }
    }

    pub fn get(&self, target: &str) -> bool {
        match target {
            "party_handler_active_player_add_bonk" => {
                self.party_handler_active_player_add_bonk
            },
            "party_handler_active_player_set_ball_location" => {
                self.party_handler_active_player_set_ball_location
            },
            "party_handler_cycle_active_player" => {
                self.party_handler_cycle_active_player
            },
            "party_handler_active_player_set_hole_completion_state_true" => {
                self.party_handler_active_player_set_hole_completion_state_true
            },
            "game_handler_cycle_state_camera" => {
                self.game_handler_cycle_state_camera
            },
            "game_handler_cycle_state_map_set" => {
                self.game_handler_cycle_state_map_set
            },
            "game_handler_cycle_current_level" => {
                self.game_handler_cycle_current_level
            },
            "game_handler_get_active_ball_location" => {
                self.game_handler_set_active_ball_location
            },
            "game_handler_reset_active_ball_location" => {
                self.game_handler_reset_active_ball_location
            },
            "game_handler_set_active_ball_location" => {
                self.game_handler_set_active_ball_location
            },
            "game_handler_state_turn_next_player_turn" => {
                self.game_handler_state_turn_next_player_turn
            },
            "game_handler_start_game_local" => {
                self.game_handler_start_game_local
            },
            "game_handler_toggle_state_game" => {
                self.game_handler_toggle_state_game
            },
            "leader_board_log_game" => {
                self.leader_board_log_game
            },
            "leader_board_review_last_game" => {
                self.leader_board_review_last_game
            },
            _ => {false},
        }
    }

    pub fn set_target(&mut self, target: &str, state: bool) {
        match target {
            "party_handler_active_player_add_bonk" => {
                self.party_handler_active_player_add_bonk = state;
                info!("response: party_handler_active_player_add_bonk: {}", self.get("party_handler_active_player_add_bonk"));  
            },
            "party_handler_active_player_set_ball_location" => {
                self.party_handler_active_player_set_ball_location = state;
                info!("response: party_handler_active_player_set_ball_location: {}", self.get("party_handler_active_player_set_ball_location"));  
            },
            "party_handler_cycle_active_player" => {
                self.party_handler_cycle_active_player = state;
                info!("response: party_handler_cycle_active_player: {}", self.get("party_handler_cycle_active_player"));  
            },
            "party_handler_active_player_set_hole_completion_state_true" => {
                self.party_handler_active_player_set_hole_completion_state_true = state;
                info!("response: party_handler_active_player_set_hole_completion_state_true: {}", self.get("party_handler_active_player_set_hole_completion_state_true"));
            }
            "game_handler_cycle_state_camera" => {
                self.game_handler_cycle_state_camera = state;
                info!("response: game_handler_cycle_state_camera: {}", self.get("game_handler_cycle_state_camera"));  
            },
            "game_handler_cycle_state_map_set" => {
                self.game_handler_cycle_state_map_set = state;
                info!("response: game_handler_cycle_state_map_set: {}", self.get("game_handler_cycle_state_map_set"));
            }
            "game_handler_cycle_current_level" => {
                self.game_handler_cycle_current_level = state;
                info!("response: game_handler_cycle_current_level: {}", self.get("game_handler_cycle_current_level"));  
            },
            "game_handler_get_active_ball_location" => {
                self.game_handler_get_active_ball_location = state;
                info!("response: game_handler_get_active_ball_location: {}", self.get("game_handler_get_active_ball_location"));
            }
            "game_handler_reset_active_ball_location" => {
                self.game_handler_reset_active_ball_location = state;
                info!("response: game_handler_reset_active_ball_location: {}", self.get("game_handler_reset_active_ball_location"));
            }
            "game_handler_set_active_ball_location" => {
                self.game_handler_set_active_ball_location = state;
                info!("response: game_handler_set_active_ball_location: {}", self.get("game_handler_set_active_ball_location"));
            }
            "game_handler_state_turn_next_player_turn" => {
                self.game_handler_state_turn_next_player_turn = state;
                info!("response: game_handler_state_turn_next_player_turn: {}", self.get("game_handler_state_turn_next_player_turn"));
            }
            "game_handler_start_game_local" => {
                self.game_handler_start_game_local = state;
                info!("response: game_handler_start_game_local: {}", self.get("game_handler_start_game_local"));
            }
            "game_handler_toggle_state_game" => {
                self.game_handler_toggle_state_game = state;
                info!("response: game_handler_toggle_state_game: {}", self.get("game_handler_toggle_state_game"));
            }
            "leader_board_log_game" => {
                self.leader_board_log_game = state;
                info!("response: leader_board_log_game: {}", self.get("leader_board_log_game"));
            }
            "leader_board_review_last_game" => {
                self.leader_board_review_last_game = state;
                info!("response: leader_board_review_last_game: {}", self.get("leader_board_review_last_game"));
            }
            _ => {
                info!("Unrecognized Input: RunTrigger: {:?}", target);
            },
        }
    }

    pub fn party_handler_active_player_add_bonk(&self) -> bool {
        self.party_handler_active_player_add_bonk
    }

    pub fn party_handler_active_player_set_ball_location(&self) -> bool {
        self.party_handler_active_player_set_ball_location
    }

    pub fn party_handler_cycle_active_player(&self) -> bool {
        self.party_handler_cycle_active_player
    }

    pub fn party_handler_active_player_set_hole_completion_state_true(&self) -> bool {
        self.party_handler_active_player_set_hole_completion_state_true
    }

    pub fn game_handler_cycle_state_camera(&self) -> bool {
        self.game_handler_cycle_state_camera
    }

    pub fn game_handler_cycle_state_map_set(&self) -> bool {
        self.game_handler_cycle_state_map_set
    }

    pub fn game_handler_cycle_current_level(&self) -> bool {
        self.game_handler_cycle_current_level
    }

    pub fn game_handler_get_active_ball_location(&self) -> bool {
        self.game_handler_get_active_ball_location
    }

    pub fn game_handler_reset_active_ball_location(&self) -> bool {
        self.game_handler_reset_active_ball_location
    }

    pub fn game_handler_set_active_ball_location(&self) -> bool {
        self.game_handler_set_active_ball_location
    }

    pub fn game_handler_state_turn_next_player_turn(&self) -> bool {
        self.game_handler_state_turn_next_player_turn
    }

    pub fn game_handler_start_game_local(&self) -> bool {
        self.game_handler_start_game_local
    }

    pub fn game_handler_toggle_state_game(&self) -> bool {
        self.game_handler_toggle_state_game
    }

    pub fn leader_board_log_game(&self) -> bool {
        self.leader_board_log_game
    }

    pub fn leader_board_review_last_game(&self) -> bool {
        self.leader_board_review_last_game
    }
}