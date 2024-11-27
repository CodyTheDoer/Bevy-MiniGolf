use bevy::prelude::*;

use uuid::Uuid;

use crate::{
    LeaderBoard,
    Player,
    PlayerLocal,
    PlayerAi,
    PlayerRemote,
};

impl Player for PlayerLocal {
    fn new() -> Self {
        let player_id = Uuid::now_v7();
        LeaderBoard::new(player_id.clone());
        PlayerLocal {
            player_id: player_id,
            player_type: String::from("PlayerLocal"),
            hole_completion_state: false,
            ball_material: Color::srgb(1.0, 0.0, 1.0),
            ball_location: Vec3::new(0.0, 0.0, 0.0),
            bonks_level: 0,
        }
    }

    fn start_game(&mut self) {
        self.hole_completion_state = false;
        self.ball_location = Vec3::new(0.0, 0.0, 0.0);
        self.bonks_level = 0;
    }

    fn game_completed(&mut self) {
        self.hole_completion_state = false;
        self.ball_location = Vec3::new(0.0, 0.0, 0.0);
        self.bonks_level = 0;
    }

    fn hole_completed(&mut self) {
        self.hole_completion_state = true;
    }

    fn next_round_prep(&mut self) {
        self.hole_completion_state = false;
        self.bonks_level = 0;
        self.ball_location = Vec3::new(0.0, 0.0, 0.0);
    }

    fn add_bonk(&mut self) {
        self.bonks_level += 1;
    }

    fn get_hole_completion_state(&self) -> bool {
        self.hole_completion_state
    }

    fn set_hole_completion_state(&mut self, hole_completion_state: bool) {
        self.hole_completion_state = hole_completion_state;
    }

    fn get_player_id(&self) -> Uuid {
        self.player_id.clone()
    }

    fn get_player_type(&self) -> String {
        self.player_type.clone()
    }

    fn get_ball_location(&self) -> Vec3 {
        self.ball_location
    }

    fn set_ball_location(&mut self, location: Vec3) {
        self.ball_location = location;
    }

    fn get_bonks_level(&self) -> u32 {
        self.bonks_level
    }
}

// --------------------------------------- //

impl Player for PlayerAi {
    fn new() -> Self {
        PlayerAi {
            player_id: Uuid::now_v7(),
            player_type: String::from("PlayerAi"),
            hole_completion_state: false,
            ball_material: Color::srgb(1.0, 0.0, 1.0),
            ball_location: Vec3::new(0.0, 0.0, 0.0),
            bonks_level: 0,
        }
    }

    fn start_game(&mut self) {
        self.hole_completion_state = false;
        self.ball_location = Vec3::new(0.0, 0.0, 0.0);
        self.bonks_level = 0;
    }

    fn game_completed(&mut self) {
        self.hole_completion_state = false;
        self.ball_location = Vec3::new(0.0, 0.0, 0.0);
        self.bonks_level = 0;
    }

    fn hole_completed(&mut self) {
        self.hole_completion_state = true;
    }

    fn next_round_prep(&mut self) {
        self.hole_completion_state = false;
        self.bonks_level = 0;
        self.ball_location = Vec3::new(0.0, 0.0, 0.0);
    }

    fn add_bonk(&mut self) {
        self.bonks_level += 1;
    }

    fn get_hole_completion_state(&self) -> bool {
        self.hole_completion_state
    }

    fn set_hole_completion_state(&mut self, hole_completion_state: bool) {
        self.hole_completion_state = hole_completion_state;
    }

    fn get_player_id(&self) -> Uuid {
        self.player_id.clone()
    }

    fn get_player_type(&self) -> String {
        self.player_type.clone()
    }

    fn get_ball_location(&self) -> Vec3 {
        self.ball_location
    }

    fn set_ball_location(&mut self, location: Vec3) {
        self.ball_location = location;
    }

    fn get_bonks_level(&self) -> u32 {
        self.bonks_level
    }
}

// --------------------------------------- //

impl Player for PlayerRemote {
    fn new() -> Self {
        PlayerRemote {
            player_id: Uuid::now_v7(),
            player_type: String::from("PlayerRemote"),
            hole_completion_state: false,
            ball_material: Color::srgb(1.0, 0.0, 1.0),
            ball_location: Vec3::new(0.0, 0.0, 0.0),
            bonks_level: 0,
        }
    }

    fn start_game(&mut self) {
        self.hole_completion_state = false;
        self.ball_location = Vec3::new(0.0, 0.0, 0.0);
        self.bonks_level = 0;
    }

    fn game_completed(&mut self) {
        self.hole_completion_state = false;
        self.ball_location = Vec3::new(0.0, 0.0, 0.0);
        self.bonks_level = 0;
    }

    fn hole_completed(&mut self) {
        self.hole_completion_state = true;
    }

    fn next_round_prep(&mut self) {
        self.hole_completion_state = false;
        self.bonks_level = 0;
        self.ball_location = Vec3::new(0.0, 0.0, 0.0);
    }

    fn add_bonk(&mut self) {
        self.bonks_level += 1;
    }

    fn get_hole_completion_state(&self) -> bool {
        self.hole_completion_state
    }

    fn set_hole_completion_state(&mut self, hole_completion_state: bool) {
        self.hole_completion_state = hole_completion_state;
    }

    fn get_player_id(&self) -> Uuid {
        self.player_id.clone()
    }

    fn get_player_type(&self) -> String {
        self.player_type.clone()
    }

    fn get_ball_location(&self) -> Vec3 {
        self.ball_location
    }

    fn set_ball_location(&mut self, location: Vec3) {
        self.ball_location = location;
    }

    fn get_bonks_level(&self) -> u32 {
        self.bonks_level
    }
}