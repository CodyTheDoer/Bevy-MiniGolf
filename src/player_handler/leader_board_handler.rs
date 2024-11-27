// use bevy::prelude::*;

use crate::{
    LeaderBoard,
    LeaderBoardHandler,
};

use uuid::Uuid;

impl LeaderBoard {
    pub fn new(player_id: Uuid) -> Self {
        LeaderBoard {
            player_id,
            scores: [0; 18], // Initialize all scores to zero.
        }
    }
}

//     pub fn set_score(&mut self, hole: usize, score: u32) {
//         if hole >= 1 && hole <= 18 {
//             self.scores[hole - 1] = score;
//         } else {
//             panic!("Hole number must be between 1 and 18");
//         }
//     }

//     pub fn get_score(&self, hole: usize) -> u32 {
//         if hole >= 1 && hole <= 18 {
//             self.scores[hole - 1]
//         } else {
//             panic!("Hole number must be between 1 and 18");
//         }
//     }

//     pub fn get_total_score(&self) -> u32 {
//         self.scores.iter().sum()
//     }
// }

impl LeaderBoardHandler {
    pub fn new() -> Self {
        LeaderBoardHandler {
            leaderboards: Vec::new(),
        }
    }
}

//     pub fn add_player(&mut self, player_id: usize) {
//         self.leaderboards.push(LeaderBoard::new(player_id));
//     }

//     pub fn get_leaderboard(&self, player_id: usize) -> Option<&LeaderBoard> {
//         self.leaderboards.iter().find(|lb| lb.player_id == player_id)
//     }

//     pub fn get_leaderboard_mut(&mut self, player_id: usize) -> Option<&mut LeaderBoard> {
//         self.leaderboards.iter_mut().find(|lb| lb.player_id == player_id)
//     }

//     pub fn get_all_scores(&self) -> Vec<(usize, u32)> {
//         self.leaderboards
//             .iter()
//             .map(|lb| (lb.player_id, lb.get_total_score()))
//             .collect()
//     }
// }