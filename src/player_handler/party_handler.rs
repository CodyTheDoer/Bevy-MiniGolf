use bevy::prelude::*;

// States
use crate::{
    StateGame,
    StateLevel,
    StateMapSet,
    StateMenu,
};

// Resources
use crate::{
    GameHandler,
    Party,
    Player,
    PlayerLocal,
    PlayerAi,
    PlayerRemote,
    RunTrigger,
};

use std::sync::Arc;
use std::sync::Mutex;

impl Party {
    pub fn new() -> Self {
        let players: Arc<Mutex<Vec<Arc<Mutex<dyn Player + Send>>>>> = Arc::new(Mutex::new(vec![Arc::new(Mutex::new(PlayerLocal::new()))]));
        let players_finished: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
        let active_player: Arc<Mutex<i32>> = Arc::new(Mutex::new(1));
        // let active_level: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
        let remote_count: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
        Party {
            players,
            players_finished,
            active_player,
            // active_level,
            remote_count,
        } 
    }
    
    pub fn add_player(&self, player: Arc<Mutex<dyn Player + Send>>) {
        let mut players_lock = self.players.lock().unwrap();
        if players_lock.len() < 6 {
            players_lock.push(player);
        } else {
            info!("Error: Party full!");
        }
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

    pub fn all_players_get_finished_count(&self) -> i32 {
        let mut count: i32 = 0;
        let players_lock = self.players.lock().unwrap(); // First, lock the players mutex to get access to the Vec
        for player in 0..players_lock.len() {
            let player_arc = &players_lock[player];
            let mut player = player_arc.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
            if player.get_hole_completion_state() {
                count += 1;
            }
        }
        count
    }

    pub fn reset_players_finished(&mut self) {
        let mut count = *self.players_finished.lock().unwrap();
        count = 0;
    }

    pub fn all_finished(&self) -> bool {
        // Verify if all players have completed
        let player_count: i32 = self.get_party_size().try_into().unwrap();
        self.all_players_get_finished_count() == player_count
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

    pub fn active_player_set_ball_location(&mut self, location: Vec3) {
        info!("Locking active_player to get the current active index");
        let active_player_index: usize = {
            let active_lock = self.active_player.lock().unwrap();
            *active_lock as usize - 1
        };
        info!("Got active player index: {}", active_player_index);
        
        info!("Locking players to get the active player reference");
        let player_arc = {
            let players_lock = self.players.lock().unwrap();
            players_lock[active_player_index].clone()
        };
        info!("Locked player successfully, setting ball location to {:?}", location);
        
        let mut player = player_arc.lock().unwrap();
        player.set_ball_location(location);
        info!("Set ball location successfully for player at index {}", active_player_index);
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
        let player_arc = &players_lock[active_player_index as usize - 1]; // adjusted for 1 indexing // Get the active player (Arc<Mutex<Player>>)
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

    pub fn next_proximity_player(&self, ) {
        todo!(); 
    }

    pub fn next_set_order_player(&mut self) {
        let mut active_player_index = self.active_player.lock().unwrap();
        let players_count = self.players.lock().unwrap().len() as i32;
        *active_player_index = (*active_player_index % players_count) + 1; // Wraps to 1 after reaching the last player
        info!("post function: next_set_order_player"); 
    }

    pub fn get_active_player(&self) -> i32 {
        let active_player = *self.active_player.lock().unwrap();
        active_player
    }

    pub fn set_active_player(&mut self, target: i32) {
        let mut active_player = self.active_player.lock().unwrap();
        *active_player = target;
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

pub fn party_handler_active_player_add_bonk(
    mut run_trigger: ResMut<RunTrigger>,
    mut party: ResMut<Party>,
) {
    info!("function: party_handler_active_player_add_bonk"); 
    run_trigger.set_target("party_handler_active_player_set_ball_location", true);
    run_trigger.set_target("game_handler_set_active_ball_location", true);
    party.active_player_add_bonk();
    run_trigger.set_target("party_handler_active_player_add_bonk", false);
}

pub fn party_handler_active_player_set_ball_location(
    mut run_trigger: ResMut<RunTrigger>,
    mut party: ResMut<Party>,
    mut game_handler: ResMut<GameHandler>
    // golf_ball_tag_query: Query<Entity, With<GolfBallTag>>,
) {
    info!("function: party_handler_active_player_set_ball_location"); 
    // let owned_match = GolfBallTag(party.get_active_player().try_into().unwrap());
    // for golf_ball in golf_ball_tag_query.iter_mut() {
    //     owned_match => {
    if let Some(current_ball_location) = game_handler.get_active_ball_location() {
        info!("Setting active ball location: {:?}", current_ball_location);
        party.active_player_set_ball_location(current_ball_location);
    } else {
        info!("No ball location set for the active player, setting default ZERO");
        party.active_player_set_ball_location(Vec3::new(0.0, 0.0, 0.0));
    }
    //     },
    //     _ => {},
    // }
    run_trigger.set_target("party_handler_active_player_set_ball_location", false);
}

pub fn party_handler_active_player_set_hole_completion_state_true(
    mut run_trigger: ResMut<RunTrigger>,
    state_game: Res<State<StateGame>>,
    mut party: ResMut<Party>,
) {
    info!("function: party_handler_active_player_set_hole_completion_state_true"); 
    match state_game.get() {
        StateGame::InGame => {
            party.active_player_set_hole_completion_state(true);
        },
        StateGame::NotInGame => {},
    };
    run_trigger.set_target("party_handler_active_player_set_hole_completion_state_true", false);
}


pub fn party_handler_cycle_active_player(
    mut run_trigger: ResMut<RunTrigger>,
    mut game_handler: ResMut<GameHandler>,
    mut party: ResMut<Party>,
    state_map_set: Res<State<StateMapSet>>,
    state_level: Res<State<StateLevel>>,
) {
    info!("function: party_handler_cycle_active_player"); 
    
    let owned_finished_count = party.all_players_get_finished_count();
    let owned_party_size = party.get_party_size();
    info!("\n\n\n{:?} vs {:?}", owned_finished_count, owned_party_size);
    if owned_finished_count == owned_party_size as i32 {
        match state_map_set.get() {
            StateMapSet::Tutorial => {
                run_trigger.set_target("game_handler_toggle_state_game", true);
            },
            StateMapSet::WholeCorse => {
                match state_level.get() {
                    StateLevel::Hole18 => {
                        run_trigger.set_target("game_handler_toggle_state_game", true);
                    },
                    _ => {
                        party.next_round_prep();
                        party.set_active_player(1);
                        run_trigger.set_target("game_handler_cycle_current_level", true);
                        run_trigger.set_target("game_handler_get_active_ball_location", true);
                        game_handler.next_level();
                    },
                }
            },
            StateMapSet::FrontNine => {
                match state_level.get() {
                    StateLevel::Hole9 => {
                        run_trigger.set_target("game_handler_toggle_state_game", true);
                    },
                    _ => {
                        party.next_round_prep();
                        party.set_active_player(1);
                        run_trigger.set_target("game_handler_cycle_current_level", true);
                        run_trigger.set_target("game_handler_get_active_ball_location", true);
                        game_handler.next_level();
                    },
                }
            },
            StateMapSet::BackNine => {
                match state_level.get() {
                    StateLevel::Hole18 => {
                        run_trigger.set_target("game_handler_toggle_state_game", true);
                    },
                    _ => {
                        party.next_round_prep();
                        party.set_active_player(1);
                        run_trigger.set_target("game_handler_cycle_current_level", true);
                        run_trigger.set_target("game_handler_get_active_ball_location", true);
                        game_handler.next_level();
                    },
                }
            },
            StateMapSet::SelectAHole => {
                run_trigger.set_target("game_handler_toggle_state_game", true);
            },
        };
    } else {
        party.next_set_order_player();
        run_trigger.set_target("game_handler_get_active_ball_location", true);
    }
    run_trigger.set_target("party_handler_cycle_active_player", false);
}

