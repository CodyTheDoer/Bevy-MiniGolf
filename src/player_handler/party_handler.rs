use bevy::prelude::*;

use std::sync::Arc;
use std::sync::Mutex;

use uuid::Uuid;

// States
use crate::StateGame;

// Resources
use crate::{
    CheckStateRT,
    GameHandler,
    Party,
    Player,
    PlayerAi,
    PlayerLocal,
    PlayerRemote,
    RunTrigger,
};

impl Party {
    pub fn new() -> Self {
        let players: Arc<Mutex<Vec<Arc<Mutex<dyn Player + Send>>>>> = Arc::new(Mutex::new(vec![Arc::new(Mutex::new(PlayerLocal::new()))]));
        let active_player: Arc<Mutex<i32>> = Arc::new(Mutex::new(1));
        let ai_vec: Option<Vec<usize>> = None;
        Party {
            players,
            active_player,
            ai_vec,
        } 
    }

    pub fn active_player_add_bonk(&self, level: usize) {
        let active_player_index = *self.active_player.lock().unwrap(); // Get the active player index
        let players_lock = self.players.lock().unwrap(); // First, lock the players mutex to get access to the Vec
        let player_arc = &players_lock[active_player_index as usize - 1]; // adjusted for 1 indexing // Get the active player (Arc<Mutex<Player>>)
        let mut player = player_arc.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
        player.add_bonk(level);
        info!("post function: active_player_add_bonk"); 
    }

    pub fn active_player_finished_hole(&mut self) {
        let active_player_index = *self.active_player.lock().unwrap(); // Get the active player index
        let players_lock = self.players.lock().unwrap(); // First, lock the players mutex to get access to the Vec
        let player_arc = &players_lock[active_player_index as usize - 1]; // adjusted for 1 indexing // Get the active player (Arc<Mutex<Player>>)
        let mut player = player_arc.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
        player.hole_completed();
        info!("post function: active_player_finished_hole"); 
    }

    pub fn active_player_get_bonks_level(&self, level: usize) -> i32 {
        let active_player_index = *self.active_player.lock().unwrap(); // Get the active player index
        let players_lock = self.players.lock().unwrap(); // First, lock the players mutex to get access to the Vec
        let player_arc = &players_lock[active_player_index as usize - 1]; // adjusted for 1 indexing // Get the active player (Arc<Mutex<Player>>)
        let mut player = player_arc.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
        player.get_bonks(level) 
    }

    pub fn active_player_get_clone(&self) -> Arc<Mutex<dyn Player + Send>> {
        let active_player_index = *self.active_player.lock().unwrap(); // Get the active player index
        let players_lock = self.players.lock().unwrap(); // First, lock the players mutex to get access to the Vec
        let player_arc = players_lock[active_player_index as usize - 1].clone(); // adjusted for 1 indexing // Get the active player (Arc<Mutex<Player>>)
        player_arc
    }

    pub fn active_player_get_hole_completion_state(&self) -> bool {
        let active_player_index = *self.active_player.lock().unwrap(); // Get the active player index
        let players_lock = self.players.lock().unwrap(); // First, lock the players mutex to get access to the Vec
        let player_arc = &players_lock[active_player_index as usize - 1]; // adjusted for 1 indexing // Get the active player (Arc<Mutex<Player>>)
        let player = player_arc.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
        player.get_hole_completion_state()
    }

    pub fn active_player_get_index(&self) -> i32 {
        let active_player = *self.active_player.lock().unwrap();
        active_player
    }

    pub fn active_player_get_player_id(&self) -> Uuid {
        let active_player_index = *self.active_player.lock().unwrap(); // Get the active player index
        let players_lock = self.players.lock().unwrap(); // First, lock the players mutex to get access to the Vec
        let player_arc = &players_lock[active_player_index as usize - 1]; // adjusted for 1 indexing // Get the active player (Arc<Mutex<Player>>)
        let player = player_arc.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
        player.get_player_id()
    }

