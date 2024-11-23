use bevy::prelude::*;

use crate::{
    Player,
};

impl Player {
    pub fn new() -> Self {
        Player {
            player_id: String::from("Player@email.com"),
            hole_completion_state: false,
            ball_material: Color::srgb(1.0, 0.0, 1.0),
            ball_location: Vec3::new(0.0, 0.0, 0.0),
            bonks_level: 0,
            bonks_game: 0,
        }
    }

    pub fn start_game(&mut self) {
        self.hole_completion_state = false;
        self.ball_location = Vec3::new(0.0, 0.0, 0.0);
        self.bonks_level = 0;
        self.bonks_game = 0;
    }

    pub fn hole_completed(&mut self) {
        self.hole_completion_state = true;
    }

    pub fn next_round_prep(&mut self) {
        self.hole_completion_state = false;
    }

    pub fn add_bonk(&mut self) {
        self.bonks_level += 1;
    }

    pub fn get_bonks_level(&self) -> u32 {
        self.bonks_level
    }

    pub fn get_bonks_game(&self) -> u32 {
        self.bonks_game
    }
}
