use bevy::prelude::*;

// States
use crate::{
    StateMapSet,
};

// Resources
use crate::{
    Party,
    Player,
    PlayerLocal,
    PlayerAi,
    PlayerRemote,
};

use std::sync::Arc;
use std::sync::Mutex;

impl Party {
    pub fn new() -> Self {
        let players: Arc<Mutex<Vec<Arc<Mutex<dyn Player + Send>>>>> = Arc::new(Mutex::new(vec![Arc::new(Mutex::new(PlayerLocal::new()))]));
        let players_finished: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
        let active_player: Arc<Mutex<i32>> = Arc::new(Mutex::new(1));
        let active_level: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
        let remote_count: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
        Party {
            players,
            players_finished,
            active_player,
            active_level,
            remote_count,
        } 
    }
    
    pub fn add_player(&self, player: Arc<Mutex<dyn Player + Send>>) {
        let mut players_lock = self.players.lock().unwrap();
        players_lock.push(player);
    }

    pub fn remove_player(&self, player_id: String) {
        let mut players_lock = self.players.lock().unwrap();
        
        // Proceed only if we have more than one player in the vector
        if players_lock.len() > 1 {
            players_lock.retain(|player| {
                let player_lock = player.lock().unwrap();
                player_lock.get_player_id() != player_id
            });
        }
    }

    pub fn get_party_size(&self) -> usize {        
        // First, lock the players mutex to get access to the Vec
        let players_lock = self.players.lock().unwrap();

        // Grab the size of the party
        let party_size = &players_lock.len();
        *party_size 
    }
    
    pub fn remove_ai(&self) {
        let mut players_lock = self.players.lock().unwrap(); // Acquire the lock to get mutable access
    
        // Iterate through players and find the index of the first occurrence of "PlayerAi".
        if let Some(index) = players_lock.iter().position(|player| {
            let player_lock = player.lock().unwrap();
            player_lock.get_player_id() == "PlayerAi"
        }) {
            // Remove the player at the found index
            players_lock.remove(index);
        }
    }
    
    pub fn remove_last_player(&self) {
        let mut players_lock = self.players.lock().unwrap(); // Acquire the lock to get mutable access
        
        // Only pop if we have more than one player
        if players_lock.len() > 1 {
            players_lock.pop();
        }
    }

    pub fn get_players_finished(&self) -> i32 {
        let count: i32 = *self.players_finished.lock().unwrap();
        count
    }

    pub fn log_player_finished(&mut self) {
        let mut count = self.players_finished.lock().unwrap();
        *count += 1;
    }

    pub fn reset_players_finished(&mut self) {
        let mut count = *self.players_finished.lock().unwrap();
        count = 0;
    }

    pub fn active_player_get_bonks_game(&mut self) -> u32 {
        let active_player_index = *self.active_player.lock().unwrap(); // Get the active player index
        let players_lock = self.players.lock().unwrap(); // First, lock the players mutex to get access to the Vec
        let player_arc = &players_lock[active_player_index as usize - 1]; // adjusted for 1 indexing // Get the active player (Arc<Mutex<Player>>)
        let mut player = player_arc.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
        player.get_bonks_game()
    }

    pub fn active_player_get_bonks_level(&mut self) -> u32 {
        let active_player_index = *self.active_player.lock().unwrap(); // Get the active player index
        let players_lock = self.players.lock().unwrap(); // First, lock the players mutex to get access to the Vec
        let player_arc = &players_lock[active_player_index as usize - 1]; // adjusted for 1 indexing // Get the active player (Arc<Mutex<Player>>)
        let mut player = player_arc.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
        player.get_bonks_level()
    }

    pub fn active_player_add_bonk(&mut self) {
        let active_player_index = *self.active_player.lock().unwrap(); // Get the active player index
        let players_lock = self.players.lock().unwrap(); // First, lock the players mutex to get access to the Vec
        let player_arc = &players_lock[active_player_index as usize - 1]; // adjusted for 1 indexing // Get the active player (Arc<Mutex<Player>>)
        let mut player = player_arc.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
        player.add_bonk();
        info!("post function: active_player_add_bonk"); 
    }

    pub fn active_player_finished_hole(&mut self) {
        self.log_player_finished();
        let active_player_index = *self.active_player.lock().unwrap(); // Get the active player index
        let players_lock = self.players.lock().unwrap(); // First, lock the players mutex to get access to the Vec
        let player_arc = &players_lock[active_player_index as usize - 1]; // adjusted for 1 indexing // Get the active player (Arc<Mutex<Player>>)
        let mut player = player_arc.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
        player.hole_completed();
        info!("post function: active_player_finished_hole"); 
    }   

    pub fn active_player_get_ball_location(&mut self) -> Vec3 {
        let active_player_index = *self.active_player.lock().unwrap(); // Get the active player index
        let players_lock = self.players.lock().unwrap(); // First, lock the players mutex to get access to the Vec
        let player_arc = &players_lock[active_player_index as usize - 1]; // adjusted for 1 indexing // Get the active player (Arc<Mutex<Player>>)
        let mut player = player_arc.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
        player.get_ball_location()
    }

