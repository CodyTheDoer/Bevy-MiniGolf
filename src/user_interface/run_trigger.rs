use bevy::prelude::*;

use crate::{
    RunTrigger,
};

impl RunTrigger {
    pub fn new() -> Self {
        Self{
            active_player_add_bonk: false,
            active_player_set_ball_location: false,
            cycle_active_player: false,
            cycle_camera: false,
            cycle_state_map_set: false,
            game_handler_get_active_ball_location: false,
            game_handler_set_active_ball_location: false,
            set_hole_completion_state_true: false,
            state_turn_next_player_turn: false,
            toggle_state_game: false,
        }
    }

    pub fn get(&self, target: &str) -> bool {
        match target {
            "active_player_add_bonk" => {
                self.active_player_add_bonk
            },
            "active_player_set_ball_location" => {
                self.active_player_set_ball_location
            },
            "cycle_active_player" => {
                self.cycle_active_player
            },
            "cycle_camera" => {
                self.cycle_camera
            },
            "cycle_state_map_set" => {
                self.cycle_state_map_set
            },
            "set_hole_completion_state_true" => {
                self.set_hole_completion_state_true
            },
            "state_turn_next_player_turn" => {
                self.state_turn_next_player_turn
            },
            "toggle_state_game" => {
                self.toggle_state_game
            },
            "game_handler_set_active_ball_location" => {
                self.game_handler_set_active_ball_location
            },
            "game_handler_get_active_ball_location" => {
                self.game_handler_set_active_ball_location
            },
            _ => {false},
        }
    }

    pub fn set_target(&mut self, target: &str, state: bool) {
        match target {
            "active_player_add_bonk" => {
                self.active_player_add_bonk = state;
                info!("response: active_player_add_bonk: {}", self.get("active_player_add_bonk"));  
            },
            "active_player_set_ball_location" => {
                self.active_player_set_ball_location = state;
                info!("response: active_player_set_ball_location: {}", self.get("active_player_set_ball_location"));  
            },
            "cycle_active_player" => {
                self.cycle_active_player = state;
                info!("response: cycle_active_player: {}", self.get("cycle_active_player"));  
            },
            "cycle_camera" => {
                self.cycle_camera = state;
                info!("response: cycle_camera: {}", self.get("cycle_camera"));  
            },
            "cycle_state_map_set" => {
                self.cycle_state_map_set = state;
                info!("response: cycle_state_map_set: {}", self.get("cycle_state_map_set"));
            }
            "set_hole_completion_state_true" => {
                self.set_hole_completion_state_true = state;
                info!("response: set_hole_completion_state_true: {}", self.get("set_hole_completion_state_true"));
            }
            "state_turn_next_player_turn" => {
                self.state_turn_next_player_turn = state;
                info!("response: state_turn_next_player_turn: {}", self.get("state_turn_next_player_turn"));
            }
            "toggle_state_game" => {
                self.toggle_state_game = state;
                info!("response: toggle_state_game: {}", self.get("toggle_state_game"));
            }
            "game_handler_get_active_ball_location" => {
                self.game_handler_get_active_ball_location = state;
                info!("response: game_handler_get_active_ball_location: {}", self.get("game_handler_get_active_ball_location"));
            }
            "game_handler_set_active_ball_location" => {
                self.game_handler_set_active_ball_location = state;
                info!("response: game_handler_set_active_ball_location: {}", self.get("game_handler_set_active_ball_location"));
            }
            _ => {},
        }
    }

    pub fn active_player_add_bonk(&self) -> bool {
        self.active_player_add_bonk
    }

    pub fn active_player_set_ball_location(&self) -> bool {
        self.active_player_set_ball_location
    }

    pub fn cycle_active_player(&self) -> bool {
        self.cycle_active_player
    }

    pub fn cycle_camera(&self) -> bool {
        self.cycle_camera
    }

    pub fn cycle_state_map_set(&self) -> bool {
        self.cycle_state_map_set
    }

    pub fn game_handler_get_active_ball_location(&self) -> bool {
        self.game_handler_get_active_ball_location
    }

    pub fn game_handler_set_active_ball_location(&self) -> bool {
        self.game_handler_set_active_ball_location
    }

    pub fn set_hole_completion_state_true(&self) -> bool {
        self.set_hole_completion_state_true
    }

    pub fn state_turn_next_player_turn(&self) -> bool {
        self.state_turn_next_player_turn
    }

    pub fn toggle_state_game(&self) -> bool {
        self.toggle_state_game
    }
}