    pub fn active_player_get_player_type(&self) -> String {
        let active_player_index = *self.active_player.lock().unwrap(); // Get the active player index
        let players_lock = self.players.lock().unwrap(); // First, lock the players mutex to get access to the Vec
        let player_arc = &players_lock[active_player_index as usize - 1]; // adjusted for 1 indexing // Get the active player (Arc<Mutex<Player>>)
        let player = player_arc.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
        player.get_player_type()
    }
    
    pub fn active_player_get_score(&self) -> [i32; 18] {
        let players_lock = self.players.lock().unwrap();
        let active_idx_lock = self.active_player.lock().unwrap().to_owned() - 1;
        let scorecard = players_lock[active_idx_lock as usize].lock().unwrap().get_score();
        scorecard
    }

    pub fn active_player_set(&mut self, target: i32) {
        let mut active_player = self.active_player.lock().unwrap();
        *active_player = target;
    }

    pub fn active_player_set_hole_completion_state(&mut self, state: bool) {
        let active_player_index = *self.active_player.lock().unwrap(); // Get the active player index
        let players_lock = self.players.lock().unwrap(); // First, lock the players mutex to get access to the Vec
        let player_arc = &players_lock[active_player_index as usize - 1]; // adjusted for 1 indexing // Get the active player (Arc<Mutex<Player>>)
        let mut player = player_arc.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
        player.set_hole_completion_state(state);
    }

    pub fn all_finished(&self) -> bool {
        // Verify if all players have completed
        let player_count: i32 = self.party_size().try_into().unwrap();
        self.all_players_get_finished_count() == player_count
    }

    pub fn all_players_get_ids(&self) -> Vec<Uuid> {
        let mut id_storage: Vec<Uuid> = Vec::new();
        let players_lock = self.players.lock().unwrap(); // First, lock the players mutex to get access to the Vec
        for player in 0..players_lock.len() {
            let player_arc = &players_lock[player];
            let player = player_arc.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
            let id = player.get_player_id();
            id_storage.push(id);
        }
        id_storage
    }

    pub fn all_players_get_ids_and_types(&self) -> Vec<(Uuid, String)> {
        let mut id_type_storage: Vec<(Uuid, String)> = Vec::new();
        let players_lock = self.players.lock().unwrap(); // First, lock the players mutex to get access to the Vec
        for player in 0..players_lock.len() {
            let player_arc = &players_lock[player];
            let player = player_arc.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
            let id = player.get_player_id();
            let player_type = player.get_player_type();
            let id_type = (id, player_type);
            id_type_storage.push(id_type);
        }
        id_type_storage
    }
    
    pub fn all_players_get_ids_and_scores(&self) -> (Vec<Uuid>, Vec<[i32; 18]>) {
        let mut players: Vec<Uuid> = Vec::new();
        let mut scores: Vec<[i32; 18]> = Vec::new();
        let players_lock = self.players.lock().unwrap();
        for player in players_lock.iter() {
            let player_id = player.lock().unwrap().get_player_id();
            players.push(player_id);
            let score = player.lock().unwrap().get_score();
            scores.push(score);
        }
        (players, scores)
    }

    pub fn all_players_get_finished_count(&self) -> i32 {
        let mut count: i32 = 0;
        let players_lock = self.players.lock().unwrap(); // First, lock the players mutex to get access to the Vec
        for player in 0..players_lock.len() {
            let player_arc = &players_lock[player];
            let player = player_arc.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
            if player.get_hole_completion_state() {
                count += 1;
            }
        }
        count
    }

    pub fn game_completed(&mut self) {
        // First, lock the players mutex to get access to the Vec
        let players_lock = self.players.lock().unwrap();
        for player in 0..players_lock.len() {
            let player_arc = &players_lock[player]; // Get the active player (Arc<Mutex<Player>>)
            let mut player = player_arc.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
            player.game_completed();
        }
    }

