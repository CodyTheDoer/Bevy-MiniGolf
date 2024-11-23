use bevy::prelude::*;

// States
use crate::{
    StateMapSet,
};

// Resources
use crate::{
    Party,
    Player,
};

use std::sync::Arc;
use std::sync::Mutex;

impl Party {
    pub fn new() -> Self {
        let players: Arc<Mutex<Vec<Arc<Mutex<Player>>>>> = Arc::new(Mutex::new(vec![Arc::new(Mutex::new(Player::new()))]));
        let players_finished: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
        let active_player: Arc<Mutex<i32>> = Arc::new(Mutex::new(1));
        let active_level: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
        let ai_count: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
        let remote_count: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
        Party {
            players,
            players_finished,
            active_player,
            active_level,
            ai_count,
            remote_count,
        } 
    }
    
    pub fn add_player(&self) {
        let mut players_lock = self.players.lock().unwrap(); // Acquire the lock to get mutable access
        let owned_party_size: i32 = players_lock.len() as i32; // Gets the party size not including ai
        let mut ai_count = self.ai_count.lock().unwrap(); // account for ai
        let total_party_size = *ai_count + owned_party_size;

        if owned_party_size < 6 { // Stop making players if the party is full
            let new_player: Arc<Mutex<Player>> = Arc::new(Mutex::new(Player::new()));
            players_lock.push(new_player);
        
            if total_party_size >= 6 { // purge a single ai player if needed, I am not sure why it triggers properly on >= 6, prior to nesting in owned if it fired on > 6 
                *ai_count -= 1;
            } 
        }
    }

    pub fn remove_player(&self) {
        let mut players_lock = self.players.lock().unwrap(); // Acquire the lock to get mutable access
        let owned_party_size: i32 = players_lock.len() as i32; // Gets the party size not including ai
        if owned_party_size > 1 {
            players_lock.pop();
        };
    }

    pub fn get_party_size_w_ai(&self) -> usize {
        let players_lock = self.players.lock().unwrap(); // First, lock the players mutex to get access to the Vec
        let owned_party_size: i32 = players_lock.len() as i32; // Gets the party size not including ai
        let ai_count = self.ai_count.lock().unwrap(); // account for ai
        let total_party_size = *ai_count + owned_party_size;

        total_party_size as usize
    }

    pub fn get_party_size(&self) -> usize {        
        // First, lock the players mutex to get access to the Vec
        let players_lock = self.players.lock().unwrap();

        // Grab the size of the party
        let party_size = &players_lock.len();
        *party_size 
    }
    
    pub fn add_ai(&self) {
        let owned_party_size: i32 = self.get_party_size() as i32;
        let mut ai_count = self.ai_count.lock().unwrap();

        let total_party_size = *ai_count + owned_party_size;

        if total_party_size < 6 {
            if *ai_count < 5 {
                *ai_count += 1;
            }
        } 
    }
    
    pub fn remove_ai(&self) {
        let mut count = self.ai_count.lock().unwrap();
        if *count > 0 {
            *count -= 1;
        }
    }

    pub fn get_ai_count(&self) -> i32 {
        let count = self.ai_count.lock().unwrap();
        *count
    }

    pub fn get_players_finished(&self) -> i32 {
        let count = self.players_finished.lock().unwrap();
        *count
    }

    pub fn log_player_finished(&mut self) {
        let mut count = self.players_finished.lock().unwrap();
        *count += 1;
    }

    pub fn reset_players_finished(&mut self) {
        let mut count = *self.players_finished.lock().unwrap();
        count = 0;
    }

    pub fn get_active_player_bonks_level(&mut self) -> u32 {
        self.log_player_finished();
        // Get the active player index
        let active_player_index = *self.active_player.lock().unwrap();
        // First, lock the players mutex to get access to the Vec
        let players_lock = self.players.lock().unwrap();
        // Get the active player (Arc<Mutex<Player>>)
        let player_arc = &players_lock[active_player_index as usize - 1]; // adjusted for 1 indexing
        // Lock the player mutex to get a mutable reference to the player
        let mut player = player_arc.lock().unwrap();
        player.get_bonks_level()
    }

