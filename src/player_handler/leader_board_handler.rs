use bevy::prelude::*;

use crate::{
    user_interface::run_trigger, GameHandler, GameRecord, LeaderBoard, Party, RunTrigger
};

impl LeaderBoard {
    pub fn new() -> Self {
        let past_games: Vec<GameRecord> = Vec::new();
        LeaderBoard {
            current_scores: [0; 18], // Initialize all scores to zero.
            past_games,
        }
    }

    pub fn reset_current_scores(&mut self) {
        self.current_scores = [0; 18];
    }

    pub fn log_game(
        &mut self, 
        mut game_handler: ResMut<GameHandler>,
        mut party: ResMut<Party>,
    ) {
        let game_id = game_handler.get_game_id();
        let (players, scores) = party.get_all_player_ids_and_scores();
        let record = GameRecord {
            game_id,
            players,
            scores,
        };
        self.past_games.push(record);
        self.reset_current_scores();
        game_handler.clear_game_id();
    }

    pub fn get_game_count(&self) -> usize {
        self.past_games.len()
    }

    pub fn get_last_game(&self) -> GameRecord {
        let index_adj = (self.past_games.len() as i32 - 1) as usize;
        self.past_games[index_adj].clone()
    }

    pub fn review_last_game(&self) {
        let (game_id, players, scores) = self.get_last_game().unwrap();
        info!("Review Last Game:");
        info!("game_id: {:?}", game_id);
        for i in 0..players.len() {
            let result = format!(
                "Player: {:?}, Score: {:?}",
                players[i],
                scores[i],
            );
            info!("{:?}", result);
        }
    }
}

pub fn leader_board_log_game(
    mut run_trigger: ResMut<RunTrigger>,
    mut leader_board: ResMut<LeaderBoard>,
    mut game_handler: ResMut<GameHandler>,
    mut party: ResMut<Party>,
) {
    leader_board.log_game(game_handler, party);
    run_trigger.set_target("leader_board_log_game", false);
}

pub fn leader_board_review_last_game(
    mut run_trigger: ResMut<RunTrigger>,
    leader_board: Res<LeaderBoard>,
) {
    // leader_board.review_last_game();
    run_trigger.set_target("leader_board_review_last_game", false);
}