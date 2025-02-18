use bevy::prelude::*;

use uuid::Uuid;

use crate::{
    // LeaderBoard,
    Player,
    PlayerLocal,
    PlayerAi,
    PlayerRemote,
};

impl Player for PlayerAi {
    fn new() -> Self {
        PlayerAi {
            player_id: Uuid::now_v7(),
            player_type: String::from("PlayerAi"),
            hole_completion_state: false,
            ball_material: Color::srgb(1.0, 0.0, 1.0),
            score: [0; 18],
        }
    }

    fn start_game(&mut self) {
        self.hole_completion_state = false;
        self.score = [0; 18];
    }

    fn game_completed(&mut self) {
        self.hole_completion_state = false;
        self.score = [0; 18];
    }

    fn hole_completed(&mut self) {
        self.hole_completion_state = true;
    }

    fn next_round_prep(&mut self) {
        self.hole_completion_state = false;
    }

    fn add_bonk(&mut self, level: usize) {
        let index_adj = (level as i32 - 1) as usize;
        self.score[index_adj] += 1;
    }

    fn get_bonks(&mut self, level: usize) -> i32 {
        if level == 0 {
            return 0;
        }
        let index_adj = (level as i32 - 1) as usize;
        self.score[index_adj]
    }

    fn get_hole_completion_state(&self) -> bool {
        self.hole_completion_state
    }

    fn get_player_id(&self) -> Uuid {
        self.player_id.clone()
    }

    fn get_player_type(&self) -> String {
        self.player_type.clone()
    }

    fn get_score(&self) -> [i32; 18] {
        self.score
    }

    fn set_hole_completion_state(&mut self, hole_completion_state: bool) {
        self.hole_completion_state = hole_completion_state;
    }

    fn set_player_id(&mut self, new_id: Uuid) {
        self.player_id = new_id;
    }
}

// --------------------------------------- //

impl Player for PlayerLocal {
    fn new() -> Self {
        PlayerLocal {
            player_id: Uuid::now_v7(),
            player_type: String::from("PlayerLocal"),
            hole_completion_state: false,
            ball_material: Color::srgb(1.0, 0.0, 1.0),
            score: [0; 18],
        }
    }

    fn start_game(&mut self) {
        self.hole_completion_state = false;
        self.score = [0; 18];
    }

    fn game_completed(&mut self) {
        self.hole_completion_state = false;
        self.score = [0; 18];
    }

    fn hole_completed(&mut self) {
        self.hole_completion_state = true;
    }

    fn next_round_prep(&mut self) {
        self.hole_completion_state = false;
    }

    fn add_bonk(&mut self, level: usize) {
        let index_adj = (level as i32 - 1) as usize;
        if level == 19 { // Tutorial level
            return;
        }
        self.score[index_adj] += 1;
    }

    fn get_bonks(&mut self, level: usize) -> i32 {
        if level == 0 || level > 17 {
            return 0;
        }
        let index_adj = (level as i32 - 1) as usize;
        self.score[index_adj]
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

    fn set_player_id(&mut self, new_id: Uuid) {
        self.player_id = new_id;
    }

    fn get_player_type(&self) -> String {
        self.player_type.clone()
    }

    fn get_score(&self) -> [i32; 18] {
        self.score
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
            score: [0; 18],
        }
    }

    fn start_game(&mut self) {
        self.hole_completion_state = false;
        self.score = [0; 18];
    }

    fn game_completed(&mut self) {
        self.hole_completion_state = false;
        self.score = [0; 18];
    }

    fn hole_completed(&mut self) {
        self.hole_completion_state = true;
    }

    fn next_round_prep(&mut self) {
        self.hole_completion_state = false;
    }

    fn add_bonk(&mut self, level: usize) {
        let index_adj = (level as i32 - 1) as usize;
        self.score[index_adj] += 1;
    }

    fn get_bonks(&mut self, level: usize) -> i32 {
        if level == 0 {
            return 0;
        }
        let index_adj = (level as i32 - 1) as usize;
        self.score[index_adj]
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

    fn set_player_id(&mut self, new_id: Uuid) {
        self.player_id = new_id;
    }

    fn get_player_type(&self) -> String {
        self.player_type.clone()
    }

    fn get_score(&self) -> [i32; 18] {
        self.score
    }
}