    pub fn get_count_ai(&self) -> usize {
        let mut count: usize = 0;
        let players_lock = self.players.lock().unwrap();
        for player in players_lock.iter() {
            let player_type = player.lock().unwrap().get_player_type();
            if player_type == String::from("PlayerAi") {
                count += 1;
            }
        }
        count
    }

    pub fn get_count_local(&self) -> usize {
        let mut count: usize = 0;
        let players_lock = self.players.lock().unwrap();
        for player in players_lock.iter() {
            let player_type = player.lock().unwrap().get_player_type();
            if player_type == String::from("PlayerLocal") {
                count += 1;
            }
        }
        count
    }

    pub fn get_count_party(&self) -> usize {
        let players_lock = self.players.lock().unwrap();
        let count: usize = players_lock.len();
        count
    }

    pub fn get_party_ai_index_vec(&self) -> Vec<usize> {
        let mut ai_index: Vec<usize> = Vec::new();
        let players_lock = self.players.lock().unwrap();
        for (index, player) in players_lock.iter().enumerate() {
            let player_type = player.lock().unwrap().get_player_type();
            if player_type == String::from("PlayerAi") {
                ai_index.push(index);
            };
        }
        ai_index
    }

    pub fn main_player_get_player_id(&self) -> Uuid {
        let players_lock = self.players.lock().unwrap(); // First, lock the players mutex to get access to the Vec
        let player_arc = &players_lock[0]; // adjusted for 1 indexing // Get the active player (Arc<Mutex<Player>>)
        let player = player_arc.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
        player.get_player_id()
    }

    pub fn next_player_proximity(&self, ) {
        todo!(); 
    }

    pub fn next_player_set_order(&mut self) {
        let mut active_player_index = self.active_player.lock().unwrap();
        let players_len = self.players.lock().unwrap().len() as i32;

        *active_player_index = (*active_player_index % players_len) + 1; // Wraps to 1 after reaching the last player
        info!("post function: next_set_order_player");
    }

    pub fn next_round_prep(&mut self) {
        // First, lock the players mutex to get access to the Vec
        let players_lock = self.players.lock().unwrap();
        for player in 0..players_lock.len() {
            let player_arc = &players_lock[player]; // Get the active player (Arc<Mutex<Player>>)
            let mut player = player_arc.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
            player.next_round_prep();
        }
    }

    pub fn party_size(&self) -> usize {        
        // First, lock the players mutex to get access to the Vec
        let players_lock = self.players.lock().unwrap();

        // Grab the size of the party
        let party_size = &players_lock.len();
        *party_size 
    }

    pub fn player_set_hole_completion_state(&mut self, player_id: Uuid, state: bool) {
        let mut players = self.players.lock().unwrap(); // First, lock the players mutex to get access to the Vec
        for player in players.iter_mut() {
            let mut player = player.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
            let id = player.get_player_id();
            if id == player_id {
                player.set_hole_completion_state(state);
            };
        }
    }

    pub fn player_set_player_id(&mut self, player_idx: usize, new_id: Uuid) {
        let players_lock = self.players.lock().unwrap(); // First, lock the players mutex to get access to the Vec
        let player_arc = &players_lock[player_idx]; // adjusted for 1 indexing // Get the active player (Arc<Mutex<Player>>)
        let mut player = player_arc.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
        player.set_player_id(new_id);
    }

    pub fn players_add_player(&self, player: Arc<Mutex<dyn Player + Send>>) {
        let mut players_lock = self.players.lock().unwrap();
        if players_lock.len() < 6 {
            players_lock.push(player);
        } else {
            info!("Error: Party full!");
        }
    }
    
    pub fn players_remove_ai(&self) {
        let mut players_lock = self.players.lock().unwrap(); // Acquire the lock to get mutable access
    
        // Iterate through players and find the index of the first occurrence of "PlayerAi".
        if let Some(index) = players_lock.iter().position(|player| {
            let player_lock = player.lock().unwrap();
            player_lock.get_player_type().as_str() == "PlayerAi"
        }) {
            // Remove the player at the found index
            players_lock.remove(index);
        }
    }
    
