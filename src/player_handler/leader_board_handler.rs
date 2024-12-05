use bevy::prelude::*;

use crate::{
    GameHandler, 
    GameRecord, 
    LeaderBoard, 
    Party, 
    RunTrigger,
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
        let game_id = game_handler.game_id_get();
        let (players, scores) = party.get_all_player_ids_and_scores();
        let record = GameRecord {
            game_id,
            players,
            scores,
        };
        self.past_games.push(record);
        self.reset_current_scores();
        game_handler.game_id_clear();
    }

    pub fn get_game_count(&self) -> usize {
        self.past_games.len()
    }

    pub fn get_last_game(&self) -> GameRecord {
        let index = self.past_games.len() as i32 - 1;
        self.past_games[index as usize].clone()
    }

    pub fn review_game(&self, record: GameRecord) {
        info!("Review Game Record:");
        let (game_id, players, scores) = record.unwrap();
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
    game_handler: ResMut<GameHandler>,
    party: ResMut<Party>,
) {
    leader_board.log_game(game_handler, party);
    run_trigger.set_target("leader_board_log_game", false);
}

pub fn leader_board_review_last_game(
    mut run_trigger: ResMut<RunTrigger>,
    leader_board: Res<LeaderBoard>,
) {
    if leader_board.get_game_count() > 0 {
        let record = leader_board.get_last_game();
        leader_board.review_game(record);
    }
    run_trigger.set_target("leader_board_review_last_game", false);
}