    pub fn active_player_get_hole_completion_state(&mut self) -> bool {
        let active_player_index = *self.active_player.lock().unwrap(); // Get the active player index
        let players_lock = self.players.lock().unwrap(); // First, lock the players mutex to get access to the Vec
        let player_arc = &players_lock[active_player_index as usize - 1]; // adjusted for 1 indexing // Get the active player (Arc<Mutex<Player>>)
        let mut player = player_arc.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
        player.get_hole_completion_state()
    }

    pub fn active_player_set_hole_completion_state(&mut self, player_id: bool) {
        let active_player_index = *self.active_player.lock().unwrap(); // Get the active player index
        let players_lock = self.players.lock().unwrap(); // First, lock the players mutex to get access to the Vec
        let player_arc = &players_lock[active_player_index as usize - 1]; // adjusted for 1 indexing // Get the active player (Arc<Mutex<Player>>)
        let mut player = player_arc.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
        player.set_hole_completion_state(true);
    }

    pub fn active_player_get_player_id(&mut self) -> String {
        let active_player_index = *self.active_player.lock().unwrap(); // Get the active player index
        let players_lock = self.players.lock().unwrap(); // First, lock the players mutex to get access to the Vec
        let player_arc = &players_lock[active_player_index as usize - 1]; // adjusted for 1 indexing  // Get the active player (Arc<Mutex<Player>>)
        let mut player = player_arc.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
        player.get_player_id()
    }

    pub fn active_player_set_player_id(&mut self, player_id: String) {
        let active_player_index = *self.active_player.lock().unwrap(); // Get the active player index
        let players_lock = self.players.lock().unwrap(); // First, lock the players mutex to get access to the Vec
        let player_arc = &players_lock[active_player_index as usize - 1]; // adjusted for 1 indexing // Get the active player (Arc<Mutex<Player>>)
        let mut player = player_arc.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
        player.set_player_id(player_id);
    }

    pub fn active_player_set_ball_location(&mut self, location: Vec3) {
        // Get the active player index in a separate block to reduce lock scope
        let active_player_index = {
            let active_lock = self.active_player.lock().unwrap();
            *active_lock as usize - 1 // Convert to zero-based index
        };
    
        // Lock the players vector and get the active player
        let player_arc = {
            let players_lock = self.players.lock().unwrap();
            players_lock[active_player_index].clone()
        };
    
        // Now lock the specific player and set its ball location
        let mut player = player_arc.lock().unwrap();
        player.set_ball_location(location);
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
        let party_size: i32 = self.get_party_size() as i32;
        if *active_player == party_size {
            *active_player = 1;
        } else {
            *active_player += 1;
        }
        info!("post function: next_set_order_player"); 
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
            let player_arc = &players_lock[player]; // Get the active player (Arc<Mutex<Player>>)
            let mut player = player_arc.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
            player.start_game();
        }
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
}


    // pub fn add_player(&self) {
    //     let mut players_lock = self.players.lock().unwrap(); // Acquire the lock to get mutable access
    //     let mut ai_players_lock = self.players_ai.lock().unwrap(); // Acquire the lock to get mutable access
    //     let owned_party_size: usize = players_lock.len(); // Gets the party size not including ai
    //     let mut ai_count = ai_players_lock.len(); // account for ai
    //     let total_party_size = ai_count + owned_party_size;

    //     if owned_party_size < 6 { // Stop making players if the party is full
    //         let new_player: Arc<Mutex<Player>> = Arc::new(Mutex::new(PlayerLocal {
    //                 player_id: String::from(format!("PlayerLocal{}@email.com", owned_party_size + 1)),
    //                 hole_completion_state: false,
    //                 ball_material: Color::srgb(1.0, 0.0, 1.0),
    //                 ball_location: Vec3::new(0.0, 0.0, 0.0),
    //                 bonks_level: 0,
    //                 bonks_game: 0,
    //             }));
    //         players_lock.push(new_player);
        
    //         if total_party_size >= 6 { // purge a single ai player if needed, I am not sure why it triggers properly on >= 6, prior to nesting in owned if it fired on > 6 
    //             ai_players_lock.pop();
    //         } 
    //     }
    // }

    // pub fn remove_player(&self) {
    //     let mut players_lock = self.players.lock().unwrap();
    //     let owned_party_size = players_lock.len() as i32;

    //     if owned_party_size > 1 {
    //         players_lock.pop();
    //     };
    // }

    // pub fn remove_player(&self) {
    //     let mut players_lock = self.players.lock().unwrap(); // Acquire the lock to get mutable access
    //     let owned_party_size: i32 = players_lock.len() as i32; // Gets the party size not including ai
    //     if owned_party_size > 1 {
    //         players_lock.pop();
    //     };
    // }

    // pub fn get_party_size_w_ai(&self) -> usize { 
    //     let mut players_lock = self.players.lock().unwrap(); // Acquire the lock to get mutable access
    //     let owned_party_size: usize = players_lock.len(); // Gets the party size not including ai
    //     let players_ai_lock = self.players_ai.lock().unwrap();
    //     let ai_party_size = &players_ai_lock.len();
    //     let total_party_size = *ai_party_size + owned_party_size;

    //     total_party_size as usize
    // }

    // pub fn get_ai_count(&self) -> i32 {
    //     let mut ai_players_lock = self.players_ai.lock().unwrap(); // Acquire the lock to get mutable access
    //     let mut count: i32 = ai_players_lock.len() as i32;
    //     count
    // }