    pub fn players_remove_last_player(&self) {
        let mut players_lock = self.players.lock().unwrap(); // Acquire the lock to get mutable access
        // Only pop if we have more than one player
        if players_lock.len() > 1 {
            players_lock.pop();
        }
    }
    
    pub fn players_remove_local_player(&self) {
        let mut players_lock = self.players.lock().unwrap(); // Acquire the lock to get mutable access
        // Only pop if we have more than one player
        if players_lock.len() > 1 {
            if let Some(rev_index) = players_lock.iter().rev().position(|player| {
                let player_lock = player.lock().unwrap();
                player_lock.get_player_type().as_str() == "PlayerLocal" 
            }) {
                // Convert the reversed index to the original index
                let original_index = players_lock.len() - 1 - rev_index;
    
                // Remove the player at the original index
                players_lock.remove(original_index);
            }
        }
    }

    pub fn players_remove_player(&self, player_id: Uuid) {
        let mut players_lock = self.players.lock().unwrap();
        
        // Proceed only if we have more than one player in the vector
        if players_lock.len() > 1 {
            players_lock.retain(|player| {
                let player_lock = player.lock().unwrap();
                player_lock.get_player_id() != player_id
            });
        }
    }

    pub fn start_game(&mut self) {
        // First, lock the players mutex to get access to the Vec
        let players_lock = self.players.lock().unwrap();

        for player in 0..players_lock.len() {
            let player_arc = &players_lock[player]; // Get the active player (Arc<Mutex<Player>>)
            let mut player = player_arc.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
            player.start_game();
        }
    }

    pub fn update_ai_index_vec(&mut self) {
        self.ai_vec = Some(self.get_party_ai_index_vec());
    }
}

pub fn party_handler_active_player_add_bonk(
    mut run_trigger: ResMut<RunTrigger>,
    party: Res<Party>,
    game_handler: Res<GameHandler>,
    game_state: Res<State<StateGame>>,
) {
    info!("function: party_handler_active_player_add_bonk"); 
    {
        match game_state.get() {
            StateGame::InGame => {
                party.active_player_add_bonk(game_handler.current_level_get() as usize);
            },
            _ => {},
        }
    }
    run_trigger.set_target(CheckStateRT::PartyHandlerActivePlayerAddBonk, false);
    info!("post response: party_handler_active_player_add_bonk: {}", run_trigger.get(CheckStateRT::PartyHandlerActivePlayerAddBonk));   
}

pub fn party_handler_active_player_set_hole_completion_state_true(
    mut run_trigger: ResMut<RunTrigger>,
    state_game: Res<State<StateGame>>,
    mut party: ResMut<Party>,
) {
    info!("function: party_handler_active_player_set_hole_completion_state_true"); 
    {
        match state_game.get() {
            StateGame::InGame => {
                party.active_player_set_hole_completion_state(true);
            },
            StateGame::NotInGame => {},
        };
    }
    run_trigger.set_target(CheckStateRT::PartyHandlerActivePlayerSetHoleCompletionStateTrue, false);
    info!("post response: party_handler_active_player_set_hole_completion_state_true: {}", run_trigger.get(CheckStateRT::PartyHandlerActivePlayerSetHoleCompletionStateTrue));   
}

