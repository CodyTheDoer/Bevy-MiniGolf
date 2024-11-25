use bevy::prelude::*;

// States
use crate::{
    RemoteStateUpdate,
};

// Resources
use crate::{
    GameHandler,
    Party,
};

impl GameHandler {
    pub fn new() -> Self {
        let current_level = 0;
        let active_ball_location = Vec3::ZERO;
        let arrow_state = false;
        let network_server_connection = false;
        let remotely_pushed_state = None;
        GameHandler {
            current_level,
            active_ball_location,
            arrow_state,
            network_server_connection,
            remotely_pushed_state,
        }
    }

    // Level Handling logic
    pub fn next_level(&mut self) {
        self.current_level += 1;
    }

    pub fn get_current_level(&self) -> i32 {
        self.current_level
    }

    pub fn set_current_level(&mut self, level: i32) {
        self.current_level = level;
    }

    pub fn get_active_ball_location(&self) -> Vec3 {
        self.active_ball_location
    }

    pub fn set_active_ball_location(&mut self, point: Vec3) {
        info!("function: set_active_ball_location: {}", point.clone()); 
        self.active_ball_location = point;
    }

    pub fn init_postgame_leaderboard(
        &mut self, 
        mut party: ResMut<Party>,
    ) {
        self.set_current_level(20);
    }

    pub fn init_tutorial(&mut self) {
        self.set_current_level(19);
    }

    pub fn init_menu_main(&mut self) {
        self.set_current_level(0);
    }

    pub fn init_menu_leader_board(&mut self) {
        self.set_current_level(20);
    }

    pub fn init_menu_local(&mut self) {
        self.set_current_level(21);
    }

    pub fn init_menu_online(&mut self) {
        self.set_current_level(22);
    }

    pub fn init_menu_preferences(&mut self) {
        self.set_current_level(23);
    }

    pub fn init_menu_player(&mut self) {
        self.set_current_level(24);
    }

    // Bonk UI Logic
    pub fn get_arrow_state(&self) -> bool {
        self.arrow_state
    }

    pub fn set_arrow_state_true(&mut self) {
        self.arrow_state = true;
    }

    pub fn set_arrow_state_false(&mut self) {
        self.arrow_state = false;
    }
    
    // Remote Auth Server Logic
    pub fn is_connected(&self) -> bool {
        self.network_server_connection
    }
    
    pub fn is_not_connected(&self) -> bool {
        if self.network_server_connection == false {
            true
        } else {
            false
        }
    }
    
    pub fn set_connected_false(&mut self) {
        self.network_server_connection = false;
    }
    
    pub fn set_connected_true(&mut self) {
        self.network_server_connection = true;
    }

    pub fn auth_server_handshake_received(
        &mut self, 
        parsed_state: Option<RemoteStateUpdate>,
    ) {
        self.remotely_pushed_state = Some(parsed_state.unwrap());
    }

    pub fn get_pushed_state(&self) -> RemoteStateUpdate {
        self.remotely_pushed_state.clone().expect("Push State get failed.")
    }
}