    pub fn get_active_player_bonks_game(&mut self) -> u32 {
        self.log_player_finished();
        // Get the active player index
        let active_player_index = *self.active_player.lock().unwrap();
        // First, lock the players mutex to get access to the Vec
        let players_lock = self.players.lock().unwrap();
        // Get the active player (Arc<Mutex<Player>>)
        let player_arc = &players_lock[active_player_index as usize - 1]; // adjusted for 1 indexing
        // Lock the player mutex to get a mutable reference to the player
        let mut player = player_arc.lock().unwrap();
        player.get_bonks_game()
    }

    pub fn active_player_add_bonk(&mut self) {
        self.log_player_finished();
        // Get the active player index
        let active_player_index = *self.active_player.lock().unwrap();
        // First, lock the players mutex to get access to the Vec
        let players_lock = self.players.lock().unwrap();
        // Get the active player (Arc<Mutex<Player>>)
        let player_arc = &players_lock[active_player_index as usize - 1]; // adjusted for 1 indexing
        // Lock the player mutex to get a mutable reference to the player
        let mut player = player_arc.lock().unwrap();
        player.add_bonk();
    }

    pub fn active_player_finished_hole(&mut self) {
        self.log_player_finished();
        // Get the active player index
        let active_player_index = *self.active_player.lock().unwrap();
        // First, lock the players mutex to get access to the Vec
        let players_lock = self.players.lock().unwrap();
        // Get the active player (Arc<Mutex<Player>>)
        let player_arc = &players_lock[active_player_index as usize - 1]; // adjusted for 1 indexing
        // Lock the player mutex to get a mutable reference to the player
        let mut player = player_arc.lock().unwrap();
        player.hole_completed();
    }

    pub fn all_finished(&self) -> bool {
        // Verify if all players have completed
        let player_count: i32 = self.get_party_size().try_into().unwrap();
        self.get_players_finished() == player_count
    }

    pub fn next_proximity_player(&self, ) {
        todo!(); 
    }

    pub fn next_set_order_player(&mut self) {
        let mut active_player = self.active_player.lock().unwrap();
        let party_size: i32 = self.get_party_size().try_into().unwrap();
        // info!("Party Size: {:?}, Active Player: {:?}", party_size.clone(), active_player.clone());
        if *active_player == party_size {
            *active_player = 1;
        } else {
            *active_player += 1;
        }
    }

    pub fn get_active_level(&self) -> i32 {
        let active_level = *self.active_level.lock().unwrap();
        active_level
    }

    pub fn get_active_player(&self) -> i32 {
        let active_player = *self.active_player.lock().unwrap();
        active_player
    }

    pub fn next_level(&mut self) {
        let mut active_level = self.active_level.lock().unwrap();
        *active_level += 1;
    }

    pub fn set_starting_level(&mut self, map_set_state: StateMapSet) {
        let mut active_level = self.active_level.lock().unwrap();
        let owned_map_state = map_set_state.clone();
        info!("owned_map_state: {:?}", owned_map_state);
        match map_set_state {
            StateMapSet::Tutorial => {
                *active_level = 19;
            }, 
            StateMapSet::WholeCorse => {
                *active_level = 1;
            },
            StateMapSet::FrontNine => {
                *active_level = 1;
            },
            StateMapSet::BackNine => {
                *active_level = 10;
            },
            StateMapSet::SelectAHole => {},
        }
    }

    pub fn start_game(&mut self) {
        // First, lock the players mutex to get access to the Vec
        let players_lock = self.players.lock().unwrap();

        for player in 0..players_lock.len() {
            // Get the active player (Arc<Mutex<Player>>)
            let player_arc = &players_lock[player];
            // Lock the player mutex to get a mutable reference to the player
            let mut player = player_arc.lock().unwrap();
            player.start_game();
        }
    }

    pub fn next_round_prep(&mut self) {
        // First, lock the players mutex to get access to the Vec
        let players_lock = self.players.lock().unwrap();

        for player in 0..players_lock.len() {
            // Get the active player (Arc<Mutex<Player>>)
            let player_arc = &players_lock[player];
            // Lock the player mutex to get a mutable reference to the player
            let mut player = player_arc.lock().unwrap();
            player.next_round_prep();
        }
    }
}