pub fn party_handler_cycle_active_player( 
    mut run_trigger: ResMut<RunTrigger>,
    mut party: ResMut<Party>,
) {
    info!("function: party_handler_cycle_active_player"); 
    {
        run_trigger.set_target(CheckStateRT::GolfBallHandlerPartyStoreLocations, true);

        let finished_count = party.all_players_get_finished_count() as usize;
        let party_size = party.party_size();
        if finished_count != party_size {
            loop {
                party.next_player_set_order();
                let players = party.players.lock().unwrap();
                let ref_idx = (party.active_player_get_index() - 1) as usize;
                if let Some(player) = players.get(ref_idx) {
                    if !player.lock().unwrap().get_hole_completion_state() {
                        break;
                    }
                }
            }
        }
    }
    run_trigger.set_target(CheckStateRT::PartyHandlerCycleActivePlayer, false);
    info!("post response: party_handler_cycle_active_player: {}", run_trigger.get(CheckStateRT::PartyHandlerCycleActivePlayer));  
}

pub fn party_handler_new_player_ai(
    party: Res<Party>,
    mut run_trigger: ResMut<RunTrigger>,
) {
    info!("function: party_handler_new_player_ai"); 
    {
        let new_player_ai = PlayerAi::new();
        let new_player = Arc::new(Mutex::new(new_player_ai));
        party.players_add_player(new_player);
    }
    run_trigger.set_target(CheckStateRT::PartyHandlerNewPlayerAi, false);
    info!("post response: party_handler_new_player_ai: {}", run_trigger.get(CheckStateRT::PartyHandlerNewPlayerAi));  
}

pub fn party_handler_new_player_local(
    party: Res<Party>,
    mut run_trigger: ResMut<RunTrigger>,
) {
    info!("function: party_handler_new_player_local"); 
    {
        if party.get_count_party() == 6 && party.get_count_ai() > 0 {
            party.players_remove_ai();
        };
        if party.get_count_party() < 6 {
            let new_player_local = PlayerLocal::new();
            let new_player = Arc::new(Mutex::new(new_player_local));
            party.players_add_player(new_player);
        };
    }
    run_trigger.set_target(CheckStateRT::PartyHandlerNewPlayerLocal, false);
    info!("post response: party_handler_new_player_local: {}", run_trigger.get(CheckStateRT::PartyHandlerNewPlayerLocal));  
}

pub fn party_handler_new_player_remote(
    party: Res<Party>,
    mut run_trigger: ResMut<RunTrigger>,
) {
    info!("function: party_handler_new_player_remote"); 
    {
        let new_player_remote = PlayerRemote::new();
        let new_player = Arc::new(Mutex::new(new_player_remote));
        party.players_add_player(new_player);
    }
    run_trigger.set_target(CheckStateRT::PartyHandlerNewPlayerRemote, false);
    info!("post response: party_handler_new_player_remote: {}", run_trigger.get(CheckStateRT::PartyHandlerNewPlayerRemote));  

}

pub fn party_handler_remove_ai(
    party: Res<Party>,
    mut run_trigger: ResMut<RunTrigger>,
) {
    info!("function: party_handler_remove_ai"); 
    {
        party.players_remove_ai();
    }
    run_trigger.set_target(CheckStateRT::PartyHandlerRemoveAi, false);
    info!("post response: party_handler_remove_ai: {}", run_trigger.get(CheckStateRT::PartyHandlerRemoveAi));  
}

pub fn party_handler_remove_last_player(
    party: Res<Party>,
    mut run_trigger: ResMut<RunTrigger>,
) {    
    info!("function: party_handler_remove_last_player"); 
    {
        party.players_remove_last_player();
    }
    run_trigger.set_target(CheckStateRT::PartyHandlerRemoveLastPlayer, false);
    info!("post response: party_handler_remove_last_player: {}", run_trigger.get(CheckStateRT::PartyHandlerRemoveLastPlayer));  
}

pub fn party_handler_remove_local_player(
    party: Res<Party>,
    mut run_trigger: ResMut<RunTrigger>,
) {    
    info!("function: party_handler_remove_local_player"); 
    {
        party.players_remove_local_player();
    }
    run_trigger.set_target(CheckStateRT::PartyHandlerRemoveLocalPlayer, false);
    info!("post response: party_handler_remove_local_player: {}", run_trigger.get(CheckStateRT::PartyHandlerRemoveLocalPlayer));  
}





