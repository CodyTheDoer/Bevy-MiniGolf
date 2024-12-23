use bevy::prelude::*;

use crate::RunTrigger;

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
            camera_handler_cycle_state_camera_menu_target: false,
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
            turn_handler_end_game: false,
            turn_handler_next_round_prep: false,
            turn_handler_set_turn_next: false,
            start_movement_listener_turn_handler_set_turn_next: false,
        }
    }

    pub fn get(&self, target: &str) -> bool {
        match target {
            "add_physics_query_and_update_scene" => {
                self.add_physics_query_and_update_scene
            },
            "camera_handler_cycle_state_camera" => {
                self.camera_handler_cycle_state_camera
            },
            "camera_handler_cycle_state_camera_menu_target" => {
                self.camera_handler_cycle_state_camera_menu_target
            },
            "game_handler_game_start" => {
                self.game_handler_game_start
            },
            "game_handler_game_state_exit_routines" => {
                self.game_handler_game_state_exit_routines
            },
            "game_handler_game_state_start_routines" => {
                self.game_handler_game_state_start_routines
            },
            "game_handler_start_local_back_nine" => {
                self.game_handler_start_local_back_nine
            },
            "game_handler_start_local_front_nine" => {
                self.game_handler_start_local_front_nine
            },
            "game_handler_start_local_select_a_hole" => {
                self.game_handler_start_local_select_a_hole
            },
            "game_handler_start_local_whole_corse" => {
                self.game_handler_start_local_whole_corse
            },
            "game_handler_start_tutorial" => {
                self.game_handler_start_tutorial
            },
            "golf_ball_handler_end_game" => {
                self.golf_ball_handler_end_game 
            },
            "golf_ball_handler_update_locations_post_bonk" => {
                self.golf_ball_handler_update_locations_post_bonk 
            },
            "golf_ball_handler_party_store_locations" => {
                self.golf_ball_handler_party_store_locations 
            },
            "golf_ball_handler_reset_golf_ball_locations" => {
                self.golf_ball_handler_reset_golf_ball_locations 
            },
            "golf_ball_handler_spawn_golf_balls_for_party_members" => {
                self.golf_ball_handler_spawn_golf_balls_for_party_members 
            },
            "leader_board_log_game" => {
                self.leader_board_log_game
            },
            "leader_board_review_last_game" => {
                self.leader_board_review_last_game
            },
            "level_handler_init_level_game_handler_current_level" => {
                self.level_handler_init_level_game_handler_current_level
            },
            "level_handler_next_turn_protocol" => {
                self.level_handler_next_turn_protocol
            },
            "level_handler_purge_protocol" => {
                self.level_handler_purge_protocol
            },
            "level_handler_set_state_next_level" => {
                self.level_handler_set_state_next_level
            },
            "level_handler_set_state_next_map_set" => {
                self.level_handler_set_state_next_map_set
            },
            "network_get_client_state_all" => {
                self.network_get_client_state_all
            },
            "network_get_client_state_game" => {
                self.network_get_client_state_game
            },
            "party_handler_active_player_add_bonk" => {
                self.party_handler_active_player_add_bonk
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
            "turn_handler_end_game" => {
                self.turn_handler_end_game
            },
            "turn_handler_next_round_prep" => {
                self.turn_handler_next_round_prep
            },
            "turn_handler_set_turn_next" => {
                self.turn_handler_set_turn_next
            },
            "start_movement_listener_turn_handler_set_turn_next" => {
                self.start_movement_listener_turn_handler_set_turn_next
            },
            _ => {
                warn!("Target: [{}] does not exist!!!", target); 
                false
            },
        }
    }

    pub fn set_target(&mut self, target: &str, state: bool) {
        match target {
            "add_physics_query_and_update_scene" => {
                self.add_physics_query_and_update_scene = state;
                info!("response: add_physics_query_and_update_scene: {}", self.get("add_physics_query_and_update_scene"));  
            },
            "camera_handler_cycle_state_camera" => {
                self.camera_handler_cycle_state_camera = state;
                info!("response: camera_handler_cycle_state_camera: {}", self.get("camera_handler_cycle_state_camera"));  
            },
            "camera_handler_cycle_state_camera_menu_target" => {
                self.camera_handler_cycle_state_camera_menu_target = state;
                info!("response: camera_handler_cycle_state_camera_menu_target: {}", self.get("camera_handler_cycle_state_camera_menu_target"));  
            },
            "game_handler_game_start" => {
                self.game_handler_game_start = state;
                info!("response: game_handler_game_start: {}", self.get("game_handler_game_start"));
            }
            "game_handler_game_state_exit_routines" => {
                self.game_handler_game_state_exit_routines = state;
                info!("response: game_handler_game_state_exit_routines: {}", self.get("game_handler_game_state_exit_routines"));
            }
            "game_handler_game_state_start_routines" => {
                self.game_handler_game_state_start_routines = state;
                info!("response: game_handler_game_state_start_routines: {}", self.get("game_handler_game_state_start_routines"));
            }
            "game_handler_start_local_back_nine" => {
                self.game_handler_start_local_back_nine = state;
                info!("response: game_handler_start_local_back_nine: {}", self.get("game_handler_start_local_back_nine"));
            }
            "game_handler_start_local_front_nine" => {
                self.game_handler_start_local_front_nine = state;
                info!("response: game_handler_start_local_front_nine: {}", self.get("game_handler_start_local_front_nine"));
            }
            "game_handler_start_local_select_a_hole" => {
                self.game_handler_start_local_select_a_hole = state;
                info!("response: game_handler_start_local_select_a_hole: {}", self.get("game_handler_start_local_select_a_hole"));
            }
            "game_handler_start_local_whole_corse" => {
                self.game_handler_start_local_whole_corse = state;
                info!("response: game_handler_start_local_whole_corse: {}", self.get("game_handler_start_local_whole_corse"));
            }
            "game_handler_start_tutorial" => {
                self.game_handler_start_tutorial = state;
                info!("response: game_handler_start_tutorial: {}", self.get("game_handler_start_tutorial"));
            }
            "golf_ball_handler_end_game" => {
                self.golf_ball_handler_end_game = state;
                info!("response: golf_ball_handler_end_game: {}", self.get("golf_ball_handler_end_game"));
            }
            "golf_ball_handler_update_locations_post_bonk" => {
                self.golf_ball_handler_update_locations_post_bonk = state;
                info!("response: golf_ball_handler_update_locations_post_bonk: {}", self.get("golf_ball_handler_update_locations_post_bonk"));
            }
            "golf_ball_handler_party_store_locations" => {
                self.golf_ball_handler_party_store_locations = state;
                info!("response: golf_ball_handler_party_store_locations: {}", self.get("golf_ball_handler_party_store_locations"));
            }
            "golf_ball_handler_reset_golf_ball_locations" => {
                self.golf_ball_handler_reset_golf_ball_locations = state;
                info!("response: golf_ball_handler_reset_golf_ball_locations: {}", self.get("golf_ball_handler_reset_golf_ball_locations"));
            }
            "golf_ball_handler_spawn_golf_balls_for_party_members" => {
                self.golf_ball_handler_spawn_golf_balls_for_party_members = state;
                info!("response: golf_ball_handler_spawn_golf_balls_for_party_members: {}", self.get("golf_ball_handler_spawn_golf_balls_for_party_members"));
            }
            "leader_board_log_game" => {
                self.leader_board_log_game = state;
                info!("response: leader_board_log_game: {}", self.get("leader_board_log_game"));
            }
            "leader_board_review_last_game" => {
                self.leader_board_review_last_game = state;
                info!("response: leader_board_review_last_game: {}", self.get("leader_board_review_last_game"));
            }
            "level_handler_init_level_game_handler_current_level" => {
                self.level_handler_init_level_game_handler_current_level = state;
                info!("response: level_handler_init_level_game_handler_current_level: {}", self.get("level_handler_init_level_game_handler_current_level"));
            }
            "level_handler_next_turn_protocol" => {
                self.level_handler_next_turn_protocol = state;
                info!("response: level_handler_next_turn_protocol: {}", self.get("level_handler_next_turn_protocol"));
            }
            "level_handler_purge_protocol" => {
                self.level_handler_purge_protocol = state;
                info!("response: level_handler_purge_protocol: {}", self.get("level_handler_purge_protocol"));
            }
            "level_handler_set_state_next_level" => {
                self.level_handler_set_state_next_level = state;
                info!("response: level_handler_set_state_next_level: {}", self.get("level_handler_set_state_next_level"));  
            },
            "level_handler_set_state_next_map_set" => {
                self.level_handler_set_state_next_map_set = state;
                info!("response: level_handler_set_state_next_map_set: {}", self.get("level_handler_set_state_next_map_set"));
            }
            "network_get_client_state_all" => {
                self.network_get_client_state_all = state;
                info!("response: network_get_client_state_all: {}", self.get("network_get_client_state_all"));  
            },
            "network_get_client_state_game" => {
                self.network_get_client_state_game = state;
                info!("response: network_get_client_state_game: {}", self.get("network_get_client_state_game"));  
            },
            "party_handler_active_player_add_bonk" => {
                self.party_handler_active_player_add_bonk = state;
                info!("response: party_handler_active_player_add_bonk: {}", self.get("party_handler_active_player_add_bonk"));  
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
            "turn_handler_end_game" => {
                self.turn_handler_end_game = state;
                info!("response: turn_handler_end_game: {}", self.get("turn_handler_end_game"));
            }
            "turn_handler_next_round_prep" => {
                self.turn_handler_next_round_prep = state;
                info!("response: turn_handler_next_round_prep: {}", self.get("turn_handler_next_round_prep"));
            }
            "turn_handler_set_turn_next" => {
                self.turn_handler_set_turn_next = state;
                info!("response: turn_handler_set_turn_next: {}", self.get("turn_handler_set_turn_next"));
            }
            "start_movement_listener_turn_handler_set_turn_next" => {
                self.start_movement_listener_turn_handler_set_turn_next = state;
                info!("response: start_movement_listener_turn_handler_set_turn_next: {}", self.get("start_movement_listener_turn_handler_set_turn_next"));
            }
            _ => {
                info!("Unrecognized Input: RunTrigger: {:?}", target);
            },
        }
    }

    pub fn add_physics_query_and_update_scene(&self) -> bool {
        self.add_physics_query_and_update_scene
    }

    pub fn camera_handler_cycle_state_camera(&self) -> bool {
        self.camera_handler_cycle_state_camera
    }

    pub fn camera_handler_cycle_state_camera_menu_target(&self) -> bool {
        self.camera_handler_cycle_state_camera_menu_target
    }

    pub fn game_handler_game_start(&self) -> bool {
        self.game_handler_game_start
    }

    pub fn game_handler_game_state_exit_routines(&self) -> bool {
        self.game_handler_game_state_exit_routines
    }

    pub fn game_handler_game_state_start_routines(&self) -> bool {
        self.game_handler_game_state_start_routines
    }

    pub fn game_handler_start_local_back_nine(&self) -> bool {
        self.game_handler_start_local_back_nine
    }
    
    pub fn game_handler_start_local_front_nine(&self) -> bool {
        self.game_handler_start_local_front_nine
    }
    
    pub fn game_handler_start_local_select_a_hole(&self) -> bool {
        self.game_handler_start_local_select_a_hole
    }
    
    pub fn game_handler_start_local_whole_corse(&self) -> bool {
        self.game_handler_start_local_whole_corse
    }
    
    pub fn game_handler_start_tutorial(&self) -> bool {
        self.game_handler_start_tutorial
    }

    pub fn golf_ball_handler_end_game(&self) -> bool {
        self.golf_ball_handler_end_game 
    }

    pub fn golf_ball_handler_update_locations_post_bonk(&self) -> bool {
        self.golf_ball_handler_update_locations_post_bonk 
    }

    pub fn golf_ball_handler_party_store_locations(&self) -> bool {
        self.golf_ball_handler_party_store_locations 
    }

    pub fn golf_ball_handler_reset_golf_ball_locations(&self) -> bool {
        self.golf_ball_handler_reset_golf_ball_locations 
    }

    pub fn golf_ball_handler_spawn_golf_balls_for_party_members(&self) -> bool {
        self.golf_ball_handler_spawn_golf_balls_for_party_members 
    }

    pub fn leader_board_log_game(&self) -> bool {
        self.leader_board_log_game
    }

    pub fn leader_board_review_last_game(&self) -> bool {
        self.leader_board_review_last_game
    }

    pub fn level_handler_init_level_game_handler_current_level(&self) -> bool {
        self.level_handler_init_level_game_handler_current_level
    }

    pub fn level_handler_next_turn_protocol(&self) -> bool {
        self.level_handler_next_turn_protocol
    }

    pub fn level_handler_purge_protocol(&self) -> bool {
        self.level_handler_purge_protocol
    }

    pub fn level_handler_set_state_next_level(&self) -> bool {
        self.level_handler_set_state_next_level
    }

    pub fn level_handler_set_state_next_map_set(&self) -> bool {
        self.level_handler_set_state_next_map_set
    }

    pub fn network_get_client_state_all(&self) -> bool {
        self.network_get_client_state_all
    }

    pub fn network_get_client_state_game(&self) -> bool {
        self.network_get_client_state_game
    }

    pub fn party_handler_active_player_add_bonk(&self) -> bool {
        self.party_handler_active_player_add_bonk
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

    pub fn turn_handler_end_game(&self) -> bool {
        self.turn_handler_end_game
    }

    pub fn turn_handler_next_round_prep(&self) -> bool {
        self.turn_handler_next_round_prep
    }

    pub fn turn_handler_set_turn_next(&self) -> bool {
        self.turn_handler_set_turn_next
    }

    pub fn start_movement_listener_turn_handler_set_turn_next(&self) -> bool {
        self.start_movement_listener_turn_handler_set_turn_next